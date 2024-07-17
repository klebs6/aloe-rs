/**
  | Description : Vst Parameter Implementation
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstparameters.h]

/**
  | Description of a Parameter. \ingroup
  | vstClasses
  |
  */
pub struct Parameter {
    base:             FObject,
    info:             ParameterInfo,
    value_normalized: ParamValue,
    precision:        i32,
}

obj_methods!{ Parameter, FObject }

pub trait GetInfo {

    fn get_info(&self) -> &ParameterInfo;
}

pub trait GetInfoMut {

    fn get_info(&mut self) -> &mut ParameterInfo;
}

impl GetInfo for Parameter {

    /**
      | Returns its read only info.
      |
      */
    fn get_info(&self) -> &ParameterInfo {
        
        todo!();
        /*
            return info;
        */
    }
}

impl GetInfoMut for Parameter {

    /**
      | Returns its writable info.
      |
      */
    fn get_info(&mut self) -> &mut ParameterInfo {
        
        todo!();
        /*
            return info;
        */
    }
}

pub trait SetUnitID {

    fn set_unitid(&mut self, id: UnitID);
}

impl SetUnitID for Parameter {

    /**
      | Sets its associated UnitId.
      |
      */
    fn set_unitid(&mut self, id: UnitID)  {
        
        todo!();
        /*
            info.unitId = id;
        */
    }
}

pub trait GetUnitID {

    fn get_unitid(&mut self) -> UnitID;
}

impl GetUnitID for Parameter {

    /**
      | Gets its associated UnitId.
      |
      */
    fn get_unitid(&mut self) -> UnitID {
        
        todo!();
        /*
            return info.unitId;
        */
    }
}

pub trait GetNormalized {

    fn get_normalized(&self) -> ParamValue;
}

impl GetNormalized for Parameter {

    /**
      | Gets its normalized value [0.0, 1.0].
      |
      */
    fn get_normalized(&self) -> ParamValue {
        
        todo!();
        /*
            return valueNormalized;
        */
    }
}

impl Parameter {

    /**
      | Converts a normalized value to a string.
      |
      */
    fn to_string(
        &self, 
        norm_value: ParamValue,
        string:     String128
    ) {
        todo!();

        /*
            UString wrapper (string, str16BufferSize (String128));
        if (info.stepCount == 1)
        {
            if (normValue > 0.5)
            {
                wrapper.assign (STR16 ("On"));
            }
            else
            {
                wrapper.assign (STR16 ("Off"));
            }
        }
        else
        {
            if (!wrapper.printFloat (normValue, precision))
                string[0] = 0;
        }
        */
    }
    
}

pub trait GetPrecision {

    fn get_precision(&self) -> i32;
}

impl GetPrecision for Parameter {

    /**
      | Gets the current precision (used for
      | string representation of float).
      |
      */
    fn get_precision(&self) -> i32 {
        
        todo!();
        /*
            return precision;
        */
    }
}

pub trait SetPrecision {

    fn set_precision(&mut self, val: i32);
}

impl SetPrecision for Parameter {

    /**
      | Sets the precision for string representation
      | of float value (for example 4.34 with
      | 2 as precision).
      |
      */
    fn set_precision(&mut self, val: i32)  {
        
        todo!();
        /*
            precision = val;
        */
    }
}
    
impl Default for Parameter {
    
    fn default() -> Self {
    
        todo!();
        /*
        : value_normalized(0.),
        : precision(4),

            info = {};
        */
    }
}

impl From<&ParameterInfo> for Parameter {
    
    fn from(info: &ParameterInfo) -> Self {
    
        todo!();
        /*
        : info(info),
        : value_normalized(info.defaultNormalizedValue),
        : precision(4),

        
        */
    }
}

impl Parameter {

    pub fn new(
        title:                    *const TChar,
        tag:                      ParamID,
        units:                    *const TChar,
        default_value_normalized: Option<ParamValue>,
        step_count:               Option<i32>,
        flags:                    Option<ParameterInfoParameterFlags>,
        unitid:                   Option<UnitID>,
        short_title:              *const TChar

    ) -> Self {

        let default_value_normalized: ParamValue =
                 default_value_normalized.unwrap_or(0.);

        let step_count: i32 = step_count.unwrap_or(0);

        let flags = flags.unwrap_or(ParameterInfoParameterFlags::CanAutomate);

        let unitid: UnitID = unitid.unwrap_or(ROOT_UNIT_ID);
    
        todo!();
        /*
        : precision(4),

            info = {};

        UString (info.title, str16BufferSize (String128)).assign (title);
        if (units)
            UString (info.units, str16BufferSize (String128)).assign (units);
        if (shortTitle)
            UString (info.shortTitle, str16BufferSize (String128)).assign (shortTitle);

        info.stepCount = stepCount;
        info.defaultNormalizedValue = valueNormalized = defaultValueNormalized;
        info.flags = flags;
        info.id = tag;
        info.unitId = unitID;
        */
    }
    
    pub fn set_normalized(&mut self, norm_value: ParamValue) -> bool {
        
        todo!();
        /*
            if (normValue > 1.0)
        {
            normValue = 1.0;
        }
        else if (normValue < 0.)
        {
            normValue = 0.;
        }

        if (normValue != valueNormalized)
        {
            valueNormalized = normValue;
            changed ();
            return true;
        }
        return false;
        */
    }
    
    pub fn from_string(&self, 
        string:     *const TChar,
        norm_value: &mut ParamValue) -> bool {
        
        todo!();
        /*
            UString wrapper (const_cast<TChar*> (string), tstrlen (string));
        return wrapper.scanFloat (normValue);
        */
    }
    
    pub fn to_plain(&self, norm_value: ParamValue) -> ParamValue {
        
        todo!();
        /*
            return normValue;
        */
    }
    
    pub fn to_normalized(&self, plain_value: ParamValue) -> ParamValue {
        
        todo!();
        /*
            return plainValue;
        */
    }
}
