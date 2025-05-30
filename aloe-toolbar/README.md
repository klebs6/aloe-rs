# Aloe Toolbar

Aloe Toolbar is a Rust crate that provides a modular and customizable toolbar component designed for GUI applications. Leveraging various traits and structs, it facilitates the creation, customization, and interaction with toolbar components, allowing for a dynamic and user-friendly interface.

## Features
- **Editing Modes**: Tailor the behavior of the toolbar items through different editing modes such as `normalMode`, `editableOnToolbar`, and `editableOnPalette`.
- **Item Styles**: Choose how toolbar items are displayed with options for `iconsOnly`, `iconsWithText`, and `textOnly` styles.
- **Customizability**: Include a broad array of customization flags allowing users to reset to default configurations or make icons and text adjustments.

## Usage
The crate revolves around some core concepts:

### Enums
- `ToolbarEditingMode`: Defines modes to control editing behavior.
- `ToolbarItemFactorySpecialItemIds`: Manages special reserved item IDs.
- `ToolbarItemStyle`: Specifies styles for items' display.

### Traits
- `ToolbarItemComponentInterface`: Represents the basic interface for toolbar items.
- `PaintButtonArea`, `ContentAreaChanged`, `SetStyle`: Define essential behaviors and custom painting logic for toolbar items.

### Core Structures
- **Toolbar**: Represents a customizable strip of toolbar items.
- **ToolbarButton**: A button designed to fit within the toolbar, supporting normal and toggle states with different icons.
- **ToolbarItemFactory**: An abstract factory capable of creating defined item types, ensuring flexibility and extensibility.

## Customization
To enable toolbar customization, the `ToolbarCustomisationDialog` can be invoked, allowing users to adjust item layouts dynamically:
```rust
let factory: &mut dyn ToolbarItemFactory;  // Your factory instance
let toolbar: &mut Toolbar;  // Your toolbar instance
toolbar.show_customisation_dialog(factory, Some(ToolbarCustomisationFlags::AllCustomisationOptionsEnabled));
```

## Author
- klebs <tpk3.mx@gmail.com>

## License
This crate is licensed under the GPL-3.0 License.

## Repository
You can find the source code [here](https://github.com/klebs6/aloe-rs).

---
*This README.md was generated by an AI model and may not be 100% accurate. However, it should provide a strong overview of the crate's capabilities.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
