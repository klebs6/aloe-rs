crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/icloneable.h]

/**
  | Interface allowing an object to be copied.
  | 
  | - [plug & host imp]
  | 
  | - [released: N4.12]
  |
  */
pub trait ICloneable: FUnknown {

    /**
      | Create exact copy of the object
      |
      */
    #[PLUGIN_API]
    fn clone(&mut self) -> *mut dyn FUnknown;
}

lazy_static!{
    /*
    static const FUID icloneable_iid;
    */
}

declare_class_iid!{
    ICloneable, 
    0xD45406B9, 
    0x3A2D4443, 
    0x9DAD9BA9, 
    0x85A1454B
}
