crate::ix!();

/**
  | Test Helper extension. \ingroup TestClass
  |
  */
pub trait ITestPlugProvider2: ITestPlugProvider {

    /**
      | get the plugin factory.
      | 
      | The reference count of the returned
      | factory object is not increased when
      | calling this function.
      |
      */
    #[PLUGIN_API]
    fn get_plugin_factory(&mut self) -> *mut dyn IPluginFactory;
}

lazy_static!{
    /*
    static const FUID itest_plug_provider2_iid;
    */
}

declare_class_iid!{
    ITestPlugProvider2, 
    0xC7C75364, 
    0x7B8343AC, 
    0xA4495B0A, 
    0x3E5A46C7
}
