crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCTypes.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCTypes.cpp]

/**
  | The type used for OSC type tags.
  |
  */
pub type OSCType = u8;

pub const OSC_TYPES_INT32:   OSCType = b'i';
pub const OSC_TYPES_FLOAT32: OSCType = b'f';
pub const OSC_TYPES_STRING:  OSCType = b's';
pub const OSC_TYPES_BLOB:    OSCType = b'b';
pub const OSC_TYPES_COLOUR:  OSCType = b'r';

/**
  | The type used for OSC type tag strings.
  |
  */
pub type OSCTypeList = Vec<OSCType>;

/**
  | The definitions of supported OSC types
  | and their associated OSC type tags,
  | as defined in the OpenSoundControl
  | 1.0 specification.
  | 
  | -----------
  | @note
  | 
  | This implementation does not support
  | any additional type tags that are not
  | part of the specification.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCTypes {

}

impl OSCTypes {

    pub fn is_supported_type(ty: OSCType) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::int32
                || type == OSCTypes::float32
                || type == OSCTypes::string
                || type == OSCTypes::blob
                || type == OSCTypes::colour;
        */
    }
}

/**
  | Holds a 32-bit RGBA colour for passing
  | to and from an OSCArgument. @see OSCArgument,
  | OSCTypes::colour @tags{OSC}
  |
  */
pub struct OSCColour {
    red:   u8,
    green: u8,
    blue:  u8,
    alpha: u8,
}

impl OSCColour {

    pub fn to_int32(&self) -> u32 {
        
        todo!();
        /*
            return ByteOrder::makeInt (alpha, blue, green, red);
        */
    }
    
    pub fn from_int32(&mut self, c: u32) -> OSCColour {
        
        todo!();
        /*
            return { (uint8) (c >> 24),
                 (uint8) (c >> 16),
                 (uint8) (c >> 8),
                 (uint8) c };
        */
    }
}

/**
  | Base class for exceptions that can be
  | thrown by methods in the OSC module.
  | 
  | @tags{OSC}
  |
  */
#[derive(Debug,Error)]
pub enum OSCException {

    #[error(non_std,no_from)] 
    Default {
        description: String,
    }
}

impl OSCException {

    pub fn new(desc: &String) -> Self {
    
        todo!();
        /*
        : description(desc),

            #if ! ALOE_UNIT_TESTS
            DBG ("OSCFormatError: " + description);
           #endif
        */
    }
}

/**
  | Exception type thrown when the OSC module
  | fails to parse something because of
  | a data format not compatible with the
  | OpenSoundControl 1.0 specification.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCFormatError {
    base: OSCException,
}
impl OSCFormatError {

    pub fn new(desc: &String) -> Self {
    
        todo!();
        /*
        : osc_exception(desc),

        
        */
    }
}

/**
  | Exception type thrown in cases of unexpected
  | errors in the OSC module.
  | 
  | -----------
  | @note
  | 
  | This should never happen, and all the
  | places where this is thrown should have
  | a preceding jassertfalse to facilitate
  | debugging.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCInternalError {
    base: OSCException,
}

impl OSCInternalError {

    pub fn new(desc: &String) -> Self {
    
        todo!();
        /*
        : osc_exception(desc),

        
        */
    }
}
