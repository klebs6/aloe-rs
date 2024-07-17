crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/audio_play_head/aloe_AudioPlayHead.h]

/**
  | A subclass of AudioPlayHead can supply
  | information about the position and status of
  | a moving play head during audio playback.
  |
  | One of these can be supplied to an
  | AudioProcessor object so that it can find out
  | about the position of the audio that it is
  | rendering.
  |
  | @see AudioProcessor::setPlayHead,
  | AudioProcessor::getPlayHead
  |
  | @tags{Audio}
  */
pub trait AudioPlayHeadInterface {

    /**
      | Fills-in the given structure with details
      | about the transport's position at the
      | start of the current processing block.
      | If this method returns false then the
      | current play head position is not available
      | and the given structure will be undefined.
      | You can ONLY call this from your processBlock()
      | method! Calling it at other times will
      | produce undefined behaviour, as the
      | host may not have any context in which
      | a time would make sense, and some hosts
      | will almost certainly have multithreading
      | issues if it's not called on the audio
      | thread.
      |
      */
    fn get_current_position(&mut self, 
        result: &mut AudioPlayHeadCurrentPositionInfo) -> bool;

    /**
      | Returns true if this object can control
      | the transport.
      |
      */
    fn can_control_transport(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }

    /**
      | Starts or stops the audio.
      |
      */
    fn transport_play(&mut self, should_start_playing: bool)  {
        
        todo!();
        /*
            ignoreUnused (shouldStartPlaying);
        */
    }

    /**
      | Starts or stops recording the audio.
      |
      */
    fn transport_record(&mut self, should_start_recording: bool)  {
        
        todo!();
        /*
            ignoreUnused (shouldStartRecording);
        */
    }

    /**
      | Rewinds the audio.
      |
      */
    fn transport_rewind(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
