//! Image filter and effects tool.
//!
//! Apply visual effects like grayscale, blur, contrast, etc.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;

/// Available image filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFilter {
    /// Convert to grayscale.
    Grayscale,
    /// Invert colors (negative).
    Invert,
    /// Apply sepia tone.
    Sepia,
    /// Gaussian blur (requires sigma parameter).
    Blur,
    /// Sharpen image.
    Sharpen,
    /// Increase contrast.
    ContrastIncrease,
    /// Decrease contrast.
    ContrastDecrease,
    /// Increase brightness.
    Brighten,
    /// Decrease brightness.
    Darken,
    /// Apply vintage/retro effect.
    Vintage,
    /// Apply cool (blue) tone.
    CoolTone,
    /// Apply warm (orange) tone.
    WarmTone,
    /// High contrast black and white (threshold).
    Threshold,
    /// Emboss effect.
    Emboss,
    /// Edge detection.
    EdgeDetect,
    /// Pixelate effect.
    Pixelate,
    /// Flip horizontally.
    FlipHorizontal,
    /// Flip vertically.
    FlipVertical,
    /// Rotate 90 degrees clockwise.
    Rotate90,
    /// Rotate 180 degrees.
    Rotate180,
    /// Rotate 270 degrees (90 counter-clockwise).
    Rotate270,
}

impl ImageFilter {
    /// Get a description of the filter.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Grayscale => "Convert to grayscale",
            Self::Invert => "Invert colors (negative)",
            Self::Sepia => "Apply sepia/vintage tone",
            Self::Blur => "Apply Gaussian blur",
            Self::Sharpen => "Sharpen the image",
            Self::ContrastIncrease => "Increase contrast",
            Self::ContrastDecrease => "Decrease contrast",
            Self::Brighten => "Increase brightness",
            Self::Darken => "Decrease brightness",
            Self::Vintage => "Apply vintage effect",
            Self::CoolTone => "Apply cool blue tone",
            Self::WarmTone => "Apply warm orange tone",
            Self::Threshold => "Black/white threshold",
            Self::Emboss => "Emboss effect",
            Self::EdgeDetect => "Detect edges",
            Self::Pixelate => "Pixelate effect",
            Self::FlipHorizontal => "Flip horizontally",
            Self::FlipVertical => "Flip vertically",
            Self::Rotate90 => "Rotate 90° clockwise",
            Self::Rotate180 => "Rotate 180°",
            Self::Rotate270 => "Rotate 270° clockwise",
        }
    }
    
    /// List all available filters.
    pub fn all() -> &'static [ImageFilter] {
        &[
            Self::Grayscale,
            Self::Invert,
            Self::Sepia,
            Self::Blur,
            Self::Sharpen,
            Self::ContrastIncrease,
            Self::ContrastDecrease,
            Self::Brighten,
            Self::Darken,
            Self::Vintage,
            Self::CoolTone,
            Self::WarmTone,
            Self::Threshold,
            Self::Emboss,
            Self::EdgeDetect,
            Self::Pixelate,
            Self::FlipHorizontal,
            Self::FlipVertical,
            Self::Rotate90,
            Self::Rotate180,
            Self::Rotate270,
        ]
    }
}

/// Filter options for customization.
#[derive(Debug, Clone)]
pub struct FilterOptions {
    /// Blur sigma (for blur filter).
    pub blur_sigma: f32,
    /// Contrast adjustment (-100 to 100).
    pub contrast: i32,
    /// Brightness adjustment (-100 to 100).
    pub brightness: i32,
    /// Threshold value (0-255, for threshold filter).
    pub threshold: u8,
    /// Pixel size (for pixelate filter).
    pub pixel_size: u32,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            blur_sigma: 2.0,
            contrast: 20,
            brightness: 20,
            threshold: 128,
            pixel_size: 10,
        }
    }
}

