crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AudioIODeviceCombiner<'a> {
    base:                      AudioIODevice,
    base2:                     Thread,
    base3:                     Timer,
    owner:                     WeakReference<CoreAudioIODeviceType<'a>>,
    callback_lock:             CriticalSection,
    callback:                  *mut dyn AudioIODeviceCallback, // default = nullptr
    previous_callback:         *mut dyn AudioIODeviceCallback, // default = nullptr
    current_sample_rate:       f64, // default = 0
    current_buffer_size:       i32, // default = 0
    active:                    bool, // default = false
    last_error:                String,
    fifos:                     AudioBuffer<f32>,
    fifo_read_pointers:        *const *const f32, // default = nullptr
    fifo_write_pointers:       *mut *mut f32, // default = nullptr
    thread_initialised:        WaitableEvent,
    close_lock:                CriticalSection,
    input_channels_requested:  BigInteger,
    output_channels_requested: BigInteger,
    sample_rate_requested:     f64, // default = 44100
    buffer_size_requested:     i32, // default = 512
    devices:                   Vec<Box<AudioIODeviceCombinerDeviceWrapper<'a>>>,
}

impl<'a> Drop for AudioIODeviceCombiner<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                close();
                devices.clear();
             */
    }
}

impl<'a> AudioIODeviceCombiner<'a> {

    pub fn new(
        device_name: &String,
        device_type: *mut CoreAudioIODeviceType) -> Self {
    
        todo!();
        /*


            : AudioIODevice (deviceName, "CoreAudio"),
                  Thread (deviceName),
                  owner (deviceType)
        */
    }
    
    pub fn add_device(&mut self, 
        device:      *mut CoreAudioIODevice,
        use_inputs:  bool,
        use_outputs: bool)  {
        
        todo!();
        /*
            jassert (device != nullptr);
                jassert (! isOpen());
                jassert (! device->isOpen());
                devices.add (new AudioIODeviceCombinerDeviceWrapper (*this, device, useInputs, useOutputs));

                if (currentSampleRate == 0)
                    currentSampleRate = device->getCurrentSampleRate();

                if (currentBufferSize == 0)
                    currentBufferSize = device->getCurrentBufferSizeSamples();
        */
    }
    
    pub fn get_devices(&self) -> Vec<*mut AudioIODevice> {
        
        todo!();
        /*
            Vec<AudioIODevice*> devs;

                for (auto* d : devices)
                    devs.add (d->device.get());

                return devs;
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            StringArray names;

                for (auto* d : devices)
                    names.addArray (d->getOutputChannelNames());

                names.appendNumbersToDuplicates (false, true);
                return names;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> StringArray {
        
        todo!();
        /*
            StringArray names;

                for (auto* d : devices)
                    names.addArray (d->getInputChannelNames());

                names.appendNumbersToDuplicates (false, true);
                return names;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> commonRates;
                bool first = true;

                for (auto* d : devices)
                {
                    auto rates = d->device->getAvailableSampleRates();

                    if (first)
                    {
                        first = false;
                        commonRates = rates;
                    }
                    else
                    {
                        commonRates.removeValuesNotIn (rates);
                    }
                }

                return commonRates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> commonSizes;
                bool first = true;

                for (auto* d : devices)
                {
                    auto sizes = d->device->getAvailableBufferSizes();

                    if (first)
                    {
                        first = false;
                        commonSizes = sizes;
                    }
                    else
                    {
                        commonSizes.removeValuesNotIn (sizes);
                    }
                }

                return commonSizes;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return active;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return callback != nullptr;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return currentSampleRate;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return currentBufferSize;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            int depth = 32;

                for (auto* d : devices)
                    depth = jmin (depth, d->device->getCurrentBitDepth());

                return depth;
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            int size = 0;

                for (auto* d : devices)
                    size = jmax (size, d->device->getDefaultBufferSize());

                return size;
        */
    }
    
    pub fn open(&mut self, 
        input_channels:  &BigInteger,
        output_channels: &BigInteger,
        sample_rate:     f64,
        buffer_size:     i32) -> String {
        
        todo!();
        /*
            inputChannelsRequested = inputChannels;
                outputChannelsRequested = outputChannels;
                sampleRateRequested = sampleRate;
                bufferSizeRequested = bufferSize;

                close();
                active = true;

                if (bufferSize <= 0)
                    bufferSize = getDefaultBufferSize();

                if (sampleRate <= 0)
                {
                    auto rates = getAvailableSampleRates();

                    for (int i = 0; i < rates.size() && sampleRate < 44100.0; ++i)
                        sampleRate = rates.getUnchecked(i);
                }

                currentSampleRate = sampleRate;
                currentBufferSize = bufferSize;

                const int fifoSize = bufferSize * 3 + 1;
                int totalInputChanIndex = 0, totalOutputChanIndex = 0;
                int chanIndex = 0;

                for (auto* d : devices)
                {
                    BigInteger ins (inputChannels >> totalInputChanIndex);
                    BigInteger outs (outputChannels >> totalOutputChanIndex);

                    int numIns  = d->getInputChannelNames().size();
                    int numOuts = d->getOutputChannelNames().size();

                    totalInputChanIndex += numIns;
                    totalOutputChanIndex += numOuts;

                    String err = d->open (ins, outs, sampleRate, bufferSize,
                                          chanIndex, fifoSize);

                    if (err.isNotEmpty())
                    {
                        close();
                        lastError = err;
                        return err;
                    }

                    chanIndex += d->numInputChans + d->numOutputChans;
                }

                fifos.setSize (chanIndex, fifoSize);
                fifoReadPointers  = fifos.getArrayOfReadPointers();
                fifoWritePointers = fifos.getArrayOfWritePointers();
                fifos.clear();
                startThread (9);
                threadInitialised.wait();

                return {};
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();
                stopThread (10000);
                fifos.clear();
                active = false;

                for (auto* d : devices)
                    d->close();
        */
    }
    
