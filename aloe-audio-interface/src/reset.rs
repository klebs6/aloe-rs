crate::ix!();

pub trait Reset {

    /**
      | A plugin can override this to be told
      | when it should reset any playing voices.
      | 
      | The default implementation does nothing,
      | but a host may call this to tell the plugin
      | that it should stop any tails or sounds
      | that have been left running.
      |
      */
    fn reset(&mut self);
}

pub trait IsResetAllControllers {

    /**
      | Checks whether this message is a reset
      | all controllers message. @see allControllerOff
      |
      */
    fn is_reset_all_controllers(&self) -> bool;
}
