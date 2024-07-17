/*!
  | Classes for FIR filter processing.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_FIRFilter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_FIRFilter.cpp]

/**
  | A processing class that can perform
  | FIR filtering on an audio signal, in
  | the time domain.
  | 
  | Using FIRFilter is fast enough for FIRCoefficients
  | with a size lower than 128 samples. For
  | longer filters, it might be more efficient
  | to use the class Convolution instead,
  | which does the same processing in the
  | frequency domain thanks to FFT.
  | 
  | @see FIRFilter::Coefficients, Convolution,
  | FFT
  | 
  | @tags{DSP}
  |
  */
#[leak_detector]
pub struct FIRFilter<SampleType: SampleTypeInterface> {

    /**
      | The coefficients of the FIR filter.
      | It's up to the caller to ensure that these
      | coefficients are modified in a thread-safe
      | way.
      | 
      | If you change the order of the coefficients
      | then you must call reset after modifying
      | them.
      |
      */
    coefficients: <Self as HasFIRCoefficientsPtr>::Ptr,
    memory:       HeapBlock<SampleType>,
    fifo:         *mut SampleType, // default = nullptr
    pos:          usize, // default = 0
    size:         usize, // default = 0
}

impl<SampleType: SampleTypeInterface> Default for FIRFilter<SampleType> {
    
    /**
      | This will create a filter which will
      | produce silence.
      |
      */
    fn default() -> Self {
        todo!();
        /*

            : coefficients (new Coefficients<NumericType>)                                     
        reset()
        */
    }
}

impl<SampleType: SampleTypeInterface> HasElementType for FIRFilter<SampleType> {

    type Type = SampleType;
}

impl<SampleType: SampleTypeInterface> HasFIRCoefficients for FIRFilter<SampleType> {

    /**
      | A typedef for a ref-counted pointer
      | to the coefficients object
      |
      */
    type Coefficients = FIRCoefficients<NumericType<Self>>;
}

impl<SampleType: SampleTypeInterface> HasFIRCoefficientsPtr for FIRFilter<SampleType> {}

impl<SampleType: SampleTypeInterface> FIRFilter<SampleType> {

    /**
      | Creates a filter with a given set of coefficients.
      |
      */
    pub fn new(coefficients_to_use: <Self as HasFIRCoefficientsPtr>::Ptr) -> Self {
    
        todo!();
        /*

            : coefficients (std::move (coefficientsToUse))   
        reset();
        */
    }
        
    /**
      | Prepare this filter for processing.
      |
      */
    #[inline] pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            // This class can only process mono signals. Use the ProcessorDuplicator class
                // to apply this filter on a multi-channel audio stream.
                jassertquiet (spec.numChannels == 1);
                reset();
        */
    }

    /**
      | Resets the filter's processing pipeline,
      | ready to start a new stream of data.
      | 
      | -----------
      | @note
      | 
      | this clears the processing state, but
      | the type of filter and its coefficients
      | aren't changed. To disable the filter,
      | call setEnabled (false).
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            if (coefficients != nullptr)
                {
                    auto newSize = coefficients->getFilterOrder() + 1;

                    if (newSize != size)
                    {
                        memory.malloc (1 + jmax (newSize, size, static_cast<size_t> (128)));

                        fifo = snapPointerToAlignment (memory.getData(), sizeof (SampleType));
                        size = newSize;
                    }

                    for (size_t i = 0; i < size; ++i)
                        fifo[i] = SampleType {0};
                }
        */
    }

    /**
      | Processes a block of samples
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            static_assert (std::is_same<typename ProcessContext::SampleType, SampleType>::value,
                               "The sample-type of the FIR filter must match the sample-type supplied to this process callback");
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

                auto* fir = coefficients->getRawCoefficients();
                size_t p = pos;

                if (context.isBypassed)
                {
                    for (size_t i = 0; i < numSamples; ++i)
                    {
                        fifo[p] = dst[i] = src[i];
                        p = (p == 0 ? size - 1 : p - 1);
                    }
                }
                else
                {
                    for (size_t i = 0; i < numSamples; ++i)
                        dst[i] = processSingleSample (src[i], fifo, fir, size, p);
                }

                pos = p;
        */
    }

    /**
      | Processes a single sample, without
      | any locking. Use this if you need processing
      | of a single value.
      |
      */
    pub fn process_sample(&mut self, sample: SampleType) -> SampleType {
        
        todo!();
        /*
            check();
                return processSingleSample (sample, fifo, coefficients->getRawCoefficients(), size, pos);
        */
    }

    pub fn check(&mut self)  {
        
        todo!();
        /*
            jassert (coefficients != nullptr);

                if (size != (coefficients->getFilterOrder() + 1))
                    reset();
        */
    }

    pub fn process_single_sample(
        sample: SampleType,
        buf:    *mut SampleType,
        fir:    *const NumericType<Self>,
        m:      usize,
        p:      &mut usize) -> SampleType {
        
        todo!();
        /*
            SampleType out (0);

                buf[p] = sample;

                size_t k;
                for (k = 0; k < m - p; ++k)
                    out += buf[(p + k)] * fir[k];

                for (size_t j = 0; j < p; ++j)
                    out += buf[j] * fir[j + k];

                p = (p == 0 ? m - 1 : p - 1);

                return out;
        */
    }
}
