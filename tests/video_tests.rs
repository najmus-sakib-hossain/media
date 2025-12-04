//! Tests for video tools.
//!
//! These tests cover the 11 video tools:
//! 1. Format Transcoder
//! 2. Audio Extractor
//! 3. Video Trimmer
//! 4. GIF Maker
//! 5. Thumbnail Generator
//! 6. Resolution Scaler
//! 7. Video Concatenator
//! 8. Mute Video
//! 9. Video Watermark
//! 10. Speed Changer
//! 11. Subtitle Handler
//!
//! Note: These tests require FFmpeg to be installed.

mod common;
use common::TestFixture;
use dx_media::tools::video;

// ═══════════════════════════════════════════════════════════════
// 1. FORMAT TRANSCODER TESTS
// ═══════════════════════════════════════════════════════════════

mod transcoder_tests {
    use dx_media::tools::video::transcoder;

    #[test]
    fn test_video_format_extensions() {
        assert_eq!(transcoder::VideoFormat::Mp4.extension(), "mp4");
        assert_eq!(transcoder::VideoFormat::WebM.extension(), "webm");
        assert_eq!(transcoder::VideoFormat::Mkv.extension(), "mkv");
        assert_eq!(transcoder::VideoFormat::Avi.extension(), "avi");
        assert_eq!(transcoder::VideoFormat::Mov.extension(), "mov");
        assert_eq!(transcoder::VideoFormat::Gif.extension(), "gif");
    }

    #[test]
    fn test_video_format_from_str() {
        assert!(transcoder::VideoFormat::from_str("mp4").is_some());
        assert!(transcoder::VideoFormat::from_str("webm").is_some());
        assert!(transcoder::VideoFormat::from_str("xyz").is_none());
    }

    #[test]
    fn test_video_quality_crf() {
        assert_eq!(transcoder::VideoQuality::Low.crf(), 28);
        assert_eq!(transcoder::VideoQuality::Medium.crf(), 23);
        assert_eq!(transcoder::VideoQuality::High.crf(), 18);
        assert_eq!(transcoder::VideoQuality::VeryHigh.crf(), 15);
        assert_eq!(transcoder::VideoQuality::Lossless.crf(), 0);
    }

    #[test]
    fn test_video_format_codec_args() {
        let args = transcoder::VideoFormat::Mp4.codec_args();
        assert!(!args.is_empty());
    }

