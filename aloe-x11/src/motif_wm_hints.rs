crate::ix!();

lazy_static!{
    /*
    Window aloe_messageWindowHandle;
    XContext windowHandleXContext;
    */
}

#[cfg(ALOE_X11_SUPPORTS_XEMBED)]
pub fn aloe_handle_xembed_event(
        _0: *mut ComponentPeer,
        _1: *mut c_void) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(ALOE_X11_SUPPORTS_XEMBED)]
pub fn aloe_get_current_focus_window(_0: *mut ComponentPeer) -> u64 {
    
    todo!();
        /*
        
        */
}

pub struct MotifWmHints
{
    flags:       u64, // default = 0
    functions:   u64, // default = 0
    decorations: u64, // default = 0
    input_mode:  i64, // default = 0
    status:      u64, // default = 0
}
