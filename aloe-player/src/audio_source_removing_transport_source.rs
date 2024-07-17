crate::ix!();

/**
  | An AudioSourcePlayer which will remove itself
  | from the AudioDeviceManager's callback list
  | once it finishes playing its source
  */
#[no_copy]
#[leak_detector]
pub struct AutoRemovingTransportSource<'a> {
    base2:            Timer,
    mixer:            &'a mut MixerAudioSource,
    transport_source: OptionalScopedPointer<AudioTransportSource<'a>>,
}

impl<'a> Drop for AutoRemovingTransportSource<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            setSource (nullptr);
         */
    }
}

impl<'a> AutoRemovingTransportSource<'a> {
    
    pub fn new(
        mixer_to_use:         &mut MixerAudioSource,
        ts:                   *mut AudioTransportSource,
        own_source:           bool,
        samples_per_block:    i32,
        required_sample_rate: f64) -> Self {
    
        todo!();
        /*
        : mixer(mixerToUse),
        : transport_source(ts, ownSource),

            jassert (ts != nullptr);

            setSource (transportSource);

            prepareToPlay (samplesPerBlock, requiredSampleRate);
            start();

            mixer.addInputSource (this, true);
            startTimerHz (10);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (! transportSource->isPlaying())
                mixer.removeInputSource (this);
        */
    }
}
