crate::ix!();

pub trait BroughtToFront {

    /**
      | Called when this component has been
      | moved to the front of its siblings.
      | 
      | The component may have been brought
      | to the front by the toFront() method,
      | or by the operating system if it's a top-level
      | window.
      | 
      | @see toFront
      |
      */
    fn brought_to_front(&mut self);
}

pub trait SetAlwaysOnTop {

    /**
      | Sets this window to either be always-on-top
      | or normal.
      | 
      | Some kinds of window might not be able
      | to do this, so should return false.
      |
      */
    fn set_always_on_top(&mut self, always_on_top: bool) -> bool;
}

pub trait ToFront {

    /**
      | Brings the window to the top, optionally
      | also giving it keyboard focus.
      |
      */
    fn to_front(&mut self, take_keyboard_focus: bool);
}

pub trait ToBehind {

    /**
      | Moves the window to be just behind another
      | one.
      |
      */
    fn to_behind(&mut self, other: *mut ComponentPeer);
}
