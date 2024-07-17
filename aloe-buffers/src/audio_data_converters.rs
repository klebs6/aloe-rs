crate::ix!();

pub enum AudioDataConvertersDataFormat
{
    int16LE,
    int16BE,
    int24LE,
    int24BE,
    int32LE,
    int32BE,
    float32LE,
    float32BE,
}

/**
  | A set of routines to convert buffers of 32-bit
  | floating point data to and from various
  | integer formats.
  |
  | Note that these functions are deprecated - the
  | AudioData class provides a much more flexible
  | set of conversion classes now.
  |
  | @tags{Audio}
  */
#[no_copy]
pub struct AudioDataConverters {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioDataConverters.cpp]
impl AudioDataConverters {

    pub fn convert_float_to_int16le(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(2);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                *unalignedPointerCast<uint16*> (intData) = ByteOrder::swapIfBigEndian ((uint16) (short) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                *unalignedPointerCast<uint16*> (intData) = ByteOrder::swapIfBigEndian ((uint16) (short) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
            }
        }
        */
    }
    
    pub fn convert_float_to_int16be(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(2);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                *unalignedPointerCast<uint16*> (intData) = ByteOrder::swapIfLittleEndian ((uint16) (short) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                *unalignedPointerCast<uint16*> (intData) = ByteOrder::swapIfLittleEndian ((uint16) (short) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
            }
        }
        */
    }
    
