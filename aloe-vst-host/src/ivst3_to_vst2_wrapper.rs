crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivsthostapplication.h]

/**
  | Vst 3 to Vst 2 Wrapper interface: Vst::IVst3ToVst2Wrapper
  | \ingroup vstIHost vst310
  | 
  | - [host imp]
  | 
  | - [passed as 'context' to IPluginBase::initialize
  | () ]
  | 
  | - [released: 3.1.0]
  | 
  | - [mandatory]
  | 
  | Informs the plug-in that a Vst 3 to Vst
  | 2 wrapper is used between the plug-in
  | and the real host.
  | 
  | Implemented by the Vst 2 Wrapper.
  |
  */
pub trait IVst3ToVst2Wrapper: FUnknown { }

lazy_static!{
    /*
    static const FUID ivst3_to_vst2_wrapper_iid;
    */
}

declare_class_iid!{
    IVst3ToVst2Wrapper, 
    0x29633AEC, 
    0x1D1C47E2, 
    0xBB85B97B, 
    0xD36EAC61
}

