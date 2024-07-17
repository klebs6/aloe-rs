crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI32.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI32.cpp]

pub struct FlowgraphSinkI32<'a> {
    base: FlowGraphSink<'a>,
}

impl<'a> FlowgraphSinkI32<'a> {
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSinkI32";
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
            int32_t *intData = (int32_t *) data;
        const int32_t channelCount = input.getSamplesPerFrame();

        int32_t framesLeft = numFrames;
        while (framesLeft > 0) {
            // Run the graph and pull data through the input port.
            int32_t framesRead = pullData(framesLeft);
            if (framesRead <= 0) {
                break;
            }
            const float *signal = input.getBuffer();
            int32_t numSamples = framesRead * channelCount;
    #if FLOWGRAPH_ANDROID_INTERNAL
            memcpy_to_i32_from_float(intData, signal, numSamples);
            intData += numSamples;
            signal += numSamples;
    #else
            for (int i = 0; i < numSamples; i++) {
                *intData++ = FlowgraphUtilities::clamp32FromFloat(*signal++);
            }
    #endif
            framesLeft -= framesRead;
        }
        return numFrames - framesLeft;
        */
    }
}
