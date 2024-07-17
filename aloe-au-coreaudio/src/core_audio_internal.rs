crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CoreAudioInternal<'a> {
    base:                      Timer,
    owner:                     &'a mut CoreAudioIODevice<'a>,
    input_latency:             i32, // default = 0
    output_latency:            i32, // default = 0
    bit_depth:                 i32, // default = 32
    xruns:                     i32, // default = 0
    active_input_chans:        BigInteger,
    active_output_chans:       BigInteger,
    in_chan_names:             StringArray,
    out_chan_names:            StringArray,
    sample_rates:              Vec<f64>,
    buffer_sizes:              Vec<i32>,
    callback:                  *mut dyn AudioIODeviceCallback, // default = nullptr
    audio_procid:              AudioDeviceIOProcID,
    callback_lock:             CriticalSection,
    deviceid:                  AudioDeviceID,
    started:                   bool, // default = false
    audio_device_stop_pending: bool, // default = false
    sample_rate:               f64, // default = 0
    buffer_size:               i32, // default = 512
    audio_buffer:              HeapBlock<f32>,
    num_input_chans:           i32, // default = 0
    num_output_chans:          i32, // default = 0
    callbacks_allowed:         Atomic<i32>, // default = 1 
    is_input_device:           bool,
    is_output_device:          bool,
    input_channel_info:        Vec<CallbackDetailsForChannel>,
    output_channel_info:       Vec<CallbackDetailsForChannel>,
    temp_input_buffers:        HeapBlock<*mut f32>,
    temp_output_buffers:       HeapBlock<*mut f32>,
}

impl<'a> Drop for CoreAudioInternal<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioObjectPropertySelectorWildcard;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = kAudioObjectPropertyElementWildcard;

                AudioObjectRemovePropertyListener (deviceID, &pa, deviceListenerProc, this);

                stop (false);
             */
    }
}

impl<'a> CoreAudioInternal<'a> {

    pub fn new(
        d:      &mut CoreAudioIODevice,
        id:     AudioDeviceID,
        input:  bool,
        output: bool) -> Self {
    
        todo!();
        /*
        : owner(d),
        : deviceid(id),
        : is_input_device(input),
        : is_output_device(output),

            jassert (deviceID != 0);

                updateDetailsFromDevice();
                ALOE_COREAUDIOLOG ("Creating CoreAudioInternal\n"
                                   << (isInputDevice  ? ("    inputDeviceId "  + String (deviceID) + "\n") : "")
                                   << (isOutputDevice ? ("    outputDeviceId " + String (deviceID) + "\n") : "")
                                   << getDeviceDetails().joinIntoString ("\n    "));

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioObjectPropertySelectorWildcard;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = kAudioObjectPropertyElementWildcard;

                AudioObjectAddPropertyListener (deviceID, &pa, deviceListenerProc, this);
        */
    }
    
    pub fn allocate_temp_buffers(&mut self)  {
        
        todo!();
        /*
            auto tempBufSize = bufferSize + 4;
                audioBuffer.calloc ((numInputChans + numOutputChans) * tempBufSize);

                tempInputBuffers.calloc  (numInputChans + 2);
                tempOutputBuffers.calloc (numOutputChans + 2);

                int count = 0;
                for (int i = 0; i < numInputChans;  ++i)  tempInputBuffers[i]  = audioBuffer + count++ * tempBufSize;
                for (int i = 0; i < numOutputChans; ++i)  tempOutputBuffers[i] = audioBuffer + count++ * tempBufSize;
        */
    }

