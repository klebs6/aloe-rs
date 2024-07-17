crate::ix!();

/**
  | A parameter class that maintains backwards
  | compatibility with deprecated
  | 
  | AudioProcessorValueTreeState functionality.
  | 
  | Previous calls to
  | 
  | -----------
  | @code
  | 
  | createAndAddParameter (paramID1, paramName1, ...);
  | 
  | can be replaced with
  | ----------
  | @code
  | 
  | using AudioProcessorValueTreeStateParameter = AudioProcessorValueTreeState::AudioProcessorValueTreeStateParameter;
  | createAndAddParameter (std::make_unique<AudioProcessorValueTreeStateParameter> (paramID1, paramName1, ...));
  | 
  | However, a much better approach is to
  | use the AudioProcessorValueTreeState
  | constructor directly
  | ----------
  | @code
  | 
  | using AudioProcessorValueTreeStateParameter = AudioProcessorValueTreeState::AudioProcessorValueTreeStateParameter;
  | YourAudioProcessor()
  |     : apvts (*this, &undoManager, "PARAMETERS", { std::make_unique<AudioProcessorValueTreeStateParameter> (paramID1, paramName1, ...),
  |                                                   std::make_unique<AudioProcessorValueTreeStateParameter> (paramID2, paramName2, ...),
  |                                                   ... })
  |
  */
pub struct AudioProcessorValueTreeStateParameter {
    base:              AudioParameterFloat,
    on_value_changed:  fn() -> (),
    unsnapped_default: f32,
    meta_parameter:    bool,
    automatable:       bool,
    discrete:          bool,
    boolean:           bool,
    last_value:        Atomic<f32>, // default = { -1.0f  }
}

impl AudioProcessorValueTreeStateParameter {

    pub fn new(
        parameterid:              &String,
        parameter_name:           &String,
        label_text:               &String,
        value_range:              NormalisableRange<f32>,
        default_parameter_value:  f32,
        value_to_text_function:   fn(_0: f32) -> String,
        text_to_value_function:   fn(_0: &String) -> f32,
        is_meta_parameter:        Option<bool>,
        is_automatable_parameter: Option<bool>,
        is_discrete:              Option<bool>,
        parameter_category:       Option<AudioProcessorParameterCategory>,
        is_boolean:               Option<bool>

    ) -> Self {

        let is_meta_parameter        = is_meta_parameter.unwrap_or(false);
        let is_automatable_parameter = is_automatable_parameter.unwrap_or(true);
        let is_discrete              = is_discrete.unwrap_or(false);
        let parameter_category       = parameter_category.unwrap_or(AudioProcessorParameterCategory::genericParameter);
        let is_boolean               = is_boolean.unwrap_or(false);
    
        todo!();
        /*


            : AudioParameterFloat (parameterID,
                               parameterName,
                               valueRange,
                               defaultParameterValue,
                               labelText,
                               parameterCategory,
                               valueToTextFunction == nullptr ? std::function<String (float v, int)>()
                                                              : [valueToTextFunction] (float v, int) { return valueToTextFunction (v); },
                               std::move (textToValueFunction)),
          unsnappedDefault (valueRange.convertTo0to1 (defaultParameterValue)),
          metaParameter (isMetaParameter),
          automatable (isAutomatableParameter),
          discrete (isDiscrete),
          boolean (isBoolean)
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return unsnappedDefault;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return RangedAudioParameter::getNumSteps();
        */
    }
    
    pub fn is_meta_parameter(&self) -> bool {
        
        todo!();
        /*
            return metaParameter;
        */
    }
    
    pub fn is_automatable(&self) -> bool {
        
        todo!();
        /*
            return automatable;
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return discrete;
        */
    }
    
    pub fn is_boolean(&self) -> bool {
        
        todo!();
        /*
            return boolean;
        */
    }
    
    pub fn value_changed(&mut self, new_value: f32)  {
        
        todo!();
        /*
            if (lastValue == newValue)
            return;

        lastValue = newValue;

        if (onValueChanged != nullptr)
            onValueChanged();
        */
    }
}
