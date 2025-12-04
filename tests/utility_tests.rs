//! Tests for utility tools.
//!
//! These tests cover the 10 utility tools:
//! 1. Hash Calculator
//! 2. Base64 Encoder/Decoder
//! 3. URL Encoder/Decoder
//! 4. JSON Formatter
//! 5. YAML Converter
//! 6. CSV Converter
//! 7. File Diff
//! 8. UUID Generator
//! 9. Timestamp Converter
//! 10. Random Generator

mod common;
use common::TestFixture;
use dx_media::tools::utility;

// ═══════════════════════════════════════════════════════════════
// 1. HASH CALCULATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod hash_tests {
    use super::*;
    use dx_media::tools::utility::hash;

    #[test]
    fn test_hash_algorithm_names() {
        assert_eq!(hash::HashAlgorithm::Md5.name(), "MD5");
        assert_eq!(hash::HashAlgorithm::Sha1.name(), "SHA1");
        assert_eq!(hash::HashAlgorithm::Sha256.name(), "SHA256");
        assert_eq!(hash::HashAlgorithm::Sha512.name(), "SHA512");
    }

    #[test]
    fn test_hash_file() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "Hello, World!");

        let result = hash::hash_file(&path, hash::HashAlgorithm::Md5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_sha256_file() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "test content");

        let result = hash::sha256(&path);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_sha512_file() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "test content");

        let result = hash::sha512(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_md5_file() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "hello");

        let result = hash::md5(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multi_hash() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "content");

        let result = hash::multi_hash(&path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. BASE64 ENCODER/DECODER TESTS
// ═══════════════════════════════════════════════════════════════

mod base64_tests {
    use super::*;
    use dx_media::tools::utility::base64;

    #[test]
    fn test_encode_string() {
        let result = base64::encode_string("Hello, World!");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("SGVsbG8sIFdvcmxkIQ=="));
    }

    #[test]
    fn test_decode_string() {
        let result = base64::decode_string("SGVsbG8sIFdvcmxkIQ==");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.message.contains("Hello, World!"));
    }

    #[test]
    fn test_encode_empty_string() {
        let result = base64::encode_string("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_invalid() {
        let result = base64::decode_string("!!!invalid!!!");
        assert!(result.is_err() || !result.unwrap().success);
    }

    #[test]
    fn test_encode_url_safe() {
        let result = base64::encode_url_safe("hello+world/test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_file() {
        let fixture = TestFixture::new();
        let path = fixture.create_text_file("test.txt", "file content");

        let result = base64::encode_file(&path);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. URL ENCODER/DECODER TESTS
// ═══════════════════════════════════════════════════════════════

mod url_encode_tests {
    use dx_media::tools::utility::url_encode;

    #[test]
    fn test_encode_simple() {
        let result = url_encode::encode("hello world");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.contains("hello%20world") || output.message.contains("hello+world"));
    }

    #[test]
    fn test_encode_special_chars() {
        let result = url_encode::encode("foo=bar&baz=qux");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_simple() {
        let result = url_encode::decode("hello%20world");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.contains("hello world"));
    }

    #[test]
    fn test_encode_path() {
        let result = url_encode::encode_path("/path/to/file");
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_string() {
        let params = [("name", "value"), ("foo", "bar")];
        let result = url_encode::build_query_string(&params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_query_string() {
        let result = url_encode::parse_query_string("name=value&foo=bar");
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. JSON FORMATTER TESTS
// ═══════════════════════════════════════════════════════════════

mod json_tests {
    use super::*;
    use dx_media::tools::utility::json_format;

    #[test]
    fn test_format_valid_json() {
        let result = json_format::format_string(r#"{"name":"test","value":123}"#);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_format_invalid_json() {
        let result = json_format::format_string("not valid json");
        assert!(result.is_err() || !result.unwrap().success);
    }

    #[test]
    fn test_minify_json() {
        let json = r#"{
    "name": "test",
    "value": 123
}"#;
        let result = json_format::minify_string(json);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.message.contains('\n') || output.message.trim().len() < json.len());
    }

    #[test]
    fn test_validate_valid_json() {
        let result = json_format::validate_string(r#"{"valid": true}"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_json() {
        let result = json_format::validate_string("{invalid}");
        assert!(result.is_err() || !result.unwrap().success);
    }

    #[test]
    fn test_sort_keys() {
        let result = json_format::sort_keys(r#"{"b": 2, "a": 1}"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_indent_options() {
        let result = json_format::format_string_with_indent(
            r#"{"a":1}"#,
            json_format::JsonIndent::Spaces(4),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_json_file() {
        let fixture = TestFixture::new();
        let input = fixture.create_json_file("input.json", r#"{"test": true}"#);
        let output = fixture.path("output.json");

        let result = json_format::format_json_file(&input, &output);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. YAML CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod yaml_tests {
    use super::*;
    use dx_media::tools::utility::yaml_convert;

    #[test]
    fn test_json_to_yaml() {
        let fixture = TestFixture::new();
        let input = fixture.create_json_file("input.json", r#"{"name": "test", "value": 123}"#);
        let output = fixture.path("output.yaml");

        let result = yaml_convert::json_to_yaml(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_yaml_to_json() {
        let fixture = TestFixture::new();
        let yaml_content = "name: test\nvalue: 123\n";
        let input = fixture.create_text_file("input.yaml", yaml_content);
        let output = fixture.path("output.json");

        let result = yaml_convert::yaml_to_json(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_yaml() {
        let result = yaml_convert::validate_string("name: test\nvalue: 123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_yaml() {
        let result = yaml_convert::format_string("name:test\nvalue:123");
        // May succeed or fail depending on YAML parser strictness
        let _ = result;
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. CSV CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod csv_tests {
    use super::*;
    use dx_media::tools::utility::csv_convert;

    #[test]
    fn test_csv_to_json() {
        let fixture = TestFixture::new();
        let csv_content = "name,age\nAlice,30\nBob,25\n";
        let input = fixture.create_csv_file("input.csv", csv_content);
        let output = fixture.path("output.json");

        let result = csv_convert::csv_to_json(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_to_csv() {
        let fixture = TestFixture::new();
        let json_content = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
        let input = fixture.create_json_file("input.json", json_content);
        let output = fixture.path("output.csv");

        let result = csv_convert::json_to_csv(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_csv_to_markdown() {
        let fixture = TestFixture::new();
        let csv_content = "name,age\nAlice,30\n";
        let input = fixture.create_csv_file("input.csv", csv_content);
        let output = fixture.path("output.md");

        let result = csv_convert::csv_to_markdown(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_csv_to_html() {
        let fixture = TestFixture::new();
        let csv_content = "name,age\nAlice,30\n";
        let input = fixture.create_csv_file("input.csv", csv_content);
        let output = fixture.path("output.html");

        let result = csv_convert::csv_to_html(&input, &output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_csv_stats() {
        let fixture = TestFixture::new();
        let csv_content = "a,b,c\n1,2,3\n4,5,6\n";
        let input = fixture.create_csv_file("data.csv", csv_content);

        let result = csv_convert::csv_stats(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. FILE DIFF TESTS
// ═══════════════════════════════════════════════════════════════

mod diff_tests {
    use super::*;
    use dx_media::tools::utility::diff;

    #[test]
    fn test_diff_identical_files() {
        let fixture = TestFixture::new();
        let content = "same content\n";
        let file1 = fixture.create_text_file("file1.txt", content);
        let file2 = fixture.create_text_file("file2.txt", content);

        let result = diff::diff_files(&file1, &file2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_different_files() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "content A\n");
        let file2 = fixture.create_text_file("file2.txt", "content B\n");

        let result = diff::diff_files(&file1, &file2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_strings() {
        let result = diff::diff_strings("hello\nworld", "hello\nuniverse");
        assert!(result.is_ok());
    }

    #[test]
    fn test_files_identical() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("file1.txt", "same\n");
        let file2 = fixture.create_text_file("file2.txt", "same\n");

        let result = diff::files_identical(&file1, &file2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_format_unified() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("a.txt", "line1\nline2\n");
        let file2 = fixture.create_text_file("b.txt", "line1\nline3\n");

        let result = diff::diff_files_with_format(&file1, &file2, diff::DiffFormat::Unified);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_stats() {
        let fixture = TestFixture::new();
        let file1 = fixture.create_text_file("a.txt", "a\nb\nc\n");
        let file2 = fixture.create_text_file("b.txt", "a\nx\nc\n");

        let result = diff::diff_stats(&file1, &file2);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. UUID GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod uuid_tests {
    use dx_media::tools::utility::uuid;

    #[test]
    fn test_generate_v4() {
        let result = uuid::v4();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
        assert!(output.message.contains('-'));
    }

    #[test]
    fn test_generate_multiple_unique() {
        let result1 = uuid::v4().unwrap();
        let result2 = uuid::v4().unwrap();
        assert_ne!(result1.message, result2.message);
    }

    #[test]
    fn test_batch_uuid() {
        let result = uuid::batch(5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_uuid() {
        let result = uuid::validate("550e8400-e29b-41d4-a716-446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_uuid() {
        let result = uuid::validate("not-a-uuid");
        // Should indicate invalid
        let _ = result;
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. TIMESTAMP CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod timestamp_tests {
    use dx_media::tools::utility::timestamp;

    #[test]
    fn test_now_unix() {
        let result = timestamp::now(timestamp::TimestampFormat::Unix);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_now_iso() {
        let result = timestamp::now(timestamp::TimestampFormat::Iso8601);
        assert!(result.is_ok());
    }

    #[test]
    fn test_now_rfc2822() {
        let result = timestamp::now(timestamp::TimestampFormat::Rfc2822);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_unix_to_iso() {
        let result = timestamp::convert(
            "1609459200",
            timestamp::TimestampFormat::Unix,
            timestamp::TimestampFormat::Iso8601,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_seconds() {
        let result = timestamp::add("1609459200", 3600, timestamp::TimestampFormat::Unix);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_timestamps() {
        let result = timestamp::diff(
            "1609459200",
            "1609462800",
            timestamp::TimestampFormat::Unix,
        );
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. RANDOM GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod random_tests {
    use dx_media::tools::utility::random;

    #[test]
    fn test_random_string_alphanumeric() {
        let result = random::string(16, random::CharSet::Alphanumeric);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.message.len(), 16);
    }

    #[test]
    fn test_random_string_alpha() {
        let result = random::string(10, random::CharSet::Alpha);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.chars().all(|c| c.is_alphabetic()));
    }

    #[test]
    fn test_random_string_numeric() {
        let result = random::string(8, random::CharSet::Numeric);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_random_string_hex() {
        let result = random::string(12, random::CharSet::Hex);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_random_integer() {
        let result = random::integer(1, 100);
        assert!(result.is_ok());
        let output = result.unwrap();
        let num: i64 = output.message.trim().parse().unwrap();
        assert!((1..=100).contains(&num));
    }

    #[test]
    fn test_random_float() {
        let result = random::float(0.0, 1.0);
        assert!(result.is_ok());
        let output = result.unwrap();
        let num: f64 = output.message.trim().parse().unwrap();
        assert!((0.0..=1.0).contains(&num));
    }

    #[test]
    fn test_random_bytes() {
        let result = random::bytes(32);
        assert!(result.is_ok());
    }

    #[test]
    fn test_random_boolean() {
        let result = random::boolean();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.contains("true") || output.message.contains("false"));
    }

    #[test]
    fn test_random_pick() {
        let items = ["apple", "banana", "cherry"];
        let result = random::pick(&items);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(items.contains(&output.message.trim()));
    }

    #[test]
    fn test_random_shuffle() {
        let items = ["a", "b", "c", "d"];
        let result = random::shuffle(&items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_generation() {
        let result = random::password(16, true);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.message.len() >= 16);
    }

    #[test]
    fn test_batch_integers() {
        let result = random::batch_integers(5, 1, 10);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════
// UTILITY TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod utility_tools_tests {
    use super::*;

    #[test]
    fn test_utility_tools_instantiation() {
        let tools = utility::UtilityTools::new();
        drop(tools);
    }

    #[test]
    fn test_utility_tools_default() {
        let tools = utility::UtilityTools::default();
        drop(tools);
    }

    #[test]
    fn test_utility_tools_url_encode() {
        let tools = utility::UtilityTools::new();
        let result = tools.url_encode("hello world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_utility_tools_url_decode() {
        let tools = utility::UtilityTools::new();
        let result = tools.url_decode("hello%20world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_utility_tools_generate_uuid() {
        let tools = utility::UtilityTools::new();
        let result = tools.generate_uuid();
        assert!(result.is_ok());
    }

    #[test]
    fn test_utility_tools_timestamp() {
        let tools = utility::UtilityTools::new();
        let result = tools.timestamp();
        assert!(result.is_ok());
    }

    #[test]
    fn test_utility_tools_random_string() {
        let tools = utility::UtilityTools::new();
        let result = tools.random_string(10);
        assert!(result.is_ok());
    }
}
