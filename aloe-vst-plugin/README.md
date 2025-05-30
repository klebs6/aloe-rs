# Aloe VST Plugin

Aloe VST Plugin is a meticulously designed Rust crate tailored for the creation and management of audio plugin instances conforming to the VST3 standard. It serves as an essential tool for developers aiming to integrate or develop audio processing applications with efficient plugin architectures.

## Overview

The `aloe-vst-plugin` crate provides comprehensive definitions and interfaces necessary for developing extensible and robust audio plugins based on the VST3 specification.

### Core Components:

- **`IPluginBase`**: Core component interface initializing and terminating plugin components. Establishes necessary communication with the host application.
  
- **`IPluginFactory`**: Facilitates the creation of plugin instances and provides class information; a crucial component in plugin instance management and interaction.
  
- **`IPluginFactory3` & `IPluginFactory2`**: Extends functionality with support for unicode class information and enhanced class management.

- **`IPlugInterfaceSupport`**: Query mechanism to evaluate host-supported interfaces, ensuring seamless integration and compatibility checks.

## Getting Started

Implementers can extend the `IPluginFactory` trait to create unique plugin instances. The interface offers methods such as `create_instance`, `count_classes`, and `get_class_info` to manage plugin lifecycle and metadata.

### Example Usage:

```rust
fn initialize_plugin() -> Result<(), &'static str> {
    let factory: *mut dyn IPluginFactory = get_plugin_factory();
    if factory.is_null() {
        return Err("Failed to initialize plugin factory.");
    }
    Ok(())
}
```

## License

This project is licensed under the GPL-3.0 License. See the LICENSE file for more details.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Disclaimer

This README.md file was generated by an AI model and may not be 100% accurate; however, it should be pretty good.

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
