crate::ix!();

pub struct ConvolutionImpl<'a> {
    message_queue:   OptionalQueue,
    engine_queue:    Arc<ConvolutionEngineQueue<'a>>,
    previous_engine: Box<MultichannelEngine>,
    current_engine:  Box<MultichannelEngine>,
    mixer:           CrossoverMixer,
}

impl<'a> ConvolutionImpl<'a> {

    pub fn new(
        required_latency:   ConvolutionLatency,
        required_head_size: ConvolutionNonUniform,
        queue:              OptionalQueue) -> Self {
    
        todo!();
        /*


            : messageQueue (std::move (queue)),
              engineQueue (std::make_shared<ConvolutionEngineQueue> (*messageQueue->impl,
                                                                     requiredLatency,
                                                                     requiredHeadSize))
        */
    }

    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mixer.reset();

            if (currentEngine != nullptr)
                currentEngine->reset();

            destroyPreviousEngine();
        */
    }

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            messageQueue->impl->popAll();
            mixer.prepare (spec);
            engineQueue->prepare (spec);

            if (auto newEngine = engineQueue->getEngine())
                currentEngine = std::move (newEngine);

            previousEngine = nullptr;
            jassert (currentEngine != nullptr);
        */
    }

    pub fn process_samples(&mut self, 
        input:  &AudioBlock<f32>,
        output: &mut AudioBlock<f32>)  {
        
        todo!();
        /*
            engineQueue->postPendingCommand();

            if (previousEngine == nullptr)
                installPendingEngine();

            mixer.processSamples (input,
                                  output,
                                  [this] (const AudioBlock<const float>& in, AudioBlock<float>& out)
                                  {
                                      currentEngine->processSamples (in, out);
                                  },
                                  [this] (const AudioBlock<const float>& in, AudioBlock<float>& out)
                                  {
                                      if (previousEngine != nullptr)
                                          previousEngine->processSamples (in, out);
                                      else
                                          out.copyFrom (in);
                                  },
                                  [this] { destroyPreviousEngine(); });
        */
    }

    pub fn get_current_ir_size(&self) -> i32 {
        
        todo!();
        /*
            return currentEngine != nullptr ? currentEngine->getIRSize() : 0;
        */
    }

    pub fn get_latency(&self) -> i32 {
        
        todo!();
        /*
            return currentEngine != nullptr ? currentEngine->getLatency() : 0;
        */
    }

    pub fn load_impulse_response_from_memory(
        &mut self, 
        buffer:               AudioBuffer<f32>,
        original_sample_rate: f64,
        stereo:               ConvolutionStereo,
        trim:                 ConvolutionTrim,
        normalise:            ConvolutionNormalise

    ) {
        
        todo!();
        /*
            engineQueue->loadImpulseResponse (std::move (buffer), originalSampleRate, stereo, trim, normalise);
        */
    }

    pub fn load_impulse_response_from_raw(
        &mut self, 
        source_data:      *const c_void,
        source_data_size: usize,
        stereo:           ConvolutionStereo,
        trim:             ConvolutionTrim,
        size:             usize,
        normalise:        ConvolutionNormalise

    ) {
        
        todo!();
        /*
            engineQueue->loadImpulseResponse (sourceData, sourceDataSize, stereo, trim, size, normalise);
        */
    }

    pub fn load_impulse_response_from_file(
        &mut self, 
        file_impulse_response: &File,
        stereo:                ConvolutionStereo,
        trim:                  ConvolutionTrim,
        size:                  usize,
        normalise:             ConvolutionNormalise

    ) {
        
        todo!();
        /*
            engineQueue->loadImpulseResponse (fileImpulseResponse, stereo, trim, size, normalise);
        */
    }

    pub fn destroy_previous_engine(&mut self)  {
        
        todo!();
        /*
            // If the queue is full, we'll destroy this straight away
            BackgroundMessageQueue::IncomingCommand command = [p = std::move (previousEngine)]() mutable { p = nullptr; };
            messageQueue->impl->push (command);
        */
    }

    pub fn install_new_engine(&mut self, new_engine: Box<MultichannelEngine>)  {
        
        todo!();
        /*
            destroyPreviousEngine();
            previousEngine = std::move (currentEngine);
            currentEngine = std::move (newEngine);
            mixer.beginTransition();
        */
    }

    pub fn install_pending_engine(&mut self)  {
        
        todo!();
        /*
            if (auto newEngine = engineQueue->getEngine())
                installNewEngine (std::move (newEngine));
        */
    }
}
