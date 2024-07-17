crate::ix!();

/**
  | The time at which the frame at `position`
  | was presented
  |
  */
#[derive(Clone)]
pub struct OboeFrameTimestamp {

    /**
      | in frames
      |
      */
    position:  i64,

    /**
      | in nanoseconds
      |
      */
    timestamp: i64,
}
