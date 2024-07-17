crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Standalone/aloe_StandaloneFilterWindow.h]

/**
  | An object that creates and plays a standalone
  | instance of an AudioProcessor.
  | 
  | The object will create your processor
  | using the same createPluginFilter()
  | function that the other plugin wrappers
  | use, and will run it through the computer's
  | audio/MIDI devices using AudioDeviceManager
  | and AudioProcessorPlayer.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StandalonePluginHolder<'a> {
    base2:                                 Timer,
    settings:                              OptionalScopedPointer<PropertySet>,
    processor:                             Box<AudioProcessor<'a>>,
    device_manager:                        AudioDeviceManager<'a>,
    player:                                AudioProcessorPlayer,
    channel_configuration:                 Vec<StandalonePluginHolderPluginInOuts>,

    /**
       avoid feedback loop by default
      */
    processor_has_potential_feedback_loop: bool, // default = true

    mute_input:                            AtomicBool, // default = true 
    should_mute_input:                     Value<'a>,
    empty_buffer:                          AudioBuffer<f32>,
    auto_open_midi_devices:                bool,
    options:                               Box<AudioDeviceSetup>,
    last_midi_devices:                     Vec<MidiDeviceInfo>,
    state_file_chooser:                    Box<FileChooser<'a>>,
    max_size_enforcer:                     StandalonePluginHolderCallbackMaxSizeEnforcer<'a>, //{ *this };
}

impl<'a> ValueListener for StandalonePluginHolder<'a> {

    fn value_changed(&mut self, value: &mut Value)  {
        
        todo!();
        /*
            muteInput = (bool) value.getValue();
        */
    }
}

impl<'a> AudioIODeviceCallback for StandalonePluginHolder<'a> {

    fn audio_device_io_callback(
        &mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32

    )  {
        
        todo!();
        /*
            if (muteInput)
            {
                emptyBuffer.clear();
                inputChannelData = emptyBuffer.getArrayOfReadPointers();
            }

            player.audioDeviceIOCallback (inputChannelData, numInputChannels,
                                          outputChannelData, numOutputChannels, numSamples);
        */
    }
    
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            emptyBuffer.setSize (device->getActiveInputChannels().countNumberOfSetBits(), device->getCurrentBufferSizeSamples());
            emptyBuffer.clear();

            player.audioDeviceAboutToStart (device);
            player.setMidiOutput (deviceManager.getDefaultMidiOutput());
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            player.setMidiOutput (nullptr);
            player.audioDeviceStopped();
            emptyBuffer.setSize (0, 0);
        */
    }
}

impl<'a> Drop for StandalonePluginHolder<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stopTimer();

            deletePlugin();
            shutDownAudioDevices();
         */
    }
}

impl<'a> StandalonePluginHolder<'a> {

    /**
      | Creates an instance of the default plugin.
      | 
      | The settings object can be a PropertySet
      | that the class should use to store its
      | settings - the takeOwnershipOfSettings
      | indicates whether this object will
      | delete the settings automatically
      | when no longer needed. The settings
      | can also be nullptr.
      | 
      | A default device name can be passed in.
      | 
      | Preferably a complete setup options
      | object can be used, which takes precedence
      | over the preferredDefaultDeviceName
      | and allows you to select the input & output
      | device names, sample rate, buffer size
      | etc.
      | 
      | In all instances, the settingsToUse
      | will take precedence over the "preferred"
      | options if not null.
      |
      */
    pub fn new(
        settings_to_use:               *mut PropertySet,
        take_ownership_of_settings:    Option<bool>,
        preferred_default_device_name: Option<&str>,
        preferred_setup_options:       *const AudioDeviceSetup,
        channels:                      Option<&[StandalonePluginHolderPluginInOuts]>,
        should_auto_open_midi_devices: Option<bool>

    ) -> Self {

        let take_ownership_of_settings: bool =
            take_ownership_of_settings.unwrap_or(true);

        let preferred_default_device_name = preferred_default_device_name.unwrap_or("");

        #[cfg(any(target_os="android",target_os="ios"))]
        let should_auto_open_midi_devices: bool = should_auto_open_midi_devices.unwrap_or(true);

        #[cfg(not(any(target_os="android",target_os="ios")))]
        let should_auto_open_midi_devices: bool = should_auto_open_midi_devices.unwrap_or(false);

        let channels = channels.unwrap_or(&[]);

        todo!();

        /*


            : settings (settingsToUse, takeOwnershipOfSettings),
              channelConfiguration (channels),
              autoOpenMidiDevices (shouldAutoOpenMidiDevices)

            shouldMuteInput.addListener (this);
            shouldMuteInput = ! isInterAppAudioConnected();

            createPlugin();

            auto inChannels = (channelConfiguration.size() > 0 ? channelConfiguration[0].numIns
                                                               : processor->getMainBusNumInputChannels());

            if (preferredSetupOptions != nullptr)
                options.reset (new AudioDeviceManager::AudioDeviceSetup (*preferredSetupOptions));

            auto audioInputRequired = (inChannels > 0);

            if (audioInputRequired && RuntimePermissions::isRequired (RuntimePermissions::recordAudio)
                && ! RuntimePermissions::isGranted (RuntimePermissions::recordAudio))
                RuntimePermissions::request (RuntimePermissions::recordAudio,
                                             [this, preferredDefaultDeviceName] (bool granted) { init (granted, preferredDefaultDeviceName); });
            else
                init (audioInputRequired, preferredDefaultDeviceName);
        */
    }
    
