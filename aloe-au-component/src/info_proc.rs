crate::ix!();

/*
  | these are the direct dependencies on
  | ComponentMgr calls that an AU that is
  | a component mgr is dependent on
  |
  | these are dynamically loaded
  */
#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub type get_component_info_proc = fn(
        _0: Component,
        _1: *mut ComponentDescription,
        _2: *mut c_void,
        _3: *mut c_void,
        _4: *mut c_void
) -> OSErr;

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
lazy_static!{
    /*
    static GetComponentInfoProc sGetComponentInfoProc = NULL;
    */
}

