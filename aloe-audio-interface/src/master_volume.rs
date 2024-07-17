crate::ix!();

pub trait MasterVolume {

    /**
      | Creates a master-volume change message.
      | 
      | -----------
      | @param volume
      | 
      | the volume, 0 to 1.0
      |
      */
    fn master_volume(&mut self, volume: f32) -> dyn MidiMessageInterface;
}
