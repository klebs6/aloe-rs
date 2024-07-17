crate::ix!();

/**
  | Wrapper MPE Support interface: Vst::IVst3WrapperMPESupport
  | \ingroup vstIHost vst3612
  | 
  | - [host imp]
  | 
  | - [passed as 'context' to IPluginBase::initialize() ]
  | 
  | - [released: 3.6.12]
  | 
  | - [optional]
  | 
  | Implemented on wrappers that support
  | MPE to Note Expression translation.
  | 
  | By default, MPE input processing is
  | enabled, the masterChannel will be
  | zero, the memberBeginChannel will
  | be one and the memberEndChannel will
  | be 14.
  | 
  | As MPE is a subset of the Vst3 Note Expression
  | feature, mapping from the three MPE
  | expressions is handled via the 
  | INoteExpressionPhysicalUIMapping
  | interface.
  |
  */
pub trait IVst3WrapperMPESupport: FUnknown {

    /**
      | enable or disable MPE processing
      | 
      | -----------
      | @param state
      | 
      | true to enable, false to disable MPE
      | processing
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn enable_mpe_input_processing(&mut self, state: TBool) -> tresult;

    /**
      | setup the MPE processing
      | 
      | -----------
      | @param masterChannel
      | 
      | MPE master channel (zero based)
      | ----------
      | @param memberBeginChannel
      | 
      | MPE member begin channel (zero based)
      | ----------
      | @param memberEndChannel
      | 
      | MPE member end channel (zero based)
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn set_mpe_input_device_settings(&mut self, 
            master_channel:       i32,
            member_begin_channel: i32,
            member_end_channel:   i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID ivst3_wrapper_mpe_support_iid;
    */
}

declare_class_iid!{
    IVst3WrapperMPESupport, 
    0x44149067, 
    0x42CF4BF9, 
    0x8800B750, 
    0xF7359FE3
}
