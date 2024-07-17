crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_SmoothedValue.h]

/**
  | A utility class for values that need
  | smoothing to avoid audio glitches.
  | 
  | A ValueSmoothingTypes::Linear template
  | parameter selects linear smoothing,
  | which increments the SmoothedValue
  | linearly towards its target value.
  | 
  | @code SmoothedValue<float, ValueSmoothingTypes::Linear>
  | yourSmoothedValue; @endcode
  | 
  | A ValueSmoothingTypes::Multiplicative
  | template parameter selects multiplicative
  | smoothing increments towards the target
  | value.
  | 
  | @code SmoothedValue<float, ValueSmoothingTypes::Multiplicative>
  | yourSmoothedValue; @endcode
  | 
  | Multiplicative smoothing is useful
  | when you are dealing with exponential/logarithmic
  | values like volume in dB or frequency
  | in Hz. For example a 12 step ramp from
  | 440.0 Hz (A4) to 880.0 Hz (A5) will increase
  | the frequency with an equal temperament
  | tuning across the octave. A 10 step smoothing
  | from 1.0 (0 dB) to 3.16228 (10 dB) will
  | increase the value in increments of
  | 1 dB.
  | 
  | Note that when you are using multiplicative
  | smoothing you cannot ever reach a target
  | value of zero!
  | 
  | @tags{Audio}
  |
  */
pub struct SmoothedValue<FloatType: num::Float,SmoothingType = value_smoothing_types::Linear > {

    //base:            SmoothedValueBase<FloatType,SmoothedValue<FloatType,SmoothingType>>,
    step:            FloatType, // default = FloatType()
    steps_to_target: i32,       // default = 0

    p0:              PhantomData<SmoothingType>,
}

pub mod detail {

    use super::*;

    lazy_static!{
        /*
        template <typename T>
            using LinearVoid = typename std::enable_if <std::is_same <T, ValueSmoothingTypes::Linear>::value, void>::type;

            template <typename T>
            using MultiplicativeVoid = typename std::enable_if <std::is_same <T, ValueSmoothingTypes::Multiplicative>::value, void>::type;
        */
    }

    pub type MultiplicativeVoid<T> = T; //TODO
    pub type LinearVoid<T>         = T; //TODO
}

impl<FloatType: num::Float,SmoothingType> Default for SmoothedValue<FloatType,SmoothingType> {
    
    fn default() -> Self {
        todo!();
        /*


            : SmoothedValue ((FloatType) (std::is_same<SmoothingType, ValueSmoothingTypes::Linear>::value ? 0 : 1)
        */
    }
}

impl<FloatType: num::Float,SmoothingType> SmoothedValue<FloatType,SmoothingType> {

    pub fn new(initial_value: FloatType) -> Self {
    
        todo!();
        /*


            // Multiplicative smoothed values cannot ever reach 0!
            jassert (! (std::is_same<SmoothingType, ValueSmoothingTypes::Multiplicative>::value && initialValue == 0));

            // Visual Studio can't handle base class initialisation with CRTP
            this->currentValue = initialValue;
            this->target = this->currentValue;
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
        */
    }

    /**
      | Set the next value to ramp towards.
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

            // Multiplicative smoothed values cannot ever reach 0!
            jassert (! (std::is_same<SmoothingType, ValueSmoothingTypes::Multiplicative>::value && newValue == 0));

            this->target = newValue;
            this->countdown = stepsToTarget;

            setStepSize();
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

            if (this->isSmoothing())
                setNextValue();
            else
                this->currentValue = this->target;

            return this->currentValue;
        */
    }
    
    /**
      | Skip the next numSamples samples. This
      | is identical to calling getNextValue
      | numSamples times. It returns the new
      | current value. @see getNextValue
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

            skipCurrentValue (numSamples);

            this->countdown -= numSamples;
            return this->currentValue;
        */
    }
    
    pub fn set_step_size_linear(&mut self) -> detail::LinearVoid<SmoothingType> {
    
        todo!();
        /*
            step = (this->target - this->currentValue) / (FloatType) this->countdown;
        */
    }
    
    pub fn set_step_size_multiplicative(&mut self) -> detail::MultiplicativeVoid<SmoothingType> {
    
        todo!();
        /*
            step = std::exp ((std::log (std::abs (this->target)) - std::log (std::abs (this->currentValue))) / (FloatType) this->countdown);
        */
    }
    
    pub fn set_next_value_linear(&mut self) -> detail::LinearVoid<SmoothingType> {
    
        todo!();
        /*
            this->currentValue += step;
        */
    }
    
    pub fn set_next_value_multiplicative(&mut self) -> detail::MultiplicativeVoid<SmoothingType> {
    
        todo!();
        /*
            this->currentValue *= step;
        */
    }
    
    pub fn skip_current_value_linear(&mut self, num_samples: i32) -> detail::LinearVoid<SmoothingType> {
    
        todo!();
        /*
            this->currentValue += step * (FloatType) numSamples;
        */
    }
    
    pub fn skip_current_value_multiplicative(&mut self, num_samples: i32) -> detail::MultiplicativeVoid<SmoothingType> {
    
        todo!();
        /*
            this->currentValue *= (FloatType) std::pow (step, numSamples);
        */
    }
}
