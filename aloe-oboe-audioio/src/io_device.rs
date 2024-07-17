crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OboeAudioIODevice {
    base:                          AudioIODevice,
    actual_buffer_size:            i32, // default = 0
    sample_rate:                   i32, // default = 0
    device_open:                   bool, // default = false
    last_error:                    String,
    active_output_chans:           BigInteger,
    active_input_chans:            BigInteger,
    callback:                      Atomic<*mut dyn AudioIODeviceCallback>, // default = nullptr 
    input_device_id:               i32,
    supported_input_sample_rates:  Vec<i32>,
    max_num_input_channels:        i32,
    output_device_id:              i32,
    supported_output_sample_rates: Vec<i32>,
    max_num_output_channels:       i32,
    session:                       Box<dyn OboeSessionBaseInterface>,
    running:                       bool, // default = false
}

impl Drop for OboeAudioIODevice {

    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

impl OboeAudioIODevice {

    pub fn new(
        device_name:                          &String,
        input_device_id_to_use:               i32,
        supported_input_sample_rates_to_use:  &[i32],
        max_num_input_channels_to_use:        i32,
        output_device_id_to_use:              i32,
        supported_output_sample_rates_to_use: &[i32],
        max_num_output_channels_to_use:       i32) -> Self {
    
        todo!();
        /*
        : audio_io_device(deviceName, oboeTypeName),
        : input_device_id(inputDeviceIdToUse),
        : supported_input_sample_rates(supportedInputSampleRatesToUse),
        : max_num_input_channels(maxNumInputChannelsToUse),
        : output_device_id(outputDeviceIdToUse),
        : supported_output_sample_rates(supportedOutputSampleRatesToUse),
        : max_num_output_channels(maxNumOutputChannelsToUse),

        
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            return getChannelNames (false);
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            return getChannelNames (true);
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> result;

            auto inputSampleRates  = getAvailableSampleRates (true);
            auto outputSampleRates = getAvailableSampleRates (false);

            if (inputDeviceId == -1)
            {
                for (auto& sr : outputSampleRates)
                    result.add (sr);
            }
            else if (outputDeviceId == -1)
            {
                for (auto& sr : inputSampleRates)
                    result.add (sr);
            }
            else
            {
                // For best performance, the same sample rate should be used for input and output,
                for (auto& inputSampleRate : inputSampleRates)
                {
                    if (outputSampleRates.contains (inputSampleRate))
                        result.add (inputSampleRate);
                }
            }

            // either invalid device was requested or its input&output don't have compatible sample rate
            jassert (result.size() > 0);
            return result;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return AndroidHighPerformanceAudioHelpers::getAvailableBufferSizes (getNativeBufferSize(), getAvailableSampleRates());
        */
    }
    
    pub fn open(&mut self, 
        input_channels:        &BigInteger,
        output_channels:       &BigInteger,
        requested_sample_rate: f64,
        buffer_size:           i32) -> String {
        
        todo!();
        /*
            close();

            lastError.clear();

            sampleRate = (int) (requestedSampleRate > 0 ? requestedSampleRate : AndroidHighPerformanceAudioHelpers::getNativeSampleRate());
            actualBufferSize = (bufferSize <= 0) ? getDefaultBufferSize() : bufferSize;

            // The device may report no max, claiming "no limits". Pick sensible defaults.
            int maxOutChans = maxNumOutputChannels > 0 ? maxNumOutputChannels : 2;
            int maxInChans  = maxNumInputChannels  > 0 ? maxNumInputChannels : 1;

            activeOutputChans = outputChannels;
            activeOutputChans.setRange (maxOutChans,
                                        activeOutputChans.getHighestBit() + 1 - maxOutChans,
                                        false);

            activeInputChans = inputChannels;
            activeInputChans.setRange (maxInChans,
                                       activeInputChans.getHighestBit() + 1 - maxInChans,
                                       false);

            int numOutputChans = activeOutputChans.countNumberOfSetBits();
            int numInputChans = activeInputChans.countNumberOfSetBits();

            if (numInputChans > 0 && (! RuntimePermissions::isGranted (RuntimePermissions::recordAudio)))
            {
                // If you hit this assert, you probably forgot to get RuntimePermissions::recordAudio
                // before trying to open an audio input device. This is not going to work!
                jassertfalse;
                lastError = "Error opening Oboe input device: the app was not granted android.permission.RECORD_AUDIO";
            }

            // At least one output channel should be set!
            jassert (numOutputChans >= 0);

            session.reset (OboeSessionBase::create (*this,
                                                    inputDeviceId, outputDeviceId,
                                                    numInputChans, numOutputChans,
                                                    sampleRate, actualBufferSize));

            deviceOpen = session != nullptr;

            if (! deviceOpen)
                lastError = "Failed to create audio session";

            return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return session->getOutputLatencyInSamples();
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return session->getInputLatencyInSamples();
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return deviceOpen;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return actualBufferSize;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return session->getCurrentBitDepth();
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeOutputChans;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeInputChans;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return callback.get() != nullptr;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return session->getXRunCount();
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return AndroidHighPerformanceAudioHelpers::getDefaultBufferSize (getNativeBufferSize(), getCurrentSampleRate());
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return (sampleRate == 0.0 ? AndroidHighPerformanceAudioHelpers::getNativeSampleRate() : sampleRate);
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (callback.get() != newCallback)
            {
                if (newCallback != nullptr)
                    newCallback->audioDeviceAboutToStart (this);

                AudioIODeviceCallback* oldCallback = callback.get();

                if (oldCallback != nullptr)
                {
                    // already running
                    if (newCallback == nullptr)
                        stop();
                    else
                        setCallback (newCallback);

                    oldCallback->audioDeviceStopped();
                }
                else
                {
                    jassert (newCallback != nullptr);

                    // session hasn't started yet
                    setCallback (newCallback);
                    running = true;

                    session->start();
                }

                callback = newCallback;
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (session != nullptr)
                session->stop();

            running = false;
            setCallback (nullptr);
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            // Oboe does not expose this setting, yet it may use preprocessing
            // for older APIs running OpenSL
            return false;
        */
    }
    
