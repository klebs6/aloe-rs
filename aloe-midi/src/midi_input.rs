crate::ix!();

/**
  | Represents a midi input device.
  | 
  | To create one of these, use the static
  | getAvailableDevices() method to find
  | out what inputs are available, and then
  | use the openDevice() method to try to
  | open one.
  | 
  | @see MidiOutput
  | 
  | @tags{Audio}
  |
  */
#[leak_detector]
#[no_copy]
pub struct MidiInput {
    device_info: MidiDeviceInfo,
    internal:    Box<MidiInputPimpl>,
}

impl MidiInput {

    #[cfg(target_os="android")]
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        AndroidMidiDeviceManager manager;
        return manager.getDevices (true);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        return getAvailableDevices().getFirst();
        */
    }
    
    #[cfg(target_os="android")]
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
    
    #[cfg(target_os="android")]
    pub fn get_devices(&mut self) -> StringArray {
        
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
    
    #[cfg(target_os="android")]
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return (getAndroidSDKVersion() < 23 ? -1 : 0);
        */
    }
    
    #[cfg(target_os="android")]
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
    
    #[cfg(target_os="android")]
    pub fn new(
        device_name:       &String,
        device_identifier: &String) -> Self {
    
        todo!();
        /*


            : deviceInfo (deviceName, deviceIdentifier)
        */
    }
    
    #[cfg(target_os="android")]
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if (auto* mi = internal.get())
            mi->start();
        */
    }
    
    #[cfg(target_os="android")]
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (auto* mi = internal.get())
            mi->stop();
        */
    }

    /**
      | Returns a list of the available midi
      | input devices.
      | 
      | You can open one of the devices by passing
      | its identifier into the openDevice()
      | method.
      | 
      | @see MidiDeviceInfo, getDevices,
      | getDefaultDeviceIndex, openDevice
      |
      */
    pub fn get_available_devices() -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the MidiDeviceInfo of the default
      | midi input device to use.
      |
      */
    pub fn get_default_device() -> MidiDeviceInfo {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to open one of the midi input devices.
      | 
      | This will return a MidiInput object
      | if it manages to open it, you can then
      | call start() and stop() on this device.
      | 
      | If the device can't be opened, this will
      | return an empty object.
      | 
      | -----------
      | @param deviceIdentifier
      | 
      | the ID of the device to open - use the getAvailableDevices()
      | method to find the available devices
      | that can be opened
      | ----------
      | @param callback
      | 
      | the object that will receive the midi
      | messages from this device
      | 
      | @see MidiInputCallback, getDevices
      |
      */
    pub fn open_device(
        device_identifier: &String,
        callback:          *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
        
        */
    }

    /**
      | This will try to create a new midi input
      | device (only available on Linux, macOS
      | and iOS).
      | 
      | This will attempt to create a new midi
      | input device with the specified name
      | for other apps to connect to.
      | 
      | NB - if you are calling this method on
      | iOS you must have enabled the "Audio
      | Background Capability" setting in
      | the iOS exporter otherwise this method
      | will fail.
      | 
      | Returns an empty object if a device can't
      | be created.
      | 
      | -----------
      | @param deviceName
      | 
      | the name of the device to create
      | ----------
      | @param callback
      | 
      | the object that will receive the midi
      | messages from this device
      |
      */
    #[cfg(any(target_os="linux",target_os="bsd",target_os="macos",target_os="ios"))]
    pub fn create_new_device(
        device_name: &String,
        callback:    *mut dyn MidiInputCallback) -> Box<MidiInput> {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Starts the device running.
      | 
      | After calling this, the device will
      | start sending midi messages to the MidiInputCallback
      | object that was specified when the openDevice()
      | method was called.
      | 
      | @see stop
      |
      */
    pub fn start(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Stops the device running.
      | 
      | @see start
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the MidiDeviceInfo struct
      | containing some information about
      | this device.
      |
      */
    pub fn get_device_info(&self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return deviceInfo;
        */
    }

    /**
      | Returns the identifier of this device.
      |
      */
    pub fn get_identifier(&self) -> String {
        
        todo!();
        /*
            return deviceInfo.identifier;
        */
    }

    /**
      | Returns the name of this device.
      |
      */
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return deviceInfo.name;
        */
    }

    /**
      | Sets a custom name for the device.
      |
      */
    pub fn set_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            deviceInfo.name = newName;
        */
    }

    #[cfg(feature = "linux-bela")]
    pub fn new(
        device_name: &String,
        deviceid:    &String) -> Self {
    
        todo!();
        /*


            : deviceInfo (deviceName, deviceID)
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn start(&mut self)  {
        
        todo!();
        /*
            internal->start();
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            internal->stop();
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            return Pimpl::getDevices (true);
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return getAvailableDevices().getFirst();
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn open_device(
        &mut self, 
        device_identifier: &String,
        callback:          *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            if (deviceIdentifier.isEmpty())
            return {};

        std::unique_ptr<MidiInput> midiInput (new MidiInput (deviceIdentifier, deviceIdentifier));
        midiInput->internal = std::make_unique<Pimpl> (deviceIdentifier, midiInput.get(), callback);

        return midiInput;
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn create_new_device(
        &mut self, 
        _0: &String,
        _1: *mut dyn MidiInputCallback

    ) -> Box<MidiInput> {
        
        todo!();
        /*
            // N/A on Bela
        jassertfalse;
        return {};
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn get_devices(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> deviceNames;

        for (auto& d : getAvailableDevices())
            deviceNames.add (d.name);

        return deviceNames;
        */
    }
    
    #[cfg(feature = "linux-bela")]
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(feature = "linux-bela")]
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
}
