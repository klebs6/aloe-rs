crate::ix!();

#[derive(Default)]
#[no_copy]
pub struct AllPassFilter {
    buffer:       HeapBlock<f32>,
    buffer_size:  i32, // default = 0
    buffer_index: i32, // default = 0
}

impl AllPassFilter {

    pub fn set_size(&mut self, size: i32)  {
        
        if size != self.buffer_size {
            self.buffer_index = 0;
            self.buffer.malloc(size, None);
            self.buffer_size = size;
        }

        self.clear();
    }
    
    pub fn clear(&mut self)  {
        
        self.buffer.clear(self.buffer_size as usize);
    }
    
    pub fn process(&mut self, input: f32) -> f32 {
        
        todo!();
        /*
            const float bufferedValue = buffer [bufferIndex];
                float temp = input + (bufferedValue * 0.5f);
                ALOE_UNDENORMALISE (temp);
                buffer [bufferIndex] = temp;
                bufferIndex = (bufferIndex + 1) % bufferSize;
                return bufferedValue - input;
        */
    }
}
