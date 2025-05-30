# Aloe Audio Devices

Aloe Audio Devices is a Rust crate providing high-level abstractions for managing and configuring audio devices. Suitable for complex audio applications, this crate allows developers to easily manipulate audio inputs and outputs, configure settings like sample rates and buffer sizes, and manage MIDI inputs and outputs. It's designed for seamless integration with the AudioDeviceManager, facilitating comprehensive audio device management within your applications.

## Features

- Enumeration of audio input and output device types.
- UI components for device selection and configuration, including dropdowns and buttons.
- Support for checking and modifying channel configurations using channel selectors.
- Audio settings manipulation including sample rate, buffer size, and stereo channel configurations.
- MIDI device selection capabilities with Bluetooth MIDI integration.
- Built-in test utilities for audio configurations.

### Structs and Components

- **AudioDeviceSettingsPanel**: Provides UI elements for configuring audio settings.
- **AudioDeviceSelectorComponent**: A component for selecting the audio device and managing related configurations.
- **SystemAudioVolume**: Abstraction for managing system audio volume with gain and mute functionalities.
- **ChannelSelectorListBox**: Displays available audio channels for selection.
- **MidiInputSelectorComponentListBox**: Displays MIDI input devices and allows toggling of their states.

### Example Usage

```rust
fn main() {
    // Initialize your audio device manager
    let mut device_manager = AudioDeviceManager::new();
    let min_input_channels = 0;
    let max_input_channels = 2;
    let min_output_channels = 2;
    let max_output_channels = 2;

    // Create the audio device selector component
    let mut audio_device_selector = AudioDeviceSelectorComponent::new(
        &mut device_manager,
        min_input_channels,
        max_input_channels,
        min_output_channels,
        max_output_channels,
        true,
        true,
        true,
        false,
    );

    // Show UI component to users
    audio_device_selector.set_item_height(30);
    audio_device_selector.resized();

    // More operations...
}
```

### Notes

- Designed with leak detection and efficient resource management in mind.
- Provides robust error handling and debugging utilities.

## License

This crate is licensed under the GPL-3.0 License. See [LICENSE](LICENSE) for more details.

---

*This README file was generated by an AI model and may not be 100% accurate, however, it should be pretty good.*


This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
