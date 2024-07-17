crate::ix!();

#[cfg(target_os="android")]
pub struct OpenSLSessionT<'a,T> {
    base:               OpenSLSession,
    player:             Box<OpenSLQueueRunnerPlayer<'a,T>>,
    recorder:           Box<OpenSLQueueRunnerRecorder<'a,T>>,
    guard:              Atomic<i32>,
    get_underrun_count: jmethodID, // default = nullptr
}

#[cfg(target_os="android")]
impl<'a,T> OpenSLSessionT<'a,T> {

    pub fn new(
        num_input_channels:  i32,
        num_output_channels: i32,
        samle_rate_to_use:   f64,
        buffer_size_to_use:  i32,
        num_buffers_to_use:  i32) -> Self {
    
        todo!();
        /*


            : OpenSLSession (numInputChannels, numOutputChannels,
                                 samleRateToUse, bufferSizeToUse, numBuffersToUse)

                jassert (numInputChannels > 0 || numOutputChannels > 0);

                if (OpenSLSession::openedOK())
                {
                    if (inputChannels > 0)
                    {
                        recorder.reset (new OpenSLQueueRunnerRecorder<T> (*this, inputChannels));

                        if (! recorder->init())
                        {
                            recorder = nullptr;
                            return;
                        }
                    }

                    if (outputChannels > 0)
                    {
                        player.reset (new OpenSLQueueRunnerPlayer<T> (*this, outputChannels));

                        if (! player->init())
                        {
                            player = nullptr;
                            return;
                        }

                        const bool supportsUnderrunCount = (getAndroidSDKVersion() >= 24);
                        getUnderrunCount = supportsUnderrunCount ? getEnv()->GetMethodID (AudioTrack, "getUnderrunCount", "()I") : nullptr;
                    }
                }
        */
    }
    
    pub fn openedok(&self) -> bool {
        
        todo!();
        /*
            return OpenSLSession::openedOK() && (inputChannels == 0  || recorder != nullptr)
                                                 && (outputChannels == 0 || player   != nullptr);
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            OpenSLSession::start();

                guard.set (0);

                if (inputChannels > 0)
                    recorder->clear();

                if (outputChannels > 0)
                    player->clear();

                // first enqueue all buffers
                for (int i = 0; i < numBuffers; ++i)
                    doSomeWorkOnAudioThread();

                if (inputChannels > 0)
                    recorder->setState (true);

                if (outputChannels > 0)
                    player->setState (true);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            OpenSLSession::stop();

                while (! guard.compareAndSetBool (1, 0))
                    Thread::sleep (1);

                if (inputChannels > 0)
                    recorder->setState (false);

                if (outputChannels > 0)
                    player->setState (false);

                guard.set (0);
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, should_enable: bool) -> bool {
        
        todo!();
        /*
            if (shouldEnable != audioProcessingEnabled)
                {
                    audioProcessingEnabled = shouldEnable;

                    if (recorder != nullptr)
                        return recorder->setAudioPreprocessingEnabled (audioProcessingEnabled);
                }

                return true;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            if (player != nullptr && player->javaProxy != nullptr && getUnderrunCount != nullptr)
                    return getEnv()->CallIntMethod (player->javaProxy, getUnderrunCount);

                return -1;
        */
    }
    
    pub fn supports_floating_point(&self) -> bool {
        
        todo!();
        /*
            return (BufferHelpers<T>::isFloatingPoint != 0);
        */
    }
    
    pub fn do_some_work_on_audio_thread(&mut self)  {
        
        todo!();
        /*
            // only the player or the recorder should enter this section at any time
                if (guard.compareAndSetBool (1, 0))
                {
                    // are there enough buffers available to process some audio
                    if ((inputChannels == 0 || recorder->isBufferAvailable()) && (outputChannels == 0 || player->isBufferAvailable()))
                    {
                        T* recorderBuffer = (inputChannels  > 0 ? recorder->getNextBuffer() : nullptr);
                        T* playerBuffer   = (outputChannels > 0 ? player->getNextBuffer()   : nullptr);

                        const float** inputChannelData = nullptr;
                        float** outputChannelData = nullptr;

                        if (recorderBuffer != nullptr)
                        {
                            BufferHelpers<T>::prepareCallbackBuffer (recorder->sampleBuffer, recorderBuffer);
                            BufferHelpers<T>::convertFromOpenSL (recorderBuffer, recorder->sampleBuffer);

                            inputChannelData = recorder->sampleBuffer.getArrayOfReadPointers();
                        }

                        if (playerBuffer != nullptr)
                        {
                            BufferHelpers<T>::prepareCallbackBuffer (player->sampleBuffer, playerBuffer);
                            outputChannelData = player->sampleBuffer.getArrayOfWritePointers();
                        }

                        process (inputChannelData, outputChannelData);

                        if (recorderBuffer != nullptr)
                            recorder->enqueueBuffer();

                        if (playerBuffer != nullptr)
                        {
                            BufferHelpers<T>::convertToOpenSL (player->sampleBuffer, playerBuffer);
                            player->enqueueBuffer();
                        }
                    }

                    guard.set (0);
                }
        */
    }
}
