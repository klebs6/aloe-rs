crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Limiter.h]

/**
  | A simple limiter with standard threshold
  | and release time controls, featuring
  | two compressors and a hard clipper at
  | 0 dB.
  | 
  | @tags{DSP}
  |
  */
#[derive(Default)]
pub struct Limiter<SampleType: FloatSample> {
    first_stage_compressor:  Compressor<SampleType>,
    second_stage_compressor: Compressor<SampleType>,
    output_volume:           SmoothedValue<SampleType,value_smoothing_types::Linear>,
    sample_rate:             f64,        // default = 44100.0
    thresholddb:             SampleType, // default = -10.0
    release_time:            SampleType, // default = 100.0
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Limiter.cpp]
impl<SampleType: FloatSample> Limiter<SampleType> {

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
            const auto numChannels = outputBlock.getNumChannels();
            const auto numSamples  = outputBlock.getNumSamples();

            jassert (inputBlock.getNumChannels() == numChannels);
            jassert (inputBlock.getNumSamples()  == numSamples);

            if (context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            firstStageCompressor.process (context);

            auto secondContext = ProcessContextReplacing<SampleType> (outputBlock);
            secondStageCompressor.process (secondContext);

            outputBlock.multiplyBy (outputVolume);

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                FloatVectorOperations::clip (outputBlock.getChannelPointer (channel), outputBlock.getChannelPointer (channel),
                                             (SampleType) -1.0, (SampleType) 1.0, (int) numSamples);
            }
        */
    }
    
    /**
      | Sets the threshold in dB of the limiter.
      |
      */
    pub fn set_threshold(&mut self, new_threshold: SampleType)  {
        
        todo!();
        /*
            thresholddB = newThreshold;
        update();
        */
    }
    
    /**
      | Sets the release time in milliseconds
      | of the limiter.
      |
      */
    pub fn set_release(&mut self, new_release: SampleType)  {
        
        todo!();
        /*
            releaseTime = newRelease;
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

        firstStageCompressor.prepare (spec);
        secondStageCompressor.prepare (spec);

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
            firstStageCompressor.reset();
        secondStageCompressor.reset();

        outputVolume.reset (sampleRate, 0.001);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
        firstStageCompressor.setThreshold ((SampleType) -10.0);
        firstStageCompressor.setRatio     ((SampleType) 4.0);
        firstStageCompressor.setAttack    ((SampleType) 2.0);
        firstStageCompressor.setRelease   ((SampleType) 200.0);

        secondStageCompressor.setThreshold (thresholddB);
        secondStageCompressor.setRatio     ((SampleType) 1000.0);
        secondStageCompressor.setAttack    ((SampleType) 0.001);
        secondStageCompressor.setRelease   (releaseTime);

        auto ratioInverse = (SampleType) (1.0 / 4.0);

        auto gain = (SampleType) std::pow (10.0, 10.0 * (1.0 - ratioInverse) / 40.0);
        gain *= Decibels::decibelsToGain (-thresholddB, (SampleType) -100.0);

        outputVolume.setTargetValue (gain);
        */
    }
}
