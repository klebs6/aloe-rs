crate::ix!();

/**
  | Component extension to access program
  | list data: Vst::IProgramListData
  | \ingroup vstIPlug vst300
  | 
  | - [plug imp]
  | 
  | - [extends IComponent]
  | 
  | - [released: 3.0.0]
  | 
  | - [optional]
  | 
  | A component can support program list
  | data via this interface or/and unit
  | preset data (IUnitData).
  | 
  | \see IUnitData, \ref vst3MultitimbralPrograms
  |
  */
pub trait IProgramListData: FUnknown {

    /**
      | Returns kResultTrue if the given Program
      | List ID supports Program Data.
      |
      */
    #[PLUGIN_API]
    fn program_data_supported(&mut self, list_id: ProgramListID) -> tresult;

    /**
      | Gets for a given program list ID and program
      | index the program Data.
      |
      */
    #[PLUGIN_API]
    fn get_program_data(
        &mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        data:          *mut dyn IBStream

    ) -> tresult;

    /**
      | Sets for a given program list ID and program
      | index a program Data.
      |
      */
    #[PLUGIN_API]
    fn set_program_data(
        &mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        data:          *mut dyn IBStream
    ) -> tresult;
}

lazy_static!{
    /*
    static const FUID iprogram_list_data_iid;
    */
}

declare_class_iid!{
    IProgramListData, 
    0x8683B01F, 
    0x7B354F70, 
    0xA2651DEC, 
    0x353AF4FF
}
