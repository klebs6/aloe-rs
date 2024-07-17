crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates that the samples
 | are stored in little-endian order. 
 */
pub struct AudioDataLittleEndian {

}

pub mod little_endian {
    pub const isBigEndian: usize = 0;
}

impl AudioDataLittleEndian {

    pub fn get_as_float<SampleFormatType>(s: &mut SampleFormatType) -> f32 {
    
        todo!();
        /*
            return s.getAsFloatLE();
        */
    }
    
    pub fn set_as_float<SampleFormatType>(
        s:         &mut SampleFormatType,
        new_value: f32)  {
    
        todo!();
        /*
            s.setAsFloatLE (newValue);
        */
    }
    
    pub fn get_as_int32<SampleFormatType>(s: &mut SampleFormatType) -> i32 {
    
        todo!();
        /*
            return s.getAsInt32LE();
        */
    }
    
    pub fn set_as_int32<SampleFormatType>(
        s:         &mut SampleFormatType,
        new_value: i32)  {
    
        todo!();
        /*
            s.setAsInt32LE (newValue);
        */
    }
    
    pub fn copy_from<SourceType, DestType>(
        dest:   &mut DestType,
        source: &mut SourceType)  {
    
        todo!();
        /*
            dest.copyFromLE (source);
        */
    }
}

