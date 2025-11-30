//! Web scraper for extracting media from any website.
//!
//! This module provides functionality to scrape images, videos, and other media
//! from arbitrary web pages without requiring API keys.

use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

use crate::error::{DxError, Result};
use crate::http::HttpClient;
use crate::types::{MediaAsset, MediaType, License};

/// Web scraper for extracting media from websites.
#[derive(Debug, Clone)]
pub struct Scraper {
    client: HttpClient,
    max_depth: usize,
    follow_links: bool,
}

/// Options for scraping.
#[derive(Debug, Clone)]
pub struct ScrapeOptions {
    /// Maximum depth to follow links (0 = only the given URL).
    pub max_depth: usize,
    /// File pattern to match (glob-like, e.g., "*.jpg").
    pub pattern: Option<String>,
    /// Media types to extract.
    pub media_types: Vec<MediaType>,
    /// Maximum number of assets to find.
    pub max_assets: usize,
}

impl Default for ScrapeOptions {
    fn default() -> Self {
        Self {
            max_depth: 0,
            pattern: None,
            media_types: vec![MediaType::Image],
            max_assets: 100,
        }
    }
}

/// Result of a scrape operation.
#[derive(Debug, Clone)]
pub struct ScrapeResult {
    /// Source URL that was scraped.
    pub source_url: String,
    /// Extracted media assets.
    pub assets: Vec<MediaAsset>,
    /// Number of pages scraped.
    pub pages_scraped: usize,
    /// Any errors encountered (non-fatal).
    pub errors: Vec<String>,
}

impl Scraper {
    /// Create a new scraper with default settings.
    pub fn new() -> Result<Self> {
        let client = HttpClient::new()?;
        Ok(Self {
            client,
            max_depth: 0,
            follow_links: false,
        })
    }

