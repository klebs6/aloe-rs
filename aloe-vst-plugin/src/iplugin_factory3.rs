crate::ix!();

/* ----- IPluginFactory3 interface declaration  ----- */

/**
  | Version 3 of class factory supporting
  | PClassInfoW: IPluginFactory3 \ingroup
  | pluginBase \copydoc IPluginFactory
  |
  */
pub trait IPluginFactory3: IPluginFactory2 {

    /**
      | Returns the unicode class info for a
      | given index.
      |
      */
    #[PLUGIN_API]
    fn get_class_info_unicode(&mut self, 
            index: i32,
            info:  *mut PClassInfoW) -> tresult;

    /**
      | Receives information about host
      |
      */
    #[PLUGIN_API]
    fn set_host_context(&mut self, context: *mut dyn FUnknown) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplugin_factory3_iid;
    */
}

declare_class_iid!{
    IPluginFactory3, 
    0x4555A2AB, 
    0xC1234E57, 
    0x9B122910, 
    0x36878931
}
