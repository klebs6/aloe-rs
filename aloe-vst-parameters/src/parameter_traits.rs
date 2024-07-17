crate::ix!();

pub trait SetNormalized {

    /**
      | Sets its normalized value [0.0, 1.0].
      |
      */
    fn set_normalized(&mut self, v: ParamValue) -> bool;
}

pub trait FromString {

    /**
      | Converts a string to a normalized value.
      |
      */
    fn from_string(&self, 
            string:           *const TChar,
            value_normalized: &mut ParamValue) -> bool;
}

pub trait ToPlain {

    /**
      | Converts a normalized value to plain
      | value (e.g. 0.5 to 10000.0Hz).
      |
      */
    fn to_plain(&self, value_normalized: ParamValue) -> ParamValue;
}

pub trait ToNormalized {

    /**
      | Converts a plain value to a normalized
      | value (e.g. 10000 to 0.5).
      |
      */
    fn to_normalized(&self, plain_value: ParamValue) -> ParamValue;
}

pub trait AppendString {

    /**
      | Appends a string and increases the stepCount.
      |
      */
    fn append_string(&mut self, string: String128);
}

pub trait ReplaceString {

    /**
      | Replaces the string at index. Index
      | must be between 0 and stepCount+1
      |
      */
    fn replace_string(&mut self, 
            index:  i32,
            string: String128) -> bool;
}
