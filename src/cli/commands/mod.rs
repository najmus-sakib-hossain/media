//! Command execution module.

mod download;
mod providers;
mod scrape;
mod search;

use crate::cli::args::{Args, Command};
use crate::error::Result;

/// Execute a CLI command.
pub async fn execute(args: Args) -> Result<()> {
    match args.command {
        Command::Search(search_args) => {
            search::execute(search_args, args.format, args.quiet).await
        }
        Command::Download(download_args) => {
            download::execute(download_args, args.quiet).await
        }
        Command::Scrape(scrape_args) => {
            scrape::execute(scrape_args, args.format, args.quiet).await
        }
        Command::Providers(provider_args) => {
            providers::execute(provider_args, args.format).await
        }
        Command::Config => {
            config_command(args.format).await
        }
        Command::Interactive => {
            interactive_command().await
        }
    }
}

/// Execute the config command.
async fn config_command(format: crate::cli::args::OutputFormat) -> Result<()> {
    use crate::DxMedia;
    use colored::Colorize;

    let dx = DxMedia::new()?;
    let config = dx.config();

    match format {
        crate::cli::args::OutputFormat::Json | crate::cli::args::OutputFormat::JsonCompact => {
            let json = serde_json::json!({
                "download_dir": config.download_dir,
                "timeout_secs": config.timeout_secs,
                "retry_attempts": config.retry_attempts,
                "providers": "All providers are FREE - no API keys required",
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        _ => {
            println!("{}", "DX Media Configuration".bold().cyan());
            println!();
            println!("  {} {}", "Download Directory:".dimmed(), config.download_dir.display());
            println!("  {} {} seconds", "Timeout:".dimmed(), config.timeout_secs);
            println!("  {} {}", "Retry Attempts:".dimmed(), config.retry_attempts);
            println!();
            println!("{}", "Providers:".bold());
            println!("  {} {}", "Status:".dimmed(), "All 6 providers are FREE - no API keys required!".green());
            println!("  {} Openverse, Wikimedia, NASA, Archive, Met Museum, Picsum", "Available:".dimmed());
        }
    }

    Ok(())
}

/// Execute the interactive command (placeholder).
async fn interactive_command() -> Result<()> {
    use colored::Colorize;

    println!("{}", "Interactive mode coming soon!".yellow());
    println!("For now, use the search and download commands.");
    
    Ok(())
}
