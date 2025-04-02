# aloe-imagecomponent

`aloe-imagecomponent` is a Rust crate designed for developers requiring a simple yet robust image display component within graphical user interfaces. Leveraging Rust's type safety and performance, this crate provides a direct interface for setting and manipulating image display attributes, ensuring seamless integration and smooth rendering.

## Features
- **Image Display**: Easily display images using the provided `set_image` method.
- **Placement Control**: Customize image positioning within a component using `set_image_with_placement` and `set_image_placement` methods.
- **Aspect Ratio Maintenance**: Ensures images are displayed with correct aspect ratios, providing a default centered positioning.
- **Accessibility Support**: Create an accessibility handler with `create_accessibility_handler` to improve user interactions.

## Usage
Include this crate in your `Cargo.toml`:
```toml
[dependencies]
aloe-imagecomponent = "0.1.0"
```

### Example
```rust
use aloe::ImageComponent;
use aloe::RectanglePlacement;

// Initialize a new ImageComponent with a given name.
let mut image_component = ImageComponent::new("MyImageComponent");

// Set an image to display.
image_component.set_image(&my_image);

// Optionally, specify placement.
image_component.set_image_with_placement(&my_image, RectanglePlacement::centred);
```

## Contributing
Contributions via pull requests are welcome. Please adhere to our coding standards and include tests where applicable.

## License
`aloe-imagecomponent` is licensed under the GPL-3.0. See [LICENSE](https://github.com/klebs6/aloe-rs/blob/main/LICENSE) for more details.

## Acknowledgement
This README.md file was generated with the assistance of an AI model and might not be entirely accurate, yet it should provide extensive insights into the crate's functionalities.

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. As such, some of its features may be pending-translation. For progress updates, please see the workspacer rust project. workspacer contains a growing set of utilities designed specifically for rust projects.
