crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/StateVariableFilterDemo.h]

pub struct StateVariableFilterDemoDSP<'a> {
    filter:       StateVariableTPTFilter<f32>,
    type_param:   ChoiceParameter<'a>,                  // {{ "Low-pass", "Band-pass", "High-pass" }, 1, "Type" };
    cutoff_param: SliderParameter<'a>,                  // {{ 20.0, 20000.0 }, 0.5, 440.0f, "Cutoff", "Hz" };
    q_param:      SliderParameter<'a>,                  // {{ 0.3, 20.0 }, 0.5, 1.0 / MathConstants<double>::sqrt2, "Resonance" };
    parameters:   Vec<*mut DSPDemoParameterBase<'a>>,   // { &typeParam, &cutoffParam, &qParam };
    sample_rate:  f64, // default = 0.0
}

impl<'a> StateVariableFilterDemoDSP<'a> {

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;
            filter.prepare (spec);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            filter.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            filter.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            if (sampleRate != 0.0)
            {
                filter.setCutoffFrequency (static_cast<float> (cutoffParam.getCurrentValue()));
                filter.setResonance       (static_cast<float> (qParam.getCurrentValue()));

                switch (typeParam.getCurrentSelectedID() - 1)
                {
                    case 0:   filter.setType (StateVariableTPTFilterType::lowpass);  break;
                    case 1:   filter.setType (StateVariableTPTFilterType::bandpass); break;
                    case 2:   filter.setType (StateVariableTPTFilterType::highpass); break;
                    default:  jassertfalse;                                                   break;
                };
            }
        */
    }
}
