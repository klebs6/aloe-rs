crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates an 16-bit integer
 | packed data format. 
 */
pub struct AudioDataInt16 {
    data: *mut u16,
}

pub mod int16 {
    pub const bytesPerSample: usize = 2;
    pub const maxValue:       usize = 0x7fff;
    pub const resolution:     usize = (1 << 16);
    pub const isFloat:        usize = 0;
}

impl AudioDataInt16 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*
            : data (static_cast<uint16*> (d))
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
            return (float) ((1.0 / (1.0 + (double) maxValue)) * (int16) ByteOrder::swapIfBigEndian    (*data));
        */
    }
    
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            return (float) ((1.0 / (1.0 + (double) maxValue)) * (int16) ByteOrder::swapIfLittleEndian (*data));
        */
    }
    
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfBigEndian    ((uint16) jlimit ((int) -maxValue, (int) maxValue, roundToInt (newValue * (1.0 + (double) maxValue))));
        */
    }
    
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint16) jlimit ((int) -maxValue, (int) maxValue, roundToInt (newValue * (1.0 + (double) maxValue))));
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int32) (ByteOrder::swapIfBigEndian    ((uint16) *data) << 16);
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return (int32) (ByteOrder::swapIfLittleEndian ((uint16) *data) << 16);
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfBigEndian    ((uint16) (newValue >> 16));
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint16) (newValue >> 16));
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
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataInt16)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
