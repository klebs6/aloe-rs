crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstplugview.h]

/**
  | Extension for IPlugView to find view
  | parameters (lookup value under mouse
  | support): Vst::IParameterFinder
  | \ingroup pluginGUI vst302
  | 
  | - [plug imp]
  | 
  | - [extends IPlugView]
  | 
  | - [released: 3.0.2]
  | 
  | - [optional]
  | 
  | It is highly recommended to implement
  | this interface.
  | 
  | A host can implement important functionality
  | when a plug-in supports this interface.
  | 
  | For example, all Steinberg hosts require
  | this interface in order to support the
  | "AI Knob".
  |
  */
pub trait IParameterFinder: FUnknown {

    /**
      | Find out which parameter in plug-in
      | view is at given position (relative
      | to plug-in view).
      |
      */
    #[PLUGIN_API]
    fn find_parameter(&mut self, 
            x_pos:      i32,
            y_pos:      i32,
            result_tag: &mut ParamID) -> tresult;
}

lazy_static!{
    /*
    static const FUID iparameter_finder_iid;
    */
}

declare_class_iid!{
    IParameterFinder, 
    0x0F618302, 
    0x215D4587, 
    0xA512073C, 
    0x77B9D383
}
