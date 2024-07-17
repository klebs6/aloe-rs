crate::ix!();

/**
  | This class caches the data required to build
  | a new convolution engine (in particular,
  | impulse response data and a ProcessSpec).
  |
  | Calls to `setProcessSpec` and
  | `setImpulseResponse` construct a new engine,
  | which can be retrieved by calling `getEngine`.
  */
pub struct ConvolutionEngineFactory {
    process_spec:           ProcessSpec, // { 44100.0, 128, 2 };
    impulse_response:       AudioBuffer<f32>, // = makeImpulseBuffer();
    original_sample_rate:   f64, // = processSpec.sampleRate;
    wants_normalise:        ConvolutionNormalise, // = Convolution::ConvolutionNormalise::no;
    latency:                ConvolutionLatency,
    head_size:              ConvolutionNonUniform,
    should_be_zero_latency: bool,
    engine:                 TryLockedPtr<MultichannelEngine>,
    mutex:                  RefCell<parking_lot::RawMutex>,
}

impl ConvolutionEngineFactory {

    pub fn new(
        required_latency:   ConvolutionLatency,
        required_head_size: ConvolutionNonUniform) -> Self {
    
        todo!();
        /*


            : latency  { (requiredLatency.latencyInSamples   <= 0) ? 0 : jmax (64, nextPowerOfTwo (requiredLatency.latencyInSamples)) },
              headSize { (requiredHeadSize.headSizeInSamples <= 0) ? 0 : jmax (64, nextPowerOfTwo (requiredHeadSize.headSizeInSamples)) },
              shouldBeZeroLatency (requiredLatency.latencyInSamples == 0)
        */
    }

    /**
      | It is safe to call this method simultaneously
      | with other public member functions.
      |
      */
    pub fn set_process_spec(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            const std::lock_guard<std::mutex> lock (mutex);
            processSpec = spec;

            engine.set (makeEngine());
        */
    }

    /**
      | It is safe to call this method simultaneously
      | with other public member functions.
      |
      */
    pub fn set_impulse_response(&mut self, 
        buf:       BufferWithSampleRate,
        stereo:    ConvolutionStereo,
        trim:      ConvolutionTrim,
        normalise: ConvolutionNormalise)  {
        
        todo!();
        /*
            const std::lock_guard<std::mutex> lock (mutex);
            wantsNormalise = normalise;
            originalSampleRate = buf.sampleRate;

            impulseResponse = [&]
            {
                auto corrected = fixNumChannels (buf.buffer, stereo);
                return trim == Convolution::ConvolutionTrim::yes ? trimImpulseResponse (corrected) : corrected;
            }();

            engine.set (makeEngine());
        */
    }

    /**
      | Returns the most recently-created engine,
      | or nullptr if there is no pending engine,
      | or if the engine is currently being updated
      | by one of the setter methods.  It is safe
      | to call this simultaneously with other
      | public member functions.
      */
    pub fn get_engine(&mut self) -> Box<MultichannelEngine> {
        
        todo!();
        /*
            return engine.get();
        */
    }
    
    pub fn make_engine(&mut self) -> Box<MultichannelEngine> {
        
        todo!();
        /*
            auto resampled = resampleImpulseResponse (impulseResponse, originalSampleRate, processSpec.sampleRate);

            if (wantsNormalise == Convolution::ConvolutionNormalise::yes)
                normaliseImpulseResponse (resampled);
            else
                resampled.applyGain ((float) (originalSampleRate / processSpec.sampleRate));

            const auto currentLatency = jmax (processSpec.maximumBlockSize, (uint32) latency.latencyInSamples);
            const auto maxBufferSize = shouldBeZeroLatency ? static_cast<int> (processSpec.maximumBlockSize)
                                                           : nextPowerOfTwo (static_cast<int> (currentLatency));

            return std::make_unique<MultichannelEngine> (resampled,
                                                         processSpec.maximumBlockSize,
                                                         maxBufferSize,
                                                         headSize,
                                                         shouldBeZeroLatency);
        */
    }
    
    pub fn make_impulse_buffer() -> AudioBuffer<f32> {
        
        todo!();
        /*
            AudioBuffer<float> result (1, 1);
            result.setSample (0, 0, 1.0f);
            return result;
        */
    }
}
