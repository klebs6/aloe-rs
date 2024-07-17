crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_MixerAudioSource.h]

/**
  | An AudioSource that mixes together the output
  | of a set of other AudioSources.
  |
  | Input sources can be added and removed while
  | the mixer is running as long as their
  | prepareToPlay() and releaseResources() methods
  | are called before and after adding them to the
  | mixer.
  |
  | @tags{Audio}
  */
#[no_copy]
#[leak_detector]
pub struct MixerAudioSource {
    inputs:               Vec<*mut dyn AudioSource>,
    inputs_to_delete:     BigInteger,
    lock:                 CriticalSection,
    temp_buffer:          AudioBuffer<f32>,
    current_sample_rate:  f64,
    buffer_size_expected: i32,
}

impl AudioSource for MixerAudioSource {}

impl PrepareToPlayAudioSource for MixerAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      | 
      | -----------
      | @note
      | 
      | This will call prepareToPlay() on all
      | its input sources.
      |
      */
    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64)  {
        
        todo!();
        /*
            tempBuffer.setSize (2, samplesPerBlockExpected);

        const ScopedLock sl (lock);

        currentSampleRate = sampleRate;
        bufferSizeExpected = samplesPerBlockExpected;

        for (int i = inputs.size(); --i >= 0;)
            inputs.getUnchecked(i)->prepareToPlay (samplesPerBlockExpected, sampleRate);
        */
    }
}

impl ReleaseResources for MixerAudioSource {
    
    /**
      | Implementation of the AudioSource
      | method.
      | 
      | -----------
      | @note
      | 
      | This will call releaseResources()
      | on all its input sources.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (int i = inputs.size(); --i >= 0;)
            inputs.getUnchecked(i)->releaseResources();

        tempBuffer.setSize (2, 0);

        currentSampleRate = 0;
        bufferSizeExpected = 0;
        */
    }
}

impl GetNextAudioBlock for MixerAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (inputs.size() > 0)
        {
            inputs.getUnchecked(0)->getNextAudioBlock (info);

            if (inputs.size() > 1)
            {
                tempBuffer.setSize (jmax (1, info.buffer->getNumChannels()),
                                    info.buffer->getNumSamples());

                AudioSourceChannelInfo info2 (&tempBuffer, 0, info.numSamples);

                for (int i = 1; i < inputs.size(); ++i)
                {
                    inputs.getUnchecked(i)->getNextAudioBlock (info2);

                    for (int chan = 0; chan < info.buffer->getNumChannels(); ++chan)
                        info.buffer->addFrom (chan, info.startSample, tempBuffer, chan, 0, info.numSamples);
                }
            }
        }
        else
        {
            info.clearActiveBufferRegion();
        }
        */
    }
}

impl Default for MixerAudioSource {

    fn default() -> Self {
    
        todo!();
        /*
        : current_sample_rate(0.0),
        : buffer_size_expected(0),
        */
    }
}

impl Drop for MixerAudioSource {

    fn drop(&mut self) {

        todo!();

        /* 
        removeAllInputs();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_MixerAudioSource.cpp]

impl MixerAudioSource {
    
    /**
      | Adds an input source to the mixer.
      | 
      | -----------
      | @note
      | 
      | If the mixer is running you'll need to
      | make sure that the input source
      | 
      | is ready to play by calling its prepareToPlay()
      | method before adding it.
      | 
      | If the mixer is stopped, then its input
      | sources will be automatically
      | 
      | prepared when the mixer's prepareToPlay()
      | method is called.
      | 
      | -----------
      | @param newInput
      | 
      | the source to add to the mixer
      | ----------
      | @param deleteWhenRemoved
      | 
      | if true, then this source will be deleted
      | when no longer needed by the mixer.
      |
      */
    pub fn add_input_source(
        &mut self, 
        input:               *mut dyn AudioSource,
        delete_when_removed: bool

    ) {
        
        todo!();
        /*
            if (input != nullptr && ! inputs.contains (input))
        {
            double localRate;
            int localBufferSize;

            {
                const ScopedLock sl (lock);
                localRate = currentSampleRate;
                localBufferSize = bufferSizeExpected;
            }

            if (localRate > 0.0)
                input->prepareToPlay (localBufferSize, localRate);

            const ScopedLock sl (lock);

            inputsToDelete.setBit (inputs.size(), deleteWhenRemoved);
            inputs.add (input);
        }
        */
    }
    
    /**
      | Removes an input source.
      | 
      | -----------
      | @note
      | 
      | If the source was added by calling addInputSource()
      | with the deleteWhenRemoved
      | 
      | flag set, it will be deleted by this method.
      |
      */
    pub fn remove_input_source(&mut self, input: *mut dyn AudioSource) {
        
        todo!();
        /*
            if (input != nullptr)
        {
            std::unique_ptr<AudioSource> toDelete;

            {
                const ScopedLock sl (lock);
                const int index = inputs.indexOf (input);

                if (index < 0)
                    return;

                if (inputsToDelete [index])
                    toDelete.reset (input);

                inputsToDelete.shiftBits (-1, index);
                inputs.remove (index);
            }

            input->releaseResources();
        }
        */
    }
    
    /**
      | Removes all the input sources.
      | 
      | -----------
      | @note
      | 
      | Any sources which were added by calling
      | addInputSource() with the deleteWhenRemoved
      | 
      | flag set will be deleted by this method.
      |
      */
    pub fn remove_all_inputs(&mut self)  {
        
        todo!();
        /*
            Vec<Box<AudioSource>> toDelete;

        {
            const ScopedLock sl (lock);

            for (int i = inputs.size(); --i >= 0;)
                if (inputsToDelete[i])
                    toDelete.add (inputs.getUnchecked(i));

            inputs.clear();
        }

        for (int i = toDelete.size(); --i >= 0;)
            toDelete.getUnchecked(i)->releaseResources();
        */
    }
    
}
