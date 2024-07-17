crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/OscillatorDemo.h]

pub struct OscillatorDemoDSP<'a> {
    oscillators:            [Oscillator<f32>; 6],
    current_oscillator_idx: i32,             // default = 0
    gain:                   Gain<f32>,
    type_param:             ChoiceParameter<'a>, //{ {"sine", "saw", "square"}, 1, "Type" };
    accuracy:               ChoiceParameter<'a>, //{ {"No Approximation", "Use Wavetable"}, 1, "Accuracy" };
    freq_param:             SliderParameter<'a>, //{ { 20.0, 24000.0 }, 0.4, 440.0, "Frequency", "Hz" };
    gain_param:             SliderParameter<'a>, //{ { -100.0, 20.0 }, 3.0, -20.0, "Gain", "dB" };
    mix_param:              SliderParameter<'a>, //{ { 0.0, 1.0 }, 1.0, 0.0, "File mix" };
    temp_buffer_memory:     HeapBlock<u8>,
    temp_buffer:            AudioBlock<f32>,
    file_mix:               f64,
    parameters:             Vec<*mut DSPDemoParameterBase<'a>>, //{ &typeParam, &accuracy, &freqParam, &gainParam, &mixParam };
}

impl<'a> OscillatorDemoDSP<'a> {

    pub fn default_oscillators() -> Oscillator<f32> {
        todo!();
        /*
        {
            // No Approximation
            {[] (float x) { return std::sin (x); }},                   // sine
            {[] (float x) { return x / MathConstants<float>::pi; }},   // saw
            {[] (float x) { return x < 0.0f ? -1.0f : 1.0f; }},        // square

            // Approximated by a wave-table
            {[] (float x) { return std::sin (x); }, 100},                 // sine
            {[] (float x) { return x / MathConstants<float>::pi; }, 100}, // saw
            {[] (float x) { return x < 0.0f ? -1.0f : 1.0f; }, 100}       // square
        };
        */
    }

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            gain.setGainDecibels (-6.0f);

            for (auto&& oscillator : oscillators)
            {
                oscillator.setFrequency (440.f);
                oscillator.prepare (spec);
            }

            updateParameters();

            tempBuffer = AudioBlock<float> (tempBufferMemory, spec.numChannels, spec.maximumBlockSize);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            tempBuffer.copyFrom (context.getInputBlock());
            tempBuffer.multiplyBy (static_cast<float> (fileMix));

            oscillators[currentOscillatorIdx].process (context);
            context.getOutputBlock().multiplyBy (static_cast<float> (1.0 - fileMix));

            context.getOutputBlock().add (tempBuffer);

            gain.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            oscillators[currentOscillatorIdx].reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            currentOscillatorIdx = jmin (numElementsInArray (oscillators),
                                         3 * (accuracy.getCurrentSelectedID() - 1) + (typeParam.getCurrentSelectedID() - 1));

            auto freq = static_cast<float> (freqParam.getCurrentValue());

            for (auto&& oscillator : oscillators)
                oscillator.setFrequency (freq);

            gain.setGainDecibels (static_cast<float> (gainParam.getCurrentValue()));

            fileMix = mixParam.getCurrentValue();
        */
    }
}

