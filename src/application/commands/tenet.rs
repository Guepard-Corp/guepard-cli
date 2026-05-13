use std::fs;
use std::io::Write;
use std::net::IpAddr;

use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use tabled::Tabled;

use crate::application::dto::tenet::{TenetDeployRequest, TenetDeployResponse};
use crate::application::output::{print_json, print_row_or_json, OutputFormat};
use crate::application::services::tenet;
use crate::config::config::Config;
use crate::domain::errors::tenet_error::TenetError;
use crate::structure::{
    OutputArgs, TenetArgs, TenetCommand, TenetDeployCliArgs, TenetProxyAction, TenetProxyGetArgs,
    TenetProxySetArgs,
};

fn output_format(args: &OutputArgs) -> OutputFormat {
    if args.json {
        OutputFormat::Json
    } else {
        OutputFormat::Table
    }
}

#[derive(Tabled, Serialize)]
struct TenetDeployRow {
    #[tabled(rename = "Job ID")]
    #[serde(rename = "job_id")]
    job_id: String,
    #[tabled(rename = "Eval ID")]
    #[serde(rename = "eval_id")]
    eval_id: String,
    #[tabled(rename = "Alloc ID")]
    #[serde(rename = "alloc_id")]
    alloc_id: String,
    #[tabled(rename = "Node ID")]
    #[serde(rename = "node_id")]
    node_id: String,
    #[tabled(rename = "Host")]
    host: String,
    #[tabled(rename = "Proxy port")]
    #[serde(rename = "proxy_port")]
    proxy_port: String,
    #[tabled(rename = "API port")]
    #[serde(rename = "api_port")]
    api_port: String,
}

#[derive(Tabled, Serialize)]
struct TenetLifecycleRow {
    #[tabled(rename = "Job ID")]
    job_id: String,
    #[tabled(rename = "Action")]
    action: String,
    #[tabled(rename = "Message")]
    message: String,
}

pub async fn tenet(args: &TenetArgs, config: &Config) -> Result<()> {
    match &args.command {
        TenetCommand::Deploy(d) => deploy(d, config, output_format(&d.output)).await,
        TenetCommand::Start(a) => {
            lifecycle("start", &a.job_id, config, output_format(&a.output)).await
        }
        TenetCommand::Stop(a) => {
            lifecycle("stop", &a.job_id, config, output_format(&a.output)).await
        }
        TenetCommand::Purge(a) => {
            lifecycle("purge", &a.job_id, config, output_format(&a.output)).await
        }
        TenetCommand::Proxy(p) => match &p.action {
            TenetProxyAction::Get(g) => proxy_get(g, config, output_format(&g.output)).await,
            TenetProxyAction::Set(s) => proxy_set(s, config, output_format(&s.output)).await,
        },
    }
}

/// Table + psql hints: merged API `host`, `--client-host`, or resolved `--upstream-host` (literal IP or DNS).
fn tenet_deploy_hint_lines(resp: &TenetDeployResponse, connect_host: Option<&str>) -> Vec<String> {
    let mut lines = Vec::new();
    if let (Some(host), Some(port)) = (connect_host, resp.proxy_port) {
        lines.push(format!(
            r#"PGPASSWORD=<password> psql "host={} port={} dbname=postgres user=guepard sslmode=disable""#,
            host, port
        ));
    }
    if let (Some(host), Some(port)) = (connect_host, resp.api_port) {
        lines.push(format!("Tenet API: http://{}:{}", host, port));
    }
    lines
}

async fn resolve_postgres_host_for_hints(upstream_host: &str) -> Option<String> {
    let host = upstream_host.trim();
    if host.is_empty() {
        return None;
    }
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Some(ip.to_string());
    }
    let addrs: Vec<_> = tokio::net::lookup_host((host, 0u16)).await.ok()?.collect();
    if addrs.is_empty() {
        return None;
    }
    let picked = addrs
        .iter()
        .find(|a| a.is_ipv4())
        .or_else(|| addrs.first())?;
    Some(picked.ip().to_string())
}

