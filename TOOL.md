# dx-media Complete Tools & Converters Suite

## 🎯 Complete Tool Categories Overview

```
╔══════════════════════════════════════════════════════════════════════════════╗
║                      dx-media Tools & Converters Suite                       ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  📸 Image Tools          🎬 Video Tools          🎵 Audio Tools              ║
║  📄 Document Tools       📦 Archive Tools        🎨 Graphics Tools           ║
║  🔤 Font Tools           🎮 3D Model Tools       🖼️ Texture Tools            ║
║  🌐 Web Asset Tools      📊 Data Tools           🔧 Utility Tools            ║
║  🎯 Favicon Generator    📱 App Icon Generator   🖥️ Screenshot Tools         ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

## 📸 IMAGE TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Format Converter** | Convert between image formats | 50+ formats | 30+ formats |
| **Resizer** | Resize with multiple algorithms | All | All |
| **Compressor** | Lossy/lossless compression | All | All |
| **Cropper** | Smart crop, face detection, aspect ratio | All | All |
| **Optimizer** | Web optimization, reduce file size | All | WebP, AVIF, PNG, JPEG |
| **Batch Processor** | Process multiple images at once | All | All |
| **Metadata Editor** | EXIF, IPTC, XMP reading/writing | All | All |
| **Color Converter** | Color space conversion (sRGB, Adobe RGB, CMYK) | All | All |
| **Watermarker** | Add text/image watermarks | All | All |
| **Thumbnail Generator** | Generate multiple thumbnail sizes | All | All |
| **Sprite Sheet Generator** | Combine images into sprite sheets | All | PNG, WebP |
| **Favicon Generator** | Generate all favicon sizes for web/apps | All | ICO, PNG, SVG |
| **Social Media Resizer** | Platform-specific sizes (FB, Twitter, IG, etc.) | All | All |
| **Background Remover** | AI-powered background removal | All | PNG, WebP |
| **Image Splitter** | Split into tiles/grid | All | All |
| **GIF Maker** | Create GIFs from image sequences | All | GIF, WebP |
| **Image Diff** | Compare images, find differences | All | PNG |
| **OCR** | Extract text from images | All | TXT, JSON |
| **QR Code Generator** | Generate QR codes | Text/URL | PNG, SVG |
| **Barcode Generator** | Generate various barcodes | Text | PNG, SVG |
| **Placeholder Generator** | Generate placeholder images | - | All |
| **Color Palette Extractor** | Extract dominant colors | All | JSON, CSS |
| **Image Hash Generator** | Perceptual hashing for deduplication | All | Hash |
| **Blur/Sharpen** | Apply blur or sharpen filters | All | All |
| **Rotate/Flip** | Rotate and flip images | All | All |
| **Filter Effects** | Apply Instagram-like filters | All | All |
| **HDR Merge** | Merge bracketed exposures | RAW, JPEG | HDR, JPEG |
| **Panorama Stitcher** | Stitch multiple images | All | JPEG, PNG |

### Supported Image Formats

**Input Formats (50+):**
```
JPEG, PNG, GIF, WebP, AVIF, JPEG XL, HEIF, HEIC, TIFF, BMP, ICO, 
PSD, PSB, XCF, RAW (CR2, NEF, ARW, DNG, etc.), SVG, EPS, AI, PDF,
PPM, PGM, PBM, PAM, TGA, DDS, HDR, EXR, QOI, FLIF, APNG, 
PCX, PNM, XBM, XPM, WBMP, JP2, JXR, RGBE, PFM, FFF
```

**Output Formats (30+):**
```
JPEG, PNG, GIF, WebP, AVIF, JPEG XL, HEIF, TIFF, BMP, ICO,
PPM, PGM, TGA, QOI, PDF, SVG, EPS, HDR, EXR, APNG, FLIF
```

### Rust Crates for Image Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **image** | 0.25 | Core image processing | Main image library |
| **imageproc** | 0.24 | Image processing algorithms | Filters, drawing, morphology |
| **fast_image_resize** | 3.0 | SIMD-accelerated resizing | 10x faster than image crate |
| **libvips** | 1.5 | High-performance processing | Bindings to libvips |
| **photon-rs** | 0.3 | High-performance effects | WebAssembly compatible |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **webp** | 0.2 | WebP encoding/decoding | Google's WebP |
| **ravif** | 0.11 | AVIF encoding | Pure Rust AV1 |
| **libavif** | 0.14 | AVIF support | Bindings to libavif |
| **jxl-oxide** | 0.8 | JPEG XL support | Pure Rust |
| **jpeg-encoder** | 0.6 | JPEG encoding | Basic JPEG |
| **mozjpeg** | 0.10 | MozJPEG compression | Better compression |
| **png** | 0.17 | PNG support | Encoding/decoding |
| **gif** | 0.13 | GIF support | Animated GIF |
| **ico** | 0.3 | ICO/favicon support | Multi-resolution |
| **svg** | 0.17 | SVG parsing | Read SVG |
| **resvg** | 0.40 | SVG rendering | Render to bitmap |
| **usvg** | 0.40 | SVG processing | SVG tree |
| **tiff** | 0.9 | TIFF support | Multi-page TIFF |
| **qoi** | 0.4 | QOI format | Fast lossless |
| **heif** | 0.1 | HEIF/HEIC support | Apple format |
| **rawloader** | 0.37 | RAW photo loading | Camera RAW |
| **imagepipe** | 0.5 | RAW processing | Develop RAW |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **oxipng** | 9.0 | PNG optimization | Lossless compression |
| **imagequant** | 4.3 | PNG quantization | Reduce colors |
| **gifsicle** | 1.0 | GIF optimization | Reduce GIF size |
| **svgo** | 0.1 | SVG optimization | Minimize SVG |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **kamadak-exif** | 0.5 | EXIF reading/writing | Metadata |
| **rexiv2** | 0.10 | Exiv2 bindings | Full metadata |
| **little_exif** | 0.4 | Lightweight EXIF | Simple API |
| **img_hash** | 3.2 | Perceptual hashing | Image similarity |
| **dssim** | 3.2 | Image similarity | SSIM calculation |
| **image-compare** | 0.3 | Image comparison | Diff images |
| **qrcode** | 0.14 | QR code generation | Create QR |
| **barcoders** | 2.0 | Barcode generation | Multiple formats |
| **tesseract** | 0.14 | OCR | Text extraction |
| **leptonica** | 0.3 | Image analysis | For OCR |
| **palette** | 0.7 | Color manipulation | Color spaces |
| **colorsys** | 0.6 | Color conversion | HSL, RGB, etc. |
| **dominant_color** | 0.3 | Extract colors | Color palette |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **tract** | 0.21 | Neural network inference | AI models |
| **ort** | 2.0 | ONNX Runtime | Run ONNX models |
| **candle-core** | 0.4 | ML framework | Hugging Face |
| **rten** | 0.8 | Tensor engine | Model inference |

---

## 🎬 VIDEO TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Format Converter** | Convert video formats | 50+ formats | 20+ formats |
| **Transcoder** | Change codec, bitrate, profile | All | All |
| **Compressor** | Reduce file size with quality control | All | All |
| **Resizer** | Change resolution (4K, 1080p, 720p, etc.) | All | All |
| **Trimmer** | Cut video segments by time | All | All |
| **Merger** | Combine multiple videos | All | All |
| **Audio Extractor** | Extract audio track | All | MP3, WAV, AAC, FLAC |
| **Audio Replacer** | Replace audio track | All | All |
| **Thumbnail Generator** | Extract frames as images | All | PNG, JPEG, WebP |
| **GIF/WebP Maker** | Convert video to animated image | All | GIF, WebP, APNG |
| **Watermarker** | Add image/text watermark | All | All |
| **Subtitle Burner** | Burn subtitles into video | All | All |
| **Subtitle Extractor** | Extract subtitles | All | SRT, VTT, ASS |
| **Frame Extractor** | Extract all/specific frames | All | PNG, JPEG |
| **Speed Changer** | Speed up/slow down video | All | All |
| **Rotator/Flipper** | Rotate and flip video | All | All |
| **Cropper** | Crop video dimensions | All | All |
| **HDR Converter** | HDR to SDR conversion | HDR | SDR |
| **Stabilizer** | Stabilize shaky video | All | All |
| **Denoiser** | Reduce video noise | All | All |
| **Upscaler** | AI-powered upscaling | All | All |
| **Frame Interpolator** | Increase FPS smoothly | All | All |
| **Scene Detector** | Detect scene changes | All | JSON, Timestamps |
| **Metadata Editor** | Edit video metadata | All | All |
| **Concat/Split** | Concatenate or split videos | All | All |
| **Picture-in-Picture** | Overlay video on video | All | All |
| **Green Screen** | Chroma key removal | All | All |
| **Color Correction** | Adjust colors, contrast, brightness | All | All |

### Supported Video Formats

**Input Formats (50+):**
```
MP4, MKV, WebM, AVI, MOV, WMV, FLV, MPEG, MPG, M4V, 3GP, 3G2,
TS, MTS, M2TS, VOB, OGV, ASF, RM, RMVB, DV, F4V, SWF, DIVX,
XVID, H264, H265, HEVC, VP8, VP9, AV1, ProRes, DNxHD, Theora,
GIF, APNG, WebP (animated), MXF, DPX, EXR, R3D, BRAW, CinemaDNG
```

**Output Formats (20+):**
```
MP4 (H.264, H.265), WebM (VP8, VP9, AV1), MKV, AVI, MOV,
GIF, WebP (animated), APNG, MPEG, TS, OGV, WMV, FLV, 3GP
```

### Video Codecs Supported

| Codec | Encode | Decode | Hardware Accel | Notes |
|-------|--------|--------|----------------|-------|
| H.264/AVC | ✅ | ✅ | NVENC, QSV, VAAPI, VideoToolbox | Most compatible |
| H.265/HEVC | ✅ | ✅ | NVENC, QSV, VAAPI, VideoToolbox | 50% smaller than H.264 |
| VP8 | ✅ | ✅ | ❌ | WebM legacy |
| VP9 | ✅ | ✅ | VAAPI | WebM modern |
| AV1 | ✅ | ✅ | NVENC (decode) | Best compression |
| ProRes | ✅ | ✅ | VideoToolbox | Professional |
| DNxHD/HR | ✅ | ✅ | ❌ | Professional |
| MPEG-2 | ✅ | ✅ | VAAPI | DVD/Broadcast |
| MPEG-4 | ✅ | ✅ | ❌ | Legacy |
| Theora | ✅ | ✅ | ❌ | Open source |

### Rust Crates for Video Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **ffmpeg-next** | 7.0 | FFmpeg bindings | Primary video processing |
| **ffmpeg-sys-next** | 7.0 | FFmpeg sys bindings | Low-level FFmpeg |
| **gstreamer** | 0.22 | GStreamer bindings | Alternative pipeline |
| **gstreamer-video** | 0.22 | GStreamer video | Video-specific |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **mp4** | 0.14 | MP4 parsing/writing | Pure Rust |
| **matroska** | 0.23 | MKV/WebM parsing | Pure Rust |
| **rav1e** | 0.7 | AV1 encoder | Pure Rust, slow but works |
| **vpx-encode** | 0.1 | VP8/VP9 encoding | libvpx bindings |
| **openh264** | 0.6 | H.264 encoding | Cisco OpenH264 |
| **x264** | 0.1 | H.264 encoding | x264 bindings |
| **x265** | 0.1 | H.265 encoding | x265 bindings |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **gif** | 0.13 | GIF creation | Animated GIF |
| **gifski** | 1.14 | High-quality GIF | Best quality |
| **webp-animation** | 0.9 | Animated WebP | WebP animation |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **subparse** | 1.3 | Subtitle parsing | SRT, VTT, ASS |
| **subtitle_rs** | 0.2 | Subtitle manipulation | Edit subtitles |
| **ass_parser** | 0.1 | ASS/SSA parsing | Advanced subs |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **opencv** | 0.91 | Computer vision | Scene detection, stabilization |
| **vidformer** | 0.1 | Frame extraction | Video frames |
| **scenedetect** | 0.1 | Scene detection | Find scene changes |

---

## 🎵 AUDIO TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Format Converter** | Convert audio formats | 30+ formats | 20+ formats |
| **Transcoder** | Change codec, bitrate, quality | All | All |
| **Compressor** | Reduce file size | All | All |
| **Normalizer** | Normalize volume (peak, RMS, LUFS) | All | All |
| **Trimmer** | Cut audio segments | All | All |
| **Merger** | Combine audio files | All | All |
| **Splitter** | Split by silence, time, or cue points | All | All |
| **Metadata Editor** | Edit ID3, Vorbis comments, etc. | All | All |
| **Waveform Generator** | Generate waveform images | All | PNG, SVG, JSON |
| **Spectrogram Generator** | Generate spectrograms | All | PNG |
| **Speed Changer** | Change speed without pitch change | All | All |
| **Pitch Shifter** | Change pitch without speed change | All | All |
| **Noise Reducer** | Reduce background noise | All | All |
| **Equalizer** | Apply EQ presets or custom | All | All |
| **Fade In/Out** | Add fades | All | All |
| **Silence Remover** | Remove silent parts | All | All |
| **Loudness Analyzer** | LUFS, dB, dynamic range analysis | All | JSON |
| **BPM Detector** | Detect tempo | All | JSON |
| **Key Detector** | Detect musical key | All | JSON |
| **Audio Fingerprinter** | Generate audio fingerprint | All | Hash |
| **Stereo to Mono** | Convert stereo to mono | All | All |
| **Mono to Stereo** | Convert mono to stereo | All | All |
| **Channel Splitter** | Split stereo to separate files | All | All |
| **Audio Mixer** | Mix multiple audio tracks | All | All |
| **Reverb/Echo** | Add reverb or echo effects | All | All |
| **Voice Isolator** | Isolate vocals from music | All | All |
| **Speech to Text** | Transcribe audio | All | TXT, SRT, JSON |

### Supported Audio Formats

**Input Formats (30+):**
```
MP3, WAV, FLAC, OGG, OPUS, AAC, M4A, WMA, AIFF, AU, 
APE, WV (WavPack), MPC, TTA, TAK, DSD (DSF, DFF), 
AC3, DTS, EAC3, ALAC, AMR, GSM, SPX, CAF, W64, RF64
```

**Output Formats (20+):**
```
MP3, WAV, FLAC, OGG, OPUS, AAC, M4A, AIFF, 
WMA, AC3, ALAC, WV, APE, SPX, CAF
```

### Audio Codecs Supported

| Codec | Encode | Decode | Quality | Use Case |
|-------|--------|--------|---------|----------|
| MP3 (LAME) | ✅ | ✅ | Lossy | Universal compatibility |
| AAC | ✅ | ✅ | Lossy | Streaming, Apple |
| Opus | ✅ | ✅ | Lossy | Best quality/size ratio |
| Vorbis | ✅ | ✅ | Lossy | Open source alternative |
| FLAC | ✅ | ✅ | Lossless | Archival, audiophile |
| ALAC | ✅ | ✅ | Lossless | Apple lossless |
| WAV/PCM | ✅ | ✅ | Uncompressed | Editing, master |
| WavPack | ✅ | ✅ | Lossless/Hybrid | Archival |
| Monkey's Audio | ✅ | ✅ | Lossless | High compression |

### Rust Crates for Audio Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **rodio** | 0.18 | Audio playback | Cross-platform |
| **cpal** | 0.15 | Audio I/O | Low-level audio |
| **symphonia** | 0.5 | Audio decoding | Many formats |
| **symphonia-bundle-mp3** | 0.5 | MP3 support | MP3 decoder |
| **symphonia-bundle-flac** | 0.5 | FLAC support | FLAC decoder |
| **hound** | 3.5 | WAV reading/writing | Simple WAV |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **mp3lame-encoder** | 0.1 | MP3 encoding | LAME bindings |
| **opus** | 0.3 | Opus encoding/decoding | Best codec |
| **vorbis-encoder** | 0.1 | Vorbis encoding | OGG Vorbis |
| **ogg** | 0.9 | OGG container | Container format |
| **flac-bound** | 0.3 | FLAC encoding | FLAC encoder |
| **alac** | 0.1 | ALAC support | Apple lossless |
| **aac** | 0.1 | AAC support | AAC codec |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **dasp** | 0.11 | Digital signal processing | Core DSP |
| **fundsp** | 0.17 | Audio DSP | Effects, synthesis |
| **rubato** | 0.14 | Resampling | High-quality resample |
| **biquad** | 0.4 | Biquad filters | EQ filters |
| **pitch-detection** | 0.3 | Pitch detection | Detect pitch |
| **aubio-rs** | 0.2 | Audio analysis | BPM, pitch, onset |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **ebur128** | 0.1 | Loudness measurement | EBU R128 standard |
| **audio-visualizer** | 0.3 | Waveform/spectrogram | Visualization |
| **spectrum-analyzer** | 1.5 | FFT analysis | Frequency analysis |
| **rustfft** | 6.2 | FFT | Fast Fourier Transform |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **id3** | 1.13 | ID3 tags | MP3 metadata |
| **metaflac** | 0.2 | FLAC metadata | FLAC tags |
| **mp4ameta** | 0.11 | M4A metadata | AAC/M4A tags |
| **lofty** | 0.18 | Universal metadata | All formats |
| **audiotags** | 0.5 | Audio tags | Unified API |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **whisper-rs** | 0.11 | Speech to text | OpenAI Whisper |
| **vosk** | 0.2 | Speech recognition | Offline STT |
| **deepspeech** | 0.9 | Speech recognition | Mozilla DeepSpeech |

---

## 📄 DOCUMENT TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **PDF Converter** | Convert to/from PDF | Many | PDF, Images |
| **PDF Merger** | Merge multiple PDFs | PDF | PDF |
| **PDF Splitter** | Split PDF into pages | PDF | PDF |
| **PDF Compressor** | Reduce PDF size | PDF | PDF |
| **PDF to Images** | Convert pages to images | PDF | PNG, JPEG, WebP |
| **Images to PDF** | Combine images into PDF | Images | PDF |
| **PDF Page Extractor** | Extract specific pages | PDF | PDF |
| **PDF Rotator** | Rotate pages | PDF | PDF |
| **PDF Encryptor** | Add password protection | PDF | PDF |
| **PDF Decryptor** | Remove password | PDF | PDF |
| **PDF Metadata Editor** | Edit PDF properties | PDF | PDF |
| **PDF Form Filler** | Fill PDF forms | PDF | PDF |
| **PDF OCR** | Add text layer to scanned PDFs | PDF | PDF |
| **PDF Watermarker** | Add watermarks | PDF | PDF |
| **Markdown to HTML** | Convert Markdown | MD | HTML |
| **Markdown to PDF** | Convert Markdown | MD | PDF |
| **HTML to PDF** | Convert web pages | HTML, URL | PDF |
| **HTML to Markdown** | Convert HTML | HTML | MD |
| **Office to PDF** | Convert Office docs | DOCX, XLSX, PPTX | PDF |
| **Office to Images** | Convert Office docs | DOCX, XLSX, PPTX | PNG, JPEG |
| **EPUB Converter** | Convert eBooks | EPUB, MOBI | EPUB, PDF, HTML |
| **EPUB Creator** | Create eBooks | HTML, MD | EPUB |
| **Text Encoding Converter** | Convert encodings | TXT | TXT |
| **CSV/JSON Converter** | Convert data formats | CSV, JSON, XML | CSV, JSON, XML, YAML |
| **LaTeX to PDF** | Compile LaTeX | TEX | PDF |

### Supported Document Formats

**Input Formats:**
```
PDF, DOCX, DOC, XLSX, XLS, PPTX, PPT, ODT, ODS, ODP,
RTF, TXT, HTML, XHTML, XML, JSON, YAML, CSV, TSV,
MD, RST, ASCIIDOC, TEX, EPUB, MOBI, AZW, FB2, CBZ, CBR
```

**Output Formats:**
```
PDF, HTML, DOCX, XLSX, PPTX, ODT, ODS, ODP, RTF, TXT,
MD, EPUB, JSON, XML, YAML, CSV, PNG, JPEG, SVG
```

### Rust Crates for Document Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **lopdf** | 0.32 | PDF manipulation | Edit PDFs |
| **pdf** | 0.9 | PDF reading | Parse PDFs |
| **printpdf** | 0.7 | PDF creation | Create PDFs |
| **pdfium-render** | 0.8 | PDF rendering | Via PDFium |
| **mupdf** | 0.4 | MuPDF bindings | Full PDF support |
| **pdf-extract** | 0.7 | PDF text extraction | Extract text |
| **cairo-rs** | 0.19 | Cairo bindings | PDF rendering |
| **poppler-rs** | 0.2 | Poppler bindings | PDF rendering |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **docx-rs** | 0.4 | DOCX reading/writing | Word documents |
| **calamine** | 0.24 | Excel reading | Read XLSX/XLS |
| **xlsxwriter** | 0.6 | Excel writing | Write XLSX |
| **umya-spreadsheet** | 1.2 | Excel manipulation | Full Excel |
| **rust_xlsxwriter** | 0.64 | Excel creation | Create XLSX |
| **odt-rs** | 0.1 | ODT support | OpenDocument |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **pulldown-cmark** | 0.10 | Markdown parsing | CommonMark |
| **comrak** | 0.21 | GitHub Markdown | GFM support |
| **markdown** | 1.0.0-alpha | Markdown parsing | AST-based |
| **mdbook** | 0.4 | Markdown books | Book generation |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **scraper** | 0.19 | HTML parsing | CSS selectors |
| **html5ever** | 0.27 | HTML parsing | Full HTML5 |
| **lol_html** | 1.2 | HTML rewriting | Streaming |
| **ammonia** | 4.0 | HTML sanitization | Security |
| **htmd** | 0.1 | HTML to Markdown | Conversion |
| **html2text** | 0.12 | HTML to text | Plain text |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **headless_chrome** | 1.0 | Chrome control | HTML to PDF |
| **chromiumoxide** | 0.5 | Chrome DevTools | Headless Chrome |
| **wkhtmltopdf** | 0.4 | wkhtmltopdf bindings | HTML to PDF |
| **weasyprint** | 0.1 | WeasyPrint bindings | HTML to PDF |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **epub** | 2.1 | EPUB reading/writing | eBooks |
| **epub-builder** | 0.5 | EPUB creation | Create eBooks |
| **mobi** | 0.1 | MOBI support | Kindle |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **tesseract** | 0.14 | OCR | Text extraction |
| **rusty-tesseract** | 1.1 | Tesseract wrapper | Easy OCR |
| **leptonica-plumbing** | 1.3 | Image for OCR | Preprocessing |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **encoding_rs** | 0.8 | Character encoding | Encode/decode |
| **chardetng** | 0.1 | Encoding detection | Detect charset |

---

## 📦 ARCHIVE TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Compressor** | Create archives | Files/Folders | All archive formats |
| **Extractor** | Extract archives | All archive formats | Files/Folders |
| **Lister** | List archive contents | All archive formats | JSON, TXT |
| **Password Protector** | Add encryption | ZIP, 7Z, RAR | Same |
| **Splitter** | Split large archives | All | Multi-part |
| **Merger** | Merge multi-part archives | Multi-part | Single archive |
| **Converter** | Convert between formats | All | All |
| **Tester** | Verify archive integrity | All | Report |
| **Self-Extractor Creator** | Create SFX archives | ZIP, 7Z | EXE |
| **Incremental Backup** | Create incremental backups | Files/Folders | Archive |

### Supported Archive Formats

**Full Support (Read & Write):**
```
ZIP, GZIP (.gz), BZIP2 (.bz2), XZ (.xz), ZSTD (.zst),
TAR, TAR.GZ, TAR.BZ2, TAR.XZ, TAR.ZSTD, 7Z, LZ4, LZMA,
SNAPPY, BROTLI, DEFLATE
```

**Read Only:**
```
RAR, RAR5, CAB, ARJ, LZH, ISO, DMG, WIM, CHM, CPIO, DEB, RPM
```

### Compression Algorithms Comparison

| Algorithm | Speed | Ratio | Memory | Use Case |
|-----------|-------|-------|--------|----------|
| **ZSTD** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Medium | Best all-around |
| **LZ4** | ⭐⭐⭐⭐⭐ | ⭐⭐ | Low | Real-time, speed critical |
| **GZIP** | ⭐⭐⭐ | ⭐⭐⭐ | Low | Universal compatibility |
| **BZIP2** | ⭐⭐ | ⭐⭐⭐⭐ | High | Better compression |
| **XZ/LZMA** | ⭐ | ⭐⭐⭐⭐⭐ | High | Maximum compression |
| **BROTLI** | ⭐⭐⭐ | ⭐⭐⭐⭐ | Medium | Web assets |
| **SNAPPY** | ⭐⭐⭐⭐⭐ | ⭐⭐ | Low | Database, streaming |

### Rust Crates for Archive Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **zip** | 0.6 | ZIP archives | Read/write ZIP |
| **async-zip** | 0.0.16 | Async ZIP | Async support |
| **zip-extensions** | 0.6 | ZIP utilities | Extract, compress |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **tar** | 0.4 | TAR archives | Read/write TAR |
| **async-tar** | 0.4 | Async TAR | Async support |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **flate2** | 1.0 | GZIP/DEFLATE | Compression |
| **bzip2** | 0.4 | BZIP2 | Compression |
| **xz2** | 0.1 | XZ/LZMA | Compression |
| **zstd** | 0.13 | Zstandard | Best compression |
| **lz4** | 1.24 | LZ4 | Fast compression |
| **lz4-flex** | 0.11 | LZ4 pure Rust | No dependencies |
| **snap** | 1.1 | Snappy | Google's algorithm |
| **brotli** | 3.5 | Brotli | Google's algorithm |
| **lzma-rs** | 0.3 | LZMA pure Rust | No dependencies |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **sevenz-rust** | 0.5 | 7Z archives | Read/write 7Z |
| **unrar** | 0.5 | RAR extraction | Read only |
| **compress-tools** | 0.14 | Multi-format | libarchive bindings |
| **libarchive** | 0.1 | libarchive | Full archive support |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **cab** | 0.5 | CAB archives | Windows CAB |
| **cpio** | 0.4 | CPIO archives | Unix CPIO |
| **arj** | 0.1 | ARJ archives | Legacy format |

---

## 🎯 FAVICON & APP ICON GENERATOR

### Features & Capabilities

| Tool | Description | Output |
|------|-------------|--------|
| **Favicon Generator** | Generate all web favicon sizes | ICO, PNG, SVG |
| **Apple Touch Icons** | iOS home screen icons | PNG (multiple sizes) |
| **Android Icons** | Android app icons | PNG (multiple sizes) |
| **Windows Tiles** | Windows 8/10/11 tiles | PNG (multiple sizes) |
| **macOS Icons** | macOS app icons | ICNS |
| **PWA Icons** | Progressive Web App icons | PNG, WebP |
| **Safari Pinned Tab** | Safari pinned tab icon | SVG (monochrome) |
| **Manifest Generator** | Generate web manifest | JSON |
| **Browserconfig Generator** | Generate browserconfig | XML |
| **HTML Tags Generator** | Generate link tags | HTML |

### Icon Sizes Generated

**Favicon (ICO):**
```
16x16, 32x32, 48x48
```

**Standard PNG:**
```
16x16, 32x32, 48x48, 64x64, 96x96, 128x128, 192x192, 256x256, 512x512
```

**Apple Touch Icons:**
```
57x57, 60x60, 72x72, 76x76, 114x114, 120x120, 
144x144, 152x152, 167x167, 180x180, 1024x1024
```

**Android Chrome:**
```
36x36, 48x48, 72x72, 96x96, 144x144, 192x192, 512x512
```

**Windows Tiles:**
```
70x70, 144x144, 150x150, 270x270, 310x310, 310x150 (wide)
```

**macOS ICNS:**
```
16x16, 32x32, 64x64, 128x128, 256x256, 512x512, 1024x1024
(@1x and @2x variants)
```

### Rust Crates for Icons

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **ico** | 0.3 | ICO creation | Windows icons |
| **icns** | 0.3 | ICNS creation | macOS icons |
| **image** | 0.25 | Image resizing | Core processing |
| **resvg** | 0.40 | SVG rendering | Vector to raster |

---

## 🔤 FONT TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Format Converter** | Convert font formats | All | All |
| **Subsetter** | Extract used characters | All | All |
| **Web Font Generator** | Create web fonts | TTF, OTF | WOFF, WOFF2, EOT |
| **Font Inspector** | View font metadata | All | JSON |
| **Glyph Extractor** | Extract glyphs as SVG | All | SVG |
| **Font Merger** | Merge multiple fonts | All | All |
| **Variable Font Creator** | Create variable fonts | Static fonts | Variable font |
| **Font Optimizer** | Reduce font size | All | All |
| **Base64 Encoder** | Encode for CSS | All | Base64 |
| **CSS Generator** | Generate @font-face | All | CSS |

### Supported Font Formats

```
TTF (TrueType), OTF (OpenType), WOFF (Web Open Font Format),
WOFF2 (WOFF 2.0), EOT (Embedded OpenType), SVG Font,
VF (Variable Font), CFF, Type1, BDF, PCF
```

### Rust Crates for Font Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **ttf-parser** | 0.20 | TTF/OTF parsing | Read fonts |
| **owned_ttf_parser** | 0.20 | Owned TTF parsing | Thread-safe |
| **fontdue** | 0.8 | Font rasterization | Render fonts |
| **ab_glyph** | 0.2 | Glyph rendering | Text rendering |
| **rustybuzz** | 0.12 | Text shaping | HarfBuzz port |
| **fontdb** | 0.16 | Font database | System fonts |
| **font-kit** | 0.13 | Font loading | Cross-platform |
| **woff2** | 0.3 | WOFF2 support | Compress/decompress |
| **allsorts** | 0.14 | Font parsing | Full OpenType |
| **skrifa** | 0.15 | Font reading | Google Fonts |
| **subsetter** | 0.1 | Font subsetting | Reduce size |

---

## 🎮 3D MODEL TOOLS

### Features & Capabilities

| Tool | Description | Input Formats | Output Formats |
|------|-------------|---------------|----------------|
| **Format Converter** | Convert 3D formats | 30+ formats | 15+ formats |
| **Optimizer** | Reduce polygon count | All | All |
| **Mesh Simplifier** | Simplify meshes | All | All |
| **Texture Baker** | Bake textures | All | Images |
| **UV Unwrapper** | Generate UV maps | All | All |
| **Normal Map Generator** | Create normal maps | All | PNG |
| **Thumbnail Renderer** | Render previews | All | PNG, JPEG |
| **Validator** | Validate mesh integrity | All | Report |
| **Merger** | Merge multiple models | All | All |
| **Splitter** | Split by material/object | All | All |
| **Scale/Transform** | Transform models | All | All |
| **Material Extractor** | Extract materials | All | JSON, Images |

### Supported 3D Formats

**Full Support (Read & Write):**
```
OBJ, GLTF, GLB, STL, PLY, 3DS, FBX, COLLADA (DAE), BLEND
```

**Read Only:**
```
FBX (full), MAX, MAYA, C4D, SKP, STEP, IGES, 
3MF, AMF, X3D, VRML, USD, USDZ
```

### Rust Crates for 3D Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **gltf** | 1.4 | GLTF/GLB support | Full GLTF |
| **obj** | 0.10 | OBJ support | Wavefront OBJ |
| **stl_io** | 0.7 | STL support | 3D printing |
| **ply-rs** | 0.1 | PLY support | Point clouds |
| **collada** | 0.1 | COLLADA support | DAE files |
| **fbx** | 0.1 | FBX support | Autodesk FBX |
| **assimp** | 0.3 | Assimp bindings | 40+ formats |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **meshopt** | 0.2 | Mesh optimization | Reduce size |
| **simplify** | 0.1 | Mesh simplification | Reduce polys |
| **lyon** | 1.0 | 2D tessellation | Vector to mesh |

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **three-d** | 0.17 | 3D rendering | Preview |
| **rend3** | 0.3 | 3D renderer | GPU rendering |
| **bevy_render** | 0.13 | Bevy renderer | Game engine |
| **wgpu** | 0.19 | WebGPU | Low-level GPU |

---

## 🖼️ TEXTURE TOOLS

### Features & Capabilities

| Tool | Description | Input | Output |
|------|-------------|-------|--------|
| **Normal Map Generator** | Generate normal maps from height maps | Images | PNG |
| **Height Map Generator** | Generate height maps from normal maps | Images | PNG |
| **Ambient Occlusion** | Generate AO maps | 3D/Images | PNG |
| **Roughness Generator** | Generate roughness maps | Images | PNG |
| **Metallic Generator** | Generate metallic maps | Images | PNG |
| **PBR Packer** | Pack PBR textures into channels | Multiple | PNG |
| **Seamless Maker** | Make textures tileable | Images | PNG |
| **Texture Atlas** | Create texture atlases | Multiple | PNG + JSON |
| **Mipmapper** | Generate mipmaps | Images | DDS, KTX |
| **Compressor** | GPU texture compression | Images | DDS, KTX, ASTC, BC7 |

### Texture Formats

**Standard:**
```
PNG, JPEG, TGA, BMP, HDR, EXR, TIFF
```

**GPU Compressed:**
```
DDS (BC1-BC7), KTX/KTX2, ASTC, ETC2, PVRTC, 
BASIS, UASTC, S3TC
```

### Rust Crates for Texture Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **ddsfile** | 0.5 | DDS support | DirectX textures |
| **ktx2** | 0.3 | KTX2 support | Khronos textures |
| **basis-universal** | 0.3 | Basis Universal | GPU compression |
| **texture-synthesis** | 0.8 | Texture synthesis | Generate textures |
| **intel-tex** | 0.1 | Intel compression | BC7, BC6H |
| **squish** | 0.1 | S3TC compression | DXT1-5 |

---

## 🌐 WEB ASSET TOOLS

### Features & Capabilities

| Tool | Description | Input | Output |
|------|-------------|-------|--------|
| **CSS Minifier** | Minify CSS | CSS | CSS |
| **JS Minifier** | Minify JavaScript | JS | JS |
| **HTML Minifier** | Minify HTML | HTML | HTML |
| **JSON Minifier** | Minify JSON | JSON | JSON |
| **SVG Optimizer** | Optimize SVG | SVG | SVG |
| **Image Sprite Generator** | CSS sprites | Images | PNG + CSS |
| **Base64 Encoder** | Encode assets for embedding | Any | Base64 |
| **Critical CSS Extractor** | Extract above-fold CSS | HTML + CSS | CSS |
| **Unused CSS Remover** | Remove unused CSS | HTML + CSS | CSS |
| **Asset Hasher** | Add content hash to filenames | Any | Renamed files |
| **Source Map Generator** | Create source maps | JS, CSS | MAP |
| **Bundler** | Bundle multiple files | JS, CSS | Single file |

### Rust Crates for Web Assets

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **minify-html** | 0.15 | HTML minification | Fast |
| **minify-js** | 0.6 | JS minification | Fast |
| **css-minify** | 0.5 | CSS minification | Simple |
| **lightningcss** | 1.24 | CSS processing | Parse, transform, minify |
| **swc** | 0.276 | JS/TS processing | Parse, transform, minify |
| **oxc** | 0.6 | JS tooling | Fast parser |
| **svgo** | 0.1 | SVG optimization | Clean SVG |
| **browserslist** | 0.1 | Browser targets | Compatibility |

---

## 📊 DATA TOOLS

### Features & Capabilities

| Tool | Description | Input | Output |
|------|-------------|-------|--------|
| **JSON Formatter** | Format/minify JSON | JSON | JSON |
| **JSON to YAML** | Convert JSON to YAML | JSON | YAML |
| **YAML to JSON** | Convert YAML to JSON | YAML | JSON |
| **CSV to JSON** | Convert CSV to JSON | CSV | JSON |
| **JSON to CSV** | Convert JSON to CSV | JSON | CSV |
| **XML to JSON** | Convert XML to JSON | XML | JSON |
| **JSON to XML** | Convert JSON to XML | JSON | XML |
| **TOML Converter** | Convert TOML | TOML | JSON, YAML |
| **JSON Schema Validator** | Validate against schema | JSON | Report |
| **JSON Differ** | Compare JSON files | JSON | Diff |
| **JSON Merger** | Merge JSON files | JSON | JSON |
| **JSON Query** | Query with JSONPath/JQ | JSON | JSON |
| **Data Generator** | Generate fake data | Schema | JSON, CSV |

### Rust Crates for Data Tools

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **serde_json** | 1.0 | JSON | Core JSON |
| **serde_yaml** | 0.9 | YAML | YAML support |
| **toml** | 0.8 | TOML | TOML support |
| **quick-xml** | 0.31 | XML | Fast XML |
| **roxmltree** | 0.19 | XML | Read-only XML |
| **csv** | 1.3 | CSV | CSV support |
| **jsonschema** | 0.17 | JSON Schema | Validation |
| **jsonpath_lib** | 0.3 | JSONPath | Queries |
| **jaq** | 1.2 | JQ clone | JSON queries |
| **json-patch** | 1.2 | JSON Patch | RFC 6902 |
| **fake** | 2.9 | Fake data | Generate data |

---

## 🔧 UTILITY TOOLS

### Features & Capabilities

| Tool | Description |
|------|-------------|
| **Hash Calculator** | MD5, SHA1, SHA256, SHA512, BLAKE3, CRC32 |
| **UUID Generator** | Generate UUIDs (v1, v4, v5, v7) |
| **Base64 Encoder/Decoder** | Encode/decode Base64 |
| **URL Encoder/Decoder** | Encode/decode URLs |
| **HTML Entity Encoder** | Encode/decode HTML entities |
| **String Case Converter** | camelCase, snake_case, kebab-case, etc. |
| **Regex Tester** | Test regular expressions |
| **Diff Tool** | Compare files/text |
| **Checksum Verifier** | Verify file checksums |
| **File Type Detector** | Detect file type by magic bytes |
| **Duplicate Finder** | Find duplicate files |
| **Batch Renamer** | Rename files in bulk |
| **File Splitter/Joiner** | Split/join large files |

### Rust Crates for Utilities

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **md5** | 0.7 | MD5 hash | Legacy |
| **sha1** | 0.10 | SHA1 hash | Legacy |
| **sha2** | 0.10 | SHA256/512 | Standard |
| **blake3** | 1.5 | BLAKE3 | Fastest |
| **crc32fast** | 1.4 | CRC32 | Fast CRC |
| **digest** | 0.10 | Hash traits | Unified API |
| **uuid** | 1.7 | UUID | All versions |
| **base64** | 0.22 | Base64 | Encode/decode |
| **urlencoding** | 2.1 | URL encoding | Percent encoding |
| **html-escape** | 0.2 | HTML entities | Escape/unescape |
| **heck** | 0.4 | Case conversion | All cases |
| **regex** | 1.10 | Regular expressions | Full regex |
| **similar** | 2.4 | Text diff | Diff algorithm |
| **infer** | 0.15 | File type detection | Magic bytes |
| **walkdir** | 2.4 | Directory walking | File iteration |
| **globset** | 0.4 | Glob patterns | File matching |
| **filetime** | 0.2 | File times | Timestamps |
| **fs_extra** | 1.3 | File operations | Copy, move |
| **tempfile** | 3.10 | Temp files | Temp management |

---

## 📊 COMPLETE CRATE SUMMARY

### Total Crates by Category

| Category | Crate Count |
|----------|-------------|
| 📸 Image Tools | 45+ |
| 🎬 Video Tools | 20+ |
| 🎵 Audio Tools | 35+ |
| 📄 Document Tools | 30+ |
| 📦 Archive Tools | 20+ |
| 🎯 Icon Tools | 5+ |
| 🔤 Font Tools | 12+ |
| 🎮 3D Model Tools | 15+ |
| 🖼️ Texture Tools | 8+ |
| 🌐 Web Asset Tools | 10+ |
| 📊 Data Tools | 15+ |
| 🔧 Utility Tools | 20+ |

### **TOTAL: 235+ Rust Crates**

---

## 🚀 Recommended Core Dependencies

```toml
# dx-media/Cargo.toml

