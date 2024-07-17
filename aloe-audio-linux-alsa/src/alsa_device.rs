crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ALSADevice {
    handle:               *mut SndPcm,
    error:                String,
    bit_depth:            i32,
    num_channels_running: i32,
    latency:              i32,
    underrun_count:       i32, // default = 0
    overrun_count:        i32, // default = 0
    deviceid:             String,
    is_input:             bool,
    is_interleaved:       bool,
    scratch:              MemoryBlock,
    converter:            Box<AudioDataConverter>,
}

impl Drop for ALSADevice {

    fn drop(&mut self) {
        todo!();
        /* 
            closeNow();
         */
    }
}

impl ALSADevice {

    pub fn new(
        devid:     &String,
        for_input: bool) -> Self {
    
        todo!();
        /*
        : handle(nullptr),
        : bit_depth(16),
        : num_channels_running(0),
        : latency(0),
        : deviceid(devID),
        : is_input(forInput),
        : is_interleaved(true),

            ALOE_ALSA_LOG ("snd_pcm_open (" << deviceID.toUTF8().getAddress() << ", forInput=" << (int) forInput << ")");

            int err = snd_pcm_open (&handle, deviceID.toUTF8(),
                                    forInput ? SND_PCM_STREAM_CAPTURE : SND_PCM_STREAM_PLAYBACK,
                                    SND_PCM_ASYNC);
            if (err < 0)
            {
                if (-err == EBUSY)
                    error << "The device \"" << deviceID << "\" is busy (another application is using it).";
                else if (-err == ENOENT)
                    error << "The device \"" << deviceID << "\" is not available.";
                else
                    error << "Could not open " << (forInput ? "input" : "output") << " device \"" << deviceID
                          << "\": " << snd_strerror(err) << " (" << err << ")";

                ALOE_ALSA_LOG ("snd_pcm_open failed; " << error);
            }
        */
    }
    
    pub fn close_now(&mut self)  {
        
        todo!();
        /*
            if (handle != nullptr)
            {
                snd_pcm_close (handle);
                handle = nullptr;
            }
        */
    }
    
