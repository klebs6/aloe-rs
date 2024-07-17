crate::ix!();

/**
  | VST 3 to AAX Wrapper interface: Vst::IVst3ToAAXWrapper
  | \ingroup vstIHost vst368
  | 
  | - [host imp]
  | 
  | - [passed as 'context' to IPluginBase::initialize() ]
  | 
  | - [released: 3.6.8]
  | 
  | - [mandatory]
  | 
  | Informs the plug-in that a VST 3 to AAX
  | wrapper is used between the plug-in
  | and the real host.
  | 
  | Implemented by the AAX Wrapper.
  |
  */
pub trait IVst3ToAAXWrapper: FUnknown {

}

lazy_static!{
    /*
    static const FUID ivst3_to_aax_wrapper_iid;
    */
}

declare_class_iid!{
    IVst3ToAAXWrapper, 
    0x6D319DC6, 
    0x60C56242, 
    0xB32C951B, 
    0x93BEF4C6
}
