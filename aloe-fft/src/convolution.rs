crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_Convolution.h]

/**
  | Performs stereo partitioned convolution
  | of an input signal with an impulse response
  | in the frequency domain, using the Aloe
  | FFT class.
  | 
  | This class provides some thread-safe
  | functions to load impulse responses
  | from audio files or memory on-the-fly
  | without noticeable artefacts, performing
  | resampling and trimming if necessary.
  | 
  | The processing performed by this class
  | is equivalent to the time domain convolution
  | done in the FIRFilter class, with a FIRFilter::Coefficients
  | object having the samples of the impulse
  | response as its coefficients. However,
  | in general it is more efficient to do
  | frequency domain convolution when
  | the size of the impulse response is 64
  | samples or greater.
  | 
  | Note: The default operation of this
  | class uses zero latency and a uniform
  | partitioned algorithm. If the impulse
  | response size is large, or if the algorithm
  | is too CPU intensive, it is possible
  | to use either a fixed latency version
  | of the algorithm, or a simple non-uniform
  | partitioned convolution algorithm.
  | 
  | Threading: It is not safe to interleave
  | calls to the methods of this class. If
  | you need to load new impulse responses
  | during processing the load() calls
  | must be synchronised with process()
  | calls, which in practice means making
  | the load() call from the audio thread.
  | The loadImpulseResponse() functions
  | *are* wait-free and are therefore suitable
  | for use in a realtime context.
  | 
  | @see FIRFilter, FIRFilter::Coefficients,
  | FFT
  | 
  | @tags{DSP}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Convolution<'a> {
    pimpl:     Box<ConvolutionImpl<'a>>,
    mixer:     ConvolutionMixer,
    is_active: bool, // default = false
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_Convolution.cpp]
impl<'a> Default for Convolution<'a> {

    /**
      | Initialises an object for performing
      | convolution in the frequency domain.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            : Convolution (ConvolutionLatency { 0 })
        */
    }
}

impl<'a> Convolution<'a> {

    /**
      | Performs the filter operation on the
      | given set of samples with optional stereo
      | processing.
      |
      */
    pub fn process<ProcessContext: HasSampleType<SampleType = f32>>(&mut self, context: &ProcessContext) {
    
        todo!();
        /*
            processSamples (context.getInputBlock(), context.getOutputBlock(), context.isBypassed);
        */
    }

    /**
      | Initialises a convolution engine using
      | a shared background message queue.
      | 
      | IMPORTANT: the queue *must* remain
      | alive throughout the lifetime of the
      | Convolution.
      |
      */
    pub fn new_with_shared_background_message_queue(queue: &mut ConvolutionMessageQueue) -> Self {
    
        todo!();
        /*


            : Convolution (ConvolutionLatency { 0 }, queue)
        */
    }
    
    /**
      | Initialises an object for performing
      | convolution with a fixed latency.
      | 
      | If the requested latency is zero, the
      | actual latency will also be zero. For
      | requested latencies greater than zero,
      | the actual latency will always at least
      | as large as the requested latency. Using
      | a fixed non-zero latency can reduce
      | the CPU consumption of the convolution
      | algorithm.
      | 
      | -----------
      | @param requiredLatency
      | 
      | the minimum latency
      |
      */
    pub fn new_with_fixed_latency(required_latency: &ConvolutionLatency) -> Self {
    
        todo!();
        /*


            : Convolution (requiredLatency,
                       {},
                       OptionalQueue { std::make_unique<ConvolutionMessageQueue>() })
        */
    }
    
    /**
      | Initialises an object for performing
      | convolution in the frequency domain
      | using a non-uniform partitioned algorithm.
      | 
      | A requiredHeadSize of 256 samples or
      | greater will improve the efficiency
      | of the processing for IR sizes of 4096
      | samples or greater (recommended for
      | reverberation IRs).
      | 
      | -----------
      | @param requiredHeadSize
      | 
      | the head IR size for two stage non-uniform
      | partitioned convolution
      |
      */
    pub fn new_with_nonuniform_partitioned_algorithm(non_uniform: &ConvolutionNonUniform) -> Self {
    
        todo!();
        /*


            : Convolution ({},
                       nonUniform,
                       OptionalQueue { std::make_unique<ConvolutionMessageQueue>() })
        */
    }
    
