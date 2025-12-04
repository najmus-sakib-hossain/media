//! Common test utilities and fixtures.

use std::path::PathBuf;
use tempfile::TempDir;

/// Test fixture helper providing temporary directories and test file creation.
pub struct TestFixture {
    /// Temporary directory for test files.
    pub temp_dir: TempDir,
}

impl TestFixture {
    /// Create a new test fixture with a temporary directory.
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
        }
    }

    /// Get path for a file in the temp directory.
    pub fn path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    /// Create a test text file with given content.
    pub fn create_text_file(&self, name: &str, content: &str) -> PathBuf {
        let path = self.path(name);
        std::fs::write(&path, content).expect("Failed to write test file");
        path
    }

    /// Create a test JSON file.
    pub fn create_json_file(&self, name: &str, json: &str) -> PathBuf {
        self.create_text_file(name, json)
    }

    /// Create a test CSV file.
    pub fn create_csv_file(&self, name: &str, content: &str) -> PathBuf {
        self.create_text_file(name, content)
    }

    /// Create a binary file with given bytes.
    pub fn create_binary_file(&self, name: &str, data: &[u8]) -> PathBuf {
        let path = self.path(name);
        std::fs::write(&path, data).expect("Failed to write binary file");
        path
    }

    /// Check if a file exists.
    pub fn exists(&self, name: &str) -> bool {
        self.path(name).exists()
    }

    /// Read file contents as string.
    pub fn read_string(&self, name: &str) -> String {
        std::fs::read_to_string(self.path(name)).expect("Failed to read file")
    }
}

impl Default for TestFixture {
    fn default() -> Self {
        Self::new()
    }
}