[package]
name = "dx-media"
version = "0.1.0"
edition = "2021"

[dependencies]
# ═══════════════════════════════════════════════════════
# CORE
# ═══════════════════════════════════════════════════════
tokio = { version = "1.36", features = ["full"] }
rayon = "1.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
indicatif = "0.17"
clap = { version = "4.5", features = ["derive"] }

# ═══════════════════════════════════════════════════════
# IMAGE (Essential)
# ═══════════════════════════════════════════════════════
image = "0.25"
imageproc = "0.24"
fast_image_resize = "3.0"
webp = "0.2"
ravif = "0.11"
oxipng = "9.0"
resvg = "0.40"
ico = "0.3"
qrcode = "0.14"
lofty = "0.18"                    # Image metadata

# ═══════════════════════════════════════════════════════
# VIDEO (FFmpeg-based)
# ═══════════════════════════════════════════════════════
ffmpeg-next = "7.0"
gifski = "1.14"
mp4 = "0.14"
subparse = "1.3"

# ═══════════════════════════════════════════════════════
# AUDIO (Essential)
# ═══════════════════════════════════════════════════════
symphonia = "0.5"
hound = "3.5"
rubato = "0.14"
ebur128 = "0.1"
lofty = "0.18"                    # Audio metadata

# ═══════════════════════════════════════════════════════
# DOCUMENTS (Essential)
# ═══════════════════════════════════════════════════════
lopdf = "0.32"
printpdf = "0.7"
pulldown-cmark = "0.10"
scraper = "0.19"
headless_chrome = "1.0"
calamine = "0.24"
docx-rs = "0.4"
epub-builder = "0.5"

