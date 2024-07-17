crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioInputStreamOpenSLES.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeAudioInputStreamOpenSLES.cpp]

pub struct OboeAudioInputStreamOpenSLES<'a> {
    base:             OboeAudioStreamOpenSLES<'a>,
    record_interface: SLRecordItf, // default = nullptr
}

impl<'a> OboeAudioInputStreamOpenSLES<'a> {

    pub fn new(builder: &AudioStreamBuilder) -> Self {
    
        todo!();
        /*
        : audio_stream_opensles(builder),

        
        */
    }

    /**
       Calculate masks specific to INPUT streams.
      */
    pub fn channel_count_to_channel_mask(&self, channel_count: i32) -> u32 {
        
        todo!();
        /*
            // Derived from internal sles_channel_in_mask_from_count(chanCount);
        // in "frameworks/wilhelm/src/android/channels.cpp".
        // Yes, it seems strange to use SPEAKER constants to describe inputs.
        // But that is how OpenSL ES does it internally.
        switch (channelCount) {
            case 1:
                return SL_SPEAKER_FRONT_LEFT;
            case 2:
                return SL_SPEAKER_FRONT_LEFT | SL_SPEAKER_FRONT_RIGHT;
            default:
                return channelCountToChannelMaskDefault(channelCount);
        }
        */
    }
    
    pub fn open(&mut self) -> OboeResult {
        
        todo!();
        /*
            logUnsupportedAttributes();

        SLAndroidConfigurationItf configItf = nullptr;

        if (getSdkVersion() < __ANDROID_API_M__ && mFormat == AudioFormat::Float){
            // TODO: Allow floating point format on API <23 using float->int16 converter
            return OboeResult::ErrorInvalidFormat;
        }

        // If audio format is unspecified then choose a suitable default.
        // API 23+: FLOAT
        // API <23: INT16
        if (mFormat == AudioFormat::Unspecified){
            mFormat = (getSdkVersion() < __ANDROID_API_M__) ?
                      AudioFormat::I16 : AudioFormat::Float;
        }

        OboeResult oboeResult = AudioStreamOpenSLES::open();
        if (OboeResult::OK != oboeResult) return oboeResult;

        u32 bitsPerSample = static_cast<u32>(getBytesPerSample() * kBitsPerByte);

        // configure audio sink
        SLDataLocator_AndroidSimpleBufferQueue loc_bufq = {
                SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE,    // locatorType
                static_cast<u32>(kBufferQueueLength)};   // numBuffers

        // Define the audio data format.
        SLDataFormat_PCM format_pcm = {
                SL_DATAFORMAT_PCM,       // formatType
                static_cast<u32>(mChannelCount),           // numChannels
                static_cast<u32>(mSampleRate * kMillisPerSecond), // milliSamplesPerSec
                bitsPerSample,                      // bitsPerSample
                bitsPerSample,                      // containerSize;
                channelCountToChannelMask(mChannelCount), // channelMask
                getDefaultByteOrder(),
        };

        SLDataSink audioSink = {&loc_bufq, &format_pcm};

        /**
         * API 23 (Marshmallow) introduced support for floating-point data representation and an
         * extended data format type: SLAndroidDataFormat_PCM_EX for recording streams (playback streams
         * got this in API 21). If running on API 23+ use this newer format type, creating it from our
         * original format.
         */
        SLAndroidDataFormat_PCM_EX format_pcm_ex;
        if (getSdkVersion() >= __ANDROID_API_M__) {
            u32 representation = OpenSLES_ConvertFormatToRepresentation(getFormat());
            // Fill in the format structure.
            format_pcm_ex = OpenSLES_createExtendedFormat(format_pcm, representation);
            // Use in place of the previous format.
            audioSink.pFormat = &format_pcm_ex;
        }

        // configure audio source
        SLDataLocator_IODevice loc_dev = {SL_DATALOCATOR_IODEVICE,
                                          SL_IODEVICE_AUDIOINPUT,
                                          SL_DEFAULTDEVICEID_AUDIOINPUT,
                                          NULL};
        SLDataSource audioSrc = {&loc_dev, NULL};

        SLresult result = EngineOpenSLES::getInstance().createAudioRecorder(&mObjectInterface,
                                                                            &audioSrc,
                                                                            &audioSink);

        if (SL_RESULT_SUCCESS != result) {
            LOGE("createAudioRecorder() result:%s", getSLErrStr(result));
            goto error;
        }

        // Configure the stream.
        result = (*mObjectInterface)->GetInterface(mObjectInterface,
                                                SL_IID_ANDROIDCONFIGURATION,
                                                &configItf);

        if (SL_RESULT_SUCCESS != result) {
            LOGW("%s() GetInterface(SL_IID_ANDROIDCONFIGURATION) failed with %s",
                 __func__, getSLErrStr(result));
        } else {
            if (getInputPreset() == InputPreset::VoicePerformance) {
                LOGD("OpenSL ES does not support InputPreset::VoicePerformance. Use VoiceRecognition.");
                mInputPreset = InputPreset::VoiceRecognition;
            }
            u32 presetValue = OpenSLES_convertInputPreset(getInputPreset());
            result = (*configItf)->SetConfiguration(configItf,
                                             SL_ANDROID_KEY_RECORDING_PRESET,
                                             &presetValue,
                                             sizeof(u32));
            if (SL_RESULT_SUCCESS != result
                    && presetValue != SL_ANDROID_RECORDING_PRESET_VOICE_RECOGNITION) {
                presetValue = SL_ANDROID_RECORDING_PRESET_VOICE_RECOGNITION;
                LOGD("Setting InputPreset %d failed. Using VoiceRecognition instead.", getInputPreset());
                mInputPreset = InputPreset::VoiceRecognition;
                (*configItf)->SetConfiguration(configItf,
                                                 SL_ANDROID_KEY_RECORDING_PRESET,
                                                 &presetValue,
                                                 sizeof(u32));
            }

            result = configurePerformanceMode(configItf);
            if (SL_RESULT_SUCCESS != result) {
                goto error;
            }
        }

        result = (*mObjectInterface)->Realize(mObjectInterface, SL_BOOLEAN_FALSE);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("Realize recorder object result:%s", getSLErrStr(result));
            goto error;
        }

