/*!
  | Contains functions to control the system's
  | master volume.
  | 
  | @tags{Audio}
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_SystemAudioVolume.h]

/**
  | Returns the operating system's current
  | volume level in the range 0 to 1.0
  |
  */
pub fn system_audio_volume_get_gain() -> f32 {
    
    todo!();
    /*
    
    */
}

/**
  | Attempts to set the operating system's
  | current volume level.
  | 
  | -----------
  | @param newGain
  | 
  | the level, between 0 and 1.0
  | 
  | -----------
  | @return
  | 
  | true if the operation succeeds
  |
  */
pub fn system_audio_volume_set_gain(new_gain: f32) -> bool {
    
    todo!();
    /*
    
    */
}

/**
  | Returns true if the system's audio output
  | is currently muted.
  |
  */
pub fn system_audio_volume_is_muted() -> bool {
    
    todo!();
    /*
    
    */
}

/**
  | Attempts to mute the operating system's
  | audio output.
  | 
  | -----------
  | @param shouldBeMuted
  | 
  | true if you want it to be muted
  | 
  | -----------
  | @return
  | 
  | true if the operation succeeds
  |
  */
pub fn system_audio_volume_set_muted(should_be_muted: bool) -> bool {
    
    todo!();
    /*
    
    */
}
