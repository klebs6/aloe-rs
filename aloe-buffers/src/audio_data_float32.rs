crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates an 32-bit
 | float data format
 */
pub struct AudioDataFloat32 {
    data: *mut f32,
}

pub mod float32 {
    pub const bytesPerSample: usize = 4;
    pub const maxValue:       usize = 0x7fffffff;
    pub const resolution:     usize = (1 << 8);
    pub const isFloat:        usize = 1;
}

impl AudioDataFloat32 {

    pub fn new(d: *mut c_void) -> Self {
    
        todo!();
        /*


            : data (static_cast<float*> (d))
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
    
    #[cfg(ALOE_BIG_ENDIAN)]
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            return *data;
        */
    }
    
    #[cfg(ALOE_BIG_ENDIAN)]
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = newValue;
        */
    }
    
    #[cfg(ALOE_BIG_ENDIAN)]
    #[inline] pub fn get_as_floatle(&self) -> f32 {
        
        todo!();
        /*
            union { uint32 asInt; float asFloat; } n; n.asInt = ByteOrder::swap (*(uint32*) data); return n.asFloat;
        */
    }
    
    #[cfg(ALOE_BIG_ENDIAN)]
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            union { uint32 asInt; float asFloat; } n; n.asFloat = newValue; *(uint32*) data = ByteOrder::swap (n.asInt);
        */
    }
    
    ///--------------------------
    #[cfg(not(ALOE_BIG_ENDIAN))]
    #[inline] pub fn get_as_floatle(&self) -> f32 {
        
        todo!();
        /*
            return *data;
        */
    }
    
    #[cfg(not(ALOE_BIG_ENDIAN))]
    #[inline] pub fn set_as_floatle(&mut self, new_value: f32)  {
        
        todo!();
        /*
            *data = newValue;
        */
    }
    
    #[cfg(not(ALOE_BIG_ENDIAN))]
    #[inline] pub fn get_as_floatbe(&self) -> f32 {
        
        todo!();
        /*
            union { uint32 asInt; float asFloat; } n; n.asInt = ByteOrder::swap (*(uint32*) data); return n.asFloat;
        */
    }
    
    #[cfg(not(ALOE_BIG_ENDIAN))]
    #[inline] pub fn set_as_floatbe(&mut self, new_value: f32)  {
        
        todo!();
        /*
            union { uint32 asInt; float asFloat; } n; n.asFloat = newValue; *(uint32*) data = ByteOrder::swap (n.asInt);
        */
    }
    
    #[inline] pub fn get_as_int32le(&self) -> i32 {
        
        todo!();
        /*
            return (int32) roundToInt (jlimit (-1.0, 1.0, (double) getAsFloatLE()) * (double) maxValue);
        */
    }
    
    #[inline] pub fn get_as_int32be(&self) -> i32 {
        
        todo!();
        /*
            return (int32) roundToInt (jlimit (-1.0, 1.0, (double) getAsFloatBE()) * (double) maxValue);
        */
    }
    
    #[inline] pub fn set_as_int32le(&mut self, new_value: i32)  {
        
        todo!();
        /*
            setAsFloatLE ((float) (newValue * (1.0 / (1.0 + (double) maxValue))));
        */
    }
    
    #[inline] pub fn set_as_int32be(&mut self, new_value: i32)  {
        
        todo!();
        /*
            setAsFloatBE ((float) (newValue * (1.0 / (1.0 + (double) maxValue))));
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
            setAsFloatLE (source.getAsFloat());
        */
    }
    
    #[inline] pub fn copy_frombe<SourceType>(&mut self, source: &mut SourceType)  {
    
        todo!();
        /*
            setAsFloatBE (source.getAsFloat());
        */
    }
    
    #[inline] pub fn copy_from_same_type(&mut self, source: &mut AudioDataFloat32)  {
        
        todo!();
        /*
            *data = *source.data;
        */
    }
}
