crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/ComponentBase.cpp]

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(TARGET_OS_MAC)]
lazy_static!{
    /*
    static dispatch_once_t sCSInitOnce = 0;
    */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(TARGET_OS_MAC)]
pub fn cs_init()  {
    
    todo!();
        /*
            dispatch_once_f(&sCSInitOnce, NULL, CSInitOnce);
        */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(not(TARGET_OS_MAC))]
pub fn cs_init()  {
    
    todo!();
        /*
            static int sDoCSLoad = 1;
        if (sDoCSLoad) {
            sDoCSLoad = 0;
            CSInitOnce(NULL);
        }
        */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn cs_init_once(unused: *mut c_void)  {
    
    todo!();
        /*
            void *theImage = dlopen("/System/Library/Frameworks/CoreServices.framework/CoreServices", RTLD_LAZY);
        if (!theImage) return;

        sGetComponentInstanceStorageProc = (GetComponentInstanceStorageProc) dlsym(theImage, "GetComponentInstanceStorage");
        sGetComponentInfoProc = (GetComponentInfoProc)dlsym (theImage, "GetComponentInfo");
        sSetComponentInstanceStorageProc = (SetComponentInstanceStorageProc) dlsym(theImage, "SetComponentInstanceStorage");
        */
}
