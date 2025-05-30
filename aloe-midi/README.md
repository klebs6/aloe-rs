# Aloe MIDI Crate

The `aloe-midi` crate provides comprehensive support for processing and handling MIDI (Musical Instrument Digital Interface) messages in real-time audio applications. This crate facilitates diverse MIDI data operations such as conversion between various MIDI protocol formats, handling sysex messages, and interfacing MIDI inputs and outputs. It is optimized for performance, making it suitable for use in high-performance audio processing environments.

## Key Features

- **Universal MIDI Packet Conversion:** Convert between different MIDI standards such as MIDI 1.0, MIDI 2.0, and bytestream formats. This feature is crucial for interoperability in multi-format environments.
- **Real-time MIDI Message Handling:** Process incoming and outgoing MIDI messages efficiently with support for various MIDI commands and controller interactions.
- **SysEx (System Exclusive) Message Support:** Parse and construct complex SysEx messages using predefined handlers.
- **MIDI State Management:** Manage the state of MIDI components, including note-on/note-off states, which are essential for real-time applications like virtual instruments and MIDI controllers.
- **Cross-Platform Compatibility:** Supports multiple operating systems with conditional compilation for platform-specific features (e.g., ALSA support on Linux).

## Usage

First, add `aloe-midi` to your `Cargo.toml` dependencies like so:

```toml
[dependencies]
aloe-midi = "0.1.0"
```

When using the crate, import the appropriate modules or functions for tasks like handling MIDI input/output and converting MIDI packet formats.

### Example

Here is a simple example demonstrating how to handle incoming MIDI messages:

```rust
use aloe_midi::{...

fn my_midi_handler(source: &mut MidiInput, message: &MidiMessage) {
    println!("Received MIDI message: {}", message.get_description());
}

let mut midi_input = MidiInput::open_device("default", Box::new(my_midi_handler)).expect("Failed to open MIDI input");
midi_input.start();
```

In this example, `my_midi_handler` is a callback function that gets invoked whenever a MIDI message is received.

## Advanced Topics

### Universal MIDI Packets (UMP)

Understanding UMP is pivotal for working with MIDI 2.0. UMPs allow extended resolution and richer data representation, which can be fully utilized with this crate.

### Platform-Specific Features

For Linux users, this crate provides advanced features using the ALSA API when compiled with the `alsa` feature flag enabled.

## Contributing

Contributions are welcome! Please check the repository's GitHub page for contribution guidelines and open issues.

## License

This crate is licensed under the GPL-3.0 license.

---

*Note: This README.md file was generated by an AI model and may contain approximations or inaccuracies. Please refer to the crate documentation and source code for precise information.*


This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
