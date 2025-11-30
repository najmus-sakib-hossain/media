//! Provider abstraction layer.
//!
//! This module defines the [`Provider`] trait and implementations for various
//! media asset providers.

mod pexels;
mod pixabay;
mod registry;
mod unsplash;

/// Provider traits module.
pub mod traits;

pub use pexels::PexelsProvider;
pub use pixabay::PixabayProvider;
pub use registry::ProviderRegistry;
pub use traits::{Provider, ProviderInfo};
pub use unsplash::UnsplashProvider;
