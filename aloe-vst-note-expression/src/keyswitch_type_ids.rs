crate::ix!();

pub type KeyswitchTypeID = u32;

/**
  | KeyswitchTypeIDs describes the type
  | of a key switch \see KeyswitchInfo
  |
  */
#[repr(u32)]
pub enum KeyswitchTypeIDs 
{
    /**
      | press before noteOn is played
      |
      */
    NoteOnKeyswitchTypeID = 0, 

    /**
      | press while noteOn is played
      |
      */
    OnTheFlyKeyswitchTypeID,   

    /**
      | press before entering release
      |
      */
    OnReleaseKeyswitchTypeID,  

    /**
      | key should be maintained pressed for
      | playing
      |
      */
    KeyRangeTypeID,             
}
