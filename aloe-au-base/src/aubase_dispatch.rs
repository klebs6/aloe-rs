crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUDispatch.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUDispatch.cpp]

/**
   comp instance, parameters in forward order
  */
#[cfg(TARGET_OS_MAC)]
#[cfg(__LP64__)]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_index + 1];
        */
    }
}

/**
   parameters in reverse order, then comp instance
  */
#[cfg(TARGET_OS_MAC)]
#[cfg(not(__LP64__))]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_nparams - 1 - _index];
        */
    }
}

/**
   (no comp instance), parameters in forward order
  */
#[cfg(TARGET_OS_WIN32)]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_index];
        */
    }
}

/**
   Fast dispatch entry points -- these need to
   replicate all error-checking logic from above
  */
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_audio_unit_base_get_parameter(
        this:       *mut AUBase,
        inid:       AudioUnitParameterID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_value:  *mut f32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = AUBase::noErr;

        try {
            if (This == NULL || outValue == NULL) return kAudio_ParamError;
            result = This->GetParameter(inID, inScope, inElement, *outValue);
        }
        COMPONENT_CATCH

        return result;
        */
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_audio_unit_base_set_parameter(
        this:             *mut AUBase,
        inid:             AudioUnitParameterID,
        in_scope:         AudioUnitScope,
        in_element:       AudioUnitElement,
        in_value:         f32,
        in_buffer_offset: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = AUBase::noErr;

        try {
            if (This == NULL) return kAudio_ParamError;
            result = This->SetParameter(inID, inScope, inElement, inValue, inBufferOffset);
        }
        COMPONENT_CATCH

        return result;
        */
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn mgr_audio_unit_base_render(
        this:             *mut AUBase,
        io_action_flags:  *mut AudioUnitRenderActionFlags,
        in_time_stamp:    *const AudioTimeStamp,
        in_bus_number:    u32,
        in_number_frames: u32,
        io_data:          *mut AudioBufferList) -> OSStatus {
    
    todo!();
        /*
            if (inTimeStamp == NULL || ioData == NULL) return kAudio_ParamError;

        OSStatus result = AUBase::noErr;
        AudioUnitRenderActionFlags tempFlags;

        try {
            if (ioActionFlags == NULL) {
                tempFlags = 0;
                ioActionFlags = &tempFlags;
            }
            result = This->DoRender(*ioActionFlags, *inTimeStamp, inBusNumber, inNumberFrames, *ioData);
        }
        COMPONENT_CATCH

        return result;
        */
}

