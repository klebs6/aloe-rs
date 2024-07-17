crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioPlaybackDemo.h]

#[no_copy]
#[leak_detector]
pub struct AudioPlaybackDemo<'a> {

    base: Component<'a>,

    #[cfg(any(target_os="android",target_os="ios"))]
    base2: ButtonListener,


    /**
      | if this PIP is running inside the demo
      | runner, we'll use the shared device
      | manager instead
      |
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager: AudioDeviceManager<'a>,

    /**
      | default = { getSharedAudioDeviceManager
      | (0, 2) };
      |
      */
    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager: &mut AudioDeviceManager, 

    format_manager:       AudioFormatManager,
    thread:               TimeSliceThread,         // default = "audio file preview" 

    ///-------------------
    #[cfg(any(target_os="android",target_os="ios"))]
    file_chooser:       Box<FileChooser>,

    /**
      | default = {"Choose Audio File...",
      | "Choose an audio file for playback"};
      |
      */
    #[cfg(any(target_os="android",target_os="ios"))]
    choose_file_button: TextButton, 

    ///-------------------
    #[cfg(not(any(target_os="android",target_os="ios")))]
    directory_list: DirectoryContentsList<'a>, // {nullptr, thread};

    #[cfg(not(any(target_os="android",target_os="ios")))]
    file_tree_comp: FileTreeComponent<'a>,     // {directoryList};

    /**
      | default = { {}, "Select an audio file
      | in the treeview above, and this page
      | will display its waveform, and let you
      | play it.." };
      |
      */
    #[cfg(not(any(target_os="android",target_os="ios")))]
    explanation:    Label<'a>,                

    current_audio_file:        Url,
    audio_source_player:       AudioSourcePlayer,
    transport_source:          AudioTransportSource<'a>,
    current_audio_file_source: Box<AudioFormatReaderSource<'a>>,
    thumbnail:                 Box<DemoThumbnailComp<'a>>,

    zoom_label:                Label<'a>,        // { {}, "zoom:" };
    zoom_slider:               Slider<'a>,       // { Slider::LinearHorizontal, Slider::NoTextBox };
    follow_transport_button:   ToggleButton<'a>, // { "Follow Transport" };
    start_stop_button:         TextButton<'a>,   // { "Play/Stop" };
}

impl<'a> ChangeListener for AudioPlaybackDemo<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == thumbnail.get())
                showAudioResource (Url (thumbnail->getLastDroppedFile()));
        */
    }
}

#[cfg(not(any(target_os="android",target_os="ios")))]
impl<'a> FileBrowserListener for AudioPlaybackDemo<'a> {

    fn selection_changed(&mut self)  {
        
        todo!();
        /*
            showAudioResource (Url (fileTreeComp.getSelectedFile()));
        */
    }
    
    fn file_clicked(&mut self, 
        _0: &File,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    fn file_double_clicked(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
    
    fn browser_root_changed(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Default for AudioPlaybackDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            addAndMakeVisible (zoomLabel);
            zoomLabel.setFont (Font (15.00f, Font::plain));
            zoomLabel.setJustificationType (Justification::centredRight);
            zoomLabel.setEditable (false, false, false);
            zoomLabel.setColour (TextEditor::textColourId, Colours::black);
            zoomLabel.setColour (TextEditor::backgroundColourId, Colour (0x00000000));

            addAndMakeVisible (followTransportButton);
            followTransportButton.onClick = [this] { updateFollowTransportState(); };

           #if (ALOE_ANDROID || ALOE_IOS)
            addAndMakeVisible (chooseFileButton);
            chooseFileButton.addListener (this);
           #else
            addAndMakeVisible (fileTreeComp);

            directoryList.setDirectory (File::getSpecialLocation (File::userHomeDirectory), true, true);

            fileTreeComp.setTitle ("Files");
            fileTreeComp.setColour (FileTreeComponent::backgroundColourId, Colours::lightgrey.withAlpha (0.6f));
            fileTreeComp.addListener (this);

            addAndMakeVisible (explanation);
            explanation.setFont (Font (14.00f, Font::plain));
            explanation.setJustificationType (Justification::bottomRight);
            explanation.setEditable (false, false, false);
            explanation.setColour (TextEditor::textColourId, Colours::black);
            explanation.setColour (TextEditor::backgroundColourId, Colour (0x00000000));
           #endif

            addAndMakeVisible (zoomSlider);
            zoomSlider.setRange (0, 1, 0);
            zoomSlider.onValueChange = [this] { thumbnail->setZoomFactor (zoomSlider.getValue()); };
            zoomSlider.setSkewFactor (2);

            thumbnail.reset (new DemoThumbnailComp (formatManager, transportSource, zoomSlider));
            addAndMakeVisible (thumbnail.get());
            thumbnail->addChangeListener (this);

            addAndMakeVisible (startStopButton);
            startStopButton.setColour (TextButton::buttonColourId, Colour (0xff79ed7f));
            startStopButton.setColour (TextButton::textColourOffId, Colours::black);
            startStopButton.onClick = [this] { startOrStop(); };

            // audio setup
            formatManager.registerBasicFormats();

            thread.startThread (3);

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             audioDeviceManager.initialise (numInputChannels, 2, nullptr, true, {}, nullptr);
                                         });
           #endif

            audioDeviceManager.addAudioCallback (&audioSourcePlayer);
            audioSourcePlayer.setSource (&transportSource);

            setOpaque (true);
            setSize (500, 500)
        */
    }
}