        result = (*mObjectInterface)->GetInterface(mObjectInterface, SL_IID_RECORD, &mRecordInterface);
        if (SL_RESULT_SUCCESS != result) {
            LOGE("GetInterface RECORD result:%s", getSLErrStr(result));
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
    
    pub fn close(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioInputStreamOpenSLES::%s()", __func__);
        std::lock_guard<std::mutex> lock(mLock);
        OboeResult result = OboeResult::OK;
        if (getState() == StreamState::Closed){
            result = OboeResult::ErrorClosed;
        } else {
            requestStop_l();
            // invalidate any interfaces
            mRecordInterface = nullptr;
            result = AudioStreamOpenSLES::close_l();
        }
        return result;
        */
    }
    
    pub fn set_record_state_l(&mut self, new_state: u32) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioInputStreamOpenSLES::%s(%u)", __func__, newState);
        OboeResult result = OboeResult::OK;

        if (mRecordInterface == nullptr) {
            LOGE("OboeAudioInputStreamOpenSLES::%s() mRecordInterface is null", __func__);
            return OboeResult::ErrorInvalidState;
        }
        SLresult slResult = (*mRecordInterface)->SetRecordState(mRecordInterface, newState);
        //LOGD("OboeAudioInputStreamOpenSLES::%s(%u) returned %u", __func__, newState, slResult);
        if (SL_RESULT_SUCCESS != slResult) {
            LOGE("OboeAudioInputStreamOpenSLES::%s(%u) returned error %s",
                    __func__, newState, getSLErrStr(slResult));
            result = OboeResult::ErrorInternal; // TODO review
        }
        return result;
        */
    }
    
