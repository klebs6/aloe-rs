crate::ix!();

/**
  | The sharing mode of the audio stream.
  |
  */
#[repr(i32)]
// aaudio_sharing_mode_t
#[derive(Clone)]
pub enum OboeSharingMode { 

    /**
      | This will be the only stream using a particular
      | source or sink.
      | 
      | This mode will provide the lowest possible
      | latency.
      | 
      | You should close EXCLUSIVE streams
      | immediately when you are not using them.
      | 
      | If you do not need the lowest possible
      | latency then we recommend using Shared,
      | which is the default.
      |
      */
    Exclusive = 0, // AAUDIO_SHARING_MODE_EXCLUSIVE,

    /**
      | Multiple applications can share the
      | same device.
      | 
      | The data from output streams will be
      | mixed by the audio service.
      | 
      | The data for input streams will be distributed
      | by the audio service.
      | 
      | This will have higher latency than the
      | EXCLUSIVE mode.
      |
      */
    Shared = 1, // AAUDIO_SHARING_MODE_SHARED,
}
