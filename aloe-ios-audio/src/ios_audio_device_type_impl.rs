crate::ix!();

#[no_copy]
pub struct iOSAudioIODevicePimpl<'a> {

    base2:                        AsyncUpdater<'a>,
    channel_data:                 IOChannelData,
    requested_input_channels:     BigInteger,
    requested_output_channels:    BigInteger,
    is_running:                   bool, // default = false
    callback:                     *mut dyn AudioIODeviceCallback, // default = nullptr
    last_error:                   String,
    target_buffer_size:           i32, // default = defaultBufferSize
    buffer_size:                  i32, // default = targetBufferSize
    target_sample_rate:           f64, // default = 44100.0
    sample_rate:                  f64, // default = targetSampleRate
    available_sample_rates:       Vec<f64>,
    available_buffer_sizes:       Vec<i32>,
    inter_app_audio_connected:    bool, // default = false
    message_collector:            *mut MidiMessageCollector, // default = nullptr
    device_type:                  WeakReference<iOSAudioIODeviceType<'a>>,
    owner:                        &'a mut iOSAudioIODevice<'a>,
    callback_lock:                CriticalSection,
    hardware_info_needs_updating: Atomic<bool>, // default = true 
    audio_unit:                   AudioUnit,
    session_holder:               SharedResourcePointer<AudioSessionHolder<'a>>,
    first_host_time:              bool,
    last_sample_time:             f64,
    last_num_frames:              u32,
    xrun:                         i32,
}

impl<'a> AudioPlayHeadInterface for iOSAudioIODevicePimpl<'a> {

    fn get_current_position(&mut self, result: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
        
        todo!();
        /*
            if (! canControlTransport())
                return false;

            zerostruct (result);

            HostCallbackInfo callbackInfo;
            fillHostCallbackInfo (callbackInfo);

            if (callbackInfo.hostUserData == nullptr)
                return false;

            Boolean hostIsPlaying               = NO;
            Boolean hostIsRecording             = NO;
            Float64 hostCurrentSampleInTimeLine = 0;
            Boolean hostIsCycling               = NO;
            Float64 hostCycleStartBeat          = 0;
            Float64 hostCycleEndBeat            = 0;
            OSStatus err = callbackInfo.transportStateProc2 (callbackInfo.hostUserData,
                                                             &hostIsPlaying,
                                                             &hostIsRecording,
                                                             nullptr,
                                                             &hostCurrentSampleInTimeLine,
                                                             &hostIsCycling,
                                                             &hostCycleStartBeat,
                                                             &hostCycleEndBeat);
            if (err == kAUGraphErr_CannotDoInCurrentContext)
                return false;

            jassert (err == noErr);

            result.timeInSamples = (int64) hostCurrentSampleInTimeLine;
            result.isPlaying     = hostIsPlaying;
            result.isRecording   = hostIsRecording;
            result.isLooping     = hostIsCycling;
            result.ppqLoopStart  = hostCycleStartBeat;
            result.ppqLoopEnd    = hostCycleEndBeat;

            result.timeInSeconds = result.timeInSamples / sampleRate;

            Float64 hostBeat = 0;
            Float64 hostTempo = 0;
            err = callbackInfo.beatAndTempoProc (callbackInfo.hostUserData,
                                                 &hostBeat,
                                                 &hostTempo);
            jassert (err == noErr);

            result.ppqPosition = hostBeat;
            result.bpm         = hostTempo;

            Float32 hostTimeSigNumerator = 0;
            UInt32 hostTimeSigDenominator = 0;
            Float64 hostCurrentMeasureDownBeat = 0;
            err = callbackInfo.musicalTimeLocationProc (callbackInfo.hostUserData,
                                                        nullptr,
                                                        &hostTimeSigNumerator,
                                                        &hostTimeSigDenominator,
                                                        &hostCurrentMeasureDownBeat);
            jassert (err == noErr);

            result.ppqPositionOfLastBarStart = hostCurrentMeasureDownBeat;
            result.timeSigNumerator          = (int) hostTimeSigNumerator;
            result.timeSigDenominator        = (int) hostTimeSigDenominator;

            result.frameRate = AudioPlayHead::fpsUnknown;

            return true;
        */
    }
}

impl<'a> Drop for iOSAudioIODevicePimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            sessionHolder->activeDevices.removeFirstMatchingValue (this);

            close();
         */
    }
}

