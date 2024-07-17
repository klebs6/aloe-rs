crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/native/aloe_OpenGL_linux_X11.h]

#[cfg(target_os="linux")]
lazy_static!{
    /*
    extern XContext windowHandleXContext;
    */
}

/**
  | Defined aloe_linux_Windowing.cpp
  |
  */
#[cfg(target_os="linux")]
pub fn aloe_linux_add_repaint_listener(
        _0:    *mut ComponentPeer,
        dummy: *mut Component)  {
    
    todo!();
        /*
        
        */
}

#[cfg(target_os="linux")]
pub fn aloe_linux_remove_repaint_listener(
        _0:    *mut ComponentPeer,
        dummy: *mut Component)  {
    
    todo!();
        /*
        
        */
}

#[cfg(target_os="linux")]
lazy_static!{
    /*
    static constexpr int embeddedWindowEventMask = ExposureMask | StructureNotifyMask;
    */
}
