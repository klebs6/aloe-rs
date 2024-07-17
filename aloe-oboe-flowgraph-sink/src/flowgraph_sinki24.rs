crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI24.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSinkI24.cpp]

/**
  | AudioSink that lets you read data as
  | packed 24-bit signed integers.
  | 
  | The sample size is 3 bytes.
  |
  */
pub struct FlowgraphSinkI24<'a> {
    base: FlowGraphSink<'a>,
}

impl<'a> FlowgraphSinkI24<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSinkI24";
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
            uint8_t *byteData = (uint8_t *) data;
        const int32_t channelCount = input.getSamplesPerFrame();

        int32_t framesLeft = numFrames;
        while (framesLeft > 0) {
            // Run the graph and pull data through the input port.
            int32_t framesRead = pullData(framesLeft);
            if (framesRead <= 0) {
                break;
            }
            const float *floatData = input.getBuffer();
            int32_t numSamples = framesRead * channelCount;
    #if FLOWGRAPH_ANDROID_INTERNAL
            memcpy_to_p24_from_float(byteData, floatData, numSamples);
            static const int kBytesPerI24Packed = 3;
            byteData += numSamples * kBytesPerI24Packed;
            floatData += numSamples;
    #else
            const int32_t kI24PackedMax = 0x007FFFFF;
            const int32_t kI24PackedMin = 0xFF800000;
            for (int i = 0; i < numSamples; i++) {
                int32_t n = (int32_t) (*floatData++ * 0x00800000);
                n = std::min(kI24PackedMax, std::max(kI24PackedMin, n)); // clip
                // Write as a packed 24-bit integer in Little Endian format.
                *byteData++ = (uint8_t) n;
                *byteData++ = (uint8_t) (n >> 8);
                *byteData++ = (uint8_t) (n >> 16);
            }
    #endif
            framesLeft -= framesRead;
        }
        return numFrames - framesLeft;
        */
    }
}
