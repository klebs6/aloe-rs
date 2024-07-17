crate::ix!();

#[cfg(target_os="android")]
lazy_static!{
    /*
    static const char* const openSLTypeName;
    const char* const OpenSLAudioIODevice::openSLTypeName = "Android OpenSL";

    */
}

#[no_copy]
#[leak_detector]
#[cfg(target_os="android")]
pub struct OpenSLAudioIODevice {
    base:                     AudioIODevice,
    actual_buffer_size:       i32, // default = 0
    sample_rate:              i32, // default = 0
    audio_buffers_to_enqueue: i32, // default = 0
    input_latency:            i32,
    output_latency:           i32,
    device_open:              bool, // default = false
    audio_processing_enabled: bool, // default = true
    last_error:               String,
    active_output_chans:      BigInteger,
    active_input_chans:       BigInteger,
    callback:                 *mut dyn AudioIODeviceCallback, // default = nullptr
    session:                  Box<OpenSLSession>,
}

#[cfg(target_os="android")]
impl Drop for OpenSLAudioIODevice {
    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

#[cfg(target_os="android")]
impl OpenSLAudioIODevice {

    pub fn new(device_name: &String) -> Self {
    
        todo!();
        /*
        : audio_io_device(deviceName, openSLTypeName),

            // OpenSL has piss-poor support for determining latency, so the only way I can find to
            // get a number for this is by asking the AudioTrack/AudioRecord classes..
            AndroidAudioIODevice javaDevice (deviceName);

            // this is a total guess about how to calculate the latency, but seems to vaguely agree
            // with the devices I've tested.. YMMV
            inputLatency  = (javaDevice.minBufferSizeIn  * 2) / 3;
            outputLatency = (javaDevice.minBufferSizeOut * 2) / 3;

            const int64 longestLatency = jmax (inputLatency, outputLatency);
            const int64 totalLatency = inputLatency + outputLatency;
            inputLatency  = (int) ((longestLatency * inputLatency)  / totalLatency) & ~15;
            outputLatency = (int) ((longestLatency * outputLatency) / totalLatency) & ~15;

            // You can only create this class if you are sure that your hardware supports OpenSL
            jassert (getEngineHolder().slLibrary.getNativeHandle() != nullptr);
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return session != nullptr;
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> s;
            s.add ("Left");
            s.add ("Right");
            return s;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> s;
            s.add ("Audio Input");
            return s;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            // see https://developer.android.com/ndk/guides/audio/opensl-for-android.html

            static const double rates[] = { 8000.0, 11025.0, 12000.0, 16000.0,
                                            22050.0, 24000.0, 32000.0, 44100.0, 48000.0 };

            Vec<double> retval (rates, numElementsInArray (rates));

            // make sure the native sample rate is part of the list
            double native = AndroidHighPerformanceAudioHelpers::getNativeSampleRate();

            if (native != 0.0 && ! retval.contains (native))
                retval.add (native);

            return retval;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return AndroidHighPerformanceAudioHelpers::getAvailableBufferSizes (AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint(),
                                                                                getAvailableSampleRates());
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
            auto preferredBufferSize = (bufferSize > 0) ? bufferSize : getDefaultBufferSize();

            audioBuffersToEnqueue = [this, preferredBufferSize]
            {
                using namespace AndroidHighPerformanceAudioHelpers;

                auto nativeBufferSize = getNativeBufferSizeHint();

                if (canUseHighPerformanceAudioPath (nativeBufferSize, preferredBufferSize, sampleRate))
                    return preferredBufferSize / nativeBufferSize;


                return 1;
            }();

            actualBufferSize = preferredBufferSize / audioBuffersToEnqueue;

            jassert ((actualBufferSize * audioBuffersToEnqueue) == preferredBufferSize);

            activeOutputChans = outputChannels;
            activeOutputChans.setRange (2, activeOutputChans.getHighestBit(), false);
            auto numOutputChannels = activeOutputChans.countNumberOfSetBits();

            activeInputChans = inputChannels;
            activeInputChans.setRange (1, activeInputChans.getHighestBit(), false);
            auto numInputChannels = activeInputChans.countNumberOfSetBits();

            if (numInputChannels > 0 && (! RuntimePermissions::isGranted (RuntimePermissions::recordAudio)))
            {
                // If you hit this assert, you probably forgot to get RuntimePermissions::recordAudio
                // before trying to open an audio input device. This is not going to work!
                jassertfalse;
                lastError = "Error opening OpenSL input device: the app was not granted android.permission.RECORD_AUDIO";
            }

            session.reset (OpenSLSession::create (numInputChannels, numOutputChannels,
                                                  sampleRate, actualBufferSize, audioBuffersToEnqueue));
            if (session != nullptr)
            {
                session->setAudioPreprocessingEnabled (audioProcessingEnabled);
            }
            else
            {
                if (numInputChannels > 0 && numOutputChannels > 0 && RuntimePermissions::isGranted (RuntimePermissions::recordAudio))
                {
                    // New versions of the Android emulator do not seem to support audio input anymore on OS X
                    activeInputChans = BigInteger(0);
                    numInputChannels = 0;

                    session.reset (OpenSLSession::create (numInputChannels, numOutputChannels,
                                                          sampleRate, actualBufferSize, audioBuffersToEnqueue));
                }
            }

            DBG ("OpenSL: numInputChannels = " << numInputChannels
                 << ", numOutputChannels = " << numOutputChannels
                 << ", nativeBufferSize = " << AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint()
                 << ", nativeSampleRate = " << AndroidHighPerformanceAudioHelpers::getNativeSampleRate()
                 << ", actualBufferSize = " << actualBufferSize
                 << ", audioBuffersToEnqueue = " << audioBuffersToEnqueue
                 << ", sampleRate = " << sampleRate
                 << ", supportsFloatingPoint = " << (session != nullptr && session->supportsFloatingPoint() ? "true" : "false"));

            if (session == nullptr)
                lastError = "Unknown error initializing opensl session";

            deviceOpen = (session != nullptr);
            return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();
            session = nullptr;
            callback = nullptr;
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
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return deviceOpen;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return actualBufferSize * audioBuffersToEnqueue;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return (session != nullptr && session->supportsFloatingPoint() ? 32 : 16);
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
            return callback != nullptr;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return (session != nullptr ? session->getXRunCount() : -1);
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return AndroidHighPerformanceAudioHelpers::getDefaultBufferSize (AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint(),
                                                                             getCurrentSampleRate());
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
            if (session != nullptr && callback != newCallback)
            {
                auto oldCallback = callback;

                if (newCallback != nullptr)
                    newCallback->audioDeviceAboutToStart (this);

                if (oldCallback != nullptr)
                {
                    // already running
                    if (newCallback == nullptr)
                        stop();
                    else
                        session->setCallback (newCallback);

                    oldCallback->audioDeviceStopped();
                }
                else
                {
                    jassert (newCallback != nullptr);

                    // session hasn't started yet
                    session->setCallback (newCallback);
                    session->start();
                }

                callback = newCallback;
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (session != nullptr && callback != nullptr)
            {
                callback = nullptr;
                session->stop();
                session->setCallback (nullptr);
            }
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, should_audio_processing_be_enabled: bool) -> bool {
        
        todo!();
        /*
            audioProcessingEnabled = shouldAudioProcessingBeEnabled;

            if (session != nullptr)
                session->setAudioPreprocessingEnabled (audioProcessingEnabled);

            return true;
        */
    }
}
