/** WARNING - UNDER CONSTRUCTION - THIS API WILL CHANGE. */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/AudioStream.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/AudioStream.cpp]

/**
  | The default number of nanoseconds to
  | wait for when performing state change
  | operations on the stream, such as `start`
  | and `stop`.
  | 
  | @see OboeAudioStream::start
  |
  */
pub const kDefaultTimeoutNanos: i64 = (2000 * kNanosPerMillisecond);

/**
  | Base class for Oboe C++ audio stream.
  |
  */
pub struct AudioStream {

    base: AudioStreamBase,

    /**
      | weak pointer to this object
      |
      */
    weak_this:             Weak<AudioStream>,

    /**
      | Number of frames which have been written into the
      | stream * This is signed integer to match the counters
      | in AAudio. At audio rates, the counter will overflow
      | in about six million years.
      */
    frames_written:        Atomic<i64>,

    /**
      | Number of frames which have been read from the
      | stream. * This is signed integer to match the counters
      | in AAudio. At audio rates, the counter will overflow
      | in about six million years.
      */
    frames_read:           Atomic<i64>,

    /**
      | for synchronizing start/stop/close
      |
      */
    lock:                  parking_lot::RawMutex,

    error_callback_result: OboeResult, // = OboeResult::OK;

    /**
      | Number of frames which will be copied
      | to/from the audio device in a single
      | read/write operation
      |
      */
    frames_per_burst:      i32, // default = kUnspecified

    previous_scheduler:    i32,        // default = -1
    data_callback_enabled: AtomicBool, // default = false
    error_callback_called: AtomicBool, // default = false
}

impl AudioStream {

    /* allow access to setWeakThis() and lockWeakThis() */

    /**
      | Open a stream based on the current settings.
      | 
      | Note that we do not recommend re-opening
      | a stream that has been closed. TODO Should
      | we prevent re-opening?
      | 
      | @return
      |
      */
    pub fn open(&mut self) -> OboeResult {
        
        todo!();
        /*
            return OboeResult::OK; // Called by subclasses. Might do more in the future.
        */
    }

