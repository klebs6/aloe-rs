crate::ix!();

/**
  | Special programIndex value for IUnitHandler::notifyProgramListChange
  |
  */
pub const ALL_PROGRAM_INVALID: i32 = -1; // all program information is invalid

/**
  | Host callback for unit support: Vst::IUnitHandler
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [extends IComponentHandler]
  | 
  | - [released: 3.0.0]
  | 
  | - [optional]
  | 
  | Host callback interface, used with
  | IUnitInfo.
  | 
  | Retrieve via queryInterface from IComponentHandler.
  | 
  | \see \ref vst3Units, IUnitInfo
  |
  */
pub trait IUnitHandler: FUnknown {

    /**
      | Notify host when a module is selected
      | in plug-in GUI.
      |
      */
    #[PLUGIN_API]
    fn notify_unit_selection(&mut self, unit_id: UnitID) -> tresult;

    /**
      | Tell host that the plug-in controller
      | changed a program list (rename, load,
      | PitchName changes).
      | 
      | -----------
      | @param listId
      | 
      | is the specified program list ID to inform.
      | ----------
      | @param programIndex
      | 
      | : when kAllProgramInvalid, all program
      | information is invalid, otherwise
      | only the program of given index.
      |
      */
    #[PLUGIN_API]
    fn notify_program_list_change(&mut self, 
            list_id:       ProgramListID,
            program_index: i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID iunit_handler_iid;
    */
}

declare_class_iid!{
    IUnitHandler, 
    0x4B5147F8, 
    0x4654486B, 
    0x8DAB30BA, 
    0x163A3C56
}
