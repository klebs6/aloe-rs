crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceFloatCaller.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeSourceFloatCaller.cpp]

/**
  | AudioSource that uses callback to get
  | more float data.
  |
  */
pub struct OboeSourceFloatCaller<'a> {
    base: AudioSourceCaller<'a>,
}

impl<'a> OboeSourceFloatCaller<'a> {
    
    pub fn new(
        channel_count:       i32,
        frames_per_callback: i32) -> Self {
    
        todo!();
        /*
            : AudioSourceCaller(channelCount, framesPerCallback, (int32_t)sizeof(float))
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "OboeSourceFloatCaller";
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t numBytes = mStream->getBytesPerFrame() * numFrames;
        int32_t bytesRead = mBlockReader.read((uint8_t *) output.getBuffer(), numBytes);
        int32_t framesRead = bytesRead / mStream->getBytesPerFrame();
        return framesRead;
        */
    }
}
