crate::ix!();

/**
  | Structure used to describe plugin parameters
  |
  */
pub struct AudioPluginInstanceParameter {
    base:        AudioProcessorParameter,
    on_strings:  Vec<String>,
    off_strings: Vec<String>,
}

impl Default for AudioPluginInstanceParameter {

    fn default() -> Self {
    
        todo!();
        /*


            onStrings.add (TRANS("on"));
            onStrings.add (TRANS("yes"));
            onStrings.add (TRANS("true"));

            offStrings.add (TRANS("off"));
            offStrings.add (TRANS("no"));
            offStrings.add (TRANS("false"));
        */
    }
}
    
impl AudioPluginInstanceParameter {

    pub fn get_text(&self, 
        value:                 f32,
        maximum_string_length: i32) -> String {
        
        todo!();
        /*
            if (isBoolean())
                return value < 0.5f ? TRANS("Off") : TRANS("On");

            return String (value).substring (0, maximumStringLength);
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            auto floatValue = text.retainCharacters ("-0123456789.").getFloatValue();

            if (isBoolean())
            {
                if (onStrings.contains (text, true))
                    return 1.0f;

                if (offStrings.contains (text, true))
                    return 0.0f;

                return floatValue < 0.5f ? 0.0f : 1.0f;
            }

            return floatValue;
        */
    }
}
