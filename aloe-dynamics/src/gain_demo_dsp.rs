crate::ix!();

pub struct GainDemoDSP<'a> {
    gain:       Gain<f32>,
    gain_param: SliderParameter<'a>, // { { -100.0, 20.0 }, 3.0, -6.0, "Gain", "dB" };
    parameters: Vec<*mut DSPDemoParameterBase<'a>>, //{ &gainParam };
}

impl<'a> GainDemoDSP<'a> {

    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
            gain.setGainDecibels (-6.0f);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            gain.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            gain.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            gain.setGainDecibels (static_cast<float> (gainParam.getCurrentValue()));
        */
    }
}
