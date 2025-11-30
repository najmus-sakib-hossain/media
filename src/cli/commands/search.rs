//! Search command implementation.

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::args::{OutputFormat, SearchArgs};
use crate::cli::OutputFormatter;
use crate::error::Result;
use crate::types::SearchQuery;
use crate::DxMedia;

/// Execute the search command.
pub async fn execute(args: SearchArgs, format: OutputFormat, quiet: bool) -> Result<()> {
    let dx = DxMedia::new()?;

    // Build the search query
    let mut query = SearchQuery::new(args.query_string());
    query.count = args.count;
    query.page = args.page;
    query.media_type = args.media_type.and_then(Into::into);
    query.providers = args.providers.clone();
    query.orientation = args.orientation.map(Into::into);
    query.color = args.color.clone();

    // Show progress indicator
    let spinner = if !quiet && matches!(format, OutputFormat::Text) {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Searching for '{}'...", args.query_string()));
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(pb)
    } else {
        None
    };

    // Execute search
    let result = dx.search_query(&query).await?;

    // Clear spinner
    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    // Format and display results
    let formatter = OutputFormatter::new(format, quiet);
    formatter.format_search_results(&result)?;

    // Auto-download if requested
    if args.download && !result.assets.is_empty() {
        if !quiet {
            println!();
            println!("{}", "Downloading first result...".cyan());
        }

        let asset = &result.assets[0];
        let path = if let Some(ref output_dir) = args.output {
            dx.download_to(asset, std::path::Path::new(output_dir)).await?
        } else {
            dx.download(asset).await?
        };

        if !quiet {
            println!("{} {}", "Downloaded:".green(), path.display());
        }
    }

    Ok(())
}
