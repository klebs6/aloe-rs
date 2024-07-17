crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Standalone/aloe_StandaloneFilterApp.cpp]

/**
  | You can set this flag in your build if you need
  | to specify a different standalone
  | ALOEApplication class for your app to use. If
  | you don't set it then by default we'll just
  | create a simple one as below.
  */
#[cfg(not(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP))]
pub mod not_custom_plugin_standalone_app {
    use super::*;

    pub struct StandaloneFilterApp<'a> {
        base:           ALOEApplication,
        app_properties: ApplicationProperties<'a>,
        main_window:    Box<StandaloneFilterWindow<'a>>,
    }

    impl<'a> Default for StandaloneFilterApp<'a> {
        
        fn default() -> Self {
            todo!();
            /*


                PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_Standalone;

                    PropertiesFile::Options options;

                    options.applicationName     = getApplicationName();
                    options.filenameSuffix      = ".settings";
                    options.osxLibrarySubFolder = "Application Support";
                   #if ALOE_LINUX || ALOE_BSD
                    options.folderName          = "~/.config";
                   #else
                    options.folderName          = "";
                   #endif

                    appProperties.setStorageParameters (options)
            */
        }
    }

    impl<'a> StandaloneFilterApp<'a> {

        
        pub fn get_application_name(&mut self) -> String {
            
            todo!();
            /*
                return AloePlugin_Name;
            */
        }
        
        pub fn get_application_version(&mut self) -> String {
            
            todo!();
            /*
                return AloePlugin_VersionString;
            */
        }
        
        pub fn more_than_one_instance_allowed(&mut self) -> bool {
            
            todo!();
            /*
                return true;
            */
        }
        
        pub fn another_instance_started(&mut self, _0: &String)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn create_window(&mut self) -> *mut StandaloneFilterWindow {
            
            todo!();
            /*
                #ifdef AloePlugin_PreferredChannelConfigurations
                    StandalonePluginHolder::PluginInOuts channels[] = { AloePlugin_PreferredChannelConfigurations };
                   #endif

                    return new StandaloneFilterWindow (getApplicationName(),
                                                       LookAndFeel::getDefaultLookAndFeel().findColour (ResizableWindow::backgroundColourId),
                                                       appProperties.getUserSettings(),
                                                       false, {}, nullptr
                                                      #ifdef AloePlugin_PreferredChannelConfigurations
                                                       , Vec<StandalonePluginHolder::PluginInOuts> (channels, numElementsInArray (channels))
                                                      #else
                                                       , {}
                                                      #endif
                                                      #if ALOE_DONT_AUTO_OPEN_MIDI_DEVICES_ON_MOBILE
                                                       , false
                                                      #endif
                                                       );
            */
        }
        
        pub fn initialise(&mut self, _0: &String)  {
            
            todo!();
            /*
                mainWindow.reset (createWindow());

                   #if ALOE_STANDALONE_FILTER_WINDOW_USE_KIOSK_MODE
                    Desktop::getInstance().setKioskModeComponent (mainWindow.get(), false);
                   #endif

                    mainWindow->setVisible (true);
            */
        }
        
        pub fn shutdown(&mut self)  {
            
            todo!();
            /*
                mainWindow = nullptr;
                    appProperties.saveIfNeeded();
            */
        }
        
        pub fn system_requested_quit(&mut self)  {
            
            todo!();
            /*
                if (mainWindow.get() != nullptr)
                        mainWindow->pluginHolder->savePluginState();

                    if (ModalComponentManager::getInstance()->cancelAllModalComponents())
                    {
                        Timer::callAfterDelay (100, []()
                        {
                            if (auto app = ALOEApplicationBase::getInstance())
                                app->systemRequestedQuit();
                        });
                    }
                    else
                    {
                        quit();
                    }
            */
        }
    }

    #[cfg(all(AloePlugin_Build_Standalone,target_os="ios"))]
    pub fn aloe_is_inter_app_audio_connected() -> bool {
        
        todo!();
        /*
            if (auto holder = StandalonePluginHolder::getInstance())
                    return holder->isInterAppAudioConnected();

                return false;
        */
    }

    #[cfg(all(AloePlugin_Build_Standalone,target_os="ios"))]
    pub fn aloe_switch_to_host_application()  {
        
        todo!();
        /*
            if (auto holder = StandalonePluginHolder::getInstance())
                    holder->switchToHostApplication();
        */
    }

    #[cfg(all(AloePlugin_Build_Standalone,target_os="ios"))]
    pub fn aloe_get_iaa_host_icon(size: i32) -> Image {
        
        todo!();
        /*
            if (auto holder = StandalonePluginHolder::getInstance())
                    return holder->getIAAHostIcon (size);

                return Image();
        */
    }
}
