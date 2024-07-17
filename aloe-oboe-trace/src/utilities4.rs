crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/Utilities.h]

/**
  | Convert an array of floats to an array
  | of 16-bit integers.
  | 
  | -----------
  | @param source
  | 
  | the input array.
  | ----------
  | @param destination
  | 
  | the output array.
  | ----------
  | @param numSamples
  | 
  | the number of values to convert.
  |
  */
pub fn oboe_convert_float_to_pcm16(
    source:      *const f32,
    destination: *mut i16,
    num_samples: i32)  {

    todo!();
    /*
    
    */
}

/**
  | Convert an array of 16-bit integers
  | to an array of floats.
  | 
  | -----------
  | @param source
  | 
  | the input array.
  | ----------
  | @param destination
  | 
  | the output array.
  | ----------
  | @param numSamples
  | 
  | the number of values to convert.
  |
  */
pub fn oboe_convert_pcm_16to_float(
    source:      *const i16,
    destination: *mut f32,
    num_samples: i32)  {

    todo!();
    /*
    
    */
}

/**
  | @return
  | 
  | the size of a sample of the given format
  | in bytes or 0 if format is invalid
  |
  */
pub fn oboe_convert_format_to_size_in_bytes(format: OboeAudioFormat) -> i32 {
    
    todo!();
    /*
    
    */
}

/**
  | The text is the ASCII symbol corresponding
  | to the supplied Oboe enum value, or an
  | English message saying the value is
  | unrecognized.
  | 
  | This is intended for developers to use
  | when debugging. It is not for displaying
  | to users.
  | 
  | -----------
  | @param input
  | 
  | object to convert from. @see common/Utilities.cpp
  | for concrete implementations
  | 
  | -----------
  | @return
  | 
  | text representation of an Oboe enum
  | value. There is no need to call free on
  | this.
  |
  */
pub fn oboe_convert_to_text<FromType>(input: FromType) -> *const u8 {

    todo!();
    /*
    
    */
}

/**
  | @param name
  | 
  | @return the value of a named system property
  | in a string or empty string
  |
  */
pub fn oboe_get_property_string(name: *const u8) -> String {
    
    todo!();
    /*
    
    */
}

/**
  | @param name
  | 
  | @param defaultValue
  | 
  | -----------
  | @return
  | 
  | integer value associated with a property
  | or the default value
  |
  */
pub fn oboe_get_property_integer(
    name:          *const u8,
    default_value: i32) -> i32 {

    todo!();
    /*
    
    */
}

/**
  | Return the version of the SDK that is
  | currently running.
  | 
  | For example, on Android, this would
  | return 27 for Oreo 8.1. If the version
  | number cannot be determined then this
  | will return -1.
  | 
  | -----------
  | @return
  | 
  | version number or -1
  |
  */
pub fn oboe_get_sdk_version() -> i32 {
    
    todo!();
    /*
    
    */
}

