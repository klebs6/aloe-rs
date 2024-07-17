crate::ix!();

/**
  | Config: ALOE_USE_CAMERA
  | 
  | Enables camera support using the CameraDevice
  | class (Mac, Windows, iOS, Android).
  |
  */
#[cfg(not(ALOE_USE_CAMERA))]
pub const ALOE_USE_CAMERA: usize = 0;

#[cfg(not(ALOE_CAMERA_LOG_ENABLED))]
pub const ALOE_CAMERA_LOG_ENABLED: usize = 0;

#[cfg(ALOE_CAMERA_LOG_ENABLED)]
macro_rules! aloe_camera_log {
    ($x:ident) => {
        /*
                DBG(x)
        */
    }
}

#[cfg(not(ALOE_CAMERA_LOG_ENABLED))]
macro_rules! aloe_camera_log {
    ($x:ident) => {
        /*
                {}
        */
    }
}


