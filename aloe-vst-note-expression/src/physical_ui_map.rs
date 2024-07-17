crate::ix!();

/**
  | PhysicalUIMap describes a mapping
  | of a noteExpression Type to a Physical
  | UI Type.
  | 
  | It is used in PhysicalUIMapList. \see
  | PhysicalUIMapList
  |
  */
pub struct PhysicalUIMap
{
    /**
      | This represents the physical UI. /see
      | PhysicalUITypeIDs, this is set by the
      | caller of getPhysicalUIMapping
      |
      */
    physical_ui_typeid:     PhysicalUITypeID,


    /**
      | This represents the associated noteExpression
      | TypeID to the given physicalUITypeID.
      | This will be filled by the plug-in in
      | the call getPhysicalUIMapping, set
      | it to kInvalidTypeID if no Note Expression
      | is associated to the given PUI.
      |
      */
    note_expression_typeid: NoteExpressionTypeID,
}

/**
  | PhysicalUIMapList describes a list
  | of PhysicalUIMap \see INoteExpressionPhysicalUIMapping
  |
  */
pub struct PhysicalUIMapList
{
    /**
      | Count of entries in the map array, set
      | by the caller of getPhysicalUIMapping.
      |
      */
    count: u32,

    /**
      | Pointer to a list of PhysicalUIMap containing
      | count entries.
      |
      */
    map:   *mut PhysicalUIMap,
}
