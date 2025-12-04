//! Provider registry for managing all media providers.
//!
//! Supports both FREE providers (no API keys) and PREMIUM providers (optional API keys).
//! Premium providers gracefully degrade when API keys are not configured.

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::Config;
use crate::error::Result;
use crate::providers::traits::Provider;
use crate::providers::{
    ClevelandMuseumProvider,
    DplaProvider,
    EuropeanaProvider,
    FreesoundProvider,
    GiphyProvider,
    // FREE providers (no API key required)
    LibraryOfCongressProvider,
    LoremPicsumProvider,
    MetMuseumProvider,
    NasaImagesProvider,
    OpenverseProvider,
    PexelsProvider,
    PixabayProvider,
    PolyHavenProvider,
    RijksmuseumProvider,
    SmithsonianProvider,
    // PREMIUM providers (optional API key - graceful degradation)
    UnsplashProvider,
    WikimediaCommonsProvider,
};
use crate::types::{MediaType, SearchQuery, SearchResult};

/// Registry for managing and querying media providers.
///
/// ## FREE Providers (10) - No API Keys Required - 890M+ Assets
/// - Openverse: 700M+ images and audio (CC/CC0)
/// - Wikimedia Commons: 92M+ files
/// - Europeana: 50M+ European cultural heritage items
/// - DPLA: 40M+ American cultural heritage items (requires API key)
/// - Library of Congress: 3M+ public domain images
/// - Rijksmuseum: 700K+ Dutch masterpieces (CC0)
/// - Met Museum: 500K+ artworks (CC0)
/// - NASA: 140K+ space images
/// - Cleveland Museum: 61K+ artworks (CC0)
/// - Poly Haven: 3.7K+ 3D models, textures, HDRIs (CC0)
/// - Lorem Picsum: Unlimited placeholder images
///
/// ## PREMIUM Providers (7) - Optional API Keys - 113M+ Additional Assets
/// - Unsplash: 5M+ high-quality photos (free API key)
/// - Pexels: 3.5M+ photos & videos (free API key)
/// - Pixabay: 4.2M+ images, videos, music (free API key)
/// - Freesound: 600K+ sound effects (free API key)
/// - Giphy: Millions of GIFs (free API key)
/// - Smithsonian: 4.5M+ CC0 images (free API key)
/// - DPLA: 40M+ American cultural heritage items (free API key)
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
        // TIER 1: High-Volume Providers (700M+ assets) - NO API KEY REQUIRED
        // ═══════════════════════════════════════════════════════════════════

        // Openverse - 700M+ images and audio (no API key required)
        let openverse = OpenverseProvider::new(config);
        providers.insert(openverse.name().to_string(), Arc::new(openverse));

        // Wikimedia Commons - 92M+ files (no API key required)
        let wikimedia = WikimediaCommonsProvider::new(config);
        providers.insert(wikimedia.name().to_string(), Arc::new(wikimedia));

        // Europeana - 50M+ European cultural heritage items
        let europeana = EuropeanaProvider::new(config);
        providers.insert(europeana.name().to_string(), Arc::new(europeana));

        // DPLA - 40M+ American cultural heritage items (requires API key now)
        let dpla = DplaProvider::new(config);
        providers.insert(dpla.name().to_string(), Arc::new(dpla));

        // Library of Congress - 3M+ public domain images
        let loc = LibraryOfCongressProvider::new(config);
        providers.insert(loc.name().to_string(), Arc::new(loc));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 2: Museum Providers - NO API KEY REQUIRED
        // ═══════════════════════════════════════════════════════════════════

        // Rijksmuseum - 700K+ Dutch masterpieces (CC0)
        let rijksmuseum = RijksmuseumProvider::new(config);
        providers.insert(rijksmuseum.name().to_string(), Arc::new(rijksmuseum));

        // Met Museum - 500K+ artworks (no API key required)
        let met = MetMuseumProvider::new(config);
        providers.insert(met.name().to_string(), Arc::new(met));

        // NASA Images - 140K+ space images (no API key required)
        let nasa = NasaImagesProvider::new(config);
        providers.insert(nasa.name().to_string(), Arc::new(nasa));

        // Cleveland Museum - 61K+ artworks (CC0)
        let cleveland = ClevelandMuseumProvider::new(config);
        providers.insert(cleveland.name().to_string(), Arc::new(cleveland));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 3: 3D & Utility Providers - NO API KEY REQUIRED
        // ═══════════════════════════════════════════════════════════════════

        // Poly Haven - 3.7K+ 3D models, textures, HDRIs (CC0)
        let polyhaven = PolyHavenProvider::new(config);
        providers.insert(polyhaven.name().to_string(), Arc::new(polyhaven));

        // Lorem Picsum - Unlimited placeholder images (no API key required)
        let picsum = LoremPicsumProvider::new(config);
        providers.insert(picsum.name().to_string(), Arc::new(picsum));

        // ═══════════════════════════════════════════════════════════════════
        // TIER 4: PREMIUM Providers - OPTIONAL API KEY (Graceful Degradation)
        // These providers are only available when API keys are configured.
        // Without keys, they simply don't appear in search results.
        // ═══════════════════════════════════════════════════════════════════

        // Unsplash - 5M+ high-quality photos (free API key at unsplash.com/developers)
        let unsplash = UnsplashProvider::new(config);
        providers.insert(unsplash.name().to_string(), Arc::new(unsplash));

        // Pexels - 3.5M+ photos & videos (free API key at pexels.com/api)
        let pexels = PexelsProvider::new(config);
        providers.insert(pexels.name().to_string(), Arc::new(pexels));

        // Pixabay - 4.2M+ images, videos, music (free API key at pixabay.com/api/docs)
        let pixabay = PixabayProvider::new(config);
        providers.insert(pixabay.name().to_string(), Arc::new(pixabay));

        // Freesound - 600K+ sound effects (free API key at freesound.org/apiv2/apply)
        let freesound = FreesoundProvider::new(config);
        providers.insert(freesound.name().to_string(), Arc::new(freesound));

        // Giphy - Millions of GIFs (free API key at developers.giphy.com)
        let giphy = GiphyProvider::new(config);
        providers.insert(giphy.name().to_string(), Arc::new(giphy));

        // Smithsonian - 4.5M+ CC0 images (free API key at api.si.edu)
        let smithsonian = SmithsonianProvider::new(config);
        providers.insert(smithsonian.name().to_string(), Arc::new(smithsonian));

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
        let provider =
            self.get(provider_name)
                .ok_or_else(|| crate::error::DxError::ProviderApi {
                    provider: provider_name.to_string(),
                    message: "Provider not found".to_string(),
                    status_code: 404,
                })?;

        provider.search(query).await
    }

    /// Search all available providers and aggregate results.
    /// 
    /// This searches all providers **concurrently** for maximum performance.
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

        // Create futures for all provider searches
        let search_futures: Vec<_> = providers
            .iter()
            .map(|provider| {
                let provider = Arc::clone(provider);
                let query = query.clone();
                async move {
                    let name = provider.name().to_string();
                    let result = provider.search(&query).await;
                    (name, result)
                }
            })
            .collect();

        // Execute all searches concurrently
        let results = futures::future::join_all(search_futures).await;

        // Aggregate results
        let mut all_assets = Vec::new();
        let mut providers_searched = Vec::new();
        let mut provider_errors = Vec::new();
        let mut total_count = 0;

        for (provider_name, result) in results {
            providers_searched.push(provider_name.clone());

            match result {
                Ok(search_result) => {
                    total_count += search_result.total_count;
                    all_assets.extend(search_result.assets);
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

        // FREE providers should be registered (no API keys required)
        // Tier 1: High-volume providers
        assert!(registry.has_provider("openverse"));
        assert!(registry.has_provider("wikimedia"));
        assert!(registry.has_provider("europeana"));
        assert!(registry.has_provider("dpla"));
        assert!(registry.has_provider("loc"));

        // Tier 2: Museum providers
        assert!(registry.has_provider("rijksmuseum"));
        assert!(registry.has_provider("met"));
        assert!(registry.has_provider("nasa"));
        assert!(registry.has_provider("cleveland"));

        // Tier 3: 3D & Utility providers
        assert!(registry.has_provider("polyhaven"));
        assert!(registry.has_provider("picsum"));

        // PREMIUM providers (registered but not available without API keys)
        assert!(registry.has_provider("unsplash"));
        assert!(registry.has_provider("pexels"));
        assert!(registry.has_provider("pixabay"));
        assert!(registry.has_provider("freesound"));
        assert!(registry.has_provider("giphy"));
        assert!(registry.has_provider("smithsonian"));

        // Removed providers
        assert!(!registry.has_provider("archive"));
        assert!(!registry.has_provider("artic"));
        assert!(!registry.has_provider("nonexistent"));
    }

    #[test]
    fn test_provider_stats() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);

        let stats = registry.stats();
        // Total: 10 FREE + 7 PREMIUM (including DPLA) = 17 providers
        assert_eq!(stats.total, 17);
        // Without API keys, only 10 FREE providers are available
        // DPLA now requires an API key, so only 10 are available
        assert_eq!(stats.available, 10);
        // 7 premium providers are unavailable without API keys
        assert_eq!(stats.unavailable, 7);
    }

    #[test]
    fn test_get_provider() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);

        let provider = registry.get("openverse");
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().name(), "openverse");
    }
}