    /**
       returns the number of actual available
       channels
      */
    pub fn get_channel_info(&self, 
        input:            bool,
        new_channel_info: &mut Vec<CallbackDetailsForChannel>) -> StringArray {
        
        todo!();
        /*
            StringArray newNames;
                int chanNum = 0;
                UInt32 size;

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyStreamConfiguration;
                pa.mScope = input ? kAudioDevicePropertyScopeInput : kAudioDevicePropertyScopeOutput;
                pa.mElement = aloeAudioObjectPropertyElementMain;

                if (OK (AudioObjectGetPropertyDataSize (deviceID, &pa, 0, nullptr, &size)))
                {
                    HeapBlock<AudioBufferList> bufList;
                    bufList.calloc (size, 1);

                    if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, bufList)))
                    {
                        const int numStreams = (int) bufList->mNumberBuffers;

                        for (int i = 0; i < numStreams; ++i)
                        {
                            auto& b = bufList->mBuffers[i];

                            for (unsigned int j = 0; j < b.mNumberChannels; ++j)
                            {
                                String name;
                                NSString* nameNSString = nil;
                                size = sizeof (nameNSString);

                                pa.mSelector = kAudioObjectPropertyElementName;
                                pa.mElement = (AudioObjectPropertyElement) chanNum + 1;

                                if (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &nameNSString) == noErr)
                                {
                                    name = nsStringToAloe (nameNSString);
                                    [nameNSString release];
                                }

                                if ((input ? activeInputChans : activeOutputChans) [chanNum])
                                {
                                    CallbackDetailsForChannel info = { i, (int) j, (int) b.mNumberChannels };
                                    newChannelInfo.add (info);
                                }

                                if (name.isEmpty())
                                    name << (input ? "Input " : "Output ") << (chanNum + 1);

                                newNames.add (name);
                                ++chanNum;
                            }
                        }
                    }
                }

                return newNames;
        */
    }
    
    pub fn get_sample_rates_from_device(&self) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> newSampleRates;

                AudioObjectPropertyAddress pa;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioDevicePropertyAvailableNominalSampleRates;
                UInt32 size = 0;

