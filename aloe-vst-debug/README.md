# aloe-vst-debug

`aloe-vst-debug` is a Rust crate envisioned to provide versatile debugging functions in a VST (Virtual Studio Technology) plugin development environment. It offers developers tools to inject custom assertion handlers, redirect debug output, and query debugger attachment statuses, specifically tailored for cross-platform use across Windows, Linux, and macOS systems.

## Features

- **Custom Assertion and Debug Print Handlers**: Define bespoke handlers for assertions and debug print outputs, allowing integration with custom dialogs or output redirection to a preferred stream or file.
- **Debugger Detection**: Functions like `am_ibeing_debugged` return true if a debugger is attached, utilizing platform-specific mechanisms for detection.
- **Robust Debug Print Functionality**: The `debug_print` functions cater to formatted string outputs alongside argument lists, supporting diverse debugging scenarios.
- **Error Reporting and Debug Breaking**: Implementations for `debug_break` and `print_last_error` enable precise error tracing and controlled debug interruptions across development stages.
- **Memory Allocation Hooks**: On macOS, override operators for new, new array, delete, and delete array, offering traceable memory allocation and deallocation processes in a debug environment.

## Compilation Conditionals

- `#[cfg(DEVELOPMENT)]` gates many of the functions to ensure they are compiled only in the development build mode.
- Specific platform configurations are abstracted using `#[cfg(SMTG_OS_WINDOWS)]`, `#[cfg(SMTG_OS_LINUX)]`, and `#[cfg(SMTG_OS_MACOS)]` to deliver seamless cross-platform support.

## Usage

In your Rust project's Cargo.toml, under [dependencies], include:  

```toml
aloe-vst-debug = "0.1.0"
```

Integrate the functions within your project to harness structured debugging and diagnostics utilities that align with professional VST plugin development needs.

---

*This README.md was generated by an AI model and may not be 100% accurate, however it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
