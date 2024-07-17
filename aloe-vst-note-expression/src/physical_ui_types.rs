crate::ix!();

/**
  | Physical UI Type
  |
  */
pub type PhysicalUITypeID = u32;

/**
  | PhysicalUITypeIDs describes the type
  | of Physical UI (PUI) which could be associated
  | to a note expression. \see PhysicalUIMap
  |
  */
pub enum PhysicalUITypeIDs
{
    /**
      | absolute X position when touching keys
      | of PUIs. Range [0=left, 0.5=middle,
      | 1=right]
      |
      */
    PUIXMovement = 0,

    /**
      | absolute Y position when touching keys
      | of PUIs. Range [0=bottom/near, 0.5=center,
      | 1=top/far]
      |
      */
    PUIYMovement,

    /**
      | pressing a key down on keys of PUIs. Range
      | [0=No Pressure, 1=Full Pressure]
      |
      */
    PUIPressure,

    /**
      count of current defined PUIs
      */
    PUITypeCount, 

    /**
       indicates an invalid or not initialized PUI
       type
      */
    InvalidPUITypeID = 0xFFFFFFFF 
}
