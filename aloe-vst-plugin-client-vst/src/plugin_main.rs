crate::ix!();

/* -------------- Mac startup code..   -------------- */
#[cfg(target_os="macos")]
pub fn vst_plugin_main(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            initialiseMacVST();
            return pluginEntryPoint (audioMaster);
        */
}

#[cfg(target_os="macos")]
pub fn main_macho(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            initialiseMacVST();
            return pluginEntryPoint (audioMaster);
        */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn vst_plugin_main(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            return pluginEntryPoint (audioMaster);
        */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
#[asm ("main")]
pub fn main_plugin(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_VST;

            return VSTPluginMain (audioMaster);
        */
}


/**
   don't put initialiseAloe_GUI or
   shutdownAloe_GUI in these... it will crash!
  */
#[cfg(any(target_os="linux",target_os="bsd"))]
#[ctor]
pub fn my_plugin_init()  {
    
    todo!();
        /*
        
        */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
#[dtor]
pub fn my_plugin_fini()  {
    
    todo!();
        /*
        
        */
}
