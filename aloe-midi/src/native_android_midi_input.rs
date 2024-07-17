crate::ix!();

impl MidiInput {

    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        AndroidMidiDeviceManager manager;
        return manager.getDevices (true);
        */
    }
    
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        return getAvailableDevices().getFirst();
        */
    }
    
    pub fn open_device(
        &mut self, 
        device_identifier: &String,
        callback:          *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23 || deviceIdentifier.isEmpty())
            return {};

        AndroidMidiDeviceManager manager;

        std::unique_ptr<MidiInput> midiInput (new MidiInput ({}, deviceIdentifier));

        if (auto* port = manager.openMidiInputPortWithID (deviceIdentifier.getIntValue(), midiInput.get(), callback))
        {
            midiInput->internal.reset (port);
            midiInput->setName (port->getName());

            return midiInput;
        }

        return {};
        */
    }
    
    pub fn get_devices(&mut self) -> Vec<String> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        StringArray deviceNames;

        for (auto& d : getAvailableDevices())
            deviceNames.add (d.name);

        return deviceNames;
        */
    }
    
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return (getAndroidSDKVersion() < 23 ? -1 : 0);
        */
    }
    
    pub fn open_device(
        &mut self, 
        index:    i32,
        callback: *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            return openDevice (getAvailableDevices()[index].identifier, callback);
        */
    }
    
    pub fn new(
        device_name:       &String,
        device_identifier: &String) -> Self {
    
        todo!();
        /*


            : deviceInfo (deviceName, deviceIdentifier)
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if (auto* mi = internal.get())
            mi->start();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (auto* mi = internal.get())
            mi->stop();
        */
    }
}

