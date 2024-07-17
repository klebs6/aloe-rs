crate::ix!();

/**
  | KeyswitchInfo is the structure describing
  | a key switch
  | 
  | This structure is used by the method
  | \ref IKeyswitchController::getKeyswitchInfo.
  | \see IKeyswitchController
  |
  */
pub struct KeyswitchInfo
{
    /**
      | see KeyswitchTypeID
      |
      */
    type_id:       KeyswitchTypeID,

    /**
      | name of key switch (e.g. "Accentuation")
      |
      */
    title:         String128,

    /**
      | short title (e.g. "Acc")
      |
      */
    short_title:   String128,

    /**
      | associated main key switch min (value
      | between [0, 127])
      |
      */
    keyswitch_min: i32,

    /**
      | associated main key switch max (value
      | between [0, 127])
      |
      */
    keyswitch_max: i32,

    /**
      | optional remapped key switch (default
      | -1), the plug-in could provide one remapped
      | key for a key switch (allowing better
      | location on the keyboard of the key switches)
      |
      */
    key_remapped:  i32,

    /**
      | id of unit this key switch belongs to (see
      | \ref vst3Units), -1 means no unit used.
      */
    unit_id:       i32,

    /**
      | not yet used (set to 0)
      |
      */
    flags:         i32,
}
