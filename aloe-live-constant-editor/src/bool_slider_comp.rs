crate::ix!();

pub fn create_bool_slider<'a>(editor: &mut LivePropertyEditorBase) -> *mut Component<'a> {
    
    todo!();
        /*
            return new BoolSliderComp (editor);
        */
}

pub fn create_integer_slider<'a>(editor: &mut LivePropertyEditorBase) -> *mut Component<'a> {
    
    todo!();
        /*
            return new SliderComp (editor, false);
        */
}

pub fn create_float_slider<'a>(editor: &mut LivePropertyEditorBase) -> *mut Component<'a> {
    
    todo!();
        /*
            return new SliderComp (editor, true);
        */
}

pub struct BoolSliderComp<'a> {
    base: SliderComp<'a>,
}

impl<'a> BoolSliderComp<'a> {

    pub fn new(e: &mut LivePropertyEditorBase) -> Self {
    
        todo!();
        /*
        : slider_comp(e, false),

            slider.onValueChange = [this] { editor.applyNewValue (slider.getValue() > 0.5 ? "true" : "false"); };
        */
    }
    
    pub fn update_range(&mut self)  {
        
        todo!();
        /*
            slider.setRange (0.0, 1.0, dontSendNotification);
            slider.setValue (editor.value.getStringValue (false) == "true", dontSendNotification);
        */
    }
}