    /// Set the maximum depth for following links.
    #[must_use]
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self.follow_links = depth > 0;
        self
    }

    /// Scrape media from a URL.
    pub async fn scrape(&self, url: &str, options: &ScrapeOptions) -> Result<ScrapeResult> {
        let base_url = Url::parse(url).map_err(|e| DxError::InvalidQuery {
            message: format!("Invalid URL: {e}"),
        })?;

        let mut result = ScrapeResult {
            source_url: url.to_string(),
            assets: Vec::new(),
            pages_scraped: 0,
            errors: Vec::new(),
        };

        let mut visited: HashSet<String> = HashSet::new();
        self.scrape_page(&base_url, options, &mut result, &mut visited, 0).await?;

        Ok(result)
    }

    /// Scrape a single page.
    async fn scrape_page(
        &self,
        url: &Url,
        options: &ScrapeOptions,
        result: &mut ScrapeResult,
        visited: &mut HashSet<String>,
        depth: usize,
    ) -> Result<()> {
        let url_str = url.to_string();
        
        // Skip if already visited or too deep
        if visited.contains(&url_str) || depth > options.max_depth {
            return Ok(());
        }
        visited.insert(url_str.clone());

        // Fetch the page
        let response = match self.client.get_raw(&url_str).await {
            Ok(r) => r,
            Err(e) => {
                result.errors.push(format!("Failed to fetch {url_str}: {e}"));
                return Ok(());
            }
        };

        let html = match response.text().await {
            Ok(t) => t,
            Err(e) => {
                result.errors.push(format!("Failed to read {url_str}: {e}"));
                return Ok(());
            }
        };

        result.pages_scraped += 1;

        // Parse HTML
        let document = Html::parse_document(&html);

        // Extract images
        if options.media_types.contains(&MediaType::Image) {
            self.extract_images(&document, url, options, result);
        }

        // Extract videos
        if options.media_types.contains(&MediaType::Video) {
            self.extract_videos(&document, url, options, result);
        }

        // Extract audio
        if options.media_types.contains(&MediaType::Audio) {
            self.extract_audio(&document, url, options, result);
        }

        // Stop if we have enough assets
        if result.assets.len() >= options.max_assets {
            return Ok(());
        }

        // Follow links if configured
        if self.follow_links && depth < options.max_depth {
            let links = self.extract_links(&document, url);
            for link in links {
                if result.assets.len() >= options.max_assets {
                    break;
                }
                Box::pin(self.scrape_page(&link, options, result, visited, depth + 1)).await?;
            }
        }

        Ok(())
    }

    /// Extract image URLs from HTML.
    fn extract_images(&self, document: &Html, base_url: &Url, options: &ScrapeOptions, result: &mut ScrapeResult) {
        // Select img tags
        let img_selector = Selector::parse("img[src]").unwrap();
        
        for (idx, element) in document.select(&img_selector).enumerate() {
            if result.assets.len() >= options.max_assets {
                break;
            }

            if let Some(src) = element.value().attr("src") {
                if let Some(asset) = self.create_image_asset(src, base_url, &element, idx, options) {
                    result.assets.push(asset);
                }
            }

            // Also check srcset
            if let Some(srcset) = element.value().attr("srcset") {
                for src in self.parse_srcset(srcset) {
                    if result.assets.len() >= options.max_assets {
                        break;
                    }
                    if let Some(asset) = self.create_image_asset(&src, base_url, &element, idx, options) {
                        result.assets.push(asset);
                    }
                }
            }
        }

        // Also look for background images in style attributes
        let style_selector = Selector::parse("[style*='background']").unwrap();
        let url_regex = Regex::new(r#"url\(['"]?([^'")\s]+)['"]?\)"#).unwrap();

        for (idx, element) in document.select(&style_selector).enumerate() {
            if result.assets.len() >= options.max_assets {
                break;
            }

            if let Some(style) = element.value().attr("style") {
                for cap in url_regex.captures_iter(style) {
                    if let Some(url_match) = cap.get(1) {
                        if let Some(asset) = self.create_image_asset(url_match.as_str(), base_url, &element, idx + 1000, options) {
                            result.assets.push(asset);
                        }
                    }
                }
            }
        }

        // Look for og:image and other meta images
        let meta_selector = Selector::parse("meta[property='og:image'], meta[name='twitter:image']").unwrap();
        for (idx, element) in document.select(&meta_selector).enumerate() {
            if result.assets.len() >= options.max_assets {
                break;
            }
            if let Some(content) = element.value().attr("content") {
                if let Some(asset) = self.create_image_asset(content, base_url, &element, idx + 2000, options) {
                    result.assets.push(asset);
                }
            }
        }
    }

    /// Extract video URLs from HTML.
    fn extract_videos(&self, document: &Html, base_url: &Url, options: &ScrapeOptions, result: &mut ScrapeResult) {
        // Select video and source tags
        let video_selector = Selector::parse("video[src], video source[src]").unwrap();

        for (idx, element) in document.select(&video_selector).enumerate() {
            if result.assets.len() >= options.max_assets {
                break;
            }

            if let Some(src) = element.value().attr("src") {
                if let Some(asset) = self.create_video_asset(src, base_url, idx, options) {
                    result.assets.push(asset);
                }
            }
        }
    }

    /// Extract audio URLs from HTML.
    fn extract_audio(&self, document: &Html, base_url: &Url, options: &ScrapeOptions, result: &mut ScrapeResult) {
        let audio_selector = Selector::parse("audio[src], audio source[src]").unwrap();

        for (idx, element) in document.select(&audio_selector).enumerate() {
            if result.assets.len() >= options.max_assets {
                break;
            }

            if let Some(src) = element.value().attr("src") {
                if let Some(asset) = self.create_audio_asset(src, base_url, idx, options) {
                    result.assets.push(asset);
                }
            }
        }
    }

    /// Create an image asset from a URL.
    fn create_image_asset(
        &self,
        src: &str,
        base_url: &Url,
        element: &scraper::ElementRef,
        idx: usize,
        options: &ScrapeOptions,
    ) -> Option<MediaAsset> {
        // Resolve relative URLs
        let absolute_url = base_url.join(src).ok()?;
        let url_str = absolute_url.to_string();

        // Skip data URLs and tiny images
        if url_str.starts_with("data:") {
            return None;
        }

        // Check pattern if specified
        if let Some(ref pattern) = options.pattern {
            if !self.matches_pattern(&url_str, pattern) {
                return None;
            }
        }

        // Skip non-image extensions
        let ext = self.get_extension(&url_str)?;
        if !["jpg", "jpeg", "png", "gif", "webp", "svg", "avif", "bmp"].contains(&ext.as_str()) {
            return None;
        }

        // Get alt text for title
        let alt = element.value().attr("alt").unwrap_or("").to_string();
        let title = if alt.is_empty() {
            format!("Image {}", idx + 1)
        } else {
            alt
        };

        // Get dimensions if available
        let width = element.value().attr("width").and_then(|w| w.parse().ok());
        let height = element.value().attr("height").and_then(|h| h.parse().ok());

        Some(MediaAsset::builder()
            .id(format!("scraped-{idx}"))
            .provider("scraper")
            .media_type(MediaType::Image)
            .title(title)
            .download_url(url_str.clone())
            .source_url(base_url.to_string())
            .license(License::Other("Unknown - Check source".to_string()))
            .dimensions(width.unwrap_or(0), height.unwrap_or(0))
            .build())
    }

    /// Create a video asset from a URL.
    fn create_video_asset(
        &self,
        src: &str,
        base_url: &Url,
        idx: usize,
        options: &ScrapeOptions,
    ) -> Option<MediaAsset> {
        let absolute_url = base_url.join(src).ok()?;
        let url_str = absolute_url.to_string();

        if let Some(ref pattern) = options.pattern {
            if !self.matches_pattern(&url_str, pattern) {
                return None;
            }
        }

        let ext = self.get_extension(&url_str)?;
        if !["mp4", "webm", "mov", "avi", "mkv", "m4v"].contains(&ext.as_str()) {
            return None;
        }

        Some(MediaAsset::builder()
            .id(format!("scraped-video-{idx}"))
            .provider("scraper")
            .media_type(MediaType::Video)
            .title(format!("Video {}", idx + 1))
            .download_url(url_str.clone())
            .source_url(base_url.to_string())
            .license(License::Other("Unknown - Check source".to_string()))
            .build())
    }

    /// Create an audio asset from a URL.
    fn create_audio_asset(
        &self,
        src: &str,
        base_url: &Url,
        idx: usize,
        options: &ScrapeOptions,
    ) -> Option<MediaAsset> {
        let absolute_url = base_url.join(src).ok()?;
        let url_str = absolute_url.to_string();

        if let Some(ref pattern) = options.pattern {
            if !self.matches_pattern(&url_str, pattern) {
                return None;
            }
        }

        let ext = self.get_extension(&url_str)?;
        if !["mp3", "wav", "ogg", "flac", "aac", "m4a"].contains(&ext.as_str()) {
            return None;
        }

        Some(MediaAsset::builder()
            .id(format!("scraped-audio-{idx}"))
            .provider("scraper")
            .media_type(MediaType::Audio)
            .title(format!("Audio {}", idx + 1))
            .download_url(url_str.clone())
            .source_url(base_url.to_string())
            .license(License::Other("Unknown - Check source".to_string()))
            .build())
    }

    /// Extract links from HTML for recursive scraping.
    fn extract_links(&self, document: &Html, base_url: &Url) -> Vec<Url> {
        let link_selector = Selector::parse("a[href]").unwrap();
        let mut links = Vec::new();

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(url) = base_url.join(href) {
                    // Only follow links to the same domain
                    if url.host() == base_url.host() {
                        links.push(url);
                    }
                }
            }
        }

        links
    }

    /// Parse srcset attribute to get URLs.
    fn parse_srcset(&self, srcset: &str) -> Vec<String> {
        srcset
            .split(',')
            .filter_map(|entry| {
                entry.trim().split_whitespace().next().map(String::from)
            })
            .collect()
    }

    /// Get file extension from URL.
    fn get_extension(&self, url: &str) -> Option<String> {
        let path = url.split('?').next()?;
        let filename = path.rsplit('/').next()?;
        let ext = filename.rsplit('.').next()?;
        Some(ext.to_lowercase())
    }

    /// Check if URL matches a glob-like pattern.
    fn matches_pattern(&self, url: &str, pattern: &str) -> bool {
        // Simple glob matching: * matches anything
        let regex_pattern = pattern
            .replace('.', r"\.")
            .replace('*', ".*");
        
        if let Ok(regex) = Regex::new(&regex_pattern) {
            regex.is_match(url)
        } else {
            url.contains(pattern)
        }
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            client: HttpClient::new().unwrap(),
            max_depth: 0,
            follow_links: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_srcset() {
        let scraper = Scraper::default();
        let srcset = "image-300.jpg 300w, image-600.jpg 600w, image-1200.jpg 1200w";
        let urls = scraper.parse_srcset(srcset);
        
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0], "image-300.jpg");
        assert_eq!(urls[1], "image-600.jpg");
        assert_eq!(urls[2], "image-1200.jpg");
    }

    #[test]
    fn test_get_extension() {
        let scraper = Scraper::default();
        
        assert_eq!(scraper.get_extension("https://example.com/image.jpg"), Some("jpg".to_string()));
        assert_eq!(scraper.get_extension("https://example.com/image.PNG"), Some("png".to_string()));
        assert_eq!(scraper.get_extension("https://example.com/image.jpg?size=large"), Some("jpg".to_string()));
    }

    #[test]
    fn test_matches_pattern() {
        let scraper = Scraper::default();
        
        assert!(scraper.matches_pattern("https://example.com/image.jpg", "*.jpg"));
        assert!(scraper.matches_pattern("https://example.com/image.png", "*.png"));
        assert!(!scraper.matches_pattern("https://example.com/image.gif", "*.jpg"));
    }
}
