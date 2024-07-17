crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/players/aloe_AudioProcessorPlayer.h]

/**
  | An AudioIODeviceCallback object which
  | streams audio through an AudioProcessor.
  | 
  | To use one of these, just make it the callback
  | used by your AudioIODevice, and give
  | it a processor to use by calling setProcessor().
  | 
  | It's also a MidiInputCallback, so you
  | can connect it to both an audio and midi
  | input to send both streams through the
  | processor. To set a MidiOutput for the
  | processor, use the setMidiOutput()
  | method.
  | 
  | @see AudioProcessor, AudioProcessorGraph
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorPlayer {
    processor:                  *mut dyn AudioProcessorInterface, // default = nullptr
    lock:                       CriticalSection,
    sample_rate:                f64, // default = 0
    block_size:                 i32, // default = 0
    is_prepared:                bool, // default = false
    is_double_precision:        bool, // default = false
    device_channels:            AudioProcessorPlayerNumChannels,
    default_processor_channels: AudioProcessorPlayerNumChannels,
    actual_processor_channels:  AudioProcessorPlayerNumChannels,
    channels:                   Vec<*mut f32>,
    temp_buffer:                AudioBuffer<f32>,
    conversion_buffer:          AudioBuffer<f64>,
    incoming_midi:              MidiBuffer,
    message_collector:          MidiMessageCollector,
    midi_output:                *mut MidiOutput, // default = nullptr
}

impl AudioIODeviceCallback for AudioProcessorPlayer { 

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
            const ScopedLock sl (lock);

        // These should have been prepared by audioDeviceAboutToStart()...
        jassert (sampleRate > 0 && blockSize > 0);

        incomingMidi.clear();
        messageCollector.removeNextBlockOfMessages (incomingMidi, numSamples);

        initialiseIoBuffers ({ inputChannelData,  numInputChannels },
                             { outputChannelData, numOutputChannels },
                             numSamples,
                             actualProcessorChannels.ins,
                             actualProcessorChannels.outs,
                             tempBuffer,
                             channels);

        const auto totalNumChannels = jmax (actualProcessorChannels.ins, actualProcessorChannels.outs);
        AudioBuffer<float> buffer (channels.data(), (int) totalNumChannels, numSamples);

        if (processor != nullptr)
        {
            // The processor should be prepared to deal with the same number of output channels
            // as our output device.
            jassert (processor->isMidiEffect() || numOutputChannels == actualProcessorChannels.outs);

            const ScopedLock sl2 (processor->getCallbackLock());

            if (! processor->isSuspended())
            {
                if (processor->isUsingDoublePrecision())
                {
                    conversionBuffer.makeCopyOf (buffer, true);
                    processor->processBlock (conversionBuffer, incomingMidi);
                    buffer.makeCopyOf (conversionBuffer, true);
                }
                else
                {
                    processor->processBlock (buffer, incomingMidi);
                }

                if (midiOutput != nullptr)
                {
                    if (midiOutput->isBackgroundThreadRunning())
                    {
                        midiOutput->sendBlockOfMessages (incomingMidi,
                                                         Time::getMillisecondCounterHiRes(),
                                                         sampleRate);
                    }
                    else
                    {
                        midiOutput->sendBlockOfMessagesNow (incomingMidi);
                    }
                }

                return;
            }
        }

        for (int i = 0; i < numOutputChannels; ++i)
            FloatVectorOperations::clear (outputChannelData[i], numSamples);
        */
    }
    
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            auto newSampleRate = device->getCurrentSampleRate();
        auto newBlockSize  = device->getCurrentBufferSizeSamples();
        auto numChansIn    = device->getActiveInputChannels().countNumberOfSetBits();
        auto numChansOut   = device->getActiveOutputChannels().countNumberOfSetBits();

        const ScopedLock sl (lock);

        sampleRate = newSampleRate;
        blockSize  = newBlockSize;
        deviceChannels = { numChansIn, numChansOut };

        resizeChannels();

        messageCollector.reset (sampleRate);

        if (processor != nullptr)
        {
            if (isPrepared)
                processor->releaseResources();

            auto* oldProcessor = processor;
            setProcessor (nullptr);
            setProcessor (oldProcessor);
        }
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (processor != nullptr && isPrepared)
            processor->releaseResources();

        sampleRate = 0.0;
        blockSize = 0;
        isPrepared = false;
        tempBuffer.setSize (1, 1);
        */
    }
}

impl MidiInputCallback for AudioProcessorPlayer {

    fn handle_incoming_midi_message(&mut self, 
        _0:      *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            messageCollector.addMessageToQueue (message);
        */
    }
}

impl Drop for AudioProcessorPlayer {

    fn drop(&mut self) {

        todo!();

        /* 
        setProcessor (nullptr);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/players/aloe_AudioProcessorPlayer.cpp]
impl AudioProcessorPlayer {

    /**
      | Returns the current audio processor
      | that is being played.
      |
      */
    pub fn get_current_processor(&self) -> *mut dyn AudioProcessorInterface {
        
        todo!();
        /*
            return processor;
        */
    }

    /**
      | Returns a midi message collector that
      | you can pass midi messages to if you want
      | them to be injected into the midi stream
      | that is being sent to the processor.
      |
      */
    pub fn get_midi_message_collector(&mut self) -> &mut MidiMessageCollector {
        
        todo!();
        /*
            return messageCollector;
        */
    }

    /**
      | Returns true if this player processes
      | internally processes the samples with
      | double floating point precision.
      |
      */
    #[inline] pub fn get_double_precision_processing(&mut self) -> bool {
        
        todo!();
        /*
            return isDoublePrecision;
        */
    }
    
