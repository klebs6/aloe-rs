crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioIODeviceType.h]

/**
  | Available modes for the WASAPI audio
  | device.
  | 
  | Pass one of these to the AudioIODeviceType::createAudioIODeviceType_WASAPI()
  | method to create a WASAPI AudioIODeviceType
  | object in this mode.
  |
  */
pub enum WASAPIDeviceMode
{
    shared,
    exclusive,
    sharedLowLatency
}

/**
  | Represents a type of audio driver, such as
  | DirectSound, ASIO, CoreAudio, etc.
  |
  | To get a list of available audio driver types,
  | use the
  | AudioDeviceManager::createAudioDeviceTypes()
  | method. Each of the objects returned can then
  | be used to list the available devices of that
  | type. E.g.
  |
  | @code
  | Vec<Box<AudioIODeviceType>> types;
  | myAudioDeviceManager.createAudioDeviceTypes (types);
  |
  | for (int i = 0; i < types.size(); ++i)
  | {
  |     // This will be things like "DirectSound", "CoreAudio", etc.
  |     String typeName (types[i]->getTypeName());  
  |
  |     // This must be called before getting the list of devices
  |     types[i]->scanForDevices();                 
  |
  |     // This will now return a list of available devices of this type
  |     Vec<String> deviceNames (types[i]->getDeviceNames());  
  |
  |     for (int j = 0; j < deviceNames.size(); ++j)
  |     {
  |         AudioIODevice* device = types[i]->createDevice (deviceNames [j]);
  |
  |         ...
  |     }
  | }
  | @endcode
  |
  | For an easier way of managing audio devices
  | and their settings, have a look at the
  | AudioDeviceManager class.
  |
  | @see AudioIODevice, AudioDeviceManager
  |
  | @tags{Audio}
  */
#[no_copy]
pub struct AudioIODeviceType {
    type_name: String,
    listeners: ListenerList<Rc<RefCell<dyn AudioIODeviceTypeListener>>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioIODeviceType.cpp]
impl AudioIODeviceType {

