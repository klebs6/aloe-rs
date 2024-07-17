crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI16.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI16.cpp]

/**
  | AudioSink that lets you read data as
  | 16-bit signed integers.
  |
  */
pub struct FlowgraphSinkI16<'a> {
    base: FlowGraphSink<'a>,
}

impl<'a> FlowgraphSinkI16<'a> {
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSinkI16";
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
            int16_t *shortData = (int16_t *) data;
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
            memcpy_to_i16_from_float(shortData, signal, numSamples);
            shortData += numSamples;
            signal += numSamples;
    #else
            for (int i = 0; i < numSamples; i++) {
                int32_t n = (int32_t) (*signal++ * 32768.0f);
                *shortData++ = std::min(INT16_MAX, std::max(INT16_MIN, n)); // clip
            }
    #endif
            framesLeft -= framesRead;
        }
        return numFrames - framesLeft;
        */
    }
}
