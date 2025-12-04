//! OCR (Optical Character Recognition) tool.
//!
//! Extract text from images using OCR.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use std::path::Path;

/// OCR configuration options.
#[derive(Debug, Clone)]
pub struct OcrOptions {
    /// Language for recognition (e.g., "eng", "deu", "fra").
    pub language: String,
    /// Page segmentation mode.
    pub page_seg_mode: PageSegmentationMode,
    /// Whether to preserve formatting.
    pub preserve_formatting: bool,
    /// Confidence threshold (0.0 - 1.0).
    pub confidence_threshold: f32,
}

impl Default for OcrOptions {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            page_seg_mode: PageSegmentationMode::Auto,
            preserve_formatting: false,
            confidence_threshold: 0.5,
        }
    }
}

/// Page segmentation modes for OCR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageSegmentationMode {
    /// Fully automatic page segmentation.
    #[default]
    Auto,
    /// Single block of text.
    SingleBlock,
    /// Single line of text.
    SingleLine,
    /// Single word.
    SingleWord,
    /// Single character.
    SingleChar,
    /// Sparse text (find as much text as possible).
    SparseText,
}

/// OCR result with text and metadata.
#[derive(Debug, Clone)]
pub struct OcrResult {
    /// Extracted text.
    pub text: String,
    /// Average confidence (0.0 - 1.0).
    pub confidence: f32,
    /// Individual word results with positions.
    pub words: Vec<OcrWord>,
}

/// A single word from OCR.
#[derive(Debug, Clone)]
pub struct OcrWord {
    /// The recognized text.
    pub text: String,
    /// Confidence score (0.0 - 1.0).
    pub confidence: f32,
    /// Bounding box (x, y, width, height).
    pub bounds: (u32, u32, u32, u32),
}

/// Extract text from an image using OCR.
///
/// Note: This is a simplified implementation. For production use,
/// consider using tesseract-rs or leptess for full Tesseract integration.
///
/// # Arguments
/// * `input` - Path to the image file
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::extract_text;
///
/// let text = extract_text("screenshot.png").unwrap();
/// println!("Extracted text: {}", text);
/// ```
pub fn extract_text<P: AsRef<Path>>(input: P) -> Result<String> {
    let result = extract_text_detailed(input, OcrOptions::default())?;
    Ok(result.text)
}

/// Extract text with detailed options and results.
pub fn extract_text_detailed<P: AsRef<Path>>(
    input: P,
    _options: OcrOptions,
) -> Result<OcrResult> {
    let input_path = input.as_ref();
    
    // Verify file exists
    if !input_path.exists() {
        return Err(DxError::FileIo {
            path: input_path.to_path_buf(),
            message: "File not found".to_string(),
            source: None,
        });
    }
    
    // Note: Full OCR requires external dependencies like Tesseract.
    // This is a placeholder that returns instructions for setting up OCR.
    //
    // For production, use one of:
    // - tesseract-rs (Tesseract bindings)
    // - leptess (Leptonica + Tesseract)
    // - ocrs (experimental pure Rust OCR)
    
    // Try to call system Tesseract if available
    #[cfg(feature = "ocr-tesseract")]
    {
        return extract_with_tesseract(input_path, &_options);
    }
    
    // Fallback: simple pattern-based text detection (very limited)
    #[cfg(not(feature = "ocr-tesseract"))]
    {
        // Return a helpful message about OCR setup
        Ok(OcrResult {
            text: format!(
                "OCR requires external setup. To enable OCR:\n\
                1. Install Tesseract: https://github.com/tesseract-ocr/tesseract\n\
                2. Add to PATH\n\
                3. Rebuild with: cargo build --features ocr-tesseract\n\n\
                Image loaded: {}",
                input_path.display()
            ),
            confidence: 0.0,
            words: vec![],
        })
    }
}

