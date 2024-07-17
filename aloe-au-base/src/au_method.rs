crate::ix!();

pub fn au_method_initialize(self_: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->DoInitialize();
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_uninitialize(self_: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            AUI->DoCleanup();
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_get_property_info(
        self_:         *mut c_void,
        prop:          AudioUnitPropertyID,
        scope:         AudioUnitScope,
        elem:          AudioUnitElement,
        out_data_size: *mut u32,
        out_writable:  *mut bool) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            UInt32 dataSize = 0;        // 13517289 GetPropetyInfo was returning an uninitialized value when there is an error. This is a problem for auval.
            Boolean writable = false;

            AUI_LOCK
            result = AUI->DispatchGetPropertyInfo(prop, scope, elem, dataSize, writable);
            if (outDataSize != NULL)
                *outDataSize = dataSize;
            if (outWritable != NULL)
                *outWritable = writable;
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_get_property(
        self_:        *mut c_void,
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        out_data:     *mut c_void,
        io_data_size: *mut u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            UInt32 actualPropertySize, clientBufferSize;
            Boolean writable;
            char *tempBuffer;
            void *destBuffer;

            AUI_LOCK
            if (ioDataSize == NULL) {
                ca_debug_string("AudioUnitGetProperty: null size pointer");
                result = kAudio_ParamError;
                goto finishGetProperty;
            }
            if (outData == NULL) {
                UInt32 dataSize;

                result = AUI->DispatchGetPropertyInfo(inID, inScope, inElement, dataSize, writable);
                *ioDataSize = dataSize;
                goto finishGetProperty;
            }

            clientBufferSize = *ioDataSize;
            if (clientBufferSize == 0)
            {
                ca_debug_string("AudioUnitGetProperty: *ioDataSize == 0 on entry");
                // $$$ or should we allow this as a shortcut for finding the size?
                result = kAudio_ParamError;
                goto finishGetProperty;
            }

            result = AUI->DispatchGetPropertyInfo(inID, inScope, inElement, actualPropertySize, writable);
            if (result != noErr)
                goto finishGetProperty;

            if (clientBufferSize < actualPropertySize)
            {
                tempBuffer = new char[actualPropertySize];
                destBuffer = tempBuffer;
            } else {
                tempBuffer = NULL;
                destBuffer = outData;
            }

            result = AUI->DispatchGetProperty(inID, inScope, inElement, destBuffer);

            if (result == noErr) {
                if (clientBufferSize < actualPropertySize && tempBuffer != NULL)
                {
                    memcpy(outData, tempBuffer, clientBufferSize);
                    delete[] tempBuffer;
                    // ioDataSize remains correct, the number of bytes we wrote
                } else
                    *ioDataSize = actualPropertySize;
            } else
                *ioDataSize = 0;
        }
        COMPONENT_CATCH
    finishGetProperty:
        return result;
        */
}

pub fn au_method_set_property(
        self_:        *mut c_void,
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            if (inData && inDataSize)
                result = AUI->DispatchSetProperty(inID, inScope, inElement, inData, inDataSize);
            else {
                if (inData == NULL && inDataSize == 0) {
                    result = AUI->DispatchRemovePropertyValue(inID, inScope, inElement);
                } else {
                    if (inData == NULL) {
                        ca_debug_string("AudioUnitSetProperty: inData == NULL");
                        result = kAudio_ParamError;
                        goto finishSetProperty;
                    }

                    if (inDataSize == 0) {
                        ca_debug_string("AudioUnitSetProperty: inDataSize == 0");
                        result = kAudio_ParamError;
                        goto finishSetProperty;
                    }
                }
            }
        }
        COMPONENT_CATCH
    finishSetProperty:
        return result;
        */
}

