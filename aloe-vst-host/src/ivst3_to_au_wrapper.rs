crate::ix!();

/**
  | VST 3 to AU Wrapper interface: Vst::IVst3ToAUWrapper
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
  | Informs the plug-in that a VST 3 to AU
  | wrapper is used between the plug-in
  | and the real host.
  | 
  | Implemented by the AU Wrapper.
  |
  */
pub trait IVst3ToAUWrapper: FUnknown { }

lazy_static!{
    /*
    static const FUID ivst3_to_au_wrapper_iid;
    */
}

declare_class_iid!{
    IVst3ToAUWrapper, 
    0xA3B8C6C5, 
    0xC0954688, 
    0xB0916F0B, 
    0xB697AA44
}
