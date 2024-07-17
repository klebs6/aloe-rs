crate::ix!();

#[cfg(target_os="android")]
pub trait OpenSLSessionInterface {
    fn set_audio_preprocessing_enabled(&mut self, should_enable: bool) -> bool;
    fn supports_floating_point(&self) -> bool;
    fn getx_run_count(&self) -> i32;
}

#[cfg(target_os="android")]
pub struct OpenSLSession {
    input_channels:           i32,
    output_channels:          i32,
    sample_rate:              f64,
    buffer_size:              i32,
    num_buffers:              i32,
    running:                  bool, // default = false
    audio_processing_enabled: bool, // default = true
    output_mix:               SlRef<SLOutputMixItf_>,
    callback:                 Atomic<*mut dyn AudioIODeviceCallback>, // default = nullptr 
}

#[cfg(target_os="android")]
impl OpenSLSession {

    pub fn new(
        num_input_channels:  i32,
        num_output_channels: i32,
        samle_rate_to_use:   f64,
        buffer_size_to_use:  i32,
        num_buffers_to_use:  i32) -> Self {
    
        todo!();
        /*
        : input_channels(numInputChannels),
        : output_channels(numOutputChannels),
        : sample_rate(samleRateToUse),
        : buffer_size(bufferSizeToUse),
        : num_buffers(numBuffersToUse),

            jassert (numInputChannels > 0 || numOutputChannels > 0);

                if (outputChannels > 0)
                {
                    auto& holder = getEngineHolder();
                    SLObjectItf obj = nullptr;

                    auto err = (*holder.engine)->CreateOutputMix (holder.engine, &obj, 0, nullptr, nullptr);

                    if (err != SL_RESULT_SUCCESS || obj == nullptr || *obj == nullptr
                         || (*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                    {
                        destroyObject (obj);
                        return;
                    }

                    outputMix = SlRef<SLOutputMixItf_>::cast (SlObjectRef (obj));
                }
        */
    }
    
    pub fn openedok(&self) -> bool {
        
        todo!();
        /*
            return (outputChannels == 0 || outputMix != nullptr);
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            stop(); jassert (callback.get() != nullptr); running = true;
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            running = false;
        */
    }
    
    pub fn set_callback(&mut self, callback_to_use: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (! running)
                {
                    callback.set (callbackToUse);
                    return;
                }

                // don't set callback to null! stop the playback instead!
                jassert (callbackToUse != nullptr);

                // spin-lock until we can set the callback
                for (;;)
                {
                    auto old = callback.get();

                    if (old == callbackToUse)
                        break;

                    if (callback.compareAndSetBool (callbackToUse, old))
                        break;

                    Thread::sleep (1);
                }
        */
    }
    
    pub fn process(&mut self, 
        input_channel_data:  *const *const f32,
        output_channel_data: *mut *mut f32)  {
        
        todo!();
        /*
            if (auto* cb = callback.exchange (nullptr))
                {
                    cb->audioDeviceIOCallback (inputChannelData, inputChannels, outputChannelData, outputChannels, bufferSize);
                    callback.set (cb);
                }
                else
                {
                    for (int i = 0; i < outputChannels; ++i)
                        zeromem (outputChannelData[i], sizeof(float) * static_cast<size_t> (bufferSize));
                }
        */
    }
    
    pub fn create(&mut self, 
        num_input_channels:  i32,
        num_output_channels: i32,
        samle_rate_to_use:   f64,
        buffer_size_to_use:  i32,
        num_buffers_to_use:  i32) -> *mut OpenSLSession {
        
        todo!();
        /*
            std::unique_ptr<OpenSLSession> retval;
        auto sdkVersion = getAndroidSDKVersion();

        // SDK versions 21 and higher should natively support floating point...
        if (sdkVersion >= 21)
        {
            retval.reset (new OpenSLSessionT<float> (numInputChannels, numOutputChannels, samleRateToUse,
                                                     bufferSizeToUse, numBuffersToUse));

            // ...however, some devices lie so re-try without floating point
            if (retval != nullptr && (! retval->openedOK()))
                retval = nullptr;
        }

        if (retval == nullptr)
        {
            retval.reset (new OpenSLSessionT<int16> (numInputChannels, numOutputChannels, samleRateToUse,
                                                     bufferSizeToUse, numBuffersToUse));

            if (retval != nullptr && (! retval->openedOK()))
                retval = nullptr;
        }

        return retval.release();
        */
    }
}
