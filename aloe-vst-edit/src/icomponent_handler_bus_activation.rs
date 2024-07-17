crate::ix!();

/**
  | Extended host callback interface for
  | an edit controller: Vst::IComponentHandlerBusActivation
  | \ingroup vstIHost vst368
  | 
  | - [host imp]
  | 
  | - [extends IComponentHandler]
  | 
  | - [released: 3.6.8]
  | 
  | - [optional]
  | 
  | Allows the plug-in to request the host
  | to activate or deactivate a specific
  | bus.
  | 
  | If the host accepts this request, it
  | will call later on \ref IComponent::activateBus.
  | 
  | This is particularly useful for instruments
  | with more than 1 outputs, where the user
  | could request from the plug-in UI a given
  | output bus activation.
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  |     // somewhere in your code when you need to inform the host to enable a specific Bus.
  |     FUnknownPtr<IComponentHandlerBusActivation> busActivation (componentHandler);
  |     if (busActivation)
  |     {
  |         // here we want to activate our audio input sidechain (the 2cd input bus: index 1)
  |         busActivation->requestBusActivation (kAudio, kInput, 1, true);
  |     }
  | \see \ref IComponentHandler
  |
  */
pub trait IComponentHandlerBusActivation: FUnknown {

    /**
      | request the host to activate or deactivate
      | a specific bus.
      |
      */
    #[PLUGIN_API]
    fn request_bus_activation(&mut self, 
            ty:    MediaType,
            dir:   BusDirection,
            index: i32,
            state: TBool) -> tresult;
}

lazy_static!{
    /*
    static const FUID icomponent_handler_bus_activation_iid;
    */
}

declare_class_iid!{
    IComponentHandlerBusActivation, 
    0x067D02C1, 
    0x5B4E274D, 
    0xA92D90FD, 
    0x6EAF7240
}
