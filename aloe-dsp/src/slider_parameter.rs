crate::ix!();

pub struct SliderParameter<'a> {
    base:   DSPDemoParameterBase<'a>,
    slider: Slider<'a>,
}

impl<'a> SliderParameter<'a> {

    pub fn new(
        range:         Range<f64>,
        skew:          f64,
        initial_value: f64,
        label_name:    &String,
        suffix:        &String) -> Self {
    
        todo!();
        /*
        : dsp_demo_parameter_base(labelName),

            slider.setRange (range.getStart(), range.getEnd(), 0.01);
            slider.setSkewFactor (skew);
            slider.setValue (initialValue);

            if (suffix.isNotEmpty())
                slider.setTextValueSuffix (suffix);

            slider.onValueChange = [this] { sendChangeMessage(); };
        */
    }
    
    pub fn get_component(&mut self) -> *mut Component {
        
        todo!();
        /*
            return &slider;
        */
    }
    
    pub fn get_preferred_height(&mut self) -> i32 {
        
        todo!();
        /*
            return 40;
        */
    }
    
    pub fn get_preferred_width(&mut self) -> i32 {
        
        todo!();
        /*
            return 500;
        */
    }
    
    pub fn get_current_value(&self) -> f64 {
        
        todo!();
        /*
            return slider.getValue();
        */
    }
}
