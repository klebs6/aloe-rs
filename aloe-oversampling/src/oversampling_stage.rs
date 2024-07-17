crate::ix!();

/**
  | Abstract class for the provided oversampling
  | stages used internally in the Oversampling
  | class.
  |
  */
pub struct OversamplingStage<SampleType> {
    buffer:       AudioBuffer<SampleType>,
    num_channels: usize,
    factor:       usize,
}

impl<SampleType: SampleTypeInterface> OversamplingStage<SampleType> {

    pub fn new(
        num_chans:  usize,
        new_factor: usize) -> Self {
    
        todo!();
        /*
        : num_channels(numChans),
        : factor(newFactor),

        
        */
    }
    
    pub fn get_processed_samples(&mut self, num_samples: usize) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            return AudioBlock<SampleType> (buffer).getSubBlock (0, numSamples);
        */
    }
    
    pub fn init_processing(&mut self, maximum_number_of_samples_before_oversampling: usize)  {
        
        todo!();
        /*
            buffer.setSize (static_cast<int> (numChannels),
                                    static_cast<int> (maximumNumberOfSamplesBeforeOversampling * factor),
                                    false, false, true);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            buffer.clear();
        */
    }
}
