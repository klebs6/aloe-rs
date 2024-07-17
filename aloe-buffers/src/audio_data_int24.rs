crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates an 24-bit integer
 | packed data format. 
 */
pub struct AudioDataInt24 {
    data: *mut u8,
}

pub mod int24 {
    pub const bytesPerSample: usize = 3;
    pub const maxValue:       usize = 0x7fffff;
    pub const resolution:     usize = (1 << 8);
    pub const isFloat:        usize = 0;
}

impl AudioDataInt24 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*
            : data (static_cast<char*> (d))
        */
    }
    
    #[inline] pub fn advance(&mut self)  {
        
        todo!();
        /*
            data += 3;
        */
    }
    
    #[inline] pub fn skip(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            data += 3 * numSamples;
        */
    }
    
    #[inline] pub fn get_as_floatle(&self) -> f32 {
        
        todo!();
        /*
            return (float) (ByteOrder::littleEndian24Bit (data) * (1.0 / (1.0 + (double) maxValue)));
        */
    }
    
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            return (float) (ByteOrder::bigEndian24Bit    (data) * (1.0 / (1.0 + (double) maxValue)));
        */
    }
    
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            ByteOrder::littleEndian24BitToChars (jlimit ((int) -maxValue, (int) maxValue, roundToInt (newValue * (1.0 + (double) maxValue))), data);
        */
    }
    
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            ByteOrder::bigEndian24BitToChars (jlimit    ((int) -maxValue, (int) maxValue, roundToInt (newValue * (1.0 + (double) maxValue))), data);
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int32) (((unsigned int) ByteOrder::littleEndian24Bit (data)) << 8);
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return (int32) (((unsigned int) ByteOrder::bigEndian24Bit    (data)) << 8);
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            ByteOrder::littleEndian24BitToChars (newValue >> 8, data);
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            ByteOrder::bigEndian24BitToChars    (newValue >> 8, data);
        */
    }
    
    #[inline] pub fn clear(&mut self)  {
        
        todo!();
        /*
            data[0] = 0; data[1] = 0; data[2] = 0;
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
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataInt24)  {
        
        todo!();
        /*
            data[0] = source.data[0]; data[1] = source.data[1]; data[2] = source.data[2];
        */
    }
}
