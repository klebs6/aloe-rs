crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/DSPDemos_Common.h]

pub struct AudioFileReaderComponent<'a> {
    base:  Component<'a>,
    base2: TimeSliceThread,
 
    /**
      | if this PIP is running inside the demo
      | runner, we'll use the shared device
      | manager instead
      |
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager: AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager: &mut AudioDeviceManager, // default = getSharedAudioDeviceManager (0, 2) 

    format_manager:       AudioFormatManager,
    play_state:           Value<'a>, // default = var (false) 
    loop_state:           Value<'a>, // default = var (false) 
    current_sample_rate:  f64,   // default = 44100.0
    current_block_size:   u32,   // default = 512
    current_num_channels: u32,   // default = 2
    reader:               Box<AudioFormatReader<'a>>,
    reader_source:        Box<AudioFormatReaderSource<'a>>,
    transport_source:     Box<AudioTransportSource<'a>>,
    current_demo:         Box<DSPDemo>,
    audio_source_player:  AudioSourcePlayer,
    header:               AudioPlayerHeader<'a>,
    file_read_buffer:     AudioBuffer<f32>,
    parameters_component: Box<DemoParametersComponent<'a>>,
}

impl<'a> ChangeListener for AudioFileReaderComponent<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (playState.getValue() && ! transportSource->isPlaying())
                stop();
        */
    }
}

impl<'a> ValueListener for AudioFileReaderComponent<'a> {

    fn value_changed(&mut self, v: &mut Value)  {
        
        todo!();
        /*
            if (readerSource.get() != nullptr)
                readerSource->setLooping (v.getValue());
        */
    }
}

impl<'a> Default for AudioFileReaderComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            : TimeSliceThread ("Audio File Reader Thread"),
              header (audioDeviceManager, formatManager, *this)

            loopState.addListener (this);

            formatManager.registerBasicFormats();
            audioDeviceManager.addAudioCallback (&audioSourcePlayer);

           #ifndef ALOE_DEMO_RUNNER
            audioDeviceManager.initialiseWithDefaultDevices (0, 2);
           #endif

            init();
            startThread();

            setOpaque (true);

            addAndMakeVisible (header);

            setSize (800, 250)
        */
    }
}

impl<'a> Drop for AudioFileReaderComponent<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            signalThreadShouldExit();
            stop();
            audioDeviceManager.removeAudioCallback (&audioSourcePlayer);
            waitForThreadToExit (10000);
         */
    }
}

impl<'a> AudioFileReaderComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
            g.fillRect (getLocalBounds());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

            header.setBounds (r.removeFromTop (120));

            r.removeFromTop (20);

            if (parametersComponent.get() != nullptr)
                parametersComponent->setBounds (r.removeFromTop (parametersComponent->getHeightNeeded()).reduced (20, 0));
        */
    }
    
    pub fn loadurl(&mut self, file_to_play: &Url) -> bool {
        
        todo!();
        /*
            stop();

            audioSourcePlayer.setSource (nullptr);
            getThumbnailComponent().setTransportSource (nullptr);
            transportSource.reset();
            readerSource.reset();

            AudioFormatReader* newReader = nullptr;

           #if ! ALOE_IOS
            if (fileToPlay.isLocalFile())
            {
                newReader = formatManager.createReaderFor (fileToPlay.getLocalFile());
            }
            else
           #endif
            {
                if (newReader == nullptr)
                    newReader = formatManager.createReaderFor (fileToPlay.createInputStream (false));
            }

            reader.reset (newReader);

            if (reader.get() != nullptr)
            {
                readerSource.reset (new AudioFormatReaderSource (reader.get(), false));
                readerSource->setLooping (loopState.getValue());

                init();

                return true;
            }

            return false;
        */
    }
    
    pub fn toggle_play(&mut self)  {
        
        todo!();
        /*
            if (playState.getValue())
                stop();
            else
                play();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            playState = false;

            if (transportSource.get() != nullptr)
            {
                transportSource->stop();
                transportSource->setPosition (0);
            }
        */
    }
    
    pub fn init(&mut self)  {
        
        todo!();
        /*
            if (transportSource.get() == nullptr)
            {
                transportSource.reset (new AudioTransportSource());
                transportSource->addChangeListener (this);

                if (readerSource.get() != nullptr)
                {
                    if (auto* device = audioDeviceManager.getCurrentAudioDevice())
                    {
                        transportSource->setSource (readerSource.get(), roundToInt (device->getCurrentSampleRate()), this, reader->sampleRate);

                        getThumbnailComponent().setTransportSource (transportSource.get());
                    }
                }
            }

            audioSourcePlayer.setSource (nullptr);
            currentDemo.reset();

            if (currentDemo.get() == nullptr)
                currentDemo.reset (new DSPDemo<DemoType> (*transportSource));

            audioSourcePlayer.setSource (currentDemo.get());

            initParameters();
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            if (readerSource.get() == nullptr)
                return;

            if (transportSource->getCurrentPosition() >= transportSource->getLengthInSeconds()
                 || transportSource->getCurrentPosition() < 0)
                transportSource->setPosition (0);

            transportSource->start();
            playState = true;
        */
    }
    
    pub fn set_looping(&mut self, should_loop: bool)  {
        
        todo!();
        /*
            if (readerSource.get() != nullptr)
                readerSource->setLooping (shouldLoop);
        */
    }
    
    pub fn get_thumbnail_component(&mut self) -> &mut AudioThumbnailComponent {
        
        todo!();
        /*
            return header.thumbnailComp;
        */
    }
    
    pub fn init_parameters(&mut self)  {
        
        todo!();
        /*
            auto& parameters = currentDemo->getParameters();

            parametersComponent.reset();

            if (parameters.size() > 0)
            {
                parametersComponent.reset (new DemoParametersComponent (parameters));
                addAndMakeVisible (parametersComponent.get());
            }

            resized();
        */
    }
}