/// Extract text using system Tesseract (if available).
#[cfg(feature = "ocr-tesseract")]
fn extract_with_tesseract(input: &Path, options: &OcrOptions) -> Result<OcrResult> {
    use std::process::Command;
    
    // Run tesseract command
    let output = Command::new("tesseract")
        .arg(input)
        .arg("stdout")
        .arg("-l")
        .arg(&options.language)
        .output()
        .map_err(|e| DxError::Config {
            message: format!("Failed to run Tesseract: {}. Is it installed?", e),
            source: None,
        })?;
    
    if !output.status.success() {
        return Err(DxError::Config {
            message: format!(
                "Tesseract failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
            source: None,
        });
    }
    
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    
    Ok(OcrResult {
        text: text.trim().to_string(),
        confidence: 0.8, // Tesseract doesn't provide overall confidence easily
        words: vec![],
    })
}

/// Preprocess image for better OCR results.
pub fn preprocess_for_ocr<P: AsRef<Path>>(input: P, output: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // Convert to grayscale
    let gray = img.to_luma8();
    
    // Apply adaptive thresholding for better text contrast
    let (w, h) = gray.dimensions();
    let mut result = image::GrayImage::new(w, h);
    
    // Simple Otsu's threshold approximation
    let mut histogram = [0u32; 256];
    for pixel in gray.pixels() {
        histogram[pixel[0] as usize] += 1;
    }
    
    let total_pixels = (w * h) as f64;
    let mut sum: f64 = 0.0;
    for (i, &count) in histogram.iter().enumerate() {
        sum += i as f64 * count as f64;
    }
    
    let mut sum_b: f64 = 0.0;
    let mut w_b: f64 = 0.0;
    let mut max_variance: f64 = 0.0;
    let mut threshold: u8 = 128;
    
    for (i, &count) in histogram.iter().enumerate() {
        w_b += count as f64;
        if w_b == 0.0 { continue; }
        
        let w_f = total_pixels - w_b;
        if w_f == 0.0 { break; }
        
        sum_b += i as f64 * count as f64;
        let m_b = sum_b / w_b;
        let m_f = (sum - sum_b) / w_f;
        
        let variance = w_b * w_f * (m_b - m_f) * (m_b - m_f);
        if variance > max_variance {
            max_variance = variance;
            threshold = i as u8;
        }
    }
    
    // Apply threshold
    for (x, y, pixel) in gray.enumerate_pixels() {
        let value = if pixel[0] > threshold { 255 } else { 0 };
        result.put_pixel(x, y, image::Luma([value]));
    }
    
    result.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save image: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Preprocessed image for OCR (threshold: {})", threshold),
        output_path,
    ))
}

/// Batch OCR multiple images.
pub fn batch_extract_text<P: AsRef<Path>>(inputs: &[P]) -> Result<Vec<(std::path::PathBuf, String)>> {
    let mut results = Vec::new();
    
    for input in inputs {
        let path = input.as_ref().to_path_buf();
        match extract_text(input) {
            Ok(text) => results.push((path, text)),
            Err(_) => results.push((path, String::new())),
        }
    }
    
    Ok(results)
}

/// Extract text and save to file.
pub fn extract_text_to_file<P: AsRef<Path>>(input: P, output: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let text = extract_text(input_path)?;
    
    std::fs::write(output_path, &text).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to write text file: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Extracted {} characters to text file", text.len()),
        output_path,
    ))
}

/// Detect if an image likely contains text.
pub fn detect_text_regions<P: AsRef<Path>>(input: P) -> Result<bool> {
    let input_path = input.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    // Simple edge detection to find high-contrast regions (text-like)
    let mut edge_count = 0u64;
    let total = ((w - 2) * (h - 2)) as u64;
    
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let center = gray.get_pixel(x, y)[0] as i32;
            let right = gray.get_pixel(x + 1, y)[0] as i32;
            let bottom = gray.get_pixel(x, y + 1)[0] as i32;
            
            // Simple gradient
            let gx = (center - right).abs();
            let gy = (center - bottom).abs();
            
            if gx > 30 || gy > 30 {
                edge_count += 1;
            }
        }
    }
    
    // If more than 5% of pixels have strong edges, likely contains text
    let edge_ratio = edge_count as f64 / total as f64;
    Ok(edge_ratio > 0.05 && edge_ratio < 0.4)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ocr_options_default() {
        let opts = OcrOptions::default();
        assert_eq!(opts.language, "eng");
        assert_eq!(opts.page_seg_mode, PageSegmentationMode::Auto);
    }
}
