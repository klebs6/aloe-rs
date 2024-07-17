crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates that the samples
 | are interleaved with a number of other
 | channels. 
 */
pub struct AudioDataInterleaved {
    num_interleaved_channels: i32, // default = 1
}

pub mod interleaved {
    pub const isInterleavedType: usize = 1;
}

impl AudioDataInterleaved {

    pub fn new_interleaved(num_interleaved_chans: i32) -> Self {
    
        todo!();
        /*
        : num_interleaved_channels(numInterleavedChans),

        
        */
    }
    
    #[inline] pub fn copy_from(&mut self, other: &AudioDataInterleaved)  {
        
        todo!();
        /*
            numInterleavedChannels = other.numInterleavedChannels;
        */
    }
    
    #[inline] pub fn advance_data<SampleFormatType>(
        &mut self, 
        s: &mut SampleFormatType
    )
    {
        todo!();

        /*
            s.skip (numInterleavedChannels);
        */
    }
    
    #[inline] pub fn advance_data_by<SampleFormatType>(
        &mut self, 
        s:           &mut SampleFormatType,
        num_samples: i32)  {
    
        todo!();
        /*
            s.skip (numInterleavedChannels * numSamples);
        */
    }
    
    #[inline] pub fn clear<SampleFormatType>(&mut self, 
        s:           &mut SampleFormatType,
        num_samples: i32)  {
    
        todo!();
        /*
            while (--numSamples >= 0) { s.clear(); s.skip (numInterleavedChannels); }
        */
    }
    
    #[inline] pub fn get_num_bytes_between_samples<SampleFormatType>(&self, _0: &SampleFormatType) -> i32 {
    
        todo!();
        /*
            return numInterleavedChannels * SampleFormatType::bytesPerSample;
        */
    }
}
