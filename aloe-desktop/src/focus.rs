crate::ix!();

/**
  | Classes can implement this interface
  | and register themselves with the Desktop
  | class to receive callbacks when the
  | currently focused component changes.
  | 
  | @see Desktop::addFocusChangeListener,
  | Desktop::removeFocusChangeListener
  | 
  | @tags{GUI}
  |
  */
pub trait FocusChangeListener: GlobalFocusChanged { }

pub trait GlobalFocusChanged {

    /**
      | Callback to indicate that the currently
      | focused component has changed.
      |
      */
    fn global_focus_changed(&mut self, focused_component: *mut Component);
}
