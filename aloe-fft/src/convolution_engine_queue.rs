crate::ix!();

/**
  | This class acts as a destination for
  | convolution engines which are loaded on
  | a background thread.
  |
  | Deriving from `enable_shared_from_this` allows
  | us to capture a reference to this object when
  | adding commands to the background message
  | queue.
  |
  | That way, we can avoid dangling references in
  | the background thread in the case that
  | a Convolution instance is deleted before the
  | background message queue.
  */
pub struct ConvolutionEngineQueue<'a> {
    //base:            EnableSharedFromThis<ConvolutionEngineQueue<'a>>,
    message_queue:   &'a mut BackgroundMessageQueue,
    factory:         ConvolutionEngineFactory,
    pending_command: BackgroundMessageQueueIncomingCommand,
}

impl<'a> ConvolutionEngineQueue<'a> {

    pub fn new(
        queue:        &mut BackgroundMessageQueue,
        latency_in:   ConvolutionLatency,
        head_size_in: ConvolutionNonUniform) -> Self {
    
        todo!();
        /*
        : message_queue(queue),
        : factory(latencyIn, headSizeIn),

        
        */
    }
    
    pub fn load_impulse_response_from_memory(
        &mut self, 
        buffer:    AudioBuffer<f32>,
        sr:        f64,
        stereo:    ConvolutionStereo,
        trim:      ConvolutionTrim,
        normalise: ConvolutionNormalise)  {
        
        todo!();
        /*
            callLater ([b = std::move (buffer), sr, stereo, trim, normalise] (ConvolutionEngineFactory& f) mutable
            {
                f.setImpulseResponse ({ std::move (b), sr }, stereo, trim, normalise);
            });
        */
    }
    
    pub fn load_impulse_response_from_raw(
        &mut self, 
        source_data:      *const c_void,
        source_data_size: usize,
        stereo:           ConvolutionStereo,
        trim:             ConvolutionTrim,
        size:             usize,
        normalise:        ConvolutionNormalise)  {
        
        todo!();
        /*
            callLater ([sourceData, sourceDataSize, stereo, trim, size, normalise] (ConvolutionEngineFactory& f) mutable
            {
                setImpulseResponse (f, sourceData, sourceDataSize, stereo, trim, size, normalise);
            });
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
            callLater ([fileImpulseResponse, stereo, trim, size, normalise] (ConvolutionEngineFactory& f) mutable
            {
                setImpulseResponse (f, fileImpulseResponse, stereo, trim, size, normalise);
            });
        */
    }
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            factory.setProcessSpec (spec);
        */
    }

    /**
      | Call this regularly to try to resend any
      | pending message.
      |
      | This allows us to always apply the most
      | recently requested state (eventually), even
      | if the message queue fills up.
      */
    pub fn post_pending_command(&mut self)  {
        
        todo!();
        /*
            if (pendingCommand == nullptr)
                return;

            if (messageQueue.push (pendingCommand))
                pendingCommand = nullptr;
        */
    }
    
    pub fn get_engine(&mut self) -> Box<MultichannelEngine> {
        
        todo!();
        /*
            return factory.getEngine();
        */
    }
    
    pub fn call_later<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            // If there was already a pending command (because the queue was full) we'll end up deleting it here.
            // Not much we can do about that!
            pendingCommand = [weak = weakFromThis(), callback = std::forward<Fn> (fn)]() mutable
            {
                if (auto t = weak.lock())
                    callback (t->factory);
            };

            postPendingCommand();
        */
    }
    
    pub fn weak_from_this(&mut self) -> Weak<ConvolutionEngineQueue> {
        
        todo!();
        /*
            return shared_from_this();
        */
    }
}
