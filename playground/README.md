# DX-Media Playground

This folder contains real media assets for testing all 60 dx-media tools interactively.

## Directory Structure

```
playground/
├── assets/          # Downloaded media files for testing
│   ├── images/      # Test images (flower.jpg, landscape.jpg, sample.jpg, etc.)
│   ├── audio/       # Test audio files (piano.mp3, calm_piano.mp3)
│   ├── videos/      # Test video files (sample.mp4)
│   ├── documents/   # Test documents (test.md, test.html, test.txt)
│   ├── providers/   # Assets organized by provider
│   │   ├── openverse/
│   │   ├── wikimedia/
│   │   ├── nasa/
│   │   ├── met/
│   │   └── ... (13 provider folders)
│   └── scraping/    # Scraping test outputs
├── output/          # Output files from tool tests
│   ├── image/       # Image processing outputs
│   ├── video/       # Video processing outputs
│   ├── audio/       # Audio processing outputs
│   ├── document/    # Document processing outputs
│   ├── archive/     # Archive operation outputs
│   ├── utility/     # Utility tool outputs
│   └── collections/ # Tool collection test outputs
└── tools/           # Tool test outputs by category
    ├── image/
    ├── video/
    ├── audio/
    ├── document/
    ├── archive/
    └── utility/
```

## Test Assets

### Images
- `flower.jpg` - Flower photo (93KB)
- `white_flower.jpg` - White flower photo (55KB)
- `landscape.jpg` - Landscape photo (267KB)
- `sample.jpg` - Sample image (110KB)
- `wikimedia-2112596.jpg` - Downloaded from Wikimedia (1.6MB)

### Audio
- `piano.mp3` - Short piano sample (247KB)
- `calm_piano.mp3` - Longer piano piece (3.3MB)

### Video
- `sample.mp4` - Sample video (574KB)

### Documents
- `test.md` - Markdown test document
- `test.html` - HTML test document
- `test.txt` - Plain text test document

### Provider Downloads
- `providers/openverse/openverse-*.jpg` - Downloaded from Openverse
- `providers/wikimedia/wikimedia-*.jpg` - Downloaded from Wikimedia

## Running Tests

All 60 tools are tested using real assets. Run the integration tests with:

```bash
# Run all tests (593 tests)
cargo test

# Run tool integration tests
cargo test --test test_all_tools

# Run playground integration tests (downloads from providers)
cargo test --test playground_integration

# Run with output
cargo test --test playground_integration -- --nocapture
```

## CLI Usage Examples

Search for media:
```bash
./target/release/dx search "nature" --providers openverse -n 5
```

Download media:
```bash
./target/release/dx search "flower" --providers wikimedia -n 1 --download -o playground/assets/images/
```

List available providers:
```bash
./target/release/dx providers
```

## Test Results

All 593 tests pass:
- 158 library unit tests
- 35 archive tests
- 49 audio tests
- 44 document tests
- 43 image tests
- 72 integration tests (real assets)
- 48 utility tests
- 55 video tests
- 72 doc tests

