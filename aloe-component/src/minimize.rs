crate::ix!();

pub trait SetMinimised {

    /**
      | Minimises the window.
      |
      */
    fn set_minimised(&mut self, should_be_minimised: bool);
}

pub trait IsMinimised {

    /**
      | True if the window is currently minimised.
      |
      */
    fn is_minimised(&self) -> bool;
}

pub trait MinimisationStateChanged {

    /**
      | Called for a desktop component which
      | has just been minimised or un-minimised.
      | 
      | This will only be called for components
      | on the desktop. @see getPeer, ComponentPeer::setMinimised,
      | ComponentPeer::isMinimised
      |
      */
    fn minimisation_state_changed(&mut self, is_now_minimised: bool);
}
