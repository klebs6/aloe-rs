crate::ix!();

pub trait GetMouseCursor {

    /**
      | Returns the mouse cursor shape to use
      | when the mouse is over this component.
      | 
      | The default implementation will return
      | the cursor that was set by setCursor()
      | but can be overridden for more specialised
      | purposes, e.g. returning different
      | cursors depending on the mouse position.
      | 
      | @see MouseCursor
      |
      */
    fn get_mouse_cursor(&mut self) -> MouseCursor;
}
