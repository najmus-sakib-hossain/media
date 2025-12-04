//! Common test utilities and fixtures.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test fixture for creating temporary test files.
pub struct TestFixture {
    pub temp_dir: TempDir,
}

impl TestFixture {
    /// Create a new test fixture with a temporary directory.
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp dir"),
        }
    }

    /// Get the path to the temporary directory.
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Create a file with given content.
    pub fn create_file(&self, name: &str, content: &[u8]) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        let mut file = File::create(&path).expect("Failed to create file");
        file.write_all(content).expect("Failed to write file");
        path
    }

    /// Create a text file.
    pub fn create_text_file(&self, name: &str, content: &str) -> PathBuf {
        self.create_file(name, content.as_bytes())
    }

    /// Create a JSON file.
    pub fn create_json_file(&self, name: &str, content: &str) -> PathBuf {
        self.create_text_file(name, content)
    }

    /// Create a directory.
    pub fn create_dir(&self, name: &str) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        fs::create_dir_all(&path).expect("Failed to create directory");
        path
    }

    /// Get output path for a file.
    pub fn output_path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    /// Create a sample image (1x1 PNG).
    pub fn create_sample_image(&self, name: &str) -> PathBuf {
        // Minimal valid PNG (1x1 transparent pixel)
        let png_data: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR length
            0x49, 0x48, 0x44, 0x52, // IHDR
            0x00, 0x00, 0x00, 0x01, // width: 1
            0x00, 0x00, 0x00, 0x01, // height: 1
            0x08, 0x06, // 8-bit RGBA
            0x00, 0x00, 0x00, // compression, filter, interlace
            0x1F, 0x15, 0xC4, 0x89, // CRC
            0x00, 0x00, 0x00, 0x0A, // IDAT length
            0x49, 0x44, 0x41, 0x54, // IDAT
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, // compressed data
            0x0D, 0x0A, 0x2D, 0xB4, // CRC
            0x00, 0x00, 0x00, 0x00, // IEND length
            0x49, 0x45, 0x4E, 0x44, // IEND
            0xAE, 0x42, 0x60, 0x82, // CRC
        ];
        self.create_file(name, png_data)
    }

    /// Create a sample text file for testing.
    pub fn create_sample_text(&self, name: &str) -> PathBuf {
        self.create_text_file(
            name,
            "Hello, World!\nThis is a test file.\nLine 3 of the test.",
        )
    }

    /// Create multiple test files.
    pub fn create_multiple_files(&self, count: usize, prefix: &str, ext: &str) -> Vec<PathBuf> {
        (0..count)
            .map(|i| {
                let name = format!("{}{}.{}", prefix, i, ext);
                self.create_text_file(&name, &format!("Content of file {}", i))
            })
            .collect()
    }
}

impl Default for TestFixture {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a command is available on the system.
pub fn command_available(cmd: &str) -> bool {
    std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
}

/// Skip test if command is not available.
#[macro_export]
macro_rules! skip_if_no_command {
    ($cmd:expr) => {
        if !$crate::common::command_available($cmd) {
            eprintln!("Skipping test: {} not available", $cmd);
            return;
        }
    };
}