/// Apply a filter to an image.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the filtered output
/// * `filter` - Filter to apply
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::{apply_filter, ImageFilter};
///
/// apply_filter("photo.jpg", "grayscale.jpg", ImageFilter::Grayscale).unwrap();
/// apply_filter("photo.jpg", "vintage.jpg", ImageFilter::Vintage).unwrap();
/// ```
pub fn apply_filter<P: AsRef<Path>>(
    input: P,
    output: P,
    filter: ImageFilter,
) -> Result<ToolOutput> {
    apply_filter_with_options(input, output, filter, FilterOptions::default())
}

/// Apply a filter with custom options.
pub fn apply_filter_with_options<P: AsRef<Path>>(
    input: P,
    output: P,
    filter: ImageFilter,
    options: FilterOptions,
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let result = match filter {
        ImageFilter::Grayscale => DynamicImage::ImageLuma8(img.to_luma8()).to_rgba8(),
        ImageFilter::Invert => {
            let mut rgba = img.to_rgba8();
            for pixel in rgba.pixels_mut() {
                pixel[0] = 255 - pixel[0];
                pixel[1] = 255 - pixel[1];
                pixel[2] = 255 - pixel[2];
            }
            rgba
        }
        ImageFilter::Sepia => apply_sepia(&img),
        ImageFilter::Blur => {
            image::imageops::blur(&img, options.blur_sigma).to_rgba8()
        }
        ImageFilter::Sharpen => apply_sharpen(&img),
        ImageFilter::ContrastIncrease => adjust_contrast(&img, options.contrast),
        ImageFilter::ContrastDecrease => adjust_contrast(&img, -options.contrast),
        ImageFilter::Brighten => adjust_brightness(&img, options.brightness),
        ImageFilter::Darken => adjust_brightness(&img, -options.brightness),
        ImageFilter::Vintage => apply_vintage(&img),
        ImageFilter::CoolTone => apply_color_tone(&img, 0.9, 1.0, 1.1),
        ImageFilter::WarmTone => apply_color_tone(&img, 1.1, 1.0, 0.9),
        ImageFilter::Threshold => apply_threshold(&img, options.threshold),
        ImageFilter::Emboss => apply_emboss(&img),
        ImageFilter::EdgeDetect => apply_edge_detect(&img),
        ImageFilter::Pixelate => apply_pixelate(&img, options.pixel_size),
        ImageFilter::FlipHorizontal => image::imageops::flip_horizontal(&img).to_rgba8(),
        ImageFilter::FlipVertical => image::imageops::flip_vertical(&img).to_rgba8(),
        ImageFilter::Rotate90 => image::imageops::rotate90(&img).to_rgba8(),
        ImageFilter::Rotate180 => image::imageops::rotate180(&img).to_rgba8(),
        ImageFilter::Rotate270 => image::imageops::rotate270(&img).to_rgba8(),
    };
    
    let result_img = DynamicImage::ImageRgba8(result);
    result_img.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save image: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Applied {} filter", filter.description()),
        output_path,
    ))
}

/// Apply sepia tone effect.
fn apply_sepia(img: &DynamicImage) -> image::RgbaImage {
    let mut rgba = img.to_rgba8();
    for pixel in rgba.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        
        let tr = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
        let tg = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
        let tb = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;
        
        pixel[0] = tr;
        pixel[1] = tg;
        pixel[2] = tb;
    }
    rgba
}

/// Apply sharpening filter.
fn apply_sharpen(img: &DynamicImage) -> image::RgbaImage {
    // Simple unsharp mask approximation
    let blurred = image::imageops::blur(img, 1.0);
    let mut rgba = img.to_rgba8();
    let blur_rgba = blurred.to_rgba8();
    
    for (i, (orig, blur)) in rgba.pixels_mut().zip(blur_rgba.pixels()).enumerate() {
        for c in 0..3 {
            let diff = orig[c] as i32 - blur[c] as i32;
            orig[c] = (orig[c] as i32 + diff).clamp(0, 255) as u8;
        }
    }
    rgba
}

