crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/updatehandler.h]

/**
  | Handle Send and Cancel pending message
  | for a given object
  |
  */
pub trait IUpdateManager: FUnknown {

    /**
      | cancel pending messages send by \param
      | object or by any if object is 0
      |
      */
    #[PLUGIN_API]
    fn cancel_updates(&mut self, object: *mut dyn FUnknown) -> tresult;

    /**
      | send pending messages send by \param
      | object or by any if object is 0
      |
      */
    #[PLUGIN_API]
    fn trigger_defered_updates(&mut self, object: *mut dyn FUnknown) -> tresult;
}

lazy_static!{
    /*
    static const FUID iupdate_manager_iid;
    */
}

declare_class_iid!{
    IUpdateManager, 
    0x030B780C, 
    0xD6E6418D, 
    0x8CE00BC2, 
    0x09C834D4
}
