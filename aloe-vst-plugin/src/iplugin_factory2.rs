crate::ix!();

/* ----- IPluginFactory2 interface declaration  ----- */

/**
  | Version 2 of class factory supporting
  | PClassInfo2: IPluginFactory2 \ingroup
  | pluginBase \copydoc IPluginFactory
  |
  */
pub trait IPluginFactory2: IPluginFactory {

    /**
      | Returns the class info (version 2) for
      | a given index.
      |
      */
    #[PLUGIN_API]
    fn get_class_info2(&mut self, 
            index: i32,
            info:  *mut PClassInfo2) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplugin_factory2_iid;
    */
}

declare_class_iid!{
    IPluginFactory2, 
    0x0007B650, 
    0xF24B4C0B, 
    0xA464EDB9, 
    0xF00B2ABB
}
