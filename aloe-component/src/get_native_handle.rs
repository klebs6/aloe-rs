crate::ix!();

pub trait GetIconNativeHandle {

    /**
      | Returns the raw handle to whatever kind
      | of internal OS structure is involved
      | in showing this icon. @see ComponentPeer::getNativeHandle()
      |
      */
    fn get_native_handle(&self);
}

pub trait GetWindowNativeHandle {

    /**
      | Returns the raw handle to whatever kind
      | of window is being used.
      | 
      | On windows, this is probably a HWND,
      | on the mac, it's likely to be a WindowRef,
      | but remember there's no guarantees
      | what you'll get back.
      |
      */
    fn get_native_handle(&self);
}


