crate::ix!();

/**
   These will wind up in <SLES/OpenSLES_Android.h>
  */
pub const SL_ANDROID_SPEAKER_STEREO: i32 = SL_SPEAKER_FRONT_LEFT | SL_SPEAKER_FRONT_RIGHT;
pub const SL_ANDROID_SPEAKER_QUAD:   i32 = SL_ANDROID_SPEAKER_STEREO | SL_SPEAKER_BACK_LEFT | SL_SPEAKER_BACK_RIGHT;
pub const SL_ANDROID_SPEAKER_5DOT1:  i32 = SL_ANDROID_SPEAKER_QUAD | SL_SPEAKER_FRONT_CENTER  | SL_SPEAKER_LOW_FREQUENCY;
pub const SL_ANDROID_SPEAKER_7DOT1:  i32 = SL_ANDROID_SPEAKER_5DOT1 | SL_SPEAKER_SIDE_LEFT | SL_SPEAKER_SIDE_RIGHT;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioOutputStreamOpenSLES.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioOutputStreamOpenSLES.cpp]

pub fn opensles_convert_output_usage(oboe_usage: OboeUsage) -> u32 {
    
    todo!();
    /*
        u32 openslStream = SL_ANDROID_STREAM_MEDIA;
        switch(oboeUsage) {
            case Usage::Media:
                openslStream = SL_ANDROID_STREAM_MEDIA;
                break;
            case Usage::VoiceCommunication:
            case Usage::VoiceCommunicationSignalling:
                openslStream = SL_ANDROID_STREAM_VOICE;
                break;
            case Usage::Alarm:
                openslStream = SL_ANDROID_STREAM_ALARM;
                break;
            case Usage::Notification:
            case Usage::NotificationRingtone:
            case Usage::NotificationEvent:
                openslStream = SL_ANDROID_STREAM_NOTIFICATION;
                break;
            case Usage::AssistanceAccessibility:
            case Usage::AssistanceNavigationGuidance:
            case Usage::AssistanceSonification:
                openslStream = SL_ANDROID_STREAM_SYSTEM;
                break;
            case Usage::Game:
                openslStream = SL_ANDROID_STREAM_MEDIA;
                break;
            case Usage::Assistant:
            default:
                openslStream = SL_ANDROID_STREAM_SYSTEM;
                break;
        }
        return openslStream;
    */
}

pub struct OboeAudioOutputStreamOpenSLES<'a> {
    base:           OboeAudioStreamOpenSLES<'a>,
    play_interface: SLPlayItf, // default = nullptr
}

