# aloe-text

`aloe-text` is a Rust library designed for text manipulation and conversion, with efficient algorithms for diff computation and other utilities. The crate offers the capability to transform one text into another by calculating the minimal sequence of edits. In addition, it includes functionalities for base-64 encoding/decoding and manages duplicate string storage for optimized memory usage.

## Features

- **TextDiff**: Calculate and apply sequences of changes to convert one string into another through insertions and deletions.
- **Base64**: Convert data to and from base-64 encoding, with robust support for stream writing.
- **String Pooling**: Efficiently manage and compare strings using shared storage to reduce redundancy.
- **String Region Manipulation**: Precisely handle and modify regions within strings, supporting complex operations with ease.

## Technical Overview

- **Text Transformation**: At its core, `aloe-text` provides a `TextDiff` structure that computes minimal edits required to transform an original string into a target string. Using a sequence of insertions and deletions stored internally, it achieves optimal transformations with minimal alterations.

- **Base64 Encoding**: The `Base64` structure supports conversion of binary data to base-64 strings and the inverse operation. This includes functionality to handle encoding and decoding within streams.

- **Efficient String Management**: The `StringPool` structure centralizes string handling, minimizing storage overhead by pooling identical strings together. This enables efficient equality checks via pointer comparison rather than full string comparison.

- **String Region Operations**: Facilitates detailed manipulation of substrings through structures like `StringRegion`, allowing operations to be conducted on discrete segments of strings for performance-focused processing.

## Usage

To include `aloe-text` in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
aloe-text = "0.1"
```

## Examples

### Text Diffing

```rust
use aloe_text::TextDiff;

let original = String::from("Hello, world!");
let target = String::from("Goodbye, world!");
let diff = TextDiff::new(&original, &target);
assert_eq!(diff.applied_to(original), target);
```

### Base64 Encoding

```rust
use aloe_text::Base64;

let data = b"hello world";
let base64_string = Base64::to_base64_from_raw(data.as_ptr() as *const _, data.len());
```

## License

This project is licensed under the GPL-3.0 License.

## Contributions

Contributions are welcome. Please follow standard GitHub protocols by forking the repository and submitting pull requests.

---

*This README.md file was generated by an AI model and may not be 100% accurate; however, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
