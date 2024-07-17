crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AudioVisualiserComponentChannelInfo<'a> {
    owner:       &'a mut AudioVisualiserComponent<'a>,
    levels:      Vec<Range<f32>>,
    value:       Range<f32>,
    next_sample: Atomic<i32>, // default = 0 
    sub_sample:  Atomic<i32>, // default = 0 
}

impl<'a> AudioVisualiserComponentChannelInfo<'a> {

    pub fn new(
        o:           &mut AudioVisualiserComponent,
        buffer_size: i32) -> Self {
    
        todo!();
        /*
        : owner(o),

            setBufferSize (bufferSize);
                clear();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            levels.fill ({});
                value = {};
                subSample = 0;
        */
    }
    
    pub fn push_samples(&mut self, 
        input_samples: *const f32,
        num:           i32)  {
        
        todo!();
        /*
            for (int i = 0; i < num; ++i)
                    pushSample (inputSamples[i]);
        */
    }
    
    pub fn push_sample(&mut self, new_sample: f32)  {
        
        todo!();
        /*
            if (--subSample <= 0)
                {
                    if (++nextSample == levels.size())
                        nextSample = 0;

                    levels.getReference (nextSample) = value;
                    subSample = owner.getSamplesPerBlock();
                    value = Range<float> (newSample, newSample);
                }
                else
                {
                    value = value.getUnionWith (newSample);
                }
        */
    }
    
    pub fn set_buffer_size(&mut self, new_size: i32)  {
        
        todo!();
        /*
            levels.removeRange (newSize, levels.size());
                levels.insertMultiple (-1, {}, newSize - levels.size());

                if (nextSample >= newSize)
                    nextSample = 0;
        */
    }
}