    pub fn restart(&mut self, cb: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            const ScopedLock sl (closeLock);

                close();

                auto newSampleRate = sampleRateRequested;
                auto newBufferSize = bufferSizeRequested;

                for (auto* d : devices)
                {
                    auto deviceSampleRate = d->getCurrentSampleRate();

                    if (deviceSampleRate != sampleRateRequested)
                    {
                        if (! getAvailableSampleRates().contains (deviceSampleRate))
                            return;

                        for (auto* d2 : devices)
                            if (d2 != d)
                                d2->setCurrentSampleRate (deviceSampleRate);

                        newSampleRate = deviceSampleRate;
                        break;
                    }
                }

                for (auto* d : devices)
                {
                    auto deviceBufferSize = d->getCurrentBufferSizeSamples();

                    if (deviceBufferSize != bufferSizeRequested)
                    {
                        if (! getAvailableBufferSizes().contains (deviceBufferSize))
                            return;

                        newBufferSize = deviceBufferSize;
                        break;
                    }
                }

                open (inputChannelsRequested, outputChannelsRequested,
                      newSampleRate, newBufferSize);

                start (cb);
        */
    }
    
    pub fn restart_async(&mut self)  {
        
        todo!();
        /*
            {
                    const ScopedLock sl (closeLock);

                    if (active)
                    {
                        if (callback != nullptr)
                            previousCallback = callback;

                        close();
                    }
                }

                startTimer (100);
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            BigInteger chans;
                int start = 0;

                for (auto* d : devices)
                {
                    auto numChans = d->getOutputChannelNames().size();

                    if (numChans > 0)
                    {
                        chans |= (d->device->getActiveOutputChannels() << start);
                        start += numChans;
                    }
                }

                return chans;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            BigInteger chans;
                int start = 0;

                for (auto* d : devices)
                {
                    auto numChans = d->getInputChannelNames().size();

                    if (numChans > 0)
                    {
                        chans |= (d->device->getActiveInputChannels() << start);
                        start += numChans;
                    }
                }

                return chans;
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            int lat = 0;

                for (auto* d : devices)
                    lat = jmax (lat, d->device->getOutputLatencyInSamples());

                return lat + currentBufferSize * 2;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            int lat = 0;

                for (auto* d : devices)
                    lat = jmax (lat, d->device->getInputLatencyInSamples());

                return lat + currentBufferSize * 2;
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (callback != newCallback)
                {
                    stop();
                    fifos.clear();

                    for (auto* d : devices)
                        d->start();

                    if (newCallback != nullptr)
                        newCallback->audioDeviceAboutToStart (this);

                    const ScopedLock sl (callbackLock);
                    callback = newCallback;
                    previousCallback = callback;
                }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            shutdown ({});
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            auto numSamples = currentBufferSize;

                AudioBuffer<float> buffer (fifos.getNumChannels(), numSamples);
                buffer.clear();

                Vec<const float*> inputChans;
                Vec<float*> outputChans;

                for (auto* d : devices)
                {
                    for (int j = 0; j < d->numInputChans; ++j)   inputChans.add  (buffer.getReadPointer  (d->inputIndex  + j));
                    for (int j = 0; j < d->numOutputChans; ++j)  outputChans.add (buffer.getWritePointer (d->outputIndex + j));
                }

                auto numInputChans  = inputChans.size();
                auto numOutputChans = outputChans.size();

                inputChans.add (nullptr);
                outputChans.add (nullptr);

                auto blockSizeMs = jmax (1, (int) (1000 * numSamples / currentSampleRate));

                jassert (numInputChans + numOutputChans == buffer.getNumChannels());

                threadInitialised.signal();

                while (! threadShouldExit())
                {
                    readInput (buffer, numSamples, blockSizeMs);

                    bool didCallback = true;

                    {
                        const ScopedLock sl (callbackLock);

                        if (callback != nullptr)
                            callback->audioDeviceIOCallback ((const float**) inputChans.getRawDataPointer(), numInputChans,
                                                             outputChans.getRawDataPointer(), numOutputChans, numSamples);
                        else
                            didCallback = false;
                    }

                    if (didCallback)
                    {
                        pushOutputData (buffer, numSamples, blockSizeMs);
                    }
                    else
                    {
                        for (int i = 0; i < numOutputChans; ++i)
                            FloatVectorOperations::clear (outputChans[i], numSamples);

                        reset();
                    }
                }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

                restart (previousCallback);
        */
    }
    
