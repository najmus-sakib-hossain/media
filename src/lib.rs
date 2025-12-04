//! # DX Media
//!
//! **Universal Digital Asset Acquisition Engine**
//!
//! One command. Any media. From anywhere.
//!
//! DX Media provides a unified interface to search and download digital assets:
//! - **13 FREE providers** (no API keys) with 890M+ assets
//! - **6 PREMIUM providers** (optional API keys) with 113M+ additional assets
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use dx_media::{DxMedia, MediaType};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let dx = DxMedia::new()?;
//!     
//!     // Search for images
//!     let results = dx.search("sunset")
//!         .media_type(MediaType::Image)
//!         .execute()
//!         .await?;
//!     println!("Found {} assets", results.total_count);
//!     
//!     // Download the first result
//!     if let Some(asset) = results.assets.first() {
//!         let path = dx.download(asset).await?;
//!         println!("Downloaded to: {:?}", path);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Universal Search**: One query syntax for all providers
//! - **Smart Downloads**: Async, parallel, with progress tracking
//! - **Rate Limiting**: Automatic throttling per provider
//! - **Dual Mode**: Use as CLI (`dx`) or Rust library
//! - **Graceful Degradation**: Premium providers work when keys are set, invisible otherwise
//! - **1B+ Assets**: Access to over 1 billion media assets

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::if_not_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unused_self)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_async)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::elidable_lifetime_names)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::double_ended_iterator_last)]

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

pub mod config;
pub mod engine;
pub mod error;
pub mod http;
pub mod providers;
pub mod scraping;
pub mod types;

// CLI module (not public - used by binary)
pub mod cli;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

pub use config::Config;
pub use engine::DxMedia;
pub use error::{DxError, Result};
pub use types::{License, MediaAsset, MediaType, SearchQuery, SearchResult};

// Re-export engine components
pub use engine::{Downloader, FileManager, Scraper, ScrapeOptions, ScrapeResult, SearchEngine};

// Re-export FREE providers (13 providers with 890M+ assets - NO API KEYS REQUIRED)
pub use providers::{
    // Tier 1: High-volume providers (700M+)
    OpenverseProvider, WikimediaCommonsProvider, EuropeanaProvider, DplaProvider,
    InternetArchiveProvider, LibraryOfCongressProvider,
    // Tier 2: Museum providers
    RijksmuseumProvider, MetMuseumProvider, NasaImagesProvider, 
    ClevelandMuseumProvider, ArtInstituteChicagoProvider,
    // Tier 3: 3D & Utility providers
    PolyHavenProvider, LoremPicsumProvider,
    // Registry
    ProviderRegistry,
};

// Re-export PREMIUM providers (6 providers with 113M+ assets - OPTIONAL API KEYS)
// These gracefully degrade when API keys are not configured
pub use providers::{
    UnsplashProvider,    // 5M+ high-quality photos
    PexelsProvider,      // 3.5M+ photos & videos
    PixabayProvider,     // 4.2M+ images, videos, music
    FreesoundProvider,   // 600K+ sound effects
    GiphyProvider,       // Millions of GIFs
    SmithsonianProvider, // 4.5M+ CC0 images
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent for API requests
pub const USER_AGENT: &str = concat!("dx-media/", env!("CARGO_PKG_VERSION"));
