crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUBaseHelper.h]

/**
   helpers for dealing with the file-references
   dictionary in an AUPreset
  */
pub fn get_file_ref_path(
        parent: CFDictionaryRef,
        fr_key: CFStringRef,
        path:   *mut CFStringRef) -> OSStatus {
    
    todo!();
        /*
        
        */
}

/**
  | if fileRefDict is NULL, this call creates one
  |
  | if not NULL, then the key value is added to it
  */
pub fn create_file_ref_dict(
        key:           CFStringRef,
        path:          CFStringRef,
        file_ref_dict: CFMutableDictionaryRef) -> CFMutableDictionaryRef {
    
    todo!();
        /*
        
        */
}

pub fn access_url_asset(
        inurl: CFURLRef,
        mode:  i32) -> i32 {
    
    todo!();
        /*
        
        */
}

#[cfg(DEBUG)]
pub fn print_au_param_event(
        event: &mut AudioUnitParameterEvent,
        f:     *mut libc::FILE)  {
    
    todo!();
        /*
        
        */
}
