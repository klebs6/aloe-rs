crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAUParameter.h]

/**
  | CAAUParameter
  |
  | complete parameter specification
  */
pub struct CAAUParameter {
    base:               AudioUnitParameter,

    /**
      | cached parameter info
      |
      */
    param_info:         AudioUnitParameterInfo,

    param_name:         CFStringRef,
    param_tag:          CFStringRef,
    num_indexed_params: i16,
    named_params:       CFArrayRef,
}

impl Drop for CAAUParameter {

    fn drop(&mut self) {
        todo!();
        /*
            if (mParamName) CFRelease(mParamName);
        if (mParamTag) CFRelease(mParamTag);
        if (mNamedParams) CFRelease (mNamedParams);
        */
    }
}

impl Ord for CAAUParameter {
    
    #[inline] fn cmp(&self, other: &CAAUParameter) -> std::cmp::Ordering {
        todo!();
        /*
            return memcmp(this, &a, sizeof(AudioUnitParameter)) < 0;
        */
    }
}

impl PartialOrd<CAAUParameter> for CAAUParameter {

    #[inline] fn partial_cmp(&self, other: &CAAUParameter) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<CAAUParameter> for CAAUParameter {
    
    #[inline] fn eq(&self, other: &CAAUParameter) -> bool {
        todo!();
        /*
            return !memcmp(this, &a, sizeof(AudioUnitParameter));
        */
    }
}

impl Eq for CAAUParameter {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAUParameter.cpp]
impl From<&CAAUParameter> for CAAUParameter {

    fn from(a: &CAAUParameter) -> Self {
    
        todo!();
        /*


            memset(this, 0, sizeof(CAAUParameter));
        *this = a;
        */
    }
}

impl From<&mut AudioUnitParameter> for CAAUParameter {

    fn from(in_param: &mut AudioUnitParameter) -> Self {
    
        todo!();
        /*


            memset(this, 0, sizeof(CAAUParameter));
        Init (inParam.mAudioUnit, inParam.mParameterID, inParam.mScope, inParam.mElement);
        */
    }
}

impl Default for CAAUParameter {

    fn default() -> Self {
    
        todo!();
        /*


            memset(this, 0, sizeof(CAAUParameter));
        */
    }
}

impl CAAUParameter {
    
    /**
      | borrowed reference!
      |
      */
    pub fn get_name(&self) -> CFStringRef {
        
        todo!();
        /*
            return mParamName;
        */
    }

    pub fn values_have_strings(&self) -> bool {
        
        todo!();
        /*
            return (mParamInfo.flags & kAudioUnitParameterFlag_ValuesHaveStrings) != 0;
        */
    }

    pub fn param_info(&self) -> &AudioUnitParameterInfo {
        
        todo!();
        /*
            return mParamInfo;
        */
    }

    /**
      | this may return null! -
      |
      | in which case there is no descriptive tag
      | for the parameter
      */
    pub fn get_param_tag(&self) -> CFStringRef {
        
        todo!();
        /*
            return mParamTag;
        */
    }
    
    pub fn get_param_name(&self, in_index: i32) -> CFStringRef {
        
        todo!();
        /*
            // this can return null if there is no name for the parameter
                                        return (mNamedParams && inIndex < mNumIndexedParams)
                                                    ? (CFStringRef) CFArrayGetValueAtIndex(mNamedParams, inIndex)
                                                    : 0;
        */
    }
    
    pub fn get_num_indexed_params(&self) -> i32 {
        
        todo!();
        /*
            return mNumIndexedParams;
        */
    }
    
    pub fn is_indexed_param(&self) -> bool {
        
        todo!();
        /*
            return mNumIndexedParams != 0;
        */
    }
    
    pub fn has_named_params(&self) -> bool {
        
        todo!();
        /*
            return IsIndexedParam() && mNamedParams;
        */
    }
    
    pub fn get_clumpid(&self, out_clumpid: &mut u32) -> bool {
        
        todo!();
        /*
            if (mParamInfo.flags & kAudioUnitParameterFlag_HasClump) {
                                            outClumpID = mParamInfo.clumpID;
                                            return true;
                                        }
                                        return false;
        */
    }
    
    pub fn has_display_transformation(&self) -> bool {
        
        todo!();
        /*
            return GetAudioUnitParameterDisplayType (mParamInfo.flags);
        */
    }
    
    pub fn is_expert(&self) -> bool {
        
        todo!();
        /*
            return mParamInfo.flags & kAudioUnitParameterFlag_ExpertMode;
        */
    }

