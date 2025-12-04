//! Provider abstraction layer.
//!
//! This module defines the [`Provider`] trait and implementations for various
//! media asset providers. DX Media supports:
//! - **20+ FREE providers** (no API keys required) with 1B+ assets
//! - **7 PREMIUM providers** (optional API keys) with 113M+ additional assets
//!
//! Premium providers gracefully degrade - they simply don't appear in searches
//! when API keys are not configured.

// ═══════════════════════════════════════════════════════════════════════════════
// FREE PROVIDERS (No API Key Required)
// ═══════════════════════════════════════════════════════════════════════════════
mod archive; // 26M+ media items (images, video, audio, docs)
mod artic; // 50K+ CC0 artworks
mod catapi; // 60K+ cat images
mod cleveland;
mod dicebear; // Unlimited avatar generation
mod dogceo; // 20K+ dog images
mod dpla;
mod europeana;
mod loc;
mod met;
mod nasa;
mod openlibrary; // 30M+ book covers
mod openverse;
mod picsum;
mod polyhaven;
mod randomfox; // Unlimited fox images
mod rijksmuseum;
mod robohash; // Unlimited robot avatars
mod vanda; // 1.2M+ V&A Museum items
mod walters; // 25K+ Walters artworks
mod wikimedia;

// ═══════════════════════════════════════════════════════════════════════════════
// PREMIUM PROVIDERS (Optional API Key - Graceful Degradation)
// ═══════════════════════════════════════════════════════════════════════════════
mod freesound; // 600K+ sound effects (free API key)
mod giphy; // Millions of GIFs (free API key)
mod pexels; // 3.5M+ photos & videos (free API key)
mod pixabay; // 4.2M+ images, videos, music (free API key)
mod smithsonian;
mod unsplash; // 5M+ high-quality photos (free API key) // 4.5M+ CC0 images (free API key)

mod registry;

/// Provider traits module.
pub mod traits;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC EXPORTS - FREE PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════
pub use archive::InternetArchiveProvider;
pub use artic::ArtInstituteChicagoProvider;
pub use catapi::CatApiProvider;
pub use cleveland::ClevelandMuseumProvider;
pub use dicebear::DiceBearProvider;
pub use dogceo::DogCeoProvider;
pub use dpla::DplaProvider;
pub use europeana::EuropeanaProvider;
pub use loc::LibraryOfCongressProvider;
pub use met::MetMuseumProvider;
pub use nasa::NasaImagesProvider;
pub use openlibrary::OpenLibraryProvider;
pub use openverse::OpenverseProvider;
pub use picsum::LoremPicsumProvider;
pub use polyhaven::PolyHavenProvider;
pub use randomfox::RandomFoxProvider;
pub use rijksmuseum::RijksmuseumProvider;
pub use robohash::RoboHashProvider;
pub use vanda::VandAMuseumProvider;
pub use walters::WaltersArtMuseumProvider;
pub use wikimedia::WikimediaCommonsProvider;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC EXPORTS - PREMIUM PROVIDERS (Optional API Keys)
// ═══════════════════════════════════════════════════════════════════════════════
pub use freesound::FreesoundProvider;
pub use giphy::GiphyProvider;
pub use pexels::PexelsProvider;
pub use pixabay::PixabayProvider;
pub use smithsonian::SmithsonianProvider;
pub use unsplash::UnsplashProvider;

// ═══════════════════════════════════════════════════════════════════════════════
// REGISTRY & TRAITS
// ═══════════════════════════════════════════════════════════════════════════════
pub use registry::ProviderRegistry;
pub use traits::{Provider, ProviderInfo};
