//! Tests for audio tools.
//!
//! These tests cover the 10 audio tools:
//! 1. Format Converter
//! 2. Normalizer
//! 3. Trimmer
//! 4. Merger
//! 5. Spectrum Analyzer
//! 6. Metadata Editor
//! 7. Silence Remover
//! 8. Splitter
//! 9. Effects Processor
//! 10. Speech-to-Text
//!
//! Note: These tests require FFmpeg to be installed.

mod common;
use common::TestFixture;
use dx_media::tools::audio;

// ═══════════════════════════════════════════════════════════════
// 1. FORMAT CONVERTER TESTS
// ═══════════════════════════════════════════════════════════════

mod converter_tests {
    use dx_media::tools::audio::converter;

    #[test]
    fn test_audio_output_format_extensions() {
        assert_eq!(converter::AudioOutputFormat::Mp3.extension(), "mp3");
        assert_eq!(converter::AudioOutputFormat::Wav.extension(), "wav");
        assert_eq!(converter::AudioOutputFormat::Flac.extension(), "flac");
        assert_eq!(converter::AudioOutputFormat::Ogg.extension(), "ogg");
        assert_eq!(converter::AudioOutputFormat::Aac.extension(), "aac");
    }

    #[test]
    fn test_audio_format_from_extension() {
        assert!(converter::AudioOutputFormat::from_extension("mp3").is_some());
        assert!(converter::AudioOutputFormat::from_extension("wav").is_some());
        assert!(converter::AudioOutputFormat::from_extension("xyz").is_none());
    }

    #[test]
    fn test_convert_options_mp3() {
        let options = converter::ConvertOptions::mp3(320);
        assert_eq!(options.bitrate, Some(320));
    }

    #[test]
    fn test_convert_options_flac() {
        let options = converter::ConvertOptions::flac();
        let _ = options;
    }

