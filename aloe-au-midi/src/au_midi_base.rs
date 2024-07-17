crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUMIDIBase.h]

pub type MIDIPacketList = MissingType;//TODO

pub struct AUMIDIBase {

    au_base_instance: AUBase,

    /**
      | map manager
      |
      */
    #[cfg(CA_AUTO_MIDI_MAP)]
    map_manager: *mut CAAUMIDIMapManager,
}

impl Drop for AUMIDIBase {

    fn drop(&mut self) {
        todo!();
        /*
            #if CA_AUTO_MIDI_MAP
        if (mMapManager)
            delete mMapManager;
    #endif
        */
    }
}

impl AUMIDIBase {
    
    pub fn new(in_base: *mut AUBase) -> Self {
    
        todo!();
        /*


            : mAUBaseInstance (*inBase)

    #if CA_AUTO_MIDI_MAP
        mMapManager = new CAAUMIDIMapManager();
    #endif
        */
    }
    
    #[cfg(TARGET_API_MAC_OSX)]
    pub fn delegate_get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

        switch (inID) {
    #if !TARGET_OS_IPHONE
        case kMusicDeviceProperty_MIDIXMLNames:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            if (GetXMLNames(NULL) == noErr) {
                outDataSize = sizeof(CFURLRef);
                outWritable = false;
            } else
                result = kAudioUnitErr_InvalidProperty;
            break;
    #endif
    #if CA_AUTO_MIDI_MAP
        case kAudioUnitProperty_AllParameterMIDIMappings:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            outWritable = true;
            outDataSize = sizeof (AUParameterMIDIMapping)*mMapManager->NumMaps();
            result = noErr;
            break;

        case kAudioUnitProperty_HotMapParameterMIDIMapping:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            outWritable = true;
            outDataSize = sizeof (AUParameterMIDIMapping);
            result = noErr;
            break;

        case kAudioUnitProperty_AddParameterMIDIMapping:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            outWritable = true;
            outDataSize = sizeof (AUParameterMIDIMapping);
            result = noErr;
            break;

        case kAudioUnitProperty_RemoveParameterMIDIMapping:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            outWritable = true;
            outDataSize = sizeof (AUParameterMIDIMapping);
            result = noErr;
            break;
    #endif

        default:
            result = kAudioUnitErr_InvalidProperty;
            break;
        }
        return result;

