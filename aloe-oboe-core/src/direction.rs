crate::ix!();

/**
  | The direction of the stream.
  |
  */
#[repr(i32)]
// aaudio_direction_t
#[derive(Clone)]
pub enum OboeDirection { 

    /**
      | Used for playback.
      |
      */
    Output = 0, // AAUDIO_DIRECTION_OUTPUT,

    /**
      | Used for recording.
      |
      */
    Input = 1, // AAUDIO_DIRECTION_INPUT,
}