/// Adjust image contrast.
fn adjust_contrast(img: &DynamicImage, amount: i32) -> image::RgbaImage {
    let factor = (259.0 * (amount as f32 + 255.0)) / (255.0 * (259.0 - amount as f32));
    let mut rgba = img.to_rgba8();
    
    for pixel in rgba.pixels_mut() {
        for c in 0..3 {
            let value = ((factor * (pixel[c] as f32 - 128.0)) + 128.0).clamp(0.0, 255.0);
            pixel[c] = value as u8;
        }
    }
    rgba
}

/// Adjust image brightness.
fn adjust_brightness(img: &DynamicImage, amount: i32) -> image::RgbaImage {
    let mut rgba = img.to_rgba8();
    
    for pixel in rgba.pixels_mut() {
        for c in 0..3 {
            pixel[c] = (pixel[c] as i32 + amount).clamp(0, 255) as u8;
        }
    }
    rgba
}

/// Apply vintage/retro effect.
fn apply_vintage(img: &DynamicImage) -> image::RgbaImage {
    let sepia = apply_sepia(img);
    let mut rgba = sepia;
    
    // Add slight vignette and fade
    let (w, h) = (rgba.width(), rgba.height());
    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;
    let max_dist = (cx * cx + cy * cy).sqrt();
    
    for (x, y, pixel) in rgba.enumerate_pixels_mut() {
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;
        let dist = (dx * dx + dy * dy).sqrt() / max_dist;
        let vignette = 1.0 - (dist * 0.5);
        
        for c in 0..3 {
            pixel[c] = (pixel[c] as f32 * vignette).clamp(0.0, 255.0) as u8;
        }
    }
    
    // Add slight fade
    for pixel in rgba.pixels_mut() {
        for c in 0..3 {
            pixel[c] = (pixel[c] as f32 * 0.9 + 25.0).clamp(0.0, 255.0) as u8;
        }
    }
    
    rgba
}

/// Apply color tone adjustment.
fn apply_color_tone(img: &DynamicImage, r_mult: f32, g_mult: f32, b_mult: f32) -> image::RgbaImage {
    let mut rgba = img.to_rgba8();
    
    for pixel in rgba.pixels_mut() {
        pixel[0] = (pixel[0] as f32 * r_mult).clamp(0.0, 255.0) as u8;
        pixel[1] = (pixel[1] as f32 * g_mult).clamp(0.0, 255.0) as u8;
        pixel[2] = (pixel[2] as f32 * b_mult).clamp(0.0, 255.0) as u8;
    }
    rgba
}

/// Apply threshold (high contrast B&W).
fn apply_threshold(img: &DynamicImage, threshold: u8) -> image::RgbaImage {
    let gray = img.to_luma8();
    let mut rgba = image::RgbaImage::new(gray.width(), gray.height());
    
    for (x, y, pixel) in gray.enumerate_pixels() {
        let value = if pixel[0] > threshold { 255 } else { 0 };
        rgba.put_pixel(x, y, Rgba([value, value, value, 255]));
    }
    rgba
}

/// Apply emboss effect.
fn apply_emboss(img: &DynamicImage) -> image::RgbaImage {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let mut rgba = image::RgbaImage::new(w, h);
    
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let top_left = gray.get_pixel(x - 1, y - 1)[0] as i32;
            let bottom_right = gray.get_pixel(x + 1, y + 1)[0] as i32;
            
            let value = ((bottom_right - top_left) + 128).clamp(0, 255) as u8;
            rgba.put_pixel(x, y, Rgba([value, value, value, 255]));
        }
    }
    rgba
}

