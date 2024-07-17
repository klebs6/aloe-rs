crate::ix!();

pub type StringVector = Vec<*mut TChar>;

/**
  | Description of a StringListParameter.
  | \ingroup vstClasses
  |
  */
pub struct StringListParameter {
    base:    Parameter,
    strings: StringVector,
}

obj_methods!{ StringListParameter, Parameter }

impl Drop for StringListParameter {

    fn drop(&mut self) {
        todo!();
        /*
            for (auto& string : strings)
            std::free (string);
        */
    }
}

impl From<&ParameterInfo> for StringListParameter {

    fn from(param_info: &ParameterInfo) -> Self {
    
        todo!();
        /*
        : parameter(paramInfo),
        */
    }
}

impl StringListParameter {
    
    pub fn new(
        title:       *const TChar,
        tag:         ParamID,
        units:       *const TChar,
        flags:       Option<ParameterInfoParameterFlags>,
        unitid:      Option<UnitID>,
        short_title: *const TChar

    ) -> Self {

        let flags = flags.unwrap_or(ParameterInfoParameterFlags::CanAutomate | ParameterInfoParameterFlags::IsList);

        let unitid: UnitID = unitid.unwrap_or(ROOT_UNIT_ID);
    
        todo!();
        /*


            UString (info.title, str16BufferSize (String128)).assign (title);
        if (units)
            UString (info.units, str16BufferSize (String128)).assign (units);
        if (shortTitle)
            UString (info.shortTitle, str16BufferSize (String128)).assign (shortTitle);

        info.stepCount = -1;
        info.defaultNormalizedValue = 0;
        info.flags = flags;
        info.id = tag;
        info.unitId = unitID;
        */
    }
    
    pub fn append_string(&mut self, string: String128)  {
        
        todo!();
        /*
            int32 length = strlen16 (string);
        TChar* buffer = (TChar*)std::malloc ((length + 1) * sizeof (TChar));
        if (!buffer)
            return;

        memcpy (buffer, string, length * sizeof (TChar));
        buffer[length] = 0;
        strings.push_back (buffer);
        info.stepCount++;
        */
    }
    
    pub fn replace_string(&mut self, 
        index:  i32,
        string: String128) -> bool {
        
        todo!();
        /*
            TChar* str = strings.at (index);
        if (!str)
            return false;

        int32 length = strlen16 (string);
        TChar* buffer = (TChar*)malloc ((length + 1) * sizeof (TChar));
        if (!buffer)
            return false;

        memcpy (buffer, string, length * sizeof (TChar));
        buffer[length] = 0;
        strings[index] = buffer;
        std::free (str);
        return true;
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
            int32 index = (int32)toPlain (_valueNormalized);
        if (const TChar* valueString = strings.at (index))
        {
            UString (string, str16BufferSize (String128)).assign (valueString);
        }
        else
            string[0] = 0;
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
            int32 index = 0;
        for (auto it = strings.begin (), end = strings.end (); it != end; ++it, ++index)
        {
            if (strcmp16 (*it, string) == 0)
            {
                _valueNormalized = toNormalized ((ParamValue)index);
                return true;
            }
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
            if (info.stepCount <= 0)
            return 0;
        return FromNormalized<ParamValue> (_valueNormalized, info.stepCount);
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
            if (info.stepCount <= 0)
            return 0;
        return ToNormalized<ParamValue> (plainValue, info.stepCount);
        */
    }
}
