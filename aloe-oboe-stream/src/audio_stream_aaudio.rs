crate::ix!();

/**
  | Time to sleep in order to prevent a race
  | condition with a callback after a close().
  |
  | Two milliseconds may be enough but 10 msec
  | is even safer.
  */
pub const AUDIO_STREAM_AAUDIO_K_DELAY_BEFORE_CLOSE_MILLIS: i32 = 10;

lazy_static!{
    /*
    static AAudioLoader *mLibLoader;
    */
}

/**
  | Implementation of OboeStream that
  | uses AAudio.
  | 
  | Do not create this class directly. Use
  | an OboeStreamBuilder to create one.
  |
  */
pub struct AudioStreamAAudio {

    base:                    AudioStream,

    callback_thread_enabled: AtomicBool,
    stop_thread_allowed:     AtomicBool, // default = false

    /**
      | pointer to the underlying 'C' AAudio
      | stream, valid if open, null if closed
      |
      */
    aaudio_stream:           Atomic<*mut AAudioStream>, // default = nullptr

    /**
      | to protect mAAudioStream while closing
      |
      */
    aaudio_stream_lock:      SharedMutex,

    /**
      | We may not use this but it is so small that
      | it is not worth allocating dynamically.
      |
      */
    default_error_callback:  Box<dyn AudioStreamErrorCallback>,
}

impl AudioStreamAAudio {

    pub fn is_xrun_count_supported(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }

    pub fn get_audio_api(&self) -> OboeAudioApi {
        
        todo!();
        /*
            return AudioApi::AAudio;
        */
    }


    pub fn get_underlying_stream(&self)  {
        
        todo!();
        /*
            return mAAudioStream.load();
        */
    }

    /**
      | Create a stream that uses Oboe Audio
      | API.
      |
      */
    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream(builder),
        : aaudio_stream(nullptr),

