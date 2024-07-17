crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/MusicDeviceBase.h]

pub struct MusicDeviceBase {
    base:  AUBase,
    base2: AUMIDIBase,
}

impl MusicDeviceBase {

    pub fn new(
        in_instance: AudioComponentInstance,
        num_inputs:  u32,
        num_outputs: u32,
        num_groups:  Option<u32>

    ) -> Self {

        let num_groups: u32 = num_groups.unwrap_or(0);
    
        todo!();
        /*
        : au_base(inInstance, numInputs, numOutputs, numGroups),
        : aumidi_base(this),

        
        */
    }
    
    pub fn get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
            OSStatus result;

        switch (inID)
        {
    #if !TARGET_OS_IPHONE
            case kMusicDeviceProperty_InstrumentCount:
                if (inScope != kAudioUnitScope_Global) return kAudioUnitErr_InvalidScope;
                outDataSize = sizeof(UInt32);
                outWritable = false;
                result = noErr;
                break;
    #endif
            default:
                result = AUBase::GetPropertyInfo (inID, inScope, inElement, outDataSize, outWritable);

                if (result == kAudioUnitErr_InvalidProperty)
                    result = AUMIDIBase::DelegateGetPropertyInfo (inID, inScope, inElement, outDataSize, outWritable);
                break;
        }
        return result;
        */
    }
    
    pub fn get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
            OSStatus result;

        switch (inID)
        {
    #if !CA_USE_AUDIO_PLUGIN_ONLY
            case kAudioUnitProperty_FastDispatch:
                if (!IsCMgrObject()) return kAudioUnitErr_InvalidProperty;
                if (inElement == kMusicDeviceMIDIEventSelect) {
                    *(TEMP_MusicDeviceMIDIEventProc *)outData = MusicDeviceBaseMIDIEvent;
                    return noErr;
                }
                else if (inElement == kMusicDeviceStartNoteSelect) {
                    *(TEMP_MusicDeviceStartNoteProc *)outData = MusicDeviceBaseStartNote;
                    return noErr;
                }
                else if (inElement == kMusicDeviceStopNoteSelect) {
                    *(TEMP_MusicDeviceStopNoteProc *)outData = MusicDeviceBaseStopNote;
                    return noErr;
                }
                return kAudioUnitErr_InvalidElement;
    #endif

    #if !TARGET_OS_IPHONE
            case kMusicDeviceProperty_InstrumentCount:
                if (inScope != kAudioUnitScope_Global) return kAudioUnitErr_InvalidScope;
                return GetInstrumentCount (*(UInt32*)outData);
    #endif
            default:
                result = AUBase::GetProperty (inID, inScope, inElement, outData);

                if (result == kAudioUnitErr_InvalidProperty)
                    result = AUMIDIBase::DelegateGetProperty (inID, inScope, inElement, outData);
        }

        return result;
        */
    }
    
    pub fn set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = AUBase::SetProperty (inID, inScope, inElement, inData, inDataSize);

        if (result == kAudioUnitErr_InvalidProperty)
            result = AUMIDIBase::DelegateSetProperty (inID, inScope, inElement, inData, inDataSize);

        return result;
        */
    }

    /**
      | For a MusicDevice that doesn't support
      | separate instruments (ie. is mono-timbral)
      | then this call should return an instrument
      | count of zero and noErr
      */
    pub fn get_instrument_count(&self, out_inst_count: &mut u32) -> OSStatus {
        
        todo!();
        /*
            outInstCount = 0;
        return noErr;
        */
    }
    
    pub fn handle_note_on(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            MusicDeviceNoteParams params;
        params.argCount = 2;
        params.mPitch = inNoteNumber;
        params.mVelocity = inVelocity;
        return StartNote (kMusicNoteEvent_UseGroupInstrument, inChannel, NULL, inStartFrame, params);
        */
    }
    
    pub fn handle_note_off(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return StopNote (inChannel, inNoteNumber, inStartFrame);
        */
    }
    
    pub fn handle_start_note_message(&mut self, 
        in_instrument:          MusicDeviceInstrumentID,
        in_groupid:             MusicDeviceGroupID,
        out_note_instanceid:    *mut NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params:              *const MusicDeviceNoteParams) -> OSStatus {
        
        todo!();
        /*
            if (inParams == NULL || outNoteInstanceID == NULL) return kAudio_ParamError;

        if (!IsInitialized()) return kAudioUnitErr_Uninitialized;

        return StartNote (inInstrument, inGroupID, outNoteInstanceID, inOffsetSampleFrame, *inParams);
        */
    }

    /**
      | component dispatcher
      |
      */
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn component_entry_dispatch(&mut self, 
        params: *mut ComponentParameters,
        this:   *mut MusicDeviceBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return kAudio_ParamError;

        OSStatus result;

        switch (params->what) {
        case kMusicDeviceMIDIEventSelect:
        case kMusicDeviceSysExSelect:
            {
                result = AUMIDIBase::ComponentEntryDispatch (params, This);
            }
            break;
        case kMusicDevicePrepareInstrumentSelect:
            {
                PARAM(MusicDeviceInstrumentID, inInstrument, 0, 1);
                result = This->PrepareInstrument(inInstrument);
            }
            break;
        case kMusicDeviceReleaseInstrumentSelect:
            {
                PARAM(MusicDeviceInstrumentID, inInstrument, 0, 1);
                result = This->ReleaseInstrument(inInstrument);
            }
            break;
        case kMusicDeviceStartNoteSelect:
            {
                PARAM(MusicDeviceInstrumentID, pbinInstrument, 0, 5);
                PARAM(MusicDeviceGroupID, pbinGroupID, 1, 5);
                PARAM(NoteInstanceID *, pboutNoteInstanceID, 2, 5);
                PARAM(UInt32, pbinOffsetSampleFrame, 3, 5);
                PARAM(const MusicDeviceNoteParams *, pbinParams, 4, 5);
                result = This->HandleStartNoteMessage(pbinInstrument, pbinGroupID, pboutNoteInstanceID, pbinOffsetSampleFrame, pbinParams);
            }
            break;
        case kMusicDeviceStopNoteSelect:
            {
                PARAM(MusicDeviceGroupID, pbinGroupID, 0, 3);
                PARAM(NoteInstanceID, pbinNoteInstanceID, 1, 3);
                PARAM(UInt32, pbinOffsetSampleFrame, 2, 3);
                result = This->StopNote(pbinGroupID, pbinNoteInstanceID, pbinOffsetSampleFrame);
            }
            break;

        default:
            result = AUBase::ComponentEntryDispatch(params, This);
            break;
        }

        return result;
        */
    }
}
