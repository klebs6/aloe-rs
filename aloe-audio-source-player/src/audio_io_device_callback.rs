crate::ix!();

impl AudioIODeviceCallback for AudioSourcePlayer {

    /**
      | Implementation of the AudioIODeviceCallback
      | method.
      |
      */
    fn audio_device_io_callback(&mut self, 
        input_channel_data:        *const *const f32,
        total_num_input_channels:  i32,
        output_channel_data:       *mut *mut f32,
        total_num_output_channels: i32,
        num_samples:               i32)  {
        
        todo!();
        /*
            // these should have been prepared by audioDeviceAboutToStart()...
        jassert (sampleRate > 0 && bufferSize > 0);

        const ScopedLock sl (readLock);

        if (source != nullptr)
        {
            int numActiveChans = 0, numInputs = 0, numOutputs = 0;

            // messy stuff needed to compact the channels down into an array
            // of non-zero pointers..
            for (int i = 0; i < totalNumInputChannels; ++i)
            {
                if (inputChannelData[i] != nullptr)
                {
                    inputChans [numInputs++] = inputChannelData[i];
                    if (numInputs >= numElementsInArray (inputChans))
                        break;
                }
            }

            for (int i = 0; i < totalNumOutputChannels; ++i)
            {
                if (outputChannelData[i] != nullptr)
                {
                    outputChans [numOutputs++] = outputChannelData[i];
                    if (numOutputs >= numElementsInArray (outputChans))
                        break;
                }
            }

            if (numInputs > numOutputs)
            {
                // if there aren't enough output channels for the number of
                // inputs, we need to create some temporary extra ones (can't
                // use the input data in case it gets written to)
                tempBuffer.setSize (numInputs - numOutputs, numSamples,
                                    false, false, true);

                for (int i = 0; i < numOutputs; ++i)
                {
                    channels[numActiveChans] = outputChans[i];
                    memcpy (channels[numActiveChans], inputChans[i], (size_t) numSamples * sizeof (float));
                    ++numActiveChans;
                }

                for (int i = numOutputs; i < numInputs; ++i)
                {
                    channels[numActiveChans] = tempBuffer.getWritePointer (i - numOutputs);
                    memcpy (channels[numActiveChans], inputChans[i], (size_t) numSamples * sizeof (float));
                    ++numActiveChans;
                }
            }
            else
            {
                for (int i = 0; i < numInputs; ++i)
                {
                    channels[numActiveChans] = outputChans[i];
                    memcpy (channels[numActiveChans], inputChans[i], (size_t) numSamples * sizeof (float));
                    ++numActiveChans;
                }

                for (int i = numInputs; i < numOutputs; ++i)
                {
                    channels[numActiveChans] = outputChans[i];
                    zeromem (channels[numActiveChans], (size_t) numSamples * sizeof (float));
                    ++numActiveChans;
                }
            }

            AudioBuffer<float> buffer (channels, numActiveChans, numSamples);

            AudioSourceChannelInfo info (&buffer, 0, numSamples);
            source->getNextAudioBlock (info);

            for (int i = info.buffer->getNumChannels(); --i >= 0;)
                buffer.applyGainRamp (i, info.startSample, info.numSamples, lastGain, gain);

            lastGain = gain;
        }
        else
        {
            for (int i = 0; i < totalNumOutputChannels; ++i)
                if (outputChannelData[i] != nullptr)
                    zeromem (outputChannelData[i], (size_t) numSamples * sizeof (float));
        }
        */
    }
    
    /**
      | Implementation of the AudioIODeviceCallback
      | method.
      |
      */
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            prepareToPlay (device->getCurrentSampleRate(),
                       device->getCurrentBufferSizeSamples());
        */
    }

    /**
      | Implementation of the AudioIODeviceCallback
      | method.
      |
      */
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            if (source != nullptr)
            source->releaseResources();

        sampleRate = 0.0;
        bufferSize = 0;

        tempBuffer.setSize (2, 8);
        */
    }
}
