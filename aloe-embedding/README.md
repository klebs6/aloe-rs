# Aloe Embedding

## Overview

Welcome to the `aloe-embedding` crate, designed to facilitate the embedding of platform-specific native views into applications. This crate offers a seamless integration layer for implementing platform-specific UI containers on iOS, macOS, and Windows systems, crucial for developers dealing with heterogeneous GUI environments.

- **iOS/UIViewComponent**: Embed and manage `UIView` instances.
- **macOS/NSViewComponent**: Handle embedding of `NSView` objects.
- **Windows/HWNDComponent**: Integrate `HWND` elements.
- **Windows/ActiveXControlComponent**: Manage ActiveX controls.

## Usage

To use these components, instantiate a specific component for the target platform, insert it into your application layout, and utilize the helper methods like `set_view`, `get_view`, and `resize_to_fit_view` to manage lifecycle and layout dynamics. 

### Example - UIViewComponent:
```rust
let mut view_container = UIViewComponent::default();
view_container.set_view(your_ui_view);
view_container.resize_to_fit_view();
```

### Platform-specific Components:
- **UIViewComponent**: For iOS applications, manage `UIView` embedding with `set_view()`.
- **NSViewComponent**: On macOS, `NSView` can be embedded and manipulated using similar methods.
- **HWNDComponent and ActiveXControlComponent**: Windows-specific control methods such as `sethwnd()` apply.

## Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
aloe-embedding = "0.1.0"
```

## License

This project is licensed under the GPL-3.0 license.

## Contributing

For contributions, visit the [GitHub repository](https://github.com/klebs6/aloe-rs) and follow the contribution guidelines.

---

*This README.md was generated by an AI model and its accuracy may not be 100% perfect, though it strives for helpfulness.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
