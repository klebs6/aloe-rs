crate::ix!();

/**
  | Oversampling stage class performing
  | 2 times oversampling using the Filter
  | Design IIR Polyphase Allpass Cascaded
  | method. The resulting filter is minimum
  | phase, and provided with a method to
  | get the exact resulting latency.
  |
  */
#[no_copy]
#[leak_detector]
pub struct Oversampling2TimesPolyphaseIIR<SampleType> {
    base:              OversamplingStage<SampleType>,
    coefficients_up:   Vec<SampleType>,
    coefficients_down: Vec<SampleType>,
    latency:           SampleType,
    v1up:              AudioBuffer<SampleType>,
    v1down:            AudioBuffer<SampleType>,
    delay_down:        Vec<SampleType>,
}

impl<SampleType: SampleTypeInterface> HasParentType for Oversampling2TimesPolyphaseIIR<SampleType> {
    type Type = OversamplingStage<SampleType>;
}

impl<SampleType: SampleTypeInterface> Oversampling2TimesPolyphaseIIR<SampleType> {
    
    pub fn new(
        num_chans:                        usize,
        normalised_transition_width_up:   SampleType,
        stopband_amplitudedb_up:          SampleType,
        normalised_transition_width_down: SampleType,
        stopband_amplitudedb_down:        SampleType) -> Self {
    
        todo!();
        /*
        : parent_type(numChans, 2),

            auto structureUp = FilterDesign<SampleType>::designIIRLowpassHalfBandPolyphaseAllpassMethod (normalisedTransitionWidthUp, stopbandAmplitudedBUp);
            auto coeffsUp = getCoefficients (structureUp);
            latency = static_cast<SampleType> (-(coeffsUp.getPhaseForFrequency (0.0001, 1.0)) / (0.0001 * MathConstants<double>::twoPi));

            auto structureDown = FilterDesign<SampleType>::designIIRLowpassHalfBandPolyphaseAllpassMethod (normalisedTransitionWidthDown, stopbandAmplitudedBDown);
            auto coeffsDown = getCoefficients (structureDown);
            latency += static_cast<SampleType> (-(coeffsDown.getPhaseForFrequency (0.0001, 1.0)) / (0.0001 * MathConstants<double>::twoPi));

            for (auto i = 0; i < structureUp.directPath.size(); ++i)
                coefficientsUp.add (structureUp.directPath.getObjectPointer (i)->coefficients[0]);

            for (auto i = 1; i < structureUp.delayedPath.size(); ++i)
                coefficientsUp.add (structureUp.delayedPath.getObjectPointer (i)->coefficients[0]);

            for (auto i = 0; i < structureDown.directPath.size(); ++i)
                coefficientsDown.add (structureDown.directPath.getObjectPointer (i)->coefficients[0]);

            for (auto i = 1; i < structureDown.delayedPath.size(); ++i)
                coefficientsDown.add (structureDown.delayedPath.getObjectPointer (i)->coefficients[0]);

            v1Up.setSize   (static_cast<int> (this->numChannels), coefficientsUp.size());
            v1Down.setSize (static_cast<int> (this->numChannels), coefficientsDown.size());
            delayDown.resize (static_cast<int> (this->numChannels));
        */
    }
    
    pub fn get_latency_in_samples(&self) -> SampleType {
        
        todo!();
        /*
            return latency;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            ParentType::reset();
            v1Up.clear();
            v1Down.clear();
            delayDown.fill (0);
        */
    }
    
