crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI16.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI16.cpp]

/**
  | AudioSource that reads a block of pre-defined
  | 16-bit integer data.
  |
  */
pub struct FlowgraphSourceI16<'a> {
    base: FlowGraphSourceBuffered<'a>,
}

impl<'a> FlowgraphSourceI16<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSourceI16";
        */
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
        int32_t channelCount = output.getSamplesPerFrame();

        int32_t framesLeft = mSizeInFrames - mFrameIndex;
        int32_t framesToProcess = std::min(numFrames, framesLeft);
        int32_t numSamples = framesToProcess * channelCount;

        const int16_t *shortBase = static_cast<const int16_t *>(mData);
        const int16_t *shortData = &shortBase[mFrameIndex * channelCount];

    #if FLOWGRAPH_ANDROID_INTERNAL
        memcpy_to_float_from_i16(floatData, shortData, numSamples);
    #else
        for (int i = 0; i < numSamples; i++) {
            *floatData++ = *shortData++ * (1.0f / 32768);
        }
    #endif

        mFrameIndex += framesToProcess;
        return framesToProcess;
        */
    }
}
