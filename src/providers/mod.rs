//! Provider abstraction layer.
//!
//! This module defines the [`Provider`] trait and implementations for various
//! media asset providers. DX Media supports 10+ providers with access to 1+ billion assets.

mod archive;
mod freesound;
mod giphy;
mod met;
mod nasa;
mod openverse;
mod pexels;
mod picsum;
mod pixabay;
mod registry;
mod smithsonian;
mod unsplash;
mod wikimedia;

/// Provider traits module.
pub mod traits;

pub use archive::InternetArchiveProvider;
pub use freesound::FreesoundProvider;
pub use giphy::GiphyProvider;
pub use met::MetMuseumProvider;
pub use nasa::NasaImagesProvider;
pub use openverse::OpenverseProvider;
pub use pexels::PexelsProvider;
pub use picsum::LoremPicsumProvider;
pub use pixabay::PixabayProvider;
pub use registry::ProviderRegistry;
pub use smithsonian::SmithsonianProvider;
pub use traits::{Provider, ProviderInfo};
pub use unsplash::UnsplashProvider;
pub use wikimedia::WikimediaCommonsProvider;
