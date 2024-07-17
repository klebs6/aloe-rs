crate::ix!();

pub struct OboeSessionImpl<'a, SampleType> {
    base:                                  OboeSessionBase<'a>,
    input_stream_native_buffer:            HeapBlock<SampleType>,
    input_stream_sample_buffer:            AudioBuffer<f32>,
    output_stream_sample_buffer:           AudioBuffer<f32>,
    audio_callback_guard:                  Atomic<i32>, // default = 0 
    stream_restart_guard:                  Atomic<i32>, // default = 0 
    is_input_latency_detection_supported:  bool, // default = false
    input_latency:                         i32, // default = -1
    is_output_latency_detection_supported: bool, // default = false
    output_latency:                        i32, // default = -1
}

impl<'a, SampleType> OboeSessionImpl<'a, SampleType> {

    pub fn new(
        owner_to_use:               &mut OboeAudioIODevice,
        input_device_id_in:         i32,
        output_device_id_in:        i32,
        num_input_channels_to_use:  i32,
        num_output_channels_to_use: i32,
        sample_rate_to_use:         i32,
        buffer_size_to_use:         i32) -> Self {
    
        todo!();
        /*


            : OboeSessionBase (ownerToUse,
                                   inputDeviceIdIn, outputDeviceIdIn,
                                   numInputChannelsToUse, numOutputChannelsToUse,
                                   sampleRateToUse, bufferSizeToUse,
                                   OboeAudioIODeviceBufferHelpers<SampleType>::oboeAudioFormat(),
                                   OboeAudioIODeviceBufferHelpers<SampleType>::bitDepth()),
                  inputStreamNativeBuffer (static_cast<size_t> (numInputChannelsToUse * getBufferCapacityInFrames (true))),
                  inputStreamSampleBuffer (numInputChannels, getBufferCapacityInFrames (true)),
                  outputStreamSampleBuffer (numOutputChannels, getBufferCapacityInFrames (false))
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            audioCallbackGuard.set (0);

                if (inputStream != nullptr)
                    inputStream->start();

                outputStream->start();

                isInputLatencyDetectionSupported  = isLatencyDetectionSupported (inputStream.get());
                isOutputLatencyDetectionSupported = isLatencyDetectionSupported (outputStream.get());
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            while (! audioCallbackGuard.compareAndSetBool (1, 0))
                    Thread::sleep (1);

                inputStream  = nullptr;
                outputStream = nullptr;

                audioCallbackGuard.set (0);
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return outputLatency;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return inputLatency;
        */
    }
    
    pub fn is_latency_detection_supported(&mut self, stream: *mut OboeStream) -> bool {
        
        todo!();
        /*
            if (stream == nullptr || ! openedOk())
                    return false;

                auto result = stream->getNativeStream()->getTimestamp (CLOCK_MONOTONIC, nullptr, nullptr);
                return result != OboeResult::ErrorUnimplemented;
        */
    }
    
    pub fn on_audio_ready(&mut self, 
        stream:     *mut OboeAudioStream,
        audio_data: *mut c_void,
        num_frames: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            if (audioCallbackGuard.compareAndSetBool (1, 0))
                {
                    if (stream == nullptr)
                        return OboeDataCallbackResult::Stop;

                    // only output stream should be the master stream receiving callbacks
                    jassert (stream->getDirection() == OboeDirection::Output && stream == outputStream->getNativeStream());

                    // Read input from Oboe
                    inputStreamSampleBuffer.clear();
                    inputStreamNativeBuffer.calloc (static_cast<size_t> (numInputChannels * bufferSize));

                    if (inputStream != nullptr)
                    {
                        auto* nativeInputStream = inputStream->getNativeStream();

                        if (nativeInputStream->getFormat() != OboeAudioFormat::I16 && nativeInputStream->getFormat() != OboeAudioFormat::Float)
                        {
                            ALOE_OBOE_LOG ("Unsupported input stream audio format: " + getOboeString (nativeInputStream->getFormat()));
                            jassertfalse;
                            return OboeDataCallbackResult::Continue;
                        }

                        auto result = inputStream->getNativeStream()->read (inputStreamNativeBuffer.getData(), numFrames, 0);

                        if (result)
                        {
                            auto referringDirectlyToOboeData = OboeAudioIODeviceBufferHelpers<SampleType>
                                                                 ::referAudioBufferDirectlyToOboeIfPossible (inputStreamNativeBuffer.get(),
                                                                                                             inputStreamSampleBuffer,
                                                                                                             result.value());

                            if (! referringDirectlyToOboeData)
                                OboeAudioIODeviceBufferHelpers<SampleType>::convertFromOboe (inputStreamNativeBuffer.get(), inputStreamSampleBuffer, result.value());
                        }
                        else
                        {
                            ALOE_OBOE_LOG ("Failed to read from input stream: " + getOboeString (result.error()));
                        }

                        if (isInputLatencyDetectionSupported)
                            inputLatency = getLatencyFor (*inputStream);
                    }

                    // Setup output buffer
                    auto referringDirectlyToOboeData = OboeAudioIODeviceBufferHelpers<SampleType>
                                                         ::referAudioBufferDirectlyToOboeIfPossible (static_cast<SampleType*> (audioData),
                                                                                                     outputStreamSampleBuffer,
                                                                                                     numFrames);

                    if (! referringDirectlyToOboeData)
                        outputStreamSampleBuffer.clear();

                    // Process
                    // NB: the number of samples read from the input can potentially differ from numFrames.
                    owner.process (inputStreamSampleBuffer.getArrayOfReadPointers(), numInputChannels,
                                   outputStreamSampleBuffer.getArrayOfWritePointers(), numOutputChannels,
                                   numFrames);

                    // Write output to Oboe
                    if (! referringDirectlyToOboeData)
                        OboeAudioIODeviceBufferHelpers<SampleType>::convertToOboe (outputStreamSampleBuffer, static_cast<SampleType*> (audioData), numFrames);

                    if (isOutputLatencyDetectionSupported)
                        outputLatency = getLatencyFor (*outputStream);

                    audioCallbackGuard.set (0);
                }

                return OboeDataCallbackResult::Continue;
        */
    }
    
