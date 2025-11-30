//! Provider registry for managing all media providers.

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::Config;
use crate::error::Result;
use crate::providers::traits::Provider;
use crate::providers::{PexelsProvider, PixabayProvider, UnsplashProvider};
use crate::types::{MediaType, SearchQuery, SearchResult};

/// Registry for managing and querying media providers.
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

        // Register all MVP providers
        let unsplash = UnsplashProvider::new(config);
        providers.insert(unsplash.name().to_string(), Arc::new(unsplash));

        let pexels = PexelsProvider::new(config);
        providers.insert(pexels.name().to_string(), Arc::new(pexels));

        let pixabay = PixabayProvider::new(config);
        providers.insert(pixabay.name().to_string(), Arc::new(pixabay));

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
        
        // All three MVP providers should be registered
        assert!(registry.has_provider("unsplash"));
        assert!(registry.has_provider("pexels"));
        assert!(registry.has_provider("pixabay"));
        assert!(!registry.has_provider("nonexistent"));
    }

    #[test]
    fn test_provider_stats() {
        let config = Config::default();
        let registry = ProviderRegistry::new(&config);
        
        let stats = registry.stats();
        assert_eq!(stats.total, 3);
        // Without API keys, none should be available
        assert_eq!(stats.available, 0);
        assert_eq!(stats.unavailable, 3);
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
