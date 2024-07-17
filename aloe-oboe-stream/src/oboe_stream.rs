crate::ix!();

lazy_static!{
    /*
    static const char* const oboeTypeName;
    const char* const OboeAudioIODevice::oboeTypeName = "Android Oboe";
    */
}
    
pub struct OboeStream {

    stream:              *mut OboeAudioStream, // default = nullptr

    #[cfg(ALOE_USE_ANDROID_OBOE_STABILIZED_CALLBACK)]
    stabilized_callback: Box<OboeStabilizedCallback>,

    open_result:         OboeResult,
}

impl Drop for OboeStream {

    fn drop(&mut self) {
        todo!();
        /* 
                close();
                delete stream;
             */
    }
}

impl OboeStream {

    pub fn new(
        device_id:      i32,
        direction:      OboeDirection,
        sharing_mode:   OboeSharingMode,
        channel_count:  i32,
        format:         OboeAudioFormat,
        sample_rate_in: i32,
        buffer_size:    i32,
        callback_in:    *mut OboeAudioStreamCallback) -> Self {

        todo!();
        /*


            open (deviceId, direction, sharingMode, channelCount,
                      format, sampleRateIn, bufferSize, callbackIn);
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return openResult == OboeResult::OK;
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            jassert (openedOk());

                if (openedOk() && stream != nullptr)
                {
                    auto expectedState = OboeStreamState::Starting;
                    auto nextState = OboeStreamState::Started;
                    int64 timeoutNanos = 1000 * OboekNanosPerMillisecond;

                    auto startResult = stream->requestStart();
                    ALOE_OBOE_LOG ("Requested Oboe stream start with result: " + getOboeString (startResult));

                    startResult = stream->waitForStateChange (expectedState, &nextState, timeoutNanos);

                    ALOE_OBOE_LOG ("Starting Oboe stream with result: " + getOboeString (startResult);
                                     + "\nUses AAudio = " + String ((int) stream->usesAAudio())
                                     + "\nDirection = " + getOboeString (stream->getDirection())
                                     + "\nSharingMode = " + getOboeString (stream->getSharingMode())
                                     + "\nChannelCount = " + String (stream->getChannelCount())
                                     + "\nFormat = " + getOboeString (stream->getFormat())
                                     + "\nSampleRate = " + String (stream->getSampleRate())
                                     + "\nBufferSizeInFrames = " + String (stream->getBufferSizeInFrames())
                                     + "\nBufferCapacityInFrames = " + String (stream->getBufferCapacityInFrames())
                                     + "\nFramesPerBurst = " + String (stream->getFramesPerBurst())
                                     + "\nFramesPerCallback = " + String (stream->getFramesPerCallback())
                                     + "\nBytesPerFrame = " + String (stream->getBytesPerFrame())
                                     + "\nBytesPerSample = " + String (stream->getBytesPerSample())
                                     + "\nPerformanceMode = " + getOboeString (stream->getPerformanceMode())
                                     + "\ngetDeviceId = " + String (stream->getDeviceId()));
                }
        */
    }
    
    pub fn get_native_stream(&self) -> *mut OboeAudioStream {
        
        todo!();
        /*
            jassert (openedOk());
                return stream;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            if (stream != nullptr)
                {
                    auto count = stream->getXRunCount();

                    if (count)
                        return count.value();

                    ALOE_OBOE_LOG ("Failed to get Xrun count: " + getOboeString (count.error()));
                }

                return 0;
        */
    }
    
