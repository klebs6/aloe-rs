crate::ix!();

pub struct OboeSessionBase<'a> {
    base:                OboeAudioStreamCallback,
    owner:               &'a mut OboeAudioIODevice,
    input_device_id:     i32,
    output_device_id:    i32,
    num_input_channels:  i32,
    num_output_channels: i32,
    sample_rate:         i32,
    buffer_size:         i32,
    stream_format:       OboeAudioFormat,
    bit_depth:           i32,
    input_stream:        Box<OboeStream>,
    output_stream:       Box<OboeStream>,
}

impl<'a> OboeSessionBase<'a> {

    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            if (inputStream != nullptr && ! inputStream->openedOk())
                    return false;

                return outputStream != nullptr && outputStream->openedOk();
        */
    }
    
    pub fn get_current_bit_depth(&self) -> i32 {
        
        todo!();
        /*
            return bitDepth;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            int inputXRunCount  = jmax (0, inputStream  != nullptr ? inputStream->getXRunCount() : 0);
                int outputXRunCount = jmax (0, outputStream != nullptr ? outputStream->getXRunCount() : 0);

                return inputXRunCount + outputXRunCount;
        */
    }
    
    pub fn new(
        owner_to_use:               &mut OboeAudioIODevice,
        input_device_id_to_use:     i32,
        output_device_id_to_use:    i32,
        num_input_channels_to_use:  i32,
        num_output_channels_to_use: i32,
        sample_rate_to_use:         i32,
        buffer_size_to_use:         i32,
        stream_format_to_use:       OboeAudioFormat,
        bit_depth_to_use:           i32) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  inputDeviceId (inputDeviceIdToUse),
                  outputDeviceId (outputDeviceIdToUse),
                  numInputChannels (numInputChannelsToUse),
                  numOutputChannels (numOutputChannelsToUse),
                  sampleRate (sampleRateToUse),
                  bufferSize (bufferSizeToUse),
                  streamFormat (streamFormatToUse),
                  bitDepth (bitDepthToUse),
                  outputStream (new OboeStream (outputDeviceId,
                                                OboeDirection::Output,
                                                OboeSharingMode::Exclusive,
                                                numOutputChannels,
                                                streamFormatToUse,
                                                sampleRateToUse,
                                                bufferSizeToUse,
                                                this))

                if (numInputChannels > 0)
                {
                    inputStream.reset (new OboeStream (inputDeviceId,
                                                       OboeDirection::Input,
                                                       OboeSharingMode::Exclusive,
                                                       numInputChannels,
                                                       streamFormatToUse,
                                                       sampleRateToUse,
                                                       bufferSizeToUse,
                                                       nullptr));

                    if (inputStream->openedOk() && outputStream->openedOk())
                    {
                        // Input & output sample rates should match!
                        jassert (inputStream->getNativeStream()->getSampleRate()
                                   == outputStream->getNativeStream()->getSampleRate());
                    }

                    checkStreamSetup (inputStream.get(), inputDeviceId, numInputChannels,
                                      sampleRate, bufferSize, streamFormat);
                }

                checkStreamSetup (outputStream.get(), outputDeviceId, numOutputChannels,
                                  sampleRate, bufferSize, streamFormat);
        */
    }

    /**
      | Not strictly required as these should
      | not change, but recommended by Google
      | anyway
      |
      */
    pub fn check_stream_setup(&mut self, 
        stream:               *mut OboeStream,
        device_id:            i32,
        num_channels:         i32,
        expected_sample_rate: i32,
        expected_buffer_size: i32,
        format:               OboeAudioFormat)  {
        
        todo!();
        /*
            if (auto* nativeStream = stream != nullptr ? stream->getNativeStream() : nullptr)
                {
                    ignoreUnused (deviceId, numChannels, sampleRate, expectedBufferSize);
                    ignoreUnused (streamFormat, bitDepth);

                    jassert (numChannels == 0 || numChannels == nativeStream->getChannelCount());
                    jassert (expectedSampleRate == 0 || expectedSampleRate == nativeStream->getSampleRate());
                    jassert (format == nativeStream->getFormat());
                }
        */
    }
    
    pub fn get_buffer_capacity_in_frames(&self, for_input: bool) -> i32 {
        
        todo!();
        /*
            auto& ptr = forInput ? inputStream : outputStream;

                if (ptr == nullptr || ! ptr->openedOk())
                    return 0;

                return ptr->getNativeStream()->getBufferCapacityInFrames();
        */
    }
    
    pub fn create(&mut self, 
        owner:               &mut OboeAudioIODevice,
        input_device_id:     i32,
        output_device_id:    i32,
        num_input_channels:  i32,
        num_output_channels: i32,
        sample_rate:         i32,
        buffer_size:         i32) -> *mut OboeSessionBase {
        
        todo!();
        /*
            std::unique_ptr<OboeSessionBase> session;
        auto sdkVersion = getAndroidSDKVersion();

        // SDK versions 21 and higher should natively support floating point...
        if (sdkVersion >= 21)
        {
            session.reset (new OboeSessionImpl<float> (owner, inputDeviceId, outputDeviceId,
                                                       numInputChannels, numOutputChannels, sampleRate, bufferSize));

            // ...however, some devices lie so re-try without floating point
            if (session != nullptr && (! session->openedOk()))
                session.reset();
        }

        if (session == nullptr)
        {
            session.reset (new OboeSessionImpl<int16> (owner, inputDeviceId, outputDeviceId,
                                                       numInputChannels, numOutputChannels, sampleRate, bufferSize));

            if (session != nullptr && (! session->openedOk()))
                session.reset();
        }

        return session.release();
        */
    }
}