    pub fn get_channel_names(&mut self, for_input: bool) -> StringArray {
        
        todo!();
        /*
            auto& deviceId = forInput ? inputDeviceId : outputDeviceId;
            auto& numChannels = forInput ? maxNumInputChannels : maxNumOutputChannels;

            // If the device id is unknown (on olders APIs) or if the device claims to
            // support "any" channel count, use a sensible default
            if (deviceId == -1 || numChannels == -1)
                return forInput ? StringArray ("Input") : StringArray ("Left", "Right");

            StringArray names;

            for (int i = 0; i < numChannels; ++i)
                names.add ("Channel " + String (i + 1));

            return names;
        */
    }
    
    pub fn get_available_sample_rates_for_input(&mut self, for_input: bool) -> Vec<i32> {
        
        todo!();
        /*
            auto& supportedSampleRates = forInput
                ? supportedInputSampleRates
                : supportedOutputSampleRates;

            if (! supportedSampleRates.isEmpty())
                return supportedSampleRates;

            // device claims that it supports "any" sample rate, use
            // standard ones then
            return getDefaultSampleRates();
        */
    }
    
    pub fn get_default_sample_rates() -> Vec<i32> {
        
        todo!();
        /*
            static const int standardRates[] = { 8000, 11025, 12000, 16000,
                                                22050, 24000, 32000, 44100, 48000 };

            Vec<int> rates (standardRates, numElementsInArray (standardRates));

            // make sure the native sample rate is part of the list
            int native = (int) AndroidHighPerformanceAudioHelpers::getNativeSampleRate();

            if (native != 0 && ! rates.contains (native))
                rates.add (native);

            return rates;
        */
    }
    
    pub fn get_native_buffer_size() -> i32 {
        
        todo!();
        /*
            auto bufferSizeHint = AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint();

            // providing a callback is required on some devices to get a FAST track, so we pass an
            // empty one to the temp stream to get the best available buffer size
            struct DummyCallback  : public OboeAudioStreamCallback
            {
                OboeDataCallbackResult onAudioReady (OboeAudioStream*, void*, int32_t) override  { return OboeDataCallbackResult::Stop; }
            };

            DummyCallback callback;

            // NB: Exclusive mode could be rejected if a device is already opened in that mode, so to get
            //     reliable results, only use this function when a device is closed.
            //     We initially try to open a stream with a buffer size returned from
            //     android.media.property.OUTPUT_FRAMES_PER_BUFFER property, but then we verify the actual
            //     size after the stream is open.
            OboeAudioIODevice::OboeStream tempStream (OboekUnspecified,
                                                      OboeDirection::Output,
                                                      OboeSharingMode::Exclusive,
                                                      2,
                                                      getAndroidSDKVersion() >= 21 ? OboeAudioFormat::Float : OboeAudioFormat::I16,
                                                      (int) AndroidHighPerformanceAudioHelpers::getNativeSampleRate(),
                                                      bufferSizeHint,
                                                      &callback);

            if (auto* nativeStream = tempStream.getNativeStream())
                return nativeStream->getFramesPerBurst();

            return bufferSizeHint;
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

            // Setting nullptr callback is allowed only when playback is stopped.
            jassert (callbackToUse != nullptr);

            for (;;)
            {
                auto old = callback.get();

                if (old == callbackToUse)
                    break;

                // If old is nullptr, then it means that it's currently being used!
                if (old != nullptr && callback.compareAndSetBool (callbackToUse, old))
                    break;

                Thread::sleep (1);
            }
        */
    }
    
    pub fn process(&mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_frames:          i32)  {
        
        todo!();
        /*
            if (auto* cb = callback.exchange (nullptr))
            {
                cb->audioDeviceIOCallback (inputChannelData, numInputChannels,
                                           outputChannelData, numOutputChannels, numFrames);
                callback.set (cb);
            }
            else
            {
                for (int i = 0; i < numOutputChannels; ++i)
                    zeromem (outputChannelData[i], (size_t) (numFrames) * sizeof (float));
            }
        */
    }
}
