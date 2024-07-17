crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/SourceI32Caller.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/SourceI32Caller.cpp]

/**
  | AudioSource that uses callback to get
  | more data.
  |
  */
pub struct SourceI32Caller<'a> {
    base:              AudioSourceCaller<'a>,
    conversion_buffer: Box<&'a [i32]>,
}

pub const SourceI32CallerScale: f32 = 1.0 / ((1 << 31) as f32);

impl<'a> SourceI32Caller<'a> {

    pub fn new(
        channel_count:       i32,
        frames_per_callback: i32) -> Self {
    
        todo!();
        /*
            : AudioSourceCaller(channelCount, framesPerCallback, sizeof(int32_t)) 
            mConversionBuffer = std::make_unique<int32_t[]>(channelCount * output.getFramesPerBuffer());
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "SourceI32Caller";
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t numBytes = mStream->getBytesPerFrame() * numFrames;
        int32_t bytesRead = mBlockReader.read((uint8_t *) mConversionBuffer.get(), numBytes);
        int32_t framesRead = bytesRead / mStream->getBytesPerFrame();

        float *floatData = output.getBuffer();
        const int32_t *intData = mConversionBuffer.get();
        int32_t numSamples = framesRead * output.getSamplesPerFrame();

    #if FLOWGRAPH_ANDROID_INTERNAL
        memcpy_to_float_from_i32(floatData, shortData, numSamples);
    #else
        for (int i = 0; i < numSamples; i++) {
            *floatData++ = *intData++ * kScale;
        }
    #endif

        return framesRead;
        */
    }
}
