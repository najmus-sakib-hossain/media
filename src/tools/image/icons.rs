//! Icon generation tool.
//!
//! Generate favicon.ico and app icons from a source image.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

/// Standard icon sizes for various platforms.
#[derive(Debug, Clone, Copy)]
pub struct IconSize {
    /// Width and height (icons are square).
    pub size: u32,
    /// Platform or purpose.
    pub platform: IconPlatform,
    /// File name suffix.
    pub suffix: &'static str,
}

/// Icon target platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPlatform {
    /// Favicon for browsers.
    Favicon,
    /// Apple Touch Icon.
    AppleTouch,
    /// Android Chrome icon.
    Android,
    /// Windows tile icon.
    Windows,
    /// macOS app icon.
    MacOS,
    /// iOS app icon.
    IOS,
    /// PWA manifest icons.
    PWA,
    /// Generic icon.
    Generic,
}

impl IconSize {
    /// Standard favicon sizes.
    pub fn favicons() -> Vec<IconSize> {
        vec![
            IconSize { size: 16, platform: IconPlatform::Favicon, suffix: "16x16" },
            IconSize { size: 32, platform: IconPlatform::Favicon, suffix: "32x32" },
            IconSize { size: 48, platform: IconPlatform::Favicon, suffix: "48x48" },
        ]
    }
    
    /// Apple Touch Icon sizes.
    pub fn apple_touch() -> Vec<IconSize> {
        vec![
            IconSize { size: 57, platform: IconPlatform::AppleTouch, suffix: "57x57" },
            IconSize { size: 60, platform: IconPlatform::AppleTouch, suffix: "60x60" },
            IconSize { size: 72, platform: IconPlatform::AppleTouch, suffix: "72x72" },
            IconSize { size: 76, platform: IconPlatform::AppleTouch, suffix: "76x76" },
            IconSize { size: 114, platform: IconPlatform::AppleTouch, suffix: "114x114" },
            IconSize { size: 120, platform: IconPlatform::AppleTouch, suffix: "120x120" },
            IconSize { size: 144, platform: IconPlatform::AppleTouch, suffix: "144x144" },
            IconSize { size: 152, platform: IconPlatform::AppleTouch, suffix: "152x152" },
            IconSize { size: 180, platform: IconPlatform::AppleTouch, suffix: "180x180" },
        ]
    }
    
    /// Android Chrome icon sizes.
    pub fn android() -> Vec<IconSize> {
        vec![
            IconSize { size: 36, platform: IconPlatform::Android, suffix: "36x36" },
            IconSize { size: 48, platform: IconPlatform::Android, suffix: "48x48" },
            IconSize { size: 72, platform: IconPlatform::Android, suffix: "72x72" },
            IconSize { size: 96, platform: IconPlatform::Android, suffix: "96x96" },
            IconSize { size: 144, platform: IconPlatform::Android, suffix: "144x144" },
            IconSize { size: 192, platform: IconPlatform::Android, suffix: "192x192" },
            IconSize { size: 512, platform: IconPlatform::Android, suffix: "512x512" },
        ]
    }
    
    /// Windows tile icon sizes.
    pub fn windows() -> Vec<IconSize> {
        vec![
            IconSize { size: 70, platform: IconPlatform::Windows, suffix: "70x70" },
            IconSize { size: 150, platform: IconPlatform::Windows, suffix: "150x150" },
            IconSize { size: 310, platform: IconPlatform::Windows, suffix: "310x310" },
        ]
    }
    
    /// All standard web icon sizes.
    pub fn web_icons() -> Vec<IconSize> {
        let mut sizes = Self::favicons();
        sizes.extend(Self::apple_touch());
        sizes.extend(Self::android());
        sizes
    }
}

