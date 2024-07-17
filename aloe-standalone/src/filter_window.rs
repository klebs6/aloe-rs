crate::ix!();

/**
  | A class that can be used to run a simple
  | standalone application containing
  | your filter.
  | 
  | Just create one of these objects in your
  | ALOEApplicationBase::initialise()
  | method, and let it do its work. It will
  | create your filter object using the
  | same createPluginFilter() function
  | that the other plugin wrappers use.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StandaloneFilterWindow<'a> {
    base:           DocumentWindow<'a>,
    plugin_holder:  Box<StandalonePluginHolder<'a>>,
    options_button: TextButton<'a>,
}

impl<'a> ButtonListener for StandaloneFilterWindow<'a> {

    fn button_clicked(&mut self, _0: *mut Button)  {
        
        todo!();
        /*
            PopupMenu m;
            m.addItem (1, TRANS("Audio/MIDI Settings..."));
            m.addSeparator();
            m.addItem (2, TRANS("Save current state..."));
            m.addItem (3, TRANS("Load a saved state..."));
            m.addSeparator();
            m.addItem (4, TRANS("Reset to default state"));

            m.showMenuAsync (PopupMenu::Options(),
                             ModalCallbackFunction::forComponent (menuCallback, this));
        */
    }
}

impl<'a> Drop for StandaloneFilterWindow<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
           #if (! ALOE_IOS) && (! ALOE_ANDROID)
            if (auto* props = pluginHolder->settings.get())
            {
                props->setValue ("windowX", getX());
                props->setValue ("windowY", getY());
            }
           #endif

            pluginHolder->stopPlaying();
            clearContentComponent();
            pluginHolder = nullptr;
         */
    }
}

impl<'a> StandaloneFilterWindow<'a> {

    /**
      | Creates a window with a given title and
      | colour.
      | 
      | The settings object can be a PropertySet
      | that the class should use to store its
      | settings (it can also be null). If takeOwnershipOfSettings
      | is true, then the settings object will
      | be owned and deleted by this object.
      |
      */
    pub fn new(
        title:                         &String,
        background_colour:             Colour,
        settings_to_use:               *mut PropertySet,
        take_ownership_of_settings:    bool,
        preferred_default_device_name: Option<&str>,
        preferred_setup_options:       *const AudioDeviceSetup,
        constrain_to_configuration:    &[StandalonePluginHolderPluginInOuts],
        auto_open_midi_devices:        bool

    ) -> Self {

        let preferred_default_device_name = preferred_default_device_name.unwrap_or("");

        #[cfg(any(target_os="android",target_os="ios"))]
        let auto_open_midi_devices: bool =
            auto_open_midi_devices.unwrap_or(true);

        #[cfg(any(target_os="android",target_os="ios"))]
        let auto_open_midi_devices: bool =
            auto_open_midi_devices.unwrap_or(false);

        todo!();

        /*
            : DocumentWindow (title, backgroundColour, DocumentWindow::minimiseButton | DocumentWindow::closeButton),
              optionsButton ("Options")

           #if ALOE_IOS || ALOE_ANDROID
            setTitleBarHeight (0);
           #else
            setTitleBarButtonsRequired (DocumentWindow::minimiseButton | DocumentWindow::closeButton, false);

            Component::addAndMakeVisible (optionsButton);
            optionsButton.addListener (this);
            optionsButton.setTriggeredOnMouseDown (true);
           #endif

            pluginHolder.reset (new StandalonePluginHolder (settingsToUse, takeOwnershipOfSettings,
                                                            preferredDefaultDeviceName, preferredSetupOptions,
                                                            constrainToConfiguration, autoOpenMidiDevices));

           #if ALOE_IOS || ALOE_ANDROID
            setFullScreen (true);
            setContentOwned (new StandaloneFilterWindowMainContentComponent (*this), false);
           #else
            setContentOwned (new StandaloneFilterWindowMainContentComponent (*this), true);

            const auto windowScreenBounds = [this]() -> Rectangle<int>
            {
                const auto width = getWidth();
                const auto height = getHeight();

                const auto& displays = Desktop::getInstance().getDisplays();

                if (auto* props = pluginHolder->settings.get())
                {
                    constexpr int defaultValue = -100;

                    const auto x = props->getIntValue ("windowX", defaultValue);
                    const auto y = props->getIntValue ("windowY", defaultValue);

                    if (x != defaultValue && y != defaultValue)
                    {
                        const auto screenLimits = displays.getDisplayForRect ({ x, y, width, height })->userArea;

                        return { jlimit (screenLimits.getX(), jmax (screenLimits.getX(), screenLimits.getRight()  - width),  x),
                                 jlimit (screenLimits.getY(), jmax (screenLimits.getY(), screenLimits.getBottom() - height), y),
                                 width, height };
                    }
                }

                const auto displayArea = displays.getPrimaryDisplay()->userArea;

                return { displayArea.getCentreX() - width / 2,
                         displayArea.getCentreY() - height / 2,
                         width, height };
            }();

            setBoundsConstrained (windowScreenBounds);

            if (auto* processor = getAudioProcessor())
                if (auto* editor = processor->getActiveEditor())
                    setResizable (editor->isResizable(), false);
           #endif
        */
    }
    
    pub fn get_audio_processor(&self) -> *mut AudioProcessor {
        
        todo!();
        /*
            return pluginHolder->processor.get();
        */
    }
    
    pub fn get_device_manager(&self) -> &mut AudioDeviceManager {
        
        todo!();
        /*
            return pluginHolder->deviceManager;
        */
    }

    /**
      | Deletes and re-creates the plugin,
      | resetting it to its default state.
      |
      */
    pub fn reset_to_default_state(&mut self)  {
        
        todo!();
        /*
            pluginHolder->stopPlaying();
            clearContentComponent();
            pluginHolder->deletePlugin();

            if (auto* props = pluginHolder->settings.get())
                props->removeValue ("filterState");

            pluginHolder->createPlugin();
            setContentOwned (new StandaloneFilterWindowMainContentComponent (*this), true);
            pluginHolder->startPlaying();
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            pluginHolder->savePluginState();

            ALOEApplicationBase::quit();
        */
    }
    
    pub fn handle_menu_result(&mut self, result: i32)  {
        
        todo!();
        /*
            switch (result)
            {
                case 1:  pluginHolder->showAudioSettingsDialog(); break;
                case 2:  pluginHolder->askUserToSaveState(); break;
                case 3:  pluginHolder->askUserToLoadState(); break;
                case 4:  resetToDefaultState(); break;
                default: break;
            }
        */
    }
    
    pub fn menu_callback(
        result: i32,
        button: *mut StandaloneFilterWindow)  {
        
        todo!();
        /*
            if (button != nullptr && result != 0)
                button->handleMenuResult (result);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            DocumentWindow::resized();
            optionsButton.setBounds (8, 6, 60, getTitleBarHeight() - 8);
        */
    }
    
    pub fn get_plugin_holder(&mut self) -> *mut StandalonePluginHolder {
        
        todo!();
        /*
            return pluginHolder.get();
        */
    }
    
}