    pub fn set_parameters(&mut self, 
        sample_rate:  u32,
        num_channels: i32,
        buffer_size:  i32) -> bool {
        
        todo!();
        /*
            if (handle == nullptr)
                return false;

            ALOE_ALSA_LOG ("ALSADevice::setParameters(" << deviceID << ", "
                             << (int) sampleRate << ", " << numChannels << ", " << bufferSize << ")");

            snd_pcm_hw_params_t* hwParams;
            snd_pcm_hw_params_alloca (&hwParams);

            if (snd_pcm_hw_params_any (handle, hwParams) < 0)
            {
                // this is the error message that aplay returns when an error happens here,
                // it is a bit more explicit that "Invalid parameter"
                error = "Broken configuration for this PCM: no configurations available";
                return false;
            }

            if (snd_pcm_hw_params_set_access (handle, hwParams, SND_PCM_ACCESS_RW_INTERLEAVED) >= 0) // works better for plughw..
                isInterleaved = true;
            else if (snd_pcm_hw_params_set_access (handle, hwParams, SND_PCM_ACCESS_RW_NONINTERLEAVED) >= 0)
                isInterleaved = false;
            else
            {
                jassertfalse;
                return false;
            }

            enum { isFloatBit = 1 << 16, isLittleEndianBit = 1 << 17, onlyUseLower24Bits = 1 << 18 };

            const int formatsToTry[] = { SND_PCM_FORMAT_FLOAT_LE,   32 | isFloatBit | isLittleEndianBit,
                                         SND_PCM_FORMAT_FLOAT_BE,   32 | isFloatBit,
                                         SND_PCM_FORMAT_S32_LE,     32 | isLittleEndianBit,
                                         SND_PCM_FORMAT_S32_BE,     32,
                                         SND_PCM_FORMAT_S24_3LE,    24 | isLittleEndianBit,
                                         SND_PCM_FORMAT_S24_3BE,    24,
                                         SND_PCM_FORMAT_S24_LE,     32 | isLittleEndianBit | onlyUseLower24Bits,
                                         SND_PCM_FORMAT_S16_LE,     16 | isLittleEndianBit,
                                         SND_PCM_FORMAT_S16_BE,     16 };
            bitDepth = 0;

            for (int i = 0; i < numElementsInArray (formatsToTry); i += 2)
            {
                if (snd_pcm_hw_params_set_format (handle, hwParams, (_snd_pcm_format) formatsToTry [i]) >= 0)
                {
                    const int type = formatsToTry [i + 1];
                    bitDepth = type & 255;

                    converter.reset (createConverter (isInput, bitDepth,
                                                      (type & isFloatBit) != 0,
                                                      (type & isLittleEndianBit) != 0,
                                                      (type & onlyUseLower24Bits) != 0,
                                                      numChannels,
                                                      isInterleaved));
                    break;
                }
            }

            if (bitDepth == 0)
            {
                error = "device doesn't support a compatible PCM format";
                ALOE_ALSA_LOG ("Error: " + error);
                return false;
            }

            int dir = 0;
            unsigned int periods = 4;
            snd_pcm_uframes_t samplesPerPeriod = (snd_pcm_uframes_t) bufferSize;

            if (ALOE_ALSA_FAILED (snd_pcm_hw_params_set_rate_near (handle, hwParams, &sampleRate, nullptr))
                || ALOE_ALSA_FAILED (snd_pcm_hw_params_set_channels (handle, hwParams, (unsigned int ) numChannels))
                || ALOE_ALSA_FAILED (snd_pcm_hw_params_set_periods_near (handle, hwParams, &periods, &dir))
                || ALOE_ALSA_FAILED (snd_pcm_hw_params_set_period_size_near (handle, hwParams, &samplesPerPeriod, &dir))
                || ALOE_ALSA_FAILED (snd_pcm_hw_params (handle, hwParams)))
            {
                return false;
            }

            snd_pcm_uframes_t frames = 0;

            if (ALOE_ALSA_FAILED (snd_pcm_hw_params_get_period_size (hwParams, &frames, &dir))
                 || ALOE_ALSA_FAILED (snd_pcm_hw_params_get_periods (hwParams, &periods, &dir)))
                latency = 0;
            else
                latency = (int) frames * ((int) periods - 1); // (this is the method JACK uses to guess the latency..)

            ALOE_ALSA_LOG ("frames: " << (int) frames << ", periods: " << (int) periods
                              << ", samplesPerPeriod: " << (int) samplesPerPeriod);

            snd_pcm_sw_params_t* swParams;
            snd_pcm_sw_params_alloca (&swParams);
            snd_pcm_uframes_t boundary;

            if (ALOE_ALSA_FAILED (snd_pcm_sw_params_current (handle, swParams))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params_get_boundary (swParams, &boundary))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params_set_silence_threshold (handle, swParams, 0))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params_set_silence_size (handle, swParams, boundary))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params_set_start_threshold (handle, swParams, samplesPerPeriod))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params_set_stop_threshold (handle, swParams, boundary))
                || ALOE_ALSA_FAILED (snd_pcm_sw_params (handle, swParams)))
            {
                return false;
            }

           #if ALOE_ALSA_LOGGING
            // enable this to dump the config of the devices that get opened
            snd_output_t* out;
            snd_output_stdio_attach (&out, stderr, 0);
            snd_pcm_hw_params_dump (hwParams, out);
            snd_pcm_sw_params_dump (swParams, out);
           #endif

            numChannelsRunning = numChannels;

            return true;
        */
    }
    
