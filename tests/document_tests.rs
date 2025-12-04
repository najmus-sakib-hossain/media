//! Tests for document tools.
//!
//! These tests cover the 10 document tools:
//! 1. PDF Merger
//! 2. PDF Splitter
//! 3. PDF Compressor
//! 4. PDF to Image
//! 5. Markdown Converter
//! 6. HTML to PDF
//! 7. Document Converter
//! 8. Text Extractor
//! 9. PDF Watermark
//! 10. PDF Encryption
//!
//! Note: These tests require Ghostscript and other tools to be installed.

mod common;
use common::TestFixture;
use dx_media::tools::document;

// ═══════════════════════════════════════════════════════════════
// 1. PDF MERGER TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_merge_tests {
    use dx_media::tools::document::pdf_merge;

    #[test]
    fn test_merge_functions_exist() {
        let _ = pdf_merge::merge_pdfs::<&str, &str>;
        let _ = pdf_merge::merge_directory::<&str, &str>;
        let _ = pdf_merge::append_pdf::<&str, &str, &str>;
        let _ = pdf_merge::interleave_pdfs::<&str, &str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. PDF SPLITTER TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_split_tests {
    use dx_media::tools::document::pdf_split;

    #[test]
    fn test_split_functions_exist() {
        let _ = pdf_split::split_pdf::<&str, &str>;
        let _ = pdf_split::extract_pages::<&str, &str>;
        let _ = pdf_split::extract_page::<&str, &str>;
        let _ = pdf_split::extract_odd_pages::<&str, &str>;
        let _ = pdf_split::extract_even_pages::<&str, &str>;
        let _ = pdf_split::get_page_count::<&str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. PDF COMPRESSOR TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_compress_tests {
    use dx_media::tools::document::pdf_compress;

    #[test]
    fn test_compression_quality() {
        let _ = pdf_compress::CompressionQuality::Screen;
        let _ = pdf_compress::CompressionQuality::Ebook;
        let _ = pdf_compress::CompressionQuality::Printer;
        let _ = pdf_compress::CompressionQuality::Prepress;
    }

    #[test]
    fn test_compression_quality_description() {
        let screen = pdf_compress::CompressionQuality::Screen;
        let desc = screen.description();
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_compress_functions_exist() {
        let _ = pdf_compress::compress_pdf::<&str, &str>;
        let _ = pdf_compress::compress_pdf_custom::<&str, &str>;
        let _ = pdf_compress::linearize_pdf::<&str, &str>;
        let _ = pdf_compress::clean_pdf::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. PDF TO IMAGE TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_to_image_tests {
    use dx_media::tools::document::pdf_to_image;

    #[test]
    fn test_image_format() {
        let _ = pdf_to_image::ImageFormat::Png;
        let _ = pdf_to_image::ImageFormat::Jpeg;
        let _ = pdf_to_image::ImageFormat::Tiff;
    }

    #[test]
    fn test_pdf_to_image_options() {
        let options = pdf_to_image::PdfToImageOptions::default();
        assert!(options.dpi > 0);
    }

    #[test]
    fn test_high_quality_png() {
        let options = pdf_to_image::PdfToImageOptions::high_quality_png();
        assert!(options.dpi >= 300);
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. MARKDOWN CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod markdown_tests {
    use super::*;
    use dx_media::tools::document::markdown;

    #[test]
    fn test_markdown_options_styled() {
        let options = markdown::MarkdownOptions::styled();
        assert!(options.include_css);
    }

    #[test]
    fn test_markdown_options_plain() {
        let options = markdown::MarkdownOptions::plain();
        assert!(!options.include_css);
    }

    #[test]
    fn test_markdown_string_to_html() {
        let md = "# Hello\n\nThis is **bold** text.";
        let options = markdown::MarkdownOptions::plain();
        let html = markdown::markdown_string_to_html(md, options);
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_markdown_to_html_file() {
        let fixture = TestFixture::new();
        let input = fixture.create_text_file("test.md", "# Test\n\nParagraph");
        let output = fixture.path("test.html");

        let result = markdown::markdown_to_html(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. HTML TO PDF TESTS
// ═══════════════════════════════════════════════════════════════

mod html_to_pdf_tests {
    use dx_media::tools::document::html_to_pdf;

    #[test]
    fn test_page_orientation() {
        let _ = html_to_pdf::PageOrientation::Portrait;
        let _ = html_to_pdf::PageOrientation::Landscape;
    }

    #[test]
    fn test_html_to_pdf_options_a4() {
        let options = html_to_pdf::HtmlToPdfOptions::a4();
        let _ = options;
    }

    #[test]
    fn test_html_to_pdf_options_letter() {
        let options = html_to_pdf::HtmlToPdfOptions::letter();
        let _ = options;
    }

    #[test]
    fn test_html_to_pdf_options_landscape() {
        let options = html_to_pdf::HtmlToPdfOptions::landscape();
        assert!(matches!(options.orientation, html_to_pdf::PageOrientation::Landscape));
    }

    #[test]
    fn test_html_to_pdf_functions_exist() {
        let _ = html_to_pdf::html_to_pdf::<&str, &str>;
        let _ = html_to_pdf::html_to_pdf_with_options::<&str, &str>;
        let _ = html_to_pdf::url_to_pdf::<&str>;
        let _ = html_to_pdf::html_string_to_pdf::<&str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. DOCUMENT CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod doc_convert_tests {
    use dx_media::tools::document::doc_convert;

    #[test]
    fn test_doc_format() {
        let _ = doc_convert::DocFormat::Docx;
        let _ = doc_convert::DocFormat::Odt;
        let _ = doc_convert::DocFormat::Pdf;
        let _ = doc_convert::DocFormat::Rtf;
        let _ = doc_convert::DocFormat::Txt;
        let _ = doc_convert::DocFormat::Html;
    }

    #[test]
    fn test_doc_format_extension() {
        assert_eq!(doc_convert::DocFormat::Docx.extension(), "docx");
        assert_eq!(doc_convert::DocFormat::Pdf.extension(), "pdf");
        assert_eq!(doc_convert::DocFormat::Txt.extension(), "txt");
    }

    #[test]
    fn test_doc_format_from_extension() {
        assert!(doc_convert::DocFormat::from_extension("docx").is_some());
        assert!(doc_convert::DocFormat::from_extension("pdf").is_some());
        assert!(doc_convert::DocFormat::from_extension("xyz").is_none());
    }

    #[test]
    fn test_convert_functions_exist() {
        let _ = doc_convert::convert_document::<&str, &str>;
        let _ = doc_convert::to_pdf::<&str, &str>;
        let _ = doc_convert::to_docx::<&str, &str>;
        let _ = doc_convert::to_text::<&str, &str>;
        let _ = doc_convert::to_html::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. TEXT EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod text_extract_tests {
    use dx_media::tools::document::text_extract;

    #[test]
    fn test_extract_functions_exist() {
        let _ = text_extract::extract::<&str>;
        let _ = text_extract::extract_to_file::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. PDF WATERMARK TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_watermark_tests {
    use dx_media::tools::document::pdf_watermark;

    #[test]
    fn test_watermark_options() {
        let options = pdf_watermark::WatermarkOptions::default();
        assert!(options.font_size > 0);
        assert!(options.opacity > 0.0 && options.opacity <= 1.0);
    }

    #[test]
    fn test_watermark_functions_exist() {
        let _ = pdf_watermark::text_watermark::<&str, &str>;
        let _ = pdf_watermark::text_watermark_with_options::<&str, &str>;
        let _ = pdf_watermark::image_watermark::<&str, &str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. PDF ENCRYPTION TESTS
// ═══════════════════════════════════════════════════════════════

mod pdf_encrypt_tests {
    use dx_media::tools::document::pdf_encrypt;

    #[test]
    fn test_encryption_strength() {
        let _ = pdf_encrypt::EncryptionStrength::Aes128;
        let _ = pdf_encrypt::EncryptionStrength::Aes256;
    }

    #[test]
    fn test_pdf_permissions() {
        let none = pdf_encrypt::PdfPermissions::none();
        assert!(!none.allow_printing);
        assert!(!none.allow_copying);

        let all = pdf_encrypt::PdfPermissions::all();
        assert!(all.allow_printing);
        assert!(all.allow_copying);

        let view_print = pdf_encrypt::PdfPermissions::view_and_print();
        assert!(view_print.allow_printing);
        assert!(!view_print.allow_copying);
    }

    #[test]
    fn test_encrypt_options_with_password() {
        let options = pdf_encrypt::EncryptOptions::with_password("secret123");
        assert_eq!(options.owner_password, "secret123");
    }

    #[test]
    fn test_encrypt_options_with_passwords() {
        let options = pdf_encrypt::EncryptOptions::with_passwords("user", "owner");
        assert_eq!(options.user_password, Some("user".to_string()));
        assert_eq!(options.owner_password, "owner");
    }

    #[test]
    fn test_encrypt_functions_exist() {
        let _ = pdf_encrypt::encrypt::<&str, &str>;
        let _ = pdf_encrypt::encrypt_with_options::<&str, &str>;
        let _ = pdf_encrypt::decrypt::<&str, &str>;
        let _ = pdf_encrypt::is_encrypted::<&str>;
        let _ = pdf_encrypt::remove_restrictions::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// DOCUMENT TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod document_tools_tests {
    use super::*;

    #[test]
    fn test_document_tools_instantiation() {
        let tools = document::DocumentTools::new();
        drop(tools);
    }

    #[test]
    fn test_document_tools_default() {
        let tools = document::DocumentTools::default();
        drop(tools);
    }

    #[test]
    fn test_check_ghostscript() {
        // May pass or fail depending on installation
        let _ = document::check_ghostscript();
    }

    #[test]
    fn test_check_pdftk() {
        // May pass or fail depending on installation
        let _ = document::check_pdftk();
    }
}
