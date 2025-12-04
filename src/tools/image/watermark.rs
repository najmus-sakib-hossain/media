//! Watermark tool for images.
//!
//! Add text or image watermarks to images.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::{DynamicImage, Rgba, GenericImageView, GenericImage};
use std::path::Path;

/// Watermark position on the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WatermarkPosition {
    /// Top-left corner.
    TopLeft,
    /// Top-center.
    TopCenter,
    /// Top-right corner.
    TopRight,
    /// Center-left.
    CenterLeft,
    /// Center of image.
    Center,
    /// Center-right.
    CenterRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-center.
    #[default]
    BottomCenter,
    /// Bottom-right corner.
    BottomRight,
    /// Tile across entire image.
    Tile,
}

/// Watermark configuration options.
#[derive(Debug, Clone)]
pub struct WatermarkOptions {
    /// Text to overlay (if text watermark).
    pub text: Option<String>,
    /// Path to watermark image (if image watermark).
    pub image_path: Option<std::path::PathBuf>,
    /// Position of the watermark.
    pub position: WatermarkPosition,
    /// Opacity (0.0 - 1.0).
    pub opacity: f32,
    /// Font size for text watermarks.
    pub font_size: f32,
    /// Font color for text watermarks (RGBA).
    pub color: [u8; 4],
    /// Margin from edges in pixels.
    pub margin: u32,
    /// Scale factor for image watermarks (1.0 = original size).
    pub scale: f32,
    /// Rotation angle in degrees.
    pub rotation: f32,
}

impl Default for WatermarkOptions {
    fn default() -> Self {
        Self {
            text: None,
            image_path: None,
            position: WatermarkPosition::default(),
            opacity: 0.5,
            font_size: 24.0,
            color: [255, 255, 255, 200], // White with some transparency
            margin: 20,
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

impl WatermarkOptions {
    /// Create text watermark options.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }
    
    /// Create image watermark options.
    pub fn image<P: AsRef<Path>>(path: P) -> Self {
        Self {
            image_path: Some(path.as_ref().to_path_buf()),
            ..Default::default()
        }
    }
    
    /// Set watermark position.
    pub fn at(mut self, position: WatermarkPosition) -> Self {
        self.position = position;
        self
    }
    
    /// Set opacity (0.0 - 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    
    /// Set font size for text watermarks.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    
    /// Set color for text watermarks.
    pub fn with_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = [r, g, b, a];
        self
    }
    
    /// Set margin from edges.
    pub fn with_margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }
    
    /// Set scale for image watermarks.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// Add a watermark to an image.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the watermarked output
/// * `options` - Watermark configuration
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::{add_watermark, WatermarkOptions, WatermarkPosition};
///
/// // Add text watermark
/// let opts = WatermarkOptions::text("© 2024 My Company")
///     .at(WatermarkPosition::BottomRight)
///     .with_opacity(0.7);
/// add_watermark("photo.jpg", "watermarked.jpg", opts).unwrap();
/// ```
pub fn add_watermark<P: AsRef<Path>>(
    input: P,
    output: P,
    options: WatermarkOptions,
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let mut img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    if let Some(watermark_path) = &options.image_path {
        // Image watermark
        img = add_image_watermark(img, watermark_path, &options)?;
    } else if let Some(text) = &options.text {
        // Text watermark (simplified - draws colored rectangle with text pattern)
        img = add_text_watermark_simple(img, text, &options);
    } else {
        return Err(DxError::Config {
            message: "No watermark text or image specified".to_string(),
            source: None,
        });
    }
    
    img.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save watermarked image: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Added watermark to {}", input_path.display()),
        output_path,
    ))
}

