crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Bias.h]

/**
  | Adds a DC offset (voltage bias) to the
  | audio samples.
  | 
  | This is a useful preprocessor for asymmetric
  | waveshaping when a waveshaper is bookended
  | by a bias on input and a DC-offset removing
  | high pass filter on output.
  | 
  | This is an extremely simple bias implementation
  | that simply adds a value to a signal.
  | More complicated bias behaviours exist
  | in real circuits - for your homework
  | ;).
  | 
  | @tags{DSP}
  |
  */
#[derive(Default)]
pub struct Bias<FloatType: num::Float> {
    bias:                  SmoothedValue<FloatType>,
    sample_rate:           f64, // default = 0
    ramp_duration_seconds: f64, // default = 0
}

impl<FloatType: num::Float> Bias<FloatType> {

    /**
      | Sets the DC bias
      | 
      | -----------
      | @param newBias
      | 
      | DC offset in range [-1, 1]
      |
      */
    pub fn set_bias(&mut self, new_bias: FloatType)  {
        
        todo!();
        /*
            jassert (newBias >= static_cast<FloatType> (-1) && newBias <= static_cast<FloatType> (1));
            bias.setTargetValue (newBias);
        */
    }

    /**
      | Returns the DC bias
      | 
      | -----------
      | @return
      | 
      | DC bias, which should be in the range
      | [-1, 1]
      |
      */
    pub fn get_bias(&self) -> FloatType {
        
        todo!();
        /*
            return bias.getTargetValue();
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
                updateRamp();
            }
        */
    }
    
    pub fn get_ramp_duration_seconds(&self) -> f64 {
        
        todo!();
        /*
            return rampDurationSeconds;
        */
    }

    /**
      | Called before processing starts
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;
            updateRamp();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            bias.reset (sampleRate, rampDurationSeconds);
        */
    }

    /**
      | Returns the result of processing a single
      | sample.
      |
      */
    pub fn process_sample<SampleType>(&mut self, input_sample: SampleType) -> SampleType {
    
        todo!();
        /*
            return inputSample + bias.getNextValue();
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
            jassert (inBlock.getNumSamples()  == outBlock.getNumSamples());

            auto len         = inBlock.getNumSamples();
            auto numChannels = inBlock.getNumChannels();

            if (context.isBypassed)
            {
                bias.skip (static_cast<int> (len));

                if (context.usesSeparateInputAndOutputBlocks())
                    outBlock.copyFrom (inBlock);

                return;
            }

            if (numChannels == 1)
            {
                auto* src = inBlock.getChannelPointer (0);
                auto* dst = outBlock.getChannelPointer (0);

                for (size_t i = 0; i < len; ++i)
                    dst[i] = src[i] + bias.getNextValue();
            }
            else
            {
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255 6386)
                auto* biases = static_cast<FloatType*> (alloca (sizeof (FloatType) * len));

                for (size_t i = 0; i < len; ++i)
                    biases[i] = bias.getNextValue();

                for (size_t chan = 0; chan < numChannels; ++chan)
                    FloatVectorOperations::add (outBlock.getChannelPointer (chan),
                                                inBlock.getChannelPointer (chan),
                                                biases, static_cast<int> (len));
                ALOE_END_IGNORE_WARNINGS_MSVC
            }
        */
    }
    
    pub fn update_ramp(&mut self)  {
        
        todo!();
        /*
            if (sampleRate > 0)
                bias.reset (sampleRate, rampDurationSeconds);
        */
    }
}
