//! Provider registry for managing all media providers.

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::Config;
use crate::error::Result;
use crate::providers::traits::Provider;
use crate::providers::{
    InternetArchiveProvider, FreesoundProvider, GiphyProvider, LoremPicsumProvider,
    MetMuseumProvider, NasaImagesProvider, OpenverseProvider, PexelsProvider, 
    PixabayProvider, SmithsonianProvider, UnsplashProvider, WikimediaCommonsProvider,
};
use crate::types::{MediaType, SearchQuery, SearchResult};

/// Registry for managing and querying media providers.
/// 
/// Provides access to 12+ providers with 1+ billion total assets:
/// - Openverse: 700M+ images and audio (CC/CC0)
/// - Wikimedia Commons: 92M+ files
/// - Internet Archive: Millions of items
/// - Unsplash: 5M+ photos
/// - Pexels: 3.5M+ photos and videos
/// - Pixabay: 4.2M+ images and videos
/// - NASA: 140K+ space images
/// - Smithsonian: 4.5M+ museum items (CC0)
/// - Met Museum: 500K+ artworks (CC0)
/// - Freesound: 600K+ sound effects
/// - Giphy: Millions of GIFs
/// - Lorem Picsum: Unlimited placeholder images
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,
}

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderRegistry")
            .field("providers", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ProviderRegistry {
    /// Create a new registry with all available providers.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        let mut providers: HashMap<String, Arc<dyn Provider>> = HashMap::new();

        // ═══════════════════════════════════════════════════════════════════
        // TIER 1: High-Volume API Providers (700M+ assets)
        // ═══════════════════════════════════════════════════════════════════
        
        // Openverse - 700M+ images and audio (no API key required)
        let openverse = OpenverseProvider::new(config);
        providers.insert(openverse.name().to_string(), Arc::new(openverse));

        // Wikimedia Commons - 92M+ files (no API key required)
        let wikimedia = WikimediaCommonsProvider::new(config);
        providers.insert(wikimedia.name().to_string(), Arc::new(wikimedia));

        // Internet Archive - Millions of items (no API key required)
        let archive = InternetArchiveProvider::new(config);
        providers.insert(archive.name().to_string(), Arc::new(archive));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 2: Popular Stock Photo Providers (12M+ assets)
        // ═══════════════════════════════════════════════════════════════════
        
        // Unsplash - 5M+ photos
        let unsplash = UnsplashProvider::new(config);
        providers.insert(unsplash.name().to_string(), Arc::new(unsplash));

        // Pexels - 3.5M+ photos and videos
        let pexels = PexelsProvider::new(config);
        providers.insert(pexels.name().to_string(), Arc::new(pexels));

        // Pixabay - 4.2M+ images, videos, vectors
        let pixabay = PixabayProvider::new(config);
        providers.insert(pixabay.name().to_string(), Arc::new(pixabay));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 3: Specialized Providers (5M+ assets)
        // ═══════════════════════════════════════════════════════════════════
        
        // NASA Images - 140K+ space images (no API key required)
        let nasa = NasaImagesProvider::new(config);
        providers.insert(nasa.name().to_string(), Arc::new(nasa));

        // Smithsonian - 4.5M+ museum items
        let smithsonian = SmithsonianProvider::new(config);
        providers.insert(smithsonian.name().to_string(), Arc::new(smithsonian));

        // Met Museum - 500K+ artworks (no API key required)
        let met = MetMuseumProvider::new(config);
        providers.insert(met.name().to_string(), Arc::new(met));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 4: Audio & GIF Providers (600K+ assets)
        // ═══════════════════════════════════════════════════════════════════
        
        // Freesound - 600K+ sound effects
        let freesound = FreesoundProvider::new(config);
        providers.insert(freesound.name().to_string(), Arc::new(freesound));

        // Giphy - Millions of GIFs
        let giphy = GiphyProvider::new(config);
        providers.insert(giphy.name().to_string(), Arc::new(giphy));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 5: Utility Providers (Unlimited)
        // ═══════════════════════════════════════════════════════════════════
        
        // Lorem Picsum - Unlimited placeholder images (no API key required)
        let picsum = LoremPicsumProvider::new(config);
        providers.insert(picsum.name().to_string(), Arc::new(picsum));

        Self { providers }
    }

    /// Get a provider by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.get(name).cloned()
    }

    /// Get all registered providers.
    #[must_use]
    pub fn all(&self) -> Vec<Arc<dyn Provider>> {
        self.providers.values().cloned().collect()
    }

    /// Get all available providers (with valid API keys).
    #[must_use]
    pub fn available(&self) -> Vec<Arc<dyn Provider>> {
        self.providers
            .values()
            .filter(|p| p.is_available())
            .cloned()
            .collect()
    }

    /// Get providers that support a specific media type.
    #[must_use]
    pub fn for_media_type(&self, media_type: MediaType) -> Vec<Arc<dyn Provider>> {
        self.providers
            .values()
            .filter(|p| p.is_available() && p.supported_media_types().contains(&media_type))
            .cloned()
            .collect()
    }

    /// Get the names of all registered providers.
    #[must_use]
    pub fn provider_names(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Get the names of all available providers.
    #[must_use]
    pub fn available_provider_names(&self) -> Vec<String> {
        self.providers
            .iter()
            .filter(|(_, p)| p.is_available())
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Check if a provider exists by name.
    #[must_use]
    pub fn has_provider(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }

    /// Search a specific provider.
    pub async fn search_provider(
        &self,
        provider_name: &str,
        query: &SearchQuery,
    ) -> Result<SearchResult> {
        let provider = self.get(provider_name).ok_or_else(|| {
            crate::error::DxError::ProviderApi {
                provider: provider_name.to_string(),
                message: "Provider not found".to_string(),
                status_code: 404,
            }
        })?;

        provider.search(query).await
    }

    /// Search all available providers and aggregate results.
    pub async fn search_all(&self, query: &SearchQuery) -> Result<SearchResult> {
        let providers = match query.media_type {
            Some(media_type) => self.for_media_type(media_type),
            None => self.available(),
        };

        if providers.is_empty() {
            return Err(crate::error::DxError::NoResults {
                query: query.query.clone(),
            });
        }

        let mut all_assets = Vec::new();
        let mut providers_searched = Vec::new();
        let mut provider_errors = Vec::new();
        let mut total_count = 0;

        // Search each provider sequentially (parallel would need more complexity)
        for provider in providers {
            let provider_name = provider.name().to_string();
            providers_searched.push(provider_name.clone());

            match provider.search(query).await {
                Ok(result) => {
                    total_count += result.total_count;
                    all_assets.extend(result.assets);
                }
                Err(e) => {
                    provider_errors.push((provider_name, e.to_string()));
                }
            }
        }

        Ok(SearchResult {
            query: query.query.clone(),
            media_type: query.media_type,
            total_count,
            assets: all_assets,
            providers_searched,
            provider_errors,
            duration_ms: 0,
        })
    }

    /// Get provider count statistics.
    #[must_use]
    pub fn stats(&self) -> ProviderStats {
        let total = self.providers.len();
        let available = self.providers.values().filter(|p| p.is_available()).count();
        let unavailable = total - available;

        ProviderStats {
            total,
            available,
            unavailable,
        }
    }
}

/// Statistics about registered providers.
#[derive(Debug, Clone, Copy)]
pub struct ProviderStats {
    /// Total number of registered providers.
    pub total: usize,
    /// Number of available providers (with valid API keys).
    pub available: usize,
    /// Number of unavailable providers.
    pub unavailable: usize,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new(&Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);
        
        // All providers should be registered
        assert!(registry.has_provider("unsplash"));
        assert!(registry.has_provider("pexels"));
        assert!(registry.has_provider("pixabay"));
        assert!(registry.has_provider("openverse"));
        assert!(registry.has_provider("wikimedia"));
        assert!(registry.has_provider("nasa"));
        assert!(registry.has_provider("archive"));
        assert!(registry.has_provider("met"));
        assert!(registry.has_provider("picsum"));
        assert!(!registry.has_provider("nonexistent"));
    }

    #[test]
    fn test_provider_stats() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);
        
        let stats = registry.stats();
        // We now have 12 providers
        assert_eq!(stats.total, 12);
        // Providers without API key requirements should be available:
        // openverse, wikimedia, archive, nasa, met, picsum = 6
        assert!(stats.available >= 6);
    }

    #[test]
    fn test_get_provider() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);
        
        let provider = registry.get("unsplash");
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().name(), "unsplash");
    }
}
