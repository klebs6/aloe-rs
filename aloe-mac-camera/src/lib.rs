#[cfg(target_os="macos")]
#[macro_use] mod imports; 

#[cfg(target_os="macos")]
use imports::*;

#[cfg(target_os="macos")] x!{camera_device_impl}
#[cfg(target_os="macos")] x!{delegate_class}
#[cfg(target_os="macos")] x!{post_catalina}
#[cfg(target_os="macos")] x!{post_catalina_photo_output_delegate}
#[cfg(target_os="macos")] x!{viewer_component}

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
