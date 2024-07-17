crate::ix!();

/**
  | Host callback interface for an edit
  | controller: Vst::IComponentHandler
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | Allow transfer of parameter editing
  | to component (processor) via host and
  | support automation.
  | 
  | Cause the host to react on configuration
  | changes (restartComponent).
  | 
  | \see \ref IEditController
  |
  */
pub trait IComponentHandler: FUnknown {

    /**
      | To be called before calling a performEdit
      | (e.g. on mouse-click-down event).
      | 
      | This must be called in the UI-Thread
      | context!
      |
      */
    #[PLUGIN_API]
    fn begin_edit(&mut self, id: ParamID) -> tresult;

    /**
      | Called between beginEdit and endEdit
      | to inform the handler that a given parameter
      | has a new value. This must be called in
      | the UI-Thread context!
      |
      */
    #[PLUGIN_API]
    fn perform_edit(&mut self, 
            id:               ParamID,
            value_normalized: ParamValue) -> tresult;

    /**
      | To be called after calling a performEdit
      | (e.g. on mouse-click-up event).
      | 
      | This must be called in the UI-Thread
      | context!
      |
      */
    #[PLUGIN_API]
    fn end_edit(&mut self, id: ParamID) -> tresult;

    /**
      | Instructs host to restart the component.
      | This must be called in the UI-Thread
      | context!
      | 
      | -----------
      | @param flags
      | 
      | is a combination of RestartFlags
      |
      */
    #[PLUGIN_API]
    fn restart_component(&mut self, flags: i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID icomponent_handler_iid;
    */
}

declare_class_iid!{
    IComponentHandler, 
    0x93A0BEA3, 
    0x0BD045DB, 
    0x8E890B0C, 
    0xC1E46AC6
}
