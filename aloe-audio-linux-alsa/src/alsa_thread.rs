crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ALSAThread {
    base:                             Thread,
    error:                            String,
    sample_rate:                      f64, // default = 0
    buffer_size:                      i32, // default = 0
    output_latency:                   i32, // default = 0
    input_latency:                    i32, // default = 0
    current_input_chans:              BigInteger,
    current_output_chans:             BigInteger,
    sample_rates:                     Vec<f64>,
    channel_names_out:                Vec<String>,
    channel_names_in:                 Vec<String>,
    callback:                         *mut dyn AudioIODeviceCallback, // default = nullptr
    input_id:                         String,
    output_id:                        String,
    output_device:                    Box<ALSADevice>,
    input_device:                     Box<ALSADevice>,
    num_callbacks:                    Atomic<i32>, // default = 0 
    audio_io_in_progress:             bool, // default = false
    callback_lock:                    CriticalSection,
    input_channel_buffer:             AudioBuffer<f32>,
    output_channel_buffer:            AudioBuffer<f32>,
    input_channel_data_for_callback:  Vec<*const f32>,
    output_channel_data_for_callback: Vec<*mut f32>,
    min_chans_out:                    u32, // default = 0
    max_chans_out:                    u32, // default = 0
    min_chans_in:                     u32, // default = 0
    max_chans_in:                     u32, // default = 0
}

impl Drop for ALSAThread {
    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

impl ALSAThread {

    pub fn new(
        input_deviceid:  &String,
        output_deviceid: &String) -> Self {
    
        todo!();
        /*
        : thread("Aloe ALSA"),
        : input_id(inputDeviceID),
        : output_id(outputDeviceID),

            initialiseRatesAndChannels();
        */
    }
    
    pub fn open(&mut self, 
        input_channels:  BigInteger,
        output_channels: BigInteger,
        new_sample_rate: f64,
        new_buffer_size: i32)  {
        
        todo!();
        /*
            close();

            error.clear();
            sampleRate = newSampleRate;
            bufferSize = newBufferSize;

            int maxInputsRequested = inputChannels.getHighestBit() + 1;
            maxInputsRequested = jmax ((int) minChansIn, jmin ((int) maxChansIn, maxInputsRequested));

            inputChannelBuffer.setSize (maxInputsRequested, bufferSize);
            inputChannelBuffer.clear();
            inputChannelDataForCallback.clear();
            currentInputChans.clear();

            if (inputChannels.getHighestBit() >= 0)
            {
                for (int i = 0; i < maxInputsRequested; ++i)
                {
                    if (inputChannels[i])
                    {
                        inputChannelDataForCallback.add (inputChannelBuffer.getReadPointer (i));
                        currentInputChans.setBit (i);
                    }
                }
            }

            ensureMinimumNumBitsSet (outputChannels, (int) minChansOut);

            int maxOutputsRequested = outputChannels.getHighestBit() + 1;
            maxOutputsRequested = jmax ((int) minChansOut, jmin ((int) maxChansOut, maxOutputsRequested));

            outputChannelBuffer.setSize (maxOutputsRequested, bufferSize);
            outputChannelBuffer.clear();
            outputChannelDataForCallback.clear();
            currentOutputChans.clear();

            // Note that the input device is opened before an output, because we've heard
            // of drivers where doing it in the reverse order mysteriously fails.. If this
            // order also causes problems, let us know and we'll see if we can find a compromise!

            if (inputChannelDataForCallback.size() > 0 && inputId.isNotEmpty())
            {
                inputDevice.reset (new ALSADevice (inputId, true));

                if (inputDevice->error.isNotEmpty())
                {
                    error = inputDevice->error;
                    inputDevice.reset();
                    return;
                }

                ensureMinimumNumBitsSet (currentInputChans, (int) minChansIn);

                if (! inputDevice->setParameters ((unsigned int) sampleRate,
                                                  jlimit ((int) minChansIn, (int) maxChansIn, currentInputChans.getHighestBit() + 1),
                                                  bufferSize))
                {
                    error = inputDevice->error;
                    inputDevice.reset();
                    return;
                }

                inputLatency = inputDevice->latency;
            }

            if (outputChannels.getHighestBit() >= 0)
            {
                for (int i = 0; i < maxOutputsRequested; ++i)
                {
                    if (outputChannels[i])
                    {
                        outputChannelDataForCallback.add (outputChannelBuffer.getWritePointer (i));
                        currentOutputChans.setBit (i);
                    }
                }
            }

            if (outputChannelDataForCallback.size() > 0 && outputId.isNotEmpty())
            {
                outputDevice.reset (new ALSADevice (outputId, false));

                if (outputDevice->error.isNotEmpty())
                {
                    error = outputDevice->error;
                    outputDevice.reset();
                    return;
                }

                if (! outputDevice->setParameters ((unsigned int) sampleRate,
                                                   jlimit ((int) minChansOut, (int) maxChansOut,
                                                           currentOutputChans.getHighestBit() + 1),
                                                   bufferSize))
                {
                    error = outputDevice->error;
                    outputDevice.reset();
                    return;
                }

                outputLatency = outputDevice->latency;
            }

            if (outputDevice == nullptr && inputDevice == nullptr)
            {
                error = "no channels";
                return;
            }

            if (outputDevice != nullptr && inputDevice != nullptr)
                snd_pcm_link (outputDevice->handle, inputDevice->handle);

            if (inputDevice != nullptr && ALOE_ALSA_FAILED (snd_pcm_prepare (inputDevice->handle)))
                return;

            if (outputDevice != nullptr && ALOE_ALSA_FAILED (snd_pcm_prepare (outputDevice->handle)))
                return;

            startThread (9);

            int count = 1000;

            while (numCallbacks == 0)
            {
                sleep (5);

                if (--count < 0 || ! isThreadRunning())
                {
                    error = "device didn't start";
                    break;
                }
            }
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (isThreadRunning())
            {
                // problem: when pulseaudio is suspended (with pasuspend) , the ALSAThread::run is just stuck in
                // snd_pcm_writei -- no error, no nothing it just stays stuck. So the only way I found to exit "nicely"
                // (that is without the "killing thread by force" of stopThread) , is to just call snd_pcm_close from
                // here which will cause the thread to resume, and exit
                signalThreadShouldExit();

                const int callbacksToStop = numCallbacks;

                if ((! waitForThreadToExit (400)) && audioIoInProgress && numCallbacks == callbacksToStop)
                {
                    ALOE_ALSA_LOG ("Thread is stuck in i/o.. Is pulseaudio suspended?");

                    if (outputDevice != nullptr) outputDevice->closeNow();
                    if (inputDevice != nullptr) inputDevice->closeNow();
                }
            }

            stopThread (6000);

            inputDevice.reset();
            outputDevice.reset();

            inputChannelBuffer.setSize (1, 1);
            outputChannelBuffer.setSize (1, 1);

            numCallbacks = 0;
        */
    }
    
