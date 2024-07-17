crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/AudioSourceCaller.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/AudioSourceCaller.cpp]

/**
  | For output streams that use a callback,
  | call the application for more data.
  | 
  | For input streams that do not use a callback,
  | read from the stream.
  |
  */
pub struct AudioSourceCaller<'a> {
    base:          FlowGraphSource<'a>,
    stream:        *mut OboeAudioStream, // default = nullptr
    timeout_nanos: i64, // default = 0
    block_reader:  FixedBlockReader<'a>,
}

impl<'a> FixedBlockProcessor for AudioSourceCaller<'a> {

    fn on_process_fixed_block(
        &mut self, 
        _: *mut u8, 
        _: i32

    ) -> i32 { 

        todo!() 
    }
}

impl<'a> AudioSourceCaller<'a> {

    pub fn new(
        channel_count:       i32,
        frames_per_callback: i32,
        bytes_per_sample:    i32) -> Self {
    
        todo!();
        /*
            : FlowGraphSource(channelCount)
                , mBlockReader(*this) 

            mBlockReader.open(channelCount * framesPerCallback * bytesPerSample);
        */
    }

    /**
      | Set the stream to use as a source of data.
      | @param stream
      |
      */
    pub fn set_stream(&mut self, stream: *mut OboeAudioStream)  {
        
        todo!();
        /*
            mStream = stream;
        */
    }
    
    pub fn get_stream(&mut self) -> *mut OboeAudioStream {
        
        todo!();
        /*
            return mStream;
        */
    }

    /**
      | Timeout value to use when calling audioStream->read().
      | 
      | -----------
      | @param timeoutNanos
      | 
      | Zero for no timeout or time in nanoseconds.
      |
      */
    pub fn set_timeout_nanos(&mut self, timeout_nanos: i64)  {
        
        todo!();
        /*
            mTimeoutNanos = timeoutNanos;
        */
    }
    
    pub fn get_timeout_nanos(&self) -> i64 {
        
        todo!();
        /*
            return mTimeoutNanos;
        */
    }

    /**
      | Called internally for block size adaptation.
      | 
      | -----------
      | @param buffer
      | 
      | @param numBytes
      | 
      | @return
      |
      */
    pub fn on_process_fixed_block(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            AudioStreamDataCallback *callback = mStream->getDataCallback();
        int32_t result = 0;
        int32_t numFrames = numBytes / mStream->getBytesPerFrame();
        if (callback != nullptr) {
            DataCallbackResult callbackResult = callback->onAudioReady(mStream, buffer, numFrames);
            // onAudioReady() does not return the number of bytes processed so we have to assume all.
            result = (callbackResult == DataCallbackResult::Continue)
                    ? numBytes
                    : -1;
        } else {
            auto readResult = mStream->read(buffer, numFrames, mTimeoutNanos);
            if (!readResult) return (int32_t) readResult.error();
            result = readResult.value() * mStream->getBytesPerFrame();
        }
        return result;
        */
    }
}
