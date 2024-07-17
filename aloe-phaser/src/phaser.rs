crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Phaser.h]

/**
  | A 6 stage phaser that modulates first
  | order all-pass filters to create sweeping
  | notches in the magnitude frequency
  | response.
  | 
  | This audio effect can be controlled
  | with standard phaser parameters: the
  | speed and depth of the LFO controlling
  | the frequency response, a mix control,
  | a feedback control, and the centre frequency
  | of the modulation.
  | 
  | @tags{DSP}
  |
  */
pub struct Phaser<SampleType: FloatSample> {

    osc:                   Oscillator<SampleType>,
    filters:               Vec<Box<FirstOrderTPTFilter<SampleType>>>,
    osc_volume:            LinearSmoothedValue<SampleType>,
    feedback_volume:       Vec<LinearSmoothedValue<SampleType>>, // default = 2 
    dry_wet:               DryWetMixer<SampleType>,
    last_output:           Vec<SampleType>,          // default = 2 
    buffer_frequency:      AudioBuffer<SampleType>,
    norm_centre_frequency: SampleType,               // default = 0.5
    sample_rate:           f64,                      // default = 44100.0
    update_counter:        i32,                      // default = 0
    rate:                  SampleType,               // default = 1.0
    depth:                 SampleType,               // default = 0.5
    feedback:              SampleType,               // default = 0.0
    mix:                   SampleType,               // default = 0.5
    centre_frequency:      SampleType,               // default = 1300.0
}

impl<SampleType: FloatSample> Default for Phaser<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*
        auto oscFunction = [] (SampleType x) { return std::sin (x); };
        osc.initialise (oscFunction);

        for (auto n = 0; n < numStages; ++n)
        {
            filters.add (new FirstOrderTPTFilter<SampleType>());
            filters[n]->setType (FirstOrderTPTFilterType::allpass);
        }

        dryWet.setMixingRule (DryWetMixingRule::linear)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Phaser.cpp]
impl<SampleType: FloatSample> Phaser<SampleType> {

    pub const maxUpdateCounter: i32 = 4;
    pub const numStages:        i32 = 6;

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
            jassert (inputBlock.getNumChannels() == lastOutput.size());
            jassert (inputBlock.getNumSamples()  == numSamples);

