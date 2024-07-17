/*!
  | Classes for state variable filter processing.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_StateVariableFilter.h]

pub enum StateVariableFilterType
{
    lowPass,
    bandPass,
    highPass
}

impl Default for StateVariableFilterType {

    fn default() -> Self {
        Self::lowPass
    }
}

/**
  | An IIR filter that can perform low, band
  | and high-pass filtering on an audio
  | signal, with 12 dB of attenuation per
  | octave, using a TPT structure, designed
  | for fast modulation (see Vadim Zavalishin's
  | documentation about TPT structures
  | for more information). Its behaviour
  | is based on the analog state variable
  | filter circuit.
  | 
  | -----------
  | @note
  | 
  | The bandpass here is not the one in the
  | RBJ CookBook as its gain can be higher
  | than 0 dB. For the classic 0 dB bandpass,
  | we need to multiply the result by R2.
  | ----------
  | @note
  | 
  | 2: Using this class prevents some loud
  | audio artefacts commonly encountered
  | when changing the cutoff frequency
  | using other filter simulation structures
  | and IIR filter classes. However, this
  | class may still require additional
  | smoothing for cutoff frequency changes.
  | 
  | see IIRFilter, SmoothedValue
  | 
  | @tags{DSP}
  |
  */
#[leak_detector]
pub struct StateVariableFilter<SampleType: SampleTypeInterface> {

    /**
      | The parameters of the state variable
      | filter. It's up to the caller to ensure
      | that these parameters are modified
      | in a thread-safe way.
      |
      */
    parameters: <Self as HasParametersPtr>::ParametersPtr,
    y:          [SampleType; 3],
    s1:         SampleType,
    s2:         SampleType,
}

impl<SampleType: SampleTypeInterface> HasElementType for StateVariableFilter<SampleType> {
    type Type = SampleType;
}

impl<SampleType: SampleTypeInterface> HasParameters for StateVariableFilter<SampleType> {

    /**
      | A typedef for a ref-counted pointer
      | to the coefficients object
      |
      */
    type Parameters = StateVariableFilterParameters<NumericType<Self>>;
}

impl<SampleType: SampleTypeInterface> HasParametersPtr for StateVariableFilter<SampleType> {}

impl<SampleType: SampleTypeInterface> StateVariableFilter<SampleType> {

    /**
      | Initialization of the filter
      |
      */
    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
            reset();
        */
    }

    /**
      | Resets the filter's processing pipeline.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            s1 = s2 = SampleType {0};
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
            util::snapToZero (s1); util::snapToZero (s2);
        */
    }
    
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            static_assert (std::is_same<typename ProcessContext::SampleType, SampleType>::value,
                               "The sample-type of the filter must match the sample-type supplied to this process callback");

                if (context.isBypassed)
                    processInternal<true, ProcessContext> (context);
                else
                    processInternal<false, ProcessContext> (context);
        */
    }

    /**
      | Processes a single sample, without
      | any locking or checking. Use this if
      | you need processing of a single value.
      |
      */
    pub fn process_sample(&mut self, sample: SampleType) -> SampleType {
        
        todo!();
        /*
            switch (parameters->type)
                {
                    case StateVariableFilterParameters<NumericType>::Type::lowPass:  return processLoop<false, StateVariableFilterParameters<NumericType>::Type::lowPass>  (sample, *parameters); break;
                    case StateVariableFilterParameters<NumericType>::Type::bandPass: return processLoop<false, StateVariableFilterParameters<NumericType>::Type::bandPass> (sample, *parameters); break;
                    case StateVariableFilterParameters<NumericType>::Type::highPass: return processLoop<false, StateVariableFilterParameters<NumericType>::Type::highPass> (sample, *parameters); break;
                    default: jassertfalse;
                }

                return SampleType{0};
        */
    }
    
    pub fn process_loop<
        const isBypassed: bool /* , const ty: StateVariableFilterParameters<NumericType<Self>>::Type */
    >(
        &mut self, 
        sample: SampleType,
        state:  &mut StateVariableFilterParameters<NumericType<Self>>

    ) -> SampleType {

        todo!();
        /*
            y[2] = (sample - s1 * state.R2 - s1 * state.g - s2) * state.h;

                y[1] = y[2] * state.g + s1;
                s1   = y[2] * state.g + y[1];

                y[0] = y[1] * state.g + s2;
                s2   = y[1] * state.g + y[0];

                return isBypassed ? sample : y[static_cast<size_t> (type)];
        */
    }
    
    pub fn process_block<
        const isBypassed: bool 
        /* , const ty: StateVariableFilterParameters::<NumericType<Self>>::Type */
        >
        (
            &mut self, 
            input:  *const SampleType,
            output: *mut SampleType,
            n:      usize
        ) 
    {
        todo!();

        /*
            auto state = *parameters;

                for (size_t i = 0 ; i < n; ++i)
                    output[i] = processLoop<isBypassed, type> (input[i], state);

               #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
                snapToZero();
               #endif

                *parameters = state;
        */
    }
    
    pub fn process_internal<ProcessContext, const isBypassed: bool>(
        &mut self, 
        context: &ProcessContext

    ) {
    
        todo!();
        /*
            auto&& inputBlock  = context.getInputBlock();
                auto&& outputBlock = context.getOutputBlock();

                // This class can only process mono signals. Use the ProcessorDuplicator class
                // to apply this filter on a multi-channel audio stream.
                jassert (inputBlock.getNumChannels()  == 1);
                jassert (outputBlock.getNumChannels() == 1);

                auto n = inputBlock.getNumSamples();
                auto* src = inputBlock .getChannelPointer (0);
                auto* dst = outputBlock.getChannelPointer (0);

                switch (parameters->type)
                {
                    case StateVariableFilterParameters<NumericType>::Type::lowPass:  processBlock<isBypassed, StateVariableFilterParameters<NumericType>::Type::lowPass>  (src, dst, n); break;
                    case StateVariableFilterParameters<NumericType>::Type::bandPass: processBlock<isBypassed, StateVariableFilterParameters<NumericType>::Type::bandPass> (src, dst, n); break;
                    case StateVariableFilterParameters<NumericType>::Type::highPass: processBlock<isBypassed, StateVariableFilterParameters<NumericType>::Type::highPass> (src, dst, n); break;
                    default: jassertfalse;
                }
        */
    }
}
