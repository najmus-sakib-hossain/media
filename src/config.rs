//! Configuration management for DX Media.
//!
//! Loads configuration from environment variables and .env files.

use crate::error::Result;
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
            concurrent_downloads: 2,
            retry_attempts: 1,
            timeout_secs: 30,
            respect_rate_limits: true,
            cache_enabled: false,
            cache_ttl_hours: 1,
        }
    }

    // ─────────────────────────────────────────────────────────────
    // Helper Methods
    // ─────────────────────────────────────────────────────────────

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
