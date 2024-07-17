crate::ix!();

/**
  | Edit controller extension to describe
  | the plug-in structure: Vst::IUnitInfo
  | \ingroup vstIPlug vst300
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.0.0]
  | 
  | - [optional]
  | 
  | IUnitInfo describes the internal structure
  | of the plug-in.
  | 
  | - The root unit is the component itself,
  | so getUnitCount must return 1 at least.
  | 
  | - The root unit id has to be 0 (kRootUnitId).
  | 
  | - Each unit can reference one program
  | list - this reference must not change.
  | 
  | - Each unit, using a program list, references
  | one program of the list.
  | 
  | \see \ref vst3Units, IUnitHandler
  |
  */
pub trait IUnitInfo: FUnknown {

    /**
      | Returns the flat count of units.
      |
      */
    #[PLUGIN_API]
    fn get_unit_count(&mut self) -> i32;

    /**
      | Gets UnitInfo for a given index in the
      | flat list of unit.
      |
      */
    #[PLUGIN_API]
    fn get_unit_info(&mut self, 
            unit_index: i32,
            info:       &mut UnitInfo) -> tresult;

    /**
      | Component intern program structure.
      | */ /** Gets the count of Program List.
      |
      */
    #[PLUGIN_API]
    fn get_program_list_count(&mut self) -> i32;

    /**
      | Gets for a given index the Program List
      | Info.
      |
      */
    #[PLUGIN_API]
    fn get_program_list_info(&mut self, 
            list_index: i32,
            info:       &mut ProgramListInfo) -> tresult;

    /**
      | Gets for a given program list ID and program
      | index its program name.
      |
      */
    #[PLUGIN_API]
    fn get_program_name(&mut self, 
            list_id:       ProgramListID,
            program_index: i32,
            name:          String128) -> tresult;

    /**
      | Gets for a given program list ID, program
      | index and attributeId the associated
      | attribute value.
      |
      */
    #[PLUGIN_API]
    fn get_program_info(
        &mut self, 
        list_id:         ProgramListID,
        program_index:   i32,
        attribute_id:    &str,
        attribute_value: String128
    ) -> tresult;

    /**
      | Returns kResultTrue if the given program
      | index of a given program list ID supports
      | PitchNames.
      |
      */
    #[PLUGIN_API]
    fn has_program_pitch_names(&mut self, 
            list_id:       ProgramListID,
            program_index: i32) -> tresult;

    /**
      | Gets the PitchName for a given program
      | list ID, program index and pitch.
      | 
      | If PitchNames are changed the plug-in
      | should inform the host with IUnitHandler::notifyProgramListChange.
      |
      */
    #[PLUGIN_API]
    fn get_program_pitch_name(&mut self, 
            list_id:       ProgramListID,
            program_index: i32,
            midi_pitch:    i16,
            name:          String128) -> tresult;

    /* ----- units selection --------------------  */

    /**
      | Gets the current selected unit.
      |
      */
    #[PLUGIN_API]
    fn get_selected_unit(&mut self) -> UnitID;

    /**
      | Sets a new selected unit.
      |
      */
    #[PLUGIN_API]
    fn select_unit(&mut self, unit_id: UnitID) -> tresult;

    /**
      | Gets the according unit if there is an
      | unambiguous relation between a channel
      | or a bus and a unit.
      | 
      | This method mainly is intended to find
      | out which unit is related to a given MIDI
      | input channel.
      |
      */
    #[PLUGIN_API]
    fn get_unit_by_bus(&mut self, 
            ty:        MediaType,
            dir:       BusDirection,
            bus_index: i32,
            channel:   i32,
            unit_id:   &mut UnitID) -> tresult;

    /**
      | Receives a preset data stream.
      | 
      | - If the component supports program
      | list data (IProgramListData), the
      | destination of the data stream is the
      | program specified by list-Id and program
      | index (first and second parameter)
      | 
      | - If the component supports unit data
      | (IUnitData), the destination is the
      | unit specified by the first parameter
      | - in this case parameter programIndex
      | is < 0).
      |
      */
    #[PLUGIN_API]
    fn set_unit_program_data(
        &mut self, 
        list_or_unit_id: i32,
        program_index:   i32,
        data:            *mut dyn IBStream

    ) -> tresult;
}

lazy_static!{
    /*
    static const FUID iunit_info_iid;
    */
}

declare_class_iid!{
    IUnitInfo, 
    0x3D4BD6B5, 
    0x913A4FD2, 
    0xA886E768, 
    0xA5EB92C1
}
