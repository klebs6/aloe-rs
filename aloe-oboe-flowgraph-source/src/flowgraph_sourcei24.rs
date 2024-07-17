crate::ix!();

pub const kBytesPerI24Packed: i32 = 3;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI24.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphSourceI24.cpp]

/**
  | AudioSource that reads a block of pre-defined
  | 24-bit packed integer data.
  |
  */
pub struct FlowgraphSourceI24<'a> {
    base: FlowGraphSourceBuffered<'a>,
}

impl<'a> FlowgraphSourceI24<'a> {

    pub fn get_name(&mut self) -> &'static str {
        
        "FlowgraphSourceI24"
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
            : FlowGraphSourceBuffered(channelCount)
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

        const uint8_t *byteBase = (uint8_t *) mData;
        const uint8_t *byteData = &byteBase[mFrameIndex * channelCount * kBytesPerI24Packed];

    #if FLOWGRAPH_ANDROID_INTERNAL
        memcpy_to_float_from_p24(floatData, byteData, numSamples);
    #else
        static const float scale = 1. / (float)(1UL << 31);
        for (int i = 0; i < numSamples; i++) {
            // Assemble the data assuming Little Endian format.
            int32_t pad = byteData[2];
            pad <<= 8;
            pad |= byteData[1];
            pad <<= 8;
            pad |= byteData[0];
            pad <<= 8; // Shift to 32 bit data so the sign is correct.
            byteData += kBytesPerI24Packed;
            *floatData++ = pad * scale; // scale to range -1.0 to 1.0
        }
    #endif

        mFrameIndex += framesToProcess;
        return framesToProcess;
        */
    }
}
