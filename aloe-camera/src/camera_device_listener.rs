crate::ix!();

pub type CameraDeviceOpenCameraResultCallback = fn(_0: *mut CameraDevice, error: &String) -> ();

/**
  | Receives callbacks with individual
  | frames from a CameraDevice. It is mainly
  | useful for processing multiple frames
  | that has to be done as quickly as possible.
  | The callbacks can be called from any
  | thread.
  | 
  | If you just need to take one picture,
  | you should use takeStillPicture()
  | instead.
  | 
  | @see CameraDevice::addListener
  |
  */
pub trait CameraDeviceListener {

    /**
      | This method is called when a new image
      | arrives.
      | 
      | This may be called by any thread, so be
      | careful about thread-safety, and make
      | sure that you process the data as quickly
      | as possible to avoid glitching!
      | 
      | Simply add a listener to be continuously
      | notified about new frames becoming
      | available and remove the listener when
      | you no longer need new frames.
      | 
      | If you just need to take one picture,
      | use takeStillPicture() instead.
      | 
      | @see CameraDevice::takeStillPicture
      |
      */
    fn image_received(&mut self, image: &Image);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/native/aloe_android_CameraDevice.h]

impl CameraDevice {
    
    pub fn get_file_extension(&mut self) -> String {
        
        todo!();
        /*
            return ".mp4";
        */
    }
}
