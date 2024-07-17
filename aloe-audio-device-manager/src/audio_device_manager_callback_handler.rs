crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AudioDeviceManagerCallbackHandler<'a> {
    owner: &'a mut AudioDeviceManager<'a>,
}

impl<'a> AudioIODeviceTypeListener for AudioDeviceManagerCallbackHandler<'a> {

    fn audio_device_list_changed(&mut self)  {
        
        todo!();
        /*
            owner.audioDeviceListChanged();
        */
    }
}

impl<'a> MidiInputCallback for AudioDeviceManagerCallbackHandler<'a> {

    fn handle_incoming_midi_message(&mut self, 
        source:  *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            owner.handleIncomingMidiMessageInt (source, message);
        */
    }
}

impl<'a> AudioIODeviceCallback for AudioDeviceManagerCallbackHandler<'a> {

    fn audio_device_io_callback(&mut self, 
        ins:         *const *const f32,
        num_ins:     i32,
        outs:        *mut *mut f32,
        num_outs:    i32,
        num_samples: i32)  {
        
        todo!();
        /*
            owner.audioDeviceIOCallbackInt (ins, numIns, outs, numOuts, numSamples);
        */
    }

    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            owner.audioDeviceAboutToStartInt (device);
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            owner.audioDeviceStoppedInt();
        */
    }
}

impl<'a> AudioDeviceManagerCallbackHandler<'a> {

    pub fn new(adm: &mut AudioDeviceManager) -> Self {
    
        todo!();
        /*
        : owner(adm),
        */
    }
    
    pub fn audio_device_error(&mut self, message: &String)  {
        
        todo!();
        /*
            owner.audioDeviceErrorInt (message);
        */
    }
}