    pub fn open(
        &mut self, 
        device_id:       i32,
        direction:       OboeDirection,
        sharing_mode:    OboeSharingMode,
        channel_count:   i32,
        format:          OboeAudioFormat,
        new_sample_rate: i32,
        new_buffer_size: i32,
        new_callback:    *mut OboeAudioStreamCallback
    ) {

        todo!();
        /*
            OboeDefaultStreamValues::FramesPerBurst = AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint();

                OboeAudioStreamBuilder builder;

                if (deviceId != -1)
                    builder.setDeviceId (deviceId);

                // Note: letting OS to choose the buffer capacity & frames per callback.
                builder.setDirection (direction);
                builder.setSharingMode (sharingMode);
                builder.setChannelCount (channelCount);
                builder.setFormat (format);
                builder.setSampleRate (newSampleRate);
                builder.setPerformanceMode (OboePerformanceMode::LowLatency);

               #if ALOE_USE_ANDROID_OBOE_STABILIZED_CALLBACK
                if (newCallback != nullptr)
                {
                    stabilizedCallback = std::make_unique<OboeStabilizedCallback> (newCallback);
                    builder.setCallback (stabilizedCallback.get());
                }
               #else
                builder.setCallback (newCallback);
               #endif

                ALOE_OBOE_LOG (String ("Preparing Oboe stream with params:")
                     + "\nAAudio supported = " + String (int (builder.isAAudioSupported()))
                     + "\nAPI = " + getOboeString (builder.getAudioApi())
                     + "\nDeviceId = " + String (deviceId)
                     + "\nDirection = " + getOboeString (direction)
                     + "\nSharingMode = " + getOboeString (sharingMode)
                     + "\nChannelCount = " + String (channelCount)
                     + "\nFormat = " + getOboeString (format)
                     + "\nSampleRate = " + String (newSampleRate)
                     + "\nPerformanceMode = " + getOboeString (OboePerformanceMode::LowLatency));

                openResult = builder.openStream (&stream);
                ALOE_OBOE_LOG ("Building Oboe stream with result: " + getOboeString (openResult)
                     + "\nStream state = " + (stream != nullptr ? getOboeString (stream->getState()) : String ("?")));

                if (stream != nullptr && newBufferSize != 0)
                {
                    ALOE_OBOE_LOG ("Setting the bufferSizeInFrames to " + String (newBufferSize));
                    stream->setBufferSizeInFrames (newBufferSize);
                }

                ALOE_OBOE_LOG (String ("Stream details:")
                     + "\nUses AAudio = " + (stream != nullptr ? String ((int) stream->usesAAudio()) : String ("?"))
                     + "\nDeviceId = " + (stream != nullptr ? String (stream->getDeviceId()) : String ("?"))
                     + "\nDirection = " + (stream != nullptr ? getOboeString (stream->getDirection()) : String ("?"))
                     + "\nSharingMode = " + (stream != nullptr ? getOboeString (stream->getSharingMode()) : String ("?"))
                     + "\nChannelCount = " + (stream != nullptr ? String (stream->getChannelCount()) : String ("?"))
                     + "\nFormat = " + (stream != nullptr ? getOboeString (stream->getFormat()) : String ("?"))
                     + "\nSampleRate = " + (stream != nullptr ? String (stream->getSampleRate()) : String ("?"))
                     + "\nBufferSizeInFrames = " + (stream != nullptr ? String (stream->getBufferSizeInFrames()) : String ("?"))
                     + "\nBufferCapacityInFrames = " + (stream != nullptr ? String (stream->getBufferCapacityInFrames()) : String ("?"))
                     + "\nFramesPerBurst = " + (stream != nullptr ? String (stream->getFramesPerBurst()) : String ("?"))
                     + "\nFramesPerCallback = " + (stream != nullptr ? String (stream->getFramesPerCallback()) : String ("?"))
                     + "\nBytesPerFrame = " + (stream != nullptr ? String (stream->getBytesPerFrame()) : String ("?"))
                     + "\nBytesPerSample = " + (stream != nullptr ? String (stream->getBytesPerSample()) : String ("?"))
                     + "\nPerformanceMode = " + (stream != nullptr ? getOboeString (stream->getPerformanceMode()) : String ("?")));
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (stream != nullptr)
                {
                    OboeResult result = stream->close();
                    ignoreUnused (result);
                    ALOE_OBOE_LOG ("Requested Oboe stream close with result: " + getOboeString (result));
                }
        */
    }
}
