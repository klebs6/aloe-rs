crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SliderAccessibilityHandler<'a> {
    base:   AccessibilityHandler<'a>,
    slider: &'a mut Slider<'a>,
}

impl<'a> SliderAccessibilityHandler<'a> {

    pub fn new(slider_to_wrap: &mut Slider) -> Self {
    
        todo!();
        /*
            : AccessibilityHandler (sliderToWrap,
                                    AccessibilityRole::slider,
                                    AccessibilityActions{},
                                    AccessibilityHandler::Interfaces { std::make_unique<SliderAccessibilityHandlerValueInterface> (sliderToWrap) }),
              slider (sliderToWrap)
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return slider.getTooltip();
        */
    }
}

//-------------------------------------
#[no_copy]
#[leak_detector]
pub struct SliderAccessibilityHandlerValueInterface<'a> {
    slider:        &'a mut Slider<'a>,
    use_max_value: bool,
}

impl<'a> AccessibilityValueInterface for SliderAccessibilityHandlerValueInterface<'a> {

}

impl<'a> GetAccessibilityValueRange for SliderAccessibilityHandlerValueInterface<'a> {

    fn get_range(&self) -> AccessibleValueRange {
        
        todo!();
        /*
            return { { slider.getMinimum(), slider.getMaximum() },
                         getStepSize() };
        */
    }
}

impl<'a> SetValueAsString for SliderAccessibilityHandlerValueInterface<'a> {

    fn set_value_as_string(&mut self, new_value: &String)  {
        
        todo!();
        /*
            setValue (slider.getValueFromText (newValue));
        */
    }
}

impl<'a> SetValue for SliderAccessibilityHandlerValueInterface<'a> {

    fn set_value(&mut self, new_value: f64)  {
        
        todo!();
        /*
            Slider::SliderScopedDragNotification drag (slider);

                if (useMaxValue)
                    slider.setMaxValue (newValue, sendNotificationSync);
                else
                    slider.setValue (newValue, sendNotificationSync);
        */
    }
}

impl<'a> GetCurrentValueAsString for SliderAccessibilityHandlerValueInterface<'a> {

    fn get_current_value_as_string(&self) -> String {
        
        todo!();
        /*
            return slider.getTextFromValue (getCurrentValue());
        */
    }
}

impl<'a> GetCurrentValue for SliderAccessibilityHandlerValueInterface<'a> {

    fn get_current_value(&self) -> f64 {
        
        todo!();
        /*
            return useMaxValue ? slider.getMaximum()
                                   : slider.getValue();
        */
    }
}

impl<'a> IsReadOnly for SliderAccessibilityHandlerValueInterface<'a> {

    fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl<'a> SliderAccessibilityHandlerValueInterface<'a> {

    pub fn new(slider_to_wrap: &mut Slider) -> Self {
    
        todo!();
        /*
        : slider(sliderToWrap),
        : use_max_value(slider.isTwoValue()),

        
        */
    }
    
    pub fn get_step_size(&self) -> f64 {
        
        todo!();
        /*
            auto interval = slider.getInterval();

                return interval != 0.0 ? interval
                                       : slider.getRange().getLength() * 0.01;
        */
    }
}
