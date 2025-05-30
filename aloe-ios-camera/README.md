# Aloe iOS Camera

Aloe iOS Camera offers a comprehensive interface for managing camera functions on iOS devices, leveraging the AVFoundation framework. This crate enables efficient control over various camera settings, capturing still and video imagery, and handling orientation and device changes.

## Features

- **AVFoundation Integration**: Provides a robust abstraction over iOS's native camera handling capabilities.
- **Capture Sessions**: Manage multiple camera sessions for video recording and still photography with dynamic session configurations.
- **Orientation and Device Handling**: Facilitates the adjustment of video orientations based on device orientation changes.
- **High-Level Image Processing**: Convenience methods for handling image orientations and sizes natively with CoreGraphics integrations.
- **Listener Architecture**: Extensible listener setup for handling events such as image capture and session errors.

## Usage

```rust
use aloe_ios_camera::{AVCaptureConnection, CameraDeviceImpl, CaptureSession};

fn main() {
    // Initial setup requiring access permissions
    let camera_id = "0".to_string();
    let mut camera_device = CameraDeviceImpl::new(&mut CameraDevice {}, &camera_id, 0, 640, 480, 1280, 720, true);
    camera_device.open(|camera_id, err| {
        if err.is_empty() {
            println!("Camera {} opened successfully.", camera_id);
        } else {
            eprintln!("Failed to open camera {}: {}", camera_id, err);
        }
    });
}
```

## License

GPL-3.0

## Repository

[GitHub - klebs6/aloe-rs](https://github.com/klebs6/aloe-rs)

---

*This README.md was generated by an AI model and may not be 100% accurate, however, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
