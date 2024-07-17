crate::ix!();

/**
  | Oversampling stage class performing
  | 2 times oversampling using the Filter
  | Design FIR Equiripple method. The resulting
  | filter is linear phase, symmetric,
  | and has every two samples but the middle
  | one equal to zero, leading to specific
  | processing optimizations.
  |
  */
#[no_copy]
#[leak_detector]
pub struct Oversampling2TimesEquirippleFIR<SampleType> {
    base:              OversamplingStage<SampleType>,
    coefficients_up:   FIRCoefficients<SampleType>,
    coefficients_down: FIRCoefficients<SampleType>,
    state_up:          AudioBuffer<SampleType>,
    state_down:        AudioBuffer<SampleType>,
    state_down2:       AudioBuffer<SampleType>,
    position:          Vec<usize>,
}

impl<SampleType: SampleTypeInterface> HasParentType for Oversampling2TimesEquirippleFIR<SampleType> {
    type Type = OversamplingStage<SampleType>;
}

impl<SampleType: SampleTypeInterface> Oversampling2TimesEquirippleFIR<SampleType> {

    pub fn new(
        num_chans:                        usize,
        normalised_transition_width_up:   SampleType,
        stopband_amplitudedb_up:          SampleType,
        normalised_transition_width_down: SampleType,
        stopband_amplitudedb_down:        SampleType) -> Self {
    
        todo!();
        /*
        : parent_type(numChans, 2),

            coefficientsUp   = *FilterDesign<SampleType>::designFIRLowpassHalfBandEquirippleMethod (normalisedTransitionWidthUp,   stopbandAmplitudedBUp);
            coefficientsDown = *FilterDesign<SampleType>::designFIRLowpassHalfBandEquirippleMethod (normalisedTransitionWidthDown, stopbandAmplitudedBDown);

            auto N = coefficientsUp.getFilterOrder() + 1;
            stateUp.setSize (static_cast<int> (this->numChannels), static_cast<int> (N));

            N = coefficientsDown.getFilterOrder() + 1;
            auto Ndiv2 = N / 2;
            auto Ndiv4 = Ndiv2 / 2;

            stateDown.setSize  (static_cast<int> (this->numChannels), static_cast<int> (N));
            stateDown2.setSize (static_cast<int> (this->numChannels), static_cast<int> (Ndiv4 + 1));

            position.resize (static_cast<int> (this->numChannels));
        */
    }
    
    pub fn get_latency_in_samples(&self) -> SampleType {
        
        todo!();
        /*
            return static_cast<SampleType> (coefficientsUp.getFilterOrder() + coefficientsDown.getFilterOrder()) * 0.5f;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            ParentType::reset();

            stateUp.clear();
            stateDown.clear();
            stateDown2.clear();

            position.fill (0);
        */
    }
    
    pub fn process_samples_up(&mut self, input_block: &AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (inputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (inputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            // Initialization
            auto fir = coefficientsUp.getRawCoefficients();
            auto N = coefficientsUp.getFilterOrder() + 1;
            auto Ndiv2 = N / 2;
            auto numSamples = inputBlock.getNumSamples();

            // Processing
            for (size_t channel = 0; channel < inputBlock.getNumChannels(); ++channel)
            {
                auto bufferSamples = ParentType::buffer.getWritePointer (static_cast<int> (channel));
                auto buf = stateUp.getWritePointer (static_cast<int> (channel));
                auto samples = inputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                {
                    // Input
                    buf[N - 1] = 2 * samples[i];

                    // Convolution
                    auto out = static_cast<SampleType> (0.0);

                    for (size_t k = 0; k < Ndiv2; k += 2)
                        out += (buf[k] + buf[N - k - 1]) * fir[k];

                    // Outputs
                    bufferSamples[i << 1] = out;
                    bufferSamples[(i << 1) + 1] = buf[Ndiv2 + 1] * fir[Ndiv2];

                    // Shift data
                    for (size_t k = 0; k < N - 2; k += 2)
                        buf[k] = buf[k + 2];
                }
            }
        */
    }
    
    pub fn process_samples_down(&mut self, output_block: &mut AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (outputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (outputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            // Initialization
            auto fir = coefficientsDown.getRawCoefficients();
            auto N = coefficientsDown.getFilterOrder() + 1;
            auto Ndiv2 = N / 2;
            auto Ndiv4 = Ndiv2 / 2;
            auto numSamples = outputBlock.getNumSamples();

            // Processing
            for (size_t channel = 0; channel < outputBlock.getNumChannels(); ++channel)
            {
                auto bufferSamples = ParentType::buffer.getWritePointer (static_cast<int> (channel));
                auto buf = stateDown.getWritePointer (static_cast<int> (channel));
                auto buf2 = stateDown2.getWritePointer (static_cast<int> (channel));
                auto samples = outputBlock.getChannelPointer (channel);
                auto pos = position.getUnchecked (static_cast<int> (channel));

                for (size_t i = 0; i < numSamples; ++i)
                {
                    // Input
                    buf[N - 1] = bufferSamples[i << 1];

                    // Convolution
                    auto out = static_cast<SampleType> (0.0);

                    for (size_t k = 0; k < Ndiv2; k += 2)
                        out += (buf[k] + buf[N - k - 1]) * fir[k];

                    // Output
                    out += buf2[pos] * fir[Ndiv2];
                    buf2[pos] = bufferSamples[(i << 1) + 1];

                    samples[i] = out;

                    // Shift data
                    for (size_t k = 0; k < N - 2; ++k)
                        buf[k] = buf[k + 2];

                    // Circular buffer
                    pos = (pos == 0 ? Ndiv4 : pos - 1);
                }

                position.setUnchecked (static_cast<int> (channel), pos);
            }
        */
    }
}
