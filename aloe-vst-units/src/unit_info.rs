crate::ix!();

/**
  | Special UnitIDs for UnitInfo
  |
  */
pub const ROOT_UNIT_ID:      UnitID = 0;  // identifier for the top level unit (root)
pub const NO_PARENT_UNIT_ID: UnitID = -1; // used for the root unit which does not have a parent.

/**
  | Special ProgramListIDs for UnitInfo
  |
  */
pub const NO_PROGRAM_LIST_ID: ProgramListID = -1; // /< no programs are used in the unit.

/**
  | Basic Unit Description. \see IUnitInfo
  |
  */
pub struct UnitInfo
{
    /**
      | unit identifier
      |
      */
    id:              UnitID,

    /**
      | identifier of parent unit (kNoParentUnitId:
      | does not apply, this unit is the root)
      */
    parent_unit_id:  UnitID,

    /**
      | name, optional for the root component,
      | required otherwise
      */
    name:            String128,

    /**
      | id of program list used in unit
      | (kNoProgramListId = no programs used in this
      | unit)
      */
    program_list_id: ProgramListID,
}
