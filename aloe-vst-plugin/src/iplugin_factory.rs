crate::ix!();

/* ----- IPluginFactory interface declaration  ----- */

/**
  | Class factory that any plug-in defines
  | for creating class instances: IPluginFactory
  | \ingroup pluginBase
  | 
  | - [plug imp]
  | 
  | From the host's point of view a plug-in
  | module is a factory which can create
  | a certain kind of object(s). The interface
  | IPluginFactory provides methods to
  | get information about the classes exported
  | by the plug-in and a mechanism to create
  | instances of these classes (that usually
  | define the IPluginBase interface).
  | 
  | <b> An implementation is provided in
  | public.sdk/source/common/pluginfactory.cpp
  | </b> \see GetPluginFactory
  |
  */
pub trait IPluginFactory: FUnknown {

    /**
      | Fill a PFactoryInfo structure with
      | information about the plug-in vendor.
      |
      */
    #[PLUGIN_API]
    fn get_factory_info(&mut self, info: *mut PFactoryInfo) -> tresult;

    /**
      | Returns the number of exported classes
      | by this factory.
      | 
      | If you are using the CPluginFactory
      | implementation provided by the SDK,
      | it returns the number of classes you
      | registered with CPluginFactory::registerClass.
      |
      */
    #[PLUGIN_API]
    fn count_classes(&mut self) -> i32;

    /**
      | Fill a PClassInfo structure with information
      | about the class at the specified index.
      |
      */
    #[PLUGIN_API]
    fn get_class_info(&mut self, 
            index: i32,
            info:  *mut PClassInfo) -> tresult;

    /**
      | Create a new class instance.
      |
      */
    #[PLUGIN_API]
    fn create_instance(&mut self, 
            cid: FIDString,
            iid: FIDString,
            obj: *mut *mut c_void) -> tresult;

}

lazy_static!{
    /*
    static const FUID iplugin_factory_iid;
    */
}

declare_class_iid!{
    IPluginFactory, 
    0x7A4D811C, 0x52114A1F, 
    0xAED9D2EE, 0x0B43BF9F
}
