crate::ix!();

/**
  | Used as a template parameter for
  | AudioData::AudioDataPointer. Indicates
  | an 32-bit integer packed data format.
  |
  */
pub struct AudioDataInt32 {
    data: *mut u32,
}

pub mod int32 {
    pub const bytesPerSample: usize = 4;
    pub const maxValue:       usize = 0x7fffffff;
    pub const resolution:     usize = 1;
    pub const isFloat:        usize = 0;
}

impl AudioDataInt32 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*


            : data (static_cast<uint32*> (d))
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
            return (float) ((1.0 / (1.0 + (double) maxValue)) * (int32) ByteOrder::swapIfBigEndian    (*data));
        */
    }
    
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            return (float) ((1.0 / (1.0 + (double) maxValue)) * (int32) ByteOrder::swapIfLittleEndian (*data));
        */
    }
    
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfBigEndian    ((uint32) (int32) ((double) maxValue * jlimit (-1.0, 1.0, (double) newValue)));
        */
    }
    
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint32) (int32) ((double) maxValue * jlimit (-1.0, 1.0, (double) newValue)));
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int32) ByteOrder::swapIfBigEndian    (*data);
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return (int32) ByteOrder::swapIfLittleEndian (*data);
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfBigEndian    ((uint32) newValue);
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint32) newValue);
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
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut i32)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
