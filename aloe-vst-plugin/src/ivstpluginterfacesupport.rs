crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstpluginterfacesupport.h]

/**
  | Host callback interface for an edit
  | controller: Vst::IPlugInterfaceSupport
  | \ingroup vstIHost vst3612
  | 
  | - [host imp]
  | 
  | - [released: 3.6.12]
  | 
  | - [optional]
  | 
  | Allows a plug-in to ask the host if a given
  | plug-in interface is supported/used
  | by the host.
  | 
  | It is implemented by the hostContext
  | given when the component is initialized.
  | 
  | \section IPlugInterfaceSupportExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | tresult PLUGIN_API MyPluginController::initialize (FUnknown* context)
  | {
  |     // ...
  |     FUnknownPtr<IPlugInterfaceSupport> plugInterfaceSupport (context);
  |     if (plugInterfaceSupport)
  |     {
  |         if (plugInterfaceSupport->isPlugInterfaceSupported (IMidiMapping::iid) == kResultTrue)
  |             // IMidiMapping is used by the host
  |     }
  |     // ...
  | }
  | \see IPluginBase
  |
  */
pub trait IPlugInterfaceSupport: FUnknown {

    /**
      | Returns kResultTrue if the associated
      | interface to the given _iid is supported/used
      | by the host.
      |
      */
    #[PLUGIN_API]
    fn is_plug_interface_supported(&mut self, iid: TUID) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplug_interface_support_iid;
    */
}

declare_class_iid!{
    IPlugInterfaceSupport, 
    0x4FB58B9E, 
    0x9EAA4E0F, 
    0xAB361C1C, 
    0xCCB56FEA
}
