//! Color palette extraction tool.
//!
//! Extract dominant colors from images for design workflows.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;

/// A color with its frequency in the image.
#[derive(Debug, Clone)]
pub struct ColorInfo {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Percentage of image this color represents.
    pub percentage: f32,
}

impl ColorInfo {
    /// Convert to hex string (e.g., "#FF5733").
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to RGB string (e.g., "rgb(255, 87, 51)").
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// Convert to HSL values.
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < 0.001 {
            return (0.0, 0.0, l);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if (max - r).abs() < 0.001 {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if (max - g).abs() < 0.001 {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };

        (h * 360.0, s * 100.0, l * 100.0)
    }

    /// Get a descriptive color name (approximate).
    pub fn color_name(&self) -> &'static str {
        let (h, s, l) = self.to_hsl();

        if l < 10.0 {
            return "Black";
        }
        if l > 90.0 {
            return "White";
        }
        if s < 10.0 {
            return if l < 50.0 { "Dark Gray" } else { "Light Gray" };
        }

        match h as u32 {
            0..=15 | 346..=360 => "Red",
            16..=45 => "Orange",
            46..=70 => "Yellow",
            71..=165 => "Green",
            166..=200 => "Cyan",
            201..=260 => "Blue",
            261..=290 => "Purple",
            291..=345 => "Magenta",
            _ => "Unknown",
        }
    }
}

/// Extract dominant colors from an image.
///
/// Uses color quantization to find the most representative colors.
///
/// # Arguments
/// * `input` - Path to the image file
/// * `count` - Number of colors to extract (1-16)
///
/// # Returns
/// A vector of hex color strings (e.g., ["#FF5733", "#33FF57", ...])
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::extract_palette;
///
/// let colors = extract_palette("photo.jpg", 5).unwrap();
/// for color in colors {
///     println!("{}", color);
/// }
/// ```
pub fn extract_palette<P: AsRef<Path>>(input: P, count: usize) -> Result<Vec<String>> {
    let colors = extract_palette_detailed(input, count)?;
    Ok(colors.into_iter().map(|c| c.to_hex()).collect())
}

/// Extract dominant colors with detailed information.
pub fn extract_palette_detailed<P: AsRef<Path>>(input: P, count: usize) -> Result<Vec<ColorInfo>> {
    let input_path = input.as_ref();
    let count = count.clamp(1, 16);

    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;

    // Resize for faster processing if large
    let img = if img.width() > 200 || img.height() > 200 {
        img.thumbnail(200, 200)
    } else {
        img
    };

    let rgba = img.to_rgba8();
    let total_pixels = (rgba.width() * rgba.height()) as f32;

    // Quantize colors to reduce color space
    let mut color_counts: HashMap<(u8, u8, u8), u32> = HashMap::new();

    for pixel in rgba.pixels() {
        // Skip transparent pixels
        if pixel[3] < 128 {
            continue;
        }

        // Quantize to 5-bit color (32 levels per channel)
        let r = (pixel[0] / 8) * 8 + 4;
        let g = (pixel[1] / 8) * 8 + 4;
        let b = (pixel[2] / 8) * 8 + 4;

        *color_counts.entry((r, g, b)).or_insert(0) += 1;
    }

    // Sort by frequency
    let mut sorted_colors: Vec<_> = color_counts.into_iter().collect();
    sorted_colors.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top colors and filter similar colors
    let mut result = Vec::new();

    for (color, freq) in sorted_colors {
        // Check if too similar to existing colors
        let is_unique = result.iter().all(|c: &ColorInfo| {
            let dr = (c.r as i32 - color.0 as i32).abs();
            let dg = (c.g as i32 - color.1 as i32).abs();
            let db = (c.b as i32 - color.2 as i32).abs();
            dr + dg + db > 60 // Minimum color distance
        });

        if is_unique {
            result.push(ColorInfo {
                r: color.0,
                g: color.1,
                b: color.2,
                percentage: (freq as f32 / total_pixels) * 100.0,
            });
        }

        if result.len() >= count {
            break;
        }
    }

    Ok(result)
}

/// Generate a palette swatch image.
pub fn generate_palette_swatch<P: AsRef<Path>>(
    input: P,
    output: P,
    color_count: usize,
    swatch_size: u32,
) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();

    let colors = extract_palette_detailed(input_path, color_count)?;

    if colors.is_empty() {
        return Err(DxError::Config {
            message: "No colors found in image".to_string(),
            source: None,
        });
    }

    // Create swatch image
    let width = swatch_size * colors.len() as u32;
    let height = swatch_size;
    let mut img = image::RgbImage::new(width, height);

    for (i, color) in colors.iter().enumerate() {
        let x_start = i as u32 * swatch_size;
        for y in 0..height {
            for x in x_start..(x_start + swatch_size) {
                img.put_pixel(x, y, image::Rgb([color.r, color.g, color.b]));
            }
        }
    }

    img.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save swatch: {}", e),
        source: None,
    })?;

    let hex_colors: Vec<String> = colors.iter().map(|c| c.to_hex()).collect();

    Ok(ToolOutput::success_with_path(
        format!("Generated palette swatch with {} colors", colors.len()),
        output_path,
    )
    .with_metadata("colors", hex_colors.join(", ")))
}

/// Export palette to various formats.
#[derive(Debug, Clone, Copy)]
pub enum PaletteFormat {
    /// CSS custom properties format.
    Css,
    /// JSON array format.
    Json,
    /// GIMP palette format (.gpl).
    Gpl,
    /// Adobe Swatch Exchange (.ase) - simplified.
    Ase,
    /// Plain text list.
    Text,
}

