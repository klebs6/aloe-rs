#![feature(allocator_api)]

#[cfg(target_os="ios")] #[macro_use] mod imports; 
#[cfg(target_os="ios")] use imports::*;

#[cfg(target_os="ios")] x!{camera_device}
#[cfg(target_os="ios")] x!{camera_device_impl}
#[cfg(target_os="ios")] x!{capture_session}
#[cfg(target_os="ios")] x!{file_output_recording_delegate_class}
#[cfg(target_os="ios")] x!{internal_open_camera_result}
#[cfg(target_os="ios")] x!{ios_version}
#[cfg(target_os="ios")] x!{photo_output_delegate_class}
#[cfg(target_os="ios")] x!{session_delegate_class}
#[cfg(target_os="ios")] x!{still_picture_taker}
#[cfg(target_os="ios")] x!{video_recorder}
#[cfg(target_os="ios")] x!{viewer_class}
#[cfg(target_os="ios")] x!{viewer_component}

pub struct Missing {}
pub type AVCaptureBracketedStillImageSettings = Missing;
pub type AVCaptureConnection                  = Missing;
pub type AVCaptureDeviceFormat                = Missing;
pub type AVCaptureDevice                      = Missing;
pub type AVCaptureFileOutputRecordingDelegate = Missing;
pub type AVCaptureFileOutput                  = Missing;
pub type AVCaptureMovieFileOutput             = Missing;
pub type AVCaptureOutput                      = Missing;
pub type AVCapturePhotoOutput                 = Missing;
pub type AVCapturePhoto                       = Missing;
pub type AVCaptureResolvedPhotoSettings       = Missing;
pub type AVCaptureSession                     = Missing;
pub type AVCaptureVideoOrientation            = Missing;
pub type AVCaptureVideoPreviewLayer           = Missing;
pub type AVFrameRateRange                     = Missing;
pub type CGImagePropertyOrientation           = Missing;
pub type CMSampleBufferRef                    = Missing;
pub type CMTime                               = Missing;
pub type CMVideoDimensions                    = Missing;
pub type DispatchQueue                        = Missing;
pub type Time                                 = Missing;
pub type UIImageOrientation                   = Missing;
pub type UIImage                              = Missing;