    /**
      | This can be used to adjust the latency
      | of the buffer by changing the threshold
      | where blocking will occur. By combining
      | this with getXRunCount(), the latency
      | can be tuned at run-time for each device.
      | 
      | This cannot be set higher than getBufferCapacity().
      | 
      | -----------
      | @param requestedFrames
      | 
      | requested number of frames that can
      | be filled without blocking @return
      | the resulting buffer size in frames
      | (obtained using value()) or an error
      | (obtained using error())
      |
      */
    pub fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return OboeResult::ErrorUnimplemented;
        */
    }

    /**
      | An XRun is an Underrun or an Overrun.
      | During playing, an underrun will occur
      | if the stream is not written in time and
      | the system runs out of valid data. During
      | recording, an overrun will occur if
      | the stream is not read in time and there
      | is no place to put the incoming data so
      | it is discarded.
      | 
      | An underrun or overrun can cause an audible
      | "pop" or "glitch".
      | 
      | -----------
      | @return
      | 
      | a result which is either OboeResult::OK
      | with the xRun count as the value, or a
      | OboeResult::Error* code
      |
      */
    pub fn getx_run_count(&mut self) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return ResultWithValue<int32_t>(OboeResult::ErrorUnimplemented);
        */
    }

    /**
      | Query the number of frames that are read
      | or written by the endpoint at one time.
      | 
      | -----------
      | @return
      | 
      | burst size
      |
      */
    pub fn get_frames_per_burst(&self) -> i32 {
        
        todo!();
        /*
            return mFramesPerBurst;
        */
    }

    /**
      | Get the number of bytes in each audio
      | frame. This is calculated using the
      | channel count and the sample format.
      | For example, a 2 channel floating point
      | stream will have 2 * 4 = 8 bytes per frame.
      | 
      | -----------
      | @return
      | 
      | number of bytes in each audio frame.
      |
      */
    pub fn get_bytes_per_frame(&self) -> i32 {
        
        todo!();
        /*
            return mChannelCount * getBytesPerSample();
        */
    }

    /**
      | Calculate the latency of a stream based
      | on getTimestamp().
      | 
      | Output latency is the time it takes for
      | a given frame to travel from the app to
      | some type of digital-to-analog converter.
      | If the DAC is external, for example in
      | a USB interface or a TV connected by HDMI,
      | then there may be additional latency
      | that the Android device is unaware of.
      | 
      | Input latency is the time it takes to
      | a given frame to travel from an analog-to-digital
      | converter (ADC) to the app.
      | 
      | -----------
      | @note
      | 
      | the latency of an OUTPUT stream will
      | increase abruptly when you write data
      | to it and then decrease slowly over time
      | as the data is consumed.
      | 
      | The latency of an INPUT stream will decrease
      | abruptly when you read data from it and
      | then increase slowly over time as more
      | data arrives.
      | 
      | The latency of an OUTPUT stream is generally
      | higher than the INPUT latency because
      | an app generally tries to keep the OUTPUT
      | buffer full and the INPUT buffer empty.
      | 
      | -----------
      | @return
      | 
      | a ResultWithValue which has a result
      | of OboeResult::OK and a value containing
      | the latency in milliseconds, or a result
      | of OboeResult::Error*.
      |
      */
    pub fn calculate_latency_millis(&mut self) -> OboeResultWithValue<f64> {
        
        todo!();
        /*
            return ResultWithValue<double>(OboeResult::ErrorUnimplemented);
        */
    }

    /**
      | Get the estimated time that the frame
      | at `framePosition` entered or left
      | the audio processing pipeline.
      | 
      | This can be used to coordinate events
      | and interactions with the external
      | environment, and to estimate the latency
      | of an audio stream. An example of usage
      | can be found in the hello-oboe sample
      | (search for "calculateCurrentOutputLatencyMillis").
      | 
      | The time is based on the implementation's
      | best effort, using whatever knowledge
      | is available to the system, but cannot
      | account for any delay unknown to the
      | implementation.
      | 
      | @deprecated since 1.0, use AudioStream::getTimestamp(clockid_t
      | clockId) instead, which returns ResultWithValue
      | @param clockId the type of clock to use
      | e.g. CLOCK_MONOTONIC @param framePosition
      | the frame number to query @param timeNanoseconds
      | an output parameter which will contain
      | the presentation timestamp
      |
      */
    pub fn get_timestamp_with_frame_position(
        &mut self, 
        clock_id:         ClockId,
        frame_position:   *mut i64,
        time_nanoseconds: *mut i64) -> OboeResult {
        
        todo!();
        /*
            return OboeResult::ErrorUnimplemented;
        */
    }

    /* ============== I/O =========================== */

    /**
      | Write data from the supplied buffer
      | into the stream. This method will block
      | until the write is complete or it runs
      | out of time.
      | 
      | If `timeoutNanoseconds` is zero then
      | this call will not wait.
      | 
      | -----------
      | @param buffer
      | 
      | The address of the first sample. @param
      | numFrames Number of frames to write.
      | Only complete frames will be written.
      | @param timeoutNanoseconds Maximum
      | number of nanoseconds to wait for completion.
      | @return a ResultWithValue which has
      | a result of OboeResult::OK and a value containing
      | the number of frames actually written,
      | or result of OboeResult::Error*.
      |
      */
    pub fn write(&mut self, 
        buffer:              *const c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return ResultWithValue<int32_t>(OboeResult::ErrorUnimplemented);
        */
    }

    /**
      | Read data into the supplied buffer from
      | the stream. This method will block until
      | the read is complete or it runs out of
      | time.
      | 
      | If `timeoutNanoseconds` is zero then
      | this call will not wait.
      | 
      | -----------
      | @param buffer
      | 
      | The address of the first sample. @param
      | numFrames Number of frames to read.
      | Only complete frames will be read. @param
      | timeoutNanoseconds Maximum number
      | of nanoseconds to wait for completion.
      | @return a ResultWithValue which has
      | a result of OboeResult::OK and a value containing
      | the number of frames actually read,
      | or result of OboeResult::Error*.
      |
      */
    pub fn read(&mut self, 
        buffer:              *mut c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return ResultWithValue<int32_t>(OboeResult::ErrorUnimplemented);
        */
    }

    /**
      | Returns true if the underlying audio
      | API is AAudio.
      | 
      | -----------
      | @return
      | 
      | true if this stream is implemented using
      | the AAudio API.
      |
      */
    pub fn uses_aaudio(&self) -> bool {
        
        todo!();
        /*
            return getAudioApi() == AudioApi::AAudio;
        */
    }

    /**
      | Only for debugging. Do not use in production.
      | If you need to call this method something
      | is wrong. If you think you need it for
      | production then please let us know so
      | we can modify Oboe so that you don't need
      | this.
      | 
      | -----------
      | @return
      | 
      | nullptr or a pointer to a stream from
      | the system API
      |
      */
    pub fn get_underlying_stream(&self)  {
        
        todo!();
        /*
            return nullptr;
        */
    }

    /**
      | Swap old callback for new callback.
      | 
      | This not atomic.
      | 
      | This should only be used internally.
      | 
      | -----------
      | @param dataCallback
      | 
      | @return previous dataCallback
      |
      */
    pub fn swap_data_callback(&mut self, data_callback: *mut dyn AudioStreamDataCallback) -> *mut dyn AudioStreamDataCallback {
        
        todo!();
        /*
            AudioStreamDataCallback *previousCallback = mDataCallback;
            mDataCallback = dataCallback;
            return previousCallback;
        */
    }

    /**
      | Swap old callback for new callback.
      | 
      | This not atomic.
      | 
      | This should only be used internally.
      | 
      | -----------
      | @param errorCallback
      | 
      | @return previous errorCallback
      |
      */
    pub fn swap_error_callback(&mut self, error_callback: *mut dyn AudioStreamErrorCallback) -> *mut dyn AudioStreamErrorCallback {
        
        todo!();
        /*
            AudioStreamErrorCallback *previousCallback = mErrorCallback;
            mErrorCallback = errorCallback;
            return previousCallback;
        */
    }

    /**
      | @return
      | 
      | last result passed from an error callback
      |
      */
    pub fn get_last_error_callback_result(&self) -> OboeResult {
        
        todo!();
        /*
            return mErrorCallbackResult;
        */
    }

    /**
      | This is used to detect more than one error
      | callback from a stream.
      | 
      | These were bugs in some versions of Android
      | that caused multiple error callbacks.
      | 
      | Internal bug b/63087953
      | 
      | Calling this sets an atomic<bool> true
      | and returns the previous value.
      | 
      | -----------
      | @return
      | 
      | false on first call, true on subsequent
      | calls
      |
      */
    pub fn was_error_callback_called(&mut self) -> bool {
        
        todo!();
        /*
            return mErrorCallbackCalled.exchange(true);
        */
    }

    /**
      | Override this to provide a default for
      | when the application did not specify
      | a callback.
      | 
      | -----------
      | @param audioData
      | 
      | @param numFrames @return result
      |
      */
    pub fn on_default_callback(&mut self, 
        audio_data: *mut c_void,
        num_frames: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            return DataCallbackResult::Stop;
        */
    }

    /**
      | @return
      | 
      | true if callbacks may be called
      |
      */
    pub fn is_data_callback_enabled(&mut self) -> bool {
        
        todo!();
        /*
            return mDataCallbackEnabled;
        */
    }

    /**
      | This can be set false internally to prevent
      | callbacks after DataCallbackResult::Stop
      | has been returned.
      |
      */
    pub fn set_data_callback_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            mDataCallbackEnabled = enabled;
        */
    }

    /**
      | Set a weak_ptr to this stream from the
      | shared_ptr so that we can later use a
      | shared_ptr in the error callback.
      |
      */
    pub fn set_weak_this(&mut self, shared_stream: &mut Arc<OboeAudioStream>)  {
        
        todo!();
        /*
            mWeakThis = sharedStream;
        */
    }

    /**
      | Make a shared_ptr that will prevent
      | this stream from being deleted.
      |
      */
    pub fn lock_weak_this(&mut self) -> Arc<OboeAudioStream> {
        
        todo!();
        /*
            return mWeakThis.lock();
        */
    }

    /**
      | Construct an `AudioStream` using the
      | given `AudioStreamBuilder`
      | 
      | -----------
      | @param builder
      | 
      | containing all the stream's attributes
      |
      */
    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream_base(builder),
        */
    }
    
    /**
      | Close the stream and deallocate any
      | resources from the open() call.
      |
      */
    pub fn close(&mut self) -> OboeResult {
        
        todo!();
        /*
            // Update local counters so they can be read after the close.
        updateFramesWritten();
        updateFramesRead();
        return OboeResult::OK;
        */
    }

    /**
      | Log the scheduler if it changes.
      |
      | Call this from fireDataCallback()
      | if you want to monitor CPU scheduler.
      |
      */
    pub fn check_scheduler(&mut self)  {
        
        todo!();
        /*
            int scheduler = sched_getscheduler(0) & ~SCHED_RESET_ON_FORK; // for current thread
        if (scheduler != mPreviousScheduler) {
            LOGD("AudioStream::%s() scheduler = %s", __func__,
                    ((scheduler == SCHED_FIFO) ? "SCHED_FIFO" :
                    ((scheduler == SCHED_OTHER) ? "SCHED_OTHER" :
                    ((scheduler == SCHED_RR) ? "SCHED_RR" : "UNKNOWN")))
            );
            mPreviousScheduler = scheduler;
        }
        */
    }
    
    /**
      | Override this to provide your own behaviour
      | for the audio callback
      | 
      | -----------
      | @param audioData
      | 
      | container array which audio frames
      | will be written into or read from @param
      | numFrames number of frames which were
      | read/written @return the result of
      | the callback: stop or continue
      |
      */
    pub fn fire_data_callback(&mut self, 
        audio_data: *mut c_void,
        num_frames: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            if (!isDataCallbackEnabled()) {
            LOGW("AudioStream::%s() called with data callback disabled!", __func__);
            return DataCallbackResult::Stop; // Should not be getting called
        }

        DataCallbackResult result;
        if (mDataCallback) {
            result = mDataCallback->onAudioReady(this, audioData, numFrames);
        } else {
            result = onDefaultCallback(audioData, numFrames);
        }
        // On Oreo, we might get called after returning stop.
        // So block that here.
        setDataCallbackEnabled(result == DataCallbackResult::Continue);

        return result;
        */
    }
    
    /**
      | Wait for a transition from one state
      | to another. @return OK if the endingState
      | was observed, or ErrorUnexpectedState
      | if any state that was not the startingState
      | or endingState was observed or ErrorTimeout.
      |
      */
    pub fn wait_for_state_transition(&mut self, 
        starting_state:      OboeStreamState,
        ending_state:        OboeStreamState,
        timeout_nanoseconds: i64) -> OboeResult {
        
        todo!();
        /*
            StreamState state;
        {
            std::lock_guard<std::mutex> lock(mLock);
            state = getState();
            if (state == StreamState::Closed) {
                return OboeResult::ErrorClosed;
            } else if (state == StreamState::Disconnected) {
                return OboeResult::ErrorDisconnected;
            }
        }

        StreamState nextState = state;
        // TODO Should this be a while()?!
        if (state == startingState && state != endingState) {
            OboeResult result = waitForStateChange(state, &nextState, timeoutNanoseconds);
            if (result != OboeResult::OK) {
                return result;
            }
        }

        if (nextState != endingState) {
            return OboeResult::ErrorInvalidState;
        } else {
            return OboeResult::OK;
        }
        */
    }
    
    /**
      | Start the stream. This will block until
      | the stream has been started, an error
      | occurs or `timeoutNanoseconds` has
      | been reached.
      |
      */
    pub fn start(&mut self, timeout_nanoseconds: Option<i64>) 
        -> OboeResult 
    {
        let timeout_nanoseconds: i64 = timeout_nanoseconds.unwrap_or(kDefaultTimeoutNanos);
        
        todo!();
        /*
            OboeResult result = requestStart();
        if (result != OboeResult::OK) return result;
        if (timeoutNanoseconds <= 0) return result;
        return waitForStateTransition(StreamState::Starting,
                                      StreamState::Started, timeoutNanoseconds);
        */
    }
    
    /**
      | Pause the stream. This will block until
      | the stream has been paused, an error
      | occurs or `timeoutNanoseconds` has
      | been reached.
      |
      */
    pub fn pause(&mut self, timeout_nanoseconds: Option<i64>) -> OboeResult {

        let timeout_nanoseconds: i64 = timeout_nanoseconds.unwrap_or(kDefaultTimeoutNanos);
        
        todo!();
        /*
            OboeResult result = requestPause();
        if (result != OboeResult::OK) return result;
        if (timeoutNanoseconds <= 0) return result;
        return waitForStateTransition(StreamState::Pausing,
                                      StreamState::Paused, timeoutNanoseconds);
        */
    }
    
    /**
      | Flush the stream. This will block until
      | the stream has been flushed, an error
      | occurs or `timeoutNanoseconds` has
      | been reached.
      |
      */
    pub fn flush(&mut self, timeout_nanoseconds: Option<i64>) -> OboeResult {

        let timeout_nanoseconds: i64 = timeout_nanoseconds.unwrap_or(kDefaultTimeoutNanos);
        
        todo!();
        /*
            OboeResult result = requestFlush();
        if (result != OboeResult::OK) return result;
        if (timeoutNanoseconds <= 0) return result;
        return waitForStateTransition(StreamState::Flushing,
                                      StreamState::Flushed, timeoutNanoseconds);
        */
    }
    
    /**
      | Stop the stream. This will block until
      | the stream has been stopped, an error
      | occurs or `timeoutNanoseconds` has
      | been reached.
      |
      */
    pub fn stop(&mut self, timeout_nanoseconds: Option<i64>) -> OboeResult {

        let timeout_nanoseconds: i64 = timeout_nanoseconds.unwrap_or(kDefaultTimeoutNanos);
        
        todo!();
        /*
            OboeResult result = requestStop();
        if (result != OboeResult::OK) return result;
        if (timeoutNanoseconds <= 0) return result;
        return waitForStateTransition(StreamState::Stopping,
                                      StreamState::Stopped, timeoutNanoseconds);
        */
    }
    
    /**
      | Get the number of bytes per sample. This
      | is calculated using the sample format.
      | For example, a stream using 16-bit integer
      | samples will have 2 bytes per sample.
      | 
      | -----------
      | @return
      | 
      | the number of bytes per sample.
      |
      */
    pub fn get_bytes_per_sample(&self) -> i32 {
        
        todo!();
        /*
            return convertFormatToSizeInBytes(mFormat);
        */
    }
    
    /**
      | The number of audio frames read from
      | the stream. This monotonic counter
      | will never get reset.
      | 
      | -----------
      | @return
      | 
      | the number of frames read so far
      |
      */
    pub fn get_frames_read(&mut self) -> i64 {
        
        todo!();
        /*
            updateFramesRead();
        return mFramesRead;
        */
    }
    
    /**
      | The number of audio frames written into
      | the stream. This monotonic counter
      | will never get reset.
      | 
      | -----------
      | @return
      | 
      | the number of frames written so far
      |
      */
    pub fn get_frames_written(&mut self) -> i64 {
        
        todo!();
        /*
            updateFramesWritten();
        return mFramesWritten;
        */
    }
    
    /**
      | @return
      | 
      | number of frames of data currently in
      | the buffer
      |
      */
    pub fn get_available_frames(&mut self) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            int64_t readCounter = getFramesRead();
        if (readCounter < 0) return ResultWithValue<int32_t>::createBasedOnSign(readCounter);
        int64_t writeCounter = getFramesWritten();
        if (writeCounter < 0) return ResultWithValue<int32_t>::createBasedOnSign(writeCounter);
        int32_t framesAvailable = writeCounter - readCounter;
        return ResultWithValue<int32_t>(framesAvailable);
        */
    }
    
    /**
      | Wait until the stream has a minimum amount
      | of data available in its buffer. This
      | can be used with an EXCLUSIVE MMAP input
      | stream to avoid reading data too close
      | to the DSP write position, which may
      | cause glitches.
      | 
      | -----------
      | @param numFrames
      | 
      | minimum frames available @param timeoutNanoseconds
      | @return number of frames available,
      | ErrorTimeout
      |
      */
    pub fn wait_for_available_frames(&mut self, 
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            if (numFrames == 0) return OboeResult::OK;
        if (numFrames < 0) return OboeResult::ErrorOutOfRange;

        int64_t framesAvailable = 0;
        int64_t burstInNanos = getFramesPerBurst() * kNanosPerSecond / getSampleRate();
        bool ready = false;
        int64_t deadline = AudioClock::getNanoseconds() + timeoutNanoseconds;
        do {
            ResultWithValue<int32_t> result = getAvailableFrames();
            if (!result) return result;
            framesAvailable = result.value();
            ready = (framesAvailable >= numFrames);
            if (!ready) {
                int64_t now = AudioClock::getNanoseconds();
                if (now > deadline) break;
                AudioClock::sleepForNanos(burstInNanos);
            }
        } while (!ready);
        return (!ready)
                ? ResultWithValue<int32_t>(OboeResult::ErrorTimeout)
                : ResultWithValue<int32_t>(framesAvailable);
        */
    }
    
    /**
      | Get the estimated time that the frame
      | at `framePosition` entered or left
      | the audio processing pipeline.
      | 
      | This can be used to coordinate events
      | and interactions with the external
      | environment, and to estimate the latency
      | of an audio stream. An example of usage
      | can be found in the hello-oboe sample
      | (search for "calculateCurrentOutputLatencyMillis").
      | 
      | The time is based on the implementation's
      | best effort, using whatever knowledge
      | is available to the system, but cannot
      | account for any delay unknown to the
      | implementation.
      | 
      | 
      | 
      | -----------
      | @param clockId
      | 
      | the type of clock to use e.g. CLOCK_MONOTONIC
      | @return a FrameTimestamp containing
      | the position and time at which a particular
      | audio frame entered or left the audio
      | processing pipeline, or an error if
      | the operation failed.
      |
      */
    pub fn get_timestamp(&mut self, clock_id: ClockId) -> OboeResultWithValue<OboeFrameTimestamp> {
        
        todo!();
        /*
            FrameTimestamp frame;
        OboeResult result = getTimestamp(clockId, &frame.position, &frame.timestamp);
        if (result == OboeResult::OK){
            return ResultWithValue<FrameTimestamp>(frame);
        } else {
            return ResultWithValue<FrameTimestamp>(static_cast<OboeResult>(result));
        }
        */
    }
}