#[cfg(TARGET_IPHONE_SIMULATOR)]
pub const IOS_AUDIO_IO_DEVICE_DEFAULT_BUFFER_SIZE: i32 = 512;

#[cfg(not(TARGET_IPHONE_SIMULATOR))]
pub const IOS_AUDIO_IO_DEVICE_DEFAULT_BUFFER_SIZE: i32 = 256;

impl<'a> iOSAudioIODevicePimpl<'a> {

    pub fn new(
        io_device_type: *mut iOSAudioIODeviceType,
        io_device:      &mut iOSAudioIODevice) -> Self {
    
        todo!();
        /*


            : deviceType (ioDeviceType),
              owner (ioDevice)

            ALOE_IOS_AUDIO_LOG ("Creating iOS audio device");

            // We need to activate the audio session here to obtain the available sample rates and buffer sizes,
            // but if we don't set a category first then background audio will always be stopped. This category
            // may be changed later.
            setAudioSessionCategory (AVAudioSessionCategoryPlayAndRecord);

            setAudioSessionActive (true);
            updateHardwareInfo();
            channelData.reconfigure ({}, {});
            setAudioSessionActive (false);

            sessionHolder->activeDevices.add (this);
        */
    }
    
    pub fn set_audio_session_category(category: *mut NSString)  {
        
        todo!();
        /*
            NSUInteger options = 0;

           #if ! ALOE_DISABLE_AUDIO_MIXING_WITH_OTHER_APPS
            options |= AVAudioSessionCategoryOptionMixWithOthers; // Alternatively AVAudioSessionCategoryOptionDuckOthers
           #endif

            if (category == AVAudioSessionCategoryPlayAndRecord)
                options |= (AVAudioSessionCategoryOptionDefaultToSpeaker
                          | AVAudioSessionCategoryOptionAllowBluetooth
                          | AVAudioSessionCategoryOptionAllowBluetoothA2DP);

            ALOE_NSERROR_CHECK ([[AVAudioSession sharedInstance] setCategory: category
                                                                 withOptions: options
                                                                       error: &error]);
        */
    }
    
    pub fn set_audio_session_active(enabled: bool)  {
        
        todo!();
        /*
            ALOE_NSERROR_CHECK ([[AVAudioSession sharedInstance] setActive: enabled
                                                                     error: &error]);
        */
    }
    
    pub fn get_buffer_size(&mut self, current_sample_rate: f64) -> i32 {
        
        todo!();
        /*
            return roundToInt (currentSampleRate * [AVAudioSession sharedInstance].IOBufferDuration);
        */
    }
    
    pub fn try_buffer_size(&mut self, 
        current_sample_rate: f64,
        new_buffer_size:     i32) -> i32 {
        
        todo!();
        /*
            NSTimeInterval bufferDuration = currentSampleRate > 0 ? (NSTimeInterval) ((newBufferSize + 1) / currentSampleRate) : 0.0;

            auto session = [AVAudioSession sharedInstance];
            ALOE_NSERROR_CHECK ([session setPreferredIOBufferDuration: bufferDuration
                                                                error: &error]);

            return getBufferSize (currentSampleRate);
        */
    }
    
    pub fn update_available_buffer_sizes(&mut self)  {
        
        todo!();
        /*
            availableBufferSizes.clear();

            auto newBufferSize = tryBufferSize (sampleRate, 64);
            jassert (newBufferSize > 0);

            const auto longestBufferSize  = tryBufferSize (sampleRate, 4096);

            while (newBufferSize <= longestBufferSize)
            {
                availableBufferSizes.add (newBufferSize);
                newBufferSize *= 2;
            }

            // Sometimes the largest supported buffer size is not a power of 2
            availableBufferSizes.addIfNotAlreadyThere (longestBufferSize);

            bufferSize = tryBufferSize (sampleRate, bufferSize);

           #if ALOE_IOS_AUDIO_LOGGING
            {
                String info ("Available buffer sizes:");

                for (auto size : availableBufferSizes)
                    info << " " << size;

                ALOE_IOS_AUDIO_LOG (info);
            }
           #endif

            ALOE_IOS_AUDIO_LOG ("Buffer size after detecting available buffer sizes: " << bufferSize);
        */
    }
    
    pub fn try_sample_rate(&mut self, rate: f64) -> f64 {
        
        todo!();
        /*
            auto session = [AVAudioSession sharedInstance];
            ALOE_NSERROR_CHECK ([session setPreferredSampleRate: rate
                                                          error: &error]);

            return session.sampleRate;
        */
    }

