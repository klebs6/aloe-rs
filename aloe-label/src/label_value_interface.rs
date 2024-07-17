crate::ix!();

#[no_copy]
#[leak_detector]
pub struct LabelValueInterface<'a> {
    label: &'a mut Label<'a>,
}

impl<'a> AccessibilityTextValueInterface for LabelValueInterface<'a> {

}

impl<'a> AccessibilityValueInterface for LabelValueInterface<'a> {

}

impl<'a> GetAccessibilityValueRange for LabelValueInterface<'a> {

}

impl<'a> SetValueAsString for LabelValueInterface<'a> {

   fn set_value_as_string(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> GetCurrentValueAsString for LabelValueInterface<'a> {

   fn get_current_value_as_string(&self) -> String {
        
        todo!();
        /*
            return label.getText();
        */
    }
}

impl<'a> IsReadOnly for LabelValueInterface<'a> {

    fn is_read_only(&self) -> bool {

        todo!();
        /*
            return true;
        */
    }
}

impl<'a> LabelValueInterface<'a> {

    pub fn new(label_to_wrap: &mut Label) -> Self {
    
        todo!();
        /*
        : label(labelToWrap),

        
        */
    }
}
