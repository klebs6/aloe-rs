crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_PositionableAudioSource.h]

/**
  | A type of AudioSource which can be repositioned.
  | 
  | The basic AudioSource just streams
  | continuously with no idea of a current
  | time or length, so the PositionableAudioSource
  | is used for a finite stream that has a
  | current read position.
  | 
  | @see AudioSource, AudioTransportSource
  | 
  | @tags{Audio}
  |
  */
pub trait PositionableAudioSource: AudioSource {
    
    /**
      | Tells the stream to move to a new position.
      | 
      | Calling this indicates that the next
      | call to AudioSource::getNextAudioBlock()
      | should return samples from this position.
      | 
      | Note that this may be called on a different
      | thread to getNextAudioBlock(), so
      | the subclass should make sure it's synchronised.
      |
      */
    fn set_next_read_position(&mut self, new_position: i64);

    /**
      | Returns the position from which the
      | next block will be returned.
      | 
      | @see setNextReadPosition
      |
      */
    fn get_next_read_position(&self) -> i64;

    /**
      | Returns the total length of the stream
      | (in samples).
      |
      */
    fn get_total_length(&self) -> i64;

    /**
      | Returns true if this source is actually
      | playing in a loop.
      |
      */
    fn is_looping(&self) -> bool;

    /**
      | Tells the source whether you'd like
      | it to play in a loop.
      |
      */
    fn set_looping(&mut self, should_loop: bool)  {
        
        todo!();
        /*
            ignoreUnused (shouldLoop);
        */
    }
}
