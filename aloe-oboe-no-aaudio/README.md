# aloe-oboe-no-aaudio

A comprehensive Rust crate designed to seamlessly bridge the gap between the AAudio interface and Rust applications. It provides type definitions directly mapped from AAudio's C header files for efficient audio stream management on Android.

## Overview
The crate includes essential type definitions required for constructing and manipulating audio streams using the AAudio API on Android platforms. It's an indispensable tool for developers dealing with audio stream configurations, data callbacks, and error handling within the AAudio context.

## Usage
To leverage the AAudio API in your Rust application and manage intricate details of audio streaming, do the following:

```rust
use aloe_oboe_no_aaudio::*;

// Define your AAudio stream callback
fn my_data_callback(
    stream: *mut AAudioStream, 
    user_data: *mut c_void, 
    audio_data: *mut c_void, 
    num_frames: i32
) -> AAudioDataCallbackResult {
    // Your audio data processing logic
}

// Handle stream errors
fn my_error_callback(
    stream: *mut AAudioStream, 
    user_data: *mut c_void, 
    error: AAudioResult
) {
    // Your error handling logic
}
```

## Features
- **Comprehensive Type Definitions**: Includes all major AAudio types like `AAudioStream`, `AAudioStreamBuilder`, and `AAudioStreamState`.
- **Callback Management**: Simplifies the integration of custom data and error callbacks for audio streams.
- **Cross-Language Compatibility**: Ensures smooth operation with native Android AAudio features.

## Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
aloe-oboe-no-aaudio = "0.1.0"
```

## Author
Created and maintained by **klebs**. Contributions and feedback are welcome.

---

*Disclaimer: This README was generated by an AI model and might not fully capture the nuance of the crate's purpose or design. However, it aims to provide an accurate and insightful overview.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