pub fn au_method_add_property_listener(
        self_:     *mut c_void,
        prop:      AudioUnitPropertyID,
        proc:      AudioUnitPropertyListenerProc,
        user_data: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->AddPropertyListener(prop, proc, userData);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_remove_property_listener(
        self_: *mut c_void,
        prop:  AudioUnitPropertyID,
        proc:  AudioUnitPropertyListenerProc) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->RemovePropertyListener(prop, proc, NULL, false);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_remove_property_listener_with_user_data(
        self_:     *mut c_void,
        prop:      AudioUnitPropertyID,
        proc:      AudioUnitPropertyListenerProc,
        user_data: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->RemovePropertyListener(prop, proc, userData, true);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_add_render_notify(
        self_:     *mut c_void,
        proc:      AURenderCallback,
        user_data: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->SetRenderNotification(proc, userData);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_remove_render_notify(
        self_:     *mut c_void,
        proc:      AURenderCallback,
        user_data: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->RemoveRenderNotification(proc, userData);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_get_parameter(
        self_: *mut c_void,
        param: AudioUnitParameterID,
        scope: AudioUnitScope,
        elem:  AudioUnitElement,
        value: *mut AudioUnitParameterValue) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = (value == NULL ? kAudio_ParamError : AUI->GetParameter(param, scope, elem, *value));
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_set_parameter(
        self_:         *mut c_void,
        param:         AudioUnitParameterID,
        scope:         AudioUnitScope,
        elem:          AudioUnitElement,
        value:         AudioUnitParameterValue,
        buffer_offset: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a (potentially) realtime method; no lock
            result = AUI->SetParameter(param, scope, elem, value, bufferOffset);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_schedule_parameters(
        self_:      *mut c_void,
        events:     *const AudioUnitParameterEvent,
        num_events: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a (potentially) realtime method; no lock
            result = AUI->ScheduleParameter(events, numEvents);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_render(
        self_:                *mut c_void,
        io_action_flags:      *mut AudioUnitRenderActionFlags,
        in_time_stamp:        *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames:     u32,
        io_data:              *mut AudioBufferList) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;

    #if !TARGET_OS_IPHONE
        try {
    #endif
            // this is a processing method; no lock
            AudioUnitRenderActionFlags tempFlags;

            if (inTimeStamp == NULL || ioData == NULL)
                result = kAudio_ParamError;
            else {
                if (ioActionFlags == NULL) {
                    tempFlags = 0;
                    ioActionFlags = &tempFlags;
                }
                result = AUI->DoRender(*ioActionFlags, *inTimeStamp, inOutputBusNumber, inNumberFrames, *ioData);
            }

    #if !TARGET_OS_IPHONE
        }
        COMPONENT_CATCH
    #endif

        return result;
        */
}

pub fn au_method_complex_render(
        self_:                   *mut c_void,
        io_action_flags:         *mut AudioUnitRenderActionFlags,
        in_time_stamp:           *const AudioTimeStamp,
        in_output_bus_number:    u32,
        in_number_of_packets:    u32,
        out_number_of_packets:   *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        io_data:                 *mut AudioBufferList,
        out_metadata:            *mut c_void,
        out_metadata_byte_size:  *mut u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;

    #if !TARGET_OS_IPHONE
        try {
    #endif
            // this is a processing method; no lock
            AudioUnitRenderActionFlags tempFlags;

            if (inTimeStamp == NULL || ioData == NULL)
                result = kAudio_ParamError;
            else {
                if (ioActionFlags == NULL) {
                    tempFlags = 0;
                    ioActionFlags = &tempFlags;
                }
                result = AUI->ComplexRender(*ioActionFlags, *inTimeStamp, inOutputBusNumber, inNumberOfPackets, outNumberOfPackets, outPacketDescriptions, *ioData, outMetadata, outMetadataByteSize);
            }

    #if !TARGET_OS_IPHONE
        }
        COMPONENT_CATCH
    #endif

        return result;
        */
}

pub fn au_method_reset(
        self_: *mut c_void,
        scope: AudioUnitScope,
        elem:  AudioUnitElement) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->Reset(scope, elem);
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_process(
        self_:            *mut c_void,
        io_action_flags:  *mut AudioUnitRenderActionFlags,
        in_time_stamp:    *const AudioTimeStamp,
        in_number_frames: u32,
        io_data:          *mut AudioBufferList) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;

    #if !TARGET_OS_IPHONE
        try {
    #endif
            // this is a processing method; no lock
            bool doParamCheck = true;

            AudioUnitRenderActionFlags tempFlags;

            if (ioActionFlags == NULL) {
                tempFlags = 0;
                ioActionFlags = &tempFlags;
            } else {
                if (*ioActionFlags & (1 << 9)/*kAudioUnitRenderAction_DoNotCheckRenderArgs*/)
                    doParamCheck = false;
            }

            if (doParamCheck && (inTimeStamp == NULL || ioData == NULL))
                result = kAudio_ParamError;
            else {
                result = AUI->DoProcess(*ioActionFlags, *inTimeStamp, inNumberFrames, *ioData);
            }

    #if !TARGET_OS_IPHONE
        }
        COMPONENT_CATCH
    #endif

        return result;
        */
}

pub fn au_method_process_multiple(
        self_:                         *mut c_void,
        io_action_flags:               *mut AudioUnitRenderActionFlags,
        in_time_stamp:                 *const AudioTimeStamp,
        in_number_frames:              u32,
        in_number_input_buffer_lists:  u32,
        in_input_buffer_lists:         *const *const AudioBufferList,
        in_number_output_buffer_lists: u32,
        io_output_buffer_lists:        *mut *mut AudioBufferList) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;

    #if !TARGET_OS_IPHONE
        try {
    #endif
            // this is a processing method; no lock
            bool doParamCheck = true;

            AudioUnitRenderActionFlags tempFlags;

            if (ioActionFlags == NULL) {
                tempFlags = 0;
                ioActionFlags = &tempFlags;
            } else {
                if (*ioActionFlags & (1 << 9)/*kAudioUnitRenderAction_DoNotCheckRenderArgs*/)
                    doParamCheck = false;
            }

            if (doParamCheck && (inTimeStamp == NULL || inInputBufferLists == NULL || ioOutputBufferLists == NULL))
                result = kAudio_ParamError;
            else {
                result = AUI->DoProcessMultiple(*ioActionFlags, *inTimeStamp, inNumberFrames, inNumberInputBufferLists, inInputBufferLists, inNumberOutputBufferLists, ioOutputBufferLists);
            }

    #if !TARGET_OS_IPHONE
        }
        COMPONENT_CATCH
    #endif

        return result;
        */
}

pub fn au_method_start(self_: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->Start();
        }
        COMPONENT_CATCH
        return result;
        */
}

pub fn au_method_stop(self_: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            AUI_LOCK
            result = AUI->Stop();
        }
        COMPONENT_CATCH
        return result;
        */
}

/**
   | I don't know what I'm doing here; conflicts
   | with the multiple inheritence in
   | MusicDeviceBase.
  */
#[cfg(not(CA_BASIC_AU_FEATURES))]
pub fn au_method_midi_event(
        self_:                  *mut c_void,
        in_status:              u32,
        in_data1:               u32,
        in_data2:               u32,
        in_offset_sample_frame: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            result = AUI->MIDIEvent(inStatus, inData1, inData2, inOffsetSampleFrame);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub fn au_method_sys_ex(
        self_:     *mut c_void,
        in_data:   *const UInt8,
        in_length: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            result = AUI->SysEx(inData, inLength);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub fn au_method_start_note(
        self_:                  *mut c_void,
        in_instrument:          MusicDeviceInstrumentID,
        in_groupid:             MusicDeviceGroupID,
        out_note_instanceid:    *mut NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params:              *const MusicDeviceNoteParams) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            if (inParams == NULL || outNoteInstanceID == NULL)
                result = kAudio_ParamError;
            else
                result = AUI->StartNote(inInstrument, inGroupID, outNoteInstanceID, inOffsetSampleFrame, *inParams);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub fn au_method_stop_note(
        self_:                  *mut c_void,
        in_groupid:             MusicDeviceGroupID,
        in_note_instanceid:     NoteInstanceID,
        in_offset_sample_frame: u32) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            result = AUI->StopNote(inGroupID, inNoteInstanceID, inOffsetSampleFrame);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
#[cfg(not(TARGET_OS_IPHONE))]
pub fn au_method_prepare_instrument(
        self_:         *mut c_void,
        in_instrument: MusicDeviceInstrumentID) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            result = AUI->PrepareInstrument(inInstrument);
        }
        COMPONENT_CATCH
        return result;
        */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
#[cfg(not(TARGET_OS_IPHONE))]
pub fn au_method_release_instrument(
        self_:         *mut c_void,
        in_instrument: MusicDeviceInstrumentID) -> OSStatus {
    
    todo!();
        /*
            OSStatus result = noErr;
        try {
            // this is a potential render-time method; no lock
            result = AUI->ReleaseInstrument(inInstrument);
        }
        COMPONENT_CATCH
        return result;
        */
}
