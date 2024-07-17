crate::ix!();

/**
  | Value for DataEvent::type
  |
  */
pub enum DataEventDataTypes
{
    /**
      | for MIDI system exclusive message
      |
      */
    kMidiSysEx = 0  
}

/**
  | Data event specific data. Used in \ref
  | VstEvent (union) \ingroup vstEventGrp
  |
  */
#[derive(Copy,Clone)]
pub struct DataEvent
{
    /**
      | size in bytes of the data block bytes
      |
      */
    size:  u32,

    /**
      | type of this data block (see \ref
      | 
      | DataEventDataTypes)
      |
      */
    ty:    u32,

    /**
      | pointer to the data block
      |
      */
    bytes: *const u8,
}
