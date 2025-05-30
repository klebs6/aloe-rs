# aloe-thread-with-progress-window

## Overview

`aloe-thread-with-progress-window` is a Rust crate designed for multithreading applications that require user feedback through a modal dialog with a progress bar and a cancel button. This enables developers to run concurrent tasks while keeping the user informed about progress.

## Usage

This crate provides the `ThreadWithProgressWindow` structure and relevant traits to manage threading operations with a visual interface. It is particularly useful for long-running tasks that demand user interactivity to allow cancellations or simple progress updates.

### Key Features
- **Progress Monitoring:** The modal window provides real-time progress updates through a visual progress bar.
- **Cancellable Operations:** A cancel button allows users to stop tasks if necessary, handling cancel events seamlessly.
- **User Feedback:** Modify status messages dynamically to keep users informed about ongoing tasks.

### Example

```rust
class MyTask : public ThreadWithProgressWindow {
    MyTask() : ThreadWithProgressWindow("busy...", true, true) {}
    
    void run() {
        for (int i = 0; i < thingsToDo; ++i) {
            if (threadShouldExit())
                break;
            setProgress(i / (double)thingsToDo);
            // Perform task...
        }
    }
};

void doTheTask() {
    MyTask m;
    if (m.runThread()) {
        // Thread finished normally
    } else {
        // User pressed the cancel button
    }
}
```

## Technical Details
- **Thread Management:** Built atop Rust threading primitives, ensuring robust and efficient thread handling.
- **Progress Window:** Utilizes a modal alert window to provide a non-intrusive user interface.
- **ThreadComplete Trait:** Notifies when a thread completes execution.

## License

Licensed under the GPL-3.0.

---

*This README.md file was generated by an AI model and may not be 100% accurate, but it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
