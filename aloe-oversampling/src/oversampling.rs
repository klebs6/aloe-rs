crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_Oversampling.h]

/**
  | A processor that performs multi-channel
  | oversampling.
  | 
  | This class can be configured to do a factor
  | of 2, 4, 8 or 16 times oversampling, using
  | multiple stages, with polyphase allpass
  | IIR filters or FIR filters, and latency
  | compensation.
  | 
  | The principle of oversampling is to
  | increase the sample rate of a given non-linear
  | process to prevent it from creating
  | aliasing. Oversampling works by upsampling
  | the input signal N times, processing
  | the upsampled signal with the increased
  | internal sample rate, then downsampling
  | the result to get back to the original
  | sample rate.
  | 
  | Choose between FIR or IIR filtering
  | depending on your needs in terms of latency
  | and phase distortion. With FIR filters
  | the phase is linear but the latency is
  | maximised. With IIR filtering the phase
  | is compromised around the Nyquist frequency
  | but the latency is minimised.
  | 
  | @see FilterDesign.
  | 
  | @tags{DSP}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Oversampling<SampleType: SampleTypeInterface> {
    factor_oversampling:        usize,                          // default = 1
    num_channels:               usize,                          // default = 1
    stages:                     Vec<Box<OversamplingStage<SampleType>>>,
    is_ready:                   bool,                           // default = false
    should_use_integer_latency: bool,                           // default = false
    delay:                      DelayLine<SampleType,delay_line_interpolation_types::Thiran>, // default = 8 
    fractional_delay:           SampleType,                     // default = 0
}

impl<SampleType: SampleTypeInterface> Drop for Oversampling<SampleType> {

    fn drop(&mut self) {
        todo!();
        /* 
        stages.clear();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_Oversampling.cpp]
impl<SampleType: SampleTypeInterface> Oversampling<SampleType> {

    /**
      | The default constructor.
      | 
      | Note: This creates a "dummy" oversampling
      | stage, which needs to be removed before
      | adding proper oversampling stages.
      | 
      | -----------
      | @param numChannels
      | 
      | the number of channels to process with
      | this object
      | 
      | @see clearOversamplingStages, addOversamplingStage
      |
      */
    pub fn new_from_num_channels(num_channels: Option<usize>) -> Self {

        let num_channels: usize = num_channels.unwrap_or(1);
    
        todo!();
        /*
        : num_channels(newNumChannels),

            jassert (numChannels > 0);

        addDummyOversamplingStage();
        */
    }
    
    /**
      | Constructor.
      | 
      | -----------
      | @param numChannels
      | 
      | the number of channels to process with
      | this object
      | ----------
      | @param factor
      | 
      | the processing will perform 2 ^ factor
      | times oversampling
      | ----------
      | @param type
      | 
      | the type of filter design employed for
      | filtering during oversampling
      | ----------
      | @param isMaxQuality
      | 
      | if the oversampling is done using the
      | maximum quality, where the filters
      | will be more efficient but the CPU load
      | will increase as well
      | ----------
      | @param useIntegerLatency
      | 
      | if true this processor will add some
      | fractional delay at the end of the signal
      | path to ensure that the overall latency
      | of the oversampling is an integer
      |
      */
    pub fn new(
        new_num_channels:    usize,
        new_factor:          usize,
        new_type:            OversamplingFilterType,
        is_maximum_quality:  Option<bool>,
        use_integer_latency: Option<bool>

    ) -> Self {

        let is_maximum_quality = is_maximum_quality.unwrap_or(true);
        let use_integer_latency = use_integer_latency.unwrap_or(false);
    
        todo!();
        /*
        : num_channels(newNumChannels),
        : should_use_integer_latency(useIntegerLatency),

            jassert (isPositiveAndBelow (newFactor, 5) && numChannels > 0);

        if (newFactor == 0)
        {
            addDummyOversamplingStage();
        }
        else if (newType == OversamplingFilterType::filterHalfBandPolyphaseIIR)
        {
            for (size_t n = 0; n < newFactor; ++n)
            {
                auto twUp   = (isMaximumQuality ? 0.10f : 0.12f) * (n == 0 ? 0.5f : 1.0f);
                auto twDown = (isMaximumQuality ? 0.12f : 0.15f) * (n == 0 ? 0.5f : 1.0f);

                auto gaindBStartUp    = (isMaximumQuality ? -90.0f : -70.0f);
                auto gaindBStartDown  = (isMaximumQuality ? -75.0f : -60.0f);
                auto gaindBFactorUp   = (isMaximumQuality ? 10.0f  : 8.0f);
                auto gaindBFactorDown = (isMaximumQuality ? 10.0f  : 8.0f);

                addOversamplingStage (OversamplingFilterType::filterHalfBandPolyphaseIIR,
                                      twUp, gaindBStartUp + gaindBFactorUp * (float) n,
                                      twDown, gaindBStartDown + gaindBFactorDown * (float) n);
            }
        }
        else if (newType == OversamplingFilterType::filterHalfBandFIREquiripple)
        {
            for (size_t n = 0; n < newFactor; ++n)
            {
                auto twUp   = (isMaximumQuality ? 0.10f : 0.12f) * (n == 0 ? 0.5f : 1.0f);
                auto twDown = (isMaximumQuality ? 0.12f : 0.15f) * (n == 0 ? 0.5f : 1.0f);

                auto gaindBStartUp    = (isMaximumQuality ? -90.0f : -70.0f);
                auto gaindBStartDown  = (isMaximumQuality ? -75.0f : -60.0f);
                auto gaindBFactorUp   = (isMaximumQuality ? 10.0f  : 8.0f);
                auto gaindBFactorDown = (isMaximumQuality ? 10.0f  : 8.0f);

                addOversamplingStage (OversamplingFilterType::filterHalfBandFIREquiripple,
                                      twUp, gaindBStartUp + gaindBFactorUp * (float) n,
                                      twDown, gaindBStartDown + gaindBFactorDown * (float) n);
            }
        }
        */
    }
    
    /**
      | Adds a new "dummy" oversampling stage,
      | which does nothing to the signal. Using
      | one can be useful if your application
      | features a customisable oversampling
      | factor and if you want to select the current
      | one from an OwnedArray without changing
      | anything in the processing code.
      | 
      | @see OwnedArray, clearOversamplingStages,
      | addOversamplingStage
      |
      */
    pub fn add_dummy_oversampling_stage(&mut self)  {
        
        todo!();
        /*
            stages.add (new OversamplingDummy<SampleType> (numChannels));
        */
    }
    
    /**
      | Adds a new oversampling stage to the
      | Oversampling class, multiplying the
      | current oversampling factor by two.
      | This is used with the default constructor
      | to create custom oversampling chains,
      | requiring a call to the clearOversamplingStages
      | before any addition.
      | 
      | Note: Upsampling and downsampling
      | filtering have different purposes,
      | the former removes upsampling artefacts
      | while the latter removes useless frequency
      | content created by the oversampled
      | process, so usually the attenuation
      | is increased when upsampling compared
      | to downsampling.
      | 
      | -----------
      | @param normalisedTransitionWidthUp
      | 
      | a value between 0 and 0.5 which specifies
      | how much the transition between passband
      | and stopband is steep, for upsampling
      | filtering (the lower the better)
      | ----------
      | @param stopbandAmplitudedBUp
      | 
      | the amplitude in dB in the stopband for
      | upsampling filtering, must be negative
      | ----------
      | @param normalisedTransitionWidthDown
      | 
      | a value between 0 and 0.5 which specifies
      | how much the transition between passband
      | and stopband is steep, for downsampling
      | filtering (the lower the better)
      | ----------
      | @param stopbandAmplitudedBDown
      | 
      | the amplitude in dB in the stopband for
      | downsampling filtering, must be negative
      | 
      | @see clearOversamplingStages
      |
      */
    pub fn add_oversampling_stage(&mut self, 
        ty:                               OversamplingFilterType,
        normalised_transition_width_up:   f32,
        stopband_amplitudedb_up:          f32,
        normalised_transition_width_down: f32,
        stopband_amplitudedb_down:        f32)  {
        
        todo!();
        /*
            if (type == OversamplingFilterType::filterHalfBandPolyphaseIIR)
        {
            stages.add (new Oversampling2TimesPolyphaseIIR<SampleType> (numChannels,
                                                                        normalisedTransitionWidthUp,   stopbandAmplitudedBUp,
                                                                        normalisedTransitionWidthDown, stopbandAmplitudedBDown));
        }
        else
        {
            stages.add (new Oversampling2TimesEquirippleFIR<SampleType> (numChannels,
                                                                         normalisedTransitionWidthUp,   stopbandAmplitudedBUp,
                                                                         normalisedTransitionWidthDown, stopbandAmplitudedBDown));
        }

        factorOversampling *= 2;
        */
    }
    
    /**
      | Removes all the previously registered
      | oversampling stages, so you can add
      | your own from scratch.
      | 
      | @see addOversamplingStage, addDummyOversamplingStage
      |
      */
    pub fn clear_oversampling_stages(&mut self)  {
        
        todo!();
        /*
            stages.clear();
        factorOversampling = 1u;
        */
    }
    
    /**
      | Sets if this processor should add some
      | fractional delay at the end of the signal
      | path to ensure that the overall latency
      | of the oversampling is an integer.
      |
      */
    pub fn set_using_integer_latency(&mut self, use_integer_latency: bool)  {
        
        todo!();
        /*
            shouldUseIntegerLatency = useIntegerLatency;
        */
    }
    
    /**
      | Returns the latency in samples of the
      | overall processing. You can use this
      | information in your main processor
      | to compensate the additional latency
      | involved with the oversampling, for
      | example with a dry / wet mixer, and to
      | report the latency to the DAW.
      | 
      | -----------
      | @note
      | 
      | If you have not opted to use an integer
      | latency then the latency may not be integer,
      | so you might need to round its value or
      | to compensate it properly in your processing
      | code since plug-ins can only report
      | integer latency values in samples to
      | the DAW.
      |
      */
    pub fn get_latency_in_samples(&self) -> SampleType {
        
        todo!();
        /*
            auto latency = getUncompensatedLatency();
        return shouldUseIntegerLatency ? latency + fractionalDelay : latency;
        */
    }
    
    pub fn get_uncompensated_latency(&self) -> SampleType {
        
        todo!();
        /*
            auto latency = static_cast<SampleType> (0);
        size_t order = 1;

        for (auto* stage : stages)
        {
            order *= stage->factor;
            latency += stage->getLatencyInSamples() / static_cast<SampleType> (order);
        }

        return latency;
        */
    }
    
    /**
      | Returns the current oversampling factor.
      |
      */
    pub fn get_oversampling_factor(&self) -> usize {
        
        todo!();
        /*
            return factorOversampling;
        */
    }
    
    /**
      | Must be called before any processing,
      | to set the buffer sizes of the internal
      | buffers of the oversampling processing.
      |
      */
    pub fn init_processing(&mut self, maximum_number_of_samples_before_oversampling: usize)  {
        
        todo!();
        /*
            jassert (! stages.isEmpty());
        auto currentNumSamples = maximumNumberOfSamplesBeforeOversampling;

        for (auto* stage : stages)
        {
            stage->initProcessing (currentNumSamples);
            currentNumSamples *= stage->factor;
        }

        ProcessSpec spec = { 0.0, (uint32) maximumNumberOfSamplesBeforeOversampling, (uint32) numChannels };
        delay.prepare (spec);
        updateDelayLine();

        isReady = true;
        reset();
        */
    }
    
    /**
      | Resets the processing pipeline, ready
      | to oversample a new stream of data.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            jassert (! stages.isEmpty());

        if (isReady)
            for (auto* stage : stages)
               stage->reset();

        delay.reset();
        */
    }
    
    /**
      | Must be called to perform the upsampling,
      | prior to any oversampled processing.
      | 
      | Returns an AudioBlock referencing
      | the oversampled input signal, which
      | must be used to perform the non-linear
      | processing which needs the higher sample
      | rate. Don't forget to set the sample
      | rate of that processing to N times the
      | original sample rate.
      |
      */
    pub fn process_samples_up(&mut self, input_block: &AudioBlock<SampleType>) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            jassert (! stages.isEmpty());

        if (! isReady)
            return {};

        auto* firstStage = stages.getUnchecked (0);
        firstStage->processSamplesUp (inputBlock);
        auto block = firstStage->getProcessedSamples (inputBlock.getNumSamples() * firstStage->factor);

        for (int i = 1; i < stages.size(); ++i)
        {
            stages[i]->processSamplesUp (block);
            block = stages[i]->getProcessedSamples (block.getNumSamples() * stages[i]->factor);
        }

        return block;
        */
    }
    
    /**
      | Must be called to perform the downsampling,
      | after the upsampling and the non-linear
      | processing. The output signal is probably
      | delayed by the internal latency of the
      | whole oversampling behaviour, so don't
      | forget to take this into account.
      |
      */
    pub fn process_samples_down(&mut self, output_block: &mut AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (! stages.isEmpty());

        if (! isReady)
            return;

        auto currentNumSamples = outputBlock.getNumSamples();

        for (int n = 0; n < stages.size() - 1; ++n)
            currentNumSamples *= stages.getUnchecked(n)->factor;

        for (int n = stages.size() - 1; n > 0; --n)
        {
            auto& stage = *stages.getUnchecked(n);
            auto audioBlock = stages.getUnchecked (n - 1)->getProcessedSamples (currentNumSamples);
            stage.processSamplesDown (audioBlock);

            currentNumSamples /= stage.factor;
        }

        stages.getFirst()->processSamplesDown (outputBlock);

        if (shouldUseIntegerLatency && fractionalDelay > static_cast<SampleType> (0.0))
        {
            auto context = ProcessContextReplacing<SampleType> (outputBlock);
            delay.process (context);
        }
        */
    }
    
    pub fn update_delay_line(&mut self)  {
        
        todo!();
        /*
            auto latency = getUncompensatedLatency();
        fractionalDelay = static_cast<SampleType> (1.0) - (latency - std::floor (latency));

        if (fractionalDelay == static_cast<SampleType> (1.0))
            fractionalDelay = static_cast<SampleType> (0.0);
        else if (fractionalDelay < static_cast<SampleType> (0.618))
            fractionalDelay += static_cast<SampleType> (1.0);

        delay.setDelay (fractionalDelay);
        */
    }
}
