/*!
  | A note on use of preprocessor defines:
  | 
  | This is one of the few times when it's
  | suitable to use preprocessor defines
  | rather than constexpr Why? Because
  | C++11 requires a lot of boilerplate
  | code to convert integers into compile-time
  | string literals. The preprocessor,
  | despite it's lack of type checking,
  | is more suited to the task
  | 
  | See: https://stackoverflow.com/questions/6713420/c-convert-integer-to-string-at-compile-time/26824971#26824971
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/Version.h]

/**
  | Type: 8-bit unsigned int.
  | 
  | Min value: 0
  | 
  | Max value: 255.
  | 
  | See below for description.
  |
  */
pub const OBOE_VERSION_MAJOR: u8 = 1;

/**
   Type: 8-bit unsigned int. Min value: 0 Max
   value: 255. See below for description.
  */
pub const OBOE_VERSION_MINOR: u8 = 6;

/**
   Type: 16-bit unsigned int. Min value: 0 Max
   value: 65535. See below for description.
  */
pub const OBOE_VERSION_PATCH: u16 = 1;

macro_rules! oboe_stringify {
    ($x:ident) => {
        /*
                #x
        */
    }
}

macro_rules! oboe_tostring {
    ($x:ident) => {
        /*
                OBOE_STRINGIFY(x)
        */
    }
}

/**
   Type: String literal. See below for
   description.
  */
macro_rules! oboe_version_text {
    () => {
        /*
        
                OBOE_TOSTRING(OBOE_VERSION_MAJOR) "." 
                OBOE_TOSTRING(OBOE_VERSION_MINOR) "." 
                OBOE_TOSTRING(OBOE_VERSION_PATCH)
        */
    }
}

/**
  | Type: 32-bit unsigned int. See below
  | for description.
  |
  */
pub const OBOE_VERSION_NUMBER: u32 = ((OBOE_VERSION_MAJOR as u32) << 24) | ((OBOE_VERSION_MINOR as u32) << 16) | (OBOE_VERSION_PATCH as u32);

pub fn oboe_get_version_text() -> *const u8 {
    
    todo!();
    /*
    
    */
}

/**
  | Oboe versioning object
  |
  */
pub mod oboe_version {
    use super::*;

    /**
      | This is incremented when we make breaking
      | API changes. Based loosely on https://semver.org/.
      |
      */
    pub const Major: u8 = OBOE_VERSION_MAJOR;

    /**
      | This is incremented when we add backwards
      | compatible functionality. Or set to
      | zero when MAJOR is incremented.
      |
      */
    pub const Minor: u8 = OBOE_VERSION_MINOR;

    /**
      | This is incremented when we make backwards
      | compatible bug fixes. Or set to zero
      | when MINOR is incremented.
      |
      */
    pub const Patch: u16 = OBOE_VERSION_PATCH;

    /**
      | Version string in the form MAJOR.MINOR.PATCH.
      |
      */
    pub const Text: &'static str = OBOE_VERSION_TEXT;

    /**
      | Integer representation of the current
      | Oboe library version. This will always
      | increase when the version number changes
      | so can be compared using integer comparison.
      |
      */
    pub const Number: u32 = OBOE_VERSION_NUMBER;
}

