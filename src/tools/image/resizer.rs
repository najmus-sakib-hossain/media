//! Image resizing tool.
//!
//! Provides smart resizing with aspect ratio preservation options.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::imageops::FilterType;
use std::path::Path;

/// Resize operation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    /// Resize to exact dimensions (may distort).
    Exact,
    /// Fit within dimensions preserving aspect ratio.
    Fit,
    /// Fill dimensions, cropping if necessary.
    Fill,
    /// Scale by percentage.
    Scale,
}

/// Resize filter algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizeFilter {
    /// Nearest neighbor (fastest, lowest quality).
    Nearest,
    /// Triangle/bilinear filtering.
    Triangle,
    /// Catmull-Rom cubic filtering.
    CatmullRom,
    /// Gaussian filtering.
    Gaussian,
    /// Lanczos3 filtering (best quality).
    #[default]
    Lanczos3,
}

impl ResizeFilter {
    fn to_filter_type(&self) -> FilterType {
        match self {
            Self::Nearest => FilterType::Nearest,
            Self::Triangle => FilterType::Triangle,
            Self::CatmullRom => FilterType::CatmullRom,
            Self::Gaussian => FilterType::Gaussian,
            Self::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

/// Options for resizing operations.
#[derive(Debug, Clone)]
pub struct ResizeOptions {
    /// Target width.
    pub width: u32,
    /// Target height.
    pub height: u32,
    /// Resize mode.
    pub mode: ResizeMode,
    /// Filter algorithm.
    pub filter: ResizeFilter,
    /// Scale percentage (only used with ResizeMode::Scale).
    pub scale_percent: Option<f32>,
}

impl ResizeOptions {
    /// Create options for exact resize.
    pub fn exact(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            mode: ResizeMode::Exact,
            filter: ResizeFilter::default(),
            scale_percent: None,
        }
    }
    
    /// Create options for fit resize (preserve aspect ratio).
    pub fn fit(max_width: u32, max_height: u32) -> Self {
        Self {
            width: max_width,
            height: max_height,
            mode: ResizeMode::Fit,
            filter: ResizeFilter::default(),
            scale_percent: None,
        }
    }
    
    /// Create options for fill resize (crop to fit).
    pub fn fill(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            mode: ResizeMode::Fill,
            filter: ResizeFilter::default(),
            scale_percent: None,
        }
    }
    
    /// Create options for scale by percentage.
    pub fn scale(percent: f32) -> Self {
        Self {
            width: 0,
            height: 0,
            mode: ResizeMode::Scale,
            filter: ResizeFilter::default(),
            scale_percent: Some(percent),
        }
    }
    
    /// Set the filter algorithm.
    pub fn with_filter(mut self, filter: ResizeFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            mode: ResizeMode::Fit,
            filter: ResizeFilter::default(),
            scale_percent: None,
        }
    }
}

/// Resize an image with the given options.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the output image
/// * `options` - Resize options
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::{resize_image, ResizeOptions};
///
/// // Resize to fit within 800x600 preserving aspect ratio
/// resize_image("input.jpg", "output.jpg", ResizeOptions::fit(800, 600)).unwrap();
///
/// // Scale to 50%
/// resize_image("input.jpg", "small.jpg", ResizeOptions::scale(50.0)).unwrap();
/// ```
pub fn resize_image<P: AsRef<Path>>(input: P, output: P, options: ResizeOptions) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let (orig_width, orig_height) = (img.width(), img.height());
    
    let resized = match options.mode {
        ResizeMode::Exact => {
            img.resize_exact(options.width, options.height, options.filter.to_filter_type())
        }
        ResizeMode::Fit => {
            img.resize(options.width, options.height, options.filter.to_filter_type())
        }
        ResizeMode::Fill => {
            img.resize_to_fill(options.width, options.height, options.filter.to_filter_type())
        }
        ResizeMode::Scale => {
            let scale = options.scale_percent.unwrap_or(100.0) / 100.0;
            let new_width = (orig_width as f32 * scale) as u32;
            let new_height = (orig_height as f32 * scale) as u32;
            img.resize_exact(new_width, new_height, options.filter.to_filter_type())
        }
    };
    
    resized.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save resized image: {}", e),
        source: None,
    })?;
    
    let (new_width, new_height) = (resized.width(), resized.height());
    
    Ok(ToolOutput::success_with_path(
        format!(
            "Resized {}x{} -> {}x{}",
            orig_width, orig_height, new_width, new_height
        ),
        output_path,
    )
    .with_metadata("original_width", orig_width.to_string())
    .with_metadata("original_height", orig_height.to_string())
    .with_metadata("new_width", new_width.to_string())
    .with_metadata("new_height", new_height.to_string()))
}

/// Create a thumbnail of an image.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the thumbnail
/// * `size` - Maximum dimension for thumbnail (width or height)
pub fn create_thumbnail<P: AsRef<Path>>(input: P, output: P, size: u32) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let thumbnail = img.thumbnail(size, size);
    
    thumbnail.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save thumbnail: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Created {}x{} thumbnail", thumbnail.width(), thumbnail.height()),
        output_path,
    ))
}

/// Batch resize multiple images.
pub fn batch_resize<P: AsRef<Path>>(
    inputs: &[P],
    output_dir: P,
    options: ResizeOptions,
) -> Result<ToolOutput> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create output directory: {}", e),
        source: None,
    })?;
    
    let mut resized = Vec::new();
    
    for input in inputs {
        let input_path = input.as_ref();
        let file_name = input_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("output.jpg");
        let output_path = output_dir.join(file_name);
        
        if resize_image(input_path, &output_path, options.clone()).is_ok() {
            resized.push(output_path);
        }
    }
    
    Ok(ToolOutput::success(format!("Resized {} images", resized.len()))
        .with_paths(resized))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resize_options() {
        let opts = ResizeOptions::fit(800, 600);
        assert_eq!(opts.width, 800);
        assert_eq!(opts.height, 600);
        assert_eq!(opts.mode, ResizeMode::Fit);
    }
    
    #[test]
    fn test_scale_options() {
        let opts = ResizeOptions::scale(50.0);
        assert_eq!(opts.mode, ResizeMode::Scale);
        assert_eq!(opts.scale_percent, Some(50.0));
    }
}