    pub fn init(&mut self, 
        enable_audio_input:            bool,
        preferred_default_device_name: &String)  {
        
        todo!();
        /*
            setupAudioDevices (enableAudioInput, preferredDefaultDeviceName, options.get());
            reloadPluginState();
            startPlaying();

           if (autoOpenMidiDevices)
               startTimer (500);
        */
    }
    
    pub fn create_plugin(&mut self)  {
        
        todo!();
        /*
            processor.reset (createPluginFilterOfType (AudioProcessor::wrapperType_Standalone));
            processor->disableNonMainBuses();
            processor->setRateAndBufferSizeDetails (44100, 512);

            processorHasPotentialFeedbackLoop = (getNumInputChannels() > 0 && getNumOutputChannels() > 0);
        */
    }
    
    pub fn delete_plugin(&mut self)  {
        
        todo!();
        /*
            stopPlaying();
            processor = nullptr;
        */
    }
    
    pub fn get_num_input_channels(&self) -> i32 {
        
        todo!();
        /*
            if (processor == nullptr)
                return 0;

            return (channelConfiguration.size() > 0 ? channelConfiguration[0].numIns
                                                    : processor->getMainBusNumInputChannels());
        */
    }
    
    pub fn get_num_output_channels(&self) -> i32 {
        
        todo!();
        /*
            if (processor == nullptr)
                return 0;

            return (channelConfiguration.size() > 0 ? channelConfiguration[0].numOuts
                                                    : processor->getMainBusNumOutputChannels());
        */
    }
    
    pub fn get_file_patterns(file_suffix: &String) -> String {
        
        todo!();
        /*
            if (fileSuffix.isEmpty())
                return {};

            return (fileSuffix.startsWithChar ('.') ? "*" : "*.") + fileSuffix;
        */
    }
    
    pub fn get_mute_input_value(&mut self) -> &mut Value {
        
        todo!();
        /*
            return shouldMuteInput;
        */
    }
    
    pub fn get_processor_has_potential_feedback_loop(&self) -> bool {
        
        todo!();
        /*
            return processorHasPotentialFeedbackLoop;
        */
    }
    
    pub fn get_last_file(&self) -> File {
        
        todo!();
        /*
            File f;

            if (settings != nullptr)
                f = File (settings->getValue ("lastStateFile"));

            if (f == File())
                f = File::getSpecialLocation (File::userDocumentsDirectory);

            return f;
        */
    }
    
    pub fn set_last_file(&mut self, fc: &FileChooser)  {
        
        todo!();
        /*
            if (settings != nullptr)
                settings->setValue ("lastStateFile", fc.getResult().getFullPathName());
        */
    }

    /**
      | Pops up a dialog letting the user save
      | the processor's state to a file.
      |
      */
    pub fn ask_user_to_save_state(&mut self, file_suffix: Option<&str>)  {

        let file_suffix = file_suffix.unwrap_or("");

        todo!();
        /*
            stateFileChooser = std::make_unique<FileChooser> (TRANS("Save current state"),
                                                              getLastFile(),
                                                              getFilePatterns (fileSuffix));
            auto flags = FileBrowserComponent::saveMode
                       | FileBrowserComponent::canSelectFiles
                       | FileBrowserComponent::warnAboutOverwriting;

            stateFileChooser->launchAsync (flags, [this] (const FileChooser& fc)
            {
                if (fc.getResult() == File{})
                    return;

                setLastFile (fc);

                MemoryBlock data;
                processor->getStateInformation (data);

                if (! fc.getResult().replaceWithData (data.getData(), data.getSize()))
                    AlertWindow::showMessageBoxAsync (AlertWindow::WarningIcon,
                                                      TRANS("Error whilst saving"),
                                                      TRANS("Couldn't write to the specified file!"));
            });
        */
    }

