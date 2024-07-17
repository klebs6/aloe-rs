crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_DryWetMixer.h]

/**
  | A processor to handle dry/wet mixing
  | of two audio signals, where the wet signal
  | may have additional latency.
  | 
  | Once a DryWetMixer object is configured,
  | push the dry samples using pushDrySamples
  | and mix into the fully wet samples using
  | mixWetSamples.
  | 
  | @tags{DSP}
  |
  */
pub struct DryWetMixer<SampleType: FloatSample> {
    dry_volume:                     LinearSmoothedValue<SampleType>,
    wet_volume:                     LinearSmoothedValue<SampleType>,
    dry_delay_line:                 ThiranDelayLine<SampleType>,
    buffer_dry:                     AudioBuffer<SampleType>,
    mix:                            SampleType,  // default = 1.0
    current_mixing_rule:            DryWetMixingRule,  // default = DryWetMixingRule::linear
    sample_rate:                    f64,         // default = 44100.0
    offset_in_buffer:               usize,       // default = 0
    num_used_samples:               usize,       // default = 0
    maximum_wet_latency_in_samples: i32,         // default = 0
}

impl<SampleType: FloatSample> Default for DryWetMixer<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*
        : dry_wet_mixer(0),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_DryWetMixer.cpp]
impl<SampleType: FloatSample> DryWetMixer<SampleType> {
    
    pub fn new(maximum_wet_latency_in_samples_in: i32) -> Self {
    
        todo!();
        /*
        : dry_delay_line(maximumWetLatencyInSamplesIn),
        : maximum_wet_latency_in_samples(maximumWetLatencyInSamplesIn),

            dryDelayLine.setDelay (0);

        update();
        reset();
        */
    }
    
    /**
      | Sets the mix rule.
      |
      */
    pub fn set_mixing_rule(&mut self, new_rule: DryWetMixingRule)  {
        
        todo!();
        /*
            currentMixingRule = newRule;
        update();
        */
    }
    
    /**
      | Sets the current dry/wet mix proportion,
      | with 0.0 being full dry and 1.0 being
      | fully wet.
      |
      */
    pub fn set_wet_mix_proportion(&mut self, new_wet_mix_proportion: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndNotGreaterThan (newWetMixProportion, 1.0));