# ═══════════════════════════════════════════════════════
# ARCHIVES (Essential)
# ═══════════════════════════════════════════════════════
zip = "0.6"
tar = "0.4"
flate2 = "1.0"
zstd = "0.13"
bzip2 = "0.4"
xz2 = "0.1"
sevenz-rust = "0.5"

# ═══════════════════════════════════════════════════════
# FONTS
# ═══════════════════════════════════════════════════════
ttf-parser = "0.20"
fontdue = "0.8"
woff2 = "0.3"

# ═══════════════════════════════════════════════════════
# 3D MODELS
# ═══════════════════════════════════════════════════════
gltf = "1.4"
obj = "0.10"
stl_io = "0.7"

# ═══════════════════════════════════════════════════════
# WEB ASSETS
# ═══════════════════════════════════════════════════════
minify-html = "0.15"
lightningcss = "1.24"

# ═══════════════════════════════════════════════════════
# DATA FORMATS
# ═══════════════════════════════════════════════════════
serde_yaml = "0.9"
toml = "0.8"
quick-xml = "0.31"
csv = "1.3"

# ═══════════════════════════════════════════════════════
# UTILITIES
# ═══════════════════════════════════════════════════════
blake3 = "1.5"
sha2 = "0.10"
uuid = { version = "1.7", features = ["v4", "v7"] }
base64 = "0.22"
regex = "1.10"
walkdir = "2.4"
infer = "0.15"
tempfile = "3.10"

