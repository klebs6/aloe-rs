crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/ComponentBase.h]

#[cfg(CA_DO_NOT_USE_AUDIO_COMPONENT)]
pub fn component_base_get_component_description(
        in_instance: &AudioComponentInstance,
        out_desc:    &mut AudioComponentDescription) -> OSStatus {
    
    todo!();
        /*
        
        */
}

#[cfg(not(__COREAUDIO_USE_FLAT_INCLUDES__))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(MAC_OS_X_VERSION_MAX_ALLOWED_LTE_MAC_OS_X_VERSION_10_5)]
macro_rules! audio_component_description {
    () => {
        /*
                ComponentDescription
        */
    }
}

/**
  | everything we need is there and we should
  | be linking against it
  |
  */
#[cfg(CA_USE_AUDIO_PLUGIN_ONLY)]
pub fn cb_get_component_description(
        in_instance: AudioComponentInstance,
        out_desc:    *mut AudioComponentDescription) -> OSStatus {
    
    todo!();
        /*
            AudioComponent comp = AudioComponentInstanceGetComponent(inInstance);
        if (comp)
            return AudioComponentGetDescription(comp, outDesc);

        return kAudio_ParamError;
        */
}


/**
  | these are the direct dependencies on
  | ComponentMgr calls that an AU that is
  | a component mgr is dependent on
  |
  | these are dynamically loaded so that these
  | calls will work on Leopard
  | #include <dlfcn.h>
  */
#[cfg(not(TARGET_OS_WIN32))]
pub fn cb_get_component_description(
    in_instance: AudioComponentInstance,
    out_desc:    *mut AudioComponentDescription

) -> OSStatus {

    type AudioComponentInstanceGetComponentProc = fn(_0: AudioComponentInstance) -> AudioComponent;
    type AudioComponentGetDescriptionProc       = fn(_0: AudioComponent, _1: *mut AudioComponentDescription) -> OSStatus;
    
    todo!();
        /*
        static AudioComponentInstanceGetComponentProc aciGCProc = NULL;

        static AudioComponentGetDescriptionProc acGDProc = NULL;

        static int doneInit = 0;
        if (doneInit == 0) {
            doneInit = 1;
            void* theImage = dlopen("/System/Library/Frameworks/AudioUnit.framework/AudioUnit", RTLD_LAZY);
            if (theImage != NULL)
            {
                aciGCProc = (AudioComponentInstanceGetComponentProc)dlsym (theImage, "AudioComponentInstanceGetComponent");
                if (aciGCProc) {
                    acGDProc = (AudioComponentGetDescriptionProc)dlsym (theImage, "AudioComponentGetDescription");
                }
            }
        }

        OSStatus result = kAudio_UnimplementedError;
        if (acGDProc && aciGCProc) {
            AudioComponent comp = (*aciGCProc)(inInstance);
            if (comp)
                result = (*acGDProc)(comp, outDesc);
        }
    #if !CA_USE_AUDIO_PLUGIN_ONLY
        else {
            result = CMgr_GetComponentDescription (inInstance, outDesc);
        }
    #endif

        return result;
        */
}

/**
  |#include "ComponentManagerDependenciesWin.h"
  |
  | everything we need is there and we should be
  | linking against it
  */
#[cfg(TARGET_OS_WIN32)]
pub fn cb_get_component_description(
    in_instance: AudioComponentInstance,
    out_desc:    *mut AudioComponentDescription

) -> OSStatus {
    
    todo!();
        /*
            AudioComponent comp = AudioComponentInstanceGetComponent(inInstance);
        if (comp)
            return AudioComponentGetDescription(comp, outDesc);

        return kAudio_ParamError;
        */
}

#[cfg(not(TARGET_OS_WIN32))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_get_component_description(
        in_instance: AudioComponentInstance,
        out_desc:    *mut AudioComponentDescription) -> OSStatus {
    
    todo!();
        /*
            CSInit();
        if (sGetComponentInfoProc)
            return (*sGetComponentInfoProc)((Component)inInstance, (ComponentDescription*)outDesc, NULL, NULL, NULL);
        return kAudio_UnimplementedError;
        */
}
