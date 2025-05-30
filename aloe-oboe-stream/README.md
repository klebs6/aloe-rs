# Aloe Oboe Stream

Aloe Oboe Stream is a Rust crate designed for advanced audio processing using the Oboe API, encapsulating high-performance audio interfaces tailored for professional audio applications on Android platforms. Offering traits and structs for managing audio streams, callbacks, and error handling, Aloe Oboe Stream provides a robust framework for real-time audio data manipulation and streaming.

## Features
- **AudioStreamInterface**: Manage the lifecycle of audio streams with functions such as `request_start`, `request_pause`, `request_flush`, and `request_stop`. Includes capabilities for state monitoring and frame updates.
- **Callback Interfaces**: Implement `AudioStreamDataCallback` for handling audio data transfer and `AudioStreamErrorCallback` for error management, ensuring resilient application behavior under varied conditions.
- **Stream Configuration**: Utilize `AudioStreamBuilder` to configure essential audio stream parameters, including channel count, sample rate, format, and buffer dimensions.
- **Advanced Stream Management**: Leverage `AudioStreamAAudio` for streams utilizing AAudio API, providing access to low-level stream management and error handling.

## Example
Here is a basic example demonstrating how to use the crate:
```rust
use aloe_oboe_stream::{AudioStreamBuilder, AudioStreamInterface};

fn main() {
    let mut builder = AudioStreamBuilder::default();
    builder.set_sample_rate(48000)
        .set_channel_count(2)
        .set_format(oboe_oboe::AudioFormat::Float);

    let mut audio_stream = builder.build().expect("Failed to build audio stream");
    audio_stream.request_start().expect("Failed to start audio stream");
    
    // Do some audio processing...

    audio_stream.request_stop().expect("Failed to stop audio stream");
}
```

## Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
aloe-oboe-stream = "0.1.0"
```

## License
This crate is licensed under the GPL-3.0 license.

---
*This README.md was generated by an AI model and may not be 100% accurate; however, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project.
