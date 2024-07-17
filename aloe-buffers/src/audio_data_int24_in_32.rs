crate::ix!();

/**
  | A 32-bit integer type, of which only
  | the bottom 24 bits are used.
  |
  */
pub struct AudioDataInt24in32 {
    base: AudioDataInt32,
}

pub mod int24in32 {
    pub const bytesPerSample: usize = 4;
    pub const maxValue:       usize = 0x7fffff;
    pub const resolution:     usize = (1 << 8);
    pub const isFloat:        usize = 0;
}

impl AudioDataInt24in32 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*
        : int32(d),

        
        */
    }
    
    #[inline] pub fn get_as_floatle(&self) -> f32 {
        
        todo!();
        /*
            return (float) ((1.0 / (1.0 + (double) maxValue)) * (int32) ByteOrder::swapIfBigEndian (*data));
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
            *data = ByteOrder::swapIfBigEndian    ((uint32) ((double) maxValue * jlimit (-1.0, 1.0, (double) newValue)));
        */
    }
    
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint32) ((double) maxValue * jlimit (-1.0, 1.0, (double) newValue)));
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int32) ByteOrder::swapIfBigEndian    (*data) << 8;
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return (int32) ByteOrder::swapIfLittleEndian (*data) << 8;
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfBigEndian    ((uint32) newValue >> 8);
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            *data = ByteOrder::swapIfLittleEndian ((uint32) newValue >> 8);
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
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataInt24in32)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
