crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioLatencyDemo.h]

#[no_copy]
#[leak_detector]
pub struct LatencyTester<'a> {
    base2:                 Timer,
    results_box:           &'a mut TextEditor<'a>,
    test_sound:            AudioBuffer<f32>,
    recorded_sound:        AudioBuffer<f32>,
    spike_positions:       Vec<i32>,
    lock:                  CriticalSection,
    playing_sample_num:    i32, // default = 0
    recorded_sample_num:   i32, // default = -1
    sample_rate:           f64, // default = 0.0
    test_is_running:       bool, // default = false
    device_input_latency:  i32,
    device_output_latency: i32,
}

impl<'a> AudioIODeviceCallback for LatencyTester<'a> {

    fn audio_device_io_callback(&mut self, _: *const *const f32, _: i32, _: *mut *mut f32, _: i32, _: i32) { todo!() }

    fn audio_device_about_to_start(&mut self, _: *mut (dyn AudioIODeviceInterface + 'static)) { todo!() }

    fn audio_device_stopped(&mut self) { todo!() }
}

impl<'a> LatencyTester<'a> {

    pub fn new(editor_box: &mut TextEditor) -> Self {
    
        todo!();
        /*
        : results_box(editorBox),
        */
    }
    
    pub fn begin_test(&mut self)  {
        
        todo!();
        /*
            resultsBox.moveCaretToEnd();
            resultsBox.insertTextAtCaret (newLine + newLine + "Starting test..." + newLine);
            resultsBox.moveCaretToEnd();

            startTimer (50);

            const ScopedLock sl (lock);
            createTestSound();
            recordedSound.clear();
            playingSampleNum = recordedSampleNum = 0;
            testIsRunning = true;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (testIsRunning && recordedSampleNum >= recordedSound.getNumSamples())
            {
                testIsRunning = false;
                stopTimer();

                // Test has finished, so calculate the result..
                auto latencySamples = calculateLatencySamples();

                resultsBox.moveCaretToEnd();
                resultsBox.insertTextAtCaret (getMessageDescribingResult (latencySamples));
                resultsBox.moveCaretToEnd();
            }
        */
    }
    
    pub fn get_message_describing_result(&mut self, latency_samples: i32) -> String {
        
        todo!();
        /*
            String message;

            if (latencySamples >= 0)
            {
                message << newLine
                        << "Results:" << newLine
                        << latencySamples << " samples (" << String (latencySamples * 1000.0 / sampleRate, 1)
                        << " milliseconds)" << newLine
                        << "The audio device reports an input latency of "
                        << deviceInputLatency << " samples, output latency of "
                        << deviceOutputLatency << " samples." << newLine
                        << "So the corrected latency = "
                        << (latencySamples - deviceInputLatency - deviceOutputLatency)
                        << " samples (" << String ((latencySamples - deviceInputLatency - deviceOutputLatency) * 1000.0 / sampleRate, 2)
                        << " milliseconds)";
            }
            else
            {
                message << newLine
                        << "Couldn't detect the test signal!!" << newLine
                        << "Make sure there's no background noise that might be confusing it..";
            }

            return message;
        */
    }
    
    pub fn audio_device_about_to_start(&mut self, device: *mut AudioIODevice)  {
        
        todo!();
        /*
            testIsRunning = false;
            playingSampleNum = recordedSampleNum = 0;

            sampleRate          = device->getCurrentSampleRate();
            deviceInputLatency  = device->getInputLatencyInSamples();
            deviceOutputLatency = device->getOutputLatencyInSamples();

            recordedSound.setSize (1, (int) (0.9 * sampleRate));
            recordedSound.clear();
        */
    }
    
    pub fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn audio_device_io_callback(&mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (testIsRunning)
            {
                auto* recordingBuffer = recordedSound.getWritePointer (0);
                auto* playBuffer = testSound.getReadPointer (0);

                for (int i = 0; i < numSamples; ++i)
                {
                    if (recordedSampleNum < recordedSound.getNumSamples())
                    {
                        auto inputSamp = 0.0f;

                        for (auto j = numInputChannels; --j >= 0;)
                            if (inputChannelData[j] != nullptr)
                                inputSamp += inputChannelData[j][i];

                        recordingBuffer[recordedSampleNum] = inputSamp;
                    }

                    ++recordedSampleNum;

                    auto outputSamp = (playingSampleNum < testSound.getNumSamples()) ? playBuffer[playingSampleNum] : 0.0f;

                    for (auto j = numOutputChannels; --j >= 0;)
                        if (outputChannelData[j] != nullptr)
                            outputChannelData[j][i] = outputSamp;

                    ++playingSampleNum;
                }
            }
            else
            {
                // We need to clear the output buffers, in case they're full of junk..
                for (int i = 0; i < numOutputChannels; ++i)
                    if (outputChannelData[i] != nullptr)
                        zeromem (outputChannelData[i], (size_t) numSamples * sizeof (float));
            }
        */
    }

    /**
       create a test sound which consists of
       a series of randomly-spaced audio spikes..
      */
    pub fn create_test_sound(&mut self)  {
        
        todo!();
        /*
            auto length = ((int) sampleRate) / 4;
            testSound.setSize (1, length);
            testSound.clear();

            Random rand;

            for (int i = 0; i < length; ++i)
                testSound.setSample (0, i, (rand.nextFloat() - rand.nextFloat() + rand.nextFloat() - rand.nextFloat()) * 0.06f);

            spikePositions.clear();

            int spikePos   = 0;
            int spikeDelta = 50;

            while (spikePos < length - 1)
            {
                spikePositions.add (spikePos);

                testSound.setSample (0, spikePos,      0.99f);
                testSound.setSample (0, spikePos + 1, -0.99f);

                spikePos += spikeDelta;
                spikeDelta += spikeDelta / 6 + rand.nextInt (5);
            }
        */
    }

    /**
       Searches a buffer for a set of spikes that
       matches those in the test sound
      */
    pub fn find_offset_of_spikes(&self, buffer: &AudioBuffer<f32>) -> i32 {
        
        todo!();
        /*
            auto minSpikeLevel = 5.0f;
            auto smooth = 0.975;
            auto* s = buffer.getReadPointer (0);
            int spikeDriftAllowed = 5;

            Vec<int> spikesFound;
            spikesFound.ensureStorageAllocated (100);
            auto runningAverage = 0.0;
            int lastSpike = 0;

            for (int i = 0; i < buffer.getNumSamples() - 10; ++i)
            {
                auto samp = std::abs (s[i]);

                if (samp > runningAverage * minSpikeLevel && i > lastSpike + 20)
                {
                    lastSpike = i;
                    spikesFound.add (i);
                }

                runningAverage = runningAverage * smooth + (1.0 - smooth) * samp;
            }

            int bestMatch = -1;
            auto bestNumMatches = spikePositions.size() / 3; // the minimum number of matches required

            if (spikesFound.size() < bestNumMatches)
                return -1;

            for (int offsetToTest = 0; offsetToTest < buffer.getNumSamples() - 2048; ++offsetToTest)
            {
                int numMatchesHere = 0;
                int foundIndex     = 0;

                for (int refIndex = 0; refIndex < spikePositions.size(); ++refIndex)
                {
                    auto referenceSpike = spikePositions.getUnchecked (refIndex) + offsetToTest;
                    int spike = 0;

                    while ((spike = spikesFound.getUnchecked (foundIndex)) < referenceSpike - spikeDriftAllowed
                             && foundIndex < spikesFound.size() - 1)
                        ++foundIndex;

                    if (spike >= referenceSpike - spikeDriftAllowed && spike <= referenceSpike + spikeDriftAllowed)
                        ++numMatchesHere;
                }

                if (numMatchesHere > bestNumMatches)
                {
                    bestNumMatches = numMatchesHere;
                    bestMatch = offsetToTest;

                    if (numMatchesHere == spikePositions.size())
                        break;
                }
            }

            return bestMatch;
        */
    }
    
    pub fn calculate_latency_samples(&self) -> i32 {
        
        todo!();
        /*
            // Detect the sound in both our test sound and the recording of it, and measure the difference
            // in their start times..
            auto referenceStart = findOffsetOfSpikes (testSound);
            jassert (referenceStart >= 0);

            auto recordedStart = findOffsetOfSpikes (recordedSound);

            return (recordedStart < 0) ? -1
                                       : (recordedStart - referenceStart);
        */
    }
}

///-------------------------
#[no_copy]
#[leak_detector]
pub struct AudioLatencyDemo<'a> {

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
    audio_device_manager: &mut AudioDeviceManager, // default = getSharedAudioDeviceManager (1, 2) 

    latency_tester:      Box<LatencyTester<'a>>,
    live_audio_scroller: Box<LiveScrollingAudioDisplay<'a>>,
    start_test_button:   TextButton<'a>, // default = "Test Latency" 
    results_box:         TextEditor<'a>,
}

impl<'a> Default for AudioLatencyDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            liveAudioScroller.reset (new LiveScrollingAudioDisplay());
            addAndMakeVisible (liveAudioScroller.get());

