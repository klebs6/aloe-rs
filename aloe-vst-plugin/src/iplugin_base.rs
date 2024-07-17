crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ipluginbase.h]

/**
  | Basic interface to a plug-in component:
  | IPluginBase \ingroup pluginBase
  | 
  | - [plug imp]
  | 
  | - initialize/terminate the plug-in
  | component
  | 
  | The host uses this interface to initialize
  | and to terminate the plug-in component.
  | 
  | The context that is passed to the initialize
  | method contains any interface to the
  | host that the plug-in will need to work.
  | These interfaces can vary from category
  | to category.
  | 
  | A list of supported host context interfaces
  | should be included in the documentation
  | of a specific category.
  |
  */
pub trait IPluginBase: FUnknown {

    /**
      | The host passes a number of interfaces
      | as context to initialize the plug-in
      | class.
      | 
      | -----------
      | @note
      | 
      | Extensive memory allocations etc.
      | should be performed in this method rather
      | than in the class' constructor!
      | 
      | If the method does NOT return kResultOk,
      | the object is released immediately.
      | In this case terminate is not called!
      |
      */
    #[PLUGIN_API]
    fn initialize(&mut self, context: *mut dyn FUnknown) -> tresult;

    /**
      | This function is called before the plug-in
      | is unloaded and can be used for cleanups.
      | You have to release all references to
      | any host application interfaces.
      |
      */
    #[PLUGIN_API]
    fn terminate(&mut self) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplugin_base_iid;
    */
}

declare_class_iid!{
    IPluginBase, 
    0x22888DDB, 
    0x156E45AE, 
    0x8358B348, 
    0x08190625
}
