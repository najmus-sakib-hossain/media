//! Image processing tools.
//!
//! This module provides 10 image manipulation tools:
//! 1. Format Converter - Convert between image formats
//! 2. Smart Resizer - Resize with aspect ratio options
//! 3. Image Compressor - Reduce file size with quality control
//! 4. Watermarker - Add text/logo overlays
//! 5. EXIF Wiper - Remove metadata for privacy
//! 6. QR Code Generator/Reader - Create and decode QR codes
//! 7. Color Palette Extractor - Extract dominant colors
//! 8. Grayscale/Filter Applier - Apply visual effects
//! 9. OCR (Text Extractor) - Extract text from images
//! 10. Icon Generator - Generate favicon and app icons

mod converter;
mod resizer;
mod compressor;
mod watermark;
mod exif;
mod qrcode;
mod palette;
mod filters;
mod ocr;
mod icons;

pub use converter::*;
pub use resizer::*;
pub use compressor::*;
pub use watermark::*;
pub use exif::*;
pub use qrcode::*;
pub use palette::*;
pub use filters::*;
pub use ocr::*;
pub use icons::*;

use crate::error::Result;
use std::path::Path;

/// Image tools collection.
pub struct ImageTools;

impl ImageTools {
    /// Create a new ImageTools instance.
    pub fn new() -> Self {
        Self
    }
    
    /// Convert image format.
    pub fn convert<P: AsRef<Path>>(&self, input: P, output: P, format: ImageFormat) -> Result<super::ToolOutput> {
        converter::convert_image(input, output, format)
    }
    
    /// Resize image.
    pub fn resize<P: AsRef<Path>>(&self, input: P, output: P, options: ResizeOptions) -> Result<super::ToolOutput> {
        resizer::resize_image(input, output, options)
    }
    
    /// Compress image.
    pub fn compress<P: AsRef<Path>>(&self, input: P, output: P, quality: u8) -> Result<super::ToolOutput> {
        compressor::compress_image(input, output, quality)
    }
    
    /// Add watermark to image.
    pub fn watermark<P: AsRef<Path>>(&self, input: P, output: P, options: WatermarkOptions) -> Result<super::ToolOutput> {
        watermark::add_watermark(input, output, options)
    }
    
    /// Remove EXIF data from image.
    pub fn wipe_exif<P: AsRef<Path>>(&self, input: P, output: P) -> Result<super::ToolOutput> {
        exif::wipe_exif(input, output)
    }
    
    /// Generate QR code.
    pub fn generate_qr<P: AsRef<Path>>(&self, data: &str, output: P, size: u32) -> Result<super::ToolOutput> {
        qrcode::generate_qr(data, output, size)
    }
    
    /// Read QR code from image.
    pub fn read_qr<P: AsRef<Path>>(&self, input: P) -> Result<String> {
        qrcode::read_qr(input)
    }
    
    /// Extract color palette from image.
    pub fn extract_palette<P: AsRef<Path>>(&self, input: P, count: usize) -> Result<Vec<String>> {
        palette::extract_palette(input, count)
    }
    
    /// Apply filter to image.
    pub fn apply_filter<P: AsRef<Path>>(&self, input: P, output: P, filter: ImageFilter) -> Result<super::ToolOutput> {
        filters::apply_filter(input, output, filter)
    }
    
    /// Extract text from image (OCR).
    pub fn extract_text<P: AsRef<Path>>(&self, input: P) -> Result<String> {
        ocr::extract_text(input)
    }
    
    /// Generate icons from image.
    pub fn generate_icons<P: AsRef<Path>>(&self, input: P, output_dir: P) -> Result<super::ToolOutput> {
        icons::generate_icons(input, output_dir)
    }
}

impl Default for ImageTools {
    fn default() -> Self {
        Self::new()
    }
}
