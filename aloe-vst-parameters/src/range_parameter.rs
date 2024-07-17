crate::ix!();

/**
  | Description of a RangeParameter. \ingroup
  | vstClasses
  |
  */
pub struct RangeParameter {
    base:      Parameter,
    min_plain: ParamValue,
    max_plain: ParamValue,
}

pub trait GetMin {

    fn get_min(&self) -> ParamValue;
}

impl GetMin for RangeParameter {

    /**
      | Gets the minimum plain value, same as
      | toPlain (0).
      |
      */
    fn get_min(&self) -> ParamValue {
        
        todo!();
        /*
            return minPlain;
        */
    }
}

pub trait SetMin {

    fn set_min(&mut self, value: ParamValue);
}

impl SetMin for RangeParameter {

    /**
      | Sets the minimum plain value.
      |
      */
    fn set_min(&mut self, value: ParamValue)  {
        
        todo!();
        /*
            minPlain = value;
        */
    }
}

pub trait GetMax {

    fn get_max(&self) -> ParamValue;
}

impl GetMax for RangeParameter {

    /**
      | Gets the maximum plain value, same as
      | toPlain (1).
      |
      */
    fn get_max(&self) -> ParamValue {
        
        todo!();
        /*
            return maxPlain;
        */
    }
}

pub trait SetMax {

    fn set_max(&mut self, value: ParamValue);
}

impl SetMax for RangeParameter {

    /**
      | Sets the maximum plain value.
      |
      */
    fn set_max(&mut self, value: ParamValue)  {
        
        todo!();
        /*
            maxPlain = value;
        */
    }
}

obj_methods!{ RangeParameter, Parameter }

impl Default for RangeParameter {

    fn default() -> Self {
    
        todo!();
        /*
        : min_plain(0),
        : max_plain(1),

        
        */
    }
}

impl RangeParameter {
    
    pub fn new_from_param_info(
        param_info: &ParameterInfo,
        min:        ParamValue,
        max:        ParamValue) -> Self {
    
        todo!();
        /*
        : parameter(paramInfo),
        : min_plain(min),
        : max_plain(max),

        
        */
    }
    
    pub fn new(
        title:               *const TChar,
        tag:                 ParamID,
        units:               *const TChar,
        min_plain:           Option<ParamValue>,
        max_plain:           Option<ParamValue>,
        default_value_plain: Option<ParamValue>,
        step_count:          Option<i32>,
        flags:               Option<ParameterInfoParameterFlags>,
        unitid:              Option<UnitID>,
        short_title:         *const TChar) -> Self {
    
        let min_plain: ParamValue = min_plain.unwrap_or(0.0);

        let max_plain: ParamValue = max_plain.unwrap_or(1.0);

        let default_value_plain: ParamValue = default_value_plain.unwrap_or(0.0);

        let step_count: i32 = step_count.unwrap_or(0);

        let flags = flags.unwrap_or(ParameterInfoParameterFlags::CanAutomate);

        let unitid: UnitID = unitid.unwrap_or(ROOT_UNIT_ID);

        todo!();
        /*
        : min_plain(minPlain),
        : max_plain(maxPlain),

            UString (info.title, str16BufferSize (String128)).assign (title);
        if (units)
            UString (info.units, str16BufferSize (String128)).assign (units);
        if (shortTitle)
            UString (info.shortTitle, str16BufferSize (String128)).assign (shortTitle);

        info.stepCount = stepCount;
        info.defaultNormalizedValue = valueNormalized = toNormalized (defaultValuePlain);
        info.flags = flags;
        info.id = tag;
        info.unitId = unitID;
        */
    }
    
    /**
      | Converts a normalized value to a string.
      |
      */
    pub fn to_string(&self, 
        value_normalized: ParamValue,
        string:           String128)  {
        
        todo!();
        /*
            if (info.stepCount > 1)
        {
            UString wrapper (string, str16BufferSize (String128));
            int64 plain = static_cast<int64> (toPlain (_valueNormalized));
            if (!wrapper.printInt (plain))
                string[0] = 0;
        }
        else
        {
            Parameter::toString (toPlain (_valueNormalized), string);
        }
        */
    }
    
    /**
      | Converts a string to a normalized value.
      |
      */
    pub fn from_string(&self, 
        string:           *const TChar,
        value_normalized: &mut ParamValue) -> bool {
        
        todo!();
        /*
            UString wrapper (const_cast<TChar*> (string), tstrlen (string));
        if (info.stepCount > 1)
        {
            int64 plainValue;
            if (wrapper.scanInt (plainValue))
            {
                _valueNormalized = toNormalized ((ParamValue)plainValue);
                return true;
            }
            return false;
        }
        if (wrapper.scanFloat (_valueNormalized))
        {
            if (_valueNormalized < getMin ())
                _valueNormalized = getMin ();
            else if (_valueNormalized > getMax ())
                _valueNormalized = getMax ();
            _valueNormalized = toNormalized (_valueNormalized);
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts a normalized value to plain
      | value (e.g. 0.5 to 10000.0Hz).
      |
      */
    pub fn to_plain(&self, value_normalized: ParamValue) -> ParamValue {
        
        todo!();
        /*
            if (info.stepCount > 1)
            return FromNormalized<ParamValue> (_valueNormalized, info.stepCount) + getMin ();
        return _valueNormalized * (getMax () - getMin ()) + getMin ();
        */
    }
    
    /**
      | Converts a plain value to a normalized
      | value (e.g. 10000 to 0.5).
      |
      */
    pub fn to_normalized(&self, plain_value: ParamValue) -> ParamValue {
        
        todo!();
        /*
            if (info.stepCount > 1)
            return ToNormalized <ParamValue>(plainValue - getMin (), info.stepCount);
        return (plainValue - getMin ()) / (getMax () - getMin ());
        */
    }
}
