//! Image compression tool.
//!
//! Reduces image file size with quality control.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use std::path::Path;
use std::io::BufWriter;
use std::fs::File;

/// Compression options for different formats.
#[derive(Debug, Clone)]
pub struct CompressionOptions {
    /// JPEG quality (1-100).
    pub jpeg_quality: u8,
    /// PNG compression level (0-9).
    pub png_compression: u8,
    /// WebP quality (1-100).
    pub webp_quality: u8,
    /// Whether to use lossless compression where available.
    pub lossless: bool,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            jpeg_quality: 85,
            png_compression: 6,
            webp_quality: 80,
            lossless: false,
        }
    }
}

impl CompressionOptions {
    /// Create options for high quality output.
    pub fn high_quality() -> Self {
        Self {
            jpeg_quality: 95,
            png_compression: 3,
            webp_quality: 90,
            lossless: false,
        }
    }
    
    /// Create options for maximum compression.
    pub fn max_compression() -> Self {
        Self {
            jpeg_quality: 60,
            png_compression: 9,
            webp_quality: 50,
            lossless: false,
        }
    }
    
    /// Create options with specific quality (1-100).
    pub fn with_quality(quality: u8) -> Self {
        let quality = quality.clamp(1, 100);
        Self {
            jpeg_quality: quality,
            png_compression: ((100 - quality) / 11).clamp(0, 9) as u8,
            webp_quality: quality,
            lossless: false,
        }
    }
}

/// Compress an image with quality settings.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the compressed output
/// * `quality` - Quality level (1-100, where 100 is best quality)
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::compress_image;
///
/// // Compress with 80% quality
/// compress_image("large.jpg", "small.jpg", 80).unwrap();
/// ```
pub fn compress_image<P: AsRef<Path>>(input: P, output: P, quality: u8) -> Result<ToolOutput> {
    compress_with_options(input, output, CompressionOptions::with_quality(quality))
}

/// Compress an image with detailed options.
pub fn compress_with_options<P: AsRef<Path>>(
    input: P,
    output: P,
    options: CompressionOptions,
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let input_size = std::fs::metadata(input_path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    // Determine format from output extension
    let extension = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();
    
    let file = File::create(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to create output file: {}", e),
        source: None,
    })?;
    let mut writer = BufWriter::new(file);
    
    match extension.as_str() {
        "jpg" | "jpeg" => {
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut writer,
                options.jpeg_quality,
            );
            img.write_with_encoder(encoder).map_err(|e| DxError::FileIo {
                path: output_path.to_path_buf(),
                message: format!("Failed to encode JPEG: {}", e),
                source: None,
            })?;
        }
        "png" => {
            let compression = match options.png_compression {
                0..=2 => image::codecs::png::CompressionType::Fast,
                3..=6 => image::codecs::png::CompressionType::Default,
                _ => image::codecs::png::CompressionType::Best,
            };
            let encoder = image::codecs::png::PngEncoder::new_with_quality(
                &mut writer,
                compression,
                image::codecs::png::FilterType::Adaptive,
            );
            img.write_with_encoder(encoder).map_err(|e| DxError::FileIo {
                path: output_path.to_path_buf(),
                message: format!("Failed to encode PNG: {}", e),
                source: None,
            })?;
        }
        "webp" => {
            // WebP encoding - save as PNG with conversion note
            // Note: Full WebP quality control requires webp crate
            img.save(output_path).map_err(|e| DxError::FileIo {
                path: output_path.to_path_buf(),
                message: format!("Failed to save WebP: {}", e),
                source: None,
            })?;
        }
        _ => {
            img.save(output_path).map_err(|e| DxError::FileIo {
                path: output_path.to_path_buf(),
                message: format!("Failed to save image: {}", e),
                source: None,
            })?;
        }
    }
    
    drop(writer);
    
    let output_size = std::fs::metadata(output_path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    let reduction = if input_size > 0 {
        ((input_size - output_size) as f64 / input_size as f64 * 100.0) as i32
    } else {
        0
    };
    
    Ok(ToolOutput::success_with_path(
        format!(
            "Compressed {} ({} bytes -> {} bytes, {}% reduction)",
            input_path.display(),
            input_size,
            output_size,
            reduction.max(0)
        ),
        output_path,
    )
    .with_metadata("input_size", input_size.to_string())
    .with_metadata("output_size", output_size.to_string())
    .with_metadata("reduction_percent", reduction.to_string()))
}

/// Batch compress multiple images.
pub fn batch_compress<P: AsRef<Path>>(
    inputs: &[P],
    output_dir: P,
    quality: u8,
) -> Result<ToolOutput> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create output directory: {}", e),
        source: None,
    })?;
    
    let mut compressed = Vec::new();
    let mut total_input_size: u64 = 0;
    let mut total_output_size: u64 = 0;
    
    for input in inputs {
        let input_path = input.as_ref();
        let file_name = input_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("output.jpg");
        let output_path = output_dir.join(file_name);
        
        if let Ok(input_meta) = std::fs::metadata(input_path) {
            total_input_size += input_meta.len();
        }
        
        if compress_image(input_path, &output_path, quality).is_ok() {
            if let Ok(output_meta) = std::fs::metadata(&output_path) {
                total_output_size += output_meta.len();
            }
            compressed.push(output_path);
        }
    }
    
    let reduction = if total_input_size > 0 {
        ((total_input_size - total_output_size) as f64 / total_input_size as f64 * 100.0) as i32
    } else {
        0
    };
    
    Ok(ToolOutput::success(format!(
        "Compressed {} images ({}% total reduction)",
        compressed.len(),
        reduction.max(0)
    ))
    .with_paths(compressed)
    .with_metadata("total_input_size", total_input_size.to_string())
    .with_metadata("total_output_size", total_output_size.to_string()))
}

/// Optimize an image in-place (overwrite with compressed version).
pub fn optimize_image<P: AsRef<Path>>(path: P, quality: u8) -> Result<ToolOutput> {
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp");
    
    let result = compress_image(path, &temp_path, quality)?;
    
    std::fs::rename(&temp_path, path).map_err(|e| DxError::FileIo {
        path: path.to_path_buf(),
        message: format!("Failed to replace original: {}", e),
        source: None,
    })?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression_options() {
        let opts = CompressionOptions::with_quality(80);
        assert_eq!(opts.jpeg_quality, 80);
        assert_eq!(opts.webp_quality, 80);
    }
    
    #[test]
    fn test_quality_clamping() {
        let opts = CompressionOptions::with_quality(150);
        assert_eq!(opts.jpeg_quality, 100);
        
        let opts = CompressionOptions::with_quality(0);
        assert_eq!(opts.jpeg_quality, 1);
    }
}
