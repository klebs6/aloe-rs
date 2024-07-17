crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AloeVst3EditControllerProgramChangeParameter<'a> {
    base: VstParameter,
    owner: &'a mut dyn AudioProcessorInterface,
}

impl<'a> AloeVst3EditControllerProgramChangeParameter<'a> {
    
    pub fn new(
        p:           &mut dyn AudioProcessorInterface,
        vst_paramid: VstParamID) -> Self {
    
        todo!();
        /*
        : owner(p),

            jassert (owner.getNumPrograms() > 1);

                info.id = vstParamID;
                toString128 (info.title, "Program");
                toString128 (info.shortTitle, "Program");
                toString128 (info.units, "");
                info.stepCount = owner.getNumPrograms() - 1;
                info.defaultNormalizedValue = static_cast<VstParamValue> (owner.getCurrentProgram())
                                                / static_cast<VstParamValue> (info.stepCount);
                info.unitId = VstkRootUnitId;
                info.flags = VstParameterInfo::kIsProgramChange | VstParameterInfo::kCanAutomate;
        */
    }
    
    pub fn set_normalized(&mut self, v: VstParamValue) -> bool {
        
        todo!();
        /*
            auto programValue = roundToInt (toPlain (v));

                if (isPositiveAndBelow (programValue, owner.getNumPrograms()))
                {
                    if (programValue != owner.getCurrentProgram())
                        owner.setCurrentProgram (programValue);

                    if (valueNormalized != v)
                    {
                        valueNormalized = v;
                        changed();

                        return true;
                    }
                }

                return false;
        */
    }
    
    pub fn to_string(&self, 
        value:  VstParamValue,
        result: VstString128)  {
        
        todo!();
        /*
            toString128 (result, owner.getProgramName (roundToInt (value * info.stepCount)));
        */
    }
    
    pub fn from_string(&self, 
        text:                 *const VstTChar,
        out_value_normalized: &mut VstParamValue) -> bool {
        
        todo!();
        /*
            auto paramValueString = getStringFromVstTChars (text);
                auto n = owner.getNumPrograms();

                for (int i = 0; i < n; ++i)
                {
                    if (paramValueString == owner.getProgramName (i))
                    {
                        outValueNormalized = static_cast<VstParamValue> (i) / info.stepCount;
                        return true;
                    }
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
            return v * (info.stepCount + 1);
        */
    }
    
    pub fn to_normalized(&self, v: VstParamValue) -> VstParamValue {
        
        todo!();
        /*
            return v / (info.stepCount + 1);
        */
    }
}
