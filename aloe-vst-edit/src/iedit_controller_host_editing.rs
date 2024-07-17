crate::ix!();

/**
  | Parameter Editing from host: Vst::IEditControllerHostEditing
  | \ingroup vstIPlug vst350
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | If this interface is implemented by
  | the edit controller, and when performing
  | edits from outside the plug-in (host
  | / remote) of a not automatable and not
  | read-only, and not hidden flagged parameter
  | (kind of helper parameter), the host
  | will start with a beginEditFromHost
  | before calling setParamNormalized
  | and end with an endEditFromHost.
  | 
  | Here the sequence that the host will
  | call:
  | 
  | \section IEditControllerExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | plugEditController->beginEditFromHost (id);
  | plugEditController->setParamNormalized (id, value);
  | plugEditController->setParamNormalized (id, value + 0.1);
  | // ...
  | plugEditController->endEditFromHost (id);
  | 
  | \see \ref IEditController
  |
  */
pub trait IEditControllerHostEditing: FUnknown {

    /**
      | Called before a setParamNormalized
      | sequence, a endEditFromHost will be
      | call at the end of the editing action.
      |
      */
    #[PLUGIN_API]
    fn begin_edit_from_host(&mut self, paramid: ParamID) -> tresult;

    /**
      | Called after a beginEditFromHost and
      | a sequence of setParamNormalized.
      |
      */
    #[PLUGIN_API]
    fn end_edit_from_host(&mut self, paramid: ParamID) -> tresult;
}

lazy_static!{
    /*
    static const FUID iedit_controller_host_editing_iid;
    */
}

declare_class_iid!{
    IEditControllerHostEditing, 
    0xC1271208, 
    0x70594098, 
    0xB9DD34B3, 
    0x6BB0195E
}