    /**
      | Returns the name of this type of driver
      | that this object manages.
      | 
      | This will be something like "DirectSound",
      | "ASIO", "CoreAudio", "ALSA", etc.
      |
      */
    pub fn get_type_name(&self) -> &String {
        
        todo!();
        /*
            return typeName;
        */
    }

    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : type_name(name),
        */
    }
    
    /**
      | Adds a listener that will be called when
      | this type of device is added or removed
      | from the system.
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn AudioIODeviceTypeListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Removes a listener that was previously
      | added with addListener().
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn AudioIODeviceTypeListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    /**
      | Synchronously calls all the registered
      | device list change listeners.
      |
      */
    pub fn call_device_change_listeners(&mut self)  {
        
        todo!();
        /*
            listeners.call ([] (AudioIODeviceTypeListener& l) { l.audioDeviceListChanged(); });
        */
    }

    /**
      | Creates a CoreAudio device type if it's
      | available on this platform, or returns
      | null.
      |
      */
    #[cfg(target_os="macos")]
    pub fn create_audio_io_device_type_core_audio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new CoreAudioClasses::CoreAudioIODeviceType();
        */
    }

    #[cfg(not(target_os="macos"))]
    pub fn create_audio_io_device_type_core_audio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an iOS device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(target_os="ios")]
    pub fn create_audio_io_device_type_i_os_audio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new iOSAudioIODeviceType();
        */
    }

    #[cfg(not(target_os="ios"))]
    pub fn create_audio_io_device_type_i_os_audio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    /**
      | Creates a WASAPI device type in the specified
      | mode if it's available on this platform,
      | or returns null.
      |
      */
    #[cfg(all(target_os="windows",ALOE_WASAPI))]
    pub fn create_audio_io_device_type_wasapi_with_mode(&mut self, device_mode: WASAPIDeviceMode) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            auto windowsVersion = SystemStats::getOperatingSystemType();

         if (windowsVersion < SystemStats::WinVista
             || (WasapiClasses::isLowLatencyMode (deviceMode) && windowsVersion < SystemStats::Windows10))
             return nullptr;

         return new WasapiClasses::WASAPIAudioIODeviceType (deviceMode);
        */
    }
    
    #[cfg(all(target_os="windows",ALOE_WASAPI))]
    pub fn create_audio_io_device_type_wasapi_with_exclusive_flag(&mut self, exclusive_mode: bool) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return createAudioIODeviceType_WASAPI (exclusiveMode ? WASAPIDeviceMode::exclusive
                                                              : WASAPIDeviceMode::shared);
        */
    }

    #[cfg(not(all(target_os="windows",ALOE_WASAPI)))]
    pub fn create_audio_io_device_type_wasapi_with_mode(&mut self, _0: WASAPIDeviceMode) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    #[cfg(not(all(target_os="windows",ALOE_WASAPI)))]
    pub fn create_audio_io_device_type_wasapi_with_exclusive_flag(&mut self, _0: bool) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates a DirectSound device type if
      | it's available on this platform, or
      | returns null.
      |
      */
    #[cfg(all(target_os="windows",ALOE_DIRECTSOUND))]
    pub fn create_audio_io_device_type_direct_sound(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new DSoundAudioIODeviceType();
        */
    }

    #[cfg(not(all(target_os="windows",ALOE_DIRECTSOUND)))]
    pub fn create_audio_io_device_type_direct_sound(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an ASIO device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(all(target_os="windows",ALOE_ASIO))]
    pub fn create_audio_io_device_type_asio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new ASIOAudioIODeviceType();
        */
    }

    #[cfg(not(all(target_os="windows",ALOE_ASIO)))]
    pub fn create_audio_io_device_type_asio(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an ALSA device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(all(any(target_os="linux",target_os="bsd"),ALOE_ALSA))]
    pub fn create_audio_io_device_type_alsa(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return createAudioIODeviceType_ALSA_PCMDevices();
        */
    }

    #[cfg(not(all(any(target_os="linux",target_os="bsd"),ALOE_ALSA)))]
    pub fn create_audio_io_device_type_alsa(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates a JACK device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(all(any(target_os="linux",target_os="bsd"),ALOE_JACK))]
    pub fn create_audio_io_device_type_jack(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new JackAudioIODeviceType();
        */
    }

    #[cfg(not(all(any(target_os="linux",target_os="bsd"),ALOE_JACK)))]
    pub fn create_audio_io_device_type_jack(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates a Bela device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(all(target_os="linux",ALOE_BELA))]
    pub fn create_audio_io_device_type_bela(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return new BelaAudioIODeviceType();
        */
    }

    #[cfg(not(all(target_os="linux",ALOE_BELA)))]
    pub fn create_audio_io_device_type_bela(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an Android device type if it's
      | available on this platform, or returns
      | null.
      |
      */
    #[cfg(target_os="android")]
    pub fn create_audio_io_device_type_android(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            #if ALOE_USE_ANDROID_OBOE
         if (isOboeAvailable())
             return nullptr;
        #endif

        #if ALOE_USE_ANDROID_OPENSLES
         if (isOpenSLAvailable())
             return nullptr;
        #endif

         return new AndroidAudioIODeviceType();
        */
    }

    #[cfg(not(target_os="android"))]
    pub fn create_audio_io_device_type_android(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an Android OpenSLES device
      | type if it's available on this platform,
      | or returns null.
      |
      */
    #[cfg(all(target_os="android",ALOE_USE_ANDROID_OPENSLES))]
    pub fn create_audio_io_device_type_opensles(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return isOpenSLAvailable() ? new OpenSLAudioDeviceType() : nullptr;
        */
    }

    #[cfg(not(all(target_os="android",ALOE_USE_ANDROID_OPENSLES)))]
    pub fn create_audio_io_device_type_opensles(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Creates an Oboe device type if it's available
      | on this platform, or returns null.
      |
      */
    #[cfg(all(target_os="android",ALOE_USE_ANDROID_OBOE))]
    pub fn create_audio_io_device_type_oboe(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return isOboeAvailable() ? new OboeAudioIODeviceType() : nullptr;
        */
    }

    #[cfg(not(all(target_os="android",ALOE_USE_ANDROID_OBOE)))]
    pub fn create_audio_io_device_type_oboe(&mut self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            return nullptr;
        */
    }
}
