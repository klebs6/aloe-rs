crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioRecordingDemo.h]

///------------------
#[no_copy]
#[leak_detector]
pub struct AudioRecordingDemo<'a> {

    base: Component<'a>,

    /**
      | if this PIP is running inside the demo
      | runner, we'll use the shared device
      | manager instead
      |
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager: AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager: &mut AudioDeviceManager, // { getSharedAudioDeviceManager (1, 0) };

    live_audio_scroller: LiveScrollingAudioDisplay<'a>,
    recording_thumbnail: RecordingThumbnail<'a>,
    recorder:            AudioRecorder<'a>, // default { recordingThumbnail.getAudioThumbnail() };

    /**
    default = { {}, "This page demonstrates how to record a wave file from the live audio input..\n\n"
         #if (ALOE_ANDROID || ALOE_IOS)
          "After you are done with your recording you can share with other apps."
         #else
          "Pressing record will start recording a file in your \"Documents\" folder."
         #endif
     };
    */
    explanation_label:   Label<'a>,

    record_button:       TextButton<'a>, // default = "Record" 
    last_recording:      File,
}

impl<'a> Default for AudioRecordingDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);
            addAndMakeVisible (liveAudioScroller);

            addAndMakeVisible (explanationLabel);
            explanationLabel.setFont (Font (15.0f, Font::plain));
            explanationLabel.setJustificationType (Justification::topLeft);
            explanationLabel.setEditable (false, false, false);
            explanationLabel.setColour (TextEditor::textColourId, Colours::black);
            explanationLabel.setColour (TextEditor::backgroundColourId, Colour (0x00000000));

            addAndMakeVisible (recordButton);
            recordButton.setColour (TextButton::buttonColourId, Colour (0xffff5c5c));
            recordButton.setColour (TextButton::textColourOnId, Colours::black);

            recordButton.onClick = [this]
            {
                if (recorder.isRecording())
                    stopRecording();
                else
                    startRecording();
            };

            addAndMakeVisible (recordingThumbnail);

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             audioDeviceManager.initialise (numInputChannels, 2, nullptr, true, {}, nullptr);
                                         });
           #endif

            audioDeviceManager.addAudioCallback (&liveAudioScroller);
            audioDeviceManager.addAudioCallback (&recorder);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for AudioRecordingDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            audioDeviceManager.removeAudioCallback (&recorder);
            audioDeviceManager.removeAudioCallback (&liveAudioScroller);
         */
    }
}

impl<'a> AudioRecordingDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            liveAudioScroller .setBounds (area.removeFromTop (80).reduced (8));
            recordingThumbnail.setBounds (area.removeFromTop (80).reduced (8));
            recordButton      .setBounds (area.removeFromTop (36).removeFromLeft (140).reduced (8));
            explanationLabel  .setBounds (area.reduced (8));
        */
    }
    
    pub fn start_recording(&mut self)  {
        
        todo!();
        /*
            if (! RuntimePermissions::isGranted (RuntimePermissions::writeExternalStorage))
            {
                SafePointer<AudioRecordingDemo> safeThis (this);

                RuntimePermissions::request (RuntimePermissions::writeExternalStorage,
                                             [safeThis] (bool granted) mutable
                                             {
                                                 if (granted)
                                                     safeThis->startRecording();
                                             });
                return;
            }

           #if (ALOE_ANDROID || ALOE_IOS)
            auto parentDir = File::getSpecialLocation (File::tempDirectory);
           #else
            auto parentDir = File::getSpecialLocation (File::userDocumentsDirectory);
           #endif

            lastRecording = parentDir.getNonexistentChildFile ("Aloe Demo Audio Recording", ".wav");

            recorder.startRecording (lastRecording);

            recordButton.setButtonText ("Stop");
            recordingThumbnail.setDisplayFullThumbnail (false);
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            recorder.stop();

           #if ALOE_CONTENT_SHARING
            SafePointer<AudioRecordingDemo> safeThis (this);
            File fileToShare = lastRecording;

            ContentSharer::getInstance()->shareFiles (Vec<Url> ({Url (fileToShare)}),
                                                      [safeThis, fileToShare] (bool success, const String& error)
                                                      {
                                                          if (fileToShare.existsAsFile())
                                                              fileToShare.deleteFile();

                                                          if (! success && error.isNotEmpty())
                                                              NativeMessageBox::showAsync (MessageBoxOptions()
                                                                                             .withIconType (MessageBoxIconType::WarningIcon)
                                                                                             .withTitle ("Sharing Error")
                                                                                             .withMessage (error),
                                                                                           nullptr);
                                                      });
           #endif

            lastRecording = File();
            recordButton.setButtonText ("Record");
            recordingThumbnail.setDisplayFullThumbnail (true);
        */
    }
}
