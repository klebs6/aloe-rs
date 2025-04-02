# aloe-selection

`aloe-selection` is a Rust library designed to manage a collection of selectable items. It is particularly beneficial for applications that involve user interaction with graphical user interfaces, facilitating the tracking and management of selected items such as icons or entries in a list.

## Overview

The library offers a flexible and highly extensible interface for managing selections. It abstracts the concept of a selectable item so that it can handle either references to objects or identifiers such as ID numbers or handles. This allows for versatile integration in various systems, including GUI-based applications where selections can change dynamically.

### Key Concepts:

- **SelectableItemType**: A trait defining the type of item that can be selected.
- **ChangeListener**: Notifies changes whenever an item is selected or deselected.
- **ModifierKeys**: Handles complex selection logic involving user input modifiers like Shift or Control keys.

## Usage

Import the crate and create an instance of `SelectedItemSet`, parameterized with your desired item type. You can then add, remove, and manage selected items efficiently.

```rust
use aloe_selection::SelectedItemSet;

let mut selection_set = SelectedItemSet::new_from_items(&vec![1, 2, 3]);
selection_set.add_to_selection(4);
assert!(selection_set.is_selected(4));
```

The library provides robust capabilities for maintaining selection states and notifying any registered listeners upon changes. It is optimized for performance and integrates seamlessly with existing Rust GUIs.

## Features

- **Dynamic Selection**: Add or remove items either individually or in bulk.
- **Change Notification**: Automatically broadcasts changes to all registered listeners.
- **Modifier Key Intuition**: Integrates with keyboard modifiers for easy batch selection operations.

## Author

Developed by klebs <tpk3.mx@gmail.com> and licensed under GPL-3.0.

## Contribute

Visit our [GitHub repository](https://github.com/klebs6/aloe-rs) to contribute, file issues, or seek further enhancement discussions.

---

*This README is AI-generated and should be verified for accuracy before use.*