    #if CA_AUTO_MIDI_MAP || (!TARGET_OS_IPHONE)
    InvalidScope:
        return kAudioUnitErr_InvalidScope;
    InvalidElement:
        return kAudioUnitErr_InvalidElement;
    #endif
        */
    }
    
    #[cfg(TARGET_API_MAC_OSX)]
    pub fn delegate_get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
            OSStatus result;

        switch (inID) {
    #if !TARGET_OS_IPHONE
        case kMusicDeviceProperty_MIDIXMLNames:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            result = GetXMLNames((CFURLRef *)outData);
            break;
    #endif
    #if CA_AUTO_MIDI_MAP
        case kAudioUnitProperty_AllParameterMIDIMappings:{
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            AUParameterMIDIMapping*  maps =  (static_cast<AUParameterMIDIMapping*>(outData));
            mMapManager->GetMaps(maps);
    //      printf ("GETTING MAPS\n");
    //      mMapManager->Print();
            result = noErr;
            break;
        }

        case kAudioUnitProperty_HotMapParameterMIDIMapping:{
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inElement == 0, InvalidElement);
            AUParameterMIDIMapping *  map =  (static_cast<AUParameterMIDIMapping*>(outData));
            mMapManager->GetHotParameterMap (*map);
            result = noErr;
            break;
        }
    #endif

        default:
            result = kAudioUnitErr_InvalidProperty;
            break;
        }
        return result;

    #if CA_AUTO_MIDI_MAP || (!TARGET_OS_IPHONE)
    InvalidScope:
        return kAudioUnitErr_InvalidScope;
    InvalidElement:
        return kAudioUnitErr_InvalidElement;
    #endif
        */
    }
    
    #[cfg(TARGET_API_MAC_OSX)]
    pub fn delegate_set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
            OSStatus result;

        switch (inID) {
    #if CA_AUTO_MIDI_MAP
            case kAudioUnitProperty_AddParameterMIDIMapping:{
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                ca_require(inElement == 0, InvalidElement);
                AUParameterMIDIMapping * maps = (AUParameterMIDIMapping*)inData;
                mMapManager->SortedInsertToParamaterMaps (maps, (inDataSize / sizeof(AUParameterMIDIMapping)), mAUBaseInstance);
                mAUBaseInstance.PropertyChanged (kAudioUnitProperty_AllParameterMIDIMappings, kAudioUnitScope_Global, 0);
                result = noErr;
                break;
            }

            case kAudioUnitProperty_RemoveParameterMIDIMapping:{
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                ca_require(inElement == 0, InvalidElement);
                AUParameterMIDIMapping * maps = (AUParameterMIDIMapping*)inData;
                bool didChange;
                mMapManager->SortedRemoveFromParameterMaps(maps, (inDataSize / sizeof(AUParameterMIDIMapping)), didChange);
                if (didChange)
                    mAUBaseInstance.PropertyChanged (kAudioUnitProperty_AllParameterMIDIMappings, kAudioUnitScope_Global, 0);
                result = noErr;
                break;
            }

            case kAudioUnitProperty_HotMapParameterMIDIMapping:{
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                ca_require(inElement == 0, InvalidElement);
                AUParameterMIDIMapping & map = *((AUParameterMIDIMapping*)inData);
                mMapManager->SetHotMapping (map);
                result = noErr;
                break;
            }
            case kAudioUnitProperty_AllParameterMIDIMappings:{
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                ca_require(inElement == 0, InvalidElement);
                AUParameterMIDIMapping * mappings = (AUParameterMIDIMapping*)inData;
                mMapManager->ReplaceAllMaps (mappings, (inDataSize / sizeof(AUParameterMIDIMapping)), mAUBaseInstance);
                result = noErr;
                break;
            }
    #endif

        default:
            result = kAudioUnitErr_InvalidProperty;
            break;
        }
        return result;
    #if CA_AUTO_MIDI_MAP
        InvalidScope:
            return kAudioUnitErr_InvalidScope;
        InvalidElement:
            return kAudioUnitErr_InvalidElement;
    #endif
        */
    }
    
    pub fn handle_midi_packet_list(&mut self, pktlist: *const MIDIPacketList) -> OSStatus {
        
        todo!();
        /*
            if (!mAUBaseInstance.IsInitialized()) return kAudioUnitErr_Uninitialized;

        int nPackets = pktlist->numPackets;
        const MIDIPacket *pkt = pktlist->packet;

        while (nPackets-- > 0) {
            const Byte *event = pkt->data, *packetEnd = event + pkt->length;
            long startFrame = (long)pkt->timeStamp;
            while (event < packetEnd) {
                Byte status = event[0];
                if (status & 0x80) {
                    // really a status byte (not sysex continuation)
                    HandleMidiEvent(status & 0xF0, status & 0x0F, event[1], event[2], static_cast<UInt32>(startFrame));
                        // note that we're generating a bogus channel number for system messages (0xF0-FF)
                }
                event = NextMIDIEvent(event, packetEnd);
            }
            pkt = reinterpret_cast<const MIDIPacket *>(packetEnd);
        }
        return noErr;
        */
    }
    
    pub fn handle_midi_event(&mut self, 
        status:         u8,
        channel:        u8,
        data1:          u8,
        data2:          u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            if (!mAUBaseInstance.IsInitialized()) return kAudioUnitErr_Uninitialized;

    #if CA_AUTO_MIDI_MAP
    // you potentially have a choice to make here - if a param mapping matches, do you still want to process the
    // MIDI event or not. The default behaviour is to continue on with the MIDI event.
        if (mMapManager->HandleHotMapping (status, channel, data1, mAUBaseInstance)) {
            mAUBaseInstance.PropertyChanged (kAudioUnitProperty_HotMapParameterMIDIMapping, kAudioUnitScope_Global, 0);
        }
        else {
            mMapManager->FindParameterMapEventMatch(status, channel, data1, data2, inStartFrame, mAUBaseInstance);
        }
    #endif

        OSStatus result = noErr;

        switch(status)
        {
            case kMidiMessage_NoteOn:
                if(data2)
                {
                    result = HandleNoteOn(channel, data1, data2, inStartFrame);
                }
                else
                {
                    // zero velocity translates to note off
                    result = HandleNoteOff(channel, data1, data2, inStartFrame);
                }
                break;

            case kMidiMessage_NoteOff:
                result = HandleNoteOff(channel, data1, data2, inStartFrame);
                break;

            default:
                result = HandleNonNoteEvent (status, channel, data1, data2, inStartFrame);
                break;
        }

        return result;
        */
    }
    
    pub fn handle_non_note_event(&mut self, 
        status:         u8,
        channel:        u8,
        data1:          u8,
        data2:          u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

        switch (status)
        {
            case kMidiMessage_PitchWheel:
                result = HandlePitchWheel(channel, data1, data2, inStartFrame);
                break;

            case kMidiMessage_ProgramChange:
                result = HandleProgramChange(channel, data1);
                break;

            case kMidiMessage_ChannelPressure:
                result = HandleChannelPressure(channel, data1, inStartFrame);
                break;

            case kMidiMessage_ControlChange:
            {
                switch (data1) {
                    case kMidiController_AllNotesOff:
                        result = HandleAllNotesOff(channel);
                        break;

                    case kMidiController_ResetAllControllers:
                        result = HandleResetAllControllers(channel);
                        break;

                    case kMidiController_AllSoundOff:
                        result = HandleAllSoundOff(channel);
                        break;

                    default:
                        result = HandleControlChange(channel, data1, data2, inStartFrame);
                        break;
                }
                break;
            }
            case kMidiMessage_PolyPressure:
                result = HandlePolyPressure (channel, data1, data2, inStartFrame);
                break;
        }
        return result;
        */
    }
    
    pub fn sys_ex(&mut self, 
        in_data:   *const u8,
        in_length: u32) -> OSStatus {
        
        todo!();
        /*
            if (!mAUBaseInstance.IsInitialized()) return kAudioUnitErr_Uninitialized;

        return HandleSysEx(inData, inLength );
        */
    }

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn component_entry_dispatch(&mut self, 
        params: *mut ComponentParameters,
        this:   *mut AUMIDIBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return kAudio_ParamError;

        OSStatus result;

        switch (params->what) {
        case kMusicDeviceMIDIEventSelect:
            {
                PARAM(UInt32, pbinStatus, 0, 4);
                PARAM(UInt32, pbinData1, 1, 4);
                PARAM(UInt32, pbinData2, 2, 4);
                PARAM(UInt32, pbinOffsetSampleFrame, 3, 4);
                result = This->MIDIEvent(pbinStatus, pbinData1, pbinData2, pbinOffsetSampleFrame);
            }
            break;
        case kMusicDeviceSysExSelect:
            {
                PARAM(const u8 *, pbinData, 0, 2);
                PARAM(UInt32, pbinLength, 1, 2);
                result = This->SysEx(pbinData, pbinLength);
            }
            break;

        default:
            result = badComponentSelector;
            break;
        }

        return result;
        */
    }
}
