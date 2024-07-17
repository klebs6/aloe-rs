crate::ix!();

#[no_copy]
pub struct GraphRenderSequenceDelayChannelOp<FloatType: num::Float> {
    buffer:      HeapBlock<FloatType>,
    channel:     i32,
    buffer_size: i32,
    read_index:  i32, // default = 0
    write_index: i32,
}

impl<'a, FloatType: num::Float> GraphRenderSequenceRenderingOp<FloatType> 
for GraphRenderSequenceDelayChannelOp<FloatType> 
{
    fn perform(&mut self, c: &GraphRenderSequenceContext<FloatType>)  {
        
        todo!();
        /*
            auto* data = c.audioBuffers[channel];

                for (int i = c.numSamples; --i >= 0;)
                {
                    buffer[writeIndex] = *data;
                    *data++ = buffer[readIndex];

                    if (++readIndex  >= bufferSize) readIndex = 0;
                    if (++writeIndex >= bufferSize) writeIndex = 0;
                }
        */
    }
}

impl<FloatType: num::Float> GraphRenderSequenceDelayChannelOp<FloatType> {

    pub fn new(
        chan:       i32,
        delay_size: i32) -> Self {
    
        todo!();
        /*


            : channel (chan),
                  bufferSize (delaySize + 1),
                  writeIndex (delaySize)

                buffer.calloc ((size_t) bufferSize);
        */
    }
}