            addAndMakeVisible (resultsBox);
            resultsBox.setMultiLine (true);
            resultsBox.setReturnKeyStartsNewLine (true);
            resultsBox.setReadOnly (true);
            resultsBox.setScrollbarsShown (true);
            resultsBox.setCaretVisible (false);
            resultsBox.setPopupMenuEnabled (true);

            resultsBox.setColour (TextEditor::outlineColourId, Colour (0x1c000000));
            resultsBox.setColour (TextEditor::shadowColourId,  Colour (0x16000000));

            resultsBox.setText ("Running this test measures the round-trip latency between the audio output and input "
                                "devices you\'ve got selected.\n\n"
                                "It\'ll play a sound, then try to measure the time at which the sound arrives "
                                "back at the audio input. Obviously for this to work you need to have your "
                                "microphone somewhere near your speakers...");

            addAndMakeVisible (startTestButton);
            startTestButton.onClick = [this] { startTest(); };

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             audioDeviceManager.initialise (numInputChannels, 2, nullptr, true, {}, nullptr);
                                         });
           #endif

            audioDeviceManager.addAudioCallback (liveAudioScroller.get());

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for AudioLatencyDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            audioDeviceManager.removeAudioCallback (liveAudioScroller.get());
            audioDeviceManager.removeAudioCallback (latencyTester    .get());
            latencyTester    .reset();
            liveAudioScroller.reset();
         */
    }
}

impl<'a> AudioLatencyDemo<'a> {

    pub fn start_test(&mut self)  {
        
        todo!();
        /*
            if (latencyTester.get() == nullptr)
            {
                latencyTester.reset (new LatencyTester (resultsBox));
                audioDeviceManager.addAudioCallback (latencyTester.get());
            }

            latencyTester->beginTest();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto b = getLocalBounds().reduced (5);

            if (liveAudioScroller.get() != nullptr)
            {
                liveAudioScroller->setBounds (b.removeFromTop (b.getHeight() / 5));
                b.removeFromTop (10);
            }

            startTestButton.setBounds (b.removeFromBottom (b.getHeight() / 10));
            b.removeFromBottom (10);

            resultsBox.setBounds (b);
        */
    }
}
