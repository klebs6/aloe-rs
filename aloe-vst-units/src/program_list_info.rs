crate::ix!();

/**
  | Basic Program List Description. \see
  | IUnitInfo
  |
  */
pub struct ProgramListInfo
{
    /**
      | program list identifier
      |
      */
    id:            ProgramListID,

    /**
      | name of program list
      |
      */
    name:          String128,

    /**
      | number of programs in this list
      |
      */
    program_count: i32,
}
