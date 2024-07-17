crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_Panner.h]

/**
  | A processor to perform panning operations
  | on stereo buffers.
  | 
  | @tags{DSP}
  |
  */
pub struct Panner<SampleType: FloatSample> {
    current_rule: PannerRule, // default = Rule::balanced
    pan:          SampleType, // default = 0.0
    left_volume:  SmoothedValue<SampleType>,
    right_volume: SmoothedValue<SampleType>,
    sample_rate:  f64, // default = 44100.0
}

impl<SampleType: FloatSample> Default for Panner<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*
           update();
           reset()
           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_Panner.cpp]
impl<SampleType: FloatSample> Panner<SampleType> {

    /**
      | Processes the input and output samples
      | supplied in the processing context.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();
            auto& outputBlock      = context.getOutputBlock();

            const auto numInputChannels  = inputBlock.getNumChannels();
            const auto numOutputChannels = outputBlock.getNumChannels();
            const auto numSamples        = outputBlock.getNumSamples();

            jassertquiet (inputBlock.getNumSamples() == numSamples);

            if (numOutputChannels != 2 || numInputChannels == 0 || numInputChannels > 2)
                return;

            if (numInputChannels == 2)
            {
                outputBlock.copyFrom (inputBlock);
            }
            else
            {
                outputBlock.getSingleChannelBlock (0).copyFrom (inputBlock);
                outputBlock.getSingleChannelBlock (1).copyFrom (inputBlock);
            }

            if (context.isBypassed)
                return;

            outputBlock.getSingleChannelBlock (0).multiplyBy (leftVolume);
            outputBlock.getSingleChannelBlock (1).multiplyBy (rightVolume);
        */
    }
    
    /**
      | Sets the panning rule.
      |
      */
    pub fn set_rule(&mut self, new_rule: PannerRule)  {
        
        todo!();
        /*
            currentRule = newRule;
        update();
        */
    }
    
    /**
      | Sets the current panning value, between
      | -1 (full left) and 1 (full right).
      |
      */
    pub fn set_pan(&mut self, new_pan: SampleType)  {
        
        todo!();
        /*
            jassert (newPan >= -1.0 && newPan <= 1.0);

        pan = jlimit (static_cast<SampleType> (-1.0), static_cast<SampleType> (1.0), newPan);
        update();
        */
    }
    
    /**
      | Initialises the processor.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            jassert (spec.sampleRate > 0);
        jassert (spec.numChannels > 0);

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
            leftVolume .reset (sampleRate, 0.05);
        rightVolume.reset (sampleRate, 0.05);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            SampleType leftValue, rightValue, boostValue;

        auto normalisedPan = static_cast<SampleType> (0.5) * (pan + static_cast<SampleType> (1.0));

        switch (currentRule)
        {
            case PannerRule::balanced:
                leftValue  = jmin (static_cast<SampleType> (0.5), static_cast<SampleType> (1.0) - normalisedPan);
                rightValue = jmin (static_cast<SampleType> (0.5), normalisedPan);
                boostValue = static_cast<SampleType> (2.0);
                break;

            case PannerRule::linear:
                leftValue  = static_cast<SampleType> (1.0) - normalisedPan;
                rightValue = normalisedPan;
                boostValue = static_cast<SampleType> (2.0);
                break;

            case PannerRule::sin3dB:
                leftValue  = static_cast<SampleType> (std::sin (0.5 * MathConstants<double>::pi * (1.0 - normalisedPan)));
                rightValue = static_cast<SampleType> (std::sin (0.5 * MathConstants<double>::pi * normalisedPan));
                boostValue = std::sqrt (static_cast<SampleType> (2.0));
                break;

            case PannerRule::sin4p5dB:
                leftValue  = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * (1.0 - normalisedPan)), 1.5));
                rightValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * normalisedPan), 1.5));
                boostValue = static_cast<SampleType> (std::pow (2.0, 3.0 / 4.0));
                break;

            case PannerRule::sin6dB:
                leftValue  = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * (1.0 - normalisedPan)), 2.0));
                rightValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * normalisedPan), 2.0));
                boostValue = static_cast<SampleType> (2.0);
                break;

            case PannerRule::squareRoot3dB:
                leftValue  = std::sqrt (static_cast<SampleType> (1.0) - normalisedPan);
                rightValue = std::sqrt (normalisedPan);
                boostValue = std::sqrt (static_cast<SampleType> (2.0));
                break;

            case PannerRule::squareRoot4p5dB:
                leftValue  = static_cast<SampleType> (std::pow (std::sqrt (1.0 - normalisedPan), 1.5));
                rightValue = static_cast<SampleType> (std::pow (std::sqrt (normalisedPan), 1.5));
                boostValue = static_cast<SampleType> (std::pow (2.0, 3.0 / 4.0));
                break;

            default:
                leftValue  = jmin (static_cast<SampleType> (0.5), static_cast<SampleType> (1.0) - normalisedPan);
                rightValue = jmin (static_cast<SampleType> (0.5), normalisedPan);
                boostValue = static_cast<SampleType> (2.0);
                break;
        }

        leftVolume .setTargetValue (leftValue  * boostValue);
        rightVolume.setTargetValue (rightValue * boostValue);
        */
    }
}
