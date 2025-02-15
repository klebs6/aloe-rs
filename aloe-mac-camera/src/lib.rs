#[macro_use] mod imports; use imports::*;

x!{camera_device_impl}
x!{delegate_class}
x!{post_catalina}
x!{post_catalina_photo_output_delegate}
x!{viewer_component}

pub struct Missing;

// these are somewhere, not here
pub type AVCaptureConnection                  = Missing;
pub type AVCaptureDeviceInput                 = Missing;
pub type AVCaptureFileOutputRecordingDelegate = Missing;
pub type AVCaptureFileOutput                  = Missing;
pub type AVCaptureMovieFileOutput             = Missing;
pub type AVCapturePhotoOutput                 = Missing;
pub type AVCapturePhoto                       = Missing;
pub type AVCaptureSession                     = Missing;
