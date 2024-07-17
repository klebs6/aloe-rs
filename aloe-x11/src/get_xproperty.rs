crate::ix!();

/**
  | Gets a specified window property and
  | stores its associated data, freeing
  | it on deletion.
  | 
  | @tags{GUI}
  |
  */
pub struct GetXProperty {
    success:       bool, // default = false
    data:          *mut u8, // default = nullptr
    num_items:     u64, // default = 0
    bytes_left:    u64, // default = 0
    actual_type:   Atom,
    actual_format: i32, // default = -1
}

impl Drop for GetXProperty {

    fn drop(&mut self) {
        todo!();
        /*
            if (data != nullptr)
            X11Symbols::getInstance()->xFree (data);
        */
    }
}

impl GetXProperty {
    
    pub fn new(
        window:         Window,
        atom:           Atom,
        offset:         i64,
        length:         i64,
        should_delete:  bool,
        requested_type: Atom) -> Self {
    
        todo!();
        /*


            success = (X11Symbols::getInstance()->xGetWindowProperty (XWindowSystem::getInstance()->getDisplay(),
                                                                  window, atom, offset, length,
                                                                  (Bool) shouldDelete, requestedType, &actualType,
                                                                  &actualFormat, &numItems, &bytesLeft, &data) == Success)
                    && data != nullptr;
        */
    }
}
