//! Tests for utility tools.
//!
//! These tests cover the 10 utility tools:
//! 1. Hash Calculator
//! 2. Base64 Encoder/Decoder
//! 3. URL Encoder/Decoder
//! 4. JSON Formatter
//! 5. YAML Converter
//! 6. CSV Converter
//! 7. Diff Tool
//! 8. UUID Generator
//! 9. Timestamp Tool
//! 10. Random Generator

mod common;
use common::TestFixture;
use dx_media::tools::utility;

// ═══════════════════════════════════════════════════════════════
// 1. HASH CALCULATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod hash_tests {
    use super::*;

    #[test]
    fn test_hash_string_md5() {
        let result = utility::hash::hash_string("hello", utility::hash::HashAlgorithm::Md5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // MD5 of "hello" is 5d41402abc4b2a76b9719d911017c592
        assert!(output.message.contains("5d41402abc4b2a76b9719d911017c592"));
    }

    #[test]
    fn test_hash_string_sha256() {
        let result = utility::hash::hash_string("hello", utility::hash::HashAlgorithm::Sha256);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // SHA256 of "hello" starts with 2cf24dba
        assert!(output.message.to_lowercase().contains("2cf24dba"));
    }

    #[test]
    fn test_hash_file() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "hello");

        let result = utility::hash::hash_file(&file, utility::hash::HashAlgorithm::Md5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_verify_hash() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "hello");

        let result = utility::hash::verify_hash(
            &file,
            "5d41402abc4b2a76b9719d911017c592",
            utility::hash::HashAlgorithm::Md5,
        );
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. BASE64 ENCODER/DECODER TESTS
// ═══════════════════════════════════════════════════════════════

