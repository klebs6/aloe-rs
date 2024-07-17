crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkFloat.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkFloat.cpp]

/**
  | AudioSink that lets you read data as
  | 32-bit floats.
  |
  */
pub struct FlowgraphSinkFloat<'a> {
    base: FlowGraphSink<'a>,
}

impl<'a> FlowgraphSinkFloat<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSinkFloat";
        */
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_sink(channelCount),
        */
    }
    
    pub fn read(&mut self, 
        data:       *mut c_void,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            // printf("FlowgraphSinkFloat::read(,,%d)\n", numFrames);
        float *floatData = (float *) data;
        const int32_t channelCount = input.getSamplesPerFrame();

        int32_t framesLeft = numFrames;
        while (framesLeft > 0) {
            // Run the graph and pull data through the input port.
            int32_t framesPulled = pullData(framesLeft);
            // printf("FlowgraphSinkFloat::read: framesLeft = %d, framesPulled = %d\n", framesLeft, framesPulled);
            if (framesPulled <= 0) {
                break;
            }
            const float *signal = input.getBuffer();
            int32_t numSamples = framesPulled * channelCount;
            memcpy(floatData, signal, numSamples * sizeof(float));
            floatData += numSamples;
            framesLeft -= framesPulled;
        }
        // printf("FlowgraphSinkFloat returning %d\n", numFrames - framesLeft);
        return numFrames - framesLeft;
        */
    }
}