/// Add an image watermark overlay.
fn add_image_watermark<P: AsRef<Path>>(
    mut base: DynamicImage,
    watermark_path: P,
    options: &WatermarkOptions,
) -> Result<DynamicImage> {
    let watermark = image::open(watermark_path.as_ref()).map_err(|e| DxError::FileIo {
        path: watermark_path.as_ref().to_path_buf(),
        message: format!("Failed to open watermark image: {}", e),
        source: None,
    })?;
    
    // Scale watermark if needed
    let watermark = if (options.scale - 1.0).abs() > 0.01 {
        let new_width = (watermark.width() as f32 * options.scale) as u32;
        let new_height = (watermark.height() as f32 * options.scale) as u32;
        watermark.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        watermark
    };
    
    let (base_w, base_h) = base.dimensions();
    let (wm_w, wm_h) = watermark.dimensions();
    
    // Calculate position
    let (x, y) = calculate_position(
        base_w, base_h,
        wm_w, wm_h,
        options.position,
        options.margin,
    );
    
    // Overlay with opacity
    let watermark_rgba = watermark.to_rgba8();
    let mut base_rgba = base.to_rgba8();
    
    for wy in 0..wm_h {
        for wx in 0..wm_w {
            let bx = x + wx;
            let by = y + wy;
            
            if bx < base_w && by < base_h {
                let wm_pixel = watermark_rgba.get_pixel(wx, wy);
                let base_pixel = base_rgba.get_pixel(bx, by);
                
                // Blend with opacity
                let alpha = (wm_pixel[3] as f32 / 255.0) * options.opacity;
                let inv_alpha = 1.0 - alpha;
                
                let blended = Rgba([
                    (wm_pixel[0] as f32 * alpha + base_pixel[0] as f32 * inv_alpha) as u8,
                    (wm_pixel[1] as f32 * alpha + base_pixel[1] as f32 * inv_alpha) as u8,
                    (wm_pixel[2] as f32 * alpha + base_pixel[2] as f32 * inv_alpha) as u8,
                    255,
                ]);
                
                base_rgba.put_pixel(bx, by, blended);
            }
        }
    }
    
    Ok(DynamicImage::ImageRgba8(base_rgba))
}

/// Add a simple text watermark (without external font dependencies).
fn add_text_watermark_simple(base: DynamicImage, text: &str, options: &WatermarkOptions) -> DynamicImage {
    let mut base_rgba = base.to_rgba8();
    let (base_w, base_h) = base_rgba.dimensions();
    
    // Simple text rendering: create a colored band with the text pattern
    // For production, use imageproc + rusttype for proper text rendering
    
    // Estimate text box size (rough approximation)
    let char_width = (options.font_size * 0.6) as u32;
    let text_width = (text.len() as u32 * char_width).min(base_w - options.margin * 2);
    let text_height = (options.font_size * 1.5) as u32;
    
    let (x, y) = calculate_position(
        base_w, base_h,
        text_width, text_height,
        options.position,
        options.margin,
    );
    
    // Draw a semi-transparent background for the watermark
    let bg_color = Rgba([0, 0, 0, (128.0 * options.opacity) as u8]);
    let fg_color = Rgba([
        options.color[0],
        options.color[1],
        options.color[2],
        (options.color[3] as f32 * options.opacity) as u8,
    ]);
    
    // Draw background
    for py in y.saturating_sub(5)..(y + text_height + 5).min(base_h) {
        for px in x.saturating_sub(10)..(x + text_width + 10).min(base_w) {
            let base_pixel = base_rgba.get_pixel(px, py);
            let alpha = bg_color[3] as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;
            let blended = Rgba([
                (bg_color[0] as f32 * alpha + base_pixel[0] as f32 * inv_alpha) as u8,
                (bg_color[1] as f32 * alpha + base_pixel[1] as f32 * inv_alpha) as u8,
                (bg_color[2] as f32 * alpha + base_pixel[2] as f32 * inv_alpha) as u8,
                255,
            ]);
            base_rgba.put_pixel(px, py, blended);
        }
    }
    
    // Draw simple text pattern (placeholder for proper text rendering)
    // Each character is represented by a simple block
    for (i, _ch) in text.chars().enumerate() {
        let cx = x + (i as u32 * char_width);
        if cx + char_width > base_w { break; }
        
        // Draw character block
        for py in y..(y + text_height.min(base_h - y)) {
            for px in cx..(cx + char_width - 2).min(base_w) {
                if (py - y) > 3 && (py - y) < text_height - 3 {
                    let base_pixel = base_rgba.get_pixel(px, py);
                    let alpha = fg_color[3] as f32 / 255.0;
                    let inv_alpha = 1.0 - alpha;
                    let blended = Rgba([
                        (fg_color[0] as f32 * alpha + base_pixel[0] as f32 * inv_alpha) as u8,
                        (fg_color[1] as f32 * alpha + base_pixel[1] as f32 * inv_alpha) as u8,
                        (fg_color[2] as f32 * alpha + base_pixel[2] as f32 * inv_alpha) as u8,
                        255,
                    ]);
                    base_rgba.put_pixel(px, py, blended);
                }
            }
        }
    }
    
    DynamicImage::ImageRgba8(base_rgba)
}

