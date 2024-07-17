crate::ix!();

//--------------------------------------------------[ComponentPeer]

/**
  | Used to receive callbacks when the OS
  | scale factor of this ComponentPeer
  | changes.
  | 
  | This is used internally by some native
  | Aloe windows on Windows and Linux and
  | you shouldn't need to worry about it
  | in your own code unless you are dealing
  | directly with native windows.
  |
  */
pub trait ComponentPeerScaleFactorListener {

    /**
      | Called when the scale factor changes.
      |
      */
    fn native_scale_factor_changed(&mut self, new_scale_factor: f64);
}

pub trait GetPlatformScaleFactor {

    /**
      | On Windows and Linux this will return
      | the OS scaling factor currently being
      | applied to the native window. This is
      | used to convert between physical and
      | logical pixels at the OS API level and
      | you shouldn't need to use it in your own
      | code unless you are dealing directly
      | with the native window.
      |
      */
    fn get_platform_scale_factor(&self) -> f64 {
        
        todo!();
        /*
            return 1.0;
        */
    }
}
