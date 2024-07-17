crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioStreamOpenSLES.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioStreamOpenSLES.cpp]

pub fn is_little_endian() -> bool {
    
    todo!();
    /*
        static uint32_t value = 1;
        return (*reinterpret_cast<uint8_t *>(&value) == 1);  // Does address point to LSB?
    */
}

/**
  | This callback handler is called every
  | time a buffer has been processed by OpenSL
  | ES.
  |
  */
pub fn bq_callback_glue(
        bq:      SLAndroidSimpleBufferQueueItf,
        context: *mut c_void)  {
    
    todo!();
    /*
        (reinterpret_cast<OboeAudioStreamOpenSLES *>(context))->processBufferCallback(bq);
    */
}

pub const kBitsPerByte:       i32 = 8;
pub const kBufferQueueLength: i32 = 2; // double buffered for callbacks

pub const kHighLatencyBufferSizeMillis:        i32 = 20; // typical Android period
pub const kAudioChannelCountMax:          u32 = 30; // TODO Why 30?
pub const SL_ANDROID_UNKNOWN_CHANNELMASK: u32 = 0; // Matches name used internally.

/**
  | INTERNAL USE ONLY
  | 
  | A stream that wraps OpenSL ES.
  | 
  | Do not instantiate this class directly.
  | Use an OboeStreamBuilder to create
  | one.
  |
  */
pub struct OboeAudioStreamOpenSLES<'a> {


    /**
      | OpenSLES stuff
      |
      */
    object_interface:              SLObjectItf, // default = nullptr

    simple_buffer_queue_interface: SLAndroidSimpleBufferQueueItf, // default = nullptr
    bytes_per_callback:            i32, // default = OboekUnspecified

    /**
      | for tracking OpenSL ES service position
      |
      */
    position_millis:               MonotonicCounter,

    callback_buffer:               Box<&'a [u8]>,
    state:                         Atomic<OboeStreamState>, // default = StreamState::Uninitialized
}

impl<'a> OboeAudioStreamBufferedInterface for OboeAudioStreamOpenSLES<'a> {

    fn update_service_frame_counter(&mut self) -> OboeResult
    {
        todo!();
    }
}

impl<'a> OboeAudioStreamOpenSLES<'a> {
    
    /**
      | Query the current state, eg. OBOE_STREAM_STATE_PAUSING
      | 
      | 
      | -----------
      | @return
      | 
      | state or a negative error.
      |
      */
    pub fn get_state(&mut self) -> OboeStreamState {
        
        todo!();
        /*
            return mState.load();
        */
    }
    
    pub fn get_audio_api(&self) -> OboeAudioApi {
        
        todo!();
        /*
            return AudioApi::OpenSLES;
        */
    }

    pub fn on_before_destroy(&mut self) -> OboeResult {
        
        todo!();
        /*
            return OboeResult::OK;
        */
    }
    
    pub fn on_after_destroy(&mut self) -> OboeResult {
        
        todo!();
        /*
            return OboeResult::OK;
        */
    }

    /**
      | Internal use only.
      | 
      | Use this instead of directly setting
      | the internal state variable.
      |
      */
    pub fn set_state(&mut self, state: OboeStreamState)  {
        
        todo!();
        /*
            mState.store(state);
        */
    }
    
    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream_buffered(builder),

