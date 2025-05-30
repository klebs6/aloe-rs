# Aloe TextEditor

Aloe TextEditor is a versatile Rust library designed to facilitate the creation and manipulation of advanced text editing components. This crate provides a comprehensive suite of functionalities, enabling developers to implement rich text editors with ease.

## Features

- **Editable Text Box**: Supports single or multiline modes, accommodating mixed fonts and colors, with a variety of configurable options including word wrap and scroll bars.
- **Text Manipulation API**: Includes traits and structures for text input filtering, text editing actions (insertions, deletions), and drag operations.
- **Input Filtering**: Ability to limit input based on character restrictions or length through custom filters.
- **Accessibility Support**: Built-in handlers ensure text editors are accessible, complying with various user interface guidelines.
- **Undo/Redo System**: Integrates an undo manager allowing for sophisticated text state management.
- **Popup Menu Integration**: Supports context-sensitive menus for text operations like cut, copy, paste, and more.
- **Event Callbacks**: Interfaces for handling text changes, keyboard events, and focus events, allowing dynamic response to user interactions.

## Usage

To get started with Aloe TextEditor, add it to your `Cargo.toml`:

```toml
[dependencies]
aloe-texteditor = "0.1.0"
```

The library includes the primary `TextEditor` struct and several traits/interfaces to extend its functionalities:

- `TextEditorInterface`: Core interface for text editing functionality.
- `TextEditorListener`: Callback interface for text change events.
- `TextEditorInputFilter`: Base class for input filters.

### Example

```rust
use aloe_texteditor::{TextEditor, TextEditorListener};

struct MyListener;

impl TextEditorListener for MyListener {
    fn text_editor_text_changed(&mut self, editor: &mut TextEditor) {
        println!("Text changed: {}", editor.get_text());
    }
}

fn main() {
    let mut editor = TextEditor::new(None, None);
    editor.set_text("Hello, Aloe!", true); // Set initial text
    editor.add_listener(Box::new(MyListener));
}
```

## Community and Contributions

A warm welcome to the community! Contributions are highly appreciated. Feel free to fork the project on [GitHub](https://github.com/klebs6/aloe-rs) and submit pull requests.

---

*Note: This README.md file was generated by an AI model and may not be 100% accurate, but it aims to provide a comprehensive overview of the Aloe TextEditor crate.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
