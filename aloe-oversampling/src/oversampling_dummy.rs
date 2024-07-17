crate::ix!();

/**
  | Dummy oversampling stage class which
  | simply copies and pastes the input signal,
  | which could be equivalent to a "one time"
  | oversampling processing.
  |
  */
#[no_copy]
#[leak_detector]
pub struct OversamplingDummy<SampleType> {
    base: OversamplingStage<SampleType>,
}

impl<SampleType: SampleTypeInterface> HasParentType for OversamplingDummy<SampleType> {
    type Type = OversamplingStage<SampleType>;
}

impl<SampleType: SampleTypeInterface> OversamplingDummy<SampleType> {

    pub fn new(num_chans: usize) -> Self {
    
        todo!();
        /*
        : parent_type(numChans, 1),

        
        */
    }
    
    pub fn get_latency_in_samples(&self) -> SampleType {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn process_samples_up(&mut self, input_block: &AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (inputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (inputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            for (size_t channel = 0; channel < inputBlock.getNumChannels(); ++channel)
                ParentType::buffer.copyFrom (static_cast<int> (channel), 0,
                    inputBlock.getChannelPointer (channel), static_cast<int> (inputBlock.getNumSamples()));
        */
    }
    
    pub fn process_samples_down(&mut self, output_block: &mut AudioBlock<SampleType>)  {
        
        todo!();
        /*
            jassert (outputBlock.getNumChannels() <= static_cast<size_t> (ParentType::buffer.getNumChannels()));
            jassert (outputBlock.getNumSamples() * ParentType::factor <= static_cast<size_t> (ParentType::buffer.getNumSamples()));

            outputBlock.copyFrom (ParentType::getProcessedSamples (outputBlock.getNumSamples()));
        */
    }
}

