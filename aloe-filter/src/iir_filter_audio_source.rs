crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_IIRFilterAudioSource.h]

/**
  | An AudioSource that performs an IIR
  | filter on another source.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct IIRFilterAudioSource {
    input:       OptionalScopedPointer<dyn AudioSource>,
    iir_filters: Vec<Box<IIRFilter>>,
}

impl AudioSource for IIRFilterAudioSource {}

impl PrepareToPlayAudioSource for IIRFilterAudioSource {

    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64

    )  {
        
        todo!();
        /*
            input->prepareToPlay (samplesPerBlockExpected, sampleRate);

        for (int i = iirFilters.size(); --i >= 0;)
            iirFilters.getUnchecked(i)->reset();
        */
    }
}

impl ReleaseResources for IIRFilterAudioSource {
    
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            input->releaseResources();
        */
    }
}

impl GetNextAudioBlock for IIRFilterAudioSource {
    
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            input->getNextAudioBlock (bufferToFill);

        const int numChannels = bufferToFill.buffer->getNumChannels();

        while (numChannels > iirFilters.size())
            iirFilters.add (new IIRFilter (*iirFilters.getUnchecked (0)));

        for (int i = 0; i < numChannels; ++i)
            iirFilters.getUnchecked(i)
                ->processSamples (bufferToFill.buffer->getWritePointer (i, bufferToFill.startSample),
                                  bufferToFill.numSamples);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_IIRFilterAudioSource.cpp]
impl IIRFilterAudioSource {

    /**
      | Creates a IIRFilterAudioSource for
      | a given input source.
      | 
      | -----------
      | @param inputSource
      | 
      | the input source to read from - this must
      | not be null
      | ----------
      | @param deleteInputWhenDeleted
      | 
      | if true, the input source will be deleted
      | when this object is deleted
      |
      */
    pub fn new(
        input_source:              *mut dyn AudioSource,
        delete_input_when_deleted: bool) -> Self {
    
        todo!();
        /*
        : input(inputSource, deleteInputWhenDeleted),

            jassert (inputSource != nullptr);

        for (int i = 2; --i >= 0;)
            iirFilters.add (new IIRFilter());
        */
    }
    
    /**
      | Changes the filter to use the same parameters
      | as the one being passed in.
      |
      */
    pub fn set_coefficients<NumericType>(&mut self, new_coefficients: &IIRCoefficients<NumericType>)  {
        
        todo!();
        /*
            for (int i = iirFilters.size(); --i >= 0;)
            iirFilters.getUnchecked(i)->setCoefficients (newCoefficients);
        */
    }
    
    /**
      | Calls IIRFilter::makeInactive()
      | on all the filters being used internally.
      |
      */
    pub fn make_inactive(&mut self)  {
        
        todo!();
        /*
            for (int i = iirFilters.size(); --i >= 0;)
            iirFilters.getUnchecked(i)->makeInactive();
        */
    }
}
