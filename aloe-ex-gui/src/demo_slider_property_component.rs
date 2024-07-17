crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DemoSliderPropertyComponent<'a> {
    base: SliderPropertyComponent<'a>,
}

impl<'a> DemoSliderPropertyComponent<'a> {

    pub fn new(property_name: &String) -> Self {
    
        todo!();
        /*


            : SliderPropertyComponent (propertyName, 0.0, 100.0, 0.001)
            setValue (Random::getSystemRandom().nextDouble() * 42.0);
        */
    }
    
    pub fn set_value(&mut self, new_value: f64)  {
        
        todo!();
        /*
            slider.setValue (newValue);
        */
    }
}


