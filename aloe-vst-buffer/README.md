# Aloe VST Buffer Crate

Aloe VST Buffer provides a sophisticated mechanism for managing memory buffers, tailored specifically for applications involving variable string encodings and dynamic data handling, typical in VST plugin contexts.

## Overview

The crate introduces a `Buffer` type that supplies a robust API for memory management, allowing the addition, removal, and transformation of data seamlessly. It includes methods for converting between multibyte and wide string encodings, vital for handling locale-specific data.

The `Buffer` is a mutable object optimized for efficient memory use. It supports operations like appending, prepending, and copying while maintaining the ability to dynamically adjust its internal size. The buffer utilizes standard memory functions such as `malloc` and `free`, ensuring compatibility and performance.

### Key Features
- **Memory Management**: Efficiently allocates, reallocates, and frees memory.
- **String Encoding Conversions**: Provides utilities to convert between multibyte and wide encodings.
- **Familiar Interfaces**: Implements common traits like `AddAssign`, `Not`, `Drop`, and others to integrate naturally with idiomatic Rust patterns.

### Usage
To use `aloe-vst-buffer` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
aloe-vst-buffer = "0.1.0"
```

The `Buffer` can be used to manage data blocks dynamically. Below is a basic usage example:

```rust
use aloe_vst_buffer::Buffer;

let mut buffer = Buffer::from(1024); // Allocate a buffer of 1024 bytes
buffer.put_with_u8(65); // Append the byte 'A'
assert_eq!(buffer.get_fill_size(), 1);
```

Consult the module documentation for more detailed information on all available methods and traits.

### Safety and Guarantees
All operations within `Buffer` assume valid inputs but do not perform extensive checks on user-provided pointers for optimal performance. Ensure data consistency by managing memory pointers carefully.

### License
Licensed under GPL-3.0 - see the [LICENSE](https://github.com/klebs6/aloe-rs/blob/main/LICENSE) file for details.

***

*Note: This README file was generated by an AI model and may contain inaccuracies. Always refer to the official documentation for comprehensive and reliable information.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