    /**
      | Important: the supported audio sample rates
      | change on the iPhone 6S depending on
      | whether the headphones are plugged in or
      | not!
      */
    pub fn update_available_sample_rates(&mut self)  {
        
        todo!();
        /*
            availableSampleRates.clear();

            AudioUnitRemovePropertyListenerWithUserData (audioUnit,
                                                         kAudioUnitProperty_StreamFormat,
                                                         dispatchAudioUnitPropertyChange,
                                                         this);

            const double lowestRate = trySampleRate (4000);
            availableSampleRates.add (lowestRate);
            const double highestRate = trySampleRate (192000);

            ALOE_IOS_AUDIO_LOG ("Lowest supported sample rate: "  << lowestRate);
            ALOE_IOS_AUDIO_LOG ("Highest supported sample rate: " << highestRate);

            for (double rate = lowestRate + 1000; rate < highestRate; rate += 1000)
            {
                const double supportedRate = trySampleRate (rate);
                ALOE_IOS_AUDIO_LOG ("Trying a sample rate of " << rate << ", got " << supportedRate);
                availableSampleRates.addIfNotAlreadyThere (supportedRate);
                rate = jmax (rate, supportedRate);
            }

            availableSampleRates.addIfNotAlreadyThere (highestRate);

            // Restore the original values.
            sampleRate = trySampleRate (sampleRate);
            bufferSize = tryBufferSize (sampleRate, bufferSize);

            AudioUnitAddPropertyListener (audioUnit,
                                          kAudioUnitProperty_StreamFormat,
                                          dispatchAudioUnitPropertyChange,
                                          this);

            // Check the current stream format in case things have changed whilst we
            // were going through the sample rates
            handleStreamFormatChange();

           #if ALOE_IOS_AUDIO_LOGGING
            {
                String info ("Available sample rates:");

                for (auto rate : availableSampleRates)
                    info << " " << rate;

                ALOE_IOS_AUDIO_LOG (info);
            }
           #endif

            ALOE_IOS_AUDIO_LOG ("Sample rate after detecting available sample rates: " << sampleRate);
        */
    }
    
    pub fn update_hardware_info(&mut self, force_update: Option<bool>)  {

        let force_update: bool = force_update.unwrap_or(false);

        todo!();
        /*
            if (! forceUpdate && ! hardwareInfoNeedsUpdating.compareAndSetBool (false, true))
                return;

            ALOE_IOS_AUDIO_LOG ("Updating hardware info");

            updateAvailableSampleRates();
            updateAvailableBufferSizes();

            if (deviceType != nullptr)
                deviceType->callDeviceChangeListeners();
        */
    }
    
    pub fn set_target_sample_rate_and_buffer_size(&mut self)  {
        
        todo!();
        /*
            ALOE_IOS_AUDIO_LOG ("Setting target sample rate: " << targetSampleRate);
            sampleRate = trySampleRate (targetSampleRate);
            ALOE_IOS_AUDIO_LOG ("Actual sample rate: " << sampleRate);

            ALOE_IOS_AUDIO_LOG ("Setting target buffer size: " << targetBufferSize);
            bufferSize = tryBufferSize (sampleRate, targetBufferSize);
            ALOE_IOS_AUDIO_LOG ("Actual buffer size: " << bufferSize);
        */
    }
    
