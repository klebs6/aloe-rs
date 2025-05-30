# Aloe Tabbed

Aloe Tabbed is a Rust crate providing a sophisticated and adaptable tabbed component for building graphical user interfaces (GUIs). It allows developers to create multitab interfaces efficiently by integrating a `TabbedComponent` and `TabbedButtonBar` that automatically manage the layout and behavior of tab-based navigation systems.

## Features

- **TabbedButtonBarOrientation:** Define the position of your tab bar with options such as `TabsAtTop`, `TabsAtBottom`, `TabsAtLeft`, and `TabsAtRight`.
- **TabbedComponentColourIds:** Customize with color IDs, enabling personalized backgrounds and outlines.
- **UI Customization:** Add additional components within tabs, customizing their placement relative to the tab text.
- **Callback and Integration Support:** Implement traits such as `PopupMenuClickOnTab`, `CreateTabButton`, `GetBestTabLength`, and `CurrentTabChanged` to integrate interactive behaviors.

## Usage

Configure your GUI by creating a `TabbedComponent` and populating it with tabs via `addTab(...)`. Customize tab appearance and behavior with several configuration methods:

```rust
let mut tabbed_component = TabbedComponent::new(TabsAtTop);
tabbed_component.add_tab("Tab 1", Colour::from_rgb(200, 200, 255), my_component, true, None);
```

The layout and behavior of tab bars can be further adjusted using methods like `setOrientation`, `setMinimumTabScaleFactor`, and `setOutline`.

## License

Licensed under GPL-3.0.

## Disclaimer

This README file was generated by an AI model and may not be 100% accurate, however, it should provide a reliable overview of the crate functionality and usage scenarios.

## Repository

Find the source and contribute at: [GitHub](https://github.com/klebs6/aloe-rs)


This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
