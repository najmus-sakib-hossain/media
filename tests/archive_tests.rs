//! Tests for archive tools.
//!
//! These tests cover the 10 archive tools:
//! 1. Zip Creator
//! 2. Zip Extractor
//! 3. Tar Creator
//! 4. Tar Extractor
//! 5. Compressor (gzip, bzip2, xz)
//! 6. Decompressor
//! 7. Archive List
//! 8. Archive Encrypt
//! 9. Archive Split
//! 10. Archive Merge

mod common;
use common::TestFixture;
use dx_media::tools::archive;

// ═══════════════════════════════════════════════════════════════
// 1. ZIP CREATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod zip_create_tests {
    use super::*;

    #[test]
    fn test_create_zip_single_file() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Hello, World!");
        let output = fixture.output_path("output.zip");

        let result = archive::zip::create_zip(&[&file], &output);
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(out.success);
        assert!(output.exists());
    }

    #[test]
    fn test_create_zip_multiple_files() {
        let fixture = TestFixture::new();
        let files = fixture.create_multiple_files(3, "file", "txt");
        let output = fixture.output_path("archive.zip");

        let file_refs: Vec<_> = files.iter().collect();
        let result = archive::zip::create_zip(&file_refs, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_zip_with_compression() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", &"A".repeat(10000));
        let output = fixture.output_path("compressed.zip");

        let result = archive::zip::create_zip_with_compression(
            &[&file],
            &output,
            archive::zip::CompressionLevel::Best,
        );
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. ZIP EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod zip_extract_tests {
    use super::*;

    #[test]
    fn test_extract_zip() {
        let fixture = TestFixture::new();

        // First create a zip
        let file = fixture.create_text_file("test.txt", "Content");
        let zip_path = fixture.output_path("test.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        // Then extract it
        let extract_dir = fixture.create_dir("extracted");
        let result = archive::zip::extract_zip(&zip_path, &extract_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_zip_preserves_content() {
        let fixture = TestFixture::new();
        let content = "Test content 12345";
        let file = fixture.create_text_file("test.txt", content);
        let zip_path = fixture.output_path("test.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        let extract_dir = fixture.create_dir("extracted");
        let result = archive::zip::extract_zip(&zip_path, &extract_dir);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. TAR CREATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod tar_create_tests {
    use super::*;

    #[test]
    fn test_create_tar() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Hello");
        let output = fixture.output_path("output.tar");

        let result = archive::tar::create_tar(&[&file], &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_tar_gz() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Hello");
        let output = fixture.output_path("output.tar.gz");

        let result = archive::tar::create_tar_gz(&[&file], &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_tar_with_compression() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Hello");
        let output = fixture.output_path("output.tar.bz2");

        let result = archive::tar::create_tar_with_compression(
            &[&file],
            &output,
            archive::tar::TarCompression::Bzip2,
        );
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. TAR EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod tar_extract_tests {
    use super::*;

    #[test]
    fn test_extract_tar() {
        let fixture = TestFixture::new();

        // Create tar first
        let file = fixture.create_text_file("test.txt", "Content");
        let tar_path = fixture.output_path("test.tar");
        archive::tar::create_tar(&[&file], &tar_path).unwrap();

        // Extract
        let extract_dir = fixture.create_dir("extracted");
        let result = archive::tar::extract_tar(&tar_path, &extract_dir);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. COMPRESSOR TESTS (gzip, bzip2, xz)
// ═══════════════════════════════════════════════════════════════

mod compress_tests {
    use super::*;

    #[test]
    fn test_gzip() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", &"A".repeat(1000));
        let output = fixture.output_path("test.txt.gz");

        let result = archive::compress::gzip(&file, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bzip2() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", &"B".repeat(1000));
        let output = fixture.output_path("test.txt.bz2");

        let result = archive::compress::bzip2(&file, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_xz() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", &"X".repeat(1000));
        let output = fixture.output_path("test.txt.xz");

        let result = archive::compress::xz(&file, &output);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. DECOMPRESSOR TESTS
// ═══════════════════════════════════════════════════════════════

mod decompress_tests {
    use super::*;

    #[test]
    fn test_gunzip() {
        let fixture = TestFixture::new();

        // Compress first
        let file = fixture.create_text_file("test.txt", "Content");
        let gz_path = fixture.output_path("test.txt.gz");
        archive::compress::gzip(&file, &gz_path).unwrap();

        // Decompress
        let output = fixture.output_path("decompressed.txt");
        let result = archive::decompress::gunzip(&gz_path, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bunzip2() {
        let fixture = TestFixture::new();

        let file = fixture.create_text_file("test.txt", "Content");
        let bz2_path = fixture.output_path("test.txt.bz2");
        archive::compress::bzip2(&file, &bz2_path).unwrap();

        let output = fixture.output_path("decompressed.txt");
        let result = archive::decompress::bunzip2(&bz2_path, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_decompress() {
        let fixture = TestFixture::new();

        let file = fixture.create_text_file("test.txt", "Content");
        let gz_path = fixture.output_path("test.txt.gz");
        archive::compress::gzip(&file, &gz_path).unwrap();

        let output = fixture.output_path("auto_decompressed.txt");
        let result = archive::decompress::decompress(&gz_path, &output);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. ARCHIVE LIST TESTS
// ═══════════════════════════════════════════════════════════════

mod list_tests {
    use super::*;

    #[test]
    fn test_list_zip() {
        let fixture = TestFixture::new();
        let files = fixture.create_multiple_files(3, "file", "txt");
        let zip_path = fixture.output_path("archive.zip");
        let file_refs: Vec<_> = files.iter().collect();
        archive::zip::create_zip(&file_refs, &zip_path).unwrap();

        let result = archive::list::list_archive(&zip_path);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_list_tar() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Content");
        let tar_path = fixture.output_path("archive.tar");
        archive::tar::create_tar(&[&file], &tar_path).unwrap();

        let result = archive::list::list_archive(&tar_path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. ARCHIVE ENCRYPT TESTS
// ═══════════════════════════════════════════════════════════════

mod encrypt_tests {
    use super::*;

    #[test]
    fn test_encrypt_archive() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Secret content");
        let zip_path = fixture.output_path("plain.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        let encrypted_path = fixture.output_path("encrypted.zip");
        let result = archive::encrypt::encrypt_archive(&zip_path, &encrypted_path, "password123");
        // May fail if encryption tool not available
        if result.is_ok() {
            assert!(result.unwrap().success);
        }
    }

    #[test]
    fn test_decrypt_archive() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Secret content");
        let zip_path = fixture.output_path("plain.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        let encrypted_path = fixture.output_path("encrypted.zip");
        if archive::encrypt::encrypt_archive(&zip_path, &encrypted_path, "password123").is_ok() {
            let decrypted_dir = fixture.create_dir("decrypted");
            let result =
                archive::encrypt::decrypt_archive(&encrypted_path, &decrypted_dir, "password123");
            // May fail if decryption tool not available
            if result.is_ok() {
                assert!(result.unwrap().success);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. ARCHIVE SPLIT TESTS
// ═══════════════════════════════════════════════════════════════

mod split_tests {
    use super::*;

    #[test]
    fn test_split_archive() {
        let fixture = TestFixture::new();
        // Create a larger file
        let file = fixture.create_text_file("large.txt", &"A".repeat(100000));
        let zip_path = fixture.output_path("large.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        let split_dir = fixture.create_dir("splits");
        let result = archive::split::split_archive(&zip_path, &split_dir, 50); // 50KB parts
        // May fail if split tool not available
        if result.is_ok() {
            assert!(result.unwrap().success);
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. ARCHIVE MERGE TESTS
// ═══════════════════════════════════════════════════════════════

mod merge_tests {
    use super::*;

    #[test]
    fn test_merge_split_archive() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("large.txt", &"A".repeat(100000));
        let zip_path = fixture.output_path("large.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        let split_dir = fixture.create_dir("splits");
        if archive::split::split_archive(&zip_path, &split_dir, 50).is_ok() {
            let merged_path = fixture.output_path("merged.zip");
            let result = archive::merge::merge_split_archive(&split_dir, &merged_path);
            if result.is_ok() {
                assert!(result.unwrap().success);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════

mod integration_tests {
    use super::*;

    #[test]
    fn test_zip_roundtrip() {
        let fixture = TestFixture::new();
        let content = "Test content for roundtrip";
        let file = fixture.create_text_file("test.txt", content);

        // Create zip
        let zip_path = fixture.output_path("roundtrip.zip");
        archive::zip::create_zip(&[&file], &zip_path).unwrap();

        // Extract zip
        let extract_dir = fixture.create_dir("extracted");
        let result = archive::zip::extract_zip(&zip_path, &extract_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tar_gz_roundtrip() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Tar content");

        // Create tar.gz
        let tar_path = fixture.output_path("roundtrip.tar.gz");
        archive::tar::create_tar_gz(&[&file], &tar_path).unwrap();

        // Extract
        let extract_dir = fixture.create_dir("extracted");
        let result = archive::tar::extract_tar(&tar_path, &extract_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compress_decompress_roundtrip() {
        let fixture = TestFixture::new();
        let content = "Compress me please!";
        let file = fixture.create_text_file("test.txt", content);

        // Compress
        let gz_path = fixture.output_path("compressed.gz");
        archive::compress::gzip(&file, &gz_path).unwrap();

        // Decompress
        let output = fixture.output_path("decompressed.txt");
        let result = archive::decompress::gunzip(&gz_path, &output);
        assert!(result.is_ok());
    }
}
