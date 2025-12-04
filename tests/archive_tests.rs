//! Tests for archive tools.
//!
//! These tests cover the 10 archive tools:
//! 1. ZIP Creator/Extractor
//! 2. TAR Creator/Extractor
//! 3. GZIP Compressor
//! 4. BZIP2 Compressor
//! 5. XZ Compressor
//! 6. 7z Creator/Extractor
//! 7. Archive Encryption
//! 8. Archive Splitter
//! 9. Archive Merger
//! 10. Archive Lister

mod common;
use common::TestFixture;
use dx_media::tools::archive;

// ═══════════════════════════════════════════════════════════════
// 1. ZIP TOOLS TESTS
// ═══════════════════════════════════════════════════════════════

mod zip_tests {
    use super::*;
    use dx_media::tools::archive::zip;

    #[test]
    fn test_create_zip() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "content 1");
        let file2 = fixture.create_text_file("file2.txt", "content 2");
        let output = fixture.path("archive.zip");

        let files = [file1, file2];
        let result = zip::create_zip(&files, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_extract_zip() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "test content");
        let zip_path = fixture.path("test.zip");
        let extract_dir = fixture.path("extracted");

        // Create zip first
        let result = zip::create_zip(&[&file1], &zip_path);
        assert!(result.is_ok());

        // Extract it
        let result = zip::extract_zip(&zip_path, &extract_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_zip() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("a.txt", "a");
        let zip_path = fixture.path("list.zip");

        let _ = zip::create_zip(&[&file1], &zip_path);
        let result = zip::list_zip(&zip_path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. TAR TOOLS TESTS
// ═══════════════════════════════════════════════════════════════

mod tar_tests {
    use super::*;
    use dx_media::tools::archive::tar;

    #[test]
    fn test_tar_compression_enum() {
        assert_eq!(tar::TarCompression::None.tar_flag(), None);
        assert_eq!(tar::TarCompression::Gzip.tar_flag(), Some("-z"));
        assert_eq!(tar::TarCompression::Bzip2.tar_flag(), Some("-j"));
        assert_eq!(tar::TarCompression::Xz.tar_flag(), Some("-J"));
    }

    #[test]
    fn test_create_tar() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "content");
        let output = fixture.path("archive.tar");

        let files = [file1];
        let result = tar::create_tar(&files, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_tar_gz() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file.txt", "compressed content");
        let output = fixture.path("archive.tar.gz");

        let result = tar::create_tar_gz(&[&file1], &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tar() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "test");
        let tar_path = fixture.path("test.tar");
        let extract_dir = fixture.path("out");

        let _ = tar::create_tar(&[&file1], &tar_path);
        let result = tar::extract_tar(&tar_path, &extract_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_tar() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "data");
        let tar_path = fixture.path("list.tar");

        let _ = tar::create_tar(&[&file1], &tar_path);
        let result = tar::list_tar(&tar_path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. GZIP COMPRESSION TESTS
// ═══════════════════════════════════════════════════════════════

mod gzip_tests {
    use super::*;
    use dx_media::tools::archive::compress;

    #[test]
    fn test_gzip_compress() {
        let fixture = TestFixture::new();
        let input = fixture.create_text_file("data.txt", "uncompressed data content\n".repeat(100).as_str());
        let output = fixture.path("data.txt.gz");

        let result = compress::gzip(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_compression_algorithm_extension() {
        assert_eq!(compress::CompressionAlgorithm::Gzip.extension(), "gz");
        assert_eq!(compress::CompressionAlgorithm::Bzip2.extension(), "bz2");
        assert_eq!(compress::CompressionAlgorithm::Xz.extension(), "xz");
    }

    #[test]
    fn test_compression_level() {
        assert_eq!(compress::CompressionLevel::Fast.level(), 1);
        assert_eq!(compress::CompressionLevel::Default.level(), 6);
        assert_eq!(compress::CompressionLevel::Best.level(), 9);
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. BZIP2 COMPRESSION TESTS
// ═══════════════════════════════════════════════════════════════

mod bzip2_tests {
    use super::*;
    use dx_media::tools::archive::compress;

    #[test]
    fn test_bzip2_compress() {
        let fixture = TestFixture::new();
        let input = fixture.create_text_file("data.txt", "data to compress");
        let output = fixture.path("data.txt.bz2");

        let result = compress::bzip2(&input, &output);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. XZ COMPRESSION TESTS
// ═══════════════════════════════════════════════════════════════

mod xz_tests {
    use super::*;
    use dx_media::tools::archive::compress;

    #[test]
    fn test_xz_compress() {
        let fixture = TestFixture::new();
        let input = fixture.create_text_file("data.txt", "data to compress with xz");
        let output = fixture.path("data.txt.xz");

        let result = compress::xz(&input, &output);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. DECOMPRESSION TESTS
// ═══════════════════════════════════════════════════════════════

mod decompress_tests {
    use super::*;
    use dx_media::tools::archive::{compress, decompress};

    #[test]
    fn test_gunzip() {
        let fixture = TestFixture::new();
        let original = fixture.create_text_file("data.txt", "test data");
        let compressed = fixture.path("data.txt.gz");
        let decompressed = fixture.path("data_restored.txt");

        let _ = compress::gzip(&original, &compressed);
        let result = decompress::gunzip(&compressed, &decompressed);
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_decompress() {
        let fixture = TestFixture::new();
        let original = fixture.create_text_file("data.txt", "test");
        let compressed = fixture.path("data.txt.gz");
        let output = fixture.path("output.txt");

        let _ = compress::gzip(&original, &compressed);
        let result = decompress::auto_decompress(&compressed, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_integrity_check() {
        let fixture = TestFixture::new();
        let original = fixture.create_text_file("data.txt", "test");
        let compressed = fixture.path("data.txt.gz");

        let _ = compress::gzip(&original, &compressed);
        let result = decompress::test_integrity(&compressed);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. 7Z TOOLS TESTS
// ═══════════════════════════════════════════════════════════════

mod sevenzip_tests {
    use super::*;
    use dx_media::tools::archive::sevenzip;

    #[test]
    fn test_7z_compression_level() {
        assert_eq!(sevenzip::SevenZipLevel::Fast.level(), 1);
        assert_eq!(sevenzip::SevenZipLevel::Normal.level(), 5);
        assert_eq!(sevenzip::SevenZipLevel::Ultra.level(), 9);
    }

    // Note: These tests require 7z to be installed
    #[test]
    #[ignore = "requires 7z installation"]
    fn test_create_7z() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file.txt", "content");
        let output = fixture.path("archive.7z");

        let result = sevenzip::create_7z(&[&file1], &output);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "requires 7z installation"]
    fn test_extract_7z() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file.txt", "content");
        let archive = fixture.path("archive.7z");
        let extract_dir = fixture.path("out");

        let _ = sevenzip::create_7z(&[&file1], &archive);
        let result = sevenzip::extract_7z(&archive, &extract_dir);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. ARCHIVE ENCRYPTION TESTS
// ═══════════════════════════════════════════════════════════════

mod encrypt_tests {
    use super::*;
    use dx_media::tools::archive::encrypt;

    #[test]
    fn test_encryption_method_enum() {
        let _ = encrypt::EncryptionMethod::ZipCrypto;
        let _ = encrypt::EncryptionMethod::Aes256;
    }

    // Note: These tests require appropriate tools installed
    #[test]
    #[ignore = "requires 7z/zip with encryption support"]
    fn test_create_encrypted_zip() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("secret.txt", "secret content");
        let output = fixture.path("encrypted.zip");

        let result = encrypt::create_encrypted_zip(&[&file1], &output, "password123");
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "requires 7z installation"]
    fn test_create_encrypted_7z() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("secret.txt", "secret content");
        let output = fixture.path("encrypted.7z");

        let result = encrypt::create_encrypted_7z(&[&file1], &output, "password123");
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. ARCHIVE SPLITTER TESTS
// ═══════════════════════════════════════════════════════════════

mod split_tests {
    use super::*;
    use dx_media::tools::archive::split;

    #[test]
    fn test_calculate_split_size() {
        let total = 100 * 1024 * 1024; // 100 MB
        let max_parts = 10;
        let size = split::calculate_split_size(total, max_parts);
        assert!(size > 0);
    }

    #[test]
    fn test_get_size_mb() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "some content");

        let result = split::get_size_mb(&file);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "requires splitting tool"]
    fn test_split_archive() {
        let fixture = TestFixture::new();
        // Create a larger file for splitting
        let content = "x".repeat(1024 * 1024); // 1 MB
        let file = fixture.create_text_file("large.txt", &content);
        let output_dir = fixture.path("parts");

        let result = split::split_archive(&file, &output_dir, 256); // 256 KB parts
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. ARCHIVE MERGER TESTS
// ═══════════════════════════════════════════════════════════════

mod merge_tests {
    use super::*;
    use dx_media::tools::archive::merge;

    #[test]
    #[ignore = "requires split archives to merge"]
    fn test_merge_archives() {
        let fixture = TestFixture::new();
        // This would need actual split archive parts
        let part1 = fixture.path("archive.001");
        let part2 = fixture.path("archive.002");
        let output = fixture.path("merged.zip");

        // Would fail without actual parts, hence ignored
        let result = merge::merge_archives(&[&part1, &part2], &output);
        let _ = result;
    }

    #[test]
    fn test_find_related_parts() {
        let fixture = TestFixture::new();
        // Create simulated part files
        fixture.create_text_file("archive.001", "part1");
        fixture.create_text_file("archive.002", "part2");
        fixture.create_text_file("archive.003", "part3");

        let first = fixture.path("archive.001");
        let result = merge::find_related_parts(&first);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// ARCHIVE LIST TESTS
// ═══════════════════════════════════════════════════════════════

mod list_tests {
    use super::*;
    use dx_media::tools::archive::{list, zip};

    #[test]
    fn test_list_archive() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "a");
        let file2 = fixture.create_text_file("file2.txt", "b");
        let archive = fixture.path("archive.zip");

        let _ = zip::create_zip(&[&file1, &file2], &archive);

        let result = list::list_archive(&archive);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_archive_info() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "content");
        let archive = fixture.path("info.zip");

        let _ = zip::create_zip(&[&file1], &archive);

        let result = list::get_archive_info(&archive);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_filtered() {
        let fixture = TestFixture::new();
        let txt = fixture.create_text_file("doc.txt", "text");
        let csv = fixture.create_text_file("data.csv", "a,b");
        let archive = fixture.path("mixed.zip");

        let _ = zip::create_zip(&[&txt, &csv], &archive);

        let result = list::list_filtered(&archive, &["txt"]);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// ARCHIVE TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod archive_tools_tests {
    use super::*;

    #[test]
    fn test_archive_tools_instantiation() {
        let tools = archive::ArchiveTools::new();
        drop(tools);
    }

    #[test]
    fn test_archive_tools_default() {
        let tools = archive::ArchiveTools::default();
        drop(tools);
    }

    #[test]
    fn test_archive_tools_gzip() {
        let fixture = TestFixture::new();
        let input = fixture.create_text_file("data.txt", "content");
        let output = fixture.path("data.gz");

        let tools = archive::ArchiveTools::new();
        let result = tools.gzip(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_tools_gunzip() {
        let fixture = TestFixture::new();
        let original = fixture.create_text_file("data.txt", "content");
        let compressed = fixture.path("data.gz");
        let restored = fixture.path("restored.txt");

        let tools = archive::ArchiveTools::new();
        let _ = tools.gzip(&original, &compressed);
        let result = tools.gunzip(&compressed, &restored);
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_tools_create_zip() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "hello");
        let output = fixture.path("test.zip");

        let tools = archive::ArchiveTools::new();
        let result = tools.create_zip(&[&file1], &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_tools_list() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("test.txt", "hello");
        let archive = fixture.path("test.zip");

        let tools = archive::ArchiveTools::new();
        let _ = tools.create_zip(&[&file1], &archive);

        let result = tools.list(&archive);
        assert!(result.is_ok());
    }
}
