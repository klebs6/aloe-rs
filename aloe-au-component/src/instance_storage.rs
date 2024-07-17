crate::ix!();

#[cfg(not(__COREAUDIO_USE_FLAT_INCLUDES__))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(MAC_OS_X_VERSION_MAX_ALLOWED_LTE_MAC_OS_X_VERSION_10_5)]
macro_rules! audio_component_instance {
    () => {
        /*
                ComponentInstance
        */
    }
}

/**
  | Component Manager
  | 
  | Used for fast dispatch with audio units
  |
  */
#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub type get_component_instance_storage_proc = fn(a_component_instance: AudioComponentInstance) -> Handle;

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
lazy_static!{
    /*
    static GetComponentInstanceStorageProc sGetComponentInstanceStorageProc = NULL;
    */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub type set_component_instance_storage_proc = fn(_0: AudioComponentInstance, _1: Handle) -> *mut c_void;

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
lazy_static!{
    /*
    static SetComponentInstanceStorageProc sSetComponentInstanceStorageProc = NULL;
    */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_get_component_instance_storage(a_component_instance: AudioComponentInstance) -> Handle {
    
    todo!();
        /*
            CSInit();
        if (sGetComponentInstanceStorageProc)
            return (*sGetComponentInstanceStorageProc)(aComponentInstance);
        return NULL;
        */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_set_component_instance_storage(
        a_component_instance: AudioComponentInstance,
        the_storage:          Handle)  {
    
    todo!();
        /*
            CSInit();
        if (sSetComponentInstanceStorageProc)
            (*sSetComponentInstanceStorageProc)(aComponentInstance, theStorage);
        */
}