    /**
      | Behaves the same as the constructor
      | taking a single ConvolutionLatency argument, but
      | with a shared background message queue.
      | 
      | IMPORTANT: the queue *must* remain
      | alive throughout the lifetime of the
      | Convolution.
      |
      */
    pub fn new_with_latency_and_shared_background_message_queue(
        required_latency: &ConvolutionLatency,
        queue:            &mut ConvolutionMessageQueue) -> Self {
    
        todo!();
        /*


            : Convolution (requiredLatency, {}, OptionalQueue { queue })
        */
    }
    
    /**
      | Behaves the same as the constructor
      | taking a single ConvolutionNonUniform argument,
      | but with a shared background message
      | queue.
      | 
      | IMPORTANT: the queue *must* remain
      | alive throughout the lifetime of the
      | Convolution.
      |
      */
    pub fn new_with_nonuniform_partitioned_algorithm_and_shared_background_message_queue(
        non_uniform: &ConvolutionNonUniform,
        queue:       &mut ConvolutionMessageQueue) -> Self {
    
        todo!();
        /*


            : Convolution ({}, nonUniform, OptionalQueue { queue })
        */
    }
    
    pub fn new(
        latency:     &ConvolutionLatency,
        non_uniform: &ConvolutionNonUniform,
        queue:       OptionalQueue) -> Self {
    
        todo!();
        /*


            : pimpl (std::make_unique<ConvolutionImpl> (latency, nonUniform, std::move (queue)))
        */
    }
    
    /**
      | This function loads an impulse response
      | audio file from memory, added in a Aloe
      | project with the Proaloer as binary
      | data. It can load any of the audio formats
      | registered in Aloe, and performs some
      | resampling and pre-processing as well
      | if needed.
      | 
      | Note: Don't try to use this function
      | on float samples, since the data is expected
      | to be an audio file in its binary format.
      | Be sure that the original data remains
      | constant throughout the lifetime of
      | the Convolution object, as the loading
      | process will happen on a background
      | thread once this function has returned.
      | 
      | -----------
      | @param sourceData
      | 
      | the block of data to use as the stream's
      | source
      | ----------
      | @param sourceDataSize
      | 
      | the number of bytes in the source data
      | block
      | ----------
      | @param isStereo
      | 
      | selects either stereo or mono
      | ----------
      | @param requiresTrimming
      | 
      | optionally trim the start and the end
      | of the impulse response
      | ----------
      | @param size
      | 
      | the expected size for the impulse response
      | after loading, can be set to 0 to requesting
      | the original impulse response size
      | ----------
      | @param requiresNormalisation
      | 
      | optionally normalise the impulse response
      | amplitude
      |
      */
    pub fn load_impulse_response_from_memory(
        &mut self, 
        source_data:      *const c_void,
        source_data_size: usize,
        stereo:           ConvolutionStereo,
        trim:             ConvolutionTrim,
        size:             usize,
        normalise:        Option<ConvolutionNormalise>

    ) {
        let normalise = normalise.unwrap_or(ConvolutionNormalise::yes);
        
        todo!();
        /*
            pimpl->loadImpulseResponse (sourceData, sourceDataSize, stereo, trim, size, normalise);
        */
    }
    
    /**
      | This function loads an impulse response
      | from an audio file. It can load any of
      | the audio formats registered in Aloe,
      | and performs some resampling and pre-processing
      | as well if needed.
      | 
      | -----------
      | @param fileImpulseResponse
      | 
      | the location of the audio file
      | ----------
      | @param isStereo
      | 
      | selects either stereo or mono
      | ----------
      | @param requiresTrimming
      | 
      | optionally trim the start and the end
      | of the impulse response
      | ----------
      | @param size
      | 
      | the expected size for the impulse response
      | after loading, can be set to 0 to requesting
      | the original impulse response size
      | ----------
      | @param requiresNormalisation
      | 
      | optionally normalise the impulse response
      | amplitude
      |
      */
    pub fn load_impulse_response_from_audio_file(
        &mut self, 
        file_impulse_response: &File,
        stereo:                ConvolutionStereo,
        trim:                  ConvolutionTrim,
        size:                  usize,
        normalise:             Option<ConvolutionNormalise>

    ) {

        let normalise 
            = normalise.unwrap_or(ConvolutionNormalise::yes);
        
        todo!();
        /*
            pimpl->loadImpulseResponse (fileImpulseResponse, stereo, trim, size, normalise);
        */
    }
    
