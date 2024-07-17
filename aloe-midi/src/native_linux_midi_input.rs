crate::ix!();

#[cfg(target_os="linux")]
#[cfg(not(feature = "alsa"))]
pub struct MidiInputPimpl {

}

#[cfg(target_os="linux")]
#[cfg(not(feature = "alsa"))]
impl MidiInput {
    
    /**
       (These are just stub functions if ALSA is
       unavailable...)
      */
    pub fn new(
        device_name: &String,
        deviceid:    &String) -> Self {
    
        todo!();
        /*
        : device_info(deviceName, deviceID),

        
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn open_device(
        &mut self, 
        _0: &String,
        _1: *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn create_new_device(
        &mut self, 
        _0: &String,
        _1: *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_devices(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn open_device(
        &mut self, 
        _0: i32,
        _1: *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            return {};
        */
    }
}

///---------------------
#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct MidiInputPimpl {
    base: AlsaPortPtr,
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Drop for MidiInput {
    fn drop(&mut self) {
        todo!();
        /* 
        stop();
 */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl MidiInput {
    
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            Vec<MidiDeviceInfo> devices;
        iterateMidiDevices (true, devices, {});

        return devices;
        */
    }
    
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return getAvailableDevices().getFirst();
        */
    }
    
    pub fn open_device(&mut self, 
        device_identifier: &String,
        callback:          *mut MidiInputCallback) -> Box<MidiInput> {
        
        todo!();
        /*
            if (deviceIdentifier.isEmpty())
            return {};

        Vec<MidiDeviceInfo> devices;
        auto* port = iterateMidiDevices (true, devices, deviceIdentifier);

        if (port == nullptr || ! port->isValid())
            return {};

        jassert (port->isValid());

        std::unique_ptr<MidiInput> midiInput (new MidiInput (port->getPortName(), deviceIdentifier));

        port->setupInput (midiInput.get(), callback);
        midiInput->internal = std::make_unique<MidiInputPimpl> (port);

        return midiInput;
        */
    }
    
    pub fn create_new_device(&mut self, 
        device_name: &String,
        callback:    *mut MidiInputCallback) -> Box<MidiInput> {
        
        todo!();
        /*
            auto client = AlsaClient::getInstance();
        auto* port = client->createPort (deviceName, true, true);

        if (port == nullptr || ! port->isValid())
            return {};

        std::unique_ptr<MidiInput> midiInput (new MidiInput (deviceName, getFormattedPortIdentifier (client->getId(), port->getPortId())));

        port->setupInput (midiInput.get(), callback);
        midiInput->internal = std::make_unique<MidiInputPimpl> (port);

        return midiInput;
        */
    }
    
    pub fn get_devices(&mut self) -> StringArray {
        
        todo!();
        /*
            StringArray deviceNames;

        for (auto& d : getAvailableDevices())
            deviceNames.add (d.name);

        deviceNames.appendNumbersToDuplicates (true, true);

        return deviceNames;
        */
    }
    
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn open_device(&mut self, 
        index:    i32,
        callback: *mut MidiInputCallback) -> Box<MidiInput> {
        
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
        : device_info(deviceName, deviceIdentifier),

        
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            internal->ptr->enableCallback (true);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            internal->ptr->enableCallback (false);
        */
    }
}
