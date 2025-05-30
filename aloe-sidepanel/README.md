# Aloe SidePanel

Aloe SidePanel is a Rust library that provides a dynamic GUI component, designed to integrate seamlessly into mobile applications for navigation and form purposes.

## Features

- **Positioning**: Align the SidePanel to the left or right of a parent component.
- **Animation**: Use `show_or_hide` to animate the panel's visibility.
- **Responsive**: Reactive size adjustments and drag-based dismiss functionality.
- **Customization**: Set custom components for both the content and the title bar.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
aloe-sidepanel = "0.1.0"
```

Here's a simple example:

```rust
use aloe_sidepanel::SidePanel;

fn main() {
    let mut panel = SidePanel::new("Title", 300, true, std::ptr::null_mut(), None);
    panel.show_or_hide(true);
}
```

## Documentation

The `SidePanel` struct implements several interfaces, including:

- `ComponentListener`
- `ComponentEnablementChanged`
- `ComponentMovedOrResized`
- `ChangeListener`

### Methods

- `new()`: Instantiates a new `SidePanel`.
- `set_content()`: Define the main content component.
- `set_title_bar_component()`: Customize the title bar.
- `show_or_hide()`: Toggle the panel's visibility with an animation.

## License

This crate is licensed under the [GPL-3.0 license](https://opensource.org/licenses/GPL-3.0).

## Contribution

Contributions are welcome via [GitHub](https://github.com/klebs6/aloe-rs).

---

**Note**: This README.md file was generated by an AI model and may not be 100% accurate. However, it should provide a comprehensive overview of the crate.

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
