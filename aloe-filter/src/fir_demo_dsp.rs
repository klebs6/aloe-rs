crate::ix!();

pub struct FIRFilterDemoDSP<'a> {
    fir:          ProcessorDuplicator<FIRFilter<f32>,FIRCoefficients<f32>>,
    sample_rate:  f64,                            // default = 0.0
    cutoff_param: SliderParameter<'a>,                //{ { 20.0, 20000.0 }, 0.4, 440.0f, "Cutoff", "Hz" };
    type_param:   ChoiceParameter<'a>,                //{ { "Rectangular", "Triangular", "Hann", "Hamming", "Blackman", "Blackman-Harris", "Flat Top", "Kaiser" }, 5, "Windowing Function" };
    parameters:   Vec<*mut DSPDemoParameterBase<'a>>, //{ &cutoffParam, &typeParam };
}

impl<'a> FIRFilterDemoDSP<'a> {
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;

            fir.state = FilterDesign<float>::designFIRLowpassWindowMethod (440.0f, sampleRate, 21,
                                                                           WindowingFunction<float>::blackman);
            fir.prepare (spec);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            fir.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            fir.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            if (sampleRate != 0.0)
            {
                auto cutoff = static_cast<float> (cutoffParam.getCurrentValue());
                auto windowingMethod = static_cast<WindowingFunction<float>::WindowingMethod> (typeParam.getCurrentSelectedID() - 1);

                *fir.state = *FilterDesign<float>::designFIRLowpassWindowMethod (cutoff, sampleRate, 21, windowingMethod);
            }
        */
    }
}

