//! QR Code generation and reading tool.
//!
//! Generate QR codes from text/URLs and decode QR codes from images.

use crate::error::{DxError, Result};
use crate::tools::ToolOutput;
use std::path::Path;

/// QR Code error correction level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QrErrorCorrection {
    /// ~7% error recovery.
    Low,
    /// ~15% error recovery.
    #[default]
    Medium,
    /// ~25% error recovery.
    Quartile,
    /// ~30% error recovery.
    High,
}

impl QrErrorCorrection {
    fn to_qrcode_level(&self) -> qrcode::EcLevel {
        match self {
            Self::Low => qrcode::EcLevel::L,
            Self::Medium => qrcode::EcLevel::M,
            Self::Quartile => qrcode::EcLevel::Q,
            Self::High => qrcode::EcLevel::H,
        }
    }
}

/// QR Code generation options.
#[derive(Debug, Clone)]
pub struct QrCodeOptions {
    /// Size of the QR code in pixels.
    pub size: u32,
    /// Error correction level.
    pub error_correction: QrErrorCorrection,
    /// Quiet zone (margin) in modules.
    pub quiet_zone: u32,
    /// Foreground color (dark modules).
    pub foreground: [u8; 3],
    /// Background color (light modules).
    pub background: [u8; 3],
}

impl Default for QrCodeOptions {
    fn default() -> Self {
        Self {
            size: 256,
            error_correction: QrErrorCorrection::default(),
            quiet_zone: 4,
            foreground: [0, 0, 0],       // Black
            background: [255, 255, 255], // White
        }
    }
}

impl QrCodeOptions {
    /// Create options with specific size.
    pub fn with_size(size: u32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
    
    /// Set error correction level.
    pub fn with_error_correction(mut self, level: QrErrorCorrection) -> Self {
        self.error_correction = level;
        self
    }
    
    /// Set foreground color (RGB).
    pub fn with_foreground(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground = [r, g, b];
        self
    }
    
    /// Set background color (RGB).
    pub fn with_background(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background = [r, g, b];
        self
    }
}

/// Generate a QR code image from text data.
///
/// # Arguments
/// * `data` - The text/URL to encode
/// * `output` - Path to save the QR code image
/// * `size` - Size of the output image in pixels
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::generate_qr;
///
/// // Generate QR code for a URL
/// generate_qr("https://example.com", "qr_code.png", 256).unwrap();
/// ```
pub fn generate_qr<P: AsRef<Path>>(data: &str, output: P, size: u32) -> Result<ToolOutput> {
    generate_qr_with_options(data, output, QrCodeOptions::with_size(size))
}

/// Generate a QR code with detailed options.
pub fn generate_qr_with_options<P: AsRef<Path>>(
    data: &str,
    output: P,
    options: QrCodeOptions,
) -> Result<ToolOutput> {
    let output_path = output.as_ref();
    
    // Create QR code
    let qr = qrcode::QrCode::with_error_correction_level(data, options.error_correction.to_qrcode_level())
        .map_err(|e| DxError::Config {
            message: format!("Failed to generate QR code: {}", e),
            source: None,
        })?;
    
    // Calculate module size based on desired output size and quiet zone
    let qr_size = qr.width();
    let total_modules = qr_size + (options.quiet_zone * 2) as usize;
    let module_size = (options.size as usize / total_modules).max(1);
    let actual_size = (total_modules * module_size) as u32;
    
    // Create image
    let mut img = image::RgbImage::new(actual_size, actual_size);
    
    // Fill background
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb(options.background);
    }
    
    // Draw QR code modules
    let quiet_offset = (options.quiet_zone as usize * module_size) as u32;
    
    for (y, row) in qr.to_colors().chunks(qr_size).enumerate() {
        for (x, color) in row.iter().enumerate() {
            let is_dark = matches!(color, qrcode::Color::Dark);
            if is_dark {
                let px = quiet_offset + (x * module_size) as u32;
                let py = quiet_offset + (y * module_size) as u32;
                
                // Fill module
                for dy in 0..module_size as u32 {
                    for dx in 0..module_size as u32 {
                        if px + dx < actual_size && py + dy < actual_size {
                            img.put_pixel(px + dx, py + dy, image::Rgb(options.foreground));
                        }
                    }
                }
            }
        }
    }
    
    // Save image
    img.save(output_path).map_err(|e| DxError::FileIo {
        path: output_path.to_path_buf(),
        message: format!("Failed to save QR code image: {}", e),
        source: None,
    })?;
    
    Ok(ToolOutput::success_with_path(
        format!("Generated {}x{} QR code for {} characters of data", actual_size, actual_size, data.len()),
        output_path,
    )
    .with_metadata("data_length", data.len().to_string())
    .with_metadata("qr_version", qr.version().to_string())
    .with_metadata("image_size", actual_size.to_string()))
}

