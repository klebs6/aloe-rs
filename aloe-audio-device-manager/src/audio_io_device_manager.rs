crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioDeviceManager.h]

/**
  | Manages the state of some audio and midi
  | i/o devices.
  | 
  | This class keeps tracks of a currently-selected
  | audio device, through with which it
  | continuously streams data from an audio
  | callback, as well as one or more midi
  | inputs.
  | 
  | The idea is that your application will
  | create one global instance of this object,
  | and let it take care of creating and deleting
  | specific types of audio devices internally.
  | So when the device is changed, your callbacks
  | will just keep running without having
  | to worry about this.
  | 
  | The manager can save and reload all of
  | its device settings as XML, which makes
  | it very easy for you to save and reload
  | the audio setup of your application.
  | 
  | And to make it easy to let the user change
  | its settings, there's a component to
  | do just that - the AudioDeviceSelectorComponent
  | class, which contains a set of device
  | selection/sample-rate/latency controls.
  | 
  | To use an AudioDeviceManager, create
  | one, and use initialise() to set it up.
  | Then call addAudioCallback() to register
  | your audio callback with it, and use
  | that to process your audio data.
  | 
  | The manager also acts as a handy hub for
  | incoming midi messages, allowing a
  | listener to register for messages from
  | either a specific midi device, or from
  | whatever the current default midi input
  | device is. The listener then doesn't
  | have to worry about re-registering
  | with different midi devices if they
  | are changed or deleted.
  | 
  | And yet another neat trick is that amount
  | of CPU time being used is measured and
  | available with the getCpuUsage() method.
  | 
  | The AudioDeviceManager is a ChangeBroadcaster,
  | and will send a change message to listeners
  | whenever one of its settings is changed.
  | 
  | @see AudioDeviceSelectorComponent,
  | AudioIODevice, AudioIODeviceType
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioDeviceManager<'a> {
    base:                            ChangeBroadcaster<'a>,
    available_device_types:          Vec<Box<AudioIODeviceType>>,
    last_device_type_configs:        Vec<Box<AudioDeviceSetup>>,
    current_setup:                   AudioDeviceSetup,
    current_audio_device:            Box<AudioIODevice>,
    callbacks:                       Vec<*mut dyn AudioIODeviceCallback>,
    num_input_chans_needed:          i32, // default = 0
    num_output_chans_needed:         i32, // default = 2
    preferred_device_name:           String,
    current_device_type:             String,
    last_explicit_settings:          Box<XmlElement>,
    list_needs_scanning:             RefCell<bool>, // default = true
    temp_buffer:                     AudioBuffer<f32>,
    midi_device_infos_from_xml:      Vec<MidiDeviceInfo>,
    enabled_midi_inputs:             Vec<Box<MidiInput>>,
    midi_callbacks:                  Vec<MidiCallbackInfo>,
    default_midi_output_device_info: MidiDeviceInfo,
    default_midi_output:             Box<MidiOutput>,
    audio_callback_lock:             CriticalSection,
    midi_callback_lock:              CriticalSection,
    test_sound:                      Box<AudioBuffer<f32>>,
    test_sound_position:             i32, // default = 0
    load_measurer:                   AudioProcessLoadMeasurer,
    input_level_getter:              LevelMeterPtr, // default = LevelMeter() 
    output_level_getter:             LevelMeterPtr, // default = LevelMeter() 
    callback_handler:                Box<AudioDeviceManagerCallbackHandler<'a>>,
}

impl<'a> Default for AudioDeviceManager<'a> {
    
    /**
      | Creates a default AudioDeviceManager.
      | 
      | Initially no audio device will be selected.
      | You should call the initialise() method
      | and register an audio callback with
      | setAudioCallback() before it'll be
      | able to actually make any noise.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for AudioDeviceManager<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        currentAudioDevice.reset();
        defaultMidiOutput.reset();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioDeviceManager.cpp]
impl<'a> AudioDeviceManager<'a> {

    /**
      | Returns the currently-active audio
      | device.
      |
      */
    pub fn get_current_audio_device(&self) -> *mut AudioIODevice {
        
        todo!();
        /*
            return currentAudioDevice.get();
        */
    }

    /**
      | Returns the type of audio device currently
      | in use. @see setCurrentAudioDeviceType
      |
      */
    pub fn get_current_audio_device_type(&self) -> String {
        
        todo!();
        /*
            return currentDeviceType;
        */
    }

    /**
      | Returns the name of the default midi
      | output.
      | 
      | @see setDefaultMidiOutputDevice,
      | getDefaultMidiOutput
      |
      */
    pub fn get_default_midi_output_identifier(&self) -> &String {
        
        todo!();
        /*
            return defaultMidiOutputDeviceInfo.identifier;
        */
    }

    /**
      | Returns the current default midi output
      | device. If no device has been selected,
      | or the device can't be opened, this will
      | return nullptr.
      | 
      | @see getDefaultMidiOutputIdentifier
      |
      */
    pub fn get_default_midi_output(&self) -> *mut MidiOutput {
        
        todo!();
        /*
            return defaultMidiOutput.get();
        */
    }

    /**
      | Returns a reference-counted object
      | that can be used to get the current input
      | level.
      | 
      | You need to store this object locally
      | to ensure that the reference count is
      | incremented and decremented properly.
      | The current input level value can be
      | read using getCurrentLevel().
      |
      */
    pub fn get_input_level_getter(&mut self) -> LevelMeterPtr {
        
        todo!();
        /*
            return inputLevelGetter;
        */
    }

    /**
      | Returns a reference-counted object
      | that can be used to get the current output
      | level.
      | 
      | You need to store this object locally
      | to ensure that the reference count is
      | incremented and decremented properly.
      | The current output level value can be
      | read using getCurrentLevel().
      |
      */
    pub fn get_output_level_getter(&mut self) -> LevelMeterPtr {
        
        todo!();
        /*
            return outputLevelGetter;
        */
    }

    /**
      | Returns the a lock that can be used to
      | synchronise access to the audio callback.
      | Obviously while this is locked, you're
      | blocking the audio thread from running,
      | so it must only be used for very brief
      | periods when absolutely necessary.
      |
      */
    pub fn get_audio_callback_lock(&mut self) -> &mut CriticalSection {
        
        todo!();
        /*
            return audioCallbackLock;
        */
    }

    /**
      | Returns the a lock that can be used to
      | synchronise access to the midi callback.
      | Obviously while this is locked, you're
      | blocking the midi system from running,
      | so it must only be used for very brief
      | periods when absolutely necessary.
      |
      */
    pub fn get_midi_callback_lock(&mut self) -> &mut CriticalSection {
        
        todo!();
        /*
            return midiCallbackLock;
        */
    }

    pub fn restart_device(&mut self, 
        block_size_to_use:  i32,
        sample_rate_to_use: f64,
        ins:                &BigInteger,
        outs:               &BigInteger) -> String {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*
            callbackHandler.reset (new AudioDeviceManagerCallbackHandler (*this));
        */
    }
    
    pub fn create_device_types_if_needed(&mut self)  {
        
        todo!();
        /*
            if (availableDeviceTypes.size() == 0)
        {
            Vec<Box<AudioIODeviceType>> types;
            createAudioDeviceTypes (types);

            for (auto* t : types)
                addAudioDeviceType (std::unique_ptr<AudioIODeviceType> (t));

            types.clear (false);

            if (auto* first = availableDeviceTypes.getFirst())
                currentDeviceType = first->getTypeName();
        }
        */
    }
    
    /**
      | Returns a list of the types of device
      | supported.
      |
      */
    pub fn get_available_device_types(&mut self) -> &Vec<Box<AudioIODeviceType>> {
        
        todo!();
        /*
            scanDevicesIfNeeded();
        return availableDeviceTypes;
        */
    }
    
    pub fn update_current_setup(&mut self)  {
        
        todo!();
        /*
            if (currentAudioDevice != nullptr)
        {
            currentSetup.sampleRate     = currentAudioDevice->getCurrentSampleRate();
            currentSetup.bufferSize     = currentAudioDevice->getCurrentBufferSizeSamples();
            currentSetup.inputChannels  = currentAudioDevice->getActiveInputChannels();
            currentSetup.outputChannels = currentAudioDevice->getActiveOutputChannels();
        }
        */
    }
    
    pub fn audio_device_list_changed(&mut self)  {
        
        todo!();
        /*
            if (currentAudioDevice != nullptr)
        {
            auto currentDeviceStillAvailable = [&]
            {
                auto currentTypeName = currentAudioDevice->getTypeName();
                auto currentDeviceName = currentAudioDevice->getName();

                for (auto* deviceType : availableDeviceTypes)
                {
                    if (currentTypeName == deviceType->getTypeName())
                    {
                        for (auto& deviceName : deviceType->getDeviceNames (true))
                            if (currentDeviceName == deviceName)
                                return true;

                        for (auto& deviceName : deviceType->getDeviceNames (false))
                            if (currentDeviceName == deviceName)
                                return true;
                    }
                }

                return false;
            }();

            if (! currentDeviceStillAvailable)
            {
                closeAudioDevice();

                if (auto e = createStateXml())
                    initialiseFromXML (*e, true, preferredDeviceName, &currentSetup);
                else
                    initialiseDefault (preferredDeviceName, &currentSetup);
            }

            updateCurrentSetup();
        }

        sendChangeMessage();
        */
    }
    
    /**
      | Creates a list of available types.
      | 
      | This will add a set of new AudioIODeviceType
      | objects to the specified list, to represent
      | each available types of device.
      | 
      | You can override this if your app needs
      | to do something specific, like avoid
      | using DirectSound devices, etc.
      |
      */
    pub fn create_audio_device_types(&mut self, list: &mut Vec<Box<AudioIODeviceType>>)  {
        
        todo!();
        /*
            addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_WASAPI (WASAPIDeviceMode::shared));
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_WASAPI (WASAPIDeviceMode::exclusive));
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_WASAPI (WASAPIDeviceMode::sharedLowLatency));
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_DirectSound());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_ASIO());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_CoreAudio());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_iOSAudio());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_Bela());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_ALSA());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_JACK());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_Oboe());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_OpenSLES());
        addIfNotNull (list, AudioIODeviceType::createAudioIODeviceType_Android());
        */
    }
    
    /**
      | Adds a new device type to the list of types.
      |
      */
    pub fn add_audio_device_type(&mut self, new_device_type: Box<AudioIODeviceType>)  {
        
        todo!();
        /*
            if (newDeviceType != nullptr)
        {
            jassert (lastDeviceTypeConfigs.size() == availableDeviceTypes.size());

            availableDeviceTypes.add (newDeviceType.release());
            lastDeviceTypeConfigs.add (new AudioDeviceSetup());

            availableDeviceTypes.getLast()->addListener (callbackHandler.get());
        }
        */
    }
    
    /**
      | Removes a previously added device type
      | from the manager.
      |
      */
    pub fn remove_audio_device_type(&mut self, device_type_to_remove: *mut AudioIODeviceType)  {
        
        todo!();
        /*
            if (deviceTypeToRemove != nullptr)
        {
            jassert (lastDeviceTypeConfigs.size() == availableDeviceTypes.size());

            auto index = availableDeviceTypes.indexOf (deviceTypeToRemove);

            if (auto removed = std::unique_ptr<AudioIODeviceType> (availableDeviceTypes.removeAndReturn (index)))
            {
                removed->removeListener (callbackHandler.get());
                lastDeviceTypeConfigs.remove (index, true);
            }
        }
        */
    }
    
    /**
      | Opens a set of audio devices ready for
      | use.
      | 
      | This will attempt to open either a default
      | audio device, or one that was previously
      | saved as XML.
      | 
      | -----------
      | @param numInputChannelsNeeded
      | 
      | the maximum number of input channels
      | your app would like to use (the actual
      | number of channels opened may be less
      | than the number requested)
      | ----------
      | @param numOutputChannelsNeeded
      | 
      | the maximum number of output channels
      | your app would like to use (the actual
      | number of channels opened may be less
      | than the number requested)
      | ----------
      | @param savedState
      | 
      | either a previously-saved state that
      | was produced by createStateXml(),
      | or nullptr if you want the manager to
      | choose the best device to open.
      | ----------
      | @param selectDefaultDeviceOnFailure
      | 
      | if true, then if the device specified
      | in the XML fails to open, then a default
      | device will be used instead. If false,
      | then on failure, no device is opened.
      | ----------
      | @param preferredDefaultDeviceName
      | 
      | if this is not empty, and there's a device
      | with this name, then that will be used
      | as the default device (assuming that
      | there wasn't one specified in the XML).
      | The string can actually be a simple wildcard,
      | containing "*" and "?" characters
      | ----------
      | @param preferredSetupOptions
      | 
      | if this is non-null, the structure will
      | be used as the set of preferred settings
      | when opening the device. If you use this
      | parameter, the preferredDefaultDeviceName
      | field will be ignored
      | 
      | -----------
      | @return
      | 
      | an error message if anything went wrong,
      | or an empty string if it worked ok.
      |
      */
    pub fn initialise(
        &mut self, 
        num_input_channels_needed:        i32,
        num_output_channels_needed:       i32,
        xml:                              *const XmlElement,
        select_default_device_on_failure: bool,
        preferred_default_device_name:    Option<&str>,
        preferred_setup_options:          *const AudioDeviceSetup

    ) -> String {

        let preferred_default_device_name = preferred_default_device_name.unwrap_or("");
        
        todo!();
        /*
            scanDevicesIfNeeded();

        numInputChansNeeded = numInputChannelsNeeded;
        numOutputChansNeeded = numOutputChannelsNeeded;
        preferredDeviceName = preferredDefaultDeviceName;

        if (xml != nullptr && xml->hasTagName ("DEVICESETUP"))
            return initialiseFromXML (*xml, selectDefaultDeviceOnFailure,
                                      preferredDeviceName, preferredSetupOptions);

        return initialiseDefault (preferredDeviceName, preferredSetupOptions);
        */
    }
    
    pub fn initialise_default(
        &mut self, 
        preferred_default_device_name: &String,
        preferred_setup_options:       *const AudioDeviceSetup
    ) -> String {
        
        todo!();
        /*
            AudioDeviceSetup setup;

        if (preferredSetupOptions != nullptr)
        {
            setup = *preferredSetupOptions;
        }
        else if (preferredDefaultDeviceName.isNotEmpty())
        {
            for (auto* type : availableDeviceTypes)
            {
                for (auto& out : type->getDeviceNames (false))
                {
                    if (out.matchesWildcard (preferredDefaultDeviceName, true))
                    {
                        setup.outputDeviceName = out;
                        break;
                    }
                }

                for (auto& in : type->getDeviceNames (true))
                {
                    if (in.matchesWildcard (preferredDefaultDeviceName, true))
                    {
                        setup.inputDeviceName = in;
                        break;
                    }
                }
            }
        }

        insertDefaultDeviceNames (setup);
        return setAudioDeviceSetup (setup, false);
        */
    }
    
    pub fn initialise_fromxml(&mut self, 
        xml:                              &XmlElement,
        select_default_device_on_failure: bool,
        preferred_default_device_name:    &String,
        preferred_setup_options:          *const AudioDeviceSetup) -> String {
        
        todo!();
        /*
            lastExplicitSettings.reset (new XmlElement (xml));

        String error;
        AudioDeviceSetup setup;

        if (preferredSetupOptions != nullptr)
            setup = *preferredSetupOptions;

        if (xml.getStringAttribute ("audioDeviceName").isNotEmpty())
        {
            setup.inputDeviceName = setup.outputDeviceName
                = xml.getStringAttribute ("audioDeviceName");
        }
        else
        {
            setup.inputDeviceName  = xml.getStringAttribute ("audioInputDeviceName");
            setup.outputDeviceName = xml.getStringAttribute ("audioOutputDeviceName");
        }

        currentDeviceType = xml.getStringAttribute ("deviceType");

        if (findType (currentDeviceType) == nullptr)
        {
            if (auto* type = findType (setup.inputDeviceName, setup.outputDeviceName))
                currentDeviceType = type->getTypeName();
            else if (auto* firstType = availableDeviceTypes.getFirst())
                currentDeviceType = firstType->getTypeName();
        }

        setup.bufferSize = xml.getIntAttribute ("audioDeviceBufferSize", setup.bufferSize);
        setup.sampleRate = xml.getDoubleAttribute ("audioDeviceRate", setup.sampleRate);

        setup.inputChannels .parseString (xml.getStringAttribute ("audioDeviceInChans",  "11"), 2);
        setup.outputChannels.parseString (xml.getStringAttribute ("audioDeviceOutChans", "11"), 2);

        setup.useDefaultInputChannels  = ! xml.hasAttribute ("audioDeviceInChans");
        setup.useDefaultOutputChannels = ! xml.hasAttribute ("audioDeviceOutChans");

        error = setAudioDeviceSetup (setup, true);

        if (error.isNotEmpty() && selectDefaultDeviceOnFailure)
            error = initialise (numInputChansNeeded, numOutputChansNeeded, nullptr, false, preferredDefaultDeviceName);

        midiDeviceInfosFromXml.clear();
        enabledMidiInputs.clear();

        for (auto* c : xml.getChildWithTagNameIterator ("MIDIINPUT"))
            midiDeviceInfosFromXml.add ({ c->getStringAttribute ("name"), c->getStringAttribute ("identifier") });

        auto isIdentifierAvailable = [] (const Vec<MidiDeviceInfo>& available, const String& identifier)
        {
            for (auto& device : available)
                if (device.identifier == identifier)
                    return true;

            return false;
        };

        auto getUpdatedIdentifierForName = [&] (const Vec<MidiDeviceInfo>& available, const String& name) -> String
        {
            for (auto& device : available)
                if (device.name == name)
                    return device.identifier;

            return {};
        };

        auto inputs = MidiInput::getAvailableDevices();

        for (auto& info : midiDeviceInfosFromXml)
        {
            if (isIdentifierAvailable (inputs, info.identifier))
            {
                setMidiInputDeviceEnabled (info.identifier, true);
            }
            else
            {
                auto identifier = getUpdatedIdentifierForName (inputs, info.name);

                if (identifier.isNotEmpty())
                    setMidiInputDeviceEnabled (identifier, true);
            }
        }

        MidiDeviceInfo defaultOutputDeviceInfo (xml.getStringAttribute ("defaultMidiOutput"),
                                                xml.getStringAttribute ("defaultMidiOutputDevice"));

        auto outputs = MidiOutput::getAvailableDevices();

        if (isIdentifierAvailable (outputs, defaultOutputDeviceInfo.identifier))
        {
            setDefaultMidiOutputDevice (defaultOutputDeviceInfo.identifier);
        }
        else
        {
            auto identifier = getUpdatedIdentifierForName (outputs, defaultOutputDeviceInfo.name);

            if (identifier.isNotEmpty())
                setDefaultMidiOutputDevice (identifier);
        }

        return error;
        */
    }
    
    /**
      | Resets everything to a default device
      | setup, clearing any stored settings.
      |
      */
    pub fn initialise_with_default_devices(&mut self, 
        num_input_channels_needed:  i32,
        num_output_channels_needed: i32) -> String {
        
        todo!();
        /*
            lastExplicitSettings.reset();

        return initialise (numInputChannelsNeeded, numOutputChannelsNeeded,
                           nullptr, false, {}, nullptr);
        */
    }
    
    pub fn insert_default_device_names(&self, setup: &mut AudioDeviceSetup)  {
        
        todo!();
        /*
            if (auto* type = getCurrentDeviceTypeObject())
        {
            if (numOutputChansNeeded > 0 && setup.outputDeviceName.isEmpty())
                setup.outputDeviceName = type->getDeviceNames (false) [type->getDefaultDeviceIndex (false)];

            if (numInputChansNeeded > 0 && setup.inputDeviceName.isEmpty())
                setup.inputDeviceName = type->getDeviceNames (true) [type->getDefaultDeviceIndex (true)];
        }
        */
    }
    
    /**
      | Returns some XML representing the current
      | state of the manager.
      | 
      | This stores the current device, its
      | samplerate, block size, etc, and can
      | be restored later with initialise().
      | 
      | Note that this can return a null pointer
      | if no settings have been explicitly
      | changed (i.e. if the device manager
      | has just been left in its default state).
      |
      */
    pub fn create_state_xml(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            if (lastExplicitSettings != nullptr)
            return std::make_unique<XmlElement> (*lastExplicitSettings);

        return {};
        */
    }
    
    pub fn scan_devices_if_needed(&mut self)  {
        
        todo!();
        /*
            if (listNeedsScanning)
        {
            listNeedsScanning = false;

            createDeviceTypesIfNeeded();

            for (auto* type : availableDeviceTypes)
                type->scanForDevices();
        }
        */
    }
    
    pub fn find_type(&mut self, type_name: &String) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            scanDevicesIfNeeded();

        for (auto* type : availableDeviceTypes)
            if (type->getTypeName() == typeName)
                return type;

        return {};
        */
    }
    
    pub fn find_type_with_io_names(
        &mut self, 
        input_name:  &String,
        output_name: &String

    ) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            scanDevicesIfNeeded();

        for (auto* type : availableDeviceTypes)
            if ((inputName.isNotEmpty() && deviceListContains (type, true, inputName))
                 || (outputName.isNotEmpty() && deviceListContains (type, false, outputName)))
                return type;

        return {};
        */
    }
    
    /**
      | Returns the current device properties
      | that are in use. @see setAudioDeviceSetup
      |
      */
    pub fn get_audio_device_setup(&self) -> AudioDeviceSetup {
        
        todo!();
        /*
            return currentSetup;
        */
    }
    
    /**
      | Returns the current device properties
      | that are in use. This is an old method,
      | kept around for compatibility, but
      | you should prefer the new version which
      | returns the result rather than taking
      | an out-parameter. @see getAudioDeviceSetup()
      |
      */
    pub fn get_audio_device_setup_into(&self, setup: &mut AudioDeviceSetup)  {
        
        todo!();
        /*
            setup = currentSetup;
        */
    }
    
    pub fn delete_current_device(&mut self)  {
        
        todo!();
        /*
            currentAudioDevice.reset();
        currentSetup.inputDeviceName.clear();
        currentSetup.outputDeviceName.clear();
        */
    }
    
    /**
      | Changes the class of audio device being
      | used.
      | 
      | This switches between, e.g. ASIO and
      | DirectSound. On the Mac you probably
      | won't ever call this because there's
      | only one type: CoreAudio.
      | 
      | For a list of types, see getAvailableDeviceTypes().
      |
      */
    pub fn set_current_audio_device_type(&mut self, 
        ty:                     &String,
        treat_as_chosen_device: bool)  {
        
        todo!();
        /*
            for (int i = 0; i < availableDeviceTypes.size(); ++i)
        {
            if (availableDeviceTypes.getUnchecked(i)->getTypeName() == type
                 && currentDeviceType != type)
            {
                if (currentAudioDevice != nullptr)
                {
                    closeAudioDevice();
                    Thread::sleep (1500); // allow a moment for OS devices to sort themselves out, to help
                                          // avoid things like DirectSound/ASIO clashes
                }

                currentDeviceType = type;

                AudioDeviceSetup s (*lastDeviceTypeConfigs.getUnchecked(i));
                insertDefaultDeviceNames (s);

                setAudioDeviceSetup (s, treatAsChosenDevice);

                sendChangeMessage();
                break;
            }
        }
        */
    }
    
    /**
      | Returns the currently active audio
      | device type object. Don't keep a copy
      | of this pointer - it's owned by the device
      | manager and could change at any time.
      |
      */
    pub fn get_current_device_type_object(&self) -> *mut AudioIODeviceType {
        
        todo!();
        /*
            for (auto* type : availableDeviceTypes)
            if (type->getTypeName() == currentDeviceType)
                return type;

        return availableDeviceTypes.getFirst();
        */
    }
    
    /**
      | Changes the current device or its settings.
      | 
      | If you want to change a device property,
      | like the current sample rate or block
      | size, you can call getAudioDeviceSetup()
      | to retrieve the current settings, then
      | tweak the appropriate fields in the
      | AudioDeviceSetup structure, and pass
      | it back into this method to apply the
      | new settings.
      | 
      | -----------
      | @param newSetup
      | 
      | the settings that you'd like to use
      | ----------
      | @param treatAsChosenDevice
      | 
      | if this is true and if the device opens
      | correctly, these new settings will
      | be taken as having been explicitly chosen
      | by the user, and the next time createStateXml()
      | is called, these settings will be returned.
      | If it's false, then the device is treated
      | as a temporary or default device, and
      | a call to createStateXml() will return
      | either the last settings that were made
      | with treatAsChosenDevice as true,
      | or the last XML settings that were passed
      | into initialise().
      | 
      | -----------
      | @return
      | 
      | an error message if anything went wrong,
      | or an empty string if it worked ok.
      | 
      | @see getAudioDeviceSetup
      |
      */
    pub fn set_audio_device_setup(&mut self, 
        new_setup:              &AudioDeviceSetup,
        treat_as_chosen_device: bool) -> String {
        
        todo!();
        /*
            jassert (&newSetup != &currentSetup);    // this will have no effect

        if (newSetup != currentSetup)
            sendChangeMessage();
        else if (currentAudioDevice != nullptr)
            return {};

        stopDevice();

        if (getCurrentDeviceTypeObject() == nullptr
            || (newSetup.inputDeviceName.isEmpty() && newSetup.outputDeviceName.isEmpty()))
        {
            deleteCurrentDevice();

            if (treatAsChosenDevice)
                updateXml();

            return {};
        }

        String error;

        if (currentSetup.inputDeviceName  != newSetup.inputDeviceName
         || currentSetup.outputDeviceName != newSetup.outputDeviceName
         || currentAudioDevice == nullptr)
        {
            deleteCurrentDevice();
            scanDevicesIfNeeded();

            auto* type = getCurrentDeviceTypeObject();

            if (newSetup.outputDeviceName.isNotEmpty() && ! deviceListContains (type, false, newSetup.outputDeviceName))
                return "No such device: " + newSetup.outputDeviceName;

            if (newSetup.inputDeviceName.isNotEmpty() && ! deviceListContains (type, true, newSetup.inputDeviceName))
                return "No such device: " + newSetup.inputDeviceName;

            currentAudioDevice.reset (type->createDevice (newSetup.outputDeviceName, newSetup.inputDeviceName));

            if (currentAudioDevice == nullptr)
                error = "Can't open the audio device!\n\n"
                        "This may be because another application is currently using the same device - "
                        "if so, you should close any other applications and try again!";
            else
                error = currentAudioDevice->getLastError();

            if (error.isNotEmpty())
            {
                deleteCurrentDevice();
                return error;
            }
        }

        currentSetup = newSetup;

        if (! currentSetup.useDefaultInputChannels)    numInputChansNeeded  = currentSetup.inputChannels.countNumberOfSetBits();
        if (! currentSetup.useDefaultOutputChannels)   numOutputChansNeeded = currentSetup.outputChannels.countNumberOfSetBits();

        updateSetupChannels (currentSetup, numInputChansNeeded, numOutputChansNeeded);

        if (currentSetup.inputChannels.isZero() && currentSetup.outputChannels.isZero())
        {
            if (treatAsChosenDevice)
                updateXml();

            return {};
        }

        currentSetup.sampleRate = chooseBestSampleRate (currentSetup.sampleRate);
        currentSetup.bufferSize = chooseBestBufferSize (currentSetup.bufferSize);

        error = currentAudioDevice->open (currentSetup.inputChannels,
                                          currentSetup.outputChannels,
                                          currentSetup.sampleRate,
                                          currentSetup.bufferSize);

        if (error.isEmpty())
        {
            currentDeviceType = currentAudioDevice->getTypeName();

            currentAudioDevice->start (callbackHandler.get());

            currentSetup.sampleRate     = currentAudioDevice->getCurrentSampleRate();
            currentSetup.bufferSize     = currentAudioDevice->getCurrentBufferSizeSamples();
            currentSetup.inputChannels  = currentAudioDevice->getActiveInputChannels();
            currentSetup.outputChannels = currentAudioDevice->getActiveOutputChannels();

            for (int i = 0; i < availableDeviceTypes.size(); ++i)
                if (availableDeviceTypes.getUnchecked (i)->getTypeName() == currentDeviceType)
                    *(lastDeviceTypeConfigs.getUnchecked (i)) = currentSetup;

            if (treatAsChosenDevice)
                updateXml();
        }
        else
        {
            deleteCurrentDevice();
        }

        return error;
        */
    }
    
    pub fn choose_best_sample_rate(&self, rate: f64) -> f64 {
        
        todo!();
        /*
            jassert (currentAudioDevice != nullptr);

        auto rates = currentAudioDevice->getAvailableSampleRates();

        if (rate > 0 && rates.contains (rate))
            return rate;

        rate = currentAudioDevice->getCurrentSampleRate();

        if (rate > 0 && rates.contains (rate))
            return rate;

        double lowestAbove44 = 0.0;

        for (int i = rates.size(); --i >= 0;)
        {
            auto sr = rates[i];

            if (sr >= 44100.0 && (lowestAbove44 < 1.0 || sr < lowestAbove44))
                lowestAbove44 = sr;
        }

        if (lowestAbove44 > 0.0)
            return lowestAbove44;

        return rates[0];
        */
    }
    
    pub fn choose_best_buffer_size(&self, buffer_size: i32) -> i32 {
        
        todo!();
        /*
            jassert (currentAudioDevice != nullptr);

        if (bufferSize > 0 && currentAudioDevice->getAvailableBufferSizes().contains (bufferSize))
            return bufferSize;

        return currentAudioDevice->getDefaultBufferSize();
        */
    }
    
    pub fn stop_device(&mut self)  {
        
        todo!();
        /*
            if (currentAudioDevice != nullptr)
            currentAudioDevice->stop();

        testSound.reset();
        */
    }
    
    /**
      | Closes the currently-open device.
      | You can call restartLastAudioDevice()
      | later to reopen it in the same state that
      | it was just in.
      |
      */
    pub fn close_audio_device(&mut self)  {
        
        todo!();
        /*
            stopDevice();
        currentAudioDevice.reset();
        loadMeasurer.reset();
        */
    }
    
    /**
      | Tries to reload the last audio device
      | that was running.
      | 
      | Note that this only reloads the last
      | device that was running before closeAudioDevice()
      | was called - it doesn't reload any kind
      | of saved-state, and can only be called
      | after a device has been opened with setAudioDeviceSetup().
      | 
      | If a device is already open, this call
      | will do nothing.
      |
      */
    pub fn restart_last_audio_device(&mut self)  {
        
        todo!();
        /*
            if (currentAudioDevice == nullptr)
        {
            if (currentSetup.inputDeviceName.isEmpty()
                  && currentSetup.outputDeviceName.isEmpty())
            {
                // This method will only reload the last device that was running
                // before closeAudioDevice() was called - you need to actually open
                // one first, with setAudioDeviceSetup().
                jassertfalse;
                return;
            }

            AudioDeviceSetup s (currentSetup);
            setAudioDeviceSetup (s, false);
        }
        */
    }
    
    pub fn update_xml(&mut self)  {
        
        todo!();
        /*
            lastExplicitSettings.reset (new XmlElement ("DEVICESETUP"));

        lastExplicitSettings->setAttribute ("deviceType", currentDeviceType);
        lastExplicitSettings->setAttribute ("audioOutputDeviceName", currentSetup.outputDeviceName);
        lastExplicitSettings->setAttribute ("audioInputDeviceName", currentSetup.inputDeviceName);

        if (currentAudioDevice != nullptr)
        {
            lastExplicitSettings->setAttribute ("audioDeviceRate", currentAudioDevice->getCurrentSampleRate());

            if (currentAudioDevice->getDefaultBufferSize() != currentAudioDevice->getCurrentBufferSizeSamples())
                lastExplicitSettings->setAttribute ("audioDeviceBufferSize", currentAudioDevice->getCurrentBufferSizeSamples());

            if (! currentSetup.useDefaultInputChannels)
                lastExplicitSettings->setAttribute ("audioDeviceInChans", currentSetup.inputChannels.toString (2));

            if (! currentSetup.useDefaultOutputChannels)
                lastExplicitSettings->setAttribute ("audioDeviceOutChans", currentSetup.outputChannels.toString (2));
        }

        for (auto& input : enabledMidiInputs)
        {
            auto* child = lastExplicitSettings->createNewChildElement ("MIDIINPUT");

            child->setAttribute ("name",       input->getName());
            child->setAttribute ("identifier", input->getIdentifier());
        }

        if (midiDeviceInfosFromXml.size() > 0)
        {
            // Add any midi devices that have been enabled before, but which aren't currently
            // open because the device has been disconnected.
            auto availableMidiDevices = MidiInput::getAvailableDevices();

            for (auto& d : midiDeviceInfosFromXml)
            {
                if (! availableMidiDevices.contains (d))
                {
                    auto* child = lastExplicitSettings->createNewChildElement ("MIDIINPUT");

                    child->setAttribute ("name",       d.name);
                    child->setAttribute ("identifier", d.identifier);
                }
            }
        }

        if (defaultMidiOutputDeviceInfo != MidiDeviceInfo())
        {
            lastExplicitSettings->setAttribute ("defaultMidiOutput",       defaultMidiOutputDeviceInfo.name);
            lastExplicitSettings->setAttribute ("defaultMidiOutputDevice", defaultMidiOutputDeviceInfo.identifier);
        }
        */
    }
    
    /**
      | Registers an audio callback to be used.
      | 
      | The manager will redirect callbacks
      | from whatever audio device is currently
      | in use to all registered callback objects.
      | If more than one callback is active,
      | they will all be given the same input
      | data, and their outputs will be summed.
      | 
      | If necessary, this method will invoke
      | audioDeviceAboutToStart() on the
      | callback object before returning.
      | 
      | To remove a callback, use removeAudioCallback().
      |
      */
    pub fn add_audio_callback(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            {
            const ScopedLock sl (audioCallbackLock);

            if (callbacks.contains (newCallback))
                return;
        }

        if (currentAudioDevice != nullptr && newCallback != nullptr)
            newCallback->audioDeviceAboutToStart (currentAudioDevice.get());

        const ScopedLock sl (audioCallbackLock);
        callbacks.add (newCallback);
        */
    }
    
    /**
      | Deregisters a previously added callback.
      | 
      | If necessary, this method will invoke
      | audioDeviceStopped() on the callback
      | object before returning.
      | 
      | @see addAudioCallback
      |
      */
    pub fn remove_audio_callback(&mut self, callback_to_remove: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (callbackToRemove != nullptr)
        {
            bool needsDeinitialising = currentAudioDevice != nullptr;

            {
                const ScopedLock sl (audioCallbackLock);

                needsDeinitialising = needsDeinitialising && callbacks.contains (callbackToRemove);
                callbacks.removeFirstMatchingValue (callbackToRemove);
            }

            if (needsDeinitialising)
                callbackToRemove->audioDeviceStopped();
        }
        */
    }
    
    pub fn audio_device_io_callback_int(&mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32)  {
        
        todo!();
        /*
            const ScopedLock sl (audioCallbackLock);

        inputLevelGetter->updateLevel (inputChannelData, numInputChannels, numSamples);
        outputLevelGetter->updateLevel (const_cast<const float**> (outputChannelData), numOutputChannels, numSamples);

        if (callbacks.size() > 0)
        {
            AudioProcessLoadMeasurer::ScopedTimer timer (loadMeasurer);

            tempBuffer.setSize (jmax (1, numOutputChannels), jmax (1, numSamples), false, false, true);

            callbacks.getUnchecked(0)->audioDeviceIOCallback (inputChannelData, numInputChannels,
                                                              outputChannelData, numOutputChannels, numSamples);

            auto** tempChans = tempBuffer.getArrayOfWritePointers();

            for (int i = callbacks.size(); --i > 0;)
            {
                callbacks.getUnchecked(i)->audioDeviceIOCallback (inputChannelData, numInputChannels,
                                                                  tempChans, numOutputChannels, numSamples);

                for (int chan = 0; chan < numOutputChannels; ++chan)
                {
                    if (auto* src = tempChans [chan])
                        if (auto* dst = outputChannelData [chan])
                            for (int j = 0; j < numSamples; ++j)
                                dst[j] += src[j];
                }
            }
        }
        else
        {
            for (int i = 0; i < numOutputChannels; ++i)
                zeromem (outputChannelData[i], (size_t) numSamples * sizeof (float));
        }

        if (testSound != nullptr)
        {
            auto numSamps = jmin (numSamples, testSound->getNumSamples() - testSoundPosition);
            auto* src = testSound->getReadPointer (0, testSoundPosition);

            for (int i = 0; i < numOutputChannels; ++i)
                if (auto* dst = outputChannelData [i])
                    for (int j = 0; j < numSamps; ++j)
                        dst[j] += src[j];

            testSoundPosition += numSamps;

            if (testSoundPosition >= testSound->getNumSamples())
                testSound.reset();
        }
        */
    }
    
    pub fn audio_device_about_to_start_int(&mut self, device: *mut AudioIODevice)  {
        
        todo!();
        /*
            loadMeasurer.reset (device->getCurrentSampleRate(),
                            device->getCurrentBufferSizeSamples());

        {
            const ScopedLock sl (audioCallbackLock);

            for (int i = callbacks.size(); --i >= 0;)
                callbacks.getUnchecked(i)->audioDeviceAboutToStart (device);
        }

        updateCurrentSetup();
        sendChangeMessage();
        */
    }
    
    pub fn audio_device_stopped_int(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage();

        const ScopedLock sl (audioCallbackLock);

        loadMeasurer.reset();

        for (int i = callbacks.size(); --i >= 0;)
            callbacks.getUnchecked(i)->audioDeviceStopped();
        */
    }
    
    pub fn audio_device_error_int(&mut self, message: &String)  {
        
        todo!();
        /*
            const ScopedLock sl (audioCallbackLock);

        for (int i = callbacks.size(); --i >= 0;)
            callbacks.getUnchecked(i)->audioDeviceError (message);
        */
    }
    
    /**
      | Returns the average proportion of available
      | CPU being spent inside the audio callbacks.
      | 
      | -----------
      | @return
      | 
      | A value between 0 and 1.0 to indicate
      | the approximate proportion of CPU time
      | spent in the callbacks.
      |
      */
    pub fn get_cpu_usage(&self) -> f64 {
        
        todo!();
        /*
            return loadMeasurer.getLoadAsProportion();
        */
    }
    
    /**
      | Enables or disables a midi input device.
      | 
      | The list of devices can be obtained with
      | the MidiInput::getAvailableDevices()
      | method.
      | 
      | Any incoming messages from enabled
      | input devices will be forwarded on to
      | all the listeners that have been registered
      | with the addMidiInputDeviceCallback()
      | method. They can either register for
      | messages from a particular device,
      | or from just the "default" midi input.
      | 
      | Routing the midi input via an AudioDeviceManager
      | means that when a listener registers
      | for the default midi input, this default
      | device can be changed by the manager
      | without the listeners having to know
      | about it or re-register.
      | 
      | It also means that a listener can stay
      | registered for a midi input that is disabled
      | or not present, so that when the input
      | is re-enabled, the listener will start
      | receiving messages again.
      | 
      | @see addMidiInputDeviceCallback,
      | isMidiInputDeviceEnabled
      |
      */
    pub fn set_midi_input_device_enabled(&mut self, 
        identifier: &String,
        enabled:    bool)  {
        
        todo!();
        /*
            if (enabled != isMidiInputDeviceEnabled (identifier))
        {
            if (enabled)
            {
                if (auto midiIn = MidiInput::openDevice (identifier, callbackHandler.get()))
                {
                    enabledMidiInputs.push_back (std::move (midiIn));
                    enabledMidiInputs.back()->start();
                }
            }
            else
            {
                auto removePredicate = [identifier] (const std::unique_ptr<MidiInput>& in) { return in->getIdentifier() == identifier; };
                enabledMidiInputs.erase (std::remove_if (std::begin (enabledMidiInputs), std::end (enabledMidiInputs), removePredicate),
                                         std::end (enabledMidiInputs));
            }

            updateXml();
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Returns true if a given midi input device
      | is being used.
      | 
      | @see setMidiInputDeviceEnabled
      |
      */
    pub fn is_midi_input_device_enabled(&self, identifier: &String) -> bool {
        
        todo!();
        /*
            for (auto& mi : enabledMidiInputs)
            if (mi->getIdentifier() == identifier)
                return true;

        return false;
        */
    }
    
    /**
      | Registers a listener for callbacks
      | when midi events arrive from a midi input.
      | 
      | The device identifier can be empty to
      | indicate that it wants to receive all
      | incoming events from all the enabled
      | MIDI inputs. Or it can be the identifier
      | of one of the MIDI input devices if it
      | just wants the events from that device.
      | (see MidiInput::getAvailableDevices()
      | for the list of devices).
      | 
      | Only devices which are enabled (see
      | the setMidiInputDeviceEnabled()
      | method) will have their events forwarded
      | on to listeners.
      |
      */
    pub fn add_midi_input_device_callback(
        &mut self, 
        identifier:      &String,
        callback_to_add: *mut dyn MidiInputCallback

    ) {
        
        todo!();
        /*
            removeMidiInputDeviceCallback (identifier, callbackToAdd);

        if (identifier.isEmpty() || isMidiInputDeviceEnabled (identifier))
        {
            const ScopedLock sl (midiCallbackLock);
            midiCallbacks.add ({ identifier, callbackToAdd });
        }
        */
    }
    
    /**
      | Removes a listener that was previously
      | registered with addMidiInputDeviceCallback().
      |
      */
    pub fn remove_midi_input_device_callback(
        &mut self, 
        identifier:         &String,
        callback_to_remove: *mut dyn MidiInputCallback

    ) {
        
        todo!();
        /*
            for (int i = midiCallbacks.size(); --i >= 0;)
        {
            auto& mc = midiCallbacks.getReference (i);

            if (mc.callback == callbackToRemove && mc.deviceIdentifier == identifier)
            {
                const ScopedLock sl (midiCallbackLock);
                midiCallbacks.remove (i);
            }
        }
        */
    }
    
    pub fn handle_incoming_midi_message_int(&mut self, 
        source:  *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            if (! message.isActiveSense())
        {
            const ScopedLock sl (midiCallbackLock);

            for (auto& mc : midiCallbacks)
                if (mc.deviceIdentifier.isEmpty() || mc.deviceIdentifier == source->getIdentifier())
                    mc.callback->handleIncomingMidiMessage (source, message);
        }
        */
    }
    
    /**
      | Sets a midi output device to use as the
      | default.
      | 
      | The list of devices can be obtained with
      | the MidiOutput::getAvailableDevices()
      | method.
      | 
      | The specified device will be opened
      | automatically and can be retrieved
      | with the getDefaultMidiOutput() method.
      | 
      | Pass in an empty string to deselect all
      | devices. For the default device, you
      | can use MidiOutput::getDefaultDevice().
      | 
      | @see getDefaultMidiOutput, getDefaultMidiOutputIdentifier
      |
      */
    pub fn set_default_midi_output_device(&mut self, identifier: &String)  {
        
        todo!();
        /*
            if (defaultMidiOutputDeviceInfo.identifier != identifier)
        {
            Vec<AudioIODeviceCallback*> oldCallbacks;

            {
                const ScopedLock sl (audioCallbackLock);
                oldCallbacks.swapWith (callbacks);
            }

            if (currentAudioDevice != nullptr)
                for (int i = oldCallbacks.size(); --i >= 0;)
                    oldCallbacks.getUnchecked (i)->audioDeviceStopped();

            defaultMidiOutput.reset();

            if (identifier.isNotEmpty())
                defaultMidiOutput = MidiOutput::openDevice (identifier);

            if (defaultMidiOutput != nullptr)
                defaultMidiOutputDeviceInfo = defaultMidiOutput->getDeviceInfo();
            else
                defaultMidiOutputDeviceInfo = {};

            if (currentAudioDevice != nullptr)
                for (auto* c : oldCallbacks)
                    c->audioDeviceAboutToStart (currentAudioDevice.get());

            {
                const ScopedLock sl (audioCallbackLock);
                oldCallbacks.swapWith (callbacks);
            }

            updateXml();
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Plays a beep through the current audio
      | device.
      | 
      | This is here to allow the audio setup
      | UI panels to easily include a "test"
      | button so that the user can check where
      | the audio is coming from.
      |
      */
    pub fn play_test_sound(&mut self)  {
        
        todo!();
        /*
            { // cunningly nested to swap, unlock and delete in that order.
            std::unique_ptr<AudioBuffer<float>> oldSound;

            {
                const ScopedLock sl (audioCallbackLock);
                std::swap (oldSound, testSound);
            }
        }

        testSoundPosition = 0;

        if (currentAudioDevice != nullptr)
        {
            auto sampleRate = currentAudioDevice->getCurrentSampleRate();
            auto soundLength = (int) sampleRate;

            double frequency = 440.0;
            float amplitude = 0.5f;

            auto phasePerSample = MathConstants<double>::twoPi / (sampleRate / frequency);

            std::unique_ptr<AudioBuffer<float>> newSound (new AudioBuffer<float> (1, soundLength));

            for (int i = 0; i < soundLength; ++i)
                newSound->setSample (0, i, amplitude * (float) std::sin (i * phasePerSample));

            newSound->applyGainRamp (0, 0, soundLength / 10, 0.0f, 1.0f);
            newSound->applyGainRamp (0, soundLength - soundLength / 4, soundLength / 4, 1.0f, 0.0f);

            {
                const ScopedLock sl (audioCallbackLock);
                std::swap (testSound, newSound);
            }
        }
        */
    }
    
    /**
      | Returns the number of under- or over
      | runs reported.
      | 
      | This method will use the underlying
      | device's native getXRunCount if it
      | supports it. Otherwise it will estimate
      | the number of under-/overruns by measuring
      | the time it spent in the audio callback.
      |
      */
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            auto deviceXRuns = (currentAudioDevice != nullptr ? currentAudioDevice->getXRunCount() : -1);
        return jmax (0, deviceXRuns) + loadMeasurer.getXRunCount();
        */
    }
    
    #[deprecated]
    pub fn set_midi_input_enabled(&mut self, 
        name:    &String,
        enabled: bool)  {
        
        todo!();
        /*
            for (auto& device : MidiInput::getAvailableDevices())
        {
            if (device.name == name)
            {
                setMidiInputDeviceEnabled (device.identifier, enabled);
                return;
            }
        }
        */
    }
    
    pub fn is_midi_input_enabled(&self, name: &String) -> bool {
        
        todo!();
        /*
            for (auto& device : MidiInput::getAvailableDevices())
            if (device.name == name)
                return isMidiInputDeviceEnabled (device.identifier);

        return false;
        */
    }
    
    pub fn add_midi_input_callback(
        &mut self, 
        name:            &String,
        callback_to_add: *mut dyn MidiInputCallback

    ) {
        
        todo!();
        /*
            if (name.isEmpty())
        {
            addMidiInputDeviceCallback ({}, callbackToAdd);
        }
        else
        {
            for (auto& device : MidiInput::getAvailableDevices())
            {
                if (device.name == name)
                {
                    addMidiInputDeviceCallback (device.identifier, callbackToAdd);
                    return;
                }
            }
        }
        */
    }
    
    pub fn remove_midi_input_callback(
        &mut self, 
        name:               &String,
        callback_to_remove: *mut dyn MidiInputCallback

    ) {
        
        todo!();
        /*
            if (name.isEmpty())
        {
            removeMidiInputDeviceCallback ({}, callbackToRemove);
        }
        else
        {
            for (auto& device : MidiInput::getAvailableDevices())
            {
                if (device.name == name)
                {
                    removeMidiInputDeviceCallback (device.identifier, callbackToRemove);
                    return;
                }
            }
        }
        */
    }
    
    pub fn set_default_midi_output(&mut self, name: &String)  {
        
        todo!();
        /*
            for (auto& device : MidiOutput::getAvailableDevices())
        {
            if (device.name == name)
            {
                setDefaultMidiOutputDevice (device.identifier);
                return;
            }
        }
        */
    }
}
