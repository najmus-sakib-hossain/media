//! Image format conversion tool.
//!
//! Converts images between PNG, JPEG, WebP, GIF, BMP, ICO, and TIFF formats.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::{DynamicImage, ImageFormat as ImgFormat};
use std::path::Path;

/// Supported image formats for conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// PNG format (lossless).
    Png,
    /// JPEG format (lossy).
    Jpeg,
    /// WebP format (modern, efficient).
    WebP,
    /// GIF format (limited colors, animation support).
    Gif,
    /// BMP format (uncompressed).
    Bmp,
    /// ICO format (Windows icons).
    Ico,
    /// TIFF format (high quality).
    Tiff,
}

impl ImageFormat {
    /// Get the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::WebP => "webp",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Ico => "ico",
            Self::Tiff => "tiff",
        }
    }
    
    /// Convert to image crate's ImageFormat.
    pub fn to_image_format(&self) -> ImgFormat {
        match self {
            Self::Png => ImgFormat::Png,
            Self::Jpeg => ImgFormat::Jpeg,
            Self::WebP => ImgFormat::WebP,
            Self::Gif => ImgFormat::Gif,
            Self::Bmp => ImgFormat::Bmp,
            Self::Ico => ImgFormat::Ico,
            Self::Tiff => ImgFormat::Tiff,
        }
    }
    
    /// Parse format from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "webp" => Some(Self::WebP),
            "gif" => Some(Self::Gif),
            "bmp" => Some(Self::Bmp),
            "ico" => Some(Self::Ico),
            "tiff" | "tif" => Some(Self::Tiff),
            _ => None,
        }
    }
    
    /// Detect format from file extension.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_str)
    }
}

/// Convert an image from one format to another.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the output image
/// * `format` - Target format for conversion
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::{convert_image, ImageFormat};
///
/// convert_image("input.png", "output.jpg", ImageFormat::Jpeg).unwrap();
/// ```
pub fn convert_image<P: AsRef<Path>>(input: P, output: P, format: ImageFormat) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    // Load the image
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // Save in the target format
    img.save_with_format(output_path, format.to_image_format())
        .map_err(|e| DxError::FileIo {
            path: output_path.to_path_buf(),
            message: format!("Failed to save image: {}", e),
            source: None,
        })?;
    
    let input_size = std::fs::metadata(input_path)
        .map(|m| m.len())
        .unwrap_or(0);
    let output_size = std::fs::metadata(output_path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    Ok(ToolOutput::success_with_path(
        format!(
            "Converted {} to {} format ({} bytes -> {} bytes)",
            input_path.display(),
            format.extension(),
            input_size,
            output_size
        ),
        output_path,
    )
    .with_metadata("input_format", input_path.extension().and_then(|e| e.to_str()).unwrap_or("unknown"))
    .with_metadata("output_format", format.extension())
    .with_metadata("input_size", input_size.to_string())
    .with_metadata("output_size", output_size.to_string()))
}

/// Batch convert multiple images to a target format.
///
/// # Arguments
/// * `inputs` - List of input image paths
/// * `output_dir` - Directory to save converted images
/// * `format` - Target format for all images
pub fn batch_convert<P: AsRef<Path>>(
    inputs: &[P],
    output_dir: P,
    format: ImageFormat,
) -> Result<ToolOutput> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create output directory: {}", e),
        source: None,
    })?;
    
    let mut converted = Vec::new();
    let mut failed = Vec::new();
    
    for input in inputs {
        let input_path = input.as_ref();
        let file_stem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let output_path = output_dir.join(format!("{}.{}", file_stem, format.extension()));
        
        match convert_image(input_path, &output_path, format) {
            Ok(_) => converted.push(output_path),
            Err(e) => failed.push((input_path.to_path_buf(), e.to_string())),
        }
    }
    
    let message = if failed.is_empty() {
        format!("Successfully converted {} images to {} format", converted.len(), format.extension())
    } else {
        format!(
            "Converted {} images, {} failed",
            converted.len(),
            failed.len()
        )
    };
    
    Ok(ToolOutput::success(message).with_paths(converted))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_from_str() {
        assert_eq!(ImageFormat::from_str("png"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_str("JPG"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_str("jpeg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_str("webp"), Some(ImageFormat::WebP));
        assert_eq!(ImageFormat::from_str("invalid"), None);
    }
    
    #[test]
    fn test_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::WebP.extension(), "webp");
    }
}
