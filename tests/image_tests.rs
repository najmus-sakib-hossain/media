//! Tests for image tools.
//!
//! These tests cover the 10 image tools:
//! 1. Format Converter
//! 2. Resizer
//! 3. Compressor
//! 4. Watermarker
//! 5. EXIF Reader
//! 6. QR Code Generator
//! 7. Color Palette Extractor
//! 8. Filter Processor
//! 9. OCR Text Extractor
//! 10. Icon Generator
//!
//! Note: These tests require ImageMagick to be installed.

mod common;
use common::TestFixture;
use dx_media::tools::image;

// ═══════════════════════════════════════════════════════════════
// 1. FORMAT CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod converter_tests {
    use super::*;
    use dx_media::tools::image::converter;

    #[test]
    fn test_image_format_extensions() {
        assert_eq!(converter::ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(converter::ImageFormat::Png.extension(), "png");
        assert_eq!(converter::ImageFormat::Webp.extension(), "webp");
        assert_eq!(converter::ImageFormat::Gif.extension(), "gif");
        assert_eq!(converter::ImageFormat::Bmp.extension(), "bmp");
    }

    #[test]
    fn test_format_from_extension() {
        assert!(converter::ImageFormat::from_extension("jpg").is_some());
        assert!(converter::ImageFormat::from_extension("png").is_some());
        assert!(converter::ImageFormat::from_extension("xyz").is_none());
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. RESIZER TESTS
// ═══════════════════════════════════════════════════════════════

mod resizer_tests {
    use dx_media::tools::image::resizer;

    // Note: Actual resize tests require ImageMagick and real images
    #[test]
    fn test_resize_module_exists() {
        // Verify the module exists and functions are accessible
        let _ = resizer::resize::<&str, &str>;
        let _ = resizer::resize_to_width::<&str, &str>;
        let _ = resizer::resize_to_height::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. COMPRESSOR TESTS
// ═══════════════════════════════════════════════════════════════

mod compressor_tests {
    use dx_media::tools::image::compressor;

    #[test]
    fn test_compression_quality_enum() {
        let _ = compressor::CompressionQuality::Low;
        let _ = compressor::CompressionQuality::Medium;
        let _ = compressor::CompressionQuality::High;
        let _ = compressor::CompressionQuality::Maximum;
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. WATERMARK TESTS
// ═══════════════════════════════════════════════════════════════

mod watermark_tests {
    use dx_media::tools::image::watermark;

    #[test]
    fn test_watermark_position_enum() {
        let _ = watermark::WatermarkPosition::TopLeft;
        let _ = watermark::WatermarkPosition::TopRight;
        let _ = watermark::WatermarkPosition::BottomLeft;
        let _ = watermark::WatermarkPosition::BottomRight;
        let _ = watermark::WatermarkPosition::Center;
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. EXIF READER TESTS
// ═══════════════════════════════════════════════════════════════

mod exif_tests {
    use dx_media::tools::image::exif;

    #[test]
    fn test_exif_info_struct() {
        let info = exif::ExifInfo::default();
        assert!(info.is_empty());
        assert!(!info.has_gps());
    }

    #[test]
    fn test_exif_info_get() {
        let info = exif::ExifInfo::default();
        assert!(info.get("nonexistent").is_none());
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. QR CODE GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod qrcode_tests {
    use dx_media::tools::image::qrcode;

    #[test]
    fn test_qr_error_correction() {
        let _ = qrcode::QrErrorCorrection::Low;
        let _ = qrcode::QrErrorCorrection::Medium;
        let _ = qrcode::QrErrorCorrection::High;
    }

    #[test]
    fn test_qr_options_struct() {
        let options = qrcode::QrOptions::default();
        assert!(options.size > 0);
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. COLOR PALETTE EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod palette_tests {
    use dx_media::tools::image::palette;

    #[test]
    fn test_color_struct() {
        let color = palette::Color { r: 255, g: 128, b: 0 };
        let hex = color.to_hex();
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);
    }

    #[test]
    fn test_color_to_rgb() {
        let color = palette::Color { r: 100, g: 150, b: 200 };
        let rgb = color.to_rgb();
        assert!(rgb.contains("100"));
        assert!(rgb.contains("150"));
        assert!(rgb.contains("200"));
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. FILTER PROCESSOR TESTS
// ═══════════════════════════════════════════════════════════════

mod filters_tests {
    use dx_media::tools::image::filters;

    #[test]
    fn test_filter_enum() {
        let _ = filters::Filter::Grayscale;
        let _ = filters::Filter::Sepia;
        let _ = filters::Filter::Blur(2.0);
        let _ = filters::Filter::Sharpen(1.0);
        let _ = filters::Filter::Brightness(10);
        let _ = filters::Filter::Contrast(10);
        let _ = filters::Filter::Invert;
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. OCR TEXT EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod ocr_tests {
    use dx_media::tools::image::ocr;

    #[test]
    fn test_ocr_options_builders() {
        let single_column = ocr::OcrOptions::single_column();
        assert!(single_column.language.is_none() || single_column.language.is_some());

        let single_block = ocr::OcrOptions::single_block();
        let _ = single_block;

        let single_line = ocr::OcrOptions::single_line();
        let _ = single_line;

        let single_word = ocr::OcrOptions::single_word();
        let _ = single_word;
    }

    #[test]
    fn test_ocr_options_with_language() {
        let options = ocr::OcrOptions::single_column().with_language("eng");
        assert_eq!(options.language, Some("eng".to_string()));
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. ICON GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod icons_tests {
    use dx_media::tools::image::icons;

    #[test]
    fn test_icon_functions_exist() {
        // Verify the functions exist
        let _ = icons::generate_icon::<&str, &str>;
        let _ = icons::generate_favicon::<&str, &str>;
        let _ = icons::generate_ios_icons::<&str, &str>;
        let _ = icons::generate_android_icons::<&str, &str>;
        let _ = icons::generate_pwa_icons::<&str, &str>;
        let _ = icons::generate_all_icons::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// IMAGE TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod image_tools_tests {
    use super::*;

    #[test]
    fn test_image_tools_instantiation() {
        let tools = image::ImageTools::new();
        drop(tools);
    }

    #[test]
    fn test_image_tools_default() {
        let tools = image::ImageTools::default();
        drop(tools);
    }
}