    pub fn new(do_double_precision_processing: Option<bool>) -> Self {

        let do_double_precision_processing: bool =
            do_double_precision_processing.unwrap_or(false);
    
        todo!();
        /*
        : is_double_precision(doDoublePrecisionProcessing),

        
        */
    }
    
    pub fn find_most_suitable_layout(&self, proc: &dyn AudioProcessorInterface) -> AudioProcessorPlayerNumChannels {
        
        todo!();
        /*
            if (proc.isMidiEffect())
            return {};

        std::vector<AudioProcessorPlayerNumChannels> layouts { deviceChannels };

        if (deviceChannels.ins == 0 || deviceChannels.ins == 1)
        {
            layouts.emplace_back (defaultProcessorChannels.ins, deviceChannels.outs);
            layouts.emplace_back (deviceChannels.outs, deviceChannels.outs);
        }

        const auto it = std::find_if (layouts.begin(), layouts.end(), [&] (const AudioProcessorPlayerNumChannels& chans)
        {
            return proc.checkBusesLayoutSupported (chans.toLayout());
        });

        return it != std::end (layouts) ? *it : layouts[0];
        */
    }
    
    pub fn resize_channels(&mut self)  {
        
        todo!();
        /*
            const auto maxChannels = jmax (deviceChannels.ins,
                                       deviceChannels.outs,
                                       actualProcessorChannels.ins,
                                       actualProcessorChannels.outs);
        channels.resize ((size_t) maxChannels);
        tempBuffer.setSize (maxChannels, blockSize);
        */
    }
    
    /**
      | Sets the processor that should be played.
      | 
      | The processor that is passed in will
      | not be deleted or owned by this object.
      | To stop anything playing, pass a nullptr
      | to this method.
      |
      */
    pub fn set_processor(&mut self, processor_to_play: *mut dyn AudioProcessorInterface)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (processor == processorToPlay)
            return;

        if (processorToPlay != nullptr && sampleRate > 0 && blockSize > 0)
        {
            defaultProcessorChannels = AudioProcessorPlayerNumChannels { processorToPlay->getBusesLayout() };
            actualProcessorChannels  = findMostSuitableLayout (*processorToPlay);

            if (processorToPlay->isMidiEffect())
                processorToPlay->setRateAndBufferSizeDetails (sampleRate, blockSize);
            else
                processorToPlay->setPlayConfigDetails (actualProcessorChannels.ins,
                                                       actualProcessorChannels.outs,
                                                       sampleRate,
                                                       blockSize);

            auto supportsDouble = processorToPlay->supportsDoublePrecisionProcessing() && isDoublePrecision;

            processorToPlay->setProcessingPrecision (supportsDouble ? AudioProcessor::doublePrecision
                                                                    : AudioProcessor::singlePrecision);
            processorToPlay->prepareToPlay (sampleRate, blockSize);
        }

        AudioProcessor* oldOne = nullptr;

        oldOne = isPrepared ? processor : nullptr;
        processor = processorToPlay;
        isPrepared = true;
        resizeChannels();

        if (oldOne != nullptr)
            oldOne->releaseResources();
        */
    }
    
    /**
      | Switch between double and single floating
      | point precisions processing.
      | 
      | The audio IO callbacks will still operate
      | in single floating point precision,
      | however, all internal processing including
      | the AudioProcessor will be processed
      | in double floating point precision
      | if the AudioProcessor supports it (see
      | AudioProcessor::supportsDoublePrecisionProcessing()).
      | Otherwise, the processing will remain
      | single precision irrespective of the
      | parameter doublePrecision.
      |
      */
    pub fn set_double_precision_processing(&mut self, double_precision: bool)  {
        
        todo!();
        /*
            if (doublePrecision != isDoublePrecision)
        {
            const ScopedLock sl (lock);

            if (processor != nullptr)
            {
                processor->releaseResources();

                auto supportsDouble = processor->supportsDoublePrecisionProcessing() && doublePrecision;

                processor->setProcessingPrecision (supportsDouble ? AudioProcessor::doublePrecision
                                                                  : AudioProcessor::singlePrecision);
                processor->prepareToPlay (sampleRate, blockSize);
            }

            isDoublePrecision = doublePrecision;
        }
        */
    }
    
    /**
      | Sets the MIDI output that should be used,
      | if required.
      | 
      | The MIDI output will not be deleted or
      | owned by this object. If the MIDI output
      | is deleted, pass a nullptr to this method.
      |
      */
    pub fn set_midi_output(&mut self, midi_output_to_use: *mut MidiOutput)  {
        
        todo!();
        /*
            if (midiOutput != midiOutputToUse)
        {
            const ScopedLock sl (lock);
            midiOutput = midiOutputToUse;
        }
        */
    }
    
    
}
