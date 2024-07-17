/*!
  | Classes for IIR filter processing.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_IIRFilter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_IIRFilter.cpp]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_IIRFilter.h]

/**
  | An IIR filter that can perform low, high,
  | or band-pass filtering on an audio signal,
  | and which attempts to implement basic
  | thread-safety.
  | 
  | This class synchronises calls to some
  | of its member functions, making it safe
  | (although not necessarily real-time-safe)
  | to reset the filter or apply new coefficients
  | while the filter is processing on another
  | thread. In most cases this style of internal
  | locking should not be used, and you should
  | attempt to provide thread-safety at
  | a higher level in your program. If you
  | can guarantee that calls to the filter
  | will be synchronised externally, you
  | could consider switching to SingleThreadedIIRFilter
  | instead.
  | 
  | @see SingleThreadedIIRFilter, IIRCoefficient,
  | IIRFilterAudioSource
  | 
  | @tags{Audio}
  |
  */
pub struct IIRFilter {
    base: IIRFilterBase<SpinLock>,
}

/**
  | A processing class that can perform
  | IIR filtering on an audio signal, using
  | the Transposed Direct Form II digital
  | structure.
  | 
  | If you need a lowpass, bandpass or highpass
  | filter with fast modulation of its cutoff
  | frequency, you might use the class StateVariableFilter
  | instead, which is designed to prevent
  | artefacts at parameter changes, instead
  | of the class IIRFilter.
  | 
  | @see IIRFilter::IIRCoefficients, FilterAudioSource,
  | StateVariableFilter
  | 
  | @tags{DSP}
  |
  */
#[leak_detector]
pub struct IIRFilterTransposedDirectFormII<SampleType: SampleTypeInterface> {

    /**
      | The coefficients of the IIR filter.
      | It's up to the caller to ensure that these
      | coefficients are modified in a thread-safe
      | way.
      | 
      | If you change the order of the coefficients
      | then you must call reset after modifying
      | them.
      |
      */
    coefficients: IIRCoefficientsPtr<Self>,
    memory:       HeapBlock<SampleType>,
    state:        *mut SampleType, // default = nullptr
    order:        usize, // default = 0
}

impl<SampleType: SampleTypeInterface> HasElementType for IIRFilterTransposedDirectFormII<SampleType> {
    type Type = SampleType;
}

impl<SampleType: SampleTypeInterface> HasIIRCoefficients for IIRFilterTransposedDirectFormII<SampleType> {

    /**
      | A typedef for a ref-counted pointer
      | to the coefficients object
      |
      */
    type Coefficients = IIRCoefficients<NumericType<Self>>;
}

impl<SampleType: SampleTypeInterface> HasIIRCoefficientsPtr for IIRFilterTransposedDirectFormII<SampleType> {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_IIRFilter_Impl.h]
impl<SampleType: SampleTypeInterface> Default for IIRFilterTransposedDirectFormII<SampleType> {
    
    /**
      | Creates a filter.
      | 
      | Initially the filter is inactive, so
      | will have no effect on samples that you
      | process with it. You can modify the coefficients
      | member to turn it into the type of filter
      | needed.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : coefficients (new IIRCoefficients<typename IIRFilter<SampleType>::NumericType> (1, 0, 1, 0))
        reset()
        */
    }
}

impl<SampleType: SampleTypeInterface> IIRFilterTransposedDirectFormII<SampleType> {