/// Read and decode a QR code from an image.
///
/// # Arguments
/// * `input` - Path to the image containing a QR code
///
/// # Example
/// ```no_run
/// use dx_media::tools::image::read_qr;
///
/// let data = read_qr("qr_code.png").unwrap();
/// println!("QR code contains: {}", data);
/// ```
pub fn read_qr<P: AsRef<Path>>(input: P) -> Result<String> {
    let input_path = input.as_ref();
    
    // Load image
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let luma = img.to_luma8();
    let (width, height) = luma.dimensions();
    
    // Prepare image data for rxing
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for pixel in luma.pixels() {
        pixels.push(pixel.0[0]);
    }
    
    // Create decoder
    let mut hints = rxing::DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
    
    // Decode
    let result = rxing::helpers::detect_in_luma_with_hints(
        pixels,
        width,
        height,
        None,
        &mut hints,
    ).map_err(|e| DxError::Config {
        message: format!("Failed to decode QR code: {:?}", e),
        source: None,
    })?;
    
    Ok(result.getText().to_string())
}

/// Read QR code and return detailed information.
pub fn read_qr_detailed<P: AsRef<Path>>(input: P) -> Result<QrReadResult> {
    let input_path = input.as_ref();
    
    let img = image::open(input_path).map_err(|e| DxError::FileIo {
        path: input_path.to_path_buf(),
        message: format!("Failed to open image: {}", e),
        source: None,
    })?;
    
    let luma = img.to_luma8();
    let (width, height) = luma.dimensions();
    
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for pixel in luma.pixels() {
        pixels.push(pixel.0[0]);
    }
    
    let mut hints = rxing::DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
    
    let result = rxing::helpers::detect_in_luma_with_hints(
        pixels,
        width,
        height,
        None,
        &mut hints,
    ).map_err(|e| DxError::Config {
        message: format!("Failed to decode QR code: {:?}", e),
        source: None,
    })?;
    
    Ok(QrReadResult {
        data: result.getText().to_string(),
        format: result.getBarcodeFormat().to_string(),
        raw_bytes: result.getRawBytes().to_vec(),
    })
}

/// Result of QR code reading operation.
#[derive(Debug, Clone)]
pub struct QrReadResult {
    /// Decoded text data.
    pub data: String,
    /// Barcode format (should be "QR_CODE").
    pub format: String,
    /// Raw byte data.
    pub raw_bytes: Vec<u8>,
}

/// Generate multiple QR codes in batch.
pub fn batch_generate_qr<P: AsRef<Path>>(
    data: &[&str],
    output_dir: P,
    size: u32,
) -> Result<ToolOutput> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir).map_err(|e| DxError::FileIo {
        path: output_dir.to_path_buf(),
        message: format!("Failed to create directory: {}", e),
        source: None,
    })?;
    
    let mut generated = Vec::new();
    
    for (i, text) in data.iter().enumerate() {
        let output_path = output_dir.join(format!("qr_{:04}.png", i + 1));
        if generate_qr(text, &output_path, size).is_ok() {
            generated.push(output_path);
        }
    }
    
    Ok(ToolOutput::success(format!("Generated {} QR codes", generated.len()))
        .with_paths(generated))
}

/// Generate a vCard QR code.
pub fn generate_vcard_qr<P: AsRef<Path>>(
    name: &str,
    phone: Option<&str>,
    email: Option<&str>,
    output: P,
    size: u32,
) -> Result<ToolOutput> {
    let mut vcard = format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\n",
        name
    );
    
    if let Some(p) = phone {
        vcard.push_str(&format!("TEL:{}\n", p));
    }
    if let Some(e) = email {
        vcard.push_str(&format!("EMAIL:{}\n", e));
    }
    vcard.push_str("END:VCARD");
    
    generate_qr(&vcard, output, size)
}

/// Generate a WiFi QR code.
pub fn generate_wifi_qr<P: AsRef<Path>>(
    ssid: &str,
    password: &str,
    encryption: &str, // "WPA", "WEP", or "nopass"
    output: P,
    size: u32,
) -> Result<ToolOutput> {
    let wifi_data = format!(
        "WIFI:T:{};S:{};P:{};;",
        encryption, ssid, password
    );
    
    generate_qr(&wifi_data, output, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_qr_options() {
        let opts = QrCodeOptions::with_size(512)
            .with_error_correction(QrErrorCorrection::High)
            .with_foreground(0, 0, 128);
        
        assert_eq!(opts.size, 512);
        assert_eq!(opts.error_correction, QrErrorCorrection::High);
        assert_eq!(opts.foreground, [0, 0, 128]);
    }
}
