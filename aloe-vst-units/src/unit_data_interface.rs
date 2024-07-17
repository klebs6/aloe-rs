crate::ix!();

/**
  | Component extension to access unit
  | data: Vst::IUnitData \ingroup vstIPlug
  | vst300
  | 
  | - [plug imp]
  | 
  | - [extends IComponent]
  | 
  | - [released: 3.0.0]
  | 
  | - [optional]
  | 
  | A component can support unit preset
  | data via this interface or program list
  | data (IProgramListData).
  | 
  | \see \ref vst3ProgramLists
  |
  */
pub trait IUnitData: FUnknown {

    /**
      | Returns kResultTrue if the specified
      | unit supports export and import of preset
      | data.
      |
      */
    #[PLUGIN_API]
    fn unit_data_supported(&mut self, unitid: UnitID) -> tresult;

    /**
      | Gets the preset data for the specified
      | unit.
      |
      */
    #[PLUGIN_API]
    fn get_unit_data(
        &mut self, 
        unit_id: UnitID,
        data:    *mut dyn IBStream
    ) -> tresult;

    /**
      | Sets the preset data for the specified
      | unit.
      |
      */
    #[PLUGIN_API]
    fn set_unit_data(
        &mut self, 
        unit_id: UnitID,
        data:    *mut dyn IBStream

    ) -> tresult;
}

lazy_static!{
    /*
    static const FUID iunit_data_iid;
    */
}

declare_class_iid!{
    IUnitData, 
    0x6C389611, 
    0xD391455D, 
    0xB870B833, 
    0x94A0EFDD
}
