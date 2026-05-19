use crate::application::dto::node::NodeResourceAvailability;
use crate::application::output::{print_json, OutputFormat};
use crate::application::services::node;
use crate::config::config::Config;
use crate::structure::NodeCommand;
use anyhow::Result;
use colored::Colorize;

pub fn node_output_format(command: &NodeCommand) -> bool {
    match command {
        NodeCommand::Resources { output, .. } => output.json,
    }
}

pub async fn run(command: &NodeCommand, config: &Config, output_format: OutputFormat) -> Result<()> {
    match command {
        NodeCommand::Resources { node_id, .. } => {
            let resources = node::get_node_resources(config, node_id.as_deref()).await?;
            print_node_resources(&resources, output_format);
        }
    }
    Ok(())
}

fn print_node_resources(resources: &NodeResourceAvailability, output_format: OutputFormat) {
    if output_format == OutputFormat::Json {
        print_json(resources);
        return;
    }

    let sched = if resources.schedulable {
        "yes".green().to_string()
    } else {
        "no".red().to_string()
    };

    println!(
        "{} {} ({}, {}) — schedulable: {}",
        "📦",
        resources.label_name.bold(),
        resources.node_type,
        resources.datacenter,
        sched
    );
    println!(
        "CPU available: {} MHz (total {})",
        resources.cpu.available_mhz, resources.cpu.total_mhz
    );
    println!(
        "Memory available: {} MB (total {})",
        resources.memory.available_mb, resources.memory.total_mb
    );
}
