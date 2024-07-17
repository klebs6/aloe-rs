crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_linux_Midi.cpp]

#[cfg(target_os="linux")]
#[cfg(not(feature = "alsa"))]
pub struct MidiOutputPimpl {

}

#[cfg(target_os="linux")]
#[cfg(not(feature = "alsa"))]
impl MidiOutputPimpl {
    
    pub fn send_message_now(&mut self, _0: &MidiMessage)  {
        
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
    
    pub fn open_device(&mut self, _0: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn create_new_device(&mut self, _0: &String) -> Box<MidiOutput> {
        
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
    
    pub fn open_device(&mut self, _0: i32) -> Box<MidiOutput> {
        
        todo!();
        /*
            return {};
        */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct MidiOutputPimpl {
    base: AlsaPortPtr,
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Drop for MidiOutput {
    fn drop(&mut self) {
        todo!();
        /* 
        stopBackgroundThread();
 */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl MidiOutput {

    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            Vec<MidiDeviceInfo> devices;
        iterateMidiDevices (false, devices, {});

        return devices;
        */
    }
    
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return getAvailableDevices().getFirst();
        */
    }
    
    pub fn open_device(&mut self, device_identifier: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            if (deviceIdentifier.isEmpty())
            return {};

        Vec<MidiDeviceInfo> devices;
        auto* port = iterateMidiDevices (false, devices, deviceIdentifier);

        if (port == nullptr || ! port->isValid())
            return {};

        std::unique_ptr<MidiOutput> midiOutput (new MidiOutput (port->getPortName(), deviceIdentifier));

        port->setupOutput();
        midiOutput->internal = std::make_unique<MidiOutputPimpl> (port);

        return midiOutput;
        */
    }
    
    pub fn create_new_device(&mut self, device_name: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            auto client = AlsaClient::getInstance();
        auto* port = client->createPort (deviceName, false, true);

        if (port == nullptr || ! port->isValid())
            return {};

        std::unique_ptr<MidiOutput> midiOutput (new MidiOutput (deviceName, getFormattedPortIdentifier (client->getId(), port->getPortId())));

        port->setupOutput();
        midiOutput->internal = std::make_unique<MidiOutputPimpl> (port);

        return midiOutput;
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
    
    pub fn open_device(&mut self, index: i32) -> Box<MidiOutput> {
        
        todo!();
        /*
            return openDevice (getAvailableDevices()[index].identifier);
        */
    }
    
    pub fn send_message_now(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            internal->ptr->sendMessageNow (message);
        */
    }
}
