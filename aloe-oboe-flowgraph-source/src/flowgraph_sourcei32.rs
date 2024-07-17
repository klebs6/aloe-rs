crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI32.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI32.cpp]

pub struct FlowgraphSourceI32<'a> {
    base: FlowGraphSourceBuffered<'a>,
}

impl<'a> FlowgraphSourceI32<'a> {

    pub const kScale: f32 = 1.0 / ((1 << 31) as f32);

    pub fn get_name(&mut self) -> &'static str {

        "FlowgraphSourceI32"
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_source_buffered(channelCount),

        
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            float *floatData = output.getBuffer();
        const int32_t channelCount = output.getSamplesPerFrame();

        const int32_t framesLeft = mSizeInFrames - mFrameIndex;
        const int32_t framesToProcess = std::min(numFrames, framesLeft);
        const int32_t numSamples = framesToProcess * channelCount;

        const int32_t *intBase = static_cast<const int32_t *>(mData);
        const int32_t *intData = &intBase[mFrameIndex * channelCount];

    #if FLOWGRAPH_ANDROID_INTERNAL
        memcpy_to_float_from_i32(floatData, intData, numSamples);
    #else
        for (int i = 0; i < numSamples; i++) {
            *floatData++ = *intData++ * kScale;
        }
    #endif

        mFrameIndex += framesToProcess;
        return framesToProcess;
        */
    }
}