    pub fn open(&mut self, 
        input_channels_wanted:  &BigInteger,
        output_channels_wanted: &BigInteger,
        sample_rate_wanted:     f64,
        buffer_size_wanted:     i32) -> String {
        
        todo!();
        /*
            close();

            firstHostTime = true;
            lastNumFrames = 0;
            xrun = 0;
            lastError.clear();

            requestedInputChannels  = inputChannelsWanted;
            requestedOutputChannels = outputChannelsWanted;
            targetSampleRate = sampleRateWanted;
            targetBufferSize = bufferSizeWanted > 0 ? bufferSizeWanted : defaultBufferSize;

            ALOE_IOS_AUDIO_LOG ("Opening audio device:"
                                <<  " inputChannelsWanted: "  << requestedInputChannels .toString (2)
                                << ", outputChannelsWanted: " << requestedOutputChannels.toString (2)
                                << ", targetSampleRate: " << targetSampleRate
                                << ", targetBufferSize: " << targetBufferSize);

            setAudioSessionActive (true);
            setAudioSessionCategory (requestedInputChannels > 0 ? AVAudioSessionCategoryPlayAndRecord
                                                                : AVAudioSessionCategoryPlayback);
            channelData.reconfigure (requestedInputChannels, requestedOutputChannels);
            updateHardwareInfo (true);
            setTargetSampleRateAndBufferSize();
            fixAudioRouteIfSetToReceiver();

            isRunning = true;

            if (! createAudioUnit())
            {
                lastError = "Couldn't open the device";
                return lastError;
            }

            const ScopedLock sl (callbackLock);

            AudioOutputUnitStart (audioUnit);

            if (callback != nullptr)
                callback->audioDeviceAboutToStart (&owner);

            return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();

            if (isRunning)
            {
                isRunning = false;

                if (audioUnit != nullptr)
                {
                    AudioOutputUnitStart (audioUnit);
                    AudioComponentInstanceDispose (audioUnit);
                    audioUnit = nullptr;
                }

                setAudioSessionActive (false);
            }
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (isRunning && callback != newCallback)
            {
                if (newCallback != nullptr)
                    newCallback->audioDeviceAboutToStart (&owner);

                const ScopedLock sl (callbackLock);
                callback = newCallback;
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (isRunning)
            {
                AudioIODeviceCallback* lastCallback;

                {
                    const ScopedLock sl (callbackLock);
                    lastCallback = callback;
                    callback = nullptr;
                }

                if (lastCallback != nullptr)
                    lastCallback->audioDeviceStopped();
            }
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, enable: bool) -> bool {
        
        todo!();
        /*
            auto session = [AVAudioSession sharedInstance];

            NSString* mode = (enable ? AVAudioSessionModeDefault
                                     : AVAudioSessionModeMeasurement);

            ALOE_NSERROR_CHECK ([session setMode: mode
                                           error: &error]);

            return session.mode == mode;
        */
    }
    
    pub fn can_control_transport(&mut self) -> bool {
        
        todo!();
        /*
            return interAppAudioConnected;
        */
    }
    
    pub fn transport_play(&mut self, should_sart_playing: bool)  {
        
        todo!();
        /*
            if (! canControlTransport())
                return;

            HostCallbackInfo callbackInfo;
            fillHostCallbackInfo (callbackInfo);

            Boolean hostIsPlaying = NO;
            OSStatus err = callbackInfo.transportStateProc2 (callbackInfo.hostUserData,
                                                             &hostIsPlaying,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr);

            ignoreUnused (err);
            jassert (err == noErr);

            if (hostIsPlaying != shouldSartPlaying)
                handleAudioTransportEvent (kAudioUnitRemoteControlEvent_TogglePlayPause);
        */
    }
    
    pub fn transport_record(&mut self, should_start_recording: bool)  {
        
        todo!();
        /*
            if (! canControlTransport())
                return;

            HostCallbackInfo callbackInfo;
            fillHostCallbackInfo (callbackInfo);

            Boolean hostIsRecording = NO;
            OSStatus err = callbackInfo.transportStateProc2 (callbackInfo.hostUserData,
                                                             nullptr,
                                                             &hostIsRecording,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr,
                                                             nullptr);
            ignoreUnused (err);
            jassert (err == noErr);

            if (hostIsRecording != shouldStartRecording)
                handleAudioTransportEvent (kAudioUnitRemoteControlEvent_ToggleRecord);
        */
    }
    
    pub fn transport_rewind(&mut self)  {
        
        todo!();
        /*
            if (canControlTransport())
                handleAudioTransportEvent (kAudioUnitRemoteControlEvent_Rewind);
        */
    }
    
    #[cfg(ALOE_MODULE_AVAILABLE_aloe_graphics)]
    pub fn get_icon(&mut self, size: i32) -> Image {
        
        todo!();
        /*
            if (interAppAudioConnected)
            {
                UIImage* hostUIImage = AudioOutputUnitGetHostIcon (audioUnit, size);
                if (hostUIImage != nullptr)
                    return aloe_createImageFromUIImage (hostUIImage);
            }
            return Image();
        */
    }
    