            // OpenSL ES does not support device IDs. So overwrite value from builder.
        mDeviceId = kUnspecified;
        // OpenSL ES does not support session IDs. So overwrite value from builder.
        mSessionId = SessionId::None;
        */
    }
    
    pub fn channel_count_to_channel_mask_default(&self, channel_count: i32) -> u32 {
        
        todo!();
        /*
            if (channelCount > kAudioChannelCountMax) {
            return SL_ANDROID_UNKNOWN_CHANNELMASK;
        }

        u32 bitfield = (1 << channelCount) - 1;

        // Check for OS at run-time.
        if(getSdkVersion() >= __ANDROID_API_N__) {
            return SL_ANDROID_MAKE_INDEXED_CHANNEL_MASK(bitfield);
        }

        // Indexed channels masks were added in N.
        // For before N, the best we can do is use a positional channel mask.
        return bitfield;
        */
    }
    
    pub fn get_default_byte_order(&mut self) -> u32 {
        
        todo!();
        /*
            return s_isLittleEndian() ? SL_BYTEORDER_LITTLEENDIAN : SL_BYTEORDER_BIGENDIAN;
        */
    }
    
    pub fn open(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGI("OboeAudioStreamOpenSLES::open() chans=%d, rate=%d", mChannelCount, mSampleRate);

        SLresult result = EngineOpenSLES::getInstance().open();
        if (SL_RESULT_SUCCESS != result) {
            return OboeResult::ErrorInternal;
        }

        OboeResult oboeResult = AudioStreamBuffered::open();
        if (oboeResult != OboeResult::OK) {
            return oboeResult;
        }
        // Convert to defaults if UNSPECIFIED
        if (mSampleRate == kUnspecified) {
            mSampleRate = DefaultStreamValues::SampleRate;
        }
        if (mChannelCount == kUnspecified) {
            mChannelCount = DefaultStreamValues::ChannelCount;
        }

        mSharingMode = SharingMode::Shared;

        return OboeResult::OK;
        */
    }
    
    pub fn configure_buffer_sizes(&mut self, sample_rate: i32) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioStreamOpenSLES:%s(%d) initial mFramesPerBurst = %d, mFramesPerCallback = %d",
                __func__, sampleRate, mFramesPerBurst, mFramesPerCallback);
        // Decide frames per burst based on hints from caller.
        if (mFramesPerCallback != kUnspecified) {
            // Requested framesPerCallback must be honored.
            mFramesPerBurst = mFramesPerCallback;
        } else {
            mFramesPerBurst = DefaultStreamValues::FramesPerBurst;

            // Calculate the size of a fixed duration high latency buffer based on sample rate.
            int32_t framesPerHighLatencyBuffer =
                    (kHighLatencyBufferSizeMillis * sampleRate) / kMillisPerSecond;

            // For high latency streams, use a larger buffer size.
            // Performance Mode support was added in N_MR1 (7.1)
            if (getSdkVersion() >= __ANDROID_API_N_MR1__
                && mPerformanceMode != PerformanceMode::LowLatency
                && mFramesPerBurst < framesPerHighLatencyBuffer) {
                // Find a multiple of framesPerBurst >= framesPerHighLatencyBuffer.
                int32_t numBursts = (framesPerHighLatencyBuffer + mFramesPerBurst - 1) / mFramesPerBurst;
                mFramesPerBurst *= numBursts;
                LOGD("OboeAudioStreamOpenSLES:%s() NOT low latency, set mFramesPerBurst = %d",
                     __func__, mFramesPerBurst);
            }
            mFramesPerCallback = mFramesPerBurst;
        }
        LOGD("OboeAudioStreamOpenSLES:%s(%d) final mFramesPerBurst = %d, mFramesPerCallback = %d",
             __func__, sampleRate, mFramesPerBurst, mFramesPerCallback);

        mBytesPerCallback = mFramesPerCallback * getBytesPerFrame();
        if (mBytesPerCallback <= 0) {
            LOGE("OboeAudioStreamOpenSLES::open() bytesPerCallback < 0 = %d, bad format?",
                 mBytesPerCallback);
            return OboeResult::ErrorInvalidFormat; // causing bytesPerFrame == 0
        }

        mCallbackBuffer = std::make_unique<uint8_t[]>(mBytesPerCallback);

        if (!usingFIFO()) {
            mBufferCapacityInFrames = mFramesPerBurst * kBufferQueueLength;
            // Check for overflow.
            if (mBufferCapacityInFrames <= 0) {
                mBufferCapacityInFrames = 0;
                LOGE("OboeAudioStreamOpenSLES::open() numeric overflow because mFramesPerBurst = %d",
                     mFramesPerBurst);
                return OboeResult::ErrorOutOfRange;
            }
            mBufferSizeInFrames = mBufferCapacityInFrames;
        }

        return OboeResult::OK;
        */
    }
    
    pub fn convert_performance_mode(&self, oboe_mode: OboePerformanceMode) -> u32 {
        
        todo!();
        /*
            u32 openslMode = SL_ANDROID_PERFORMANCE_NONE;
        switch(oboeMode) {
            case PerformanceMode::None:
                openslMode =  SL_ANDROID_PERFORMANCE_NONE;
                break;
            case PerformanceMode::LowLatency:
                openslMode =  (getSessionId() == SessionId::None) ?  SL_ANDROID_PERFORMANCE_LATENCY : SL_ANDROID_PERFORMANCE_LATENCY_EFFECTS;
                break;
            case PerformanceMode::PowerSaving:
                openslMode =  SL_ANDROID_PERFORMANCE_POWER_SAVING;
                break;
            default:
                break;
        }
        return openslMode;
        */
    }
    
    pub fn convert_performance_mode_with_opensl_mode(&self, opensl_mode: u32) -> OboePerformanceMode {
        
        todo!();
        /*
            PerformanceMode oboeMode = PerformanceMode::None;
        switch(openslMode) {
            case SL_ANDROID_PERFORMANCE_NONE:
                oboeMode =  PerformanceMode::None;
                break;
            case SL_ANDROID_PERFORMANCE_LATENCY:
            case SL_ANDROID_PERFORMANCE_LATENCY_EFFECTS:
                oboeMode =  PerformanceMode::LowLatency;
                break;
            case SL_ANDROID_PERFORMANCE_POWER_SAVING:
                oboeMode =  PerformanceMode::PowerSaving;
                break;
            default:
                break;
        }
        return oboeMode;
        */
    }
    
    pub fn log_unsupported_attributes(&mut self)  {
        
        todo!();
        /*
            // Log unsupported attributes
        // only report if changed from the default

        // Device ID
        if (mDeviceId != kUnspecified) {
            LOGW("Device ID [AudioStreamBuilder::setDeviceId()] "
                 "is not supported on OpenSLES streams.");
        }
        // Sharing Mode
        if (mSharingMode != SharingMode::Shared) {
            LOGW("SharingMode [AudioStreamBuilder::setSharingMode()] "
                 "is not supported on OpenSLES streams.");
        }
        // Performance Mode
        int sdkVersion = getSdkVersion();
        if (mPerformanceMode != PerformanceMode::None && sdkVersion < __ANDROID_API_N_MR1__) {
            LOGW("PerformanceMode [AudioStreamBuilder::setPerformanceMode()] "
                 "is not supported on OpenSLES streams running on pre-Android N-MR1 versions.");
        }
        // Content Type
        if (mContentType != ContentType::Music) {
            LOGW("ContentType [AudioStreamBuilder::setContentType()] "
                 "is not supported on OpenSLES streams.");
        }

        // Session Id
        if (mSessionId != SessionId::None) {
            LOGW("SessionId [AudioStreamBuilder::setSessionId()] "
                 "is not supported on OpenSLES streams.");
        }
        */
    }
    
    pub fn configure_performance_mode(&mut self, config_itf: SLAndroidConfigurationItf) -> SLresult {
        
        todo!();
        /*
            if (configItf == nullptr) {
            LOGW("%s() called with NULL configuration", __func__);
            mPerformanceMode = PerformanceMode::None;
            return SL_RESULT_INTERNAL_ERROR;
        }
        if (getSdkVersion() < __ANDROID_API_N_MR1__) {
            LOGW("%s() not supported until N_MR1", __func__);
            mPerformanceMode = PerformanceMode::None;
            return SL_RESULT_SUCCESS;
        }

        SLresult result = SL_RESULT_SUCCESS;
        u32 performanceMode = convertPerformanceMode(getPerformanceMode());
        result = (*configItf)->SetConfiguration(configItf, SL_ANDROID_KEY_PERFORMANCE_MODE,
                                                         &performanceMode, sizeof(performanceMode));
        if (SL_RESULT_SUCCESS != result) {
            LOGW("SetConfiguration(PERFORMANCE_MODE, SL %u) returned %s",
                 performanceMode, getSLErrStr(result));
            mPerformanceMode = PerformanceMode::None;
        }

        return result;
        */
    }
    
    pub fn update_stream_parameters(&mut self, config_itf: SLAndroidConfigurationItf) -> SLresult {
        
        todo!();
        /*
            SLresult result = SL_RESULT_SUCCESS;
        if(getSdkVersion() >= __ANDROID_API_N_MR1__ && configItf != nullptr) {
            u32 performanceMode = 0;
            u32 performanceModeSize = sizeof(performanceMode);
            result = (*configItf)->GetConfiguration(configItf, SL_ANDROID_KEY_PERFORMANCE_MODE,
                                                    &performanceModeSize, &performanceMode);
            // A bug in GetConfiguration() before P caused a wrong result code to be returned.
            if (getSdkVersion() <= __ANDROID_API_O_MR1__) {
                result = SL_RESULT_SUCCESS; // Ignore actual result before P.
            }

            if (SL_RESULT_SUCCESS != result) {
                LOGW("GetConfiguration(SL_ANDROID_KEY_PERFORMANCE_MODE) returned %d", result);
                mPerformanceMode = PerformanceMode::None; // If we can't query it then assume None.
            } else {
                mPerformanceMode = convertPerformanceMode(performanceMode); // convert SL to Oboe mode
            }
        } else {
            mPerformanceMode = PerformanceMode::None; // If we can't query it then assume None.
        }
        return result;
        */
    }

    /**
      | This must be called under mLock.
      |
      */
    pub fn close_l(&mut self) -> OboeResult {
        
        todo!();
        /*
            if (mState == StreamState::Closed) {
            return OboeResult::ErrorClosed;
        }

        AudioStreamBuffered::close();

        onBeforeDestroy();

        if (mObjectInterface != nullptr) {
            (*mObjectInterface)->Destroy(mObjectInterface);
            mObjectInterface = nullptr;
        }

        onAfterDestroy();

        mSimpleBufferQueueInterface = nullptr;
        EngineOpenSLES::getInstance().close();

        setState(StreamState::Closed);
        return OboeResult::OK;
        */
    }
    
    pub fn enqueue_callback_buffer(&mut self, bq: SLAndroidSimpleBufferQueueItf) -> SLresult {
        
        todo!();
        /*
            return (*bq)->Enqueue(bq, mCallbackBuffer.get(), mBytesPerCallback);
        */
    }
    
    pub fn get_buffer_depth(&mut self, bq: SLAndroidSimpleBufferQueueItf) -> i32 {
        
        todo!();
        /*
            SLAndroidSimpleBufferQueueState queueState;
        SLresult result = (*bq)->GetState(bq, &queueState);
        return (result == SL_RESULT_SUCCESS) ? queueState.count : -1;
        */
    }
    
    /**
      | Process next OpenSL ES buffer.
      | 
      | Called by by OpenSL ES framework.
      | 
      | This is public, but don't call it directly.
      |
      */
    pub fn process_buffer_callback(&mut self, bq: SLAndroidSimpleBufferQueueItf)  {
        
        todo!();
        /*
            bool stopStream = false;
        // Ask the app callback to process the buffer.
        DataCallbackResult result = fireDataCallback(mCallbackBuffer.get(), mFramesPerCallback);
        if (result == DataCallbackResult::Continue) {
            // Pass the buffer to OpenSLES.
            SLresult enqueueResult = enqueueCallbackBuffer(bq);
            if (enqueueResult != SL_RESULT_SUCCESS) {
                LOGE("%s() returned %d", __func__, enqueueResult);
                stopStream = true;
            }
            // Update Oboe client position with frames handled by the callback.
            if (getDirection() == Direction::Input) {
                mFramesRead += mFramesPerCallback;
            } else {
                mFramesWritten += mFramesPerCallback;
            }
        } else if (result == DataCallbackResult::Stop) {
            LOGD("Oboe callback returned Stop");
            stopStream = true;
        } else {
            LOGW("Oboe callback returned unexpected value = %d", result);
            stopStream = true;
        }
        if (stopStream) {
            requestStop();
        }
        */
    }
    
    pub fn register_buffer_queue_callback(&mut self) -> SLresult {
        
        todo!();
        /*
            // The BufferQueue
        SLresult result = (*mObjectInterface)->GetInterface(mObjectInterface, SL_IID_ANDROIDSIMPLEBUFFERQUEUE,
                                                    &mSimpleBufferQueueInterface);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("get buffer queue interface:%p result:%s",
                 mSimpleBufferQueueInterface,
                 getSLErrStr(result));
        } else {
            // Register the BufferQueue callback
            result = (*mSimpleBufferQueueInterface)->RegisterCallback(mSimpleBufferQueueInterface,
                                                                      bqCallbackGlue, this);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("RegisterCallback result:%s", getSLErrStr(result));
            }
        }
        return result;
        */
    }
    
    pub fn get_frames_processed_by_server(&mut self) -> i64 {
        
        todo!();
        /*
            updateServiceFrameCounter();
        int64_t millis64 = mPositionMillis.get();
        int64_t framesProcessed = millis64 * getSampleRate() / kMillisPerSecond;
        return framesProcessed;
        */
    }
    
    pub fn wait_for_state_change(&mut self, 
        current_state:       OboeStreamState,
        next_state:          *mut OboeStreamState,
        timeout_nanoseconds: i64) -> OboeResult {
        
        todo!();
        /*
            OboeResult oboeResult = OboeResult::ErrorTimeout;
        int64_t sleepTimeNanos = 20 * kNanosPerMillisecond; // arbitrary
        int64_t timeLeftNanos = timeoutNanoseconds;

        while (true) {
            const StreamState state = getState(); // this does not require a lock
            if (nextState != nullptr) {
                *nextState = state;
            }
            if (currentState != state) { // state changed?
                oboeResult = OboeResult::OK;
                break;
            }

            // Did we timeout or did user ask for non-blocking?
            if (timeLeftNanos <= 0) {
                break;
            }

            if (sleepTimeNanos > timeLeftNanos){
                sleepTimeNanos = timeLeftNanos;
            }
            AudioClock::sleepForNanos(sleepTimeNanos);
            timeLeftNanos -= sleepTimeNanos;
        }

        return oboeResult;
        */
    }
}
