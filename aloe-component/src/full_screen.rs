crate::ix!();

pub trait SetFullScreen {

    /**
      | Enable/disable fullscreen mode for
      | the window.
      |
      */
    fn set_full_screen(&mut self, should_be_full_screen: bool);
}

pub trait IsFullScreen {

    /**
      | True if the window is currently full-screen.
      |
      */
    fn is_full_screen(&self) -> bool;

}