    pub fn switch_application(&mut self)  {
        
        todo!();
        /*
            if (! interAppAudioConnected)
                return;

            CFURLRef hostUrl;
            UInt32 dataSize = sizeof (hostUrl);
            OSStatus err = AudioUnitGetProperty(audioUnit,
                                                kAudioUnitProperty_PeerURL,
                                                kAudioUnitScope_Global,
                                                0,
                                                &hostUrl,
                                                &dataSize);
            if (err == noErr)
            {
               #if (! defined __IPHONE_10_0) || (__IPHONE_OS_VERSION_MIN_REQUIRED < __IPHONE_10_0)
                [[UIApplication sharedApplication] openURL: (NSURL*)hostUrl];
               #else
                [[UIApplication sharedApplication] openURL: (NSURL*)hostUrl options: @{} completionHandler: nil];
               #endif
            }
        */
    }
    
    pub fn invoke_audio_device_error_callback(&mut self, reason: &String)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            if (callback != nullptr)
                callback->audioDeviceError (reason);
        */
    }
    
    pub fn handle_status_change(&mut self, 
        enabled: bool,
        reason:  *const u8)  {
        
        todo!();
        /*
            const ScopedLock myScopedLock (callbackLock);

            ALOE_IOS_AUDIO_LOG ("handleStatusChange: enabled: " << (int) enabled << ", reason: " << reason);

            isRunning = enabled;
            setAudioSessionActive (enabled);

            if (enabled)
                AudioOutputUnitStart (audioUnit);
            else
                AudioOutputUnitStop (audioUnit);

            if (! enabled)
                invokeAudioDeviceErrorCallback (reason);
        */
    }
    
    pub fn handle_route_change(&mut self, reason: AVAudioSessionRouteChangeReason)  {
        
        todo!();
        /*
            const ScopedLock myScopedLock (callbackLock);

            const String reasonString (getRoutingChangeReason (reason));
            ALOE_IOS_AUDIO_LOG ("handleRouteChange: " << reasonString);

            if (isRunning)
                invokeAudioDeviceErrorCallback (reasonString);

            switch (reason)
            {
            case AVAudioSessionRouteChangeReasonCategoryChange:
            case AVAudioSessionRouteChangeReasonOverride:
            case AVAudioSessionRouteChangeReasonRouteConfigurationChange:
                break;
            case AVAudioSessionRouteChangeReasonUnknown:
            case AVAudioSessionRouteChangeReasonNewDeviceAvailable:
            case AVAudioSessionRouteChangeReasonOldDeviceUnavailable:
            case AVAudioSessionRouteChangeReasonWakeFromSleep:
            case AVAudioSessionRouteChangeReasonNoSuitableRouteForCategory:
            {
                hardwareInfoNeedsUpdating = true;
                triggerAsyncUpdate();
                break;
            }

            // No default so the code doesn't compile if this enum is extended.
            }
        */
    }
    
    pub fn handle_audio_unit_property_change(&mut self, 
        _0:         AudioUnit,
        propertyid: AudioUnitPropertyID,
        scope:      AUScope,
        element:    AUElement)  {
        
        todo!();
        /*
            ignoreUnused (scope);
            ignoreUnused (element);
            ALOE_IOS_AUDIO_LOG ("handleAudioUnitPropertyChange: propertyID: " << String (propertyID)
                                                                << " scope: " << String (scope)
                                                              << " element: " << String (element));

            switch (propertyID)
            {
                case kAudioUnitProperty_IsInterAppConnected:
                    handleInterAppAudioConnectionChange();
                    return;
                case kAudioUnitProperty_StreamFormat:
                    handleStreamFormatChange();
                    return;
                default:
                    jassertfalse;
            }
        */
    }
    
    pub fn handle_inter_app_audio_connection_change(&mut self)  {
        
        todo!();
        /*
            UInt32 connected;
            UInt32 dataSize = sizeof (connected);
            OSStatus err = AudioUnitGetProperty (audioUnit, kAudioUnitProperty_IsInterAppConnected,
                                                 kAudioUnitScope_Global, 0, &connected, &dataSize);
            ignoreUnused (err);
            jassert (err == noErr);

            ALOE_IOS_AUDIO_LOG ("handleInterAppAudioConnectionChange: " << (connected ? "connected"
                                                                                      : "disconnected"));

            if (connected != interAppAudioConnected)
            {
                const ScopedLock myScopedLock (callbackLock);

                interAppAudioConnected = connected;

                UIApplicationState appstate = [UIApplication sharedApplication].applicationState;
                bool inForeground = (appstate != UIApplicationStateBackground);

                if (interAppAudioConnected || inForeground)
                {
                    setAudioSessionActive (true);
                    AudioOutputUnitStart (audioUnit);

                    if (callback != nullptr)
                        callback->audioDeviceAboutToStart (&owner);
                }
                else if (! inForeground)
                {
                    AudioOutputUnitStop (audioUnit);
                    setAudioSessionActive (false);

                    if (callback != nullptr)
                        callback->audioDeviceStopped();
                }
            }
        */
    }
    
    pub fn process(&mut self, 
        flags:      *mut AudioUnitRenderActionFlags,
        time:       *const AudioTimeStamp,
        num_frames: u32,
        data:       *mut CoreAudioBufferList) -> OSStatus {
        
        todo!();
        /*
            OSStatus err = noErr;

            recordXruns (time, numFrames);

            const bool useInput = channelData.areInputChannelsAvailable();

            if (useInput)
                err = AudioUnitRender (audioUnit, flags, time, 1, numFrames, data);

            const auto channelDataSize = sizeof (float) * numFrames;

            const ScopedTryLock stl (callbackLock);

            if (stl.isLocked() && callback != nullptr)
            {
                if ((int) numFrames > channelData.getFloatBufferSize())
                    channelData.setFloatBufferSize ((int) numFrames);

                float** const inputData = channelData.audioData.getArrayOfWritePointers();
                float** const outputData = inputData + channelData.inputs->numActiveChannels;

                if (useInput)
                {
                    for (int c = 0; c < channelData.inputs->numActiveChannels; ++c)
                    {
                        auto channelIndex = channelData.inputs->activeChannelIndices[c];
                        memcpy (inputData[c], (float*) data->mBuffers[channelIndex].mData, channelDataSize);
                    }
                }
                else
                {
                    for (int c = 0; c < channelData.inputs->numActiveChannels; ++c)
                        zeromem (inputData[c], channelDataSize);
                }

                callback->audioDeviceIOCallback ((const float**) inputData,  channelData.inputs ->numActiveChannels,
                                                                 outputData, channelData.outputs->numActiveChannels,
                                                 (int) numFrames);

                for (int c = 0; c < channelData.outputs->numActiveChannels; ++c)
                {
                    auto channelIndex = channelData.outputs->activeChannelIndices[c];
                    memcpy (data->mBuffers[channelIndex].mData, outputData[c], channelDataSize);
                }

                for (auto c : channelData.outputs->inactiveChannelIndices)
                    zeromem (data->mBuffers[c].mData, channelDataSize);
            }
            else
            {
                for (uint32 c = 0; c < data->mNumberBuffers; ++c)
                    zeromem (data->mBuffers[c].mData, channelDataSize);
            }

            return err;
        */
    }
    
    pub fn record_xruns(&mut self, 
        time:       *const AudioTimeStamp,
        num_frames: u32)  {
        
        todo!();
        /*
            if (time != nullptr && (time->mFlags & kAudioTimeStampSampleTimeValid) != 0)
            {
                if (! firstHostTime)
                {
                    if ((time->mSampleTime - lastSampleTime) != lastNumFrames)
                        xrun++;
                }
                else
                    firstHostTime = false;

                lastSampleTime = time->mSampleTime;
            }
            else
                firstHostTime = true;

            lastNumFrames = numFrames;
        */
    }
    
    pub fn process_static(
        client:     *mut c_void,
        flags:      *mut AudioUnitRenderActionFlags,
        time:       *const AudioTimeStamp,
        bus_number: u32,
        num_frames: u32,
        data:       *mut CoreAudioBufferList) -> OSStatus {
        
        todo!();
        /*
            return static_cast<Pimpl*> (client)->process (flags, time, numFrames, data);
        */
    }
    
    pub fn create_audio_unit(&mut self) -> bool {
        
        todo!();
        /*
            ALOE_IOS_AUDIO_LOG ("Creating the audio unit");

            if (audioUnit != nullptr)
            {
                AudioComponentInstanceDispose (audioUnit);
                audioUnit = nullptr;
            }

            AudioComponentDescription desc;
            desc.componentType = kAudioUnitType_Output;
            desc.componentSubType = kAudioUnitSubType_RemoteIO;
            desc.componentManufacturer = kAudioUnitManufacturer_Apple;
            desc.componentFlags = 0;
            desc.componentFlagsMask = 0;

            AudioComponent comp = AudioComponentFindNext (nullptr, &desc);
            AudioComponentInstanceNew (comp, &audioUnit);

            if (audioUnit == nullptr)
                return false;

           #if AloePlugin_Enable_IAA
            AudioComponentDescription appDesc;
            appDesc.componentType = AloePlugin_IAAType;
            appDesc.componentSubType = AloePlugin_IAASubType;
            appDesc.componentManufacturer = AloePlugin_ManufacturerCode;
            appDesc.componentFlags = 0;
            appDesc.componentFlagsMask = 0;
            OSStatus err = AudioOutputUnitPublish (&appDesc,
                                                   CFSTR(AloePlugin_IAAName),
                                                   AloePlugin_VersionCode,
                                                   audioUnit);

            // This assert will be hit if the Inter-App Audio entitlement has not
            // been enabled, or the description being published with
            // AudioOutputUnitPublish is different from any in the AudioComponents
            // array in this application's .plist file.
            jassert (err == noErr);

            err = AudioUnitAddPropertyListener (audioUnit,
                                                kAudioUnitProperty_IsInterAppConnected,
                                                dispatchAudioUnitPropertyChange,
                                                this);
            jassert (err == noErr);

            AudioOutputUnitMIDICallbacks midiCallbacks;
            midiCallbacks.userData = this;
            midiCallbacks.MIDIEventProc = midiEventCallback;
            midiCallbacks.MIDISysExProc = midiSysExCallback;
            err = AudioUnitSetProperty (audioUnit,
                                        kAudioOutputUnitProperty_MIDICallbacks,
                                        kAudioUnitScope_Global,
                                        0,
                                        &midiCallbacks,
                                        sizeof (midiCallbacks));
            jassert (err == noErr);
           #endif

            if (channelData.areInputChannelsAvailable())
            {
                const UInt32 one = 1;
                AudioUnitSetProperty (audioUnit, kAudioOutputUnitProperty_EnableIO, kAudioUnitScope_Input, 1, &one, sizeof (one));
            }

            {
                AURenderCallbackStruct inputProc;
                inputProc.inputProc = processStatic;
                inputProc.inputProcRefCon = this;
                AudioUnitSetProperty (audioUnit, kAudioUnitProperty_SetRenderCallback, kAudioUnitScope_Input, 0, &inputProc, sizeof (inputProc));
            }

            {
                AudioStreamBasicDescription format;
                zerostruct (format);
                format.mSampleRate = sampleRate;
                format.mFormatID = kAudioFormatLinearPCM;
                format.mFormatFlags = kAudioFormatFlagIsFloat | kAudioFormatFlagIsNonInterleaved | kAudioFormatFlagsNativeEndian | kLinearPCMFormatFlagIsPacked;
                format.mBitsPerChannel = 8 * sizeof (float);
                format.mFramesPerPacket = 1;
                format.mChannelsPerFrame = (UInt32) jmax (channelData.inputs->numHardwareChannels, channelData.outputs->numHardwareChannels);
                format.mBytesPerFrame = format.mBytesPerPacket = sizeof (float);

                AudioUnitSetProperty (audioUnit, kAudioUnitProperty_StreamFormat, kAudioUnitScope_Input,  0, &format, sizeof (format));
                AudioUnitSetProperty (audioUnit, kAudioUnitProperty_StreamFormat, kAudioUnitScope_Output, 1, &format, sizeof (format));
            }

            AudioUnitInitialize (audioUnit);

            {
                // Querying the kAudioUnitProperty_MaximumFramesPerSlice property after calling AudioUnitInitialize
                // seems to be more reliable than calling it before.
                UInt32 framesPerSlice, dataSize = sizeof (framesPerSlice);

                if (AudioUnitGetProperty (audioUnit, kAudioUnitProperty_MaximumFramesPerSlice,
                                          kAudioUnitScope_Global, 0, &framesPerSlice, &dataSize) == noErr
                        && dataSize == sizeof (framesPerSlice)
                        && static_cast<int> (framesPerSlice) != bufferSize)
                {
                    ALOE_IOS_AUDIO_LOG ("Internal buffer size: " << String (framesPerSlice));
                    channelData.setFloatBufferSize (static_cast<int> (framesPerSlice));
                }
            }

            AudioUnitAddPropertyListener (audioUnit, kAudioUnitProperty_StreamFormat, dispatchAudioUnitPropertyChange, this);

            return true;
        */
    }
    
    pub fn fill_host_callback_info(&mut self, callback_info: &mut HostCallbackInfo)  {
        
        todo!();
        /*
            zerostruct (callbackInfo);
            UInt32 dataSize = sizeof (HostCallbackInfo);
            OSStatus err = AudioUnitGetProperty (audioUnit,
                                                 kAudioUnitProperty_HostCallbacks,
                                                 kAudioUnitScope_Global,
                                                 0,
                                                 &callbackInfo,
                                                 &dataSize);
            ignoreUnused (err);
            jassert (err == noErr);
        */
    }
    
    pub fn handle_audio_transport_event(&mut self, event: AudioUnitRemoteControlEvent)  {
        
        todo!();
        /*
            OSStatus err = AudioUnitSetProperty (audioUnit, kAudioOutputUnitProperty_RemoteControlToHost,
                                                 kAudioUnitScope_Global, 0, &event, sizeof (event));
            ignoreUnused (err);
            jassert (err == noErr);
        */
    }

    /**
      | If the routing is set to go through the
      | receiver (i.e. the speaker, but quiet),
      | this re-routes it to make it loud. Needed
      | because by default when using an input
      | + output, the output is kept quiet.
      */
    pub fn fix_audio_route_if_set_to_receiver()  {
        
        todo!();
        /*
            auto session = [AVAudioSession sharedInstance];
            auto route = session.currentRoute;

            for (AVAudioSessionPortDescription* port in route.outputs)
            {
                if ([port.portName isEqualToString: @"Receiver"])
                {
                    ALOE_NSERROR_CHECK ([session overrideOutputAudioPort: AVAudioSessionPortOverrideSpeaker
                                                                   error: &error]);
                    setAudioSessionActive (true);
                }
            }
        */
    }
    
    pub fn restart(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            updateHardwareInfo();
            setTargetSampleRateAndBufferSize();

            if (isRunning)
            {
                if (audioUnit != nullptr)
                {
                    AudioComponentInstanceDispose (audioUnit);
                    audioUnit = nullptr;

                    if (callback != nullptr)
                        callback->audioDeviceStopped();
                }

                channelData.reconfigure (requestedInputChannels, requestedOutputChannels);

                createAudioUnit();

                if (audioUnit != nullptr)
                {
                    isRunning = true;

                    if (callback != nullptr)
                        callback->audioDeviceAboutToStart (&owner);

                    AudioOutputUnitStart (audioUnit);
                }
            }
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            restart();
        */
    }
    
    pub fn handle_stream_format_change(&mut self)  {
        
        todo!();
        /*
            AudioStreamBasicDescription desc;
            zerostruct (desc);
            UInt32 dataSize = sizeof (desc);
            AudioUnitGetProperty (audioUnit,
                                  kAudioUnitProperty_StreamFormat,
                                  kAudioUnitScope_Output,
                                  0,
                                  &desc,
                                  &dataSize);

            if (desc.mSampleRate != 0 && desc.mSampleRate != sampleRate)
            {
                ALOE_IOS_AUDIO_LOG ("Stream format has changed: Sample rate " << desc.mSampleRate);
                triggerAsyncUpdate();
            }
        */
    }
    
    pub fn dispatch_audio_unit_property_change(
        data:       *mut c_void,
        unit:       AudioUnit,
        propertyid: AudioUnitPropertyID,
        scope:      AUScope,
        element:    AUElement)  {
        
        todo!();
        /*
            static_cast<Pimpl*> (data)->handleAudioUnitPropertyChange (unit, propertyID, scope, element);
        */
    }
    
    pub fn get_timestamp_formidi() -> f64 {
        
        todo!();
        /*
            return Time::getMillisecondCounter() / 1000.0;
        */
    }
    
    pub fn midi_event_callback(
        client: *mut c_void,
        status: u32,
        data1:  u32,
        data2:  u32,
        _4:     u32)  {
        
        todo!();
        /*
            return static_cast<Pimpl*> (client)->handleMidiMessage (MidiMessage ((int) status,
                                                                                 (int) data1,
                                                                                 (int) data2,
                                                                                 getTimestampForMIDI()));
        */
    }
    
    pub fn midi_sys_ex_callback(
        client: *mut c_void,
        data:   *const u8,
        length: u32

    ) {
        
        todo!();
        /*
            return static_cast<Pimpl*> (client)->handleMidiMessage (MidiMessage (data, (int) length, getTimestampForMIDI()));
        */
    }
    
    pub fn handle_midi_message(&mut self, msg: MidiMessage)  {
        
        todo!();
        /*
            if (messageCollector != nullptr)
                messageCollector->addMessageToQueue (msg);
        */
    }
}