    pub fn write_to_output_device(&mut self, 
        output_channel_buffer: &mut AudioBuffer<f32>,
        num_samples:           i32) -> bool {
        
        todo!();
        /*
            jassert (numChannelsRunning <= outputChannelBuffer.getNumChannels());
            float* const* const data = outputChannelBuffer.getArrayOfWritePointers();
            snd_pcm_sframes_t numDone = 0;

            if (isInterleaved)
            {
                scratch.ensureSize ((size_t) ((int) sizeof (float) * numSamples * numChannelsRunning), false);

                for (int i = 0; i < numChannelsRunning; ++i)
                    converter->convertSamples (scratch.getData(), i, data[i], 0, numSamples);

                numDone = snd_pcm_writei (handle, scratch.getData(), (snd_pcm_uframes_t) numSamples);
            }
            else
            {
                for (int i = 0; i < numChannelsRunning; ++i)
                    converter->convertSamples (data[i], data[i], numSamples);

                numDone = snd_pcm_writen (handle, (void**) data, (snd_pcm_uframes_t) numSamples);
            }

            if (numDone < 0)
            {
                if (numDone == -(EPIPE))
                    underrunCount++;

                if (ALOE_ALSA_FAILED (snd_pcm_recover (handle, (int) numDone, 1 /* silent */)))
                    return false;
            }

            if (numDone < numSamples)
                ALOE_ALSA_LOG ("Did not write all samples: numDone: " << numDone << ", numSamples: " << numSamples);

            return true;
        */
    }
    
    pub fn read_from_input_device(&mut self, 
        input_channel_buffer: &mut AudioBuffer<f32>,
        num_samples:          i32) -> bool {
        
        todo!();
        /*
            jassert (numChannelsRunning <= inputChannelBuffer.getNumChannels());
            float* const* const data = inputChannelBuffer.getArrayOfWritePointers();

            if (isInterleaved)
            {
                scratch.ensureSize ((size_t) ((int) sizeof (float) * numSamples * numChannelsRunning), false);
                scratch.fillWith (0); // (not clearing this data causes warnings in valgrind)

                auto num = snd_pcm_readi (handle, scratch.getData(), (snd_pcm_uframes_t) numSamples);

                if (num < 0)
                {
                    if (num == -(EPIPE))
                        overrunCount++;

                    if (ALOE_ALSA_FAILED (snd_pcm_recover (handle, (int) num, 1 /* silent */)))
                        return false;
                }


                if (num < numSamples)
                    ALOE_ALSA_LOG ("Did not read all samples: num: " << num << ", numSamples: " << numSamples);

                for (int i = 0; i < numChannelsRunning; ++i)
                    converter->convertSamples (data[i], 0, scratch.getData(), i, numSamples);
            }
            else
            {
                auto num = snd_pcm_readn (handle, (void**) data, (snd_pcm_uframes_t) numSamples);

                if (num < 0)
                {
                    if (num == -(EPIPE))
                        overrunCount++;

                    if (ALOE_ALSA_FAILED (snd_pcm_recover (handle, (int) num, 1 /* silent */)))
                        return false;
                }

                if (num < numSamples)
                    ALOE_ALSA_LOG ("Did not read all samples: num: " << num << ", numSamples: " << numSamples);

                for (int i = 0; i < numChannelsRunning; ++i)
                    converter->convertSamples (data[i], data[i], numSamples);
            }

            return true;
        */
    }
    
    pub fn create_converter(
        for_input:                bool,
        bit_depth:                i32,
        is_float:                 bool,
        is_little_endian:         bool,
        use_only_lower_24bits:    bool,
        num_interleaved_channels: i32,
        interleaved:              bool) -> *mut AudioDataConverter {
        
        todo!();
        /*
            ALOE_ALSA_LOG ("format: bitDepth=" << bitDepth << ", isFloat=" << (int) isFloat
                            << ", isLittleEndian=" << (int) isLittleEndian << ", numChannels=" << numInterleavedChannels);

            if (isFloat)         return ALSADeviceConverterHelper <AudioData::Float32>::createConverter (forInput, isLittleEndian, numInterleavedChannels, interleaved);
            if (bitDepth == 16)  return ALSADeviceConverterHelper <AudioData::Int16>  ::createConverter (forInput, isLittleEndian, numInterleavedChannels, interleaved);
            if (bitDepth == 24)  return ALSADeviceConverterHelper <AudioData::Int24>  ::createConverter (forInput, isLittleEndian, numInterleavedChannels, interleaved);

            jassert (bitDepth == 32);

            if (useOnlyLower24Bits)
                return ALSADeviceConverterHelper <AudioData::Int24in32>::createConverter (forInput, isLittleEndian, numInterleavedChannels, interleaved);

            return ALSADeviceConverterHelper <AudioData::Int32>::createConverter (forInput, isLittleEndian, numInterleavedChannels, interleaved);
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
}

