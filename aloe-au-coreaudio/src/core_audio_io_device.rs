crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CoreAudioIODevice<'a> {
    base:                            AudioIODevice,
    base2:                           Timer,
    device_type:                     WeakReference<CoreAudioIODeviceType<'a>>,
    input_index:                     i32,
    output_index:                    i32,
    internal:                        Box<CoreAudioInternal<'a>>,
    is_open:                         bool, // default = false
    is_started:                      bool, // default = false
    restart_device:                  bool, // default = true
    last_error:                      String,
    previous_callback:               *mut dyn AudioIODeviceCallback, // default = nullptr
    device_wrapper_restart_callback: fn() -> (), // default = nullptr
    input_channels_requested:        BigInteger,
    output_channels_requested:       BigInteger,
    close_lock:                      CriticalSection,
}

impl<'a> Drop for CoreAudioIODevice<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                close();

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioObjectPropertySelectorWildcard;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = kAudioObjectPropertyElementWildcard;

                AudioObjectRemovePropertyListener (kAudioObjectSystemObject, &pa, hardwareListenerProc, internal.get());
             */
    }
}

impl<'a> CoreAudioIODevice<'a> {

    pub fn new(
        dt:               *mut CoreAudioIODeviceType,
        device_name:      &String,
        input_device_id:  AudioDeviceID,
        input_index:      i32,
        output_device_id: AudioDeviceID,
        output_index:     i32) -> Self {
    
        todo!();
        /*


            : AudioIODevice (deviceName, "CoreAudio"),
                  deviceType (dt),
                  inputIndex (inputIndex_),
                  outputIndex (outputIndex_)

                CoreAudioInternal* device = nullptr;

                if (outputDeviceId == 0 || outputDeviceId == inputDeviceId)
                {
                    jassert (inputDeviceId != 0);
                    device = new CoreAudioInternal (*this, inputDeviceId, true, outputDeviceId != 0);
                }
                else
                {
                    device = new CoreAudioInternal (*this, outputDeviceId, false, true);
                }

                jassert (device != nullptr);
                internal.reset (device);

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioObjectPropertySelectorWildcard;
                pa.mScope    = kAudioObjectPropertyScopeWildcard;
                pa.mElement  = kAudioObjectPropertyElementWildcard;

                AudioObjectAddPropertyListener (kAudioObjectSystemObject, &pa, hardwareListenerProc, internal.get());
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            return internal->outChanNames;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            return internal->inChanNames;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return isOpen_;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            return internal->sampleRates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return internal->bufferSizes;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return internal->getSampleRate();
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return internal->bitDepth;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return internal->getBufferSize();
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return internal->xruns;
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            int best = 0;

                for (int i = 0; best < 512 && i < internal->bufferSizes.size(); ++i)
                    best = internal->bufferSizes.getUnchecked(i);

                if (best == 0)
                    best = 512;

                return best;
        */
    }
    
    pub fn open(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        sample_rate:         f64,
        buffer_size_samples: i32) -> String {
        
        todo!();
        /*
            isOpen_ = true;
                internal->xruns = 0;

                inputChannelsRequested = inputChannels;
                outputChannelsRequested = outputChannels;

                if (bufferSizeSamples <= 0)
                    bufferSizeSamples = getDefaultBufferSize();

                if (sampleRate <= 0)
                    sampleRate = internal->getNominalSampleRate();

                lastError = internal->reopen (inputChannels, outputChannels, sampleRate, bufferSizeSamples);
                ALOE_COREAUDIOLOG ("Opened: " << getName());

                isOpen_ = lastError.isEmpty();

                return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            isOpen_ = false;
                internal->stop (false);
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return internal->activeOutputChans;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return internal->activeInputChans;
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            // this seems like a good guess at getting the latency right - comparing
                // this with a round-trip measurement, it gets it to within a few millisecs
                // for the built-in mac soundcard
                return internal->outputLatency;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return internal->inputLatency;
        */
    }
    
    pub fn start(&mut self, callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (! isStarted)
                {
                    if (callback != nullptr)
                        callback->audioDeviceAboutToStart (this);

                    isStarted = internal->start();

                    if (isStarted)
                    {
                        internal->setCallback (callback);
                        previousCallback = callback;
                    }
                }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            restartDevice = false;

                if (isStarted)
                {
                    auto lastCallback = internal->callback;

                    isStarted = false;
                    internal->stop (true);

                    if (lastCallback != nullptr)
                        lastCallback->audioDeviceStopped();
                }
        */
    }
    
    pub fn stop_internal(&mut self)  {
        
        todo!();
        /*
            stop();
                restartDevice = true;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            if (internal->callback == nullptr)
                    isStarted = false;

                return isStarted;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn audio_device_list_changed(&mut self)  {
        
        todo!();
        /*
            if (deviceType != nullptr)
                    deviceType->audioDeviceListChanged();
        */
    }
    
    pub fn restart(&mut self)  {
        
        todo!();
        /*
            if (deviceWrapperRestartCallback != nullptr)
                {
                    deviceWrapperRestartCallback();
                }
                else
                {
                    {
                        const ScopedLock sl (closeLock);

                        if (isStarted)
                        {
                            if (internal->callback != nullptr)
                                previousCallback = internal->callback;

                            stopInternal();
                        }
                    }

                    startTimer (100);
                }
        */
    }
    
    pub fn set_current_sample_rate(&mut self, new_sample_rate: f64) -> bool {
        
        todo!();
        /*
            return internal->setNominalSampleRate (newSampleRate);
        */
    }
    
    pub fn set_device_wrapper_restart_callback(&mut self, cb: &fn() -> ())  {
        
        todo!();
        /*
            deviceWrapperRestartCallback = cb;
        */
    }
    
    pub fn should_restart_device(&self) -> bool {
        
        todo!();
        /*
            return restartDevice;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

                stopInternal();

                internal->updateDetailsFromDevice();

                open (inputChannelsRequested, outputChannelsRequested,
                      getCurrentSampleRate(), getCurrentBufferSizeSamples());
                start (previousCallback);
        */
    }
    
    pub fn hardware_listener_proc(
        in_device:      AudioDeviceID,
        in_line:        u32,
        pa:             *const AudioObjectPropertyAddress,
        in_client_data: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            switch (pa->mSelector)
                {
                    case kAudioHardwarePropertyDevices:
                        static_cast<CoreAudioInternal*> (inClientData)->deviceDetailsChanged();
                        break;

                    case kAudioHardwarePropertyDefaultOutputDevice:
                    case kAudioHardwarePropertyDefaultInputDevice:
                    case kAudioHardwarePropertyDefaultSystemOutputDevice:
                        break;
                }

                return noErr;
        */
    }
}
