crate::ix!();

pub trait SetNonRealtime {

    /**
      | Called by the host to tell this processor
      | whether it's being used in a non-realtime
      | capacity for offline rendering or bouncing.
      |
      */
    fn set_non_realtime(&mut self, is_non_realtime: bool);
}

pub trait IsNonRealtime {

    /**
      | Returns true if the processor is being
      | run in an offline mode for rendering.
      | 
      | If the processor is being run live on
      | realtime signals, this returns false.
      | 
      | If the mode is unknown, this will assume
      | it's realtime and return false.
      | 
      | This value may be unreliable until the
      | prepareToPlay() method has been called,
      | and could change each time prepareToPlay()
      | is called.
      | 
      | @see setNonRealtime()
      |
      */
    fn is_non_realtime(&self) -> bool;
}
