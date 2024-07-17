crate::ix!();

pub trait OboeAudioStreamBufferedInterface {

    fn update_service_frame_counter(&mut self) -> OboeResult;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioStreamBuffered.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioStreamBuffered.cpp]

// arbitrary, allows dynamic latency tuning
pub const kDefaultBurstsPerBuffer: i32 = 16;      

// arbitrary, allows dynamic latency tuning
pub const kMinBurstsPerBuffer:     i32 = 4;       

// arbitrary
pub const kMinFramesPerBuffer:     i32 = 48 * 32; 

/**
  | A stream that contains a FIFO buffer.
  |
  | This is used to implement blocking reads and
  | writes.
  */
pub struct OboeAudioStreamBuffered {
    base:                          AudioStream,
    fifo_buffer:                   Box<FifoBuffer>,
    background_ran_at_nanoseconds: i64, // default = 0
    last_background_size:          i32, // default = 0
    xrun_count:                    i32, // default = 0
}

/**
  | AudioStream with a FifoBuffer
  |
  */
impl OboeAudioStreamBuffered {

    pub fn get_xrun_count(&mut self) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return ResultWithValue<int32_t>(mXRunCount);
        */
    }

    /**
      | If there is no callback then we need a
      | FIFO between the App and OpenSL ES.
      |
      */
    pub fn usingfifo(&self) -> bool {
        
        todo!();
        /*
            return !isDataCallbackSpecified();
        */
    }

    pub fn increment_xrun_count(&mut self)  {
        
        todo!();
        /*
            ++mXRunCount;
        */
    }
    
    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream(builder),
        */
    }
    
    pub fn allocate_fifo(&mut self)  {
        
        todo!();
        /*
            // If the caller does not provide a callback use our own internal
        // callback that reads data from the FIFO.
        if (usingFIFO()) {
            // FIFO is configured with the same format and channels as the stream.
            int32_t capacityFrames = getBufferCapacityInFrames();
            if (capacityFrames == OboekUnspecified) {
                capacityFrames = getFramesPerBurst() * kDefaultBurstsPerBuffer;
            } else {
                int32_t minFramesPerBufferByBursts = getFramesPerBurst() * kMinBurstsPerBuffer;
                if (capacityFrames <= minFramesPerBufferByBursts) {
                    capacityFrames = minFramesPerBufferByBursts;
                } else {
                    capacityFrames = std::max(kMinFramesPerBuffer, capacityFrames);
                    // round up to nearest burst
                    int32_t numBursts = (capacityFrames + getFramesPerBurst() - 1)
                            / getFramesPerBurst();
                    capacityFrames = numBursts * getFramesPerBurst();
                }
            }
            // TODO consider using std::make_unique if we require c++14
            mFifoBuffer.reset(new FifoBuffer(getBytesPerFrame(), capacityFrames));
            mBufferCapacityInFrames = capacityFrames;
        }
        */
    }
    
    pub fn update_frames_written(&mut self)  {
        
        todo!();
        /*
            if (mFifoBuffer) {
            mFramesWritten = static_cast<int64_t>(mFifoBuffer->getWriteCounter());
        } // or else it will get updated by processBufferCallback()
        */
    }
    
    pub fn update_frames_read(&mut self)  {
        
        todo!();
        /*
            if (mFifoBuffer) {
            mFramesRead = static_cast<int64_t>(mFifoBuffer->getReadCounter());
        } // or else it will get updated by processBufferCallback()
        */
    }

    /**
      | This is called by the OpenSL ES callback
      | to read or write the back end of the FIFO.
      |
      */
    pub fn on_default_callback(&mut self, 
        audio_data: *mut c_void,
        num_frames: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            int32_t framesTransferred  = 0;

        if (getDirection() == OboeDirection::Output) {
            // Read from the FIFO and write to audioData, clear part of buffer if not enough data.
            framesTransferred = mFifoBuffer->readNow(audioData, numFrames);
        } else {
            // Read from audioData and write to the FIFO
            framesTransferred = mFifoBuffer->write(audioData, numFrames); // There is no writeNow()
        }

        if (framesTransferred < numFrames) {
            LOGD("OboeAudioStreamBuffered::%s(): xrun! framesTransferred = %d, numFrames = %d",
                    __func__, framesTransferred, numFrames);
            // TODO If we do not allow FIFO to wrap then our timestamps will drift when there is an XRun!
            incrementXRunCount();
        }
        markCallbackTime(static_cast<int32_t>(numFrames)); // so foreground knows how long to wait.
        return DataCallbackResult::Continue;
        */
    }
    
    pub fn mark_callback_time(&mut self, num_frames: i32)  {
        
        todo!();
        /*
            mLastBackgroundSize = numFrames;
        mBackgroundRanAtNanoseconds = AudioClock::getNanoseconds();
        */
    }
    
    pub fn predict_next_callback_time(&mut self) -> i64 {
        
        todo!();
        /*
            if (mBackgroundRanAtNanoseconds == 0) {
            return 0;
        }
        int64_t nanosPerBuffer = (kNanosPerSecond * mLastBackgroundSize) / getSampleRate();
        const int64_t margin = 200 * kNanosPerMicrosecond; // arbitrary delay so we wake up just after
        return mBackgroundRanAtNanoseconds + nanosPerBuffer + margin;
        */
    }

    /**
      | Common code for read/write.
      | 
      | Read or write to the FIFO.
      | Only pass one pointer and set the other to nullptr.
      | 
      | -----------
      | @return
      | 
      | OboeResult::OK with frames read/written,
      | or OboeResult::Error*
      |
      */
    pub fn transfer(&mut self, 
        read_buffer:         *mut c_void,
        write_buffer:        *const c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            // Validate arguments.
        if (readBuffer != nullptr && writeBuffer != nullptr) {
            LOGE("OboeAudioStreamBuffered::%s(): both buffers are not NULL", __func__);
            return ResultWithValue<int32_t>(OboeResult::ErrorInternal);
        }
        if (getDirection() == Direction::Input && readBuffer == nullptr) {
            LOGE("OboeAudioStreamBuffered::%s(): readBuffer is NULL", __func__);
            return ResultWithValue<int32_t>(OboeResult::ErrorNull);
        }
        if (getDirection() == Direction::Output && writeBuffer == nullptr) {
            LOGE("OboeAudioStreamBuffered::%s(): writeBuffer is NULL", __func__);
            return ResultWithValue<int32_t>(OboeResult::ErrorNull);
        }
        if (numFrames < 0) {
            LOGE("OboeAudioStreamBuffered::%s(): numFrames is negative", __func__);
            return ResultWithValue<int32_t>(OboeResult::ErrorOutOfRange);
        } else if (numFrames == 0) {
            return ResultWithValue<int32_t>(numFrames);
        }
        if (timeoutNanoseconds < 0) {
            LOGE("OboeAudioStreamBuffered::%s(): timeoutNanoseconds is negative", __func__);
            return ResultWithValue<int32_t>(OboeResult::ErrorOutOfRange);
        }

        int32_t result = 0;
        uint8_t *readData = reinterpret_cast<uint8_t *>(readBuffer);
        const uint8_t *writeData = reinterpret_cast<const uint8_t *>(writeBuffer);
        int32_t framesLeft = numFrames;
        int64_t timeToQuit = 0;
        bool repeat = true;

        // Calculate when to timeout.
        if (timeoutNanoseconds > 0) {
            timeToQuit = AudioClock::getNanoseconds() + timeoutNanoseconds;
        }

        // Loop until we get the data, or we have an error, or we timeout.
        do {
            // read or write
            if (getDirection() == Direction::Input) {
                result = mFifoBuffer->read(readData, framesLeft);
                if (result > 0) {
                    readData += mFifoBuffer->convertFramesToBytes(result);
                    framesLeft -= result;
                }
            } else {
                // between zero and capacity
                uint32_t fullFrames = mFifoBuffer->getFullFramesAvailable();
                // Do not write above threshold size.
                int32_t emptyFrames = getBufferSizeInFrames() - static_cast<int32_t>(fullFrames);
                int32_t framesToWrite = std::max(0, std::min(framesLeft, emptyFrames));
                result = mFifoBuffer->write(writeData, framesToWrite);
                if (result > 0) {
                    writeData += mFifoBuffer->convertFramesToBytes(result);
                    framesLeft -= result;
                }
            }

            // If we need more data then sleep and try again.
            if (framesLeft > 0 && result >= 0 && timeoutNanoseconds > 0) {
                int64_t timeNow = AudioClock::getNanoseconds();
                if (timeNow >= timeToQuit) {
                    LOGE("OboeAudioStreamBuffered::%s(): TIMEOUT", __func__);
                    repeat = false; // TIMEOUT
                } else {
                    // Figure out how long to sleep.
                    int64_t sleepForNanos;
                    int64_t wakeTimeNanos = predictNextCallbackTime();
                    if (wakeTimeNanos <= 0) {
                        // No estimate available. Sleep for one burst.
                        sleepForNanos = (getFramesPerBurst() * kNanosPerSecond) / getSampleRate();
                    } else {
                        // Don't sleep past timeout.
                        if (wakeTimeNanos > timeToQuit) {
                            wakeTimeNanos = timeToQuit;
                        }
                        sleepForNanos = wakeTimeNanos - timeNow;
                        // Avoid rapid loop with no sleep.
                        const int64_t minSleepTime = kNanosPerMillisecond; // arbitrary
                        if (sleepForNanos < minSleepTime) {
                            sleepForNanos = minSleepTime;
                        }
                    }

                    AudioClock::sleepForNanos(sleepForNanos);
                }

            } else {
                repeat = false;
            }
        } while(repeat);

        if (result < 0) {
            return ResultWithValue<int32_t>(static_cast<OboeResult>(result));
        } else {
            int32_t framesWritten = numFrames - framesLeft;
            return ResultWithValue<int32_t>(framesWritten);
        }
        */
    }

    /**
      | Write to the FIFO so the callback can
      | read from it.
      |
      */
    pub fn write(&mut self, 
        buffer:              *const c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            if (getState() == StreamState::Closed){
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }

        if (getDirection() == Direction::Input) {
            return ResultWithValue<int32_t>(OboeResult::ErrorUnavailable); // TODO review, better error code?
        }
        OboeResult result = updateServiceFrameCounter();
        if (result != OboeResult::OK) return ResultWithValue<int32_t>(static_cast<OboeResult>(result));
        return transfer(nullptr, buffer, numFrames, timeoutNanoseconds);
        */
    }

    /**
      | Read data from the FIFO that was written
      | by the callback.
      |
      */
    pub fn read(&mut self, 
        buffer:              *mut c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            if (getState() == StreamState::Closed){
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }

        if (getDirection() == Direction::Output) {
            return ResultWithValue<int32_t>(OboeResult::ErrorUnavailable); // TODO review, better error code?
        }
        OboeResult result = updateServiceFrameCounter();
        if (result != OboeResult::OK) return ResultWithValue<int32_t>(static_cast<OboeResult>(result));
        return transfer(buffer, nullptr, numFrames, timeoutNanoseconds);
        */
    }

    /**
      | Only supported when we are not using
      | a callback.
      |
      */
    pub fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            if (getState() == StreamState::Closed){
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }

        if (!mFifoBuffer) {
            return ResultWithValue<int32_t>(OboeResult::ErrorUnimplemented);
        }

        if (requestedFrames > mFifoBuffer->getBufferCapacityInFrames()) {
            requestedFrames = mFifoBuffer->getBufferCapacityInFrames();
        } else if (requestedFrames < getFramesPerBurst()) {
            requestedFrames = getFramesPerBurst();
        }
        mBufferSizeInFrames = requestedFrames;
        return ResultWithValue<int32_t>(requestedFrames);
        */
    }
    
    pub fn get_buffer_capacity_in_frames(&self) -> i32 {
        
        todo!();
        /*
            if (mFifoBuffer) {
            return mFifoBuffer->getBufferCapacityInFrames();
        } else {
            return AudioStream::getBufferCapacityInFrames();
        }
        */
    }
    
    pub fn is_xrun_count_supported(&self) -> bool {
        
        todo!();
        /*
            // XRun count is only supported if we're using blocking I/O (not callbacks)
        return (!isDataCallbackSpecified());
        */
    }
}
