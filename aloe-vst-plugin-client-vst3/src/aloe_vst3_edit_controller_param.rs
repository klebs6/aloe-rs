crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AloeVst3EditControllerParam<'a> {
    base:  VstParameter,
    owner: &'a mut AloeVst3EditController<'a>,
    param: &'a mut AudioProcessorParameter,
}

impl<'a> AloeVst3EditControllerParam<'a> {

    pub fn new(
        edit_controller:     &mut AloeVst3EditController,
        p:                   &mut AudioProcessorParameter,
        vst_paramid:         VstParamID,
        vst_unitid:          VstUnitID,
        is_bypass_parameter: bool) -> Self {
    
        todo!();
        /*
        : owner(editController),
        : param(p),

            info.id = vstParamID;
                info.unitId = vstUnitID;

                updateParameterInfo();

                info.stepCount = (i32) 0;

               #if ! ALOE_FORCE_LEGACY_PARAMETER_AUTOMATION_TYPE
                if (param.isDiscrete())
               #endif
                {
                    const int numSteps = param.getNumSteps();
                    info.stepCount = (i32) (numSteps > 0 && numSteps < 0x7fffffff ? numSteps - 1 : 0);
                }

                info.defaultNormalizedValue = param.getDefaultValue();
                jassert (info.defaultNormalizedValue >= 0 && info.defaultNormalizedValue <= 1.0f);

                // Is this a meter?
                if ((((unsigned int) param.getCategory() & 0xffff0000) >> 16) == 2)
                    info.flags = VstParameterInfo::kIsReadOnly;
                else
                    info.flags = param.isAutomatable() ? VstParameterInfo::kCanAutomate : 0;

                if (isBypassParameter)
                    info.flags |= VstParameterInfo::kIsBypass;

                valueNormalized = info.defaultNormalizedValue;
        */
    }
    
    pub fn update_parameter_info(&mut self) -> bool {
        
        todo!();
        /*
            auto updateParamIfChanged = [] (VstString128& paramToUpdate, const String& newValue)
                {
                    if (toString (paramToUpdate) == newValue)
                        return false;

                    toString128 (paramToUpdate, newValue);
                    return true;
                };

                auto anyUpdated = updateParamIfChanged (info.title,      param.getName (128));
                anyUpdated     |= updateParamIfChanged (info.shortTitle, param.getName (8));
                anyUpdated     |= updateParamIfChanged (info.units,      param.getLabel());

                return anyUpdated;
        */
    }
    
    pub fn set_normalized(&mut self, v: VstParamValue) -> bool {
        
        todo!();
        /*
            v = jlimit (0.0, 1.0, v);

                if (v != valueNormalized)
                {
                    valueNormalized = v;

                    // Only update the AudioProcessor here if we're not playing,
                    // otherwise we get parallel streams of parameter value updates
                    // during playback
                    if (! owner.vst3IsPlaying)
                    {
                        auto value = static_cast<float> (v);

                        param.setValue (value);

                        const InParameterChangedCallbackSetter scopedSetter { inParameterChangedCallback };
                        param.sendValueChangedMessageToListeners (value);
                    }

                    changed();
                    return true;
                }

                return false;
        */
    }
    
    pub fn to_string(&self, 
        value:  VstParamValue,
        result: VstString128)  {
        
        todo!();
        /*
            if (LegacyAudioParameter::isLegacy (&param))
                    // remain backward-compatible with old Aloe code
                    toString128 (result, param.getCurrentValueAsText());
                else
                    toString128 (result, param.getText ((float) value, 128));
        */
    }
    
    pub fn from_string(&self, 
        text:                 *const VstTChar,
        out_value_normalized: &mut VstParamValue) -> bool {
        
        todo!();
        /*
            if (! LegacyAudioParameter::isLegacy (&param))
                {
                    outValueNormalized = param.getValueForText (getStringFromVstTChars (text));
                    return true;
                }

                return false;
        */
    }
    
    pub fn get_string_from_vst_tchars(text: *const VstTChar) -> String {
        
        todo!();
        /*
            return String (CharPointer_UTF16 (reinterpret_cast<const CharPointer_UTF16::CharType*> (text)));
        */
    }
    
    pub fn to_plain(&self, v: VstParamValue) -> VstParamValue {
        
        todo!();
        /*
            return v;
        */
    }
    
    pub fn to_normalized(&self, v: VstParamValue) -> VstParamValue {
        
        todo!();
        /*
            return v;
        */
    }
}
