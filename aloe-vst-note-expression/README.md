# Aloe VST Note Expression

**Aloe VST Note Expression** is a Rust crate that provides advanced tools for VST (Virtual Studio Technology) plug-in development with a focus on note expression manipulation. It is designed to facilitate the handling of note expression events, key switches, and physical UI mappings between expression types and hardware controls.

## Features

- **Physical UI Type Mapping**: Define mappings between note expressions and physical UI forms using `PhysicalUITypeIDs` and `PhysicalUIMap` structures.
- **Note Expression Handling**: Utilize `NoteExpressionTypeIDs`, `NoteExpressionTypeFlags`, and related traits to manage complex note expressions including volume, pan, tuning, and custom types.
- **Keyswitch Control**: Implement the `IKeyswitchController` trait for efficient key switch management in VST instruments.
- **Interface Extensions**: Leverage `INoteExpressionPhysicalUIMapping` and `INoteExpressionController` traits to extend plug-in capabilities, allowing for dynamic control over note expressions and their physical mappings.

## Example

Here's a basic example demonstrating how to implement the `INoteExpressionPhysicalUIMapping` trait:

```rust
impl INoteExpressionPhysicalUIMapping for MyController {
    fn get_physical_ui_mapping(&mut self, bus_index: i32, channel: i16, list: &mut PhysicalUIMapList) -> tresult {
        if bus_index == 0 && channel == 0 {
            for i in 0..list.count {
                let map_entry = &mut list.map[i as usize];
                map_entry.note_expression_typeid = match map_entry.physical_ui_typeid {
                    PhysicalUITypeIDs::PUIXMovement => NoteExpressionTypeIDs::CustomStart + 1,
                    PhysicalUITypeIDs::PUIYMovement => NoteExpressionTypeIDs::CustomStart + 2,
                    _ => NoteExpressionTypeIDs::InvalidTypeID,
                };
            }
            kResultTrue
        } else {
            kResultFalse
        }
    }
}
```

## Getting Started

To include this crate in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aloe-vst-note-expression = "0.1.0"
```

## License

This project is licensed under the GPL-3.0 License. See the [LICENSE](https://github.com/klebs6/aloe-rs/blob/main/LICENSE) file for details.

---
Note: This README file was generated using an AI model and may not be 100% accurate, but it should be pretty good.

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