    pub fn convert_float_to_int24le(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(3);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fffff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                ByteOrder::littleEndian24BitToChars (roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])), intData);
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                ByteOrder::littleEndian24BitToChars (roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])), intData);
            }
        }
        */
    }
    
    pub fn convert_float_to_int24be(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(3);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fffff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                ByteOrder::bigEndian24BitToChars (roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])), intData);
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                ByteOrder::bigEndian24BitToChars (roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])), intData);
            }
        }
        */
    }
    
    pub fn convert_float_to_int32le(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fffffff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                *unalignedPointerCast<uint32*> (intData) = ByteOrder::swapIfBigEndian ((uint32) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                *unalignedPointerCast<uint32*> (intData) = ByteOrder::swapIfBigEndian ((uint32) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
            }
        }
        */
    }
    
    pub fn convert_float_to_int32be(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            auto maxVal = (double) 0x7fffffff;
        auto intData = static_cast<char*> (dest);

        if (dest != (void*) source || destBytesPerSample <= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                *unalignedPointerCast<uint32*> (intData) = ByteOrder::swapIfLittleEndian ((uint32) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
                intData += destBytesPerSample;
            }
        }
        else
        {
            intData += destBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= destBytesPerSample;
                *unalignedPointerCast<uint32*> (intData) = ByteOrder::swapIfLittleEndian ((uint32) roundToInt (jlimit (-maxVal, maxVal, maxVal * source[i])));
            }
        }
        */
    }
    
    pub fn convert_float_to_float32le(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            jassert (dest != (void*) source || destBytesPerSample <= 4); // This op can't be performed on in-place data!

        char* d = static_cast<char*> (dest);

        for (int i = 0; i < numSamples; ++i)
        {
            *unalignedPointerCast<float*> (d) = source[i];

           #if ALOE_BIG_ENDIAN
            *unalignedPointerCast<uint32*> (d) = ByteOrder::swap (*unalignedPointerCast<uint32*> (d));
           #endif

            d += destBytesPerSample;
        }
        */
    }
    
    pub fn convert_float_to_float32be(&mut self, 
        source:                *const f32,
        dest:                  *mut c_void,
        num_samples:           i32,
        dest_bytes_per_sample: Option<i32>)  {

        let dest_bytes_per_sample: i32 = dest_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            jassert (dest != (void*) source || destBytesPerSample <= 4); // This op can't be performed on in-place data!

        auto d = static_cast<char*> (dest);

        for (int i = 0; i < numSamples; ++i)
        {
            *unalignedPointerCast<float*> (d) = source[i];

           #if ALOE_LITTLE_ENDIAN
            *unalignedPointerCast<uint32*> (d) = ByteOrder::swap (*unalignedPointerCast<uint32*> (d));
           #endif

            d += destBytesPerSample;
        }
        */
    }
    
    pub fn convert_int_16le_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(2);
        
        todo!();
        /*
            const float scale = 1.0f / 0x7fff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (short) ByteOrder::swapIfBigEndian (*unalignedPointerCast<const uint16*> (intData));
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (short) ByteOrder::swapIfBigEndian (*unalignedPointerCast<const uint16*> (intData));
            }
        }
        */
    }
    
    pub fn convert_int_16be_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(2);
        
        todo!();
        /*
            const float scale = 1.0f / 0x7fff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (short) ByteOrder::swapIfLittleEndian (*unalignedPointerCast<const uint16*> (intData));
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (short) ByteOrder::swapIfLittleEndian (*unalignedPointerCast<const uint16*> (intData));
            }
        }
        */
    }
    
    pub fn convert_int_24le_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(3);
        
        todo!();
        /*
            const float scale = 1.0f / 0x7fffff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (short) ByteOrder::littleEndian24Bit (intData);
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (short) ByteOrder::littleEndian24Bit (intData);
            }
        }
        */
    }
    
    pub fn convert_int_24be_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(3);
        
        todo!();
        /*
            const float scale = 1.0f / 0x7fffff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (short) ByteOrder::bigEndian24Bit (intData);
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (short) ByteOrder::bigEndian24Bit (intData);
            }
        }
        */
    }
    
    pub fn convert_int_32le_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            const float scale = 1.0f / (float) 0x7fffffff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (float) ByteOrder::swapIfBigEndian (*unalignedPointerCast<const uint32*> (intData));
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (float) ByteOrder::swapIfBigEndian (*unalignedPointerCast<const uint32*> (intData));
            }
        }
        */
    }
    
    pub fn convert_int_32be_to_float(&mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            const float scale = 1.0f / (float) 0x7fffffff;
        auto intData = static_cast<const char*> (source);

        if (source != (void*) dest || srcBytesPerSample >= 4)
        {
            for (int i = 0; i < numSamples; ++i)
            {
                dest[i] = scale * (float) ByteOrder::swapIfLittleEndian (*unalignedPointerCast<const uint32*> (intData));
                intData += srcBytesPerSample;
            }
        }
        else
        {
            intData += srcBytesPerSample * numSamples;

            for (int i = numSamples; --i >= 0;)
            {
                intData -= srcBytesPerSample;
                dest[i] = scale * (float) ByteOrder::swapIfLittleEndian (*unalignedPointerCast<const uint32*> (intData));
            }
        }
        */
    }
    
    pub fn convert_float_32le_to_float(
        &mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>)  {

        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            auto s = static_cast<const char*> (source);

        for (int i = 0; i < numSamples; ++i)
        {
            dest[i] = *unalignedPointerCast<const float*> (s);

           #if ALOE_BIG_ENDIAN
            auto d = unalignedPointerCast<uint32*> (dest + i);
            *d = ByteOrder::swap (*d);
           #endif

            s += srcBytesPerSample;
        }
        */
    }
    
    pub fn convert_float_32be_to_float(
        &mut self, 
        source:               *const c_void,
        dest:                 *mut f32,
        num_samples:          i32,
        src_bytes_per_sample: Option<i32>
    )
    {
        let src_bytes_per_sample: i32 = src_bytes_per_sample.unwrap_or(4);
        
        todo!();
        /*
            auto s = static_cast<const char*> (source);

        for (int i = 0; i < numSamples; ++i)
        {
            dest[i] = *unalignedPointerCast<const float*> (s);

           #if ALOE_LITTLE_ENDIAN
            auto d = unalignedPointerCast<uint32*> (dest + i);
            *d = ByteOrder::swap (*d);
           #endif

            s += srcBytesPerSample;
        }
        */
    }
    
    pub fn convert_float_to_format(
        &mut self, 
        dest_format: audio_data_converters::AudioDataConvertersDataFormat,
        source:      *const f32,
        dest:        *mut c_void,
        num_samples: i32
    )  {
        
        todo!();
        /*
            switch (destFormat)
        {
            case int16LE:       convertFloatToInt16LE   (source, dest, numSamples); break;
            case int16BE:       convertFloatToInt16BE   (source, dest, numSamples); break;
            case int24LE:       convertFloatToInt24LE   (source, dest, numSamples); break;
            case int24BE:       convertFloatToInt24BE   (source, dest, numSamples); break;
            case int32LE:       convertFloatToInt32LE   (source, dest, numSamples); break;
            case int32BE:       convertFloatToInt32BE   (source, dest, numSamples); break;
            case float32LE:     convertFloatToFloat32LE (source, dest, numSamples); break;
            case float32BE:     convertFloatToFloat32BE (source, dest, numSamples); break;
            default:            jassertfalse; break;
        }
        */
    }
    
    pub fn convert_format_to_float(
        &mut self, 
        source_format: audio_data_converters::AudioDataConvertersDataFormat,
        source:        *const c_void,
        dest:          *mut f32,
        num_samples:   i32
    ) {
        
        todo!();
        /*
            switch (sourceFormat)
        {
            case int16LE:       convertInt16LEToFloat   (source, dest, numSamples); break;
            case int16BE:       convertInt16BEToFloat   (source, dest, numSamples); break;
            case int24LE:       convertInt24LEToFloat   (source, dest, numSamples); break;
            case int24BE:       convertInt24BEToFloat   (source, dest, numSamples); break;
            case int32LE:       convertInt32LEToFloat   (source, dest, numSamples); break;
            case int32BE:       convertInt32BEToFloat   (source, dest, numSamples); break;
            case float32LE:     convertFloat32LEToFloat (source, dest, numSamples); break;
            case float32BE:     convertFloat32BEToFloat (source, dest, numSamples); break;
            default:            jassertfalse; break;
        }
        */
    }
    
    pub fn interleave_samples(
        &mut self, 
        source:       *const *const f32,
        dest:         *mut f32,
        num_samples:  i32,
        num_channels: i32
    )
    {
        todo!();
        /*
            for (int chan = 0; chan < numChannels; ++chan)
        {
            auto i = chan;
            auto src = source [chan];

            for (int j = 0; j < numSamples; ++j)
            {
                dest [i] = src [j];
                i += numChannels;
            }
        }
        */
    }
    
    pub fn deinterleave_samples(&mut self, 
        source:       *const f32,
        dest:         *mut *mut f32,
        num_samples:  i32,
        num_channels: i32)  {
        
        todo!();
        /*
            for (int chan = 0; chan < numChannels; ++chan)
        {
            auto i = chan;
            auto dst = dest [chan];

            for (int j = 0; j < numSamples; ++j)
            {
                dst [j] = source [i];
                i += numChannels;
            }
        }
        */
    }
}