    /**
      | This function loads an impulse response
      | from an audio buffer. To avoid memory
      | allocation on the audio thread, this
      | function takes ownership of the buffer
      | passed in.
      | 
      | If calling this function during processing,
      | make sure that the buffer is not allocated
      | on the audio thread (be careful of accidental
      | copies!). If you need to pass arbitrary/generated
      | buffers it's recommended to create
      | these buffers on a separate thread and
      | to use some wait-free construct (a lock-free
      | queue or a SpinLock/GenericScopedTryLock
      | combination) to transfer ownership
      | to the audio thread without allocating.
      | 
      | -----------
      | @param buffer
      | 
      | the AudioBuffer to use
      | ----------
      | @param bufferSampleRate
      | 
      | the sampleRate of the data in the AudioBuffer
      | ----------
      | @param isStereo
      | 
      | selects either stereo or mono
      | ----------
      | @param requiresTrimming
      | 
      | optionally trim the start and the end
      | of the impulse response
      | ----------
      | @param requiresNormalisation
      | 
      | optionally normalise the impulse response
      | amplitude
      |
      */
    pub fn load_impulse_response_from_audio_buffer(
        &mut self, 
        buffer:               AudioBuffer<f32>,
        original_sample_rate: f64,
        stereo:               ConvolutionStereo,
        trim:                 ConvolutionTrim,
        normalise:            ConvolutionNormalise
    ) {
        
        todo!();
        /*
            pimpl->loadImpulseResponse (std::move (buffer), originalSampleRate, stereo, trim, normalise);
        */
    }
    
    /**
      | Must be called before first calling
      | process.
      | 
      | In general, calls to loadImpulseResponse()
      | load the impulse response (IR) asynchronously.
      | The IR will become active once it has
      | been completely loaded and processed,
      | which may take some time.
      | 
      | Calling prepare() will ensure that
      | the IR supplied to the most recent call
      | to loadImpulseResponse() is fully
      | initialised. This IR will then be active
      | during the next call to process(). It
      | is recommended to call loadImpulseResponse()
      | *before* prepare() if a specific IR
      | must be active during the first process()
      | call.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            mixer.prepare (spec);
        pimpl->prepare (spec);
        isActive = true;
        */
    }
    
    /**
      | Resets the processing pipeline ready
      | to start a new stream of data.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mixer.reset();
        pimpl->reset();
        */
    }
    
    pub fn process_samples(
        &mut self, 
        input:       &AudioBlock<f32>,
        output:      &mut AudioBlock<f32>,
        is_bypassed: bool)  {
        
        todo!();
        /*
            if (! isActive)
            return;

        jassert (input.getNumChannels() == output.getNumChannels());
        jassert (isPositiveAndBelow (input.getNumChannels(), static_cast<size_t> (3))); // only mono and stereo is supported

        mixer.processSamples (input, output, isBypassed, [this] (const auto& in, auto& out)
        {
            pimpl->processSamples (in, out);
        });
        */
    }
    
    /**
      | This function returns the size of the
      | current IR in samples.
      |
      */
    pub fn get_current_ir_size(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->getCurrentIRSize();
        */
    }
    
    /**
      | This function returns the current latency
      | of the process in samples.
      | 
      | -----------
      | @note
      | 
      | This is the latency of the convolution
      | engine, not the latency associated
      | with the current impulse response choice
      | that has to be considered separately
      | (linear phase filters, for example).
      |
      */
    pub fn get_latency(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->getLatency();
        */
    }
}
