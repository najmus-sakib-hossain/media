//! Configuration management for DX Media.
//!
//! Loads configuration from environment variables and .env files.

use crate::error::{DxError, Result};
use std::env;
use std::path::PathBuf;

/// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    // ─────────────────────────────────────────────────────────────
    // Directory Configuration
    // ─────────────────────────────────────────────────────────────
    /// Directory for downloaded media (also aliased as download_dir).
    pub media_dir: PathBuf,
    /// Directory for cache files.
    pub cache_dir: PathBuf,
    /// Directory for temporary files.
    pub temp_dir: PathBuf,
    /// Alias for media_dir, for convenience.
    pub download_dir: PathBuf,

    // ─────────────────────────────────────────────────────────────
    // Provider API Keys
    // ─────────────────────────────────────────────────────────────
    /// Unsplash API access key.
    pub unsplash_access_key: Option<String>,
    /// Pexels API key.
    pub pexels_api_key: Option<String>,
    /// Pixabay API key.
    pub pixabay_api_key: Option<String>,
    /// NASA API key.
    pub nasa_api_key: Option<String>,
    /// GitHub token.
    pub github_token: Option<String>,
    /// Giphy API key.
    pub giphy_api_key: Option<String>,

    // ─────────────────────────────────────────────────────────────
    // Download Settings
    // ─────────────────────────────────────────────────────────────
    /// Maximum concurrent downloads.
    pub concurrent_downloads: usize,
    /// Number of retry attempts.
    pub retry_attempts: u32,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Whether to respect rate limits.
    pub respect_rate_limits: bool,

    // ─────────────────────────────────────────────────────────────
    // Cache Settings
    // ─────────────────────────────────────────────────────────────
    /// Whether caching is enabled.
    pub cache_enabled: bool,
    /// Cache time-to-live in hours.
    pub cache_ttl_hours: u64,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// This will also load variables from a `.env` file if present.
    ///
    /// # Errors
    ///
    /// Returns an error if critical configuration is invalid.
    pub fn load() -> Result<Self> {
        // Load .env file if present (ignore errors)
        let _ = dotenvy::dotenv();

        let media_dir = Self::get_path("DX_MEDIA_DIR", "./media");
        
        Ok(Self {
            // Directories
            download_dir: media_dir.clone(),
            media_dir,
            cache_dir: Self::get_path("DX_CACHE_DIR", "./cache"),
            temp_dir: Self::get_path("DX_TEMP_DIR", "./temp"),

            // API Keys (all optional)
            unsplash_access_key: Self::get_optional("UNSPLASH_ACCESS_KEY"),
            pexels_api_key: Self::get_optional("PEXELS_API_KEY"),
            pixabay_api_key: Self::get_optional("PIXABAY_API_KEY"),
            nasa_api_key: Self::get_optional("NASA_API_KEY"),
            github_token: Self::get_optional("GITHUB_TOKEN"),
            giphy_api_key: Self::get_optional("GIPHY_API_KEY"),

            // Download settings
            concurrent_downloads: Self::get_usize("DX_CONCURRENT_DOWNLOADS", 5),
            retry_attempts: Self::get_u32("DX_RETRY_ATTEMPTS", 3),
            timeout_secs: Self::get_u64("DX_TIMEOUT_SECONDS", 300),
            respect_rate_limits: Self::get_bool("DX_RESPECT_RATE_LIMITS", true),

            // Cache settings
            cache_enabled: Self::get_bool("DX_CACHE_ENABLED", true),
            cache_ttl_hours: Self::get_u64("DX_CACHE_TTL_HOURS", 24),
        })
    }

    /// Create a default configuration for testing.
    #[must_use]
    pub fn default_for_testing() -> Self {
        let media_dir = PathBuf::from("./test_media");
        Self {
            download_dir: media_dir.clone(),
            media_dir,
            cache_dir: PathBuf::from("./test_cache"),
            temp_dir: PathBuf::from("./test_temp"),
            unsplash_access_key: None,
            pexels_api_key: None,
            pixabay_api_key: None,
            nasa_api_key: None,
            github_token: None,
            giphy_api_key: None,
            concurrent_downloads: 2,
            retry_attempts: 1,
            timeout_secs: 30,
            respect_rate_limits: true,
            cache_enabled: false,
            cache_ttl_hours: 1,
        }
    }

    /// Check if a specific provider has an API key configured.
    #[must_use]
    pub fn has_api_key(&self, provider: &str) -> bool {
        match provider.to_lowercase().as_str() {
            "unsplash" => self.unsplash_access_key.is_some(),
            "pexels" => self.pexels_api_key.is_some(),
            "pixabay" => self.pixabay_api_key.is_some(),
            "nasa" => self.nasa_api_key.is_some(),
            "github" => self.github_token.is_some(),
            "giphy" => self.giphy_api_key.is_some(),
            // Providers that don't need API keys
            "lorem_picsum" | "placeholder" | "dog_api" | "cat_api" => true,
            _ => false,
        }
    }

    /// Get API key for a provider.
    ///
    /// # Errors
    ///
    /// Returns `MissingApiKey` error if the provider requires a key but none is configured.
    pub fn get_api_key(&self, provider: &str) -> Result<&str> {
        let key = match provider.to_lowercase().as_str() {
            "unsplash" => self.unsplash_access_key.as_deref(),
            "pexels" => self.pexels_api_key.as_deref(),
            "pixabay" => self.pixabay_api_key.as_deref(),
            "nasa" => self.nasa_api_key.as_deref(),
            "github" => self.github_token.as_deref(),
            "giphy" => self.giphy_api_key.as_deref(),
            _ => None,
        };

        key.ok_or_else(|| DxError::MissingApiKey {
            provider: provider.to_string(),
            env_var: Self::env_var_for_provider(provider),
        })
    }

    /// Get the environment variable name for a provider's API key.
    fn env_var_for_provider(provider: &str) -> String {
        match provider.to_lowercase().as_str() {
            "unsplash" => "UNSPLASH_ACCESS_KEY".to_string(),
            "pexels" => "PEXELS_API_KEY".to_string(),
            "pixabay" => "PIXABAY_API_KEY".to_string(),
            "nasa" => "NASA_API_KEY".to_string(),
            "github" => "GITHUB_TOKEN".to_string(),
            "giphy" => "GIPHY_API_KEY".to_string(),
            _ => format!("{}_API_KEY", provider.to_uppercase()),
        }
    }

    // ─────────────────────────────────────────────────────────────
    // Helper Methods
    // ─────────────────────────────────────────────────────────────

    fn get_optional(key: &str) -> Option<String> {
        env::var(key).ok().filter(|s| !s.is_empty())
    }

    fn get_path(key: &str, default: &str) -> PathBuf {
        env::var(key)
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(default))
    }

    fn get_usize(key: &str, default: usize) -> usize {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_u32(key: &str, default: u32) -> u32 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_u64(key: &str, default: u64) -> u64 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_bool(key: &str, default: bool) -> bool {
        env::var(key)
            .ok()
            .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes"))
            .unwrap_or(default)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::load().unwrap_or_else(|_| Self::default_for_testing())
    }
}
