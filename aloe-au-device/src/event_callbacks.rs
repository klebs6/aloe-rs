crate::ix!();

/**
   compatibility with older OS SDK releases
  */
pub type TempMusicDeviceMidiEventProc = fn(
    in_component_storage:   *mut c_void,
    in_status:              u32,
    in_data1:               u32,
    in_data2:               u32,
    in_offset_sample_frame: u32
) -> OSStatus;

pub type TempMusicDeviceStartNoteProc = fn(
        in_component_storage:   *mut c_void,
        in_instrument:          MusicDeviceInstrumentID,
        in_groupid:             MusicDeviceGroupID,
        out_note_instanceid:    *mut NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params:              *const MusicDeviceNoteParams
) -> OSStatus;

pub type TempMusicDeviceStopNoteProc = fn(
        in_component_storage:   *mut c_void,
        in_groupid:             MusicDeviceGroupID,
        in_note_instanceid:     NoteInstanceID,
        in_offset_sample_frame: u32
) -> OSStatus;


//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/MusicDeviceBase.cpp]

/**
   fast dispatch
  */
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn music_device_base_midi_event(
        in_component_storage:   *mut c_void,
        in_status:              u32,
        in_data1:               u32,
        in_data2:               u32,
        in_offset_sample_frame: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            MusicDeviceBase *This = static_cast<MusicDeviceBase *>(inComponentStorage);
            if (This == NULL) return kAudio_ParamError;
            result = This->MIDIEvent(inStatus, inData1, inData2, inOffsetSampleFrame);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn music_device_base_start_note(
        in_component_storage:   *mut c_void,
        in_instrument:          MusicDeviceInstrumentID,
        in_groupid:             MusicDeviceGroupID,
        out_note_instanceid:    *mut NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params:              *const MusicDeviceNoteParams) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            if (inParams == NULL || outNoteInstanceID == NULL) return kAudio_ParamError;
            MusicDeviceBase *This = static_cast<MusicDeviceBase *>(inComponentStorage);
            if (This == NULL) return kAudio_ParamError;
            result = This->StartNote(inInstrument, inGroupID, outNoteInstanceID, inOffsetSampleFrame, *inParams);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub fn music_device_base_stop_note(
        in_component_storage:   *mut c_void,
        in_groupid:             MusicDeviceGroupID,
        in_note_instanceid:     NoteInstanceID,
        in_offset_sample_frame: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            MusicDeviceBase *This = static_cast<MusicDeviceBase *>(inComponentStorage);
            if (This == NULL) return kAudio_ParamError;
            result = This->StopNote(inGroupID, inNoteInstanceID, inOffsetSampleFrame);
        }
        COMPONENT_CATCH
        return result;
        */
}
