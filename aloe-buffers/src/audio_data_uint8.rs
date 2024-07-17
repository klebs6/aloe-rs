crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates an 8-bit unsigned
 | integer packed data format. 
 */
pub struct AudioDataUInt8 {
    data: *mut u8,
}

pub mod uint8 {
    use super::*;
    pub const bytesPerSample: usize = 1;
    pub const maxValue:       usize = 0x7f;
    pub const resolution:     usize = (1 << 24);
    pub const isFloat:        usize = 0;
}

impl AudioDataUInt8 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*


            : data (static_cast<uint8*> (d))
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
            return (float) ((*data - 128) * (1.0 / (1.0 + (double) maxValue)));
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
            *data = (uint8) jlimit (0, 255, 128 + roundToInt (newValue * (1.0 + (double) maxValue)));
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
            return (int) (((uint8) (*data - 128)) << 24);
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
            *data = (uint8) (128 + (newValue >> 24));
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
            *data = 128;
        */
    }
    
    #[inline] pub fn clear_multiple(&mut self, num: i32)  {
        
        todo!();
        /*
            memset (data, 128, (size_t) num) ;
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
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataUInt8)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
