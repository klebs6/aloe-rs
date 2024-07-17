crate::ix!();

pub struct AudioThumbnailMinMaxValue {
    values: [i8; 2],
}

impl Default for AudioThumbnailMinMaxValue {
    
    fn default() -> Self {
        todo!();
        /*


            values[0] = 0;
                values[1] = 0
        */
    }
}

impl AudioThumbnailMinMaxValue {

    #[inline] pub fn set(&mut self, 
        new_min: i8,
        new_max: i8)  {
        
        todo!();
        /*
            values[0] = newMin;
                values[1] = newMax;
        */
    }
    
    #[inline] pub fn get_min_value(&self) -> i8 {
        
        todo!();
        /*
            return values[0];
        */
    }
    
    #[inline] pub fn get_max_value(&self) -> i8 {
        
        todo!();
        /*
            return values[1];
        */
    }
    
    #[inline] pub fn set_float(&mut self, new_range: Range<f32>)  {
        
        todo!();
        /*
            // Workaround for an ndk armeabi compiler bug which crashes on signed saturation
               #if ALOE_ANDROID
                Range<float> limitedRange (jlimit (-1.0f, 1.0f, newRange.getStart()),
                                           jlimit (-1.0f, 1.0f, newRange.getEnd()));
                values[0] = (int8) (limitedRange.getStart() * 127.0f);
                values[1] = (int8) (limitedRange.getEnd()   * 127.0f);
               #else
                values[0] = (int8) jlimit (-128, 127, roundToInt (newRange.getStart() * 127.0f));
                values[1] = (int8) jlimit (-128, 127, roundToInt (newRange.getEnd()   * 127.0f));
               #endif

                if (values[0] == values[1])
                {
                    if (values[1] == 127)
                        values[0]--;
                    else
                        values[1]++;
                }
        */
    }
    
    #[inline] pub fn is_non_zero(&self) -> bool {
        
        todo!();
        /*
            return values[1] > values[0];
        */
    }
    
    #[inline] pub fn get_peak(&self) -> i32 {
        
        todo!();
        /*
            return jmax (std::abs ((int) values[0]),
                             std::abs ((int) values[1]));
        */
    }
    
    #[inline] pub fn read<R: Read>(&mut self, input: &mut R)  {
        
        todo!();
        /*
            input.read (values, 2);
        */
    }
    
    #[inline] pub fn write<W: Write>(&mut self, output: &mut W)  {
        
        todo!();
        /*
            output.write (values, 2);
        */
    }
}
