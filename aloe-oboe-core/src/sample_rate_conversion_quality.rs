crate::ix!();

/**
  | Specifies the quality of the sample
  | rate conversion performed by Oboe.
  | 
  | Higher quality will require more CPU
  | load.
  | 
  | Higher quality conversion will probably
  | be implemented using a sinc based resampler.
  |
  */
#[repr(i32)]
#[derive(Default,Clone)]
pub enum OboeSampleRateConversionQuality {

    /**
      | No conversion by Oboe. Underlying APIs
      | may still do conversion.
      |
      */
    #[default]
    None,

    /**
      | Fastest conversion but may not sound
      | great.
      | 
      | This may be implemented using bilinear
      | interpolation.
      |
      */
    Fastest,
    Low,
    Medium,
    High,

    /**
      | Highest quality conversion, which
      | may be expensive in terms of CPU.
      |
      */
    Best,
}
