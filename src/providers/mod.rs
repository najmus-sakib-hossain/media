//! Provider abstraction layer.
//!
//! This module defines the [`Provider`] trait and implementations for various
//! media asset providers. DX Media supports 13 FREE providers (no API keys required)
//! with access to 800+ million assets.

mod archive;
mod artic;
mod cleveland;
mod dpla;
mod europeana;
mod loc;
mod met;
mod nasa;
mod openverse;
mod picsum;
mod polyhaven;
mod registry;
mod rijksmuseum;
mod wikimedia;

/// Provider traits module.
pub mod traits;

pub use archive::InternetArchiveProvider;
pub use artic::ArtInstituteChicagoProvider;
pub use cleveland::ClevelandMuseumProvider;
pub use dpla::DplaProvider;
pub use europeana::EuropeanaProvider;
pub use loc::LibraryOfCongressProvider;
pub use met::MetMuseumProvider;
pub use nasa::NasaImagesProvider;
pub use openverse::OpenverseProvider;
pub use picsum::LoremPicsumProvider;
pub use polyhaven::PolyHavenProvider;
pub use registry::ProviderRegistry;
pub use rijksmuseum::RijksmuseumProvider;
pub use traits::{Provider, ProviderInfo};
pub use wikimedia::WikimediaCommonsProvider;
