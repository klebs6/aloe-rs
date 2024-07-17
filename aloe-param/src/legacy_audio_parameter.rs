crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_LegacyAudioParameter.cpp]

pub struct LegacyAudioParameter {
    base: AudioProcessorParameter,
}

impl LegacyAudioParameter {

    pub fn new(
        audio_processor_to_use: &mut dyn AudioProcessorInterface,
        audio_parameter_index:  i32) -> Self {
    
        todo!();
        /*


            processor = &audioProcessorToUse;

            parameterIndex = audioParameterIndex;
            jassert (parameterIndex < processor->getNumParameters());
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            return processor->getParameter (parameterIndex);
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            processor->setParameter (parameterIndex, newValue);
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return processor->getParameterDefaultValue (parameterIndex);
        */
    }
    
    pub fn get_name(&self, max_len: i32) -> String {
        
        todo!();
        /*
            return processor->getParameterName (parameterIndex, maxLen);
        */
    }
    
    pub fn get_label(&self) -> String {
        
        todo!();
        /*
            return processor->getParameterLabel (parameterIndex);
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return processor->getParameterNumSteps (parameterIndex);
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return processor->isParameterDiscrete (parameterIndex);
        */
    }
    
    pub fn is_boolean(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_orientation_inverted(&self) -> bool {
        
        todo!();
        /*
            return processor->isParameterOrientationInverted (parameterIndex);
        */
    }
    
    pub fn is_automatable(&self) -> bool {
        
        todo!();
        /*
            return processor->isParameterAutomatable (parameterIndex);
        */
    }
    
    pub fn is_meta_parameter(&self) -> bool {
        
        todo!();
        /*
            return processor->isMetaParameter (parameterIndex);
        */
    }
    
    pub fn get_category(&self) -> AudioProcessorParameterCategory {
        
        todo!();
        /*
            return processor->getParameterCategory (parameterIndex);
        */
    }
    
    pub fn get_current_value_as_text(&self) -> String {
        
        todo!();
        /*
            return processor->getParameterText (parameterIndex);
        */
    }
    
    pub fn get_paramid(&self) -> String {
        
        todo!();
        /*
            return processor->getParameterID (parameterIndex);
        */
    }
    
    pub fn get_value_for_text(&self, _0: &String) -> f32 {
        
        todo!();
        /*
            // legacy parameters do not support this method
            jassertfalse;
            return 0.0f;
        */
    }
    
    pub fn get_text(&self, _0: f32, _1: i32) -> String {
        
        todo!();
        /*
            // legacy parameters do not support this method
            jassertfalse;
            return {};
        */
    }
    
    pub fn is_legacy(param: *mut AudioProcessorParameter) -> bool {
        
        todo!();
        /*
            return (dynamic_cast<LegacyAudioParameter*> (param) != nullptr);
        */
    }
    
    pub fn get_param_index(
        processor: &mut dyn AudioProcessorInterface,
        param:     *mut AudioProcessorParameter

    ) -> i32 {
        
        todo!();
        /*
            if (auto* legacy = dynamic_cast<LegacyAudioParameter*> (param))
            {
                return legacy->parameterIndex;
            }
            else
            {
                auto n = processor.getNumParameters();
                jassert (n == processor.getParameters().size());

                for (int i = 0; i < n; ++i)
                {
                    if (processor.getParameters()[i] == param)
                        return i;
                }
            }

            return -1;
        */
    }
    
    pub fn get_paramid_from_param(
        param:                  *mut AudioProcessorParameter,
        force_legacy_param_ids: bool) -> String {
        
        todo!();
        /*
            if (auto* legacy = dynamic_cast<LegacyAudioParameter*> (param))
                return forceLegacyParamIDs ? String (legacy->parameterIndex) : legacy->getParamID();

            if (auto* paramWithID = dynamic_cast<AudioProcessorParameterWithID*> (param))
            {
                if (! forceLegacyParamIDs)
                    return paramWithID->paramID;
            }

            if (param != nullptr)
                return String (param->getParameterIndex());

            return {};
        */
    }
}
