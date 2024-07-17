crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceI24Caller.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceI24Caller.cpp]

/**
  | AudioSource that uses callback to get
  | more data.
  |
  */
pub struct OboeSourceI24Caller<'a> {
    base:              AudioSourceCaller<'a>,
    conversion_buffer: Box<&'a [u8]>,
}

pub const SourceI24CallerBytesPerI24Packed: i32 = 3;

impl<'a> OboeSourceI24Caller<'a> {

    pub fn new(
        channel_count:       i32,
        frames_per_callback: i32) -> Self {
    
        todo!();
        /*


            : AudioSourceCaller(channelCount, framesPerCallback, kBytesPerI24Packed) 
            mConversionBuffer = std::make_unique<uint8_t[]>(
                    kBytesPerI24Packed * channelCount * output.getFramesPerBuffer());
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "OboeSourceI24Caller";
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t numBytes = mStream->getBytesPerFrame() * numFrames;
        int32_t bytesRead = mBlockReader.read((uint8_t *) mConversionBuffer.get(), numBytes);
        int32_t framesRead = bytesRead / mStream->getBytesPerFrame();

        float *floatData = output.getBuffer();
        const uint8_t *byteData = mConversionBuffer.get();
        int32_t numSamples = framesRead * output.getSamplesPerFrame();

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

        return framesRead;
        */
    }
}
