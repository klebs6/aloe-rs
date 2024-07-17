crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ReverbAudioSource.h]

/**
  | An AudioSource that uses the Reverb
  | class to apply a reverb to another AudioSource.
  | 
  | @see Reverb
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ReverbAudioSource {
    lock:   CriticalSection,
    input:  OptionalScopedPointer<dyn AudioSource>,
    reverb: Reverb,
    bypass: AtomicBool,
}

impl AudioSource for ReverbAudioSource {}

impl PrepareToPlayAudioSource for ReverbAudioSource {

    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64

    ) {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        input->prepareToPlay (samplesPerBlockExpected, sampleRate);
        reverb.setSampleRate (sampleRate);
        */
    }
}

impl ReleaseResources for ReverbAudioSource {
    
    fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetNextAudioBlock for ReverbAudioSource {
    
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        input->getNextAudioBlock (bufferToFill);

        if (! bypass)
        {
            float* const firstChannel = bufferToFill.buffer->getWritePointer (0, bufferToFill.startSample);

            if (bufferToFill.buffer->getNumChannels() > 1)
            {
                reverb.processStereo (firstChannel,
                                      bufferToFill.buffer->getWritePointer (1, bufferToFill.startSample),
                                      bufferToFill.numSamples);
            }
            else
            {
                reverb.processMono (firstChannel, bufferToFill.numSamples);
            }
        }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ReverbAudioSource.cpp]
impl ReverbAudioSource {

    /**
      | Returns the parameters from the reverb.
      |
      */
    pub fn get_parameters(&self) -> &ReverbParameters {
        
        todo!();
        /*
            return reverb.getParameters();
        */
    }

    pub fn is_bypassed(&self) -> bool {
        
        todo!();
        /*
            return bypass;
        */
    }
    
    /**
      | Creates a ReverbAudioSource to process
      | a given input source.
      | 
      | -----------
      | @param inputSource
      | 
      | the input source to read from - this must
      | not be null
      | ----------
      | @param deleteInputWhenDeleted
      | 
      | if true, the input source will be deleted
      | when this object is deleted
      |
      */
    pub fn new(
        input_source:              *mut dyn AudioSource,
        delete_input_when_deleted: bool) -> Self {
    
        todo!();
        /*
        : input(inputSource, deleteInputWhenDeleted),
        : bypass(false),

            jassert (inputSource != nullptr);
        */
    }
    
    /**
      | Changes the reverb's parameters.
      |
      */
    pub fn set_parameters(&mut self, new_params: &ReverbParameters)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        reverb.setParameters (newParams);
        */
    }
    
    pub fn set_bypassed(&mut self, b: bool)  {
        
        todo!();
        /*
            if (bypass != b)
        {
            const ScopedLock sl (lock);
            bypass = b;
            reverb.reset();
        }
        */
    }
}
