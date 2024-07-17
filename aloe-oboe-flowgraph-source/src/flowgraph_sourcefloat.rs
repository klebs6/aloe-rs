crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceFloat.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceFloat.cpp]

/**
  | AudioSource that reads a block of pre-defined
  | float data.
  |
  */
pub struct FlowgraphSourceFloat<'a> {
    base: FlowGraphSourceBuffered<'a>,
}

impl<'a> FlowgraphSourceFloat<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphSourceFloat";
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
            float *outputBuffer = output.getBuffer();
        const int32_t channelCount = output.getSamplesPerFrame();

        const int32_t framesLeft = mSizeInFrames - mFrameIndex;
        const int32_t framesToProcess = std::min(numFrames, framesLeft);
        const int32_t numSamples = framesToProcess * channelCount;

        const float *floatBase = (float *) mData;
        const float *floatData = &floatBase[mFrameIndex * channelCount];
        memcpy(outputBuffer, floatData, numSamples * sizeof(float));
        mFrameIndex += framesToProcess;
        return framesToProcess;
        */
    }
}
