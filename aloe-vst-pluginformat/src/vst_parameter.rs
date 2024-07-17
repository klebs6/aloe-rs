crate::ix!();

pub struct VSTParameter<'a> {
    base:              Parameter,
    plugin_instance:   &'a mut VSTPluginInstance<'a>,
    name:              String,
    short_names:       Vec<String>,
    default_value:     f32,
    label:             String,
    automatable:       bool,
    discrete:          bool,
    num_steps:         i32,
    is_switch:         bool,
    vst_value_strings: StringArray,
    value_type:        *const VSTXMLInfoValueType,
}

impl<'a> VSTParameter<'a> {

    pub fn new(
        parent:               &mut VSTPluginInstance,
        param_name:           &String,
        short_param_names:    &[String],
        param_default_value:  f32,
        param_label:          &String,
        param_is_automatable: bool,
        param_is_discrete:    bool,
        num_param_steps:      i32,
        is_bool_switch:       bool,
        param_value_strings:  &StringArray,
        param_value_type:     *const VSTXMLInfoValueType) -> Self {
    
        todo!();
        /*


            : pluginInstance (parent),
                  name (paramName),
                  shortNames (shortParamNames),
                  defaultValue (paramDefaultValue),
                  label (paramLabel),
                  automatable (paramIsAutomatable),
                  discrete (paramIsDiscrete),
                  numSteps (numParamSteps),
                  isSwitch (isBoolSwitch),
                  vstValueStrings (paramValueStrings),
                  valueType (paramValueType)
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            if (auto* effect = pluginInstance.vstEffect)
                {
                    const ScopedLock sl (pluginInstance.lock);

                    return effect->getParameter (effect, getParameterIndex());
                }

                return 0.0f;
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            if (auto* effect = pluginInstance.vstEffect)
                {
                    const ScopedLock sl (pluginInstance.lock);

                    if (effect->getParameter (effect, getParameterIndex()) != newValue)
                        effect->setParameter (effect, getParameterIndex(), newValue);
                }
        */
    }
    
    pub fn get_text(&self, 
        value:                 f32,
        maximum_string_length: i32) -> String {
        
        todo!();
        /*
            if (valueType != nullptr)
                {
                    for (auto& v : valueType->entries)
                        if (v->range.contains (value))
                            return v->name;
                }

                return Parameter::getText (value, maximumStringLength);
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            if (valueType != nullptr)
                {
                    for (auto& v : valueType->entries)
                        if (v->name == text)
                            return (v->range.high + v->range.low) / 2.0f;
                }

                return Parameter::getValueForText (text);
        */
    }
    
    pub fn get_current_value_as_text(&self) -> String {
        
        todo!();
        /*
            if (valueType != nullptr || ! vstValueStrings.isEmpty())
                    return getText (getValue(), 1024);

                return pluginInstance.getTextForOpcode (getParameterIndex(), typename Vst2EffGetParamDisplay);
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return defaultValue;
        */
    }
    
    pub fn get_name(&self, maximum_string_length: i32) -> String {
        
        todo!();
        /*
            if (name.isEmpty())
                    return pluginInstance.getTextForOpcode (getParameterIndex(),
                                                            typename Vst2EffGetParamName);

                if (name.length() <= maximumStringLength)
                    return name;

                if (! shortNames.isEmpty())
                {
                    for (auto& n : shortNames)
                        if (n.length() <= maximumStringLength)
                            return n;

                    return shortNames.getLast();
                }

                return name;
        */
    }
    
    pub fn get_label(&self) -> String {
        
        todo!();
        /*
            return label.isEmpty() ? pluginInstance.getTextForOpcode (getParameterIndex(),
                                                                          typename Vst2EffGetParamLabel)
                                       : label;
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
            return isSwitch;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return numSteps;
        */
    }
    
    pub fn get_all_value_strings(&self) -> StringArray {
        
        todo!();
        /*
            return vstValueStrings;
        */
    }
}