            if (context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            int numSamplesDown = 0;
            auto counter = updateCounter;

            for (size_t i = 0; i < numSamples; ++i)
            {
                if (counter == 0)
                    numSamplesDown++;

                counter++;

                if (counter == maxUpdateCounter)
                    counter = 0;
            }

            if (numSamplesDown > 0)
            {
                auto freqBlock = AudioBlock<SampleType>(bufferFrequency).getSubBlock (0, (size_t) numSamplesDown);
                auto contextFreq = ProcessContextReplacing<SampleType> (freqBlock);
                freqBlock.clear();

                osc.process (contextFreq);
                freqBlock.multiplyBy (oscVolume);
            }

            auto* freqSamples = bufferFrequency.getWritePointer (0);

            for (int i = 0; i < numSamplesDown; ++i)
            {
                auto lfo = jlimit (static_cast<SampleType> (0.0),
                                   static_cast<SampleType> (1.0),
                                   freqSamples[i] + normCentreFrequency);

                freqSamples[i] = mapToLog10 (lfo, static_cast<SampleType> (20.0),
                                             static_cast<SampleType> (jmin (20000.0, 0.49 * sampleRate)));
            }

            auto currentFrequency = filters[0]->getCutoffFrequency();
            dryWet.pushDrySamples (inputBlock);

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                counter = updateCounter;
                int k = 0;

                auto* inputSamples  = inputBlock .getChannelPointer (channel);
                auto* outputSamples = outputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = inputSamples[i];
                    auto output = input - lastOutput[channel];

                    if (i == 0 && counter != 0)
                        for (int n = 0; n < numStages; ++n)
                            filters[n]->setCutoffFrequency (currentFrequency);

                    if (counter == 0)
                    {
                        for (int n = 0; n < numStages; ++n)
                            filters[n]->setCutoffFrequency (freqSamples[k]);

                        k++;
                    }

                    for (int n = 0; n < numStages; ++n)
                        output = filters[n]->processSample ((int) channel, output);

                    outputSamples[i] = output;
                    lastOutput[channel] = output * feedbackVolume[channel].getNextValue();

                    counter++;

                    if (counter == maxUpdateCounter)
                        counter = 0;
                }
            }

            dryWet.mixWetSamples (outputBlock);
            updateCounter = (updateCounter + (int) numSamples) % maxUpdateCounter;
        */
    }
    
    /**
      | Sets the rate (in Hz) of the LFO modulating
      | the phaser all-pass filters. This rate
      | must be lower than 100 Hz.
      |
      */
    pub fn set_rate(&mut self, new_rate_hz: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newRateHz, static_cast<SampleType> (100.0)));

        rate = newRateHz;
        update();
        */
    }
    
    /**
      | Sets the volume (between 0 and 1) of the
      | LFO modulating the phaser all-pass
      | filters.
      |
      */
    pub fn set_depth(&mut self, new_depth: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndNotGreaterThan (newDepth, static_cast<SampleType> (1.0)));

        depth = newDepth;
        update();
        */
    }
    
    /**
      | Sets the centre frequency (in Hz) of
      | the phaser all-pass filters modulation.
      |
      */
    pub fn set_centre_frequency(&mut self, new_centre_hz: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newCentreHz, static_cast<SampleType> (sampleRate * 0.5)));

        centreFrequency = newCentreHz;
        normCentreFrequency = mapFromLog10 (centreFrequency, static_cast<SampleType> (20.0), static_cast<SampleType> (jmin (20000.0, 0.49 * sampleRate)));
        */
    }
    
    /**
      | Sets the feedback volume (between -1
      | and 1) of the phaser. Negative can be
      | used to get specific phaser sounds.
      |
      */
    pub fn set_feedback(&mut self, new_feedback: SampleType)  {
        
        todo!();
        /*
            jassert (newFeedback >= static_cast<SampleType> (-1.0) && newFeedback <= static_cast<SampleType> (1.0));

        feedback = newFeedback;
        update();
        */
    }
    
    /**
      | Sets the amount of dry and wet signal
      | in the output of the phaser (between
      | 0 for full dry and 1 for full wet).
      |
      */
    pub fn set_mix(&mut self, new_mix: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndNotGreaterThan (newMix, static_cast<SampleType> (1.0)));

        mix = newMix;
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

        for (auto n = 0; n < numStages; ++n)
            filters[n]->prepare (spec);

        dryWet.prepare (spec);
        feedbackVolume.resize (spec.numChannels);
        lastOutput.resize (spec.numChannels);

        auto specDown = spec;
        specDown.sampleRate /= (double) maxUpdateCounter;
        specDown.maximumBlockSize = specDown.maximumBlockSize / (uint32) maxUpdateCounter + 1;

        osc.prepare (specDown);
        bufferFrequency.setSize (1, (int) specDown.maximumBlockSize, false, false, true);

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
            std::fill (lastOutput.begin(), lastOutput.end(), static_cast<SampleType> (0));

        for (auto n = 0; n < numStages; ++n)
            filters[n]->reset();

        osc.reset();
        dryWet.reset();

        oscVolume.reset (sampleRate / (double) maxUpdateCounter, 0.05);

        for (auto& vol : feedbackVolume)
            vol.reset (sampleRate, 0.05);

        updateCounter = 0;
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            osc.setFrequency (rate);
        oscVolume.setTargetValue (depth * (SampleType) 0.5);
        dryWet.setWetMixProportion (mix);

        for (auto& vol : feedbackVolume)
            vol.setTargetValue (feedback);
        */
    }
}
