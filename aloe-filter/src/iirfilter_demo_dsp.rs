crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/IIRFilterDemo.h]

pub struct IIRFilterDemoDSP<'a> {

    iir:          ProcessorDuplicator<IIRFilterTransposedDirectFormII<f32>,IIRCoefficients<f32>>,
    type_param:   ChoiceParameter<'a>, // { { "Low-pass", "High-pass", "Band-pass" }, 1, "Type" };
    cutoff_param: SliderParameter<'a>, // { { 20.0, 20000.0 }, 0.5, 440.0f, "Cutoff", "Hz" };
    q_param:      SliderParameter<'a>, // { { 0.3, 20.0 }, 0.5, 1.0 / std::sqrt(2.0), "Q" };
    parameters:   Vec<*mut DSPDemoParameterBase<'a>>, // { &typeParam, &cutoffParam, &qParam };
    sample_rate:  f64, // default = 0.0
}

impl<'a> IIRFilterDemoDSP<'a> {

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;

            iir.state = IIR::Coefficients<float>::makeLowPass (sampleRate, 440.0);
            iir.prepare (spec);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            iir.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            iir.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            if (sampleRate != 0.0)
            {
                auto cutoff = static_cast<float> (cutoffParam.getCurrentValue());
                auto qVal   = static_cast<float> (qParam.getCurrentValue());

                switch (typeParam.getCurrentSelectedID())
                {
                    case 1:     *iir.state = IIR::ArrayCoefficients<float>::makeLowPass  (sampleRate, cutoff, qVal); break;
                    case 2:     *iir.state = IIR::ArrayCoefficients<float>::makeHighPass (sampleRate, cutoff, qVal); break;
                    case 3:     *iir.state = IIR::ArrayCoefficients<float>::makeBandPass (sampleRate, cutoff, qVal); break;
                    default:    break;
                }
            }
        */
    }
}
