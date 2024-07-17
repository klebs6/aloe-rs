crate::ix!();

#[derive(Default)]
#[no_copy]
pub struct CombFilter {
    buffer:       HeapBlock<f32>,
    buffer_size:  i32, // default = 0
    buffer_index: i32, // default = 0
    last:         f32, // default = 0.0f
}

impl CombFilter {

    pub fn set_size(&mut self, size: i32)  {
        
        todo!();
        /*
            if (size != bufferSize)
                {
                    bufferIndex = 0;
                    buffer.malloc (size);
                    bufferSize = size;
                }

                clear();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            last = 0;
                buffer.clear ((size_t) bufferSize);
        */
    }
    
    pub fn process(&mut self, 
        input:          f32,
        damp:           f32,
        feedback_level: f32) -> f32 {
        
        todo!();
        /*
            const float output = buffer[bufferIndex];
                last = (output * (1.0f - damp)) + (last * damp);
                ALOE_UNDENORMALISE (last);

                float temp = input + (last * feedbackLevel);
                ALOE_UNDENORMALISE (temp);
                buffer[bufferIndex] = temp;
                bufferIndex = (bufferIndex + 1) % bufferSize;
                return output;
        */
    }
}