    /**
      | Resets the filter's processing pipeline,
      | ready to start a new stream of data.
      | 
      | -----------
      | @note
      | 
      | this clears the processing state, but
      | the type of filter and its coefficients
      | aren't changed.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            reset (SampleType {0});
        */
    }

    /**
      | Processes a block of samples
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            if (context.isBypassed)
                    processInternal<ProcessContext, true> (context);
                else
                    processInternal<ProcessContext, false> (context);

               #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
                snapToZero();
               #endif
        */
    }

    /**
      | Creates a filter with a given set of coefficients.
      |
      */
    pub fn new(c: IIRCoefficientsPtr<Self>) -> Self {
    
        todo!();
        /*


            : coefficients (std::move (c))
        reset();
        */
    }
    
    /**
      | Resets the filter's processing pipeline
      | to a specific value. @see reset
      |
      */
    pub fn reset_with_value(&mut self, reset_to_value: SampleType)  {
        
        todo!();
        /*
            auto newOrder = coefficients->getFilterOrder();

        if (newOrder != order)
        {
            memory.malloc (jmax (order, newOrder, static_cast<size_t> (3)) + 1);
            state = snapPointerToAlignment (memory.getData(), sizeof (SampleType));
            order = newOrder;
        }

        for (size_t i = 0; i < order; ++i)
            state[i] = resetToValue;
        */
    }
    
    /**
      | Called before processing starts.
      |
      */
    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
            reset();
        */
    }
    
    /**
      | Processes a block of samples
      |
      */
    pub fn process_internal<ProcessContext, const bypassed: bool>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            static_assert (std::is_same<typename ProcessContext::SampleType, SampleType>::value,
                       "The sample-type of the IIR filter must match the sample-type supplied to this process callback");
        check();

        auto&& inputBlock  = context.getInputBlock();
        auto&& outputBlock = context.getOutputBlock();

        // This class can only process mono signals. Use the ProcessorDuplicator class
        // to apply this filter on a multi-channel audio stream.
        jassert (inputBlock.getNumChannels()  == 1);
        jassert (outputBlock.getNumChannels() == 1);

        auto numSamples = inputBlock.getNumSamples();
        auto* src = inputBlock .getChannelPointer (0);
        auto* dst = outputBlock.getChannelPointer (0);
        auto* coeffs = coefficients->getRawCoefficients();

        switch (order)
        {
            case 1:
            {
                auto b0 = coeffs[0];
                auto b1 = coeffs[1];
                auto a1 = coeffs[2];

                auto lv1 = state[0];

                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = src[i];
                    auto output = input * b0 + lv1;

                    dst[i] = bypassed ? input : output;

                    lv1 = (input * b1) - (output * a1);
                }

                util::snapToZero (lv1); state[0] = lv1;
            }
            break;

            case 2:
            {
                auto b0 = coeffs[0];
                auto b1 = coeffs[1];
                auto b2 = coeffs[2];
                auto a1 = coeffs[3];
                auto a2 = coeffs[4];

                auto lv1 = state[0];
                auto lv2 = state[1];

                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = src[i];
                    auto output = (input * b0) + lv1;
                    dst[i] = bypassed ? input : output;

                    lv1 = (input * b1) - (output* a1) + lv2;
                    lv2 = (input * b2) - (output* a2);
                }

                util::snapToZero (lv1); state[0] = lv1;
                util::snapToZero (lv2); state[1] = lv2;
            }
            break;

            case 3:
            {
                auto b0 = coeffs[0];
                auto b1 = coeffs[1];
                auto b2 = coeffs[2];
                auto b3 = coeffs[3];
                auto a1 = coeffs[4];
                auto a2 = coeffs[5];
                auto a3 = coeffs[6];

                auto lv1 = state[0];
                auto lv2 = state[1];
                auto lv3 = state[2];

                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = src[i];
                    auto output = (input * b0) + lv1;
                    dst[i] = bypassed ? input : output;

                    lv1 = (input * b1) - (output* a1) + lv2;
                    lv2 = (input * b2) - (output* a2) + lv3;
                    lv3 = (input * b3) - (output* a3);
                }

                util::snapToZero (lv1); state[0] = lv1;
                util::snapToZero (lv2); state[1] = lv2;
                util::snapToZero (lv3); state[2] = lv3;
            }
            break;

            default:
            {
                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = src[i];
                    auto output= (input * coeffs[0]) + state[0];
                    dst[i] = bypassed ? input : output;

                    for (size_t j = 0; j < order - 1; ++j)
                        state[j] = (input * coeffs[j + 1]) - (output* coeffs[order + j + 1]) + state[j + 1];

                    state[order - 1] = (input * coeffs[order]) - (output* coeffs[order * 2]);
                }

                snapToZero();
            }
        }
        */
    }
    
    /**
      | Processes a single sample, without
      | any locking.
      | 
      | Use this if you need processing of a single
      | value.
      | 
      | Moreover, you might need the function
      | snapToZero after a few calls to avoid
      | potential denormalisation issues.
      |
      */
    pub fn process_sample(&mut self, sample: SampleType) -> SampleType {
        
        todo!();
        /*
            check();
        auto* c = coefficients->getRawCoefficients();

        auto output = (c[0] * sample) + state[0];

        for (size_t j = 0; j < order - 1; ++j)
            state[j] = (c[j + 1] * sample) - (c[order + j + 1] * output) + state[j + 1];

        state[order - 1] = (c[order] * sample) - (c[order * 2] * output);

        return output;
        */
    }
    
    /**
      | Ensure that the state variables are
      | rounded to zero if the state variables
      | are denormals. This is only needed if
      | you are doing sample by sample processing.
      |
      */
    pub fn snap_to_zero(&mut self)  {
        
        todo!();
        /*
            for (size_t i = 0; i < order; ++i)
            util::snapToZero (state[i]);
        */
    }
    
    pub fn check(&mut self)  {
        
        todo!();
        /*
            jassert (coefficients != nullptr);

        if (order != coefficients->getFilterOrder())
            reset();
        */
    }
}
