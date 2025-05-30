# Aloe VST PresetFile

Aloe VST PresetFile is a sophisticated Rust crate designed for handling VST 3 preset files through an implementation of various stream types, such as memory, file, and read-only subsections. This crate provides a comprehensive API to read, write, and manage VST 3 preset data, including component states, controller states, and program data, in a modular and efficient manner.

## Features

- **Chunk Management**: Seamlessly handle VST 3 file components using categorized chunks like Header, ComponentState, ControllerState, ProgramData, MetaInfo, and ChunkList.
- **Stream Implementations**: Utilize `BufferStream`, `ReadOnlyBStream`, and `FileStream` to interact with different data sources and sinks.
- **Stream Manipulation**: Read, write, seek, and tell operations are supported across the stream implementations, enabling robust manipulation of VST 3 data.
- **Preset Management**: Create, save, load, and manage VST 3 presets efficiently, ensuring compatibility and adherence to VST specifications.
- **Integration Capabilities**: Easily integrate with VST component interfaces for seamless data exchange and state management.

## Usage

### Basic Example

```rust
use aloe_vst_presetfile::{PresetFile, ChunkType};

fn main() {
    // Example to create and manipulate a PresetFile
    let file_stream = FileStream::open(b"preset.vstpreset", b"rb");
    let mut preset_file = PresetFile::new(file_stream);

    // Check and read a specific chunk
    if preset_file.contains(ChunkType::ComponentState) {
        // Perform operations
    }
}
```

### Stream Operations
```rust
impl IBStream for BufferStream {
   // Define method implementations for read, write, seek, tell
}

impl IBStream for ReadOnlyBStream {
   // Define method implementations for read, write, seek, tell
}

impl IBStream for FileStream {
   // Define method implementations for read, write, seek, tell
}
```

## Contributing

We welcome contributions! Please fork the repository, submit pull requests, and engage with the community.

## License

Licensed under the GPL-3.0 license.

## Maintainers

This crate is maintained by [klebs](mailto:tpk3.mx@gmail.com).

## Disclaimer
This README.md file was generated by an AI model and may not be 100% accurate, however, it should be pretty good.


This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