/// Export extracted palette to a specific format.
pub fn export_palette<P: AsRef<Path>>(
    input: P,
    output: P,
    count: usize,
    format: PaletteFormat,
) -> Result<ToolOutput> {
    let output_path = output.as_ref();
    let colors = extract_palette_detailed(input, count)?;

    let content = match format {
        PaletteFormat::Css => {
            let mut css = String::from(":root {\n");
            for (i, c) in colors.iter().enumerate() {
                css.push_str(&format!("  --color-{}: {};\n", i + 1, c.to_hex()));
            }
            css.push_str("}\n");
            css
        }
        PaletteFormat::Json => {
            let json_colors: Vec<_> = colors
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "hex": c.to_hex(),
                        "rgb": [c.r, c.g, c.b],
                        "percentage": c.percentage,
                        "name": c.color_name()
                    })
                })
                .collect();
            serde_json::to_string_pretty(&json_colors).unwrap_or_default()
        }
        PaletteFormat::Gpl => {
            let mut gpl = String::from("GIMP Palette\nName: Extracted Palette\nColumns: 0\n#\n");
            for c in &colors {
                gpl.push_str(&format!(
                    "{:3} {:3} {:3}\t{}\n",
                    c.r,
                    c.g,
                    c.b,
                    c.color_name()
                ));
            }
            gpl
        }
        PaletteFormat::Ase => {
            // Simplified text representation (real ASE is binary)
            let mut ase = String::from("Adobe Swatch Exchange (Text)\n");
            for (i, c) in colors.iter().enumerate() {
                ase.push_str(&format!(
                    "Color {}: {} ({}, {}, {})\n",
                    i + 1,
                    c.to_hex(),
                    c.r,
                    c.g,
                    c.b
                ));
            }
            ase
        }
        PaletteFormat::Text => colors
            .iter()
            .map(|c| format!("{} - {} ({:.1}%)", c.to_hex(), c.color_name(), c.percentage))
            .collect::<Vec<_>>()
            .join("\n"),
    };

    std::fs::write(output_path, &content).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to write palette file: {}", e),
        source: None,
    })?;

    Ok(ToolOutput::success_with_path(
        format!("Exported {} colors to {:?} format", colors.len(), format),
        output_path,
    ))
}

/// Analyze image color statistics.
pub fn analyze_colors<P: AsRef<Path>>(input: P) -> Result<ColorAnalysis> {
    let input_path = input.as_ref();

    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;

    let rgba = img.to_rgba8();
    let total_pixels = (rgba.width() * rgba.height()) as f64;

    let mut r_sum: u64 = 0;
    let mut g_sum: u64 = 0;
    let mut b_sum: u64 = 0;
    let mut brightness_sum: u64 = 0;
    let mut saturated_count = 0u64;

    for pixel in rgba.pixels() {
        r_sum += pixel[0] as u64;
        g_sum += pixel[1] as u64;
        b_sum += pixel[2] as u64;

        let brightness = (pixel[0] as u64 + pixel[1] as u64 + pixel[2] as u64) / 3;
        brightness_sum += brightness;

        // Check if saturated (near pure color)
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        if max > 200 && (max - min) > 100 {
            saturated_count += 1;
        }
    }

    let pixel_count = total_pixels as u64;

    Ok(ColorAnalysis {
        average_r: (r_sum / pixel_count) as u8,
        average_g: (g_sum / pixel_count) as u8,
        average_b: (b_sum / pixel_count) as u8,
        average_brightness: (brightness_sum / pixel_count) as u8,
        saturation_ratio: saturated_count as f32 / total_pixels as f32,
        dominant_colors: extract_palette_detailed(input_path, 5).unwrap_or_default(),
    })
}

/// Color analysis results.
#[derive(Debug, Clone)]
pub struct ColorAnalysis {
    /// Average red component.
    pub average_r: u8,
    /// Average green component.
    pub average_g: u8,
    /// Average blue component.
    pub average_b: u8,
    /// Average brightness (0-255).
    pub average_brightness: u8,
    /// Ratio of saturated pixels.
    pub saturation_ratio: f32,
    /// Top dominant colors.
    pub dominant_colors: Vec<ColorInfo>,
}

impl ColorAnalysis {
    /// Get average color as hex.
    pub fn average_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            self.average_r, self.average_g, self.average_b
        )
    }

    /// Determine if image is predominantly dark or light.
    pub fn is_dark(&self) -> bool {
        self.average_brightness < 128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_info_hex() {
        let color = ColorInfo {
            r: 255,
            g: 87,
            b: 51,
            percentage: 10.0,
        };
        assert_eq!(color.to_hex(), "#FF5733");
    }

    #[test]
    fn test_color_info_rgb_string() {
        let color = ColorInfo {
            r: 255,
            g: 87,
            b: 51,
            percentage: 10.0,
        };
        assert_eq!(color.to_rgb_string(), "rgb(255, 87, 51)");
    }

    #[test]
    fn test_color_name() {
        let red = ColorInfo {
            r: 255,
            g: 0,
            b: 0,
            percentage: 0.0,
        };
        assert_eq!(red.color_name(), "Red");

        let green = ColorInfo {
            r: 0,
            g: 255,
            b: 0,
            percentage: 0.0,
        };
        assert_eq!(green.color_name(), "Green");

        let blue = ColorInfo {
            r: 0,
            g: 0,
            b: 255,
            percentage: 0.0,
        };
        assert_eq!(blue.color_name(), "Blue");
    }
}