async fn deploy(
    d: &TenetDeployCliArgs,
    config: &Config,
    output_format: OutputFormat,
) -> Result<()> {
    let config_yaml = if let Some(path) = &d.proxy_config {
        fs::read_to_string(path).map_err(|e| {
            TenetError::IoError(format!(
                "Cannot read --proxy-config {}: {}",
                path.display(),
                e
            ))
        })?
    } else {
        d.config_yaml.clone().ok_or_else(|| {
            TenetError::Unexpected("Provide --proxy-config or --config-yaml".to_string())
        })?
    };

    let compute_job_id = d
        .compute_job_id
        .clone()
        .or_else(|| Some(format!("{}-compute", d.tenant_id)));

    let body = TenetDeployRequest {
        tenant_id: d.tenant_id.clone(),
        compute_job_id,
        upstream_host: d.upstream_host.clone(),
        upstream_port: d.upstream_port,
        masking_salt: d.masking_salt.clone(),
        config_dir: d.config_dir.clone(),
        config_yaml,
        proxy_port: d.proxy_port,
        api_port: d.api_port,
    };

    let resp = tenet::deploy_tenet(&body, config).await?;
    let resolved_upstream = resolve_postgres_host_for_hints(&d.upstream_host).await;
    let connect_host = resp
        .host
        .clone()
        .or_else(|| d.client_host.clone())
        .or(resolved_upstream);
    let connect_host_str = connect_host.as_deref();
    let row = TenetDeployRow {
        job_id: resp.job_id.clone(),
        eval_id: resp.eval_id.clone(),
        alloc_id: resp.alloc_id.clone(),
        node_id: resp.node_id.clone(),
        host: connect_host
            .clone()
            .unwrap_or_else(|| "-".to_string()),
        proxy_port: resp
            .proxy_port
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string()),
        api_port: resp
            .api_port
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string()),
    };

    if output_format == OutputFormat::Table {
        println!("{} Tenet deploy submitted", "✅".green());
        print_row_or_json(row, output_format);
        for line in tenet_deploy_hint_lines(&resp, connect_host_str) {
            println!("{}", line);
        }
    } else {
        print_json(&resp);
    }
    Ok(())
}

async fn lifecycle(
    action: &str,
    job_id: &str,
    config: &Config,
    output_format: OutputFormat,
) -> Result<()> {
    let resp = match action {
        "start" => tenet::start_tenet(job_id, config).await?,
        "stop" => tenet::stop_tenet(job_id, config).await?,
        "purge" => tenet::purge_tenet(job_id, config).await?,
        _ => unreachable!(),
    };

    let message = resp
        .message
        .unwrap_or_else(|| format!("Tenet {} completed", action));
    let row = TenetLifecycleRow {
        job_id: job_id.to_string(),
        action: action.to_string(),
        message,
    };

    if output_format == OutputFormat::Table {
        println!("{} Tenet {}", "✅".green(), action);
        print_row_or_json(row, output_format);
    } else {
        print_json(&row);
    }
    Ok(())
}

async fn proxy_get(
    g: &TenetProxyGetArgs,
    config: &Config,
    output_format: OutputFormat,
) -> Result<()> {
    let yaml = tenet::get_proxy_yaml(&g.job_id, config).await?;

    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({
            "job_id": g.job_id,
            "proxy_yaml": yaml,
        }));
        return Ok(());
    }

    if let Some(path) = &g.out_file {
        let mut f = fs::File::create(path)
            .map_err(|e| TenetError::IoError(format!("Cannot create {}: {}", path.display(), e)))?;
        f.write_all(yaml.as_bytes())
            .map_err(|e| TenetError::IoError(format!("Cannot write {}: {}", path.display(), e)))?;
        println!(
            "{} Wrote proxy YAML to {}",
            "✅".green(),
            path.display().to_string().cyan()
        );
    } else {
        print!("{}", yaml);
        if !yaml.ends_with('\n') {
            println!();
        }
    }
    Ok(())
}

