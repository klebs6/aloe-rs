crate::ix!();

pub type InternalOpenCameraResultCallback = fn(camera_id: &str, error: &str) -> ();
