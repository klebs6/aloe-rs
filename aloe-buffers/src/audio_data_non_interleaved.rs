/*!
  | These types can be used as the
  | 
  | InterleavingType template parameter
  | for the
  | 
  | AudioData::AudioDataPointer class.
  |
  */

crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates that the samples
 | are stored contiguously.
 */
#[derive(Default)]
pub struct AudioDataNonInterleaved {

}

pub mod non_interleaved {
    pub const isInterleavedType:      usize = 0;
    pub const numInterleavedChannels: usize = 1;
}

impl AudioDataNonInterleaved {

    pub fn new(int: AudioDataConst) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    #[inline] pub fn copy_from(&mut self, _0: &AudioDataNonInterleaved)  {
        
        todo!();
        /*
        
        */
    }
    
    #[inline] pub fn advance_data<SampleFormatType>(&mut self, s: &mut SampleFormatType)  {
    
        todo!();
        /*
            s.advance();
        */
    }
    
    #[inline] pub fn advance_data_by<SampleFormatType>(
        &mut self, 
        s:           &mut SampleFormatType,
        num_samples: i32)  {
    
        todo!();
        /*
            s.skip (numSamples);
        */
    }
    
    #[inline] pub fn clear<SampleFormatType>(
        &mut self, 
        s:           &mut SampleFormatType,
        num_samples: i32)  {
    
        todo!();
        /*
            s.clearMultiple (numSamples);
        */
    }
    
    pub fn get_num_bytes_between_samples<SampleFormatType>(_0: &SampleFormatType) -> i32 {
    
        todo!();
        /*
            return SampleFormatType::bytesPerSample;
        */
    }
}
