crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/istringresult.h]

/**
  | Interface to return an ascii string
  | of variable size.
  | 
  | In order to manage memory allocation
  | and deallocation properly, this interface
  | is used to transfer a string as result
  | parameter of a method requires a string
  | of unknown size.
  | 
  | - [host imp] or [plug imp]
  | 
  | - [released: SX 4]
  |
  */
pub trait IStringResult: FUnknown {

    #[PLUGIN_API]
    fn set_text(&mut self, text: *const u8);
}

lazy_static!{
    /*
    static const FUID istring_result_iid;
    */
}

declare_class_iid!{
    IStringResult, 
    0x550798BC, 
    0x872049DB, 
    0x84920A15, 
    0x3B50B7A8
}

/**
  | Interface to a string of variable size
  | and encoding.
  | 
  | - [host imp] or [plug imp]
  | 
  | - [released: ]
  |
  */
pub trait IString: FUnknown {

    /**
      | Assign ASCII string
      |
      */
    #[PLUGIN_API]
    fn set_text8(&mut self, text: *const u8);

    /**
      | Assign unicode string
      |
      */
    #[PLUGIN_API]
    fn set_text16(&mut self, text: *const u16);

    /**
      | Return ASCII string. If the string is
      | unicode so far, it will be converted.
      | 
      | So you need to be careful, because the
      | conversion can result in data loss.
      | 
      | It is save though to call getText8 if
      | isWideString() returns false
      |
      */
    #[PLUGIN_API]
    fn get_text8(&mut self) -> *const u8;

    /**
      | Return unicode string. If the string
      | is ASCII so far, it will be converted.
      |
      */
    #[PLUGIN_API]
    fn get_text16(&mut self) -> *const u16;

    /**
      | !Do not use this method! Early implementations
      | take the given pointer as internal string
      | and this will cause problems because
      | 'free' will be used to delete the passed
      | memory.
      | 
      | Later implementations will redirect
      | 'take' to setText8 and setText16
      |
      */
    #[PLUGIN_API]
    fn take(&mut self, 
            s:       *mut c_void,
            is_wide: bool);

    /**
      | Returns true if the string is in unicode
      | format, returns false if the string
      | is ASCII
      |
      */
    #[PLUGIN_API]
    fn is_wide_string(&self) -> bool;
}

lazy_static!{
    /*
    static const FUID istring_iid;
    */
}

declare_class_iid!{
    IString, 
    0xF99DB7A3, 
    0x0FC14821, 
    0x800B0CF9, 
    0x8E348EDF
}