/// Calculate watermark position based on alignment.
fn calculate_position(
    base_w: u32, base_h: u32,
    wm_w: u32, wm_h: u32,
    position: WatermarkPosition,
    margin: u32,
) -> (u32, u32) {
    match position {
        WatermarkPosition::TopLeft => (margin, margin),
        WatermarkPosition::TopCenter => ((base_w - wm_w) / 2, margin),
        WatermarkPosition::TopRight => (base_w - wm_w - margin, margin),
        WatermarkPosition::CenterLeft => (margin, (base_h - wm_h) / 2),
        WatermarkPosition::Center => ((base_w - wm_w) / 2, (base_h - wm_h) / 2),
        WatermarkPosition::CenterRight => (base_w - wm_w - margin, (base_h - wm_h) / 2),
        WatermarkPosition::BottomLeft => (margin, base_h - wm_h - margin),
        WatermarkPosition::BottomCenter => ((base_w - wm_w) / 2, base_h - wm_h - margin),
        WatermarkPosition::BottomRight => (base_w - wm_w - margin, base_h - wm_h - margin),
        WatermarkPosition::Tile => (0, 0), // Special case handled separately
    }
}

/// Add tiled watermark across the entire image.
pub fn add_tiled_watermark<P: AsRef<Path>>(
    input: P,
    output: P,
    watermark_path: P,
    opacity: f32,
    spacing: u32,
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let base = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let watermark = image::open(watermark_path.as_ref()).map_err(|e| DxError::FileIo {
        path: watermark_path.as_ref().to_path_buf(),
        message: format!("Failed to open watermark: {}", e),
        source: None,
    })?;
    
    let (base_w, base_h) = base.dimensions();
    let (wm_w, wm_h) = watermark.dimensions();
    let watermark_rgba = watermark.to_rgba8();
    let mut base_rgba = base.to_rgba8();
    
    // Tile watermark across image
    let mut y = 0u32;
    while y < base_h {
        let mut x = 0u32;
        while x < base_w {
            // Overlay watermark at this position
            for wy in 0..wm_h {
                for wx in 0..wm_w {
                    let bx = x + wx;
                    let by = y + wy;
                    
                    if bx < base_w && by < base_h {
                        let wm_pixel = watermark_rgba.get_pixel(wx, wy);
                        let base_pixel = base_rgba.get_pixel(bx, by);
                        
                        let alpha = (wm_pixel[3] as f32 / 255.0) * opacity;
                        let inv_alpha = 1.0 - alpha;
                        
                        let blended = Rgba([
                            (wm_pixel[0] as f32 * alpha + base_pixel[0] as f32 * inv_alpha) as u8,
                            (wm_pixel[1] as f32 * alpha + base_pixel[1] as f32 * inv_alpha) as u8,
                            (wm_pixel[2] as f32 * alpha + base_pixel[2] as f32 * inv_alpha) as u8,
                            255,
                        ]);
                        
                        base_rgba.put_pixel(bx, by, blended);
                    }
                }
            }
            x += wm_w + spacing;
        }
        y += wm_h + spacing;
    }
    
    let result = DynamicImage::ImageRgba8(base_rgba);
    result.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save image: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        "Added tiled watermark",
        output_path,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_watermark_options() {
        let opts = WatermarkOptions::text("Test")
            .at(WatermarkPosition::BottomRight)
            .with_opacity(0.5);
        
        assert_eq!(opts.text, Some("Test".to_string()));
        assert_eq!(opts.position, WatermarkPosition::BottomRight);
        assert!((opts.opacity - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_position_calculation() {
        let (x, y) = calculate_position(1000, 800, 100, 50, WatermarkPosition::BottomRight, 20);
        assert_eq!(x, 880); // 1000 - 100 - 20
        assert_eq!(y, 730); // 800 - 50 - 20
    }
}
