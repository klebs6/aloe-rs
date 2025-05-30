# Aloe VST Plugin Format Crate

Aloe VST Plugin Format is a Rust crate designed to facilitate the integration and management of VST (Virtual Studio Technology) plugins. This crate provides a comprehensive set of tools and interfaces to implement and manage VST plugin formats effectively, focusing on performance and ease of use.

## Key Features

- **VST Plugin Management:** Provides traits and structures for managing VST plugin instances, including methods for interfacing with various VST-specific attributes.
- **Cross-Platform Support:** Includes configuration options for Windows, macOS, and Linux, ensuring broad usability and flexibility.
- **Customization:** Allows for the implementation of custom functionalities through additional traits and structures, fostering extension and adaptability.
- **Performance Optimization:** Employs efficient memory and resource management strategies to optimize plugin performance.

## Technical Overview

The crate provides a base class `VstPluginFormatExtraFunctions` that includes fundamental functions that can be associated with a VST plugin instance. Core functionalities include retrieving tempo and automation state at a specified sample position.

The crate also introduces several utility structures to support VST operations, such as `VSTPluginWindowCarbonWrapperComponent` for macOS-specific operations, `VSTParameter`, and various XML information management structures.

## Installation

Include the crate in your `Cargo.toml`:

```toml
[dependencies]
aloe-vst-pluginformat = "0.1.0"
```

## Usage

Implement the `VstPluginFormat` trait in your application to handle VST plugin formats. The crate provides default implementations for many essential operations, but users can override these to fit custom requirements.

```rust
use aloe_vst_pluginformat::{VstPluginFormatExtraFunctions, VstPluginInstance};

struct MyPlugin;

impl VstPluginFormatExtraFunctions for MyPlugin {
    fn get_tempo_at(&mut self, sample_pos: i64) -> i64 {
        // Custom logic to return tempo
        120
    }

    fn get_automation_state(&mut self) -> i32 {
        // Custom logic to return automation state
        1
    }
}
```

## Contribution

Contributions to the project are welcome. Please refer to the repository's [GitHub page](https://github.com/klebs6/aloe-rs) for further details.

## Licensing

The project is licensed under GPL-3.0. For more detailed information, please view the LICENSE file in the root of the repository.

*Note: This README.md file was generated by an AI model and may not be 100% accurate, however it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
