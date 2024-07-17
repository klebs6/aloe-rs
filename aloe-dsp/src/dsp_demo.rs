crate::ix!();

pub trait DemoInterface {}

pub struct DSPDemo {
    base2:               ProcessorWrapper<Box<dyn DemoInterface>>,
    audio_callback_lock: CriticalSection,
    input_source:        *mut dyn AudioSource,
}

impl ChangeListener for DSPDemo {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            ScopedLock audioLock (audioCallbackLock);
            static_cast<DemoType&> (this->processor).updateParameters();
        */
    }
}

impl AudioSource for DSPDemo {}

impl PrepareToPlayAudioSource for DSPDemo {

    fn prepare_to_play(&mut self, 
        block_size:  i32,
        sample_rate: f64)  {
        
        todo!();
        /*
            inputSource->prepareToPlay (blockSize, sampleRate);
            this->prepare ({ sampleRate, (uint32) blockSize, 2 });
        */
    }
}

impl ReleaseResources for DSPDemo {
    
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            inputSource->releaseResources();
        */
    }
}

impl GetNextAudioBlock for DSPDemo {
    
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            jassert (bufferToFill.buffer != nullptr);

            inputSource->getNextAudioBlock (bufferToFill);

            AudioBlock<FloatType> block (*bufferToFill.buffer,
                                     (size_t) bufferToFill.startSample);

            ScopedLock audioLock (audioCallbackLock);
            this->process (ProcessContextReplacing<FloatType> (block));
        */
    }
}

impl DSPDemo {

    pub fn new(input: &mut dyn AudioSource) -> Self {
    
        todo!();
        /*
        : input_source(&input),

            for (auto* p : getParameters())
                p->addChangeListener (this);
        */
    }
    
    pub fn get_parameters(&mut self) -> &Vec<*mut DSPDemoParameterBase> {
        
        todo!();
        /*
            return this->processor.parameters;
        */
    }
}
