crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivsttestplugprovider.h]

/**
  | Test Helper. \ingroup TestClass
  | 
  | This class provides access to the component
  | and the controller of a plug-in when
  | running a unit test (see ITest).
  | 
  | You get this interface as the context
  | argument in the ITestFactory::createTests
  | method.
  |
  */
pub trait ITestPlugProvider: FUnknown {

    /**
      | get the component of the plug-in.
      | 
      | The reference count of the component
      | is increased in this function and you
      | need to call releasePlugIn when done
      | with the component.
      |
      */
    #[PLUGIN_API]
    fn get_component(&mut self) -> *mut dyn VstIComponent;

    /**
      | get the controller of the plug-in.
      | 
      | The reference count of the controller
      | is increased in this function and you
      | need to call releasePlugIn when done
      | with the controller.
      |
      */
    #[PLUGIN_API]
    fn get_controller(&mut self) -> *mut dyn IEditController;

    /**
      | release the component and/or controller
      |
      */
    #[PLUGIN_API]
    fn release_plug_in(
        &mut self, 
        component:  *mut dyn VstIComponent,
        controller: *mut dyn IEditController

    ) -> tresult;

    /**
      | get the sub categories of the plug-in
      |
      */
    #[PLUGIN_API]
    fn get_sub_categories(&self, result: &mut dyn IStringResult) -> tresult;

    /**
      | get the component UID of the plug-in
      |
      */
    #[PLUGIN_API]
    fn get_componentuid(&self, uid: &mut FUID) -> tresult;
}

lazy_static!{
    /*
    static const FUID itest_plug_provider_iid;
    */
}

declare_class_iid!{
    ITestPlugProvider, 
    0x86BE70EE, 
    0x4E99430F, 
    0x978F1E6E, 
    0xD68FB5BA
}