    #[cfg(DEBUG)]
    pub fn print(&self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | these methods are defined in
      | CAPersistence.cpp
      |
      | they will persist and restore only the
      | scope, element and param ID's of the
      | AudioUnitParameter however, this is
      | sufficient to be able to save/restore
      | a CAAUParameter object
      */
    pub fn save(&self, out_data: &mut CFPropertyListRef)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn save_with_param(
        in_param: &AudioUnitParameter,
        out_data: &mut CFPropertyListRef)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn restore(
        in_data:   CFPropertyListRef,
        out_param: &mut AudioUnitParameter) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    pub fn new(
        au:      AudioUnit,
        param:   AudioUnitParameterID,
        scope:   AudioUnitScope,
        element: AudioUnitElement) -> Self {
    
        todo!();
        /*


            memset(this, 0, sizeof(CAAUParameter));
        Init (au, param, scope, element);
        */
    }
    
    pub fn assign_from(&mut self, a: &CAAUParameter) -> &mut CAAUParameter {
        
        todo!();
        /*
            if (mParamName) CFRelease(mParamName);
        if (mParamTag) CFRelease(mParamTag);
        if (mNamedParams) CFRelease(mNamedParams);

        memcpy(this, &a, sizeof(CAAUParameter));

        if (mParamName) CFRetain(mParamName);
        if (mParamTag) CFRetain(mParamTag);
        if (mNamedParams) CFRetain(mNamedParams);

        return *this;
        */
    }
    
    pub fn init(&mut self, 
        au:      AudioUnit,
        param:   AudioUnitParameterID,
        scope:   AudioUnitScope,
        element: AudioUnitElement)  {
        
        todo!();
        /*
            mAudioUnit = au;
        mParameterID = param;
        mScope = scope;
        mElement = element;

        UInt32 propertySize = sizeof(mParamInfo);
        OSStatus err = AudioUnitGetProperty(au, kAudioUnitProperty_ParameterInfo,
                scope, param, &mParamInfo, &propertySize);
        if (err)
            memset(&mParamInfo, 0, sizeof(mParamInfo));
        if (mParamInfo.flags & kAudioUnitParameterFlag_HasCFNameString) {
            mParamName = mParamInfo.cfNameString;
            if (!(mParamInfo.flags & kAudioUnitParameterFlag_CFNameRelease))
                CFRetain (mParamName);
        } else
            mParamName = CFStringCreateWithCString(NULL, mParamInfo.name, kCFStringEncodingUTF8);

        const char* str = 0;
        switch (mParamInfo.unit)
        {
            case kAudioUnitParameterUnit_Boolean:
                str = "T/F";
                break;
            case kAudioUnitParameterUnit_Percent:
            case kAudioUnitParameterUnit_EqualPowerCrossfade:
                str = "%";
                break;
            case kAudioUnitParameterUnit_Seconds:
                str = "Secs";
                break;
            case kAudioUnitParameterUnit_SampleFrames:
                str = "Samps";
                break;
            case kAudioUnitParameterUnit_Phase:
            case kAudioUnitParameterUnit_Degrees:
                str = "Degr.";
                break;
            case kAudioUnitParameterUnit_Hertz:
                str = "Hz";
                break;
            case kAudioUnitParameterUnit_Cents:
            case kAudioUnitParameterUnit_AbsoluteCents:
                str = "Cents";
                break;
            case kAudioUnitParameterUnit_RelativeSemiTones:
                str = "S-T";
                break;
            case kAudioUnitParameterUnit_MIDINoteNumber:
            case kAudioUnitParameterUnit_MIDIController:
                str = "MIDI";
                    //these are inclusive, so add one value here
                mNumIndexedParams = short(mParamInfo.maxValue+1 - mParamInfo.minValue);
                break;
            case kAudioUnitParameterUnit_Decibels:
                str = "dB";
                break;
            case kAudioUnitParameterUnit_MixerFaderCurve1:
            case kAudioUnitParameterUnit_LinearGain:
                str = "Gain";
                break;
            case kAudioUnitParameterUnit_Pan:
                str = "L/R";
                break;
            case kAudioUnitParameterUnit_Meters:
                str = "Mtrs";
                break;
            case kAudioUnitParameterUnit_Octaves:
                str = "8ve";
                break;
            case kAudioUnitParameterUnit_BPM:
                str = "BPM";
                break;
            case kAudioUnitParameterUnit_Beats:
                str = "Beats";
                break;
            case kAudioUnitParameterUnit_Milliseconds:
                str = "msecs";
                break;
            case kAudioUnitParameterUnit_Ratio:
                str = "Ratio";
                break;
            case kAudioUnitParameterUnit_Indexed:
                {
                    propertySize = sizeof(mNamedParams);
                    err = AudioUnitGetProperty (au,
                                        kAudioUnitProperty_ParameterValueStrings,
                                        scope,
                                        param,
                                        &mNamedParams,
                                        &propertySize);
                    if (!err && mNamedParams) {
                        mNumIndexedParams = CFArrayGetCount(mNamedParams);
                    } else {
                            //these are inclusive, so add one value here
                        mNumIndexedParams = short(mParamInfo.maxValue+1 - mParamInfo.minValue);
                    }
                    str = NULL;
                }
                break;
            case kAudioUnitParameterUnit_CustomUnit:
            {
                CFStringRef unitName = mParamInfo.unitName;
                static char paramStr[256];
                CFStringGetCString (unitName, paramStr, 256, kCFStringEncodingUTF8);
                if (mParamInfo.flags & kAudioUnitParameterFlag_CFNameRelease)
                    CFRelease (unitName);
                str = paramStr;
                break;
            }
            case kAudioUnitParameterUnit_Generic:
            case kAudioUnitParameterUnit_Rate:
            default:
                str = NULL;
                break;
        }

        if (str)
            mParamTag = CFStringCreateWithCString(NULL, str, kCFStringEncodingUTF8);
        else
            mParamTag = NULL;
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            Float32 value = 0.;
        //OSStatus err =
        AudioUnitGetParameter(mAudioUnit, mParameterID, mScope, mElement, &value);
        return value;
        */
    }

    /**
      | returns a copy of the name of the current
      | parameter value or null if there is no name
      | associated caller must release
      */
    pub fn get_string_from_value_copy(&self, value: *const f32) -> CFStringRef {
        
        todo!();
        /*
            if (HasNamedParams())
        {
            Float32 val = (value == NULL ? GetValue() : *value);
            int index = int(mParamInfo.minValue) + int(val);
            CFStringRef str = GetParamName (index);
            if (str) {
                CFRetain (str);
                return str;
            }
        }
        else if (ValuesHaveStrings())
        {
            AudioUnitParameterStringFromValue stringValue;
            stringValue.inParamID = mParameterID;
            stringValue.inValue = value;
            stringValue.outString = NULL;
            UInt32 propertySize = sizeof(stringValue);

            OSStatus err = AudioUnitGetProperty (mAudioUnit,
                                                kAudioUnitProperty_ParameterStringFromValue,
                                                mScope,
                                                0,
                                                &stringValue,
                                                &propertySize);

            if (!err && stringValue.outString != NULL)
                return stringValue.outString;
        }

        Float32 val = (value == NULL ? GetValue() : *value);
        AudioUnitParameterUnit unit = this->ParamInfo().unit;
        if (unit ==  kAudioUnitParameterUnit_Cents || unit == kAudioUnitParameterUnit_AbsoluteCents)
            return CreateLocalizedStringForParameterValue(val, this, 4, 0);
        else
            return CreateLocalizedStringForParameterValue(val, this, 4);
        */
    }
    
    /**
      | caller must release
      |
      */
    pub fn get_value_from_string(&self, str_: CFStringRef) -> f32 {
        
        todo!();
        /*
            if (ValuesHaveStrings())
        {
            AudioUnitParameterValueFromString valueString;
            valueString.inParamID = mParameterID;
            valueString.inString = str;
            UInt32 propertySize = sizeof(valueString);

            OSStatus err = AudioUnitGetProperty (mAudioUnit,
                                            kAudioUnitProperty_ParameterValueFromString,
                                            mScope,
                                            0,
                                            &valueString,
                                            &propertySize);

            if (!err) {
                return valueString.outValue;
            }
        }

        return (Float32) ValueForLocalizedParameterString(str, this);
        */
    }
    
    pub fn set_value(&self, 
        in_listener: AUParameterListenerRef,
        in_object:   *mut c_void,
        in_value:    f32)  {
        
        todo!();
        /*
            // clip inValue as: maxValue >= inValue >= minValue before setting
        Float32 valueToSet = inValue;
        if (valueToSet > mParamInfo.maxValue)
            valueToSet = mParamInfo.maxValue;
        if (valueToSet < mParamInfo.minValue)
            valueToSet = mParamInfo.minValue;

        AUParameterSet(inListener, inObject, this, valueToSet, 0);
        */
    }

    #[cfg(DEBUG)]
    pub fn print(&self)  {
        
        todo!();
        /*
            UInt32 clump = 0;
        GetClumpID (clump);

        UInt32 len = static_cast<UInt32>(CFStringGetLength(mParamName));
        char* chars = (char*)malloc (len * 2); // give us plenty of room for unichar chars
        if (!CFStringGetCString (mParamName, chars, len * 2, kCFStringEncodingUTF8))
            chars[0] = 0;

        printf ("ID: %ld, Clump: %u, Name: %s\n", (long unsigned int) mParameterID, (unsigned int) clump, chars);
        free (chars);
        */
    }
}