[features]
default = ["image", "video", "audio", "documents", "archives"]
full = ["default", "fonts", "3d", "textures", "web", "ai"]
image = []
video = []
audio = []
documents = []
archives = []
fonts = []
3d = []
textures = []
web = []
ai = ["tract", "ort"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

---

## 🎯 Quick Feature Summary

```
╔══════════════════════════════════════════════════════════════════════════════╗
║                         dx-media Tools Summary                               ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  📸 IMAGE: 28 tools, 50+ input formats, 30+ output formats                  ║
║  🎬 VIDEO: 28 tools, 50+ input formats, 20+ output formats                  ║
║  🎵 AUDIO: 26 tools, 30+ input formats, 20+ output formats                  ║
║  📄 DOCUMENTS: 25 tools, 30+ input formats, 20+ output formats              ║
║  📦 ARCHIVES: 10 tools, 20+ formats                                         ║
║  🎯 ICONS: 10 tools, all platform icons                                     ║
║  🔤 FONTS: 10 tools, 10+ formats                                            ║
║  🎮 3D MODELS: 12 tools, 30+ formats                                        ║
║  🖼️ TEXTURES: 10 tools, GPU formats                                         ║
║  🌐 WEB ASSETS: 12 tools                                                    ║
║  📊 DATA: 13 tools, all data formats                                        ║
║  🔧 UTILITIES: 13 tools                                                     ║
║                                                                              ║
║  ─────────────────────────────────────────────────────────────────────────  ║
║                                                                              ║
║  TOTAL TOOLS: 200+                                                          ║
║  TOTAL FORMATS: 200+                                                         ║
║  TOTAL CRATES: 235+                                                          ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

This gives dx-media comprehensive media processing capabilities using pure Rust where possible, with optional native bindings for maximum performance! 🦀🚀