    /**
      | Pops up a dialog letting the user re-load
      | the processor's state from a file.
      |
      */
    pub fn ask_user_to_load_state(&mut self, file_suffix: Option<&str>)  {

        let file_suffix = file_suffix.unwrap_or("");

        todo!();
        /*
            stateFileChooser = std::make_unique<FileChooser> (TRANS("Load a saved state"),
                                                              getLastFile(),
                                                              getFilePatterns (fileSuffix));
            auto flags = FileBrowserComponent::openMode
                       | FileBrowserComponent::canSelectFiles;

            stateFileChooser->launchAsync (flags, [this] (const FileChooser& fc)
            {
                if (fc.getResult() == File{})
                    return;

                setLastFile (fc);

                MemoryBlock data;

                if (fc.getResult().loadFileAsData (data))
                    processor->setStateInformation (data.getData(), (int) data.getSize());
                else
                    AlertWindow::showMessageBoxAsync (AlertWindow::WarningIcon,
                                                      TRANS("Error whilst loading"),
                                                      TRANS("Couldn't read from the specified file!"));
            });
        */
    }
    
    pub fn start_playing(&mut self)  {
        
        todo!();
        /*
            player.setProcessor (processor.get());

           #if AloePlugin_Enable_IAA && ALOE_IOS
            if (auto device = dynamic_cast<iOSAudioIODevice*> (deviceManager.getCurrentAudioDevice()))
            {
                processor->setPlayHead (device->getAudioPlayHead());
                device->setMidiMessageCollector (&player.getMidiMessageCollector());
            }
           #endif
        */
    }
    
    pub fn stop_playing(&mut self)  {
        
        todo!();
        /*
            player.setProcessor (nullptr);
        */
    }
    
    /**
      | Shows an audio properties dialog box
      | modally.
      |
      */
    pub fn show_audio_settings_dialog(&mut self)  {
        
        todo!();
        /*
            DialogWindowLaunchOptions o;

            int maxNumInputs = 0, maxNumOutputs = 0;

            if (channelConfiguration.size() > 0)
            {
                auto& defaultConfig = channelConfiguration.getReference (0);

                maxNumInputs  = jmax (0, (int) defaultConfig.numIns);
                maxNumOutputs = jmax (0, (int) defaultConfig.numOuts);
            }

            if (auto* bus = processor->getBus (true, 0))
                maxNumInputs = jmax (0, bus->getDefaultLayout().size());

            if (auto* bus = processor->getBus (false, 0))
                maxNumOutputs = jmax (0, bus->getDefaultLayout().size());

            auto content = std::make_unique<StandalonePluginHolderSettingsComponent> (*this, deviceManager, maxNumInputs, maxNumOutputs);
            content->setSize (500, 550);
            content->setToRecommendedSize();

            o.content.setOwned (content.release());

            o.dialogTitle                   = TRANS("Audio/MIDI Settings");
            o.dialogBackgroundColour        = o.content->getLookAndFeel().findColour (ResizableWindow::backgroundColourId);
            o.escapeKeyTriggersCloseButton  = true;
            o.useNativeTitleBar             = true;
            o.resizable                     = false;

            o.launchAsync();
        */
    }
    
    pub fn save_audio_device_state(&mut self)  {
        
        todo!();
        /*
            if (settings != nullptr)
            {
                auto xml = deviceManager.createStateXml();

                settings->setValue ("audioSetup", xml.get());

               #if ! (ALOE_IOS || ALOE_ANDROID)
                settings->setValue ("shouldMuteInput", (bool) shouldMuteInput.getValue());
               #endif
            }
        */
    }
    
