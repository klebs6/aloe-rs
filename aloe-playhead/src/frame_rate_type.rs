crate::ix!();

/**
  | Frame rate types
  |
  */
pub enum FrameRateType
{
    fps23976        = 0,
    fps24           = 1,
    fps25           = 2,
    fps2997         = 3,
    fps30           = 4,
    fps2997drop     = 5,
    fps30drop       = 6,
    fps60           = 7,
    fps60drop       = 8,
    fpsUnknown      = 99
}