    pub fn process_samples_up(&mut self, input_block: &AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (inputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (inputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            // Initialization
            auto coeffs = coefficientsUp.getRawDataPointer();
            auto numStages = coefficientsUp.size();
            auto delayedStages = numStages / 2;
            auto directStages = numStages - delayedStages;
            auto numSamples = inputBlock.getNumSamples();

            // Processing
            for (size_t channel = 0; channel < inputBlock.getNumChannels(); ++channel)
            {
                auto bufferSamples = ParentType::buffer.getWritePointer (static_cast<int> (channel));
                auto lv1 = v1Up.getWritePointer (static_cast<int> (channel));
                auto samples = inputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                {
                    // Direct path cascaded allpass filters
                    auto input = samples[i];

                    for (auto n = 0; n < directStages; ++n)
                    {
                        auto alpha = coeffs[n];
                        auto output = alpha * input + lv1[n];
                        lv1[n] = input - alpha * output;
                        input = output;
                    }

                    // Output
                    bufferSamples[i << 1] = input;

                    // Delayed path cascaded allpass filters
                    input = samples[i];

                    for (auto n = directStages; n < numStages; ++n)
                    {
                        auto alpha = coeffs[n];
                        auto output = alpha * input + lv1[n];
                        lv1[n] = input - alpha * output;
                        input = output;
                    }

                    // Output
                    bufferSamples[(i << 1) + 1] = input;
                }
            }

           #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
            snapToZero (true);
           #endif
        */
    }
    
    pub fn process_samples_down(&mut self, output_block: &mut AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (outputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (outputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            // Initialization
            auto coeffs = coefficientsDown.getRawDataPointer();
            auto numStages = coefficientsDown.size();
            auto delayedStages = numStages / 2;
            auto directStages = numStages - delayedStages;
            auto numSamples = outputBlock.getNumSamples();

            // Processing
            for (size_t channel = 0; channel < outputBlock.getNumChannels(); ++channel)
            {
                auto bufferSamples = ParentType::buffer.getWritePointer (static_cast<int> (channel));
                auto lv1 = v1Down.getWritePointer (static_cast<int> (channel));
                auto samples = outputBlock.getChannelPointer (channel);
                auto delay = delayDown.getUnchecked (static_cast<int> (channel));

                for (size_t i = 0; i < numSamples; ++i)
                {
                    // Direct path cascaded allpass filters
                    auto input = bufferSamples[i << 1];

                    for (auto n = 0; n < directStages; ++n)
                    {
                        auto alpha = coeffs[n];
                        auto output = alpha * input + lv1[n];
                        lv1[n] = input - alpha * output;
                        input = output;
                    }

                    auto directOut = input;

                    // Delayed path cascaded allpass filters
                    input = bufferSamples[(i << 1) + 1];

                    for (auto n = directStages; n < numStages; ++n)
                    {
                        auto alpha = coeffs[n];
                        auto output = alpha * input + lv1[n];
                        lv1[n] = input - alpha * output;
                        input = output;
                    }

                    // Output
                    samples[i] = (delay + directOut) * static_cast<SampleType> (0.5);
                    delay = input;
                }

                delayDown.setUnchecked (static_cast<int> (channel), delay);
            }

           #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
            snapToZero (false);
           #endif
        */
    }
    
    pub fn snap_to_zero(&mut self, snap_up_processing: bool)  {
        
        todo!();
        /*
            if (snapUpProcessing)
            {
                for (auto channel = 0; channel < ParentType::buffer.getNumChannels(); ++channel)
                {
                    auto lv1 = v1Up.getWritePointer (channel);
                    auto numStages = coefficientsUp.size();

                    for (auto n = 0; n < numStages; ++n)
                        util::snapToZero (lv1[n]);
                }
            }
            else
            {
                for (auto channel = 0; channel < ParentType::buffer.getNumChannels(); ++channel)
                {
                    auto lv1 = v1Down.getWritePointer (channel);
                    auto numStages = coefficientsDown.size();

                    for (auto n = 0; n < numStages; ++n)
                        util::snapToZero (lv1[n]);
                }
            }
        */
    }
    
    /**
      | This function calculates the equivalent
      | high order IIR filter of a given polyphase
      | cascaded allpass filters structure.
      |
      */
    pub fn get_coefficients(&self, structure: &mut IIRPolyphaseAllpassStructure<SampleType>) -> IIRCoefficients<SampleType> {
        
        todo!();
        /*
            constexpr auto one = static_cast<SampleType> (1.0);

            Polynomial<SampleType> numerator1 ({ one }), denominator1 ({ one }),
                                   numerator2 ({ one }), denominator2 ({ one });

            for (auto* i : structure.directPath)
            {
                auto coeffs = i->getRawCoefficients();

                if (i->getFilterOrder() == 1)
                {
                    numerator1   = numerator1  .getProductWith (Polynomial<SampleType> ({ coeffs[0], coeffs[1] }));
                    denominator1 = denominator1.getProductWith (Polynomial<SampleType> ({ one,       coeffs[2] }));
                }
                else
                {
                    numerator1   = numerator1  .getProductWith (Polynomial<SampleType> ({ coeffs[0], coeffs[1], coeffs[2] }));
                    denominator1 = denominator1.getProductWith (Polynomial<SampleType> ({ one,       coeffs[3], coeffs[4] }));
                }
            }

            for (auto* i : structure.delayedPath)
            {
                auto coeffs = i->getRawCoefficients();

                if (i->getFilterOrder() == 1)
                {
                    numerator2   = numerator2  .getProductWith (Polynomial<SampleType> ({ coeffs[0], coeffs[1] }));
                    denominator2 = denominator2.getProductWith (Polynomial<SampleType> ({ one,       coeffs[2] }));
                }
                else
                {
                    numerator2   = numerator2  .getProductWith (Polynomial<SampleType> ({ coeffs[0], coeffs[1], coeffs[2] }));
                    denominator2 = denominator2.getProductWith (Polynomial<SampleType> ({ one,       coeffs[3], coeffs[4] }));
                }
            }

            auto numeratorf1 = numerator1.getProductWith (denominator2);
            auto numeratorf2 = numerator2.getProductWith (denominator1);
            auto numerator   = numeratorf1.getSumWith (numeratorf2);
            auto denominator = denominator1.getProductWith (denominator2);

            IIR::Coefficients<SampleType> coeffs;

            coeffs.coefficients.clear();
            auto inversion = one / denominator[0];

            for (int i = 0; i <= numerator.getOrder(); ++i)
                coeffs.coefficients.add (numerator[i] * inversion);

            for (int i = 1; i <= denominator.getOrder(); ++i)
                coeffs.coefficients.add (denominator[i] * inversion);

            return coeffs;
        */
    }
}
