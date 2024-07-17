crate::ix!();

/**
  | Edit controller component interface
  | extension: Vst::IEditController2
  | \ingroup vstIPlug vst310
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.1.0]
  | 
  | - [optional]
  | 
  | Extension to allow the host to inform
  | the plug-in about the host Knob Mode,
  | and to open the plug-in about box or help
  | documentation.
  | 
  | \see \ref IEditController, \ref EditController
  |
  */
pub trait IEditController2: FUnknown {

    /**
      | Host could set the Knob Mode for the plug-in.
      | Return kResultFalse means not supported
      | mode. \see KnobModes.
      |
      */
    #[PLUGIN_API]
    fn set_knob_mode(&mut self, mode: KnobMode) -> tresult;

    /**
      | Host could ask to open the plug-in help
      | (could be: opening a PDF document or
      | link to a web page).
      | 
      | The host could call it with onlyCheck
      | set to true for testing support of open
      | Help.
      | 
      | Return kResultFalse means not supported
      | function.
      |
      */
    #[PLUGIN_API]
    fn open_help(&mut self, only_check: TBool) -> tresult;

    /**
      | Host could ask to open the plug-in about
      | box.
      | 
      | The host could call it with onlyCheck
      | set to true for testing support of open
      | AboutBox.
      | 
      | Return kResultFalse means not supported
      | function.
      |
      */
    #[PLUGIN_API]
    fn open_about_box(&mut self, only_check: TBool) -> tresult;
}

lazy_static!{
    /*
    static const FUID iedit_controller2_iid;
    */
}

declare_class_iid!{
    IEditController2, 
    0x7F4EFE59, 
    0xF3204967, 
    0xAC27A3AE, 
    0xAFB63038
}