/// Generate icons for multiple platforms from a source image.
///
/// # Arguments
/// * `input` - Path to the source image (should be at least 512x512)
/// * `output_dir` - Directory to save generated icons
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::generate_icons;
///
/// generate_icons("logo.png", "./icons").unwrap();
/// ```
pub fn generate_icons<P: AsRef<Path>>(input: P, output_dir: P) -> Result<ToolOutput> {
    generate_icons_for_sizes(input, output_dir, &IconSize::web_icons())
}

/// Generate icons for specific sizes.
pub fn generate_icons_for_sizes<P: AsRef<Path>>(
    input: P,
    output_dir: P,
    sizes: &[IconSize],
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_dir = output_dir.as_ref();
    
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create output directory: {}", e),
        source: None,
    })?;
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // Warn if source is too small
    let (w, h) = img.dimensions();
    if w < 512 || h < 512 {
        tracing::warn!(
            "Source image is {}x{}, recommended minimum is 512x512 for best quality",
            w, h
        );
    }
    
    let mut generated = Vec::new();
    
    for icon_size in sizes {
        let resized = img.resize_to_fill(
            icon_size.size,
            icon_size.size,
            image::imageops::FilterType::Lanczos3,
        );
        
        let filename = format!("icon-{}.png", icon_size.suffix);
        let output_path = output_dir.join(&filename);
        
        resized.save(&output_path).map_err(|e| DxError::FileIo {
            path: output_path.clone(),
            message: format!("Failed to save icon: {}", e),
            source: None,
        })?;
        
        generated.push(output_path);
    }
    
    Ok(ToolOutput::success(format!("Generated {} icons", generated.len()))
        .with_paths(generated))
}

/// Generate a favicon.ico file with multiple sizes.
///
/// Creates a .ico file containing 16x16, 32x32, and 48x48 versions.
pub fn generate_favicon<P: AsRef<Path>>(input: P, output: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // ICO format requires RGBA
    let rgba = img.to_rgba8();
    
    // For proper ICO files, we'd use the ico crate
    // For now, create individual PNGs that browsers can use
    
    let sizes = [16u32, 32, 48];
    let mut icon_data: Vec<Vec<u8>> = Vec::new();
    
    for size in sizes {
        let resized = image::imageops::resize(
            &rgba,
            size,
            size,
            image::imageops::FilterType::Lanczos3,
        );
        
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.encode(
            &resized,
            size,
            size,
            image::ExtendedColorType::Rgba8,
        ).map_err(|e| DxError::FileIo {
            path: output_path.to_path_buf(),
            message: format!("Failed to encode PNG: {}", e),
            source: None,
        })?;
        icon_data.push(png_data);
    }
    
    // Create ICO file manually
    let ico_data = create_ico_file(&sizes, &icon_data)?;
    
    std::fs::write(output_path, &ico_data).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to write ICO file: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Generated favicon.ico with {} sizes", sizes.len()),
        output_path,
    ))
}

/// Create ICO file format from PNG data.
fn create_ico_file(sizes: &[u32], png_data: &[Vec<u8>]) -> Result<Vec<u8>> {
    let mut ico = Vec::new();
    
    // ICO header
    ico.extend_from_slice(&[0, 0]); // Reserved
    ico.extend_from_slice(&[1, 0]); // Type (1 = ICO)
    ico.extend_from_slice(&(sizes.len() as u16).to_le_bytes()); // Image count
    
    // Calculate offsets
    let header_size = 6 + (sizes.len() * 16);
    let mut offset = header_size;
    
    // Write directory entries
    for (i, &size) in sizes.iter().enumerate() {
        let size_byte = if size >= 256 { 0u8 } else { size as u8 };
        ico.push(size_byte); // Width
        ico.push(size_byte); // Height
        ico.push(0); // Color palette
        ico.push(0); // Reserved
        ico.extend_from_slice(&[1, 0]); // Color planes
        ico.extend_from_slice(&[32, 0]); // Bits per pixel
        ico.extend_from_slice(&(png_data[i].len() as u32).to_le_bytes()); // Image size
        ico.extend_from_slice(&(offset as u32).to_le_bytes()); // Offset
        
        offset += png_data[i].len();
    }
    
    // Write image data
    for data in png_data {
        ico.extend_from_slice(data);
    }
    
    Ok(ico)
}

