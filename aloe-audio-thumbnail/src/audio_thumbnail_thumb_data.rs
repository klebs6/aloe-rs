crate::ix!();

pub struct AudioThumbnailThumbData {
    data:       Vec<AudioThumbnailMinMaxValue>,
    peak_level: i32, // default = -1
}

impl AudioThumbnailThumbData {

    pub fn new(num_thumb_samples: i32) -> Self {
    
        todo!();
        /*


            ensureSize (numThumbSamples);
        */
    }
    
    #[inline] pub fn get_data(&mut self, thumb_sample_index: i32) -> *mut AudioThumbnailMinMaxValue {
        
        todo!();
        /*
            jassert (thumbSampleIndex < data.size());
                return data.getRawDataPointer() + thumbSampleIndex;
        */
    }
    
    pub fn get_size(&self) -> i32 {
        
        todo!();
        /*
            return data.size();
        */
    }
    
    pub fn get_min_max(&self, 
        start_sample: i32,
        end_sample:   i32,
        result:       &mut AudioThumbnailMinMaxValue)  {
        
        todo!();
        /*
            if (startSample >= 0)
                {
                    endSample = jmin (endSample, data.size() - 1);

                    int8 mx = -128;
                    int8 mn = 127;

                    while (startSample <= endSample)
                    {
                        auto& v = data.getReference (startSample);

                        if (v.getMinValue() < mn)  mn = v.getMinValue();
                        if (v.getMaxValue() > mx)  mx = v.getMaxValue();

                        ++startSample;
                    }

                    if (mn <= mx)
                    {
                        result.set (mn, mx);
                        return;
                    }
                }

                result.set (1, 0);
        */
    }
    
    pub fn write(&mut self, 
        values:      *const AudioThumbnailMinMaxValue,
        start_index: i32,
        num_values:  i32)  {
        
        todo!();
        /*
            resetPeak();

                if (startIndex + numValues > data.size())
                    ensureSize (startIndex + numValues);

                auto* dest = getData (startIndex);

                for (int i = 0; i < numValues; ++i)
                    dest[i] = values[i];
        */
    }
    
    pub fn reset_peak(&mut self)  {
        
        todo!();
        /*
            peakLevel = -1;
        */
    }
    
    pub fn get_peak(&mut self) -> i32 {
        
        todo!();
        /*
            if (peakLevel < 0)
                {
                    for (auto& s : data)
                    {
                        auto peak = s.getPeak();

                        if (peak > peakLevel)
                            peakLevel = peak;
                    }
                }

                return peakLevel;
        */
    }
    
    pub fn ensure_size(&mut self, thumb_samples: i32)  {
        
        todo!();
        /*
            auto extraNeeded = thumbSamples - data.size();

                if (extraNeeded > 0)
                    data.insertMultiple (-1, AudioThumbnailMinMaxValue(), extraNeeded);
        */
    }
}