impl<'a> OboeAudioOutputStreamOpenSLES<'a> {
    
    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream_opensles(builder),

        
        */
    }
    
    pub fn channel_count_to_channel_mask(&self, channel_count: i32) -> u32 {
        
        todo!();
        /*
            u32 channelMask = 0;

        switch (channelCount) {
            case  1:
                channelMask = SL_SPEAKER_FRONT_CENTER;
                break;

            case  2:
                channelMask = SL_ANDROID_SPEAKER_STEREO;
                break;

            case  4: // Quad
                channelMask = SL_ANDROID_SPEAKER_QUAD;
                break;

            case  6: // 5.1
                channelMask = SL_ANDROID_SPEAKER_5DOT1;
                break;

            case  8: // 7.1
                channelMask = SL_ANDROID_SPEAKER_7DOT1;
                break;

            default:
                channelMask = channelCountToChannelMaskDefault(channelCount);
                break;
        }
        return channelMask;
        */
    }
    
    pub fn open(&mut self) -> OboeResult {
        
        todo!();
        /*
            logUnsupportedAttributes();

        SLAndroidConfigurationItf configItf = nullptr;

        if (getSdkVersion() < __ANDROID_API_L__ && mFormat == AudioFormat::Float){
            // TODO: Allow floating point format on API <21 using float->int16 converter
            return OboeResult::ErrorInvalidFormat;
        }

        // If audio format is unspecified then choose a suitable default.
        // API 21+: FLOAT
        // API <21: INT16
        if (mFormat == AudioFormat::Unspecified){
            mFormat = (getSdkVersion() < __ANDROID_API_L__) ?
                      AudioFormat::I16 : AudioFormat::Float;
        }

        OboeResult oboeResult = AudioStreamOpenSLES::open();
        if (OboeResult::OK != oboeResult)  return oboeResult;

        SLresult result = OutputMixerOpenSL::getInstance().open();
        if (SL_RESULT_SUCCESS != result) {
            AudioStreamOpenSLES::close();
            return OboeResult::ErrorInternal;
        }

        u32 bitsPerSample = static_cast<u32>(getBytesPerSample() * kBitsPerByte);

        // configure audio source
        SLDataLocator_AndroidSimpleBufferQueue loc_bufq = {
                SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE,    // locatorType
                static_cast<u32>(kBufferQueueLength)};   // numBuffers

        // Define the audio data format.
        SLDataFormat_PCM format_pcm = {
                SL_DATAFORMAT_PCM,       // formatType
                static_cast<u32>(mChannelCount),           // numChannels
                static_cast<u32>(mSampleRate * kMillisPerSecond),    // milliSamplesPerSec
                bitsPerSample,                      // bitsPerSample
                bitsPerSample,                      // containerSize;
                channelCountToChannelMask(mChannelCount), // channelMask
                getDefaultByteOrder(),
        };

        SLDataSource audioSrc = {&loc_bufq, &format_pcm};

        /**
         * API 21 (Lollipop) introduced support for floating-point data representation and an extended
         * data format type: SLAndroidDataFormat_PCM_EX. If running on API 21+ use this newer format
         * type, creating it from our original format.
         */
        SLAndroidDataFormat_PCM_EX format_pcm_ex;
        if (getSdkVersion() >= __ANDROID_API_L__) {
            u32 representation = OpenSLES_ConvertFormatToRepresentation(getFormat());
            // Fill in the format structure.
            format_pcm_ex = OpenSLES_createExtendedFormat(format_pcm, representation);
            // Use in place of the previous format.
            audioSrc.pFormat = &format_pcm_ex;
        }

        result = OutputMixerOpenSL::getInstance().createAudioPlayer(&mObjectInterface,
                                                                              &audioSrc);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("createAudioPlayer() result:%s", getSLErrStr(result));
            goto error;
        }

        // Configure the stream.
        result = (*mObjectInterface)->GetInterface(mObjectInterface,
                                                   SL_IID_ANDROIDCONFIGURATION,
                                                   (void *)&configItf);
        if (SL_RESULT_SUCCESS != result) {
            LOGW("%s() GetInterface(SL_IID_ANDROIDCONFIGURATION) failed with %s",
                 __func__, getSLErrStr(result));
        } else {
            result = configurePerformanceMode(configItf);
            if (SL_RESULT_SUCCESS != result) {
                goto error;
            }

            u32 presetValue = OpenSLES_convertOutputUsage(getUsage());
            result = (*configItf)->SetConfiguration(configItf,
                                                    SL_ANDROID_KEY_STREAM_TYPE,
                                                    &presetValue,
                                                    sizeof(presetValue));
            if (SL_RESULT_SUCCESS != result) {
                goto error;
            }
        }

        result = (*mObjectInterface)->Realize(mObjectInterface, SL_BOOLEAN_FALSE);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("Realize player object result:%s", getSLErrStr(result));
            goto error;
        }

        result = (*mObjectInterface)->GetInterface(mObjectInterface, SL_IID_PLAY, &mPlayInterface);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("GetInterface PLAY result:%s", getSLErrStr(result));
            goto error;
        }

        result = AudioStreamOpenSLES::registerBufferQueueCallback();
        if (SL_RESULT_SUCCESS != result) {
            goto error;
        }

        result = updateStreamParameters(configItf);
        if (SL_RESULT_SUCCESS != result) {
            goto error;
        }

        oboeResult = configureBufferSizes(mSampleRate);
        if (OboeResult::OK != oboeResult) {
            goto error;
        }

        allocateFifo();

        setState(StreamState::Open);
        return OboeResult::OK;

    error:
        return OboeResult::ErrorInternal; // TODO convert error from SLES to OBOE
        */
    }
    
    pub fn on_after_destroy(&mut self) -> OboeResult {
        
        todo!();
        /*
            OutputMixerOpenSL::getInstance().close();
        return OboeResult::OK;
        */
    }
    
    pub fn close(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES::%s()", __func__);
        std::lock_guard<std::mutex> lock(mLock);
        OboeResult result = OboeResult::OK;
        if (getState() == StreamState::Closed){
            result = OboeResult::ErrorClosed;
        } else {
            requestPause_l();
            // invalidate any interfaces
            mPlayInterface = nullptr;
            result = AudioStreamOpenSLES::close_l();
        }
        return result;
        */
    }
    
    /**
      | Set OpenSL ES PLAYSTATE.
      | 
      | -----------
      | @param newState
      | 
      | SL_PLAYSTATE_PAUSED, SL_PLAYSTATE_PLAYING,
      | SL_PLAYSTATE_STOPPED @return
      |
      */
    pub fn set_play_state_l(&mut self, new_state: u32) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES(): %s() called", __func__);
        OboeResult result = OboeResult::OK;

        if (mPlayInterface == nullptr){
            LOGE("OboeAudioOutputStreamOpenSLES::%s() mPlayInterface is null", __func__);
            return OboeResult::ErrorInvalidState;
        }

        SLresult slResult = (*mPlayInterface)->SetPlayState(mPlayInterface, newState);
        if (SL_RESULT_SUCCESS != slResult) {
            LOGW("OboeAudioOutputStreamOpenSLES(): %s() returned %s", __func__, getSLErrStr(slResult));
            result = OboeResult::ErrorInternal; // TODO convert slResult to OboeResult::Error
        }
        return result;
        */
    }
    
    pub fn request_start(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES(): %s() called", __func__);

        mLock.lock();
        StreamState initialState = getState();
        switch (initialState) {
            case StreamState::Starting:
            case StreamState::Started:
                mLock.unlock();
                return OboeResult::OK;
            case StreamState::Closed:
                mLock.unlock();
                return OboeResult::ErrorClosed;
            default:
                break;
        }

        // We use a callback if the user requests one
        // OR if we have an internal callback to read the blocking IO buffer.
        setDataCallbackEnabled(true);

        setState(StreamState::Starting);
        OboeResult result = setPlayState_l(SL_PLAYSTATE_PLAYING);
        if (result == OboeResult::OK) {
            setState(StreamState::Started);
            mLock.unlock();
            if (getBufferDepth(mSimpleBufferQueueInterface) == 0) {
                // Enqueue the first buffer if needed to start the streaming.
                // This might call requestStop() so try to avoid a recursive lock.
                processBufferCallback(mSimpleBufferQueueInterface);
            }
        } else {
            setState(initialState);
            mLock.unlock();
        }
        return result;
        */
    }
    
    pub fn request_pause(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES(): %s() called", __func__);
        std::lock_guard<std::mutex> lock(mLock);
        return requestPause_l();
        */
    }

    /**
       Call under mLock
      */
    pub fn request_pause_l(&mut self) -> OboeResult {
        
        todo!();
        /*
            StreamState initialState = getState();
        switch (initialState) {
            case StreamState::Pausing:
            case StreamState::Paused:
                return OboeResult::OK;
            case StreamState::Closed:
                return OboeResult::ErrorClosed;
            default:
                break;
        }

        setState(StreamState::Pausing);
        OboeResult result = setPlayState_l(SL_PLAYSTATE_PAUSED);
        if (result == OboeResult::OK) {
            // Note that OpenSL ES does NOT reset its millisecond position when OUTPUT is paused.
            int64_t framesWritten = getFramesWritten();
            if (framesWritten >= 0) {
                setFramesRead(framesWritten);
            }
            setState(StreamState::Paused);
        } else {
            setState(initialState);
        }
        return result;
        */
    }

    /**
      | Flush/clear the queue buffers
      |
      */
    pub fn request_flush(&mut self) -> OboeResult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        return requestFlush_l();
        */
    }
    
    pub fn request_flush_l(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES(): %s() called", __func__);
        if (getState() == StreamState::Closed) {
            return OboeResult::ErrorClosed;
        }

        OboeResult result = OboeResult::OK;
        if (mPlayInterface == nullptr || mSimpleBufferQueueInterface == nullptr) {
            result = OboeResult::ErrorInvalidState;
        } else {
            SLresult slResult = (*mSimpleBufferQueueInterface)->Clear(mSimpleBufferQueueInterface);
            if (slResult != SL_RESULT_SUCCESS){
                LOGW("Failed to clear buffer queue. OpenSLES error: %d", result);
                result = OboeResult::ErrorInternal;
            }
        }
        return result;
        */
    }
    
    pub fn request_stop(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioOutputStreamOpenSLES(): %s() called", __func__);
        std::lock_guard<std::mutex> lock(mLock);

        StreamState initialState = getState();
        switch (initialState) {
            case StreamState::Stopping:
            case StreamState::Stopped:
                return OboeResult::OK;
            case StreamState::Closed:
                return OboeResult::ErrorClosed;
            default:
                break;
        }

        setState(StreamState::Stopping);

        OboeResult result = setPlayState_l(SL_PLAYSTATE_STOPPED);
        if (result == OboeResult::OK) {

            // Also clear the buffer queue so the old data won't be played if the stream is restarted.
            // Call the _l function that expects to already be under a lock.
            if (requestFlush_l() != OboeResult::OK) {
                LOGW("Failed to flush the stream. Error %s", convertToText(flush()));
            }

            mPositionMillis.reset32(); // OpenSL ES resets its millisecond position when stopped.
            int64_t framesWritten = getFramesWritten();
            if (framesWritten >= 0) {
                setFramesRead(framesWritten);
            }
            setState(StreamState::Stopped);
        } else {
            setState(initialState);
        }
        return result;
        */
    }
    
    pub fn set_frames_read(&mut self, frames_read: i64)  {
        
        todo!();
        /*
            int64_t millisWritten = framesRead * kMillisPerSecond / getSampleRate();
        mPositionMillis.set(millisWritten);
        */
    }
    
    pub fn update_frames_read(&mut self)  {
        
        todo!();
        /*
            if (usingFIFO()) {
            AudioStreamBuffered::updateFramesRead();
        } else {
            mFramesRead = getFramesProcessedByServer();
        }
        */
    }
    
    pub fn update_service_frame_counter(&mut self) -> OboeResult {
        
        todo!();
        /*
            OboeResult result = OboeResult::OK;
        // Avoid deadlock if another thread is trying to stop or close this stream
        // and this is being called from a callback.
        if (mLock.try_lock()) {

            if (mPlayInterface == nullptr) {
                mLock.unlock();
                return OboeResult::ErrorNull;
            }
            SLmillisecond msec = 0;
            SLresult slResult = (*mPlayInterface)->GetPosition(mPlayInterface, &msec);
            if (SL_RESULT_SUCCESS != slResult) {
                LOGW("%s(): GetPosition() returned %s", __func__, getSLErrStr(slResult));
                // set result based on SLresult
                result = OboeResult::ErrorInternal;
            } else {
                mPositionMillis.update32(msec);
            }
            mLock.unlock();
        }
        return result;
        */
    }
}
