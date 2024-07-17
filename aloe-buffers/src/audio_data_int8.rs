/*!
  | These types can be used as the SampleFormat
  | template parameter for the
  | AudioData::AudioDataPointer class.
  |
  */

crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates an 8-bit
 | integer packed data format. 
 */
pub struct AudioDataInt8 {
    data: *mut i8,
}

pub mod int8 {
    pub const bytesPerSample: usize = 1;
    pub const maxValue:       usize = 0x7f;
    pub const resolution:     usize = (1 << 24);
    pub const isFloat:        usize = 0;
}

impl AudioDataInt8 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*


            : data (static_cast<int8*> (d))
        */
    }
    
    #[inline] pub fn advance(&mut self)  {
        
        todo!();
        /*
            ++data;
        */
    }
    
    #[inline] pub fn skip(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            data += numSamples;
        */
    }
    
    #[inline] pub fn get_as_floatle(&self) -> f32 {
        
        todo!();
        /*
            return (float) (*data * (1.0 / (1.0 + (double) maxValue)));
        */
    }
    
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            return getAsFloatLE();
        */
    }
    
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = (int8) jlimit ((int) -maxValue, (int) maxValue, roundToInt (newValue * (1.0 + (double) maxValue)));
        */
    }
    
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            setAsFloatLE (newValue);
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int) (*((uint8*) data) << 24);
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return getAsInt32LE();
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = (int8) (newValue >> 24);
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            setAsInt32LE (newValue);
        */
    }
    
    #[inline] pub fn clear(&mut self)  {
        
        todo!();
        /*
            *data = 0;
        */
    }
    
    #[inline] pub fn clear_multiple(&mut self, num: i32)  {
        
        todo!();
        /*
            zeromem (data, (size_t) (num * bytesPerSample)) ;
        */
    }
    
    #[inline] pub fn copy_fromle<SourceType>(&mut self, source: &mut SourceType)  {
    
        todo!();
        /*
            setAsInt32LE (source.getAsInt32());
        */
    }
    
    #[inline] pub fn copy_frombe<SourceType>(&mut self, source: &mut SourceType)  {
    
        todo!();
        /*
            setAsInt32BE (source.getAsInt32());
        */
    }
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataInt8)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
