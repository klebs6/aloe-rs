crate::ix!();

///------------------------
pub struct ColourComponentSlider<'a> {
    base: Slider<'a>,
}

impl<'a> ColourComponentSlider<'a> {

    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : slider(name),

            setRange (0.0, 255.0, 1.0);
        */
    }
    
    pub fn get_text_from_value(&mut self, value: f64) -> String {
        
        todo!();
        /*
            return String::toHexString ((int) value).toUpperCase().paddedLeft ('0', 2);
        */
    }
    
    pub fn get_value_from_text(&mut self, text: &String) -> f64 {
        
        todo!();
        /*
            return (double) text.getHexValue32();
        */
    }
}