                if (OK (AudioObjectGetPropertyDataSize (deviceID, &pa, 0, nullptr, &size)))
                {
                    HeapBlock<AudioValueRange> ranges;
                    ranges.calloc (size, 1);

                    if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, ranges)))
                    {
                        for (auto r : { 8000, 11025, 16000, 22050, 32000,
                                        44100, 48000, 88200, 96000, 176400,
                                        192000, 352800, 384000, 705600, 768000 })
                        {
                            auto rate = (double) r;

                            for (int j = size / (int) sizeof (AudioValueRange); --j >= 0;)
                            {
                                if (rate >= ranges[j].mMinimum - 2 && rate <= ranges[j].mMaximum + 2)
                                {
                                    newSampleRates.add (rate);
                                    break;
                                }
                            }
                        }
                    }
                }

                if (newSampleRates.isEmpty() && sampleRate > 0)
                    newSampleRates.add (sampleRate);

                return newSampleRates;
        */
    }
    
    pub fn get_buffer_sizes_from_device(&self) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> newBufferSizes;

                AudioObjectPropertyAddress pa;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioDevicePropertyBufferFrameSizeRange;
                UInt32 size = 0;

                if (OK (AudioObjectGetPropertyDataSize (deviceID, &pa, 0, nullptr, &size)))
                {
                    HeapBlock<AudioValueRange> ranges;
                    ranges.calloc (size, 1);

                    if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, ranges)))
                    {
                        newBufferSizes.add ((int) (ranges[0].mMinimum + 15) & ~15);

                        for (int i = 32; i <= 2048; i += 32)
                        {
                            for (int j = size / (int) sizeof (AudioValueRange); --j >= 0;)
                            {
                                if (i >= ranges[j].mMinimum && i <= ranges[j].mMaximum)
                                {
                                    newBufferSizes.addIfNotAlreadyThere (i);
                                    break;
                                }
                            }
                        }

                        if (bufferSize > 0)
                            newBufferSizes.addIfNotAlreadyThere (bufferSize);
                    }
                }

                if (newBufferSizes.isEmpty() && bufferSize > 0)
                    newBufferSizes.add (bufferSize);

                return newBufferSizes;
        */
    }
    
    pub fn get_latency_from_device(&self, scope: AudioObjectPropertyScope) -> i32 {
        
        todo!();
        /*
            UInt32 latency = 0;
                UInt32 size = sizeof (latency);
                AudioObjectPropertyAddress pa;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioDevicePropertyLatency;
                pa.mScope = scope;
                AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &latency);

                UInt32 safetyOffset = 0;
                size = sizeof (safetyOffset);
                pa.mSelector = kAudioDevicePropertySafetyOffset;
                AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &safetyOffset);

                return (int) (latency + safetyOffset);
        */
    }
    
    pub fn get_bit_depth_from_device(&self, scope: AudioObjectPropertyScope) -> i32 {
        
        todo!();
        /*
            AudioObjectPropertyAddress pa;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioStreamPropertyPhysicalFormat;
                pa.mScope = scope;

                AudioStreamBasicDescription asbd;
                UInt32 size = sizeof (asbd);

                if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &asbd)))
                    return (int) asbd.mBitsPerChannel;

                return 0;
        */
    }
    
    pub fn get_frame_size_from_device(&self) -> i32 {
        
        todo!();
        /*
            AudioObjectPropertyAddress pa;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioDevicePropertyBufferFrameSize;

                UInt32 framesPerBuf = (UInt32) bufferSize;
                UInt32 size = sizeof (framesPerBuf);
                AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &framesPerBuf);
                return (int) framesPerBuf;
        */
    }
    
    pub fn is_device_alive(&self) -> bool {
        
        todo!();
        /*
            AudioObjectPropertyAddress pa;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                pa.mSelector = kAudioDevicePropertyDeviceIsAlive;

                UInt32 isAlive = 0;
                UInt32 size = sizeof (isAlive);
                return deviceID != 0
                        && OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &isAlive))
                        && isAlive != 0;
        */
    }
    
    pub fn update_details_from_device(&mut self) -> bool {
        
        todo!();
        /*
            stopTimer();

                if (! isDeviceAlive())
                    return false;

                // this collects all the new details from the device without any locking, then
                // locks + swaps them afterwards.

                auto newSampleRate = getNominalSampleRate();
                auto newBufferSize = getFrameSizeFromDevice();

                auto newBufferSizes = getBufferSizesFromDevice();
                auto newSampleRates = getSampleRatesFromDevice();

                auto newInputLatency  = getLatencyFromDevice (kAudioDevicePropertyScopeInput);
                auto newOutputLatency = getLatencyFromDevice (kAudioDevicePropertyScopeOutput);

                Vec<CallbackDetailsForChannel> newInChans, newOutChans;
                auto newInNames  = isInputDevice  ? getChannelInfo (true,  newInChans)  : StringArray();
                auto newOutNames = isOutputDevice ? getChannelInfo (false, newOutChans) : StringArray();

                auto inputBitDepth  = isInputDevice  ? getBitDepthFromDevice (kAudioDevicePropertyScopeInput)  : 0;
                auto outputBitDepth = isOutputDevice ? getBitDepthFromDevice (kAudioDevicePropertyScopeOutput) : 0;
                auto newBitDepth = jmax (inputBitDepth, outputBitDepth);

                {
                    const ScopedLock sl (callbackLock);

                    bitDepth = newBitDepth > 0 ? newBitDepth : 32;

                    if (newSampleRate > 0)
                        sampleRate = newSampleRate;

                    inputLatency  = newInputLatency;
                    outputLatency = newOutputLatency;
                    bufferSize = newBufferSize;

                    sampleRates.swapWith (newSampleRates);
                    bufferSizes.swapWith (newBufferSizes);

                    inChanNames.swapWith (newInNames);
                    outChanNames.swapWith (newOutNames);

                    inputChannelInfo.swapWith (newInChans);
                    outputChannelInfo.swapWith (newOutChans);

                    numInputChans  = inputChannelInfo.size();
                    numOutputChans = outputChannelInfo.size();

                    allocateTempBuffers();
                }

                return true;
        */
    }
    
    pub fn get_device_details(&mut self) -> StringArray {
        
        todo!();
        /*
            StringArray result;

                String availableSampleRates ("Available sample rates:");

                for (auto& s : sampleRates)
                    availableSampleRates << " " << s;

                result.add (availableSampleRates);
                result.add ("Sample rate: " + String (sampleRate));
                String availableBufferSizes ("Available buffer sizes:");

                for (auto& b : bufferSizes)
                    availableBufferSizes << " " << b;

                result.add (availableBufferSizes);
                result.add ("Buffer size: " + String (bufferSize));
                result.add ("Bit depth: " + String (bitDepth));
                result.add ("Input latency: " + String (inputLatency));
                result.add ("Output latency: " + String (outputLatency));
                result.add ("Input channel names: "  +  inChanNames.joinIntoString (" "));
                result.add ("Output channel names: " + outChanNames.joinIntoString (" "));

                return result;
        */
    }
    
    pub fn get_sources(&mut self, input: bool) -> StringArray {
        
        todo!();
        /*
            StringArray s;
                HeapBlock<OSType> types;
                auto num = getAllDataSourcesForDevice (deviceID, types);

                for (int i = 0; i < num; ++i)
                {
                    AudioValueTranslation avt;
                    char buffer[256];

                    avt.mInputData = &(types[i]);
                    avt.mInputDataSize = sizeof (UInt32);
                    avt.mOutputData = buffer;
                    avt.mOutputDataSize = 256;

                    UInt32 transSize = sizeof (avt);

                    AudioObjectPropertyAddress pa;
                    pa.mSelector = kAudioDevicePropertyDataSourceNameForID;
                    pa.mScope = input ? kAudioDevicePropertyScopeInput : kAudioDevicePropertyScopeOutput;
                    pa.mElement = aloeAudioObjectPropertyElementMain;

                    if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &transSize, &avt)))
                        s.add (buffer);
                }

                return s;
        */
    }
    
    pub fn get_current_source_index(&self, input: bool) -> i32 {
        
        todo!();
        /*
            OSType currentSourceID = 0;
                UInt32 size = sizeof (currentSourceID);
                int result = -1;

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyDataSource;
                pa.mScope = input ? kAudioDevicePropertyScopeInput : kAudioDevicePropertyScopeOutput;
                pa.mElement = aloeAudioObjectPropertyElementMain;

                if (deviceID != 0)
                {
                    if (OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &currentSourceID)))
                    {
                        HeapBlock<OSType> types;
                        auto num = getAllDataSourcesForDevice (deviceID, types);

                        for (int i = 0; i < num; ++i)
                        {
                            if (types[num] == currentSourceID)
                            {
                                result = i;
                                break;
                            }
                        }
                    }
                }

                return result;
        */
    }
    
    pub fn set_current_source_index(&mut self, 
        index: i32,
        input: bool)  {
        
        todo!();
        /*
            if (deviceID != 0)
                {
                    HeapBlock<OSType> types;
                    auto num = getAllDataSourcesForDevice (deviceID, types);

                    if (isPositiveAndBelow (index, num))
                    {
                        AudioObjectPropertyAddress pa;
                        pa.mSelector = kAudioDevicePropertyDataSource;
                        pa.mScope = input ? kAudioDevicePropertyScopeInput : kAudioDevicePropertyScopeOutput;
                        pa.mElement = aloeAudioObjectPropertyElementMain;

                        OSType typeId = types[index];

                        OK (AudioObjectSetPropertyData (deviceID, &pa, 0, nullptr, sizeof (typeId), &typeId));
                    }
                }
        */
    }
    
    pub fn get_nominal_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyNominalSampleRate;
                pa.mScope = kAudioObjectPropertyScopeGlobal;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                Float64 sr = 0;
                UInt32 size = (UInt32) sizeof (sr);
                return OK (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, &sr)) ? (double) sr : 0.0;
        */
    }
    
    pub fn set_nominal_sample_rate(&self, new_sample_rate: f64) -> bool {
        
        todo!();
        /*
            if (std::abs (getNominalSampleRate() - newSampleRate) < 1.0)
                    return true;

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyNominalSampleRate;
                pa.mScope = kAudioObjectPropertyScopeGlobal;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                Float64 sr = newSampleRate;
                return OK (AudioObjectSetPropertyData (deviceID, &pa, 0, nullptr, sizeof (sr), &sr));
        */
    }
    
    pub fn reopen(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        new_sample_rate:     f64,
        buffer_size_samples: i32) -> String {
        
        todo!();
        /*
            String error;
                callbacksAllowed = false;
                stopTimer();

                stop (false);

                updateDetailsFromDevice();

                activeInputChans = inputChannels;
                activeInputChans.setRange (inChanNames.size(),
                                           activeInputChans.getHighestBit() + 1 - inChanNames.size(),
                                           false);

                activeOutputChans = outputChannels;
                activeOutputChans.setRange (outChanNames.size(),
                                            activeOutputChans.getHighestBit() + 1 - outChanNames.size(),
                                            false);

                numInputChans = activeInputChans.countNumberOfSetBits();
                numOutputChans = activeOutputChans.countNumberOfSetBits();

                if (! setNominalSampleRate (newSampleRate))
                {
                    updateDetailsFromDevice();
                    error = "Couldn't change sample rate";
                }
                else
                {
                    // change buffer size
                    AudioObjectPropertyAddress pa;
                    pa.mSelector = kAudioDevicePropertyBufferFrameSize;
                    pa.mScope = kAudioObjectPropertyScopeGlobal;
                    pa.mElement = aloeAudioObjectPropertyElementMain;
                    UInt32 framesPerBuf = (UInt32) bufferSizeSamples;

                    if (! OK (AudioObjectSetPropertyData (deviceID, &pa, 0, nullptr, sizeof (framesPerBuf), &framesPerBuf)))
                    {
                        updateDetailsFromDevice();
                        error = "Couldn't change buffer size";
                    }
                    else
                    {
                        // Annoyingly, after changing the rate and buffer size, some devices fail to
                        // correctly report their new settings until some random time in the future, so
                        // after calling updateDetailsFromDevice, we need to manually bodge these values
                        // to make sure we're using the correct numbers..
                        updateDetailsFromDevice();
                        sampleRate = newSampleRate;
                        bufferSize = bufferSizeSamples;

                        if (sampleRates.size() == 0)
                            error = "Device has no available sample-rates";
                        else if (bufferSizes.size() == 0)
                            error = "Device has no available buffer-sizes";
                    }
                }

                callbacksAllowed = true;
                return error;
        */
    }
    
    pub fn start(&mut self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

                if (! started)
                {
                    callback = nullptr;

                    if (deviceID != 0)
                    {
                        if (OK (AudioDeviceCreateIOProcID (deviceID, audioIOProc, this, &audioProcID)))
                        {
                            if (OK (AudioDeviceStart (deviceID, audioProcID)))
                            {
                                started = true;
                            }
                            else
                            {
                                OK (AudioDeviceDestroyIOProcID (deviceID, audioProcID));
                                audioProcID = {};
                            }
                        }
                    }
                }

                return started;
        */
    }
    
    pub fn set_callback(&mut self, cb: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);
                callback = cb;
        */
    }
    
    pub fn stop(&mut self, leave_interrupt_running: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

                callback = nullptr;

                if (started && (deviceID != 0) && ! leaveInterruptRunning)
                {
                    audioDeviceStopPending = true;

                    // wait until AudioDeviceStop() has been called on the IO thread
                    for (int i = 40; --i >= 0;)
                    {
                        if (audioDeviceStopPending == false)
                            break;

                        const ScopedUnlock ul (callbackLock);
                        Thread::sleep (50);
                    }

                    OK (AudioDeviceDestroyIOProcID (deviceID, audioProcID));
                    audioProcID = {};
                    started = false;
                }
        */
    }
    
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return sampleRate;
        */
    }
    
    pub fn get_buffer_size(&self) -> i32 {
        
        todo!();
        /*
            return bufferSize;
        */
    }
    
    pub fn audio_callback(&mut self, 
        in_input_data:   *const AudioBufferList,
        out_output_data: *mut AudioBufferList)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

                if (audioDeviceStopPending)
                {
                    if (OK (AudioDeviceStop (deviceID, audioProcID)))
                        audioDeviceStopPending = false;

                    return;
                }

                if (callback != nullptr)
                {
                    for (int i = numInputChans; --i >= 0;)
                    {
                        auto& info = inputChannelInfo.getReference(i);
                        auto dest = tempInputBuffers[i];
                        auto src = ((const float*) inInputData->mBuffers[info.streamNum].mData) + info.dataOffsetSamples;
                        auto stride = info.dataStrideSamples;

                        if (stride != 0) // if this is zero, info is invalid
                        {
                            for (int j = bufferSize; --j >= 0;)
                            {
                                *dest++ = *src;
                                src += stride;
                            }
                        }
                    }

                    callback->audioDeviceIOCallback (const_cast<const float**> (tempInputBuffers.get()),
                                                     numInputChans,
                                                     tempOutputBuffers,
                                                     numOutputChans,
                                                     bufferSize);

                    for (int i = numOutputChans; --i >= 0;)
                    {
                        auto& info = outputChannelInfo.getReference (i);
                        auto src = tempOutputBuffers[i];
                        auto dest = ((float*) outOutputData->mBuffers[info.streamNum].mData) + info.dataOffsetSamples;
                        auto stride = info.dataStrideSamples;

                        if (stride != 0) // if this is zero, info is invalid
                        {
                            for (int j = bufferSize; --j >= 0;)
                            {
                                *dest = *src++;
                                dest += stride;
                            }
                        }
                    }
                }
                else
                {
                    for (UInt32 i = 0; i < outOutputData->mNumberBuffers; ++i)
                        zeromem (outOutputData->mBuffers[i].mData,
                                 outOutputData->mBuffers[i].mDataByteSize);
                }
        */
    }

    /**
      called by callbacks
      */
    pub fn device_details_changed(&mut self)  {
        
        todo!();
        /*
            if (callbacksAllowed.get() == 1)
                    startTimer (100);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            ALOE_COREAUDIOLOG ("Device changed");

                stopTimer();
                auto oldSampleRate = sampleRate;
                auto oldBufferSize = bufferSize;

                if (! updateDetailsFromDevice())
                    owner.stopInternal();
                else if ((oldBufferSize != bufferSize || oldSampleRate != sampleRate) && owner.shouldRestartDevice())
                    owner.restart();
        */
    }
    
    pub fn audio_io_proc(
        in_device:       AudioDeviceID,
        in_now:          *const AudioTimeStamp,
        in_input_data:   *const AudioBufferList,
        in_input_time:   *const AudioTimeStamp,
        out_output_data: *mut AudioBufferList,
        in_output_time:  *const AudioTimeStamp,
        device:          *mut c_void) -> OSStatus {
        
        todo!();
        /*
            static_cast<CoreAudioInternal*> (device)->audioCallback (inInputData, outOutputData);
                return noErr;
        */
    }
    
    pub fn device_listener_proc(
        in_device:      AudioDeviceID,
        in_line:        u32,
        pa:             *const AudioObjectPropertyAddress,
        in_client_data: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            auto intern = static_cast<CoreAudioInternal*> (inClientData);

                switch (pa->mSelector)
                {
                    case kAudioDeviceProcessorOverload:
                        intern->xruns++;
                        break;

                    case kAudioDevicePropertyBufferSize:
                    case kAudioDevicePropertyBufferFrameSize:
                    case kAudioDevicePropertyNominalSampleRate:
                    case kAudioDevicePropertyStreamFormat:
                    case kAudioDevicePropertyDeviceIsAlive:
                    case kAudioStreamPropertyPhysicalFormat:
                        intern->deviceDetailsChanged();
                        break;

                    case kAudioDevicePropertyDeviceHasChanged:
                    case kAudioObjectPropertyOwnedObjects:
                        intern->owner.restart();

                        if (intern->owner.deviceType != nullptr)
                            intern->owner.deviceType->triggerAsyncAudioDeviceListChange();
                        break;

                    case kAudioDevicePropertyBufferSizeRange:
                    case kAudioDevicePropertyVolumeScalar:
                    case kAudioDevicePropertyMute:
                    case kAudioDevicePropertyPlayThru:
                    case kAudioDevicePropertyDataSource:
                    case kAudioDevicePropertyDeviceIsRunning:
                        break;
                }

                return noErr;
        */
    }
    
    pub fn get_all_data_sources_for_device(
        deviceid: AudioDeviceID,
        types:    &mut HeapBlock<OSType>) -> i32 {
        
        todo!();
        /*
            AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyDataSources;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;
                UInt32 size = 0;

                if (deviceID != 0
                     && AudioObjectGetPropertyDataSize (deviceID, &pa, 0, nullptr, &size) == noErr)
                {
                    types.calloc (size, 1);

                    if (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, types) == noErr)
                        return size / (int) sizeof (OSType);
                }

                return 0;
        */
    }
    
    pub fn ok(&self, error_code: OSStatus) -> bool {
        
        todo!();
        /*
            if (errorCode == noErr)
                    return true;

                const String errorMessage ("CoreAudio error: " + String::toHexString ((int) errorCode));
                ALOE_COREAUDIOLOG (errorMessage);

                if (callback != nullptr)
                    callback->audioDeviceError (errorMessage);

                return false;
        */
    }
}
