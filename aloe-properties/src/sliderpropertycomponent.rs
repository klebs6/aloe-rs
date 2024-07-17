crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_SliderPropertyComponent.h]

/**
  | A PropertyComponent that shows its
  | value as a slider.
  | 
  | @see PropertyComponent, Slider
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct SliderPropertyComponent<'a> {

    base: PropertyComponent<'a>,

    /**
      | The slider component being used in this
      | component.
      | 
      | Your subclass has access to this in case
      | it needs to customise it in some way.
      |
      */
    slider: Slider<'a>,
}

pub trait SliderPropertyComponentInterface: SetValue + GetValue { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_SliderPropertyComponent.cpp]
impl<'a> SliderPropertyComponent<'a> {
    
    /**
      | Creates the property component.
      | 
      | The ranges, interval and skew factor
      | are passed to the Slider component.
      | 
      | If you need to customise the slider in
      | other ways, your constructor can access
      | the slider member variable and change
      | it directly.
      |
      */
    pub fn new(
        name:           &String,
        range_min:      f64,
        range_max:      f64,
        interval:       f64,
        skew_factor:    Option<f64>,
        symmetric_skew: Option<bool>

    ) -> Self {

        let skew_factor: f64 = skew_factor.unwrap_or(1.0);
        let symmetric_skew: bool = symmetric_skew.unwrap_or(false);
    
        todo!();
        /*
        : property_component(name),

            addAndMakeVisible (slider);

        slider.setRange (rangeMin, rangeMax, interval);
        slider.setSkewFactor (skewFactor, symmetricSkew);
        slider.setSliderStyle (Slider::LinearBar);

        slider.onValueChange = [this]
        {
            if (getValue() != slider.getValue())
                setValue (slider.getValue());
        };
        */

    }
    
    /**
      | Creates the property component.
      | 
      | The ranges, interval and skew factor
      | are passed to the Slider component.
      | 
      | If you need to customise the slider in
      | other ways, your constructor can access
      | the slider member variable and change
      | it directly.
      | 
      | -----------
      | @note
      | 
      | if you call this constructor then you
      | must use the Value to interact with the
      | value, and you can't override the class
      | with your own setValue or getValue methods.
      | 
      | If you want to use those methods, call
      | the other constructor instead.
      |
      */
    pub fn new_with_value_to_control(
        value_to_control: &Value,
        name:             &String,
        range_min:        f64,
        range_max:        f64,
        interval:         f64,
        skew_factor:      Option<f64>,
        symmetric_skew:   Option<bool>

    ) -> Self {

        let skew_factor: f64 = skew_factor.unwrap_or(1.0);
        let symmetric_skew: bool = symmetric_skew.unwrap_or(false);
    
        todo!();
        /*
        : property_component(name),

            addAndMakeVisible (slider);

        slider.setRange (rangeMin, rangeMax, interval);
        slider.setSkewFactor (skewFactor, symmetricSkew);
        slider.setSliderStyle (Slider::LinearBar);

        slider.getValueObject().referTo (valueToControl);
        */

    }
    
    pub fn set_value(&mut self, new_value: f64)  {
        
        todo!();
        /*
        
        */

    }
    
    pub fn get_value(&self) -> f64 {
        
        todo!();
        /*
            return slider.getValue();
        */

    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            slider.setValue (getValue(), dontSendNotification);
        */

    }
}
