/*!
  | These types can be used as the Endianness
  | template parameter for the
  | 
  | AudioData::AudioDataPointer class.
  |
  */
crate::ix!();
    
/** 
  | Used as a template parameter for
  | AudioData::AudioDataPointer. Indicates that the samples
  | are stored in big-endian order. 
  */
pub struct AudioDataBigEndian {

}

pub mod big_endian {
    pub const isBigEndian: usize = 1;
}

impl AudioDataBigEndian {

    pub fn get_as_float<SampleFormatType>(s: &mut SampleFormatType) -> f32 {

        todo!();
        /*
           return s.getAsFloatBE();
           */
    }

    pub fn set_as_float<SampleFormatType>(
        s:         &mut SampleFormatType,
        new_value: f32
    )  {

        todo!();
        /*
           s.setAsFloatBE (newValue);
           */
    }

    pub fn get_as_int32<SampleFormatType>(s: &mut SampleFormatType) -> i32 {

        todo!();
        /*
           return s.getAsInt32BE();
           */
    }

    pub fn set_as_int32<SampleFormatType>(
        s:         &mut SampleFormatType,
        new_value: i32
    ) {

        todo!();
        /*
           s.setAsInt32BE (newValue);
           */
    }

    pub fn copy_from<SourceType, DestType>(
        dest:   &mut DestType,
        source: &mut SourceType
    ) {

        todo!();
        /*
           dest.copyFromBE (source);
           */
    }
}
