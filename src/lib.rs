//! # DX Media
//!
//! **Universal Digital Asset Acquisition Engine**
//!
//! One command. Any media. From anywhere.
//!
//! DX Media provides a unified interface to search and download digital assets
//! from 50+ free API providers including Unsplash, Pexels, Pixabay, and more.
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
//! - **Universal Search**: One query syntax for 50+ providers
//! - **Smart Downloads**: Async, parallel, with progress tracking
//! - **Rate Limiting**: Automatic throttling per provider
//! - **Dual Mode**: Use as CLI (`dx`) or Rust library

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

pub mod config;
pub mod engine;
pub mod error;
pub mod http;
pub mod providers;
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
pub use engine::{Downloader, FileManager, SearchEngine};

// Re-export providers
pub use providers::{PexelsProvider, PixabayProvider, ProviderRegistry, UnsplashProvider};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent for API requests
pub const USER_AGENT: &str = concat!("dx-media/", env!("CARGO_PKG_VERSION"));