impl<'a> Drop for AudioPlaybackDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            transportSource  .setSource (nullptr);
            audioSourcePlayer.setSource (nullptr);

            audioDeviceManager.removeAudioCallback (&audioSourcePlayer);

           #if (ALOE_ANDROID || ALOE_IOS)
            chooseFileButton.removeListener (this);
           #else
            fileTreeComp.removeListener (this);
           #endif

            thumbnail->removeChangeListener (this);
         */
    }
}

impl<'a> AudioPlaybackDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4);

            auto controls = r.removeFromBottom (90);

            auto controlRightBounds = controls.removeFromRight (controls.getWidth() / 3);

           #if (ALOE_ANDROID || ALOE_IOS)
            chooseFileButton.setBounds (controlRightBounds.reduced (10));
           #else
            explanation.setBounds (controlRightBounds);
           #endif

            auto zoom = controls.removeFromTop (25);
            zoomLabel .setBounds (zoom.removeFromLeft (50));
            zoomSlider.setBounds (zoom);

            followTransportButton.setBounds (controls.removeFromTop (25));
            startStopButton      .setBounds (controls);

            r.removeFromBottom (6);

           #if ALOE_ANDROID || ALOE_IOS
            thumbnail->setBounds (r);
           #else
            thumbnail->setBounds (r.removeFromBottom (140));
            r.removeFromBottom (6);

            fileTreeComp.setBounds (r);
           #endif
        */
    }
    
    pub fn show_audio_resource(&mut self, resource: Url)  {
        
        todo!();
        /*
            if (loadURLIntoTransport (resource))
                currentAudioFile = std::move (resource);

            zoomSlider.setValue (0, dontSendNotification);
            thumbnail->setURL (currentAudioFile);
        */
    }
    
    pub fn load_url_into_transport(&mut self, audiourl: &Url) -> bool {
        
        todo!();
        /*
            // unload the previous file source and delete it..
            transportSource.stop();
            transportSource.setSource (nullptr);
            currentAudioFileSource.reset();

            AudioFormatReader* reader = nullptr;

           #if ! ALOE_IOS
            if (audioURL.isLocalFile())
            {
                reader = formatManager.createReaderFor (audioURL.getLocalFile());
            }
            else
           #endif
            {
                if (reader == nullptr)
                    reader = formatManager.createReaderFor (audioURL.createInputStream (false));
            }

            if (reader != nullptr)
            {
                currentAudioFileSource.reset (new AudioFormatReaderSource (reader, true));

                // ..and plug it into our transport source
                transportSource.setSource (currentAudioFileSource.get(),
                                           32768,                   // tells it to buffer this many samples ahead
                                           &thread,                 // this is the background thread to use for reading-ahead
                                           reader->sampleRate);     // allows for sample rate correction

                return true;
            }

            return false;
        */
    }
    
    pub fn start_or_stop(&mut self)  {
        
        todo!();
        /*
            if (transportSource.isPlaying())
            {
                transportSource.stop();
            }
            else
            {
                transportSource.setPosition (0);
                transportSource.start();
            }
        */
    }
    
    pub fn update_follow_transport_state(&mut self)  {
        
        todo!();
        /*
            thumbnail->setFollowsTransport (followTransportButton.getToggleState());
        */
    }

    #[cfg(any(target_os="android",target_os="ios"))]
    pub fn button_clicked(&mut self, btn: *mut Button)  {
        
        todo!();
        /*
            if (btn == &chooseFileButton && fileChooser.get() == nullptr)
            {
                if (! RuntimePermissions::isGranted (RuntimePermissions::readExternalStorage))
                {
                    SafePointer<AudioPlaybackDemo> safeThis (this);
                    RuntimePermissions::request (RuntimePermissions::readExternalStorage,
                                                 [safeThis] (bool granted) mutable
                                                 {
                                                     if (safeThis != nullptr && granted)
                                                         safeThis->buttonClicked (&safeThis->chooseFileButton);
                                                 });
                    return;
                }

                if (FileChooser::isPlatformDialogAvailable())
                {
                    fileChooser.reset (new FileChooser ("Select an audio file...", File(), "*.wav;*.mp3;*.aif"));

                    fileChooser->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles,
                                              [this] (const FileChooser& fc) mutable
                                              {
                                                  if (fc.getURLResults().size() > 0)
                                                  {
                                                      auto u = fc.getURLResult();

                                                      showAudioResource (std::move (u));
                                                  }

                                                  fileChooser = nullptr;
                                              }, nullptr);
                }
                else
                {
                    NativeMessageBox::showAsync (MessageBoxOptions()
                                                   .withIconType (MessageBoxIconType::WarningIcon)
                                                   .withTitle ("Enable Code Signing")
                                                   .withMessage ("You need to enable code-signing for your iOS project and enable \"iCloud Documents\" "
                                                                 "permissions to be able to open audio files on your iDevice. See: "
                                                                 "https://forum.aloe.com/t/native-ios-android-file-choosers"),
                                                 nullptr);
                }
            }
        */
    }
}
