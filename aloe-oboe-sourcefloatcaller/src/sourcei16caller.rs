crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceI16Caller.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceI16Caller.cpp]

/**
  | AudioSource that uses callback to get
  | more data.
  |
  */
pub struct OboeSourceI16Caller<'a> {
    base:              AudioSourceCaller<'a>,
    conversion_buffer: Box<&'a [i16]>,
}

impl<'a> OboeSourceI16Caller<'a> {

    pub fn new(
        channel_count:       i32,
        frames_per_callback: i32) -> Self {
    
        todo!();
        /*
            : AudioSourceCaller(channelCount, framesPerCallback, sizeof(int16_t)) 
            mConversionBuffer = std::make_unique<int16_t[]>(channelCount * output.getFramesPerBuffer());
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "OboeSourceI16Caller";
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t numBytes = mStream->getBytesPerFrame() * numFrames;
        int32_t bytesRead = mBlockReader.read((uint8_t *) mConversionBuffer.get(), numBytes);
        int32_t framesRead = bytesRead / mStream->getBytesPerFrame();

        float *floatData = output.getBuffer();
        const int16_t *shortData = mConversionBuffer.get();
        int32_t numSamples = framesRead * output.getSamplesPerFrame();

    #if FLOWGRAPH_ANDROID_INTERNAL
        memcpy_to_float_from_i16(floatData, shortData, numSamples);
    #else
        for (int i = 0; i < numSamples; i++) {
            *floatData++ = *shortData++ * (1.0f / 32768);
        }
    #endif

        return framesRead;
        */
    }
}
