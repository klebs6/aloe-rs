crate::ix!();

/**
  | A base class for the smoothed value classes.
  | 
  | This class is used to provide common
  | functionality to the SmoothedValue
  | and dsp::LogRampedValue classes.
  | 
  | @tags{Audio}
  |
  */
pub struct SmoothedValueBase<FloatType: num::Float,SmoothedValueType> {
    current_value: FloatType, // default = 0
    target:        FloatType, // default = currentValue
    countdown:     i32,       // default = 0
    p0:            PhantomData<SmoothedValueType>,
}

impl<FloatType: num::Float,SmoothedValueType> SmoothedValueBase<FloatType,SmoothedValueType> {

    /**
      | Returns true if the current value is
      | currently being interpolated.
      |
      */
    pub fn is_smoothing(&self) -> bool {
        
        todo!();
        /*
            return countdown > 0;
        */
    }

    /**
      | Returns the current value of the ramp.
      |
      */
    pub fn get_current_value(&self) -> FloatType {
        
        todo!();
        /*
            return currentValue;
        */
    }
    
    /**
      | Returns the target value towards which
      | the smoothed value is currently moving.
      |
      */
    pub fn get_target_value(&self) -> FloatType {
        
        todo!();
        /*
            return target;
        */
    }

    /**
      | Sets the current value and the target
      | value.
      | 
      | -----------
      | @param newValue
      | 
      | the new value to take
      |
      */
    pub fn set_current_and_target_value(&mut self, new_value: FloatType)  {
        
        todo!();
        /*
            target = currentValue = newValue;
            countdown = 0;
        */
    }

    /**
      | Applies a smoothed gain to a stream of
      | samples S[i] *= gain
      | 
      | -----------
      | @param samples
      | 
      | Pointer to a raw array of samples
      | ----------
      | @param numSamples
      | 
      | Length of array of samples
      |
      */
    pub fn apply_gain_to_stream_of_samples(
        &mut self, 
        samples:     *mut FloatType,
        num_samples: i32)  {
        
        todo!();
        /*
            jassert (numSamples >= 0);

            if (isSmoothing())
            {
                for (int i = 0; i < numSamples; ++i)
                    samples[i] *= getNextSmoothedValue();
            }
            else
            {
                FloatVectorOperations::multiply (samples, target, numSamples);
            }
        */
    }

    /**
      | Computes output as a smoothed gain applied
      | to a stream of samples. Sout[i] = Sin[i]
      | * gain
      | 
      | -----------
      | @param samplesOut
      | 
      | A pointer to a raw array of output samples
      | ----------
      | @param samplesIn
      | 
      | A pointer to a raw array of input samples
      | ----------
      | @param numSamples
      | 
      | The length of the array of samples
      |
      */
    pub fn apply_gain_to_output(
        &mut self, 
        samples_out: *mut FloatType,
        samples_in:  *const FloatType,
        num_samples: i32)  {
        
        todo!();
        /*
            jassert (numSamples >= 0);

            if (isSmoothing())
            {
                for (int i = 0; i < numSamples; ++i)
                    samplesOut[i] = samplesIn[i] * getNextSmoothedValue();
            }
            else
            {
                FloatVectorOperations::multiply (samplesOut, samplesIn, target, numSamples);
            }
        */
    }

    /**
      | Applies a smoothed gain to a buffer
      |
      */
    pub fn apply_gain_to_buffer(
        &mut self, 
        buffer:      &mut AudioBuffer<FloatType>,
        num_samples: i32)  {
        
        todo!();
        /*
            jassert (numSamples >= 0);

            if (isSmoothing())
            {
                if (buffer.getNumChannels() == 1)
                {
                    auto* samples = buffer.getWritePointer (0);

                    for (int i = 0; i < numSamples; ++i)
                        samples[i] *= getNextSmoothedValue();
                }
                else
                {
                    for (auto i = 0; i < numSamples; ++i)
                    {
                        auto gain = getNextSmoothedValue();

                        for (int channel = 0; channel < buffer.getNumChannels(); channel++)
                            buffer.setSample (channel, i, buffer.getSample (channel, i) * gain);
                    }
                }
            }
            else
            {
                buffer.applyGain (0, numSamples, target);
            }
        */
    }
    
    pub fn get_next_smoothed_value(&mut self) -> FloatType {
        
        todo!();
        /*
            return static_cast <SmoothedValueType*> (this)->getNextValue();
        */
    }
}