    pub fn request_start(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioInputStreamOpenSLES(): %s() called", __func__);
        std::lock_guard<std::mutex> lock(mLock);
        StreamState initialState = getState();
        switch (initialState) {
            case StreamState::Starting:
            case StreamState::Started:
                return OboeResult::OK;
            case StreamState::Closed:
                return OboeResult::ErrorClosed;
            default:
                break;
        }

        // We use a callback if the user requests one
        // OR if we have an internal callback to fill the blocking IO buffer.
        setDataCallbackEnabled(true);

        setState(StreamState::Starting);
        OboeResult result = setRecordState_l(SL_RECORDSTATE_RECORDING);
        if (result == OboeResult::OK) {
            setState(StreamState::Started);
            // Enqueue the first buffer to start the streaming.
            // This does not call the callback function.
            enqueueCallbackBuffer(mSimpleBufferQueueInterface);
        } else {
            setState(initialState);
        }
        return result;
        */
    }
    
    pub fn request_pause(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGW("OboeAudioInputStreamOpenSLES::%s() is intentionally not implemented for input "
             "streams", __func__);
        return OboeResult::ErrorUnimplemented; // Matches AAudio behavior.
        */
    }
    
    pub fn request_flush(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGW("OboeAudioInputStreamOpenSLES::%s() is intentionally not implemented for input "
             "streams", __func__);
        return OboeResult::ErrorUnimplemented; // Matches AAudio behavior.
        */
    }

    pub fn request_stop(&mut self) -> OboeResult {
        
        todo!();
        /*
            LOGD("OboeAudioInputStreamOpenSLES(): %s() called", __func__);
        std::lock_guard<std::mutex> lock(mLock);
        return requestStop_l();
        */
    }

    /**
       Call under mLock
      */
    pub fn request_stop_l(&mut self) -> OboeResult {
        
        todo!();
        /*
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

        OboeResult result = setRecordState_l(SL_RECORDSTATE_STOPPED);
        if (result == OboeResult::OK) {
            mPositionMillis.reset32(); // OpenSL ES resets its millisecond position when stopped.
            setState(StreamState::Stopped);
        } else {
            setState(initialState);
        }
        return result;
        */
    }

    pub fn update_frames_written(&mut self)  {
        
        todo!();
        /*
            if (usingFIFO()) {
            AudioStreamBuffered::updateFramesWritten();
        } else {
            mFramesWritten = getFramesProcessedByServer();
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

            if (mRecordInterface == nullptr) {
                mLock.unlock();
                return OboeResult::ErrorNull;
            }
            SLmillisecond msec = 0;
            SLresult slResult = (*mRecordInterface)->GetPosition(mRecordInterface, &msec);
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


pub fn opensles_convert_input_preset(oboe_preset: OboeInputPreset) -> u32 {
    
    todo!();
    /*
        u32 openslPreset = SL_ANDROID_RECORDING_PRESET_NONE;
        switch(oboePreset) {
            case InputPreset::Generic:
                openslPreset =  SL_ANDROID_RECORDING_PRESET_GENERIC;
                break;
            case InputPreset::Camcorder:
                openslPreset =  SL_ANDROID_RECORDING_PRESET_CAMCORDER;
                break;
            case InputPreset::VoiceRecognition:
            case InputPreset::VoicePerformance:
                openslPreset =  SL_ANDROID_RECORDING_PRESET_VOICE_RECOGNITION;
                break;
            case InputPreset::VoiceCommunication:
                openslPreset =  SL_ANDROID_RECORDING_PRESET_VOICE_COMMUNICATION;
                break;
            case InputPreset::Unprocessed:
                openslPreset =  SL_ANDROID_RECORDING_PRESET_UNPROCESSED;
                break;
            default:
                break;
        }
        return openslPreset;
    */
}
