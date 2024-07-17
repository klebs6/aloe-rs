crate::ix!();

pub trait MemoryWarningReceived {

    /**
      | Called by the operating system to indicate
      | that you should reduce your memory footprint.
      | 
      | You should override this method to free
      | up some memory gracefully, if possible,
      | otherwise the host may forcibly kill
      | your app.
      | 
      | At the moment this method is only called
      | on iOS.
      |
      */
    fn memory_warning_received(&mut self)  {

        panic!("memory warning received"); // may override this default behavior
    }
}

pub trait ReleaseResources {

    /**
      | Called after playback has stopped,
      | to let the object free up any resources
      | it no longer needs.
      |
      */
    fn release_resources(&mut self);
}

