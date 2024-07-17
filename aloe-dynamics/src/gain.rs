crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Gain.h]

/**
  | Applies a gain to audio samples as single
  | samples or AudioBlocks.
  | 
  | @tags{DSP}
  |
  */
#[derive(Default)]
pub struct Gain<FloatType: num::Float> {
    gain:                  SmoothedValue<FloatType>,
    sample_rate:           f64, // default = 0
    ramp_duration_seconds: f64, // default = 0
}

impl<FloatType: num::Float> Gain<FloatType> {

    /**
      | Applies a new gain as a linear value.
      |
      */
    pub fn set_gain_linear(&mut self, new_gain: FloatType)  {
        
        todo!();
        /*
            gain.setTargetValue (newGain);
        */
    }

    /**
      | Applies a new gain as a decibel value.
      |
      */
    pub fn set_gain_decibels(&mut self, new_gain_decibels: FloatType)  {
        
        todo!();
        /*
            setGainLinear (Decibels::decibelsToGain<FloatType> (newGainDecibels));
        */
    }

    /**
      | Returns the current gain as a linear
      | value.
      |
      */
    pub fn get_gain_linear(&self) -> FloatType {
        
        todo!();
        /*
            return gain.getTargetValue();
        */
    }

    /**
      | Returns the current gain in decibels.
      |
      */
    pub fn get_gain_decibels(&self) -> FloatType {
        
        todo!();
        /*
            return Decibels::gainToDecibels<FloatType> (getGainLinear());
        */
    }

    /**
      | Sets the length of the ramp used for smoothing
      | gain changes.
      |
      */
    pub fn set_ramp_duration_seconds(&mut self, new_duration_seconds: f64)  {
        
        todo!();
        /*
            if (rampDurationSeconds != newDurationSeconds)
            {
                rampDurationSeconds = newDurationSeconds;
                reset();
            }
        */
    }

    /**
      | Returns the ramp duration in seconds.
      |
      */
    pub fn get_ramp_duration_seconds(&self) -> f64 {
        
        todo!();
        /*
            return rampDurationSeconds;
        */
    }

    /**
      | Returns true if the current value is
      | currently being interpolated.
      |
      */
    pub fn is_smoothing(&self) -> bool {
        
        todo!();
        /*
            return gain.isSmoothing();
        */
    }
    
    /**
      | Called before processing starts.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;
            reset();
        */
    }

    /**
      | Resets the internal state of the gain
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            if (sampleRate > 0)
                gain.reset (sampleRate, rampDurationSeconds);
        */
    }

    /**
      | Returns the result of processing a single
      | sample.
      |
      */
    pub fn process_sample<SampleType>(&mut self, s: SampleType) -> SampleType {
    
        todo!();
        /*
            return s * gain.getNextValue();
        */
    }

    /**
      | Processes the input and output buffers
      | supplied in the processing context.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            auto&& inBlock  = context.getInputBlock();
            auto&& outBlock = context.getOutputBlock();

            jassert (inBlock.getNumChannels() == outBlock.getNumChannels());
            jassert (inBlock.getNumSamples() == outBlock.getNumSamples());

            auto len         = inBlock.getNumSamples();
            auto numChannels = inBlock.getNumChannels();

            if (context.isBypassed)
            {
                gain.skip (static_cast<int> (len));

                if (context.usesSeparateInputAndOutputBlocks())
                    outBlock.copyFrom (inBlock);

                return;
            }

            if (numChannels == 1)
            {
                auto* src = inBlock.getChannelPointer (0);
                auto* dst = outBlock.getChannelPointer (0);

                for (size_t i = 0; i < len; ++i)
                    dst[i] = src[i] * gain.getNextValue();
            }
            else
            {
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255 6386)
                auto* gains = static_cast<FloatType*> (alloca (sizeof (FloatType) * len));

                for (size_t i = 0; i < len; ++i)
                    gains[i] = gain.getNextValue();
                ALOE_END_IGNORE_WARNINGS_MSVC

                for (size_t chan = 0; chan < numChannels; ++chan)
                    FloatVectorOperations::multiply (outBlock.getChannelPointer (chan),
                                                     inBlock.getChannelPointer (chan),
                                                     gains, static_cast<int> (len));
            }
        */
    }
}
