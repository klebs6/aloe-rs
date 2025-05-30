# Aloe VST MIDI

## Overview

The `aloe-vst-midi` crate offers a sophisticated interface for handling MIDI events within VST (Virtual Studio Technology) plugins and hosts. This Rust library is tailored for developers creating audio applications that require precise MIDI control and customization.

## Core Features

- **MIDI Controller Management**: Provides an enumeration of MIDI controller numbers, allowing precise mapping and control of various MIDI parameters.
- **MIDI Learn Interface**: Incorporates the `IMidiLearn` trait for dynamic MIDI-CC parameter changes, facilitating real-time updates and host notifications.
- **VST MIDI Event Management**: Includes a `VSTMidiEventList` structure to handle VST MIDI events, offering utility methods for event addition, clearing, and memory management.

## Technical Details

- **ControllerNumbers Enum**: Enumerates all standard MIDI controllers, simplifying the process of handling MIDI CC messages.
- **IMidiLearn Trait**: Provides methods to react to live MIDI controller inputs, enabling the customization of MIDI parameters dynamically.
- **VSTMidiEventList Struct**: Manages a collection of MIDI events with facilities for automatic memory allocation and deallocation.

## Quick Start

To integrate `aloe-vst-midi` into your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aloe-vst-midi = "0.1.0"
```

Then you can use it as follows:

```rust
use aloe_vst_midi::{ControllerNumbers, IMidiLearn};

struct MyMIDIController;

impl IMidiLearn for MyMIDIController {
    fn on_live_midi_controller_input(&mut self, bus_index: i32, channel: i16, midicc: ControllerNumbers) -> tresult {
        // Custom implementation here
        Ok(())
    }
}
```

## License

This crate is licensed under the GPL-3.0 License.

## Contribution

Contributions are welcome! Please open an issue or submit a pull request. 

---

*This README.md file was generated by an AI model and may not be 100% accurate. However, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