    pub fn reload_audio_device_state(
        &mut self, 
        enable_audio_input:            bool,
        preferred_default_device_name: &String,
        preferred_setup_options:       *const AudioDeviceSetup

    ) {
        
        todo!();
        /*
            std::unique_ptr<XmlElement> savedState;

            if (settings != nullptr)
            {
                savedState = settings->getXmlValue ("audioSetup");

               #if ! (ALOE_IOS || ALOE_ANDROID)
                shouldMuteInput.setValue (settings->getBoolValue ("shouldMuteInput", true));
               #endif
            }

            auto inputChannels  = getNumInputChannels();
            auto outputChannels = getNumOutputChannels();

            if (inputChannels == 0 && outputChannels == 0 && processor->isMidiEffect())
            {
                // add a dummy output channel for MIDI effect plug-ins so they can receive audio callbacks
                outputChannels = 1;
            }

            deviceManager.initialise (enableAudioInput ? inputChannels : 0,
                                      outputChannels,
                                      savedState.get(),
                                      true,
                                      preferredDefaultDeviceName,
                                      preferredSetupOptions);
        */
    }
    
    pub fn save_plugin_state(&mut self)  {
        
        todo!();
        /*
            if (settings != nullptr && processor != nullptr)
            {
                MemoryBlock data;
                processor->getStateInformation (data);

                settings->setValue ("filterState", data.toBase64Encoding());
            }
        */
    }
    
    pub fn reload_plugin_state(&mut self)  {
        
        todo!();
        /*
            if (settings != nullptr)
            {
                MemoryBlock data;

                if (data.fromBase64Encoding (settings->getValue ("filterState")) && data.getSize() > 0)
                    processor->setStateInformation (data.getData(), (int) data.getSize());
            }
        */
    }
    
    pub fn switch_to_host_application(&mut self)  {
        
        todo!();
        /*
            #if ALOE_IOS
            if (auto device = dynamic_cast<iOSAudioIODevice*> (deviceManager.getCurrentAudioDevice()))
                device->switchApplication();
           #endif
        */
    }
    
    pub fn is_inter_app_audio_connected(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_IOS
            if (auto device = dynamic_cast<iOSAudioIODevice*> (deviceManager.getCurrentAudioDevice()))
                return device->isInterAppAudioConnected();
           #endif

            return false;
        */
    }
    
    pub fn get_iaa_host_icon(&mut self, size: i32) -> Image {
        
        todo!();
        /*
            #if ALOE_IOS && AloePlugin_Enable_IAA
            if (auto device = dynamic_cast<iOSAudioIODevice*> (deviceManager.getCurrentAudioDevice()))
                return device->getIcon (size);
           #else
            ignoreUnused (size);
           #endif

            return {};
        */
    }
    
    
    pub fn setup_audio_devices(
        &mut self, 
        enable_audio_input:            bool,
        preferred_default_device_name: &String,
        preferred_setup_options:       *const AudioDeviceSetup

    ) {
        
        todo!();
        /*
            deviceManager.addAudioCallback (&maxSizeEnforcer);
            deviceManager.addMidiInputDeviceCallback ({}, &player);

            reloadAudioDeviceState (enableAudioInput, preferredDefaultDeviceName, preferredSetupOptions);
        */
    }
    
    pub fn shut_down_audio_devices(&mut self)  {
        
        todo!();
        /*
            saveAudioDeviceState();

            deviceManager.removeMidiInputDeviceCallback ({}, &player);
            deviceManager.removeAudioCallback (&maxSizeEnforcer);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto newMidiDevices = MidiInput::getAvailableDevices();

            if (newMidiDevices != lastMidiDevices)
            {
                for (auto& oldDevice : lastMidiDevices)
                    if (! newMidiDevices.contains (oldDevice))
                        deviceManager.setMidiInputDeviceEnabled (oldDevice.identifier, false);

                for (auto& newDevice : newMidiDevices)
                    if (! lastMidiDevices.contains (newDevice))
                        deviceManager.setMidiInputDeviceEnabled (newDevice.identifier, true);

                lastMidiDevices = newMidiDevices;
            }
        */
    }
    
    #[inline] pub fn get_instance(&mut self) -> *mut StandalonePluginHolder {
        
        todo!();
        /*
            #if AloePlugin_Enable_IAA || AloePlugin_Build_Standalone
        if (PluginHostType::getPluginLoadedAs() == AudioProcessor::wrapperType_Standalone)
        {
            auto& desktop = Desktop::getInstance();
            const int numTopLevelWindows = desktop.getNumComponents();

            for (int i = 0; i < numTopLevelWindows; ++i)
                if (auto window = dynamic_cast<StandaloneFilterWindow*> (desktop.getComponent (i)))
                    return window->getPluginHolder();
        }
       #endif

        return nullptr;
        */
    }
}
