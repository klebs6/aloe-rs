crate::ix!();

/**
  | Extended host callback interface for
  | an edit controller: Vst::IComponentHandler2
  | \ingroup vstIHost vst310
  | 
  | - [host imp]
  | 
  | - [extends IComponentHandler]
  | 
  | - [released: 3.1.0]
  | 
  | - [optional]
  | 
  | One part handles:
  | 
  | - Setting dirty state of the plug-in
  | 
  | - Requesting the host to open the editor
  | 
  | The other part handles parameter group
  | editing from the plug-in UI. It wraps
  | a set of \ref IComponentHandler::beginEdit
  | / \ref typename Steinberg::Vst::IComponentHandler::performEdit
  | / \ref typename Steinberg::Vst::IComponentHandler::endEdit
  | functions (see \ref IComponentHandler)
  | which should use the same timestamp
  | in the host when writing automation.
  | 
  | This allows for better synchronizing
  | of multiple parameter changes at once.
  | 
  | \section IComponentHandler2Example
  | Examples of different use cases
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | // we are in the editcontroller...
  | // in case of multiple switch buttons (with associated ParamID 1 and 3)
  | // on mouse down :
  | hostHandler2->startGroupEdit ();
  | hostHandler->beginEdit (1);
  | hostHandler->beginEdit (3);
  | hostHandler->performEdit (1, 1.0);
  | hostHandler->performEdit (3, 0.0); // the opposite of paramID 1 for example
  | ....
  | // on mouse up :
  | hostHandler->endEdit (1);
  | hostHandler->endEdit (3);
  | hostHandler2->finishGroupEdit ();
  | ....
  | ....
  | 
  | // in case of multiple faders (with associated ParamID 1 and 3)
  | // on mouse down :
  | hostHandler2->startGroupEdit ();
  | hostHandler->beginEdit (1);
  | hostHandler->beginEdit (3);
  | hostHandler2->finishGroupEdit ();
  | ....
  | // on mouse move :
  | hostHandler2->startGroupEdit ();
  | hostHandler->performEdit (1, x); // x the wanted value
  | hostHandler->performEdit (3, x);
  | hostHandler2->finishGroupEdit ();
  | ....
  | // on mouse up :
  | hostHandler2->startGroupEdit ();
  | hostHandler->endEdit (1);
  | hostHandler->endEdit (3);
  | hostHandler2->finishGroupEdit ();
  | \see \ref IComponentHandler, \ref
  | IEditController
  |
  */
pub trait IComponentHandler2: FUnknown {

    /**
      | Tells host that the plug-in is dirty
      | (something besides parameters has
      | changed since last save), if true the
      | host should apply a save before quitting.
      |
      */
    #[PLUGIN_API]
    fn set_dirty(&mut self, state: TBool) -> tresult;

    /**
      | Tells host that it should open the plug-in
      | editor the next time it's possible.
      | 
      | You should use this instead of showing
      | an alert and blocking the program flow
      | (especially on loading projects).
      |
      */
    #[PLUGIN_API]
    //ViewType_kEditor
    fn request_open_editor(&mut self, name: FIDString) -> tresult;

    /**
      | Starts the group editing (call before
      | a \ref IComponentHandler::beginEdit),
      | the host will keep the current timestamp
      | at this call and will use it for all \ref
      | IComponentHandler::beginEdit / \ref
      | IComponentHandler::performEdit
      | / \ref IComponentHandler::endEdit
      | calls until a \ref finishGroupEdit
      | ().
      |
      */
    #[PLUGIN_API]
    fn start_group_edit(&mut self) -> tresult;

    /**
      | Finishes the group editing started
      | by a \ref startGroupEdit (call after
      | a \ref IComponentHandler::endEdit).
      |
      */
    #[PLUGIN_API]
    fn finish_group_edit(&mut self) -> tresult;
}

lazy_static!{
    /*
    static const FUID icomponent_handler2_iid;
    */
}

declare_class_iid!{
    IComponentHandler2, 
    0xF040B4B3, 
    0xA36045EC, 
    0xABCDC045, 
    0xB4D5A2CC
}
