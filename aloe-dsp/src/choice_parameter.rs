crate::ix!();

pub struct ChoiceParameter<'a> {
    base:          DSPDemoParameterBase<'a>,
    parameter_box: ComboBox<'a>,
}

impl<'a> ChoiceParameter<'a> {

    pub fn new(
        options:    &Vec<String>,
        initial_id: i32,
        label_name: &String) -> Self {
    
        todo!();
        /*
        : dsp_demo_parameter_base(labelName),

            parameterBox.addItemList (options, 1);
            parameterBox.onChange = [this] { sendChangeMessage(); };

            parameterBox.setSelectedId (initialId);
        */
    }
    
    pub fn get_component(&mut self) -> *mut Component {
        
        todo!();
        /*
            return &parameterBox;
        */
    }
    
    pub fn get_preferred_height(&mut self) -> i32 {
        
        todo!();
        /*
            return 25;
        */
    }
    
    pub fn get_preferred_width(&mut self) -> i32 {
        
        todo!();
        /*
            return 250;
        */
    }
    
    pub fn get_current_selectedid(&self) -> i32 {
        
        todo!();
        /*
            return parameterBox.getSelectedId();
        */
    }
}
