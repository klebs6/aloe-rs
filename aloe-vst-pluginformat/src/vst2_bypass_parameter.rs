crate::ix!();

pub struct VST2BypassParameter<'a> {
    base:            Parameter,
    parent:          &'a mut VSTPluginInstance<'a>,
    current_value:   bool, // default = false
    vst_on_strings:  StringArray,
    vst_off_strings: StringArray,
    values:          StringArray,
}

impl<'a> VST2BypassParameter<'a> {

    pub fn new(effect_to_use: &mut VSTPluginInstance) -> Self {
    
        todo!();
        /*


            : parent (effectToUse),
                  vstOnStrings (TRANS("on"), TRANS("yes"), TRANS("true")),
                  vstOffStrings (TRANS("off"), TRANS("no"), TRANS("false")),
                  values (TRANS("Off"), TRANS("On"))
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            currentValue = (newValue != 0.0f);

                if (parent.vstSupportsBypass)
                    parent.dispatch (typename Vst2EffSetBypass, 0, currentValue ? 1 : 0, nullptr, 0.0f);
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            String lowercaseText (text.toLowerCase());

                for (auto& testText : vstOnStrings)
                    if (lowercaseText == testText)
                        return 1.0f;

                for (auto& testText : vstOffStrings)
                    if (lowercaseText == testText)
                        return 0.0f;

                return text.getIntValue() != 0 ? 1.0f : 0.0f;
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            return currentValue;
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return 0.0f;
        */
    }
    
    pub fn get_name(&self, maximum_string_length: i32) -> String {
        
        todo!();
        /*
            return "Bypass";
        */
    }
    
    pub fn get_text(&self, 
        value: f32,
        _1:    i32) -> String {
        
        todo!();
        /*
            return (value != 0.0f ? TRANS("On") : TRANS("Off"));
        */
    }
    
    pub fn is_automatable(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_boolean(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
    
    pub fn get_all_value_strings(&self) -> StringArray {
        
        todo!();
        /*
            return values;
        */
    }
    
    pub fn get_label(&self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
}
