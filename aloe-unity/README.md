# Aloe-Unity Crate

Aloe-Unity is a Rust crate providing seamless integration with Unity's audio processing environment. It offers a comprehensive suite of abstractions for handling Unity audio effects, parameter definitions, and event modifiers, thus facilitating the development of sophisticated audio plugins within the Unity ecosystem. This crate ensures efficient audio processing by offering direct bindings to Unity's audio plugin API.

## Features
- **Audio Effects Management**: Utilize rich enumerations and structures to control Unity audio effects, including side chain targeting and spatialization.
- **State Monitoring**: Detailed tracking of audio effect states such as playing, paused, and muted.
- **Parameter Handling**: Define, set, and retrieve audio parameter values and buffers for dynamic audio manipulation.
- **Custom Callbacks**: Employ a range of customizable callbacks for creating, releasing, processing, and resetting audio effect states.
- **3D Audio Data Structures**: Robust structures for managing spatial and ambisonic audio data, facilitating immersive audio experiences.

## Usage
Import the crate and leverage the provided enums and structs to integrate Unity's audio functionalities. Specific callback definitions allow custom handling of audio processing tasks.

## Examples
```rust
#[cfg(target_feature = "unity")]
pub fn create_unity_peer(c: &mut Component) -> *mut ComponentPeer {
    // Implementation here...
}
```

For more comprehensive examples and applications, refer to the [repository](https://github.com/klebs6/aloe-rs).

## Contributing
Contributions are welcome! Please adhere to the guidelines outlined in the repository for submitting issues or pull requests.

---

*Note: This README was generated by an AI model and may not be 100% accurate; however, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. 
