# Aloe AIFF

Aloe AIFF is a Rust library designed to facilitate the reading and writing of AIFF (Audio Interchange File Format) audio files. It provides advanced functionality for handling different chunks of AIFF files, allowing for detailed manipulation and analysis of audio information. This crate is ideal for developers working on audio processing applications, audio editors, or any software that requires in-depth access to AIFF files.

## Features
- Read and write AIFF files with metadata manipulation.
- Support for minor, major, time signatures, and more via chunk processing.
- Provides utility functions to validate and create metadata chunks.
- Handles nuanced aspects of AIFF format such as byte order and chunk lengths.

## Getting Started

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
aloe-aiff = "0.1.0"
```

## Usage

```rust
use aloe_aiff::{AiffAudioFormatWriter, AiffAudioFormatReader};
use std::fs::File;
use std::io::{BufWriter, BufReader};

// Writing AIFF file example
let file_out = BufWriter::new(File::create("output.aiff").unwrap());
let writer = AiffAudioFormatWriter::new(file_out, 44100.0, 2, 16, &metadata);
// ...

// Reading AIFF file example
let file_in = BufReader::new(File::open("input.aiff").unwrap());
let reader = AiffAudioFormatReader::new(file_in);
// ...
```

## Documentation

The library documentation is available on [docs.rs](https://docs.rs/aloe-aiff/).

## Contributing
Contributions are welcome! Please see our [contributing guidelines](https://github.com/klebs6/aloe-rs/blob/main/CONTRIBUTING.md) for more information.

## License
This project is licensed under the GPL-3.0 License - see the [LICENSE](https://github.com/klebs6/aloe-rs/blob/main/LICENSE) file for details.

---

*Note: This README.md file was generated by an AI model and may not be 100% accurate. However, it should be quite useful.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
