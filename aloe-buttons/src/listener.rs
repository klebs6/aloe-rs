crate::ix!();

/**
  | Used to receive callbacks when a button
  | is clicked.
  | 
  | @see Button::addListener, Button::removeListener
  |
  */
pub trait ButtonListener {

    /**
      | Called when the button is clicked.
      |
      */
    fn button_clicked(&mut self, _0: *mut Button);

    /**
      | Called when the button's state changes.
      |
      */
    fn button_state_changed(&mut self, _0: *mut Button)  { }
}
