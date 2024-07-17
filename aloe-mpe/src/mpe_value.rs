crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEValue.h]

/**
  | This class represents a single value
  | for any of the MPE dimensions of control.
  | It supports values with 7-bit or 14-bit
  | resolutions (corresponding to 1 or
  | 2 MIDI bytes, respectively). It also
  | offers helper functions to query the
  | value in a variety of representations
  | that can be useful in an audio or MIDI
  | context.
  | 
  | @tags{Audio}
  |
  */
pub struct MPEValue {
    normalised_value: i32, // default = 8192
}

impl PartialEq<MPEValue> for MPEValue {
    
    #[inline] fn eq(&self, other: &MPEValue) -> bool {
        todo!();
        /*
            return normalisedValue == other.normalisedValue;
        */
    }
}

impl Eq for MPEValue {}

impl Default for MPEValue {
    
    /**
      | Default constructor.
      | 
      | Constructs an MPEValue corresponding
      | to the centre value.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEValue.cpp]
impl MPEValue {

    pub fn new(value: i32) -> Self {
    
        todo!();
        /*
        : normalised_value(value),

        
        */
    }
    
    /**
      | Constructs an MPEValue from an integer
      | between 0 and 127 (using 7-bit precision).
      |
      */
    pub fn from_7bit_int(&mut self, value: i32) -> MPEValue {
        
        todo!();
        /*
            jassert (value >= 0 && value <= 127);

        auto valueAs14Bit = value <= 64 ? value << 7
                                        : int (jmap<float> (float (value - 64), 0.0f, 63.0f, 0.0f, 8191.0f)) + 8192;

        return { valueAs14Bit };
        */
    }
    
    /**
      | Constructs an MPEValue from an integer
      | between 0 and 16383 (using 14-bit precision).
      |
      */
    pub fn from_14bit_int(&mut self, value: i32) -> MPEValue {
        
        todo!();
        /*
            jassert (value >= 0 && value <= 16383);
        return { value };
        */
    }
    
    /**
      | Constructs an MPEValue corresponding
      | to the minimum value.
      |
      */
    pub fn min_value(&mut self) -> MPEValue {
        
        todo!();
        /*
            return MPEValue::from7BitInt (0);
        */
    }
    
    /**
      | Constructs an MPEValue corresponding
      | to the centre value.
      |
      */
    pub fn centre_value(&mut self) -> MPEValue {
        
        todo!();
        /*
            return MPEValue::from7BitInt (64);
        */
    }
    
    /**
      | Constructs an MPEValue corresponding
      | to the maximum value.
      |
      */
    pub fn max_value(&mut self) -> MPEValue {
        
        todo!();
        /*
            return MPEValue::from7BitInt (127);
        */
    }
    
    /**
      | Retrieves the current value as an integer
      | between 0 and 127.
      | 
      | Information will be lost if the value
      | was initialised with a precision higher
      | than 7-bit.
      |
      */
    pub fn as_7bit_int(&self) -> i32 {
        
        todo!();
        /*
            return normalisedValue >> 7;
        */
    }
    
    /**
      | Retrieves the current value as an integer
      | between 0 and 16383.
      | 
      | Resolution will be lost if the value
      | was initialised with a precision higher
      | than 14-bit.
      |
      */
    pub fn as_14bit_int(&self) -> i32 {
        
        todo!();
        /*
            return normalisedValue;
        */
    }
    
    /**
      | Retrieves the current value mapped
      | to a float between -1.0f and 1.0f.
      |
      */
    pub fn as_signed_float(&self) -> f32 {
        
        todo!();
        /*
            return (normalisedValue < 8192)
               ? jmap<float> (float (normalisedValue), 0.0f, 8192.0f, -1.0f, 0.0f)
               : jmap<float> (float (normalisedValue), 8192.0f, 16383.0f, 0.0f, 1.0f);
        */
    }
    
    /**
      | Retrieves the current value mapped
      | to a float between 0.0f and 1.0f.
      |
      */
    pub fn as_unsigned_float(&self) -> f32 {
        
        todo!();
        /*
            return jmap<float> (float (normalisedValue), 0.0f, 16383.0f, 0.0f, 1.0f);
        */
    }
}
