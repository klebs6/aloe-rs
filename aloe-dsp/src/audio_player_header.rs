crate::ix!();

pub struct AudioPlayerHeader<'a> {
    base:              Component<'a>,
    thumbnail_comp:    AudioThumbnailComponent<'a>,
    load_button:       TextButton<'a>, // default = "Load File..." 
    play_button:       TextButton<'a>, // default = "Play" 
    loop_button:       ToggleButton<'a>, // default = "Loop File" 
    audio_file_reader: &'a mut AudioFileReaderComponent<'a>,
    file_chooser:      Box<FileChooser<'a>>,
}

impl<'a> ChangeListener for AudioPlayerHeader<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (audioFileReader.playState.getValue())
                    audioFileReader.stop();

                audioFileReader.loadURL (thumbnailComp.getCurrentURL());
        */
    }
}

impl<'a> ValueListener  for AudioPlayerHeader<'a> {
    
    fn value_changed(&mut self, v: &mut Value)  {
        
        todo!();
        /*
            playButton.setButtonText (v.getValue() ? "Stop" : "Play");
                playButton.setColour (TextButton::buttonColourId, v.getValue() ? Colour (0xffed797f) : Colour (0xff79ed7f));
        */
    }
}

impl<'a> Drop for AudioPlayerHeader<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                audioFileReader.playState.removeListener (this);
             */
    }
}

impl<'a> AudioPlayerHeader<'a> {

    pub fn new(
        adm: &mut AudioDeviceManager,
        afm: &mut AudioFormatManager,
        afr: &mut AudioFileReaderComponent) -> Self {
    
        todo!();
        /*
        : thumbnail_comp(adm, afm),
        : audio_file_reader(afr),

            setOpaque (true);

                addAndMakeVisible (loadButton);
                addAndMakeVisible (playButton);
                addAndMakeVisible (loopButton);

                playButton.setColour (TextButton::buttonColourId, Colour (0xff79ed7f));
                playButton.setColour (TextButton::textColourOffId, Colours::black);

                loadButton.setColour (TextButton::buttonColourId, Colour (0xff797fed));
                loadButton.setColour (TextButton::textColourOffId, Colours::black);

                loadButton.onClick = [this] { openFile(); };
                playButton.onClick = [this] { audioFileReader.togglePlay(); };

                addAndMakeVisible (thumbnailComp);
                thumbnailComp.addChangeListener (this);

                audioFileReader.playState.addListener (this);
                loopButton.getToggleStateValue().referTo (audioFileReader.loopState);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (getLookAndFeel().findColour (ResizableWindow::backgroundColourId).darker());
                g.fillRect (getLocalBounds());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                auto buttonBounds = bounds.removeFromLeft (jmin (250, bounds.getWidth() / 4));
                auto loopBounds = buttonBounds.removeFromBottom (30);

                loadButton.setBounds (buttonBounds.removeFromTop (buttonBounds.getHeight() / 2));
                playButton.setBounds (buttonBounds);

                loopButton.setSize (0, 25);
                loopButton.changeWidthToFitText();
                loopButton.setCentrePosition (loopBounds.getCentre());

                thumbnailComp.setBounds (bounds);
        */
    }
    
    pub fn open_file(&mut self)  {
        
        todo!();
        /*
            audioFileReader.stop();

                if (fileChooser != nullptr)
                    return;

                if (! RuntimePermissions::isGranted (RuntimePermissions::readExternalStorage))
                {
                    SafePointer<AudioPlayerHeader> safeThis (this);
                    RuntimePermissions::request (RuntimePermissions::readExternalStorage,
                        [safeThis] (bool granted) mutable
                        {
                            if (safeThis != nullptr && granted)
                                safeThis->openFile();
                        });
                    return;
                }

                fileChooser.reset (new FileChooser ("Select an audio file...", File(), "*.wav;*.mp3;*.aif"));

                fileChooser->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles,
                    [this] (const FileChooser& fc) mutable
                {
                    if (fc.getURLResults().size() > 0)
                    {
                        auto u = fc.getURLResult();

                        if (! audioFileReader.loadURL (u))
                            NativeMessageBox::showAsync (MessageBoxOptions()
                                .withIconType (MessageBoxIconType::WarningIcon)
                                .withTitle ("Error loading file")
                                .withMessage ("Unable to load audio file"),
                                nullptr);
                        else
                            thumbnailComp.setCurrentURL (u);
                    }

                    fileChooser = nullptr;
                }, nullptr);
        */
    }
}