    pub fn shutdown(&mut self, error: &String)  {
        
        todo!();
        /*
            AudioIODeviceCallback* lastCallback = nullptr;

                {
                    const ScopedLock sl (callbackLock);
                    std::swap (callback, lastCallback);
                }

                for (auto* d : devices)
                    d->device->stopInternal();

                if (lastCallback != nullptr)
                {
                    if (error.isNotEmpty())
                        lastCallback->audioDeviceError (error);
                    else
                        lastCallback->audioDeviceStopped();
                }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto* d : devices)
                    d->reset();
        */
    }
    
    pub fn underrun(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn read_input(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        num_samples:   i32,
        block_size_ms: i32)  {
        
        todo!();
        /*
            for (auto* d : devices)
                    d->done = (d->numInputChans == 0 || d->isWaitingForInput);

                float totalWaitTimeMs = blockSizeMs * 5.0f;
                constexpr int numReadAttempts = 6;
                auto sumPower2s = [] (int maxPower) { return (1 << (maxPower + 1)) - 1; };
                float waitTime = totalWaitTimeMs / (float) sumPower2s (numReadAttempts - 2);

                for (int numReadAttemptsRemaining = numReadAttempts;;)
                {
                    bool anySamplesRemaining = false;

                    for (auto* d : devices)
                    {
                        if (! d->done)
                        {
                            if (d->isInputReady (numSamples))
                            {
                                d->readInput (buffer, numSamples);
                                d->done = true;
                            }
                            else
                            {
                                anySamplesRemaining = true;
                            }
                        }
                    }

                    if (! anySamplesRemaining)
                        return;

                    if (--numReadAttemptsRemaining == 0)
                        break;

                    wait (jmax (1, roundToInt (waitTime)));
                    waitTime *= 2.0f;
                }

                for (auto* d : devices)
                    if (! d->done)
                        for (int i = 0; i < d->numInputChans; ++i)
                            buffer.clear (d->inputIndex + i, 0, numSamples);
        */
    }
    
    pub fn push_output_data(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        num_samples:   i32,
        block_size_ms: i32)  {
        
        todo!();
        /*
            for (auto* d : devices)
                    d->done = (d->numOutputChans == 0);

                for (int tries = 5;;)
                {
                    bool anyRemaining = false;

                    for (auto* d : devices)
                    {
                        if (! d->done)
                        {
                            if (d->isOutputReady (numSamples))
                            {
                                d->pushOutputData (buffer, numSamples);
                                d->done = true;
                            }
                            else
                            {
                                anyRemaining = true;
                            }
                        }
                    }

                    if ((! anyRemaining) || --tries == 0)
                        return;

                    wait (blockSizeMs);
                }
        */
    }
    
    pub fn handle_audio_device_about_to_start(&mut self, device: *mut AudioIODevice)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

                auto newSampleRate = device->getCurrentSampleRate();
                auto commonRates = getAvailableSampleRates();

                if (! commonRates.contains (newSampleRate))
                {
                    commonRates.sort();

                    if (newSampleRate < commonRates.getFirst() || newSampleRate > commonRates.getLast())
                    {
                        newSampleRate = jlimit (commonRates.getFirst(), commonRates.getLast(), newSampleRate);
                    }
                    else
                    {
                        for (auto it = commonRates.begin(); it < commonRates.end() - 1; ++it)
                        {
                            if (it[0] < newSampleRate && it[1] > newSampleRate)
                            {
                                newSampleRate = newSampleRate - it[0] < it[1] - newSampleRate ? it[0] : it[1];
                                break;
                            }
                        }
                    }
                }

                currentSampleRate = newSampleRate;
                bool anySampleRateChanges = false;

                for (auto* d : devices)
                {
                    if (d->getCurrentSampleRate() != currentSampleRate)
                    {
                        d->setCurrentSampleRate (currentSampleRate);
                        anySampleRateChanges = true;
                    }
                }

                if (anySampleRateChanges && owner != nullptr)
                    owner->audioDeviceListChanged();

                if (callback != nullptr)
                    callback->audioDeviceAboutToStart (device);
        */
    }
    
    pub fn handle_audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            shutdown ({});
        */
    }
    
    pub fn handle_audio_device_error(&mut self, error_message: &String)  {
        
        todo!();
        /*
            shutdown (errorMessage.isNotEmpty() ? errorMessage : String ("unknown"));
        */
    }
}