async fn proxy_set(
    s: &TenetProxySetArgs,
    config: &Config,
    output_format: OutputFormat,
) -> Result<()> {
    let yaml = fs::read_to_string(&s.proxy_config).map_err(|e| {
        TenetError::IoError(format!(
            "Cannot read --proxy-config {}: {}",
            s.proxy_config.display(),
            e
        ))
    })?;
    let apply = s.apply.unwrap_or(true);
    let resp = tenet::set_proxy_yaml(&s.job_id, &yaml, apply, config).await?;

    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({
            "job_id": s.job_id,
            "apply": apply,
            "message": resp.message,
            "status": "updated",
        }));
    } else {
        println!("{} Proxy config updated (apply={})", "✅".green(), apply);
        if let Some(m) = resp.message {
            println!("{}", m);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::tenet::TenetDeployRequest;
    use std::fs;

    #[test]
    fn tenet_deploy_hint_lines_psql_and_api() {
        let resp = TenetDeployResponse {
            job_id: "j".into(),
            eval_id: "e".into(),
            alloc_id: "a".into(),
            node_id: "n".into(),
            host: Some("10.0.0.2".into()),
            proxy_port: Some(6544),
            api_port: Some(3010),
        };
        let lines = tenet_deploy_hint_lines(&resp, Some("10.0.0.2"));
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("PGPASSWORD=<password>"));
        assert!(lines[0].contains("host=10.0.0.2") && lines[0].contains("port=6544"));
        assert_eq!(lines[1], "Tenet API: http://10.0.0.2:3010");
    }

    #[test]
    fn tenet_deploy_hint_lines_empty_without_connect_host() {
        let resp = TenetDeployResponse {
            job_id: "j".into(),
            eval_id: "e".into(),
            alloc_id: "a".into(),
            node_id: "n".into(),
            host: None,
            proxy_port: Some(6544),
            api_port: None,
        };
        assert!(tenet_deploy_hint_lines(&resp, None).is_empty());
    }

    #[test]
    fn tenet_deploy_hint_lines_use_explicit_connect_host() {
        let resp = TenetDeployResponse {
            job_id: "j".into(),
            eval_id: "e".into(),
            alloc_id: "a".into(),
            node_id: "n".into(),
            host: None,
            proxy_port: Some(6544),
            api_port: Some(3010),
        };
        let lines = tenet_deploy_hint_lines(&resp, Some("44.239.18.139"));
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("host=44.239.18.139") && lines[0].contains("port=6544"));
        assert_eq!(lines[1], "Tenet API: http://44.239.18.139:3010");
    }

    #[test]
    fn tenet_deploy_hint_lines_use_resolved_upstream_style_ip() {
        let resp = TenetDeployResponse {
            job_id: "j".into(),
            eval_id: "e".into(),
            alloc_id: "a".into(),
            node_id: "n".into(),
            host: None,
            proxy_port: Some(6544),
            api_port: None,
        };
        let lines = tenet_deploy_hint_lines(&resp, Some("10.0.4.20"));
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("host=10.0.4.20") && lines[0].contains("port=6544"));
    }

    #[tokio::test]
    async fn resolve_postgres_host_ipv4_literal_unchanged() {
        assert_eq!(
            resolve_postgres_host_for_hints("10.0.4.20").await,
            Some("10.0.4.20".into())
        );
        assert_eq!(
            resolve_postgres_host_for_hints("  127.0.0.1  ").await,
            Some("127.0.0.1".into())
        );
    }

    #[test]
    fn proxy_config_file_contents_become_config_yaml_field() {
        let dir = std::env::temp_dir().join(format!(
            "guepard-tenet-yaml-{}-{}",
            std::process::id(),
            line!()
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("proxy.yaml");
        fs::write(&path, "masking_enabled: true\nprotocol: postgres\n").unwrap();
        let yaml = fs::read_to_string(&path).unwrap();
        let body = TenetDeployRequest {
            tenant_id: "t".into(),
            compute_job_id: None,
            upstream_host: "h".into(),
            upstream_port: 5432,
            masking_salt: "s".into(),
            config_dir: None,
            config_yaml: yaml,
            proxy_port: None,
            api_port: None,
        };
        let j = serde_json::to_value(&body).unwrap();
        assert_eq!(
            j.get("config_yaml").and_then(|v| v.as_str()),
            Some("masking_enabled: true\nprotocol: postgres\n")
        );
        fs::remove_dir_all(&dir).ok();
    }
}