    #[test]
    fn test_convert_options_wav() {
        let options = converter::ConvertOptions::wav(44100);
        assert_eq!(options.sample_rate, Some(44100));
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. NORMALIZER TESTS
// ═══════════════════════════════════════════════════════════════

mod normalizer_tests {
    use dx_media::tools::audio::normalize;

    #[test]
    fn test_normalize_method() {
        let _ = normalize::NormalizeMethod::Peak;
        let _ = normalize::NormalizeMethod::Loudness;
        let _ = normalize::NormalizeMethod::Rms;
    }

    #[test]
    fn test_normalize_options_peak() {
        let options = normalize::NormalizeOptions::peak();
        assert!(matches!(options.method, normalize::NormalizeMethod::Peak));
    }

    #[test]
    fn test_normalize_options_broadcast() {
        let options = normalize::NormalizeOptions::broadcast();
        assert!(matches!(options.method, normalize::NormalizeMethod::Loudness));
    }

    #[test]
    fn test_normalize_options_streaming() {
        let options = normalize::NormalizeOptions::streaming();
        let _ = options;
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. TRIMMER TESTS
// ═══════════════════════════════════════════════════════════════

mod trimmer_tests {
    use dx_media::tools::audio::trimmer;

    #[test]
    fn test_trim_options_struct() {
        let options = trimmer::TrimOptions {
            start: 10.0,
            end: 30.0,
            fade_in: Some(0.5),
            fade_out: Some(0.5),
        };

        assert_eq!(options.start, 10.0);
        assert_eq!(options.end, 30.0);
    }

    #[test]
    fn test_trim_functions_exist() {
        let _ = trimmer::trim_audio::<&str, &str>;
        let _ = trimmer::trim_with_options::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. MERGER TESTS
// ═══════════════════════════════════════════════════════════════

mod merger_tests {
    use dx_media::tools::audio::merger;

    #[test]
    fn test_merge_method() {
        let _ = merger::MergeMethod::Concatenate;
        let _ = merger::MergeMethod::Overlay;
    }

    #[test]
    fn test_merge_options() {
        let options = merger::MergeOptions::default();
        let _ = options;
    }

    #[test]
    fn test_merge_functions_exist() {
        let _ = merger::merge_audio::<&str, &str>;
        let _ = merger::overlay_audio::<&str, &str, &str>;
        let _ = merger::append_silence::<&str, &str>;
        let _ = merger::prepend_silence::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. SPECTRUM ANALYZER TESTS
// ═══════════════════════════════════════════════════════════════

mod spectrum_tests {
    use dx_media::tools::audio::spectrum;

    #[test]
    fn test_spectrum_options_default() {
        let options = spectrum::SpectrumOptions::default();
        assert!(options.width > 0);
        assert!(options.height > 0);
    }

    #[test]
    fn test_spectrum_type() {
        let _ = spectrum::SpectrumType::Waveform;
        let _ = spectrum::SpectrumType::Spectrogram;
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. METADATA EDITOR TESTS
// ═══════════════════════════════════════════════════════════════

mod metadata_tests {
    use dx_media::tools::audio::metadata;

    #[test]
    fn test_audio_metadata_struct() {
        let meta = metadata::AudioMetadata::default();
        assert!(meta.is_empty());
    }

    #[test]
    fn test_audio_metadata_format() {
        let meta = metadata::AudioMetadata {
            title: Some("Test Song".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            year: Some("2024".to_string()),
            genre: Some("Rock".to_string()),
            track: None,
            ..Default::default()
        };

        let formatted = metadata::format_metadata(&meta);
        assert!(formatted.contains("Test Song") || formatted.contains("title"));
    }

    #[test]
    fn test_metadata_functions_exist() {
        let _ = metadata::read_metadata::<&str>;
        let _ = metadata::strip_metadata::<&str, &str>;
        let _ = metadata::copy_metadata::<&str, &str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. SILENCE REMOVER TESTS
// ═══════════════════════════════════════════════════════════════

mod silence_tests {
    use dx_media::tools::audio::silence;

    #[test]
    fn test_silence_options() {
        let options = silence::SilenceOptions::default();
        assert!(options.threshold_db < 0.0); // Should be negative dB
    }

    #[test]
    fn test_silence_functions_exist() {
        let _ = silence::remove_silence::<&str, &str>;
        let _ = silence::detect_silence::<&str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. SPLITTER TESTS
// ═══════════════════════════════════════════════════════════════

mod splitter_tests {
    use dx_media::tools::audio::splitter;

    #[test]
    fn test_split_method() {
        let _ = splitter::SplitMethod::ByDuration(60.0);
        let _ = splitter::SplitMethod::BySilence { 
            threshold_db: -40.0, 
            min_duration: 0.5 
        };
        let _ = splitter::SplitMethod::ByCount(10);
    }

    #[test]
    fn test_split_functions_exist() {
        let _ = splitter::split_audio::<&str, &str>;
        let _ = splitter::split_by_duration::<&str, &str>;
        let _ = splitter::split_by_silence::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. EFFECTS PROCESSOR TESTS
// ═══════════════════════════════════════════════════════════════

mod effects_tests {
    use dx_media::tools::audio::effects;

    #[test]
    fn test_audio_effects() {
        let fade_in = effects::AudioEffect::FadeIn(2.0);
        let fade_out = effects::AudioEffect::FadeOut(2.0);
        let volume = effects::AudioEffect::Volume(1.5);
        let speed = effects::AudioEffect::Speed(1.25);
        let pitch = effects::AudioEffect::Pitch(1.0);
        let echo = effects::AudioEffect::Echo { delay: 0.5, decay: 0.3 };
        let reverb = effects::AudioEffect::Reverb { room_size: 0.5, damping: 0.5 };
        let bass = effects::AudioEffect::Bass(5.0);
        let treble = effects::AudioEffect::Treble(3.0);

        assert_eq!(fade_in.name(), "fade_in");
        assert_eq!(fade_out.name(), "fade_out");
        assert_eq!(volume.name(), "volume");
        assert_eq!(speed.name(), "speed");
        assert_eq!(pitch.name(), "pitch");
        assert_eq!(echo.name(), "echo");
        assert_eq!(reverb.name(), "reverb");
        assert_eq!(bass.name(), "bass");
        assert_eq!(treble.name(), "treble");
    }

    #[test]
    fn test_apply_effect_function_exists() {
        let _ = effects::apply_effect::<&str, &str>;
        let _ = effects::apply_effects::<&str, &str>;
    }

    #[test]
    fn test_preset_effects() {
        let _ = effects::telephone_effect::<&str, &str>;
        let _ = effects::underwater_effect::<&str, &str>;
        let _ = effects::chipmunk_effect::<&str, &str>;
        let _ = effects::deep_voice_effect::<&str, &str>;
        let _ = effects::robot_effect::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. SPEECH-TO-TEXT TESTS
// ═══════════════════════════════════════════════════════════════

mod speech_tests {
    use dx_media::tools::audio::speech;

    #[test]
    fn test_speech_options() {
        let options = speech::SpeechOptions::default();
        let _ = options;
    }

    #[test]
    fn test_speech_model() {
        let _ = speech::SpeechModel::Whisper;
        let _ = speech::SpeechModel::Vosk;
    }

    #[test]
    fn test_transcribe_function_exists() {
        let _ = speech::transcribe::<&str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// AUDIO TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod audio_tools_tests {
    use super::*;

    #[test]
    fn test_audio_tools_instantiation() {
        let tools = audio::AudioTools::new();
        drop(tools);
    }

    #[test]
    fn test_audio_tools_default() {
        let tools = audio::AudioTools::default();
        drop(tools);
    }

    #[test]
    fn test_check_ffmpeg_audio() {
        // This may pass or fail depending on FFmpeg installation
        let _ = audio::check_ffmpeg_audio();
    }
}
