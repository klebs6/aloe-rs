crate::ix!();

impl AUMidiBaseInterface for AUMIDIBase {}

pub trait AUMidiBaseInterface {

    fn midi_event(&mut self, 
        in_status:              u32,
        in_data1:               u32,
        in_data2:               u32,
        in_offset_sample_frame: u32) -> OSStatus {
        
        todo!();
        /*
            UInt32 strippedStatus = inStatus & 0xf0;
            UInt32 channel = inStatus & 0x0f;

            return HandleMidiEvent(strippedStatus, channel, inData1, inData2, inOffsetSampleFrame);
        */
    }
    
    fn sys_ex(&mut self, 
        in_data:   *const u8,
        in_length: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    #[cfg(TARGET_API_MAC_OSX)]
    fn delegate_get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(TARGET_API_MAC_OSX)]
    fn delegate_get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(TARGET_API_MAC_OSX)]
    fn delegate_set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    /**
       MIDI dispatch
      */
    fn handle_midi_event(&mut self, 
        in_status:      u8,
        in_channel:     u8,
        in_data1:       u8,
        in_data2:       u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn handle_non_note_event(&mut self, 
        status:         u8,
        channel:        u8,
        data1:          u8,
        data2:          u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    /**
       if not overridden, it's unsupported
      */
    #[cfg(TARGET_API_MAC_OSX)]
    fn get_xml_names(&mut self, out_name_document: *mut CFURLRef) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }

    /* ---------------  channel messages  --------------- */
    
    fn handle_note_on(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_note_off(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_control_change(&mut self, 
        in_channel:     u8,
        in_controller:  u8,
        in_value:       u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_pitch_wheel(&mut self, 
        in_channel:     u8,
        in_pitch1:      u8,
        in_pitch2:      u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_channel_pressure(&mut self, 
        in_channel:     u8,
        in_value:       u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_program_change(&mut self, 
        in_channel: u8,
        in_value:   u8) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_poly_pressure(&mut self, 
        in_channel:     u8,
        in_key:         u8,
        in_value:       u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_reset_all_controllers(&mut self, in_channel: u8) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_all_notes_off(&mut self, in_channel: u8) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    fn handle_all_sound_off(&mut self, in_channel: u8) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }

    /**
      System messages
      */
    fn handle_sys_ex(&mut self, 
        in_data:   *const u8,
        in_length: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }

    /**
      | map manager
      |
      */
    #[cfg(CA_AUTO_MIDI_MAP)]
    fn get_midi_map_manager(&mut self) -> *mut CAAUMIDIMapManager {
        
        todo!();
        /*
            return mMapManager;}{
        */
    }

    /**
       component dispatcher
      */
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    fn component_entry_dispatch(
        params: *mut ComponentParameters,
        this:   *mut AUMIDIBase) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
