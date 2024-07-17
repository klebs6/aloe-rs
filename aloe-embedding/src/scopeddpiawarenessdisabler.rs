crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_ScopedDPIAwarenessDisabler.h]

/**
  | A Windows-specific class that temporarily
  | sets the DPI awareness context of the
  | current thread to be DPI unaware and
  | resets it to the previous context when
  | it goes out of scope.
  | 
  | If you create one of these before creating
  | a top-level window, the window will
  | be DPI unaware and bitmap stretched
  | by the OS on a display with >100% scaling.
  | 
  | You shouldn't use this unless you really
  | know what you are doing and are dealing
  | with native HWNDs.
  | 
  | @tags{GUI}
  |
  */
pub struct ScopedDPIAwarenessDisabler {
    previous_context: *mut c_void, // default = nullptr
}

#[cfg(not(target_os="windows"))]
impl ScopedDPIAwarenessDisabler {

    pub fn new() -> Self {
    
        todo!();
        /*


            ignoreUnused (previousContext);
        */
    }
}