/// Apply edge detection.
fn apply_edge_detect(img: &DynamicImage) -> image::RgbaImage {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let mut rgba = image::RgbaImage::new(w, h);
    
    // Sobel operator
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let p = |dx: i32, dy: i32| -> i32 {
                gray.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32)[0] as i32
            };
            
            let gx = -p(-1, -1) + p(1, -1) - 2 * p(-1, 0) + 2 * p(1, 0) - p(-1, 1) + p(1, 1);
            let gy = -p(-1, -1) - 2 * p(0, -1) - p(1, -1) + p(-1, 1) + 2 * p(0, 1) + p(1, 1);
            
            let magnitude = ((gx * gx + gy * gy) as f32).sqrt().clamp(0.0, 255.0) as u8;
            rgba.put_pixel(x, y, Rgba([magnitude, magnitude, magnitude, 255]));
        }
    }
    rgba
}

/// Apply pixelate effect.
fn apply_pixelate(img: &DynamicImage, pixel_size: u32) -> image::RgbaImage {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let pixel_size = pixel_size.max(1);
    let mut result = image::RgbaImage::new(w, h);
    
    for block_y in (0..h).step_by(pixel_size as usize) {
        for block_x in (0..w).step_by(pixel_size as usize) {
            // Calculate average color for block
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut count: u32 = 0;
            
            for y in block_y..(block_y + pixel_size).min(h) {
                for x in block_x..(block_x + pixel_size).min(w) {
                    let pixel = rgba.get_pixel(x, y);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    count += 1;
                }
            }
            
            let avg_r = (r_sum / count) as u8;
            let avg_g = (g_sum / count) as u8;
            let avg_b = (b_sum / count) as u8;
            
            // Fill block with average color
            for y in block_y..(block_y + pixel_size).min(h) {
                for x in block_x..(block_x + pixel_size).min(w) {
                    result.put_pixel(x, y, Rgba([avg_r, avg_g, avg_b, 255]));
                }
            }
        }
    }
    
    result
}

/// Apply multiple filters in sequence.
pub fn apply_filters<P: AsRef<Path>>(
    input: P,
    output: P,
    filters: &[ImageFilter],
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let mut img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    for filter in filters {
        let result = match filter {
            ImageFilter::Grayscale => DynamicImage::ImageLuma8(img.to_luma8()).to_rgba8(),
            ImageFilter::Invert => {
                let mut rgba = img.to_rgba8();
                for pixel in rgba.pixels_mut() {
                    pixel[0] = 255 - pixel[0];
                    pixel[1] = 255 - pixel[1];
                    pixel[2] = 255 - pixel[2];
                }
                rgba
            }
            ImageFilter::Sepia => apply_sepia(&img),
            ImageFilter::Blur => image::imageops::blur(&img, 2.0).to_rgba8(),
            ImageFilter::FlipHorizontal => image::imageops::flip_horizontal(&img).to_rgba8(),
            ImageFilter::FlipVertical => image::imageops::flip_vertical(&img).to_rgba8(),
            ImageFilter::Rotate90 => image::imageops::rotate90(&img).to_rgba8(),
            ImageFilter::Rotate180 => image::imageops::rotate180(&img).to_rgba8(),
            ImageFilter::Rotate270 => image::imageops::rotate270(&img).to_rgba8(),
            _ => apply_filter_with_options(&input_path, &output_path, *filter, FilterOptions::default())
                .map(|_| img.to_rgba8())?
        };
        img = DynamicImage::ImageRgba8(result);
    }
    
    img.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save image: {}", e),
        source: None,
    })?;
    
    let filter_names: Vec<_> = filters.iter().map(|f| f.description()).collect();
    Ok(ToolOutput::success_with_path(
        format!("Applied {} filters: {}", filters.len(), filter_names.join(", ")),
        output_path,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_list() {
        let filters = ImageFilter::all();
        assert!(!filters.is_empty());
        assert!(filters.contains(&ImageFilter::Grayscale));
    }
    
    #[test]
    fn test_filter_description() {
        assert_eq!(ImageFilter::Grayscale.description(), "Convert to grayscale");
        assert_eq!(ImageFilter::Sepia.description(), "Apply sepia/vintage tone");
    }
}
