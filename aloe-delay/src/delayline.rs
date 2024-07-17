crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_DelayLine.h]

/**
  | A delay line processor featuring several
  | algorithms for the fractional delay
  | calculation, block processing, and
  | sample-by-sample processing useful
  | when modulating the delay in real time
  | or creating a standard delay effect
  | with feedback.
  | 
  | Note: If you intend to change the delay
  | in real time, you may want to smooth changes
  | to the delay systematically using either
  | a ramp or a low-pass filter.
  | 
  | @see SmoothedValue, FirstOrderTPTFilter
  | 
  | @tags{DSP}
  |
  */
pub struct DelayLine<SampleType,B: SampleInterpolationBehavior<SampleType> = delay_line_interpolation_types::Linear> 
{
    sample_rate: f64,
    buffer_data: AudioBuffer<SampleType>,
    v:           Vec<SampleType>,
    write_pos:   Vec<i32>,
    read_pos:    Vec<i32>,
    delay:       SampleType, // default = 0.0
    delay_frac:  SampleType, // default = 0.0
    delay_int:   i32, // default = 0
    total_size:  i32, // default = 4
    alpha:       SampleType, // default = 0.0
    _interp:     std::marker::PhantomData<B>,
}

impl<SampleType: SampleTypeInterface, B: SampleInterpolationBehavior<SampleType>> 
Default for DelayLine<SampleType, B>
{
    fn default() -> Self {
        todo!();
        /*
            : DelayLine (0
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_DelayLine.cpp]
impl<SampleType: SampleTypeInterface,B: SampleInterpolationBehavior<SampleType>> DelayLine<SampleType,B> 
where SampleType: From<f32>
{

    /**
      | Processes the input and output samples
      | supplied in the processing context.
      | 
      | Can be used for block processing when
      | the delay is not going to change during
      | processing. The delay must first be
      | set by calling setDelay.
      | 
      | @see setDelay
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();
            auto& outputBlock      = context.getOutputBlock();
            const auto numChannels = outputBlock.getNumChannels();
            const auto numSamples  = outputBlock.getNumSamples();

            jassert (inputBlock.getNumChannels() == numChannels);
            jassert (inputBlock.getNumChannels() == writePos.size());
            jassert (inputBlock.getNumSamples()  == numSamples);

            if (context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                auto* inputSamples = inputBlock.getChannelPointer (channel);
                auto* outputSamples = outputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                {
                    pushSample ((int) channel, inputSamples[i]);
                    outputSamples[i] = popSample ((int) channel);
                }
            }
        */
    }
    
    #[inline] pub fn interpolate_sample(&mut self, channel: i32) -> SampleType {
        B::interpolate_sample(self, channel)
    }

    #[inline] pub fn update_internal_variables(&mut self)  {
        B::update_internal_variables(self)
    }

    pub fn new(maximum_delay_in_samples: i32) -> Self {
    
        todo!();
        /*


            jassert (maximumDelayInSamples >= 0);

        totalSize = jmax (4, maximumDelayInSamples + 1);
        sampleRate = 44100.0;
        */
    }
    
    /**
      | Sets the delay in samples.
      |
      */
    pub fn set_delay(&mut self, new_delay_in_samples: SampleType)  {
        
        todo!();
        /*
            auto upperLimit = (SampleType) (totalSize - 1);
        jassert (isPositiveAndNotGreaterThan (newDelayInSamples, upperLimit));

        delay     = jlimit ((SampleType) 0, upperLimit, newDelayInSamples);
        delayInt  = static_cast<int> (std::floor (delay));
        delayFrac = delay - (SampleType) delayInt;

        updateInternalVariables();
        */
    }
    
    /**
      | Returns the current delay in samples.
      |
      */
    pub fn get_delay(&self) -> SampleType {
        
        todo!();
        /*
            return delay;
        */
    }
    
    /**
      | Initialises the processor.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            jassert (spec.numChannels > 0);

        bufferData.setSize ((int) spec.numChannels, totalSize, false, false, true);

        writePos.resize (spec.numChannels);
        readPos.resize  (spec.numChannels);

        v.resize (spec.numChannels);
        sampleRate = spec.sampleRate;

        reset();
        */
    }
    
    /**
      | Resets the internal state variables
      | of the processor.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto vec : { &writePos, &readPos })
            std::fill (vec->begin(), vec->end(), 0);

        std::fill (v.begin(), v.end(), static_cast<SampleType> (0));

        bufferData.clear();
        */
    }
    
    /**
      | Pushes a single sample into one channel
      | of the delay line.
      | 
      | Use this function and popSample instead
      | of process if you need to modulate the
      | delay in real time instead of using a
      | fixed delay value, or if you want to code
      | a delay effect with a feedback loop.
      | 
      | @see setDelay, popSample, process
      |
      */
    pub fn push_sample(&mut self, 
        channel: i32,
        sample:  SampleType)  {
        
        todo!();
        /*
            bufferData.setSample (channel, writePos[(size_t) channel], sample);
        writePos[(size_t) channel] = (writePos[(size_t) channel] + totalSize - 1) % totalSize;
        */
    }
    
    /**
      | Pops a single sample from one channel
      | of the delay line.
      | 
      | Use this function to modulate the delay
      | in real time or implement standard delay
      | effects with feedback.
      | 
      | -----------
      | @param channel
      | 
      | the target channel for the delay line.
      | ----------
      | @param delayInSamples
      | 
      | sets the wanted fractional delay in
      | samples, or -1 to use the value being
      | used before or set with setDelay function.
      | ----------
      | @param updateReadPointer
      | 
      | should be set to true if you use the function
      | once for each sample, or false if you
      | need multi-tap delay capabilities.
      | 
      | @see setDelay, pushSample, process
      |
      */
    pub fn pop_sample(
        &mut self, 
        channel:             i32,
        delay_in_samples:    Option<SampleType>,
        update_read_pointer: Option<bool>

    ) -> SampleType {

        let delay_in_samples    = delay_in_samples.unwrap_or(SampleType::from(-1.0));
        let update_read_pointer = update_read_pointer.unwrap_or(true);
        
        todo!();
        /*
            if (delayInSamples >= 0)
            setDelay(delayInSamples);

        auto result = interpolateSample (channel);

        if (updateReadPointer)
            readPos[(size_t) channel] = (readPos[(size_t) channel] + totalSize - 1) % totalSize;

        return result;
        */
    }
}