    #[test]
    fn test_transcode_options_default() {
        let options = transcoder::TranscodeOptions::default();
        assert!(matches!(options.format, transcoder::VideoFormat::Mp4));
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. AUDIO EXTRACTOR TESTS
// ═══════════════════════════════════════════════════════════════

mod audio_extract_tests {
    use dx_media::tools::video::audio_extract;

    #[test]
    fn test_audio_format_extensions() {
        assert_eq!(audio_extract::AudioFormat::Mp3.extension(), "mp3");
        assert_eq!(audio_extract::AudioFormat::Wav.extension(), "wav");
        assert_eq!(audio_extract::AudioFormat::Aac.extension(), "aac");
        assert_eq!(audio_extract::AudioFormat::Flac.extension(), "flac");
        assert_eq!(audio_extract::AudioFormat::Ogg.extension(), "ogg");
    }

    #[test]
    fn test_audio_format_from_str() {
        assert!(audio_extract::AudioFormat::from_str("mp3").is_some());
        assert!(audio_extract::AudioFormat::from_str("wav").is_some());
        assert!(audio_extract::AudioFormat::from_str("xyz").is_none());
    }

    #[test]
    fn test_audio_extract_options() {
        let options = audio_extract::AudioExtractOptions::new(audio_extract::AudioFormat::Mp3);
        assert!(matches!(options.format, audio_extract::AudioFormat::Mp3));
    }

    #[test]
    fn test_audio_extract_options_builder() {
        let options = audio_extract::AudioExtractOptions::new(audio_extract::AudioFormat::Mp3)
            .with_bitrate("192k")
            .with_sample_rate(44100)
            .stereo();

        assert_eq!(options.bitrate, Some("192k".to_string()));
        assert_eq!(options.sample_rate, Some(44100));
        assert_eq!(options.channels, Some(2));
    }
}

// ═══════════════════════════════════════════════════════════════
// 3. VIDEO TRIMMER TESTS
// ═══════════════════════════════════════════════════════════════

mod trimmer_tests {
    use dx_media::tools::video::trimmer;

    #[test]
    fn test_trim_mode_enum() {
        let _ = trimmer::TrimMode::Copy;
        let _ = trimmer::TrimMode::Reencode;
    }

    #[test]
    fn test_trim_options_new() {
        let options = trimmer::TrimOptions::new(10.0, 30.0);
        assert_eq!(options.start, 10.0);
        assert_eq!(options.end, 30.0);
    }

    #[test]
    fn test_trim_options_with_duration() {
        let options = trimmer::TrimOptions::with_duration(10.0, 20.0);
        assert_eq!(options.start, 10.0);
        assert_eq!(options.end, 30.0); // start + duration
    }

    #[test]
    fn test_trim_options_builder() {
        let options = trimmer::TrimOptions::new(0.0, 60.0)
            .with_mode(trimmer::TrimMode::Reencode)
            .with_keyframe_seek();

        assert!(matches!(options.mode, trimmer::TrimMode::Reencode));
        assert!(options.keyframe_seek);
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(trimmer::parse_time("60"), Some(60.0));
        assert_eq!(trimmer::parse_time("1:00"), Some(60.0));
        assert_eq!(trimmer::parse_time("1:30"), Some(90.0));
        assert!(trimmer::parse_time("invalid").is_none());
    }
}

// ═══════════════════════════════════════════════════════════════
// 4. GIF MAKER TESTS
// ═══════════════════════════════════════════════════════════════

mod gif_maker_tests {
    use dx_media::tools::video::gif_maker;

    #[test]
    fn test_gif_options_default() {
        let options = gif_maker::GifOptions::default();
        assert!(options.width > 0);
        assert!(options.fps > 0);
    }

    #[test]
    fn test_gif_options_with_width() {
        let options = gif_maker::GifOptions::with_width(480);
        assert_eq!(options.width, 480);
    }

    #[test]
    fn test_gif_options_builder() {
        let options = gif_maker::GifOptions::with_width(320)
            .with_fps(15)
            .with_range(5.0, 10.0)
            .with_colors(128)
            .with_loop(0);

        assert_eq!(options.width, 320);
        assert_eq!(options.fps, 15);
        assert_eq!(options.start, Some(5.0));
        assert_eq!(options.duration, Some(10.0));
        assert_eq!(options.colors, 128);
        assert_eq!(options.loop_count, 0);
    }

    #[test]
    fn test_gif_options_fast_mode() {
        let options = gif_maker::GifOptions::default().fast_mode();
        assert!(options.fast);
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. THUMBNAIL GENERATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod thumbnail_tests {
    use dx_media::tools::video::thumbnail;

    #[test]
    fn test_thumbnail_options_default() {
        let options = thumbnail::ThumbnailOptions::default();
        assert!(options.timestamp >= 0.0);
    }

    #[test]
    fn test_thumbnail_options_builder() {
        let options = thumbnail::ThumbnailOptions::default()
            .at(5.0)
            .with_width(640)
            .with_height(480)
            .with_quality(90);

        assert_eq!(options.timestamp, 5.0);
        assert_eq!(options.width, Some(640));
        assert_eq!(options.height, Some(480));
        assert_eq!(options.quality, 90);
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. RESOLUTION SCALER TESTS
// ═══════════════════════════════════════════════════════════════

mod scaler_tests {
    use dx_media::tools::video::scaler;

    #[test]
    fn test_resolution_presets() {
        assert_eq!(scaler::Resolution::Sd480.dimensions(), (854, 480));
        assert_eq!(scaler::Resolution::Hd720.dimensions(), (1280, 720));
        assert_eq!(scaler::Resolution::Hd1080.dimensions(), (1920, 1080));
        assert_eq!(scaler::Resolution::Uhd4k.dimensions(), (3840, 2160));
    }

    #[test]
    fn test_custom_resolution() {
        let res = scaler::Resolution::Custom(800, 600);
        assert_eq!(res.dimensions(), (800, 600));
    }

    #[test]
    fn test_scale_options() {
        let options = scaler::ScaleOptions::default();
        let _ = options;
    }
}

// ═══════════════════════════════════════════════════════════════
// 7. VIDEO CONCATENATOR TESTS
// ═══════════════════════════════════════════════════════════════

mod concatenate_tests {
    use dx_media::tools::video::concatenate;

    #[test]
    fn test_concat_method() {
        let _ = concatenate::ConcatMethod::Demuxer;
        let _ = concatenate::ConcatMethod::Filter;
        let _ = concatenate::ConcatMethod::Reencode;
    }

    #[test]
    fn test_concat_options_with_filter() {
        let options = concatenate::ConcatOptions::with_filter();
        assert!(matches!(options.method, concatenate::ConcatMethod::Filter));
    }

    #[test]
    fn test_concat_options_with_reencode() {
        let options = concatenate::ConcatOptions::with_reencode(1920, 1080);
        assert!(matches!(options.method, concatenate::ConcatMethod::Reencode));
        assert_eq!(options.width, Some(1920));
        assert_eq!(options.height, Some(1080));
    }
}

// ═══════════════════════════════════════════════════════════════
// 8. MUTE VIDEO TESTS
// ═══════════════════════════════════════════════════════════════

mod mute_tests {
    use dx_media::tools::video::mute;

    #[test]
    fn test_mute_options_fast() {
        let options = mute::MuteOptions::fast();
        assert!(options.copy_video);
    }

    #[test]
    fn test_mute_options_reencode() {
        let options = mute::MuteOptions::reencode(23);
        assert!(!options.copy_video);
        assert_eq!(options.quality, Some(23));
    }

    #[test]
    fn test_mute_functions_exist() {
        let _ = mute::mute_video::<&str, &str>;
        let _ = mute::replace_audio::<&str, &str, &str>;
        let _ = mute::add_audio::<&str, &str, &str>;
        let _ = mute::adjust_volume::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// 9. VIDEO WATERMARK TESTS
// ═══════════════════════════════════════════════════════════════

mod watermark_tests {
    use dx_media::tools::video::watermark;

    #[test]
    fn test_watermark_position_enum() {
        let _ = watermark::WatermarkPosition::TopLeft;
        let _ = watermark::WatermarkPosition::TopRight;
        let _ = watermark::WatermarkPosition::BottomLeft;
        let _ = watermark::WatermarkPosition::BottomRight;
        let _ = watermark::WatermarkPosition::Center;
        let _ = watermark::WatermarkPosition::Custom(100, 100);
    }

    #[test]
    fn test_text_watermark_options() {
        let options = watermark::TextWatermarkOptions::default();
        assert!(options.font_size > 0);
    }

    #[test]
    fn test_image_watermark_options() {
        let options = watermark::ImageWatermarkOptions::default();
        assert!(options.opacity > 0.0 && options.opacity <= 1.0);
    }
}

// ═══════════════════════════════════════════════════════════════
// 10. SPEED CHANGER TESTS
// ═══════════════════════════════════════════════════════════════

mod speed_tests {
    use dx_media::tools::video::speed;

    #[test]
    fn test_speed_options() {
        let options = speed::SpeedOptions::default();
        assert!(options.factor > 0.0);
    }

    #[test]
    fn test_speed_options_builder() {
        let options = speed::SpeedOptions::default()
            .with_factor(2.0)
            .with_audio_pitch_correction();

        assert_eq!(options.factor, 2.0);
        assert!(options.preserve_pitch);
    }
}

// ═══════════════════════════════════════════════════════════════
// 11. SUBTITLE HANDLER TESTS
// ═══════════════════════════════════════════════════════════════

mod subtitle_tests {
    use dx_media::tools::video::subtitle;

    #[test]
    fn test_subtitle_format() {
        let _ = subtitle::SubtitleFormat::Srt;
        let _ = subtitle::SubtitleFormat::Ass;
        let _ = subtitle::SubtitleFormat::Vtt;
    }

    #[test]
    fn test_subtitle_functions_exist() {
        let _ = subtitle::burn_subtitles::<&str, &str, &str>;
        let _ = subtitle::extract_subtitles::<&str, &str>;
    }
}

// ═══════════════════════════════════════════════════════════════
// VIDEO TOOLS COLLECTION TESTS
// ═══════════════════════════════════════════════════════════════

mod video_tools_tests {
    use super::*;

    #[test]
    fn test_video_tools_instantiation() {
        let tools = video::VideoTools::new();
        drop(tools);
    }

    #[test]
    fn test_video_tools_default() {
        let tools = video::VideoTools::default();
        drop(tools);
    }

    #[test]
    fn test_check_ffmpeg() {
        // This may pass or fail depending on FFmpeg installation
        let _ = video::check_ffmpeg();
    }

    #[test]
    fn test_ffmpeg_version() {
        // This may return Some or None depending on installation
        let _ = video::ffmpeg_version();
    }
}