mod base64_tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        let result = utility::base64::encode_string("Hello, World!");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("SGVsbG8sIFdvcmxkIQ=="));
    }

    #[test]
    fn test_decode_string() {
        let result = utility::base64::decode_string("SGVsbG8sIFdvcmxkIQ==");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("Hello, World!"));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = "Test string with special chars: äöü 🎉";
        let encoded = utility::base64::encode_string(original).unwrap();
        let decoded = utility::base64::decode_string(&encoded.message).unwrap();
        assert!(decoded.message.contains("Test string"));
    }

    #[test]
    fn test_encode_file() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("test.txt", "Hello");

        let result = utility::base64::encode_file(&file);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_decode_file() {
        let fixture = TestFixture::new();
        let file = fixture.create_text_file("encoded.txt", "SGVsbG8=");
        let output_path = fixture.output_path("decoded.txt");

        let result = utility::base64::decode_file(&file, &output_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_valid_base64() {
        assert!(utility::base64::is_valid_base64("SGVsbG8="));
        assert!(utility::base64::is_valid_base64("YWJjZA=="));
        assert!(!utility::base64::is_valid_base64("Invalid!@#$"));
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. URL ENCODER/DECODER TESTS
// ═══════════════════════════════════════════════════════════════

mod url_encode_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let result = utility::url_encode::encode("hello world");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("hello%20world") || output.message.contains("hello+world"));
    }

    #[test]
    fn test_decode() {
        let result = utility::url_encode::decode("hello%20world");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("hello world"));
    }

    #[test]
    fn test_encode_special_chars() {
        let result = utility::url_encode::encode("key=value&foo=bar");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_roundtrip() {
        let original = "Hello World! Test=123&foo=bar";
        let encoded = utility::url_encode::encode(original).unwrap();
        let decoded = utility::url_encode::decode(&encoded.message).unwrap();
        assert!(decoded.message.contains("Hello World"));
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. JSON FORMATTER TESTS
// ═══════════════════════════════════════════════════════════════

mod json_tests {
    use super::*;

    #[test]
    fn test_format_json() {
        let result = utility::json_format::format_json(r#"{"name":"test","value":123}"#);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("name"));
    }

    #[test]
    fn test_minify_json() {
        let result = utility::json_format::minify_json(
            r#"{
            "name": "test",
            "value": 123
        }"#,
        );
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.message.contains('\n') || output.message.len() < 50);
    }

    #[test]
    fn test_validate_json_valid() {
        let result = utility::json_format::validate_json(r#"{"valid": true}"#);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_validate_json_invalid() {
        let result = utility::json_format::validate_json(r#"{"invalid": }"#);
        assert!(result.is_ok());
        let output = result.unwrap();
        // Should indicate invalid JSON
        assert!(!output.success || output.message.to_lowercase().contains("invalid"));
    }

    #[test]
    fn test_format_json_file() {
        let fixture = TestFixture::new();
        let input = fixture.create_json_file("input.json", r#"{"key":"value"}"#);
        let output_path = fixture.output_path("output.json");

        let result = utility::json_format::format_json_file(&input, &output_path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. YAML CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod yaml_tests {
    use super::*;

    #[test]
    fn test_json_to_yaml() {
        let result = utility::yaml_convert::json_to_yaml(r#"{"name":"test","value":123}"#);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("name"));
    }

    #[test]
    fn test_yaml_to_json() {
        let yaml = "name: test\nvalue: 123";
        let result = utility::yaml_convert::yaml_to_json(yaml);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_validate_yaml_valid() {
        let result = utility::yaml_convert::validate_yaml("key: value\nlist:\n  - item1\n  - item2");
        assert!(result.is_ok());
    }

    #[test]
    fn test_yaml_roundtrip() {
        let json = r#"{"name":"test","count":42}"#;
        let yaml = utility::yaml_convert::json_to_yaml(json).unwrap();
        let back_to_json = utility::yaml_convert::yaml_to_json(&yaml.message).unwrap();
        assert!(back_to_json.message.contains("name"));
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. CSV CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod csv_tests {
    use super::*;

    #[test]
    fn test_csv_to_json() {
        let csv = "name,age\nAlice,30\nBob,25";
        let result = utility::csv_convert::csv_to_json(csv);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("Alice") || output.message.contains("name"));
    }

    #[test]
    fn test_json_to_csv() {
        let json = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
        let result = utility::csv_convert::json_to_csv(json);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_parse_csv() {
        let csv = "a,b,c\n1,2,3\n4,5,6";
        let result = utility::csv_convert::parse_csv(csv);
        assert!(result.is_ok());
    }

    #[test]
    fn test_csv_with_custom_delimiter() {
        let csv = "name;age\nAlice;30";
        let result = utility::csv_convert::csv_to_json_with_delimiter(csv, ';');
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. DIFF TOOL TESTS
// ═══════════════════════════════════════════════════════════════

mod diff_tests {
    use super::*;

    #[test]
    fn test_diff_strings() {
        let result = utility::diff::diff_strings("hello world", "hello rust");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_diff_identical() {
        let result = utility::diff::diff_strings("same text", "same text");
        assert!(result.is_ok());
        let output = result.unwrap();
        // Identical strings should report no differences
        assert!(output.message.to_lowercase().contains("identical")
            || output.message.to_lowercase().contains("same")
            || output.message.is_empty()
            || output.metadata.get("identical").map(|v| v == "true").unwrap_or(false));
    }

    #[test]
    fn test_diff_files() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "line1\nline2\nline3");
        let file2 = fixture.create_text_file("file2.txt", "line1\nmodified\nline3");

        let result = utility::diff::diff_files(&file1, &file2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_line_diff() {
        let result = utility::diff::line_diff("line1\nline2", "line1\nline2\nline3");
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. UUID GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod uuid_tests {
    use super::*;

    #[test]
    fn test_generate_v4() {
        let result = utility::uuid::generate_v4();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
        assert!(output.message.len() == 36);
        assert!(output.message.contains('-'));
    }

    #[test]
    fn test_generate_multiple() {
        let result = utility::uuid::generate_multiple(5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_validate_uuid_valid() {
        let result = utility::uuid::validate("550e8400-e29b-41d4-a716-446655440000");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_validate_uuid_invalid() {
        let result = utility::uuid::validate("not-a-valid-uuid");
        assert!(result.is_ok());
        let output = result.unwrap();
        // Should indicate invalid
        assert!(!output.success || output.message.to_lowercase().contains("invalid"));
    }

    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = utility::uuid::generate_v4().unwrap().message;
        let uuid2 = utility::uuid::generate_v4().unwrap().message;
        assert_ne!(uuid1, uuid2);
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. TIMESTAMP TOOL TESTS
// ═══════════════════════════════════════════════════════════════

mod timestamp_tests {
    use super::*;

    #[test]
    fn test_now() {
        let result = utility::timestamp::now();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_unix_to_datetime() {
        let result = utility::timestamp::unix_to_datetime(1700000000);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // Should contain year 2023
        assert!(output.message.contains("2023"));
    }

    #[test]
    fn test_datetime_to_unix() {
        let result = utility::timestamp::datetime_to_unix("2023-11-14T12:00:00Z");
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_timestamp() {
        let result = utility::timestamp::format_unix(1700000000, "%Y-%m-%d");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.contains("2023"));
    }

    #[test]
    fn test_parse_timestamp() {
        let result = utility::timestamp::parse("2023-11-14", "%Y-%m-%d");
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. RANDOM GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod random_tests {
    use super::*;

    #[test]
    fn test_random_string() {
        let result = utility::random::random_string(16);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.len() >= 16);
    }

    #[test]
    fn test_random_number() {
        let result = utility::random::random_number(1, 100);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_random_bytes() {
        let result = utility::random::random_bytes(32);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_random_password() {
        let result = utility::random::random_password(16);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.len() >= 16);
    }

    #[test]
    fn test_random_hex() {
        let result = utility::random::random_hex(16);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // Hex string should only contain 0-9, a-f
        assert!(output.message.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_random_uniqueness() {
        let r1 = utility::random::random_string(32).unwrap().message;
        let r2 = utility::random::random_string(32).unwrap().message;
        assert_ne!(r1, r2);
    }
}
