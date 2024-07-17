crate::ix!();

pub trait CheckIsPlaying {

    /**
      | Returns true if the device is still calling
      | back.
      | 
      | The device might mysteriously stop,
      | so this checks whether it's still playing.
      |
      */
    fn is_playing(&mut self) -> bool;
}
