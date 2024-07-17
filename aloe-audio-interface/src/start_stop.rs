crate::ix!();

pub trait DeviceStart {

    /**
      | Starts the device actually playing.
      | 
      | This must be called after the device
      | has been opened.
      | 
      | -----------
      | @param callback
      | 
      | the callback to use for streaming the
      | data. @see AudioIODeviceCallback,
      | open
      |
      */
    fn start(&mut self, callback: *mut dyn AudioIODeviceCallback);
}

pub trait Stop {

    /**
      | Stops the device playing.
      | 
      | Once a device has been started, this
      | will stop it. Any pending calls to the
      | callback class will be flushed before
      | this method returns.
      |
      */
    fn stop(&mut self);
}