    pub fn set_callback(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);
            callback = newCallback;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
            {
                if (inputDevice != nullptr && inputDevice->handle != nullptr)
                {
                    if (outputDevice == nullptr || outputDevice->handle == nullptr)
                    {
                        ALOE_ALSA_FAILED (snd_pcm_wait (inputDevice->handle, 2000));

                        if (threadShouldExit())
                            break;

                        auto avail = snd_pcm_avail_update (inputDevice->handle);

                        if (avail < 0)
                            ALOE_ALSA_FAILED (snd_pcm_recover (inputDevice->handle, (int) avail, 0));
                    }

                    audioIoInProgress = true;

                    if (! inputDevice->readFromInputDevice (inputChannelBuffer, bufferSize))
                    {
                        ALOE_ALSA_LOG ("Read failure");
                        break;
                    }

                    audioIoInProgress = false;
                }

                if (threadShouldExit())
                    break;

                {
                    const ScopedLock sl (callbackLock);
                    ++numCallbacks;

                    if (callback != nullptr)
                    {
                        callback->audioDeviceIOCallback (inputChannelDataForCallback.getRawDataPointer(),
                                                         inputChannelDataForCallback.size(),
                                                         outputChannelDataForCallback.getRawDataPointer(),
                                                         outputChannelDataForCallback.size(),
                                                         bufferSize);
                    }
                    else
                    {
                        for (int i = 0; i < outputChannelDataForCallback.size(); ++i)
                            zeromem (outputChannelDataForCallback[i], (size_t) bufferSize * sizeof (float));
                    }
                }

                if (outputDevice != nullptr && outputDevice->handle != nullptr)
                {
                    ALOE_ALSA_FAILED (snd_pcm_wait (outputDevice->handle, 2000));

                    if (threadShouldExit())
                        break;

                    auto avail = snd_pcm_avail_update (outputDevice->handle);

                    if (avail < 0)
                        ALOE_ALSA_FAILED (snd_pcm_recover (outputDevice->handle, (int) avail, 0));

                    audioIoInProgress = true;

                    if (! outputDevice->writeToOutputDevice (outputChannelBuffer, bufferSize))
                    {
                        ALOE_ALSA_LOG ("write failure");
                        break;
                    }

                    audioIoInProgress = false;
                }
            }

            audioIoInProgress = false;
        */
    }
    
    pub fn get_bit_depth(&self) -> i32 {
        
        todo!();
        /*
            if (outputDevice != nullptr)
                return outputDevice->bitDepth;

            if (inputDevice != nullptr)
                return inputDevice->bitDepth;

            return 16;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            int result = 0;

            if (outputDevice != nullptr)
                result += outputDevice->underrunCount;

            if (inputDevice != nullptr)
                result += inputDevice->overrunCount;

            return result;
        */
    }
    
    pub fn failed(&mut self, error_num: i32) -> bool {
        
        todo!();
        /*
            if (errorNum >= 0)
                return false;

            error = snd_strerror (errorNum);
            ALOE_ALSA_LOG ("ALSA error: " << error);
            return true;
        */
    }
    
    pub fn initialise_rates_and_channels(&mut self)  {
        
        todo!();
        /*
            sampleRates.clear();
            channelNamesOut.clear();
            channelNamesIn.clear();
            minChansOut = 0;
            maxChansOut = 0;
            minChansIn = 0;
            maxChansIn = 0;
            unsigned int dummy = 0;

            getDeviceProperties (inputId, dummy, dummy, minChansIn, maxChansIn, sampleRates, false, true);
            getDeviceProperties (outputId, minChansOut, maxChansOut, dummy, dummy, sampleRates, true, false);

            for (unsigned int i = 0; i < maxChansOut; ++i)
                channelNamesOut.add ("channel " + String ((int) i + 1));

            for (unsigned int i = 0; i < maxChansIn; ++i)
                channelNamesIn.add ("channel " + String ((int) i + 1));
        */
    }
}