    pub fn print_stream_debug_info(&mut self, stream: *mut OboeAudioStream)  {
        
        todo!();
        /*
            ignoreUnused (stream);

                ALOE_OBOE_LOG ("\nUses AAudio = " + (stream != nullptr ? String ((int) stream->usesAAudio()) : String ("?"))
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
                     + "\nPerformanceMode = " + (stream != nullptr ? getOboeString (stream->getPerformanceMode()) : String ("?"))
                     + "\ngetDeviceId = " + (stream != nullptr ? String (stream->getDeviceId()) : String ("?")));
        */
    }
    
    pub fn get_latency_for(&mut self, stream: &mut OboeStream) -> i32 {
        
        todo!();
        /*
            auto& nativeStream = *stream.getNativeStream();

                if (auto latency = nativeStream.calculateLatencyMillis())
                    return static_cast<int> ((latency.value() * sampleRate) / 1000);

                // Get the time that a known audio frame was presented.
                int64_t hardwareFrameIndex = 0;
                int64_t hardwareFrameHardwareTime = 0;

                auto result = nativeStream.getTimestamp (CLOCK_MONOTONIC,
                                                         &hardwareFrameIndex,
                                                         &hardwareFrameHardwareTime);

                if (result != OboeResult::OK)
                    return 0;

                // Get counter closest to the app.
                const bool isOutput = nativeStream.getDirection() == OboeDirection::Output;
                const int64_t appFrameIndex = isOutput ? nativeStream.getFramesWritten() : nativeStream.getFramesRead();

                // Assume that the next frame will be processed at the current time
                int64_t appFrameAppTime = getCurrentTimeNanos();

                // Calculate the number of frames between app and hardware
                int64_t frameIndexDelta = appFrameIndex - hardwareFrameIndex;

                // Calculate the time which the next frame will be or was presented
                int64_t frameTimeDelta = (frameIndexDelta * OboekNanosPerSecond) / sampleRate;
                int64_t appFrameHardwareTime = hardwareFrameHardwareTime + frameTimeDelta;

                // Calculate latency as a difference in time between when the current frame is at the app
                // and when it is at the hardware.
                auto latencyNanos = isOutput ? (appFrameHardwareTime - appFrameAppTime) : (appFrameAppTime - appFrameHardwareTime);
                return static_cast<int> ((latencyNanos  * sampleRate) / OboekNanosPerSecond);
        */
    }
    
    pub fn get_current_time_nanos(&mut self) -> i64 {
        
        todo!();
        /*
            timespec time;

                if (clock_gettime (CLOCK_MONOTONIC, &time) < 0)
                    return -1;

                return time.tv_sec * OboekNanosPerSecond + time.tv_nsec;
        */
    }
    
    pub fn on_error_before_close(&mut self, 
        stream: *mut OboeAudioStream,
        error:  OboeResult)  {
        
        todo!();
        /*
            ignoreUnused (error);

                // only output stream should be the master stream receiving callbacks
                jassert (stream->getDirection() == OboeDirection::Output);

                ALOE_OBOE_LOG ("Oboe stream onErrorBeforeClose(): " + getOboeString (error));
                printStreamDebugInfo (stream);
        */
    }
    
    pub fn on_error_after_close(&mut self, 
        stream: *mut OboeAudioStream,
        error:  OboeResult)  {
        
        todo!();
        /*
            // only output stream should be the master stream receiving callbacks
                jassert (stream->getDirection() == OboeDirection::Output);

                ALOE_OBOE_LOG ("Oboe stream onErrorAfterClose(): " + getOboeString (error));

                if (error == OboeResult::ErrorDisconnected)
                {
                    if (streamRestartGuard.compareAndSetBool (1, 0))
                    {
                        // Close, recreate, and start the stream, not much use in current one.
                        // Use default device id, to let the OS pick the best ID (since our was disconnected).

                        while (! audioCallbackGuard.compareAndSetBool (1, 0))
                            Thread::sleep (1);

                        outputStream = nullptr;
                        outputStream.reset (new OboeStream (OboekUnspecified,
                                                            OboeDirection::Output,
                                                            OboeSharingMode::Exclusive,
                                                            numOutputChannels,
                                                            streamFormat,
                                                            sampleRate,
                                                            bufferSize,
                                                            this));

                        outputStream->start();

                        audioCallbackGuard.set (0);
                        streamRestartGuard.set (0);
                    }
                }
        */
    }
}
