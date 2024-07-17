crate::ix!();

/**
  | Contains configuration information
  | for a convolution with a fixed latency.
  |
  */
pub struct ConvolutionLatency { 
    latency_in_samples: i32,
}

/**
  | Contains configuration information
  | for a non-uniform convolution.
  |
  */
pub struct ConvolutionNonUniform { 
    head_size_in_samples: i32,
}

pub enum ConvolutionStereo    { no, yes }
pub enum ConvolutionTrim      { no, yes }
pub enum ConvolutionNormalise { no, yes }
