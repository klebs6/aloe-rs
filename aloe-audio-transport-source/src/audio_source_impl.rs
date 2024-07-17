crate::ix!();

impl<'a> AudioSource for AudioTransportSource<'a> {}

impl<'a> PrepareToPlayAudioSource for AudioTransportSource<'a> {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        new_sample_rate:            f64)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        sampleRate = newSampleRate;
        blockSize = samplesPerBlockExpected;

        if (masterSource != nullptr)
            masterSource->prepareToPlay (samplesPerBlockExpected, sampleRate);

        if (resamplerSource != nullptr && sourceSampleRate > 0)
            resamplerSource->setResamplingRatio (sourceSampleRate / sampleRate);

        inputStreamEOF = false;
        isPrepared = true;
        */
    }
}

impl<'a> ReleaseResources for AudioTransportSource<'a> {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            releaseMasterResources();
        */
    }
}

impl<'a> GetNextAudioBlock for AudioTransportSource<'a> {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        if (masterSource != nullptr && ! stopped)
        {
            masterSource->getNextAudioBlock (info);

            if (! playing)
            {
                // just stopped playing, so fade out the last block..
                for (int i = info.buffer->getNumChannels(); --i >= 0;)
                    info.buffer->applyGainRamp (i, info.startSample, jmin (256, info.numSamples), 1.0f, 0.0f);

                if (info.numSamples > 256)
                    info.buffer->clear (info.startSample + 256, info.numSamples - 256);
            }

            if (positionableSource->getNextReadPosition() > positionableSource->getTotalLength() + 1
                  && ! positionableSource->isLooping())
            {
                playing = false;
                inputStreamEOF = true;
                sendChangeMessage();
            }

            stopped = ! playing;

            for (int i = info.buffer->getNumChannels(); --i >= 0;)
                info.buffer->applyGainRamp (i, info.startSample, info.numSamples, lastGain, gain);
        }
        else
        {
            info.clearActiveBufferRegion();
            stopped = true;
        }

        lastGain = gain;
        */
    }
}