            mCallbackThreadEnabled.store(false);
        mLibLoader = AAudioLoader::getInstance();
        */
    }
    
    /**
      | @return
      | 
      | true if AAudio is supported on this device.
      |
      */
    pub fn is_supported(&mut self) -> bool {
        
        todo!();
        /*
            mLibLoader = AAudioLoader::getInstance();
        int openResult = mLibLoader->open();
        return openResult == 0;
        */
    }

    /**
      | Static method for the error callback.
      |
      | We use a method so we can access protected
      | methods on the stream.
      |
      | Launch a thread to handle the error.
      |
      | That other thread can safely stop, close
      | and delete the stream.
      */
    pub fn internal_error_callback(&mut self, 
        stream:    *mut AAudioStream,
        user_data: *mut c_void,
        error:     AAudioResult)  {
        
        todo!();
        /*
            OboeResult oboeResult = static_cast<OboeResult>(error);
        AudioStreamAAudio *oboeStream = reinterpret_cast<AudioStreamAAudio*>(userData);

        // Coerce the error code if needed to workaround a regression in RQ1A that caused
        // the wrong code to be passed when headsets plugged in. See b/173928197.
        if (OboeGlobals::areWorkaroundsEnabled()
                && getSdkVersion() == __ANDROID_API_R__
                && oboeResult == OboeResult::ErrorTimeout) {
            oboeResult = OboeResult::ErrorDisconnected;
            LOGD("%s() ErrorTimeout changed to ErrorDisconnected to fix b/173928197", __func__);
        }

        oboeStream->mErrorCallbackResult = oboeResult;

        // Prevents deletion of the stream if the app is using AudioStreamBuilder::openStream(shared_ptr)
        std::shared_ptr<AudioStream> sharedStream = oboeStream->lockWeakThis();

        // These checks should be enough because we assume that the stream close()
        // will join() any active callback threads and will not allow new callbacks.
        if (oboeStream->wasErrorCallbackCalled()) { // block extra error callbacks
            LOGE("%s() multiple error callbacks called!", __func__);
        } else if (stream != oboeStream->getUnderlyingStream()) {
            LOGW("%s() stream already closed or closing", __func__); // might happen if there are bugs
        } else if (sharedStream) {
            // Handle error on a separate thread using shared pointer.
            std::thread t(oboe_aaudio_error_thread_proc_shared, sharedStream, oboeResult);
            t.detach();
        } else {
            // Handle error on a separate thread.
            std::thread t(oboe_aaudio_error_thread_proc, oboeStream, oboeResult);
            t.detach();
        }
        */
    }
    
    pub fn log_unsupported_attributes(&mut self)  {
        
        todo!();
        /*
            int sdkVersion = getSdkVersion();

        // These attributes are not supported pre Android "P"
        if (sdkVersion < __ANDROID_API_P__) {
            if (mUsage != Usage::Media) {
                LOGW("Usage [AudioStreamBuilder::setUsage()] "
                     "is not supported on AAudio streams running on pre-Android P versions.");
            }

            if (mContentType != ContentType::Music) {
                LOGW("ContentType [AudioStreamBuilder::setContentType()] "
                     "is not supported on AAudio streams running on pre-Android P versions.");
            }

            if (mSessionId != SessionId::None) {
                LOGW("SessionId [AudioStreamBuilder::setSessionId()] "
                     "is not supported on AAudio streams running on pre-Android P versions.");
            }
        }
        */
    }
    
    /**
      | These functions override methods in
      | AudioStream.
      |
      | See AudioStream for documentation.
      */
    pub fn open(&mut self) -> OboeResult {
        
        todo!();
        /*
            OboeResult result = OboeResult::OK;

        if (mAAudioStream != nullptr) {
            return OboeResult::ErrorInvalidState;
        }

        result = AudioStream::open();
        if (result != OboeResult::OK) {
            return result;
        }

        AAudioStreamBuilder *aaudioBuilder;
        result = static_cast<OboeResult>(mLibLoader->createStreamBuilder(&aaudioBuilder));
        if (result != OboeResult::OK) {
            return result;
        }

        // Do not set INPUT capacity below 4096 because that prevents us from getting a FAST track
        // when using the Legacy data path.
        // If the app requests > 4096 then we allow it but we are less likely to get LowLatency.
        // See internal bug b/80308183 for more details.
        // Fixed in Q but let's still clip the capacity because high input capacity
        // does not increase latency.
        int32_t capacity = mBufferCapacityInFrames;
        constexpr int kCapacityRequiredForFastLegacyTrack = 4096; // matches value in AudioFinger
        if (OboeGlobals::areWorkaroundsEnabled()
                && mDirection == OboeDirection::Input
                && capacity != OboeUnspecified
                && capacity < kCapacityRequiredForFastLegacyTrack
                && mPerformanceMode == OboePerformanceMode::LowLatency) {
            capacity = kCapacityRequiredForFastLegacyTrack;
            LOGD("AudioStreamAAudio.open() capacity changed from %d to %d for lower latency",
                 static_cast<int>(mBufferCapacityInFrames), capacity);
        }
        mLibLoader->builder_setBufferCapacityInFrames(aaudioBuilder, capacity);

        mLibLoader->builder_setChannelCount(aaudioBuilder, mChannelCount);
        mLibLoader->builder_setDeviceId(aaudioBuilder, mDeviceId);
        mLibLoader->builder_setDirection(aaudioBuilder, static_cast<aaudio_direction_t>(mDirection));
        mLibLoader->builder_setFormat(aaudioBuilder, static_cast<aaudio_format_t>(mFormat));
        mLibLoader->builder_setSampleRate(aaudioBuilder, mSampleRate);
        mLibLoader->builder_setSharingMode(aaudioBuilder,
                                           static_cast<aaudio_sharing_mode_t>(mSharingMode));
        mLibLoader->builder_setPerformanceMode(aaudioBuilder,
                                               static_cast<aaudio_performance_mode_t>(mPerformanceMode));

        // These were added in P so we have to check for the function pointer.
        if (mLibLoader->builder_setUsage != nullptr) {
            mLibLoader->builder_setUsage(aaudioBuilder,
                                         static_cast<aaudio_usage_t>(mUsage));
        }

        if (mLibLoader->builder_setContentType != nullptr) {
            mLibLoader->builder_setContentType(aaudioBuilder,
                                               static_cast<aaudio_content_type_t>(mContentType));
        }

        if (mLibLoader->builder_setInputPreset != nullptr) {
            aaudio_input_preset_t inputPreset = mInputPreset;
            if (getSdkVersion() <= __ANDROID_API_P__ && inputPreset == InputPreset::VoicePerformance) {
                LOGD("InputPreset::VoicePerformance not supported before Q. Using VoiceRecognition.");
                inputPreset = InputPreset::VoiceRecognition; // most similar preset
            }
            mLibLoader->builder_setInputPreset(aaudioBuilder,
                                               static_cast<aaudio_input_preset_t>(inputPreset));
        }

        if (mLibLoader->builder_setSessionId != nullptr) {
            mLibLoader->builder_setSessionId(aaudioBuilder,
                                             static_cast<aaudio_session_id_t>(mSessionId));
        }

        // These were added in S so we have to check for the function pointer.
        if (mLibLoader->builder_setPackageName != nullptr && !mPackageName.empty()) {
            mLibLoader->builder_setPackageName(aaudioBuilder,
                                               mPackageName.c_str());
        }

        if (mLibLoader->builder_setAttributionTag != nullptr && !mAttributionTag.empty()) {
            mLibLoader->builder_setAttributionTag(aaudioBuilder,
                                               mAttributionTag.c_str());
        }

        if (isDataCallbackSpecified()) {
            mLibLoader->builder_setDataCallback(aaudioBuilder, oboe_aaudio_data_callback_proc, this);
            mLibLoader->builder_setFramesPerDataCallback(aaudioBuilder, getFramesPerDataCallback());

            if (!isErrorCallbackSpecified()) {
                // The app did not specify a callback so we should specify
                // our own so the stream gets closed and stopped.
                mErrorCallback = &mDefaultErrorCallback;
            }
            mLibLoader->builder_setErrorCallback(aaudioBuilder, internalErrorCallback, this);
        }
        // Else if the data callback is not being used then the write method will return an error
        // and the app can stop and close the stream.

        // ============= OPEN THE STREAM ================
        {
            AAudioStream *stream = nullptr;
            result = static_cast<OboeResult>(mLibLoader->builder_openStream(aaudioBuilder, &stream));
            mAAudioStream.store(stream);
        }
        if (result != OboeResult::OK) {
            // Warn developer because ErrorInternal is not very informative.
            if (result == OboeResult::ErrorInternal && mDirection == Direction::Input) {
                LOGW("AudioStreamAAudio.open() may have failed due to lack of "
                     "audio recording permission.");
            }
            goto error2;
        }

        // Query and cache the stream properties
        mDeviceId = mLibLoader->stream_getDeviceId(mAAudioStream);
        mChannelCount = mLibLoader->stream_getChannelCount(mAAudioStream);
        mSampleRate = mLibLoader->stream_getSampleRate(mAAudioStream);
        mFormat = static_cast<AudioFormat>(mLibLoader->stream_getFormat(mAAudioStream));
        mSharingMode = static_cast<SharingMode>(mLibLoader->stream_getSharingMode(mAAudioStream));
        mPerformanceMode = static_cast<PerformanceMode>(
                mLibLoader->stream_getPerformanceMode(mAAudioStream));
        mBufferCapacityInFrames = mLibLoader->stream_getBufferCapacity(mAAudioStream);
        mBufferSizeInFrames = mLibLoader->stream_getBufferSize(mAAudioStream);
        mFramesPerBurst = mLibLoader->stream_getFramesPerBurst(mAAudioStream);

        // These were added in P so we have to check for the function pointer.
        if (mLibLoader->stream_getUsage != nullptr) {
            mUsage = static_cast<Usage>(mLibLoader->stream_getUsage(mAAudioStream));
        }
        if (mLibLoader->stream_getContentType != nullptr) {
            mContentType = static_cast<ContentType>(mLibLoader->stream_getContentType(mAAudioStream));
        }
        if (mLibLoader->stream_getInputPreset != nullptr) {
            mInputPreset = static_cast<InputPreset>(mLibLoader->stream_getInputPreset(mAAudioStream));
        }
        if (mLibLoader->stream_getSessionId != nullptr) {
            mSessionId = static_cast<SessionId>(mLibLoader->stream_getSessionId(mAAudioStream));
        } else {
            mSessionId = SessionId::None;
        }

        LOGD("AudioStreamAAudio.open() format=%d, sampleRate=%d, capacity = %d",
                static_cast<int>(mFormat), static_cast<int>(mSampleRate),
                static_cast<int>(mBufferCapacityInFrames));

    error2:
        mLibLoader->builder_delete(aaudioBuilder);
        LOGD("AudioStreamAAudio.open: AAudioStream_Open() returned %s",
             mLibLoader->convertResultToText(static_cast<aaudio_result_t>(result)));
        return result;
        */
    }
    
    pub fn close(&mut self) -> OboeResult {
        
        todo!();
        /*
            // Prevent two threads from closing the stream at the same time and crashing.
        // This could occur, for example, if an application called close() at the same
        // time that an onError callback was being executed because of a disconnect.
        std::lock_guard<std::mutex> lock(mLock);

        AudioStream::close();

        AAudioStream *stream = nullptr;
        {
            // Wait for any methods using mAAudioStream to finish.
            std::unique_lock<std::shared_mutex> lock2(mAAudioStreamLock);
            // Closing will delete *mAAudioStream so we need to null out the pointer atomically.
            stream = mAAudioStream.exchange(nullptr);
        }
        if (stream != nullptr) {
            if (OboeGlobals::areWorkaroundsEnabled()) {
                // Make sure we are really stopped. Do it under mLock
                // so another thread cannot call requestStart() right before the close.
                requestStop_l(stream);
                // Sometimes a callback can occur shortly after a stream has been stopped and
                // even after a close! If the stream has been closed then the callback
                // can access memory that has been freed. That causes a crash.
                // This seems to be more likely in Android P or earlier.
                // But it can also occur in later versions.
                usleep(kDelayBeforeCloseMillis * 1000);
            }
            return static_cast<OboeResult>(mLibLoader->stream_close(stream));
        } else {
            return OboeResult::ErrorClosed;
        }
        */
    }
    
    /**
      | Launch a thread that will stop the stream.
      |
      */
    pub fn launch_stop_thread(&mut self)  {
        
        todo!();
        /*
            // Prevent multiple stop threads from being launched.
        if (mStopThreadAllowed.exchange(false)) {
            // Stop this stream on a separate thread
            std::thread t(oboe_stop_thread_proc, this);
            t.detach();
        }
        */
    }
    
    pub fn call_on_audio_ready(&mut self, 
        stream:     *mut AAudioStream,
        audio_data: *mut c_void,
        num_frames: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            DataCallbackResult result = fireDataCallback(audioData, numFrames);
        if (result == DataCallbackResult::Continue) {
            return result;
        } else {
            if (result == DataCallbackResult::Stop) {
                LOGD("Oboe callback returned DataCallbackResult::Stop");
            } else {
                LOGE("Oboe callback returned unexpected value = %d", result);
            }

            // Returning Stop caused various problems before S. See #1230
            if (OboeGlobals::areWorkaroundsEnabled() && getSdkVersion() <= __ANDROID_API_R__) {
                launchStopThread();
                return DataCallbackResult::Continue;
            } else {
                return DataCallbackResult::Stop; // OK >= API_S
            }
        }
        */
    }
    
    pub fn request_start(&mut self) -> OboeResult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            // Avoid state machine errors in O_MR1.
            if (getSdkVersion() <= __ANDROID_API_O_MR1__) {
                StreamState state = static_cast<StreamState>(mLibLoader->stream_getState(stream));
                if (state == StreamState::Starting || state == StreamState::Started) {
                    // WARNING: On P, AAudio is returning ErrorInvalidState for Output and OK for Input.
                    return OboeResult::OK;
                }
            }
            if (isDataCallbackSpecified()) {
                setDataCallbackEnabled(true);
            }
            mStopThreadAllowed = true;
            return static_cast<OboeResult>(mLibLoader->stream_requestStart(stream));
        } else {
            return OboeResult::ErrorClosed;
        }
        */
    }
    
    pub fn request_pause(&mut self) -> OboeResult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            // Avoid state machine errors in O_MR1.
            if (getSdkVersion() <= __ANDROID_API_O_MR1__) {
                StreamState state = static_cast<StreamState>(mLibLoader->stream_getState(stream));
                if (state == StreamState::Pausing || state == StreamState::Paused) {
                    return OboeResult::OK;
                }
            }
            return static_cast<OboeResult>(mLibLoader->stream_requestPause(stream));
        } else {
            return OboeResult::ErrorClosed;
        }
        */
    }
    
    pub fn request_flush(&mut self) -> OboeResult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            // Avoid state machine errors in O_MR1.
            if (getSdkVersion() <= __ANDROID_API_O_MR1__) {
                StreamState state = static_cast<StreamState>(mLibLoader->stream_getState(stream));
                if (state == StreamState::Flushing || state == StreamState::Flushed) {
                    return OboeResult::OK;
                }
            }
            return static_cast<OboeResult>(mLibLoader->stream_requestFlush(stream));
        } else {
            return OboeResult::ErrorClosed;
        }
        */
    }
    
    pub fn request_stop(&mut self) -> OboeResult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            return requestStop_l(stream);
        } else {
            return OboeResult::ErrorClosed;
        }
        */
    }

    /**
      | Call under mLock
      |
      | Must call under mLock. And stream must
      | NOT be nullptr.
      |
      */
    pub fn request_stop_l(&mut self, stream: *mut AAudioStream) -> OboeResult {
        
        todo!();
        /*
            // Avoid state machine errors in O_MR1.
        if (getSdkVersion() <= __ANDROID_API_O_MR1__) {
            StreamState state = static_cast<StreamState>(mLibLoader->stream_getState(stream));
            if (state == StreamState::Stopping || state == StreamState::Stopped) {
                return OboeResult::OK;
            }
        }
        return static_cast<OboeResult>(mLibLoader->stream_requestStop(stream));
        */
    }
    
    pub fn write(&mut self, 
        buffer:              *const c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            int32_t result = mLibLoader->stream_write(mAAudioStream, buffer,
                                                      numFrames, timeoutNanoseconds);
            return ResultWithValue<int32_t>::createBasedOnSign(result);
        } else {
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }
        */
    }
    
    pub fn read(&mut self, 
        buffer:              *mut c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            int32_t result = mLibLoader->stream_read(mAAudioStream, buffer,
                                                     numFrames, timeoutNanoseconds);
            return ResultWithValue<int32_t>::createBasedOnSign(result);
        } else {
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }
        */
    }

    /**
      | AAudioStream_waitForStateChange() can crash if
      | it is waiting on a stream and that stream is
      | closed from another thread.  We do not want to
      | lock the stream for the duration of the call.
      |
      | So we call AAudioStream_waitForStateChange()
      | with a timeout of zero so that it will not
      | block.
      |
      | Then we can do our own sleep with the lock
      | unlocked.
      |
      */
    pub fn wait_for_state_change(&mut self, 
        current_state:       OboeStreamState,
        next_state:          *mut OboeStreamState,
        timeout_nanoseconds: i64) -> OboeResult {
        
        todo!();
        /*
            OboeResult oboeResult = OboeResult::ErrorTimeout;
        int64_t sleepTimeNanos = 20 * kNanosPerMillisecond; // arbitrary
        aaudio_stream_state_t currentAAudioState = static_cast<aaudio_stream_state_t>(currentState);

        aaudio_result_t result = AAUDIO_OK;
        int64_t timeLeftNanos = timeoutNanoseconds;

        mLock.lock();
        while (true) {
            // Do we still have an AAudio stream? If not then stream must have been closed.
            AAudioStream *stream = mAAudioStream.load();
            if (stream == nullptr) {
                if (nextState != nullptr) {
                    *nextState = StreamState::Closed;
                }
                oboeResult = OboeResult::ErrorClosed;
                break;
            }

            // Update and query state change with no blocking.
            aaudio_stream_state_t aaudioNextState;
            result = mLibLoader->stream_waitForStateChange(
                    mAAudioStream,
                    currentAAudioState,
                    &aaudioNextState,
                    0); // timeout=0 for non-blocking
            // AAudio will return AAUDIO_ERROR_TIMEOUT if timeout=0 and the state does not change.
            if (result != AAUDIO_OK && result != AAUDIO_ERROR_TIMEOUT) {
                oboeResult = static_cast<OboeResult>(result);
                break;
            }
    #if OBOE_FIX_FORCE_STARTING_TO_STARTED
            if (OboeGlobals::areWorkaroundsEnabled()
                && aaudioNextState == static_cast<aaudio_stream_state_t >(StreamState::Starting)) {
                aaudioNextState = static_cast<aaudio_stream_state_t >(StreamState::Started);
            }
    #endif // OBOE_FIX_FORCE_STARTING_TO_STARTED
            if (nextState != nullptr) {
                *nextState = static_cast<StreamState>(aaudioNextState);
            }
            if (currentAAudioState != aaudioNextState) { // state changed?
                oboeResult = OboeResult::OK;
                break;
            }

            // Did we timeout or did user ask for non-blocking?
            if (timeLeftNanos <= 0) {
                break;
            }

            // No change yet so sleep.
            mLock.unlock(); // Don't sleep while locked.
            if (sleepTimeNanos > timeLeftNanos) {
                sleepTimeNanos = timeLeftNanos; // last little bit
            }
            AudioClock::sleepForNanos(sleepTimeNanos);
            timeLeftNanos -= sleepTimeNanos;
            mLock.lock();
        }

        mLock.unlock();
        return oboeResult;
        */
    }
    
    pub fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            int32_t adjustedFrames = requestedFrames;
        if (adjustedFrames > mBufferCapacityInFrames) {
            adjustedFrames = mBufferCapacityInFrames;
        }
        // This calls getBufferSize() so avoid recursive lock.
        adjustedFrames = QuirksManager::getInstance().clipBufferSize(*this, adjustedFrames);

        std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            int32_t newBufferSize = mLibLoader->stream_setBufferSize(mAAudioStream, adjustedFrames);
            // Cache the result if it's valid
            if (newBufferSize > 0) mBufferSizeInFrames = newBufferSize;
            return ResultWithValue<int32_t>::createBasedOnSign(newBufferSize);
        } else {
            return ResultWithValue<int32_t>(OboeResult::ErrorClosed);
        }
        */
    }
    
    pub fn get_state(&mut self) -> OboeStreamState {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            aaudio_stream_state_t aaudioState = mLibLoader->stream_getState(stream);
    #if OBOE_FIX_FORCE_STARTING_TO_STARTED
            if (OboeGlobals::areWorkaroundsEnabled()
                && aaudioState == AAUDIO_STREAM_STATE_STARTING) {
                aaudioState = AAUDIO_STREAM_STATE_STARTED;
            }
    #endif // OBOE_FIX_FORCE_STARTING_TO_STARTED
            return static_cast<StreamState>(aaudioState);
        } else {
            return StreamState::Closed;
        }
        */
    }
    
    pub fn get_buffer_size_in_frames(&mut self) -> i32 {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            mBufferSizeInFrames = mLibLoader->stream_getBufferSize(stream);
        }
        return mBufferSizeInFrames;
        */
    }
    
    pub fn update_frames_read(&mut self)  {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
    // Set to 1 for debugging race condition #1180 with mAAudioStream.
    // See also DEBUG_CLOSE_RACE in OboeTester.
    // This was left in the code so that we could test the fix again easily in the future.
    // We could not trigger the race condition without adding these get calls and the sleeps.
    #define DEBUG_CLOSE_RACE 0
    #if DEBUG_CLOSE_RACE
        // This is used when testing race conditions with close().
        // See DEBUG_CLOSE_RACE in OboeTester
        AudioClock::sleepForNanos(400 * kNanosPerMillisecond);
    #endif // DEBUG_CLOSE_RACE
        if (stream != nullptr) {
            mFramesRead = mLibLoader->stream_getFramesRead(stream);
        }
        */
    }
    
    pub fn update_frames_written(&mut self)  {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            mFramesWritten = mLibLoader->stream_getFramesWritten(stream);
        }
        */
    }
    
    pub fn get_xrun_count(&mut self) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            return ResultWithValue<int32_t>::createBasedOnSign(mLibLoader->stream_getXRunCount(stream));
        } else {
            return ResultWithValue<int32_t>(OboeResult::ErrorNull);
        }
        */
    }
    
    pub fn get_timestamp(&mut self, 
        clock_id:         ClockId,
        frame_position:   *mut i64,
        time_nanoseconds: *mut i64) -> OboeResult {
        
        todo!();
        /*
            if (getState() != StreamState::Started) {
            return OboeResult::ErrorInvalidState;
        }
        std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            return static_cast<OboeResult>(mLibLoader->stream_getTimestamp(stream, clockId,
                                                   framePosition, timeNanoseconds));
        } else {
            return OboeResult::ErrorNull;
        }
        */
    }
    
    pub fn calculate_latency_millis(&mut self) -> OboeResultWithValue<f64> {
        
        todo!();
        /*
            // Get the time that a known audio frame was presented.
        int64_t hardwareFrameIndex;
        int64_t hardwareFrameHardwareTime;
        auto result = getTimestamp(CLOCK_MONOTONIC,
                                   &hardwareFrameIndex,
                                   &hardwareFrameHardwareTime);
        if (result != OboeResult::OK) {
            return ResultWithValue<double>(static_cast<OboeResult>(result));
        }

        // Get counter closest to the app.
        bool isOutput = (getDirection() == OboeDirection::Output);
        int64_t appFrameIndex = isOutput ? getFramesWritten() : getFramesRead();

        // Assume that the next frame will be processed at the current time
        using namespace std::chrono;
        int64_t appFrameAppTime =
                duration_cast<nanoseconds>(steady_clock::now().time_since_epoch()).count();

        // Calculate the number of frames between app and hardware
        int64_t frameIndexDelta = appFrameIndex - hardwareFrameIndex;

        // Calculate the time which the next frame will be or was presented
        int64_t frameTimeDelta = (frameIndexDelta * OboekNanosPerSecond) / getSampleRate();
        int64_t appFrameHardwareTime = hardwareFrameHardwareTime + frameTimeDelta;

        // The current latency is the difference in time between when the current frame is at
        // the app and when it is at the hardware.
        double latencyNanos = static_cast<double>(isOutput
                              ? (appFrameHardwareTime - appFrameAppTime) // hardware is later
                              : (appFrameAppTime - appFrameHardwareTime)); // hardware is earlier
        double latencyMillis = latencyNanos / kNanosPerMillisecond;

        return ResultWithValue<double>(latencyMillis);
        */
    }
    
    pub fn is_mmap_used(&mut self) -> bool {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(mAAudioStreamLock);
        AAudioStream *stream = mAAudioStream.load();
        if (stream != nullptr) {
            return AAudioExtensions::getInstance().isMMapUsed(stream);
        } else {
            return false;
        }
        */
    }
}
