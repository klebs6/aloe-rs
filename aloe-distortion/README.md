# Aloe Distortion

Aloe Distortion is a Rust audio processing library designed to simulate an overdrive effect. It provides audio developers with efficient and customizable components for creating audio distortion and enhancing sound dynamics using oversampling and wave shaping. Designed with flexibility, this crate is compatible with a variety of audio processing tasks, bolstered by its components that facilitate the creation of realistic overdrive sound signatures.

## Features

- **Modular Components:** Leverage modular components like `OverdriveGainProcessor`, `OverdriveBiasProcessor`, and `OverdriveDriveProcessor` for nuanced overdrive effects.
- **Processor Chains:** Effortlessly build audio chains with `ProcessorChain`, equipped for complex audio transformations.
- **Comprehensive Oversampling:** Utilize six levels of oversampling for high-fidelity audio processing.
- **Wave Shaping & Clipping:** Integrate different wave shaping methods to model distortion with precision and safely handle signal clipping.

## Usage

Integrate Aloe Distortion in your project by adding it to your `Cargo.toml`:

```toml
[dependencies]
aloe-distortion = "0.1.0"
```

## Example

```rust
use aloe_distortion::{DistortionProcessor};

fn main() {
    let mut processor = DistortionProcessor::default();
    // Configure processor specifics, then prepare context and initiate processing
}
```

## Documentation

For detailed instructions and advanced usage, refer to the [documentation](https://docs.rs/aloe-distortion).

## Contributions

Contributions are welcome! Please see our [Contributing Guide](https://github.com/klebs6/aloe-rs/blob/master/CONTRIBUTING.md) for details.

## License

This project is licensed under the terms of the GPL-3.0 license.

_Disclaimer: This README.md file was generated by an AI model and may not be 100% accurate, however it should be pretty good._

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
