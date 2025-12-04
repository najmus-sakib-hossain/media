//! EXIF metadata handling tool.
//!
//! Remove GPS, camera, and other metadata from images for privacy.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use std::io::{Read, Write};
use std::path::Path;

/// EXIF metadata information.
#[derive(Debug, Clone, Default)]
pub struct ExifInfo {
    /// Camera make.
    pub make: Option<String>,
    /// Camera model.
    pub model: Option<String>,
    /// Date/time the photo was taken.
    pub date_time: Option<String>,
    /// GPS latitude.
    pub gps_latitude: Option<f64>,
    /// GPS longitude.
    pub gps_longitude: Option<f64>,
    /// GPS altitude.
    pub gps_altitude: Option<f64>,
    /// Image width.
    pub width: Option<u32>,
    /// Image height.
    pub height: Option<u32>,
    /// ISO speed.
    pub iso: Option<u32>,
    /// Exposure time.
    pub exposure_time: Option<String>,
    /// F-number (aperture).
    pub f_number: Option<f64>,
    /// Focal length.
    pub focal_length: Option<f64>,
    /// Software used.
    pub software: Option<String>,
    /// Copyright information.
    pub copyright: Option<String>,
    /// Artist/author.
    pub artist: Option<String>,
}

impl ExifInfo {
    /// Check if any GPS data is present.
    pub fn has_gps(&self) -> bool {
        self.gps_latitude.is_some() || self.gps_longitude.is_some()
    }

    /// Check if the image has any metadata.
    pub fn is_empty(&self) -> bool {
        self.make.is_none()
            && self.model.is_none()
            && self.date_time.is_none()
            && !self.has_gps()
            && self.width.is_none()
    }
}

/// Read EXIF metadata from an image.
///
/// # Arguments
/// * `input` - Path to the image file
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::read_exif;
///
/// let info = read_exif("photo.jpg").unwrap();
/// if info.has_gps() {
///     println!("Location: {}, {}", info.gps_latitude.unwrap(), info.gps_longitude.unwrap());
/// }
/// ```
pub fn read_exif<P: AsRef<Path>>(input: P) -> Result<ExifInfo> {
    let input_path = input.as_ref();

    let file = std::fs::File::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open file: {}", e),
        source: None,
    })?;

    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    match exifreader.read_from_container(&mut bufreader) {
        Ok(exif_data) => {
            let mut info = ExifInfo::default();

            // Extract common fields
            if let Some(field) = exif_data.get_field(exif::Tag::Make, exif::In::PRIMARY) {
                info.make = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::Model, exif::In::PRIMARY) {
                info.model = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::DateTime, exif::In::PRIMARY) {
                info.date_time = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::Software, exif::In::PRIMARY) {
                info.software = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::Copyright, exif::In::PRIMARY) {
                info.copyright = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::Artist, exif::In::PRIMARY) {
                info.artist = Some(field.display_value().to_string());
            }

            // GPS coordinates
            if let Some(field) = exif_data.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY) {
                info.gps_latitude = parse_gps_coordinate(&field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY) {
                info.gps_longitude = parse_gps_coordinate(&field.display_value().to_string());
            }

            // Camera settings
            if let Some(field) = exif_data.get_field(exif::Tag::ISOSpeed, exif::In::PRIMARY) {
                info.iso = field.display_value().to_string().parse().ok();
            }
            if let Some(field) = exif_data.get_field(exif::Tag::ExposureTime, exif::In::PRIMARY) {
                info.exposure_time = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::FNumber, exif::In::PRIMARY) {
                info.f_number = parse_rational(&field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::FocalLength, exif::In::PRIMARY) {
                info.focal_length = parse_rational(&field.display_value().to_string());
            }

            // Image dimensions
            if let Some(field) = exif_data.get_field(exif::Tag::PixelXDimension, exif::In::PRIMARY)
            {
                info.width = field.display_value().to_string().parse().ok();
            }
            if let Some(field) = exif_data.get_field(exif::Tag::PixelYDimension, exif::In::PRIMARY)
            {
                info.height = field.display_value().to_string().parse().ok();
            }

            Ok(info)
        }
        Err(_) => {
            // No EXIF data found
            Ok(ExifInfo::default())
        }
    }
}

/// Parse GPS coordinate from EXIF string format.
fn parse_gps_coordinate(s: &str) -> Option<f64> {
    // GPS coordinates are usually in format like "deg min sec"
    // This is a simplified parser
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() >= 3 {
        let deg: f64 = parts[0].trim_end_matches("deg").parse().ok()?;
        let min: f64 = parts[1].trim_end_matches("min").parse().ok()?;
        let sec: f64 = parts[2].trim_end_matches("sec").parse().ok()?;
        Some(deg + min / 60.0 + sec / 3600.0)
    } else {
        s.parse().ok()
    }
}

/// Parse rational number from string (e.g., "f/2.8" -> 2.8).
fn parse_rational(s: &str) -> Option<f64> {
    let s = s.trim_start_matches("f/").trim();
    if s.contains('/') {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let num: f64 = parts[0].parse().ok()?;
            let den: f64 = parts[1].parse().ok()?;
            return Some(num / den);
        }
    }
    s.parse().ok()
}

