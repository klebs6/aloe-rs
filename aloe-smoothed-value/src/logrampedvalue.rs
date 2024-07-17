crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_LogRampedValue.h]

/**
  | Utility class for logarithmically
  | smoothed linear values.
  | 
  | Logarithmically smoothed values can
  | be more relevant than linear ones for
  | specific cases such as algorithm change
  | smoothing, using two of them in opposite
  | directions.
  | 
  | The gradient of the logarithmic/exponential
  | slope can be configured by calling LogRampedValue::setLogParameters.
  | 
  | @see SmoothedValue
  | 
  | @tags{DSP}
  |
  */
#[derive(Default)]
pub struct LogRampedValue<FloatType> {
    //base:                      SmoothedValueBase<LogRampedValue<FloatType>,FloatType>,
    increasing_rate_of_change: bool,      // default = true
    b:                         FloatType, //= Decibels::decibelsToGain ((FloatType) -40);
    steps_to_target:           i32,       // default = 0
    temp:                      FloatType, // default = 0
    source:                    FloatType, // default = 0
    r:                         FloatType, // default = 0
    d:                         FloatType, // default = 1
}

impl<FloatType> LogRampedValue<FloatType> {
    
    pub fn new(initial_value: FloatType) -> Self {
    
        todo!();
        /*


            // Visual Studio can't handle base class initialisation with CRTP
            this->currentValue = initialValue;
            this->target = initialValue;
        */
    }
    
    /**
      | Sets the behaviour of the log ramp.
      | 
      | -----------
      | @param midPointAmplitudedB
      | 
      | Sets the amplitude of the mid point in
      | decibels, with the target value at 0
      | dB and the initial value at -inf dB
      | ----------
      | @param rateOfChangeShouldIncrease
      | 
      | If true then the ramp starts shallow
      | and gets progressively steeper, if
      | false then the ramp is initially steep
      | and flattens out as you approach the
      | target value
      |
      */
    pub fn set_log_parameters(&mut self, 
        mid_point_amplitudedb:          FloatType,
        rate_of_change_should_increase: bool)  {
        
        todo!();
        /*
            jassert (midPointAmplitudedB < (FloatType) 0.0);
            B = Decibels::decibelsToGain (midPointAmplitudedB);

            increasingRateOfChange = rateOfChangeShouldIncrease;
        */
    }

    /**
      | Reset to a new sample rate and ramp length.
      | 
      | -----------
      | @param sampleRate
      | 
      | The sample rate
      | ----------
      | @param rampLengthInSeconds
      | 
      | The duration of the ramp in seconds
      |
      */
    pub fn reset_to_a_new_sample_rate_and_ramp_length(
        &mut self, 
        sample_rate:            f64,
        ramp_length_in_seconds: f64)  {
        
        todo!();
        /*
            jassert (sampleRate > 0 && rampLengthInSeconds >= 0);
            reset ((int) std::floor (rampLengthInSeconds * sampleRate));
        */
    }

    /**
      | Set a new ramp length directly in samples.
      | 
      | -----------
      | @param numSteps
      | 
      | The number of samples over which the
      | ramp should be active
      |
      */
    pub fn reset(&mut self, num_steps: i32)  {
        
        todo!();
        /*
            stepsToTarget = numSteps;

            this->setCurrentAndTargetValue (this->target);

            updateRampParameters();
        */
    }

    /**
      | Set a new target value.
      | 
      | -----------
      | @param newValue
      | 
      | The new target value
      |
      */
    pub fn set_target_value(&mut self, new_value: FloatType)  {
        
        todo!();
        /*
            if (newValue == this->target)
                return;

            if (stepsToTarget <= 0)
            {
                this->setCurrentAndTargetValue (newValue);
                return;
            }

            this->target = newValue;
            this->countdown = stepsToTarget;
            source = this->currentValue;

            updateRampParameters();
        */
    }

    /**
      | Compute the next value.
      | 
      | -----------
      | @return
      | 
      | Smoothed value
      |
      */
    pub fn get_next_value(&mut self) -> FloatType {
        
        todo!();
        /*
            if (! this->isSmoothing())
                return this->target;

            --(this->countdown);

            temp *= r; temp += d;
            this->currentValue = jmap (temp, source, this->target);

            return this->currentValue;
        */
    }

    /**
      | Skip the next numSamples samples.
      | 
      | This is identical to calling getNextValue
      | numSamples times. @see getNextValue
      |
      */
    pub fn skip(&mut self, num_samples: i32) -> FloatType {
        
        todo!();
        /*
            if (numSamples >= this->countdown)
            {
                this->setCurrentAndTargetValue (this->target);
                return this->target;
            }

            this->countdown -= numSamples;

            auto rN = (FloatType) std::pow (r, numSamples);
            temp *= rN;
            temp += d * (rN - (FloatType) 1) / (r - (FloatType) 1);

            this->currentValue = jmap (temp, source, this->target);
            return this->currentValue;
        */
    }
    
    pub fn update_ramp_parameters(&mut self)  {
        
        todo!();
        /*
            auto D = increasingRateOfChange ? B : (FloatType) 1 - B;
            auto base = ((FloatType) 1 / D) - (FloatType) 1;
            r = std::pow (base, (FloatType) 2 / (FloatType) stepsToTarget);
            auto rN = std::pow (r, (FloatType) stepsToTarget);
            d = (r - (FloatType) 1) / (rN - (FloatType) 1);
            temp = 0;
        */
    }
}
