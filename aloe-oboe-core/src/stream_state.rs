crate::ix!();

/**
  | The state of the audio stream.
  |
  */
#[repr(i32)]
// aaudio_stream_state_t
#[derive(Clone)]
pub enum OboeStreamState { 

    /**
      | AAUDIO_STREAM_STATE_UNINITIALIZED,
      |
      */
    Uninitialized = 0, 

    /**
      | AAUDIO_STREAM_STATE_UNKNOWN,
      |
      */
    Unknown       = 1, 

    /**
      | AAUDIO_STREAM_STATE_OPEN,
      |
      */
    Open          = 2, 

    /**
      | AAUDIO_STREAM_STATE_STARTING,
      |
      */
    Starting      = 3, 

    /**
      | AAUDIO_STREAM_STATE_STARTED,
      |
      */
    Started       = 4, 

    /**
      | AAUDIO_STREAM_STATE_PAUSING,
      |
      */
    Pausing       = 5, 

    /**
      | AAUDIO_STREAM_STATE_PAUSED,
      |
      */
    Paused        = 6, 

    /**
      | AAUDIO_STREAM_STATE_FLUSHING,
      |
      */
    Flushing      = 7, 

    /**
      | AAUDIO_STREAM_STATE_FLUSHED,
      |
      */
    Flushed       = 8, 

    /**
      | AAUDIO_STREAM_STATE_STOPPING,
      |
      */
    Stopping      = 9, 

    /**
      | AAUDIO_STREAM_STATE_STOPPED,
      |
      */
    Stopped       = 10, 

    /**
      | AAUDIO_STREAM_STATE_CLOSING,
      |
      */
    Closing       = 11, 

    /**
      | AAUDIO_STREAM_STATE_CLOSED,
      |
      */
    Closed        = 12, 

    /**
      | AAUDIO_STREAM_STATE_DISCONNECTED,
      |
      */
    Disconnected  = 13, 
}
