crate::ix!();

//TODO
#[derive(Default)]
pub struct Window {}

pub fn aloe_create_key_proxy_window(_0: *mut ComponentPeer) -> Window {
    
    todo!();
        /*
        
        */
}

pub fn aloe_delete_key_proxy_window(_0: Window)  {
    
    todo!();
        /*
        
        */
}

pub fn aloe_handle_xembed_event(
    p: *mut ComponentPeer,
    e: *mut c_void) -> bool {
    
    todo!();
        /*
            return XEmbedComponent::Impl::dispatchX11Event (p, reinterpret_cast<const XEvent*> (e));
        */
}

pub fn aloe_get_current_focus_window(peer: *mut ComponentPeer) -> u64 {
    
    todo!();
        /*
            return (unsigned long) XEmbedComponent::Impl::getCurrentFocusWindow (peer);
        */
}