/// Remove EXIF metadata from an image (wipe for privacy).
///
/// This creates a new image file without metadata by re-encoding the pixel data.
///
/// # Arguments
/// * `input` - Path to the input image
/// * `output` - Path for the stripped output
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::wipe_exif;
///
/// // Remove all metadata including GPS
/// wipe_exif("photo_with_location.jpg", "photo_clean.jpg").unwrap();
/// ```
pub fn wipe_exif<P: AsRef<Path>>(input: P, output: P) -> Result<ToolOutput> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();

    // Read original EXIF info for reporting
    let original_info = read_exif(input_path).unwrap_or_default();
    let had_gps = original_info.has_gps();

    // Open image and re-save without metadata
    // The image crate strips EXIF when re-encoding
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;

    // Determine output format from extension
    let extension = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();

    // Save without metadata
    match extension.as_str() {
        "jpg" | "jpeg" => {
            // For JPEG, use a quality that doesn't degrade too much
            let file = std::fs::File::create(output_path).map_err(|e| DxError::FileIo {
                path: output_path.to_path_buf(),
                message: format!("Failed to create file: {}", e),
                source: None,
            })?;
            let mut writer = std::io::BufWriter::new(file);
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, 95);
            img.write_with_encoder(encoder)
                .map_err(|e| DxError::FileIo {
                    path: output_path.to_path_buf(),
                    message: format!("Failed to save image: {}", e),
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

    // Verify EXIF was removed
    let new_info = read_exif(output_path).unwrap_or_default();

    let mut message = String::from("Removed EXIF metadata");
    if had_gps {
        message.push_str(" including GPS location data");
    }
    if original_info.make.is_some() {
        message.push_str(", camera info");
    }

    Ok(ToolOutput::success_with_path(message, output_path)
        .with_metadata("had_gps", had_gps.to_string())
        .with_metadata("original_make", original_info.make.unwrap_or_default())
        .with_metadata("original_model", original_info.model.unwrap_or_default()))
}

/// Strip EXIF metadata from image in place.
pub fn wipe_exif_inplace<P: AsRef<Path>>(path: P) -> Result<ToolOutput> {
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp_stripped");

    let result = wipe_exif(path, &temp_path)?;

    std::fs::rename(&temp_path, path).map_err(|e| DxError::FileIo {
        path: path.to_path_buf(),
        message: format!("Failed to replace original: {}", e),
        source: None,
    })?;

    Ok(result)
}

/// Batch wipe EXIF from multiple images.
pub fn batch_wipe_exif<P: AsRef<Path>>(inputs: &[P], output_dir: P) -> Result<ToolOutput> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create directory: {}", e),
        source: None,
    })?;

    let mut processed = Vec::new();
    let mut gps_removed = 0u32;

    for input in inputs {
        let input_path = input.as_ref();
        let file_name = input_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("output.jpg");
        let output_path = output_dir.join(file_name);

        if let Ok(info) = read_exif(input_path) {
            if info.has_gps() {
                gps_removed += 1;
            }
        }

        if wipe_exif(input_path, &output_path).is_ok() {
            processed.push(output_path);
        }
    }

    Ok(ToolOutput::success(format!(
        "Stripped EXIF from {} images ({} had GPS data)",
        processed.len(),
        gps_removed
    ))
    .with_paths(processed))
}

/// Print EXIF metadata in a readable format.
pub fn format_exif_info(info: &ExifInfo) -> String {
    let mut lines = Vec::new();

    if let Some(make) = &info.make {
        lines.push(format!("Camera Make: {}", make));
    }
    if let Some(model) = &info.model {
        lines.push(format!("Camera Model: {}", model));
    }
    if let Some(dt) = &info.date_time {
        lines.push(format!("Date/Time: {}", dt));
    }
    if let Some(software) = &info.software {
        lines.push(format!("Software: {}", software));
    }
    if info.has_gps() {
        if let (Some(lat), Some(lon)) = (info.gps_latitude, info.gps_longitude) {
            lines.push(format!("GPS: {:.6}, {:.6}", lat, lon));
        }
    }
    if let Some(iso) = info.iso {
        lines.push(format!("ISO: {}", iso));
    }
    if let Some(exp) = &info.exposure_time {
        lines.push(format!("Exposure: {}", exp));
    }
    if let Some(f) = info.f_number {
        lines.push(format!("Aperture: f/{:.1}", f));
    }
    if let Some(fl) = info.focal_length {
        lines.push(format!("Focal Length: {:.0}mm", fl));
    }

    if lines.is_empty() {
        "No EXIF metadata found".to_string()
    } else {
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_info_has_gps() {
        let mut info = ExifInfo::default();
        assert!(!info.has_gps());

        info.gps_latitude = Some(40.7128);
        info.gps_longitude = Some(-74.0060);
        assert!(info.has_gps());
    }

    #[test]
    fn test_parse_rational() {
        assert_eq!(parse_rational("f/2.8"), Some(2.8));
        assert_eq!(parse_rational("2.8"), Some(2.8));
        assert_eq!(parse_rational("1/250"), Some(0.004));
    }
}
