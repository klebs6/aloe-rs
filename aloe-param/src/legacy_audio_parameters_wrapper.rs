crate::ix!();

///--------------------
pub struct LegacyAudioParametersWrapper {
    params:                   Vec<*mut AudioProcessorParameter>,
    legacy:                   Vec<Box<LegacyAudioParameter>>,
    legacy_param_ids:         bool, // default = false
    using_managed_parameters: bool, // default = false
}

impl LegacyAudioParametersWrapper {

    pub fn update(
        &mut self, 
        audio_processor:        &mut dyn AudioProcessorInterface,
        force_legacy_param_ids: bool

    ) {
        
        todo!();
        /*
            clear();

            legacyParamIDs = forceLegacyParamIDs;

            auto numParameters = audioProcessor.getNumParameters();
            usingManagedParameters = audioProcessor.getParameters().size() == numParameters;

            for (int i = 0; i < numParameters; ++i)
            {
                AudioProcessorParameter* param = usingManagedParameters ? audioProcessor.getParameters()[i]
                                                                        : (legacy.add (new LegacyAudioParameter (audioProcessor, i)));
                params.add (param);
            }
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            legacy.clear();
            params.clear();
        */
    }
    
    pub fn get_param_for_index(&self, index: i32) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, params.size()))
                return params[index];

            return nullptr;
        */
    }
    
    pub fn get_paramid(
        &self, 
        processor: &mut dyn AudioProcessorInterface,
        idx:       i32

    ) -> String {
        
        todo!();
        /*
            if (usingManagedParameters && ! legacyParamIDs)
                return processor.getParameterID (idx);

            return String (idx);
        */
    }
    
    pub fn is_using_managed_parameters(&self) -> bool {
        
        todo!();
        /*
            return usingManagedParameters;
        */
    }
    
    pub fn get_num_parameters(&self) -> i32 {
        
        todo!();
        /*
            return params.size();
        */
    }
}
