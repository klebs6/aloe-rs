crate::ix!();

pub struct Vst3Parameter<'a> {
    base:            Parameter,
    plugin_instance: &'a mut Vst3PluginInstance<'a>,
    vst_param_index: i32,
    paramid:         ParamID,
    automatable:     bool,

    /**
      = getNumSteps() != AudioProcessor::getDefaultNumParameterSteps();
      */
    discrete:        bool,
    num_steps:       i32, //default_num_steps()
}

impl<'a> Vst3Parameter<'a> {

    pub fn default_num_steps() -> i32 {

        todo!();

        /*
        const int numSteps = [&]
        {
            auto stepCount = getParameterInfo().stepCount;
            return stepCount == 0 ? AudioProcessor::getDefaultNumParameterSteps()
                                  : stepCount + 1;
        }();
        */
    }

    pub fn new(
        parent:                   &mut Vst3PluginInstance,
        vst_parameter_index:      i32,
        parameterid:              ParamID,
        parameter_is_automatable: bool) -> Self {
    
        todo!();
        /*
        : plugin_instance(parent),
        : vst_param_index(vstParameterIndex),
        : paramid(parameterID),
        : automatable(parameterIsAutomatable),

        
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            return pluginInstance.cachedParamValues.get (vstParamIndex);
        */
    }

    /**
      | The 'normal' setValue call, which will
      | update both the processor and editor.
      |
      */
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            pluginInstance.cachedParamValues.set (vstParamIndex, newValue);
                pluginInstance.parameterDispatcher.push (vstParamIndex, newValue);
        */
    }

    /**
      | If the editor set the value, there's
      | no need to notify it that the parameter
      | value changed. Instead, we set the cachedValue
      | (which will be read by the processor
      | during the next processBlock) and notify
      | listeners that the parameter has changed.
      |
      */
    pub fn set_value_from_editor(&mut self, new_value: f32)  {
        
        todo!();
        /*
            pluginInstance.cachedParamValues.set (vstParamIndex, newValue);
                sendValueChangedMessageToListeners (newValue);
        */
    }

    /**
      | If we're syncing the editor to the processor,
      | the processor won't need to be notified
      | about the parameter updates, so we can
      | avoid flagging the change when updating
      | the float cache.
      |
      */
    pub fn set_value_without_updating_processor(&mut self, new_value: f32)  {
        
        todo!();
        /*
            pluginInstance.cachedParamValues.setWithoutNotifying (vstParamIndex, newValue);
                sendValueChangedMessageToListeners (newValue);
        */
    }
    
    pub fn get_text(&self, 
        value:          f32,
        maximum_length: i32) -> String {
        
        todo!();
        /*
            MessageManagerLock lock;

                if (pluginInstance.editController != nullptr)
                {
                    VstString128 result;

                    if (pluginInstance.editController->getParamStringByValue (paramID, value, result) == kResultOk)
                        return toString (result).substring (0, maximumLength);
                }

                return Parameter::getText (value, maximumLength);
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            MessageManagerLock lock;

                if (pluginInstance.editController != nullptr)
                {
                    VstParamValue result;

                    if (pluginInstance.editController->getParamValueByString (paramID, toString (text), result) == kResultOk)
                        return (float) result;
                }

                return Parameter::getValueForText (text);
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return (float) getParameterInfo().defaultNormalizedValue;
        */
    }
    
    pub fn get_name(&self, maximum_string_length: i32) -> String {
        
        todo!();
        /*
            return toString (getParameterInfo().title);
        */
    }
    
    pub fn get_label(&self) -> String {
        
        todo!();
        /*
            return toString (getParameterInfo().units);
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
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return numSteps;
        */
    }
    
    pub fn get_all_value_strings(&self) -> StringArray {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_paramid(&self) -> ParamID {
        
        todo!();
        /*
            return paramID;
        */
    }
    
    pub fn get_parameter_info(&self) -> ParameterInfo {
        
        todo!();
        /*
            return pluginInstance.getParameterInfoForIndex (vstParamIndex);
        */
    }
}
