use crate::config::config::Config;
use crate::application::services::login;
use crate::domain::errors::login_error::LoginError;
use crate::structure::LoginArgs;
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

use crate::application::output::OutputFormat;

use crate::application::output::{OutputFormat, print_json};

pub async fn execute(args: &LoginArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    // If code is provided, save it directly
    if let Some(token) = &args.code {
        return execute_direct_login(token, config, output_format).await;
    }
    
    // Otherwise, proceed with interactive login
    if output_format == OutputFormat::Json {
        return Err(anyhow::anyhow!("Interactive login not supported with --json flag. Please use --code <token> for machine-readable login."));
    }
    
    // Step 1: Start login and get URL
    println!("{}", "Starting login process... 🐆".cyan());
    
    let url = login::start_login(config).await
        .map_err(|e| LoginError::ApiError(format!("Failed to start login: {}", e)))?;
    
    println!("{}", "Login URL generated successfully!".green());
    println!("{} {}", "URL:".yellow(), url);
    
    // Step 2: Ask user to press Enter to open browser
    print!("{}", "Press Enter to open the login page in your browser... ".cyan());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Step 3: Try to open the URL in the default browser
    if let Err(e) = open_url(&url) {
        println!("{} {}", "Could not automatically open browser:".yellow(), e);
        println!("{}", "Please manually open the URL above in your browser.".yellow());
    }
    
    // Step 4: Prompt for verification code
    print!("{}", "Enter the verification code from the webpage: ".cyan());
    io::stdout().flush()?;
    
    let mut verification_code = String::new();
    io::stdin().read_line(&mut verification_code)?;
    verification_code = verification_code.trim().to_string();
    
    if verification_code.is_empty() {
        return Err(anyhow::anyhow!("Verification code cannot be empty"));
    }
    
    // Step 5: Complete login
    println!("{}", "Completing login...".cyan());
    login::complete_login(config, &verification_code).await?;
    
    println!(
        "{} {}",
        "Login successful.".green(),
        "Happy coding! 🐆".yellow().bold()
    );
    println!(
        "You can now use the Guepard CLI to interact with your Guepard account.🐆"
    );
    println!(
        "{}",
        "To get started, run: `guepard --help`"
    );
    
    Ok(())
}

async fn execute_direct_login(token: &str, _config: &Config, output_format: OutputFormat) -> Result<()> {
    if output_format == OutputFormat::Table {
        println!("{}", "Saving access token directly... 🐆".cyan());
    }
    
    // Save the token directly without API calls
    crate::config::config::save_jwt_token_direct(token)
        .map_err(|e| LoginError::SessionError(e.to_string()))?;
    
    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({ "status": "success", "message": "Login successful" }));
    } else {
        println!(
            "{} {}",
            "Login successful.".green(),
            "Happy coding! 🐆".yellow().bold()
        );
        println!(
            "You can now use the Guepard CLI to interact with your Guepard account.🐆"
        );
        println!(
            "{}",
            "To get started, run: `guepard --help`"
        );
    }
    
    Ok(())
}

fn open_url(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).output()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(url).output()?;
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", url]).output()?;
    }
    
    Ok(())
}