/// Generate Apple Touch Icon.
pub fn generate_apple_touch_icon<P: AsRef<Path>>(input: P, output: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // Apple Touch Icon is 180x180
    let resized = img.resize_to_fill(180, 180, image::imageops::FilterType::Lanczos3);
    
    resized.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save icon: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        "Generated 180x180 Apple Touch Icon",
        output_path,
    ))
}

/// Generate PWA manifest icons.
pub fn generate_pwa_icons<P: AsRef<Path>>(input: P, output_dir: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_dir = output_dir.as_ref();
    
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create directory: {}", e),
        source: None,
    })?;
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // PWA sizes: 192x192 and 512x512
    let sizes = [(192, "icon-192x192.png"), (512, "icon-512x512.png")];
    let mut generated = Vec::new();
    
    for (size, filename) in sizes {
        let resized = img.resize_to_fill(size, size, image::imageops::FilterType::Lanczos3);
        let output_path = output_dir.join(filename);
        
        resized.save(&output_path).map_err(|e| DxError::FileIo {
            path: output_path.clone(),
            message: format!("Failed to save icon: {}", e),
            source: None,
        })?;
        
        generated.push(output_path);
    }
    
    // Generate manifest.json
    let manifest = serde_json::json!({
        "icons": [
            {
                "src": "icon-192x192.png",
                "sizes": "192x192",
                "type": "image/png"
            },
            {
                "src": "icon-512x512.png",
                "sizes": "512x512",
                "type": "image/png"
            }
        ]
    });
    
    let manifest_path = output_dir.join("icons-manifest.json");
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap())
        .map_err(|e| DxError::FileIo {
            path: manifest_path.clone(),
            message: format!("Failed to write manifest: {}", e),
            source: None,
        })?;
    
    generated.push(manifest_path);
    
    Ok(ToolOutput::success(format!("Generated PWA icons and manifest"))
        .with_paths(generated))
}

/// Generate icons with maskable safe zone for PWA.
pub fn generate_maskable_icon<P: AsRef<Path>>(
    input: P,
    output: P,
    size: u32,
    background_color: [u8; 3],
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    // Maskable icons need a safe zone (inner 80%)
    let safe_size = (size as f32 * 0.8) as u32;
    let padding = (size - safe_size) / 2;
    
    // Create canvas with background
    let mut canvas = image::RgbaImage::new(size, size);
    for pixel in canvas.pixels_mut() {
        *pixel = image::Rgba([background_color[0], background_color[1], background_color[2], 255]);
    }
    
    // Resize source to fit safe zone
    let resized = img.resize_to_fill(safe_size, safe_size, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();
    
    // Overlay on canvas
    for (x, y, pixel) in rgba.enumerate_pixels() {
        let dest_x = padding + x;
        let dest_y = padding + y;
        if dest_x < size && dest_y < size {
            let alpha = pixel[3] as f32 / 255.0;
            if alpha > 0.0 {
                let bg = canvas.get_pixel(dest_x, dest_y);
                let blended = image::Rgba([
                    ((pixel[0] as f32 * alpha) + (bg[0] as f32 * (1.0 - alpha))) as u8,
                    ((pixel[1] as f32 * alpha) + (bg[1] as f32 * (1.0 - alpha))) as u8,
                    ((pixel[2] as f32 * alpha) + (bg[2] as f32 * (1.0 - alpha))) as u8,
                    255,
                ]);
                canvas.put_pixel(dest_x, dest_y, blended);
            }
        }
    }
    
    canvas.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save icon: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Generated {}x{} maskable icon", size, size),
        output_path,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_icon_sizes() {
        let favicons = IconSize::favicons();
        assert_eq!(favicons.len(), 3);
        assert_eq!(favicons[0].size, 16);
        
        let web = IconSize::web_icons();
        assert!(web.len() > 10);
    }
}