        mix = jlimit (static_cast<SampleType> (0.0), static_cast<SampleType> (1.0), newWetMixProportion);
        update();
        */
    }
    
    /**
      | Sets the relative latency of the wet
      | signal path compared to the dry signal
      | path, and thus the amount of latency
      | compensation that will be added to the
      | dry samples in this processor.
      |
      */
    pub fn set_wet_latency(&mut self, wet_latency_samples: SampleType)  {
        
        todo!();
        /*
            dryDelayLine.setDelay (wetLatencySamples);
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

        dryDelayLine.prepare (spec);
        bufferDry.setSize ((int) spec.numChannels, (int) spec.maximumBlockSize, false, false, true);

        update();
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
            dryVolume.reset (sampleRate, 0.05);
        wetVolume.reset (sampleRate, 0.05);

        dryDelayLine.reset();

        offsetInBuffer = 0;
        numUsedSamples = 0;
        */
    }

    /**
      | Copies the dry path samples into an internal
      | delay line.
      |
      */
    pub fn push_dry_samples(&mut self, dry_samples: AudioBlock<SampleType>)  {
        
        todo!();
        /*
            const auto remainingSpace = (size_t) bufferDry.getNumSamples() - numUsedSamples;

        jassert (drySamples.getNumChannels() <= (size_t) bufferDry.getNumChannels());
        jassert (drySamples.getNumSamples() <= remainingSpace);

        auto blocks = getFirstAndSecondPartBlocks (bufferDry,
                                                   (offsetInBuffer + numUsedSamples) % (size_t) bufferDry.getNumSamples(),
                                                   drySamples.getNumChannels(),
                                                   jmin (drySamples.getNumSamples(), remainingSpace));

        const auto processSubBlock = [this, &drySamples] (AudioBlock<SampleType> block, size_t startOffset)
        {
            auto inputBlock = drySamples.getSubBlock (startOffset, block.getNumSamples());

            if (maximumWetLatencyInSamples == 0)
                block.copyFrom (inputBlock);
            else
                dryDelayLine.process (ProcessContextNonReplacing<SampleType> (inputBlock, block));
        };

        processSubBlock (blocks.first, 0);

        if (blocks.second.getNumSamples() > 0)
            processSubBlock (blocks.second, blocks.first.getNumSamples());

        numUsedSamples += blocks.first.getNumSamples() + blocks.second.getNumSamples();
        */
    }
    
    /**
      | Mixes the supplied wet samples with
      | the latency-compensated dry samples
      | from pushDrySamples.
      | 
      | -----------
      | @param wetSamples
      | 
      | Input: The AudioBlock references fully
      | wet samples. Output: The AudioBlock
      | references the wet samples mixed with
      | the latency compensated dry samples.
      | 
      | @see pushDrySamples
      |
      */
    pub fn mix_wet_samples(&mut self, in_out_block: AudioBlock<SampleType>)  {
        
        todo!();
        /*
            inOutBlock.multiplyBy (wetVolume);

        jassert (inOutBlock.getNumSamples() <= numUsedSamples);

        auto blocks = getFirstAndSecondPartBlocks (bufferDry,
                                                   offsetInBuffer,
                                                   inOutBlock.getNumChannels(),
                                                   jmin (inOutBlock.getNumSamples(), numUsedSamples));
        blocks.first.multiplyBy (dryVolume);
        inOutBlock.add (blocks.first);

        if (blocks.second.getNumSamples() != 0)
        {
            blocks.second.multiplyBy (dryVolume);
            inOutBlock.getSubBlock (blocks.first.getNumSamples()).add (blocks.second);
        }

        const auto samplesToCopy = blocks.first.getNumSamples() + blocks.second.getNumSamples();
        offsetInBuffer = (offsetInBuffer + samplesToCopy) % (size_t) bufferDry.getNumSamples();
        numUsedSamples -= samplesToCopy;
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            SampleType dryValue, wetValue;

        switch (currentMixingRule)
        {
            case DryWetMixingRule::balanced:
                dryValue = static_cast<SampleType> (2.0) * jmin (static_cast<SampleType> (0.5), static_cast<SampleType> (1.0) - mix);
                wetValue = static_cast<SampleType> (2.0) * jmin (static_cast<SampleType> (0.5), mix);
                break;

            case DryWetMixingRule::linear:
                dryValue = static_cast<SampleType> (1.0) - mix;
                wetValue = mix;
                break;

            case DryWetMixingRule::sin3dB:
                dryValue = static_cast<SampleType> (std::sin (0.5 * MathConstants<double>::pi * (1.0 - mix)));
                wetValue = static_cast<SampleType> (std::sin (0.5 * MathConstants<double>::pi * mix));
                break;

            case DryWetMixingRule::sin4p5dB:
                dryValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * (1.0 - mix)), 1.5));
                wetValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * mix), 1.5));
                break;

            case DryWetMixingRule::sin6dB:
                dryValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * (1.0 - mix)), 2.0));
                wetValue = static_cast<SampleType> (std::pow (std::sin (0.5 * MathConstants<double>::pi * mix), 2.0));
                break;

            case DryWetMixingRule::squareRoot3dB:
                dryValue = std::sqrt (static_cast<SampleType> (1.0) - mix);
                wetValue = std::sqrt (mix);
                break;

            case DryWetMixingRule::squareRoot4p5dB:
                dryValue = static_cast<SampleType> (std::pow (std::sqrt (1.0 - mix), 1.5));
                wetValue = static_cast<SampleType> (std::pow (std::sqrt (mix), 1.5));
                break;

            default:
                dryValue = jmin (static_cast<SampleType> (0.5), static_cast<SampleType> (1.0) - mix);
                wetValue = jmin (static_cast<SampleType> (0.5), mix);
                break;
        }

        dryVolume.setTargetValue (dryValue);
        wetVolume.setTargetValue (wetValue);
        */
    }
}
