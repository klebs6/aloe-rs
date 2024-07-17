crate::ix!();

/* ------------- Win32 startup code..   ------------- */

#[dllexport]
#[cfg(not(any(target_os="macos",any(target_os="linux",target_os="bsd"))))]
pub fn vst_plugin_main(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            return pluginEntryPoint (audioMaster);
        */
}

/**
  | (can't compile this on win64, but it's
  | not needed anyway with VST2.4)
  |
  */
#[dllexport]
#[cfg(all(not(ALOE_64BIT),ALOE_MSVC))]
#[cfg(not(any(target_os="macos",any(target_os="linux",target_os="bsd"))))]
pub fn main(audio_master: Vst2::audioMasterCallback) -> i32 {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            return (int) pluginEntryPoint (audioMaster);
        */
}

#[dllexport]
#[WINAPI]
#[cfg(not(any(target_os="macos",any(target_os="linux",target_os="bsd"))))]
pub fn dll_main(
    instance: HINSTANCE,
    reason:   u64,
    _2:       LPVOID

) -> bool {
    
    todo!();
        /*
            if (reason == DLL_PROCESS_ATTACH)
                Process::setCurrentModuleInstanceHandle (instance);

            return true;
        */
}
