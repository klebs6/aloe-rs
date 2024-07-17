crate::ix!();

impl MusicDeviceBaseInterface for MusicDeviceBase {}

pub trait MusicDeviceBaseInterface {

    fn midi_event(&mut self, 
        in_status:              u32,
        in_data1:               u32,
        in_data2:               u32,
        in_offset_sample_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return AUMIDIBase::MIDIEvent (inStatus, inData1, inData2, inOffsetSampleFrame);
        */
    }
    
    fn sys_ex(&mut self, 
        in_data:   *const u8,
        in_length: u32) -> OSStatus {
        
        todo!();
        /*
            return AUMIDIBase::SysEx (inData, inLength);
        */
    }
    
    fn get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn handle_note_on(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn handle_note_off(&mut self, 
        in_channel:     u8,
        in_note_number: u8,
        in_velocity:    u8,
        in_start_frame: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn get_instrument_count(&self, out_inst_count: &mut u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
