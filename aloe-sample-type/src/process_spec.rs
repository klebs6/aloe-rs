crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_ProcessContext.h]

/**
  | This structure is passed into a DSP algorithm's
  | prepare() method, and contains information
  | about various aspects of the context
  | in which it can expect to be called.
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessSpec {

    /**
      | The sample rate that will be used for
      | the data that is sent to the processor.
      |
      */
    sample_rate: f64,

    /**
      | The maximum number of samples that will
      | be in the blocks sent to process() method.
      |
      */
    maximum_block_size: u32,

    /**
      | The number of channels that the process()
      | method will be expected to handle.
      |
      */
    num_channels: u32,
}
