crate::ix!();

pub trait AllSoundOff {

    /**
      | Creates an all-sound-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isAllSoundOff
      |
      */
    fn all_sound_off(&mut self, channel: i32) -> dyn MidiMessageInterface;
}

pub trait IsAllSoundOff {

    /** 
      | Checks whether this message is an
      | all-sound-off message.
      | @see allSoundOff
      */
    fn is_all_sound_off(&self) -> bool;
}
