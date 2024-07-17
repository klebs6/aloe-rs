crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/DataConversionFlowGraph.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/DataConversionFlowGraph.cpp]

pub fn convert_oboe_sr_quality_tomcr(quality: OboeSampleRateConversionQuality) 
    -> MultiChannelResamplerQuality 
{
    todo!();

    /*
        switch (quality) {
            case SampleRateConversionQuality::Fastest:
                return MultiChannelResampler::Quality::Fastest;
            case SampleRateConversionQuality::Low:
                return MultiChannelResampler::Quality::Low;
            default:
            case SampleRateConversionQuality::Medium:
                return MultiChannelResampler::Quality::Medium;
            case SampleRateConversionQuality::High:
                return MultiChannelResampler::Quality::High;
            case SampleRateConversionQuality::Best:
                return MultiChannelResampler::Quality::Best;
        }
    */
}

/**
  | Convert PCM channels, format and sample
  | rate for optimal latency.
  |
  */
pub struct DataConversionFlowGraph<'a> {
    source:                  Box<FlowGraphSourceBuffered<'a>>,
    source_caller:           Box<AudioSourceCaller<'a>>,
    mono_to_multi_converter: Box<FlowgraphMonoToMultiConverter<'a>>,
    multi_to_mono_converter: Box<FlowgraphMultiToMonoConverter<'a>>,
    channel_count_converter: Box<FlowgraphChannelCountConverter<'a>>,
    resampler:               Box<MultiChannelResampler>,
    rate_converter:          Box<SampleRateConverter<'a>>,
    sink:                    Box<FlowGraphSink<'a>>,
    block_writer:            FixedBlockWriter<'a>,
    callback_result:         OboeDataCallbackResult, // default = DataCallbackResult::Continue
    filter_stream:           *mut AudioStream, // default = nullptr
    app_buffer:              Box<&'a [u8]>,
}

impl<'a> FixedBlockProcessor for DataConversionFlowGraph<'a> {

    fn on_process_fixed_block(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            int32_t numFrames = numBytes / mFilterStream->getBytesPerFrame();
        mCallbackResult = mFilterStream->getDataCallback()->onAudioReady(mFilterStream, buffer, numFrames);
        // TODO handle STOP from callback, process data remaining in the block adapter
        return numBytes;
        */
    }
}

impl<'a> Default for DataConversionFlowGraph<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : mBlockWriter(*this
        */
    }
}

impl<'a> DataConversionFlowGraph<'a> {

    pub fn get_data_callback_result(&mut self) -> OboeDataCallbackResult {
        
        todo!();
        /*
            return mCallbackResult;
        */
    }
    
    pub fn set_source(&mut self, 
        buffer:     *const c_void,
        num_frames: i32)  {
        
        todo!();
        /*
            mSource->setData(buffer, numFrames);
        */
    }

    /**
      | Chain together multiple processors.
      |
      | Callback Output
      |
      |     Use SourceCaller that calls original app
      |     callback from the flowgraph.
      |
      |     The child callback from FilteredAudioStream
      |     read()s from the flowgraph.
      |
      | Callback Input
      |
      |     Child callback from FilteredAudioStream
      |     writes()s to the flowgraph.
      |
      |     The output of the flowgraph goes through
      |     a BlockWriter to the app callback.
      |
      | Blocking Write
      |
      |     Write buffer is set on an AudioSource.
      |
      |     Data is pulled through the graph and
      |     written to the child stream.
      |
      | Blocking Read
      |
      |     Reads in a loop from the flowgraph Sink to
      |     fill the read buffer.
      |
      |     A SourceCaller then does a blocking read
      |     from the child Stream.
      |
      _______________________________
      | Connect several modules together to
      | convert from source to sink.
      | 
      | This should only be called once for each
      | instance.
      | 
      | -----------
      | @param sourceFormat
      | 
      | @param sourceChannelCount
      | ----------
      | @param sinkFormat
      | 
      | @param sinkChannelCount
      | 
      | @return
      |
      */
    pub fn configure(&mut self, 
        source_stream: *mut AudioStream,
        sink_stream:   *mut AudioStream) -> OboeResult {
        
        todo!();
        /*
            FlowGraphPortFloatOutput *lastOutput = nullptr;

        bool isOutput = sourceStream->getDirection() == Direction::Output;
        bool isInput = !isOutput;
        mFilterStream = isOutput ? sourceStream : sinkStream;

        AudioFormat sourceFormat = sourceStream->getFormat();
        int32_t sourceChannelCount = sourceStream->getChannelCount();
        int32_t sourceSampleRate = sourceStream->getSampleRate();
        int32_t sourceFramesPerCallback = sourceStream->getFramesPerDataCallback();

        AudioFormat sinkFormat = sinkStream->getFormat();
        int32_t sinkChannelCount = sinkStream->getChannelCount();
        int32_t sinkSampleRate = sinkStream->getSampleRate();
        int32_t sinkFramesPerCallback = sinkStream->getFramesPerDataCallback();

        LOGI("%s() flowgraph converts channels: %d to %d, format: %d to %d"
             ", rate: %d to %d, cbsize: %d to %d, qual = %d",
                __func__,
                sourceChannelCount, sinkChannelCount,
                sourceFormat, sinkFormat,
                sourceSampleRate, sinkSampleRate,
                sourceFramesPerCallback, sinkFramesPerCallback,
                sourceStream->getSampleRateConversionQuality());

        // Source
        // IF OUTPUT and using a callback then call back to the app using a SourceCaller.
        // OR IF INPUT and NOT using a callback then read from the child stream using a SourceCaller.
        bool isDataCallbackSpecified = sourceStream->isDataCallbackSpecified();
        if ((isDataCallbackSpecified && isOutput)
            || (!isDataCallbackSpecified && isInput)) {
            int32_t actualSourceFramesPerCallback = (sourceFramesPerCallback == kUnspecified)
                    ? sourceStream->getFramesPerBurst()
                    : sourceFramesPerCallback;
            switch (sourceFormat) {
                case AudioFormat::Float:
                    mSourceCaller = std::make_unique<SourceFloatCaller>(sourceChannelCount,
                                                                        actualSourceFramesPerCallback);
                    break;
                case AudioFormat::I16:
                    mSourceCaller = std::make_unique<SourceI16Caller>(sourceChannelCount,
                                                                      actualSourceFramesPerCallback);
                    break;
                case AudioFormat::I24:
                    mSourceCaller = std::make_unique<SourceI24Caller>(sourceChannelCount,
                                                                      actualSourceFramesPerCallback);
                    break;
                case AudioFormat::I32:
                    mSourceCaller = std::make_unique<SourceI32Caller>(sourceChannelCount,
                                                                      actualSourceFramesPerCallback);
                    break;
                default:
                    LOGE("%s() Unsupported source caller format = %d", __func__, sourceFormat);
                    return OboeResult::ErrorIllegalArgument;
            }
            mSourceCaller->setStream(sourceStream);
            lastOutput = &mSourceCaller->output;
        } else {
            // IF OUTPUT and NOT using a callback then write to the child stream using a BlockWriter.
            // OR IF INPUT and using a callback then write to the app using a BlockWriter.
            switch (sourceFormat) {
                case AudioFormat::Float:
                    mSource = std::make_unique<SourceFloat>(sourceChannelCount);
                    break;
                case AudioFormat::I16:
                    mSource = std::make_unique<SourceI16>(sourceChannelCount);
                    break;
                case AudioFormat::I24:
                    mSource = std::make_unique<SourceI24>(sourceChannelCount);
                    break;
                case AudioFormat::I32:
                    mSource = std::make_unique<SourceI32>(sourceChannelCount);
                    break;
                default:
                    LOGE("%s() Unsupported source format = %d", __func__, sourceFormat);
                    return OboeResult::ErrorIllegalArgument;
            }
            if (isInput) {
                int32_t actualSinkFramesPerCallback = (sinkFramesPerCallback == kUnspecified)
                        ? sinkStream->getFramesPerBurst()
                        : sinkFramesPerCallback;
                // The BlockWriter is after the Sink so use the SinkStream size.
                mBlockWriter.open(actualSinkFramesPerCallback * sinkStream->getBytesPerFrame());
                mAppBuffer = std::make_unique<uint8_t[]>(
                        kDefaultBufferSize * sinkStream->getBytesPerFrame());
            }
            lastOutput = &mSource->output;
        }

        // If we are going to reduce the number of channels then do it before the
        // sample rate converter.
        if (sourceChannelCount > sinkChannelCount) {
            if (sinkChannelCount == 1) {
                mMultiToMonoConverter = std::make_unique<MultiToMonoConverter>(sourceChannelCount);
                lastOutput->connect(&mMultiToMonoConverter->input);
                lastOutput = &mMultiToMonoConverter->output;
            } else {
                mChannelCountConverter = std::make_unique<ChannelCountConverter>(
                        sourceChannelCount,
                        sinkChannelCount);
                lastOutput->connect(&mChannelCountConverter->input);
                lastOutput = &mChannelCountConverter->output;
            }
        }

        // Sample Rate conversion
        if (sourceSampleRate != sinkSampleRate) {
            // Create a resampler to do the math.
            mResampler.reset(MultiChannelResampler::make(lastOutput->getSamplesPerFrame(),
                                                         sourceSampleRate,
                                                         sinkSampleRate,
                                                         convertOboeSRQualityToMCR(
                                                                 sourceStream->getSampleRateConversionQuality())));
            // Make a flowgraph node that uses the resampler.
            mRateConverter = std::make_unique<SampleRateConverter>(lastOutput->getSamplesPerFrame(),
                                                                   *mResampler.get());
            lastOutput->connect(&mRateConverter->input);
            lastOutput = &mRateConverter->output;
        }

        // Expand the number of channels if required.
        if (sourceChannelCount < sinkChannelCount) {
            if (sourceChannelCount == 1) {
                mMonoToMultiConverter = std::make_unique<MonoToMultiConverter>(sinkChannelCount);
                lastOutput->connect(&mMonoToMultiConverter->input);
                lastOutput = &mMonoToMultiConverter->output;
            } else {
                mChannelCountConverter = std::make_unique<ChannelCountConverter>(
                        sourceChannelCount,
                        sinkChannelCount);
                lastOutput->connect(&mChannelCountConverter->input);
                lastOutput = &mChannelCountConverter->output;
            }
        }

        // Sink
        switch (sinkFormat) {
            case AudioFormat::Float:
                mSink = std::make_unique<SinkFloat>(sinkChannelCount);
                break;
            case AudioFormat::I16:
                mSink = std::make_unique<SinkI16>(sinkChannelCount);
                break;
            case AudioFormat::I24:
                mSink = std::make_unique<SinkI24>(sinkChannelCount);
                break;
            case AudioFormat::I32:
                mSink = std::make_unique<SinkI32>(sinkChannelCount);
                break;
            default:
                LOGE("%s() Unsupported sink format = %d", __func__, sinkFormat);
                return OboeResult::ErrorIllegalArgument;;
        }
        lastOutput->connect(&mSink->input);

        return OboeResult::OK;
        */
    }
    
    pub fn read(&mut self, 
        buffer:        *mut c_void,
        num_frames:    i32,
        timeout_nanos: i64) -> i32 {
        
        todo!();
        /*
            if (mSourceCaller) {
            mSourceCaller->setTimeoutNanos(timeoutNanos);
        }
        int32_t numRead = mSink->read(buffer, numFrames);
        return numRead;
        */
    }

    /**
       This is similar to pushing data through the
       flowgraph.
      */
    pub fn write(&mut self, 
        input_buffer: *mut c_void,
        num_frames:   i32) -> i32 {
        
        todo!();
        /*
            // Put the data from the input at the head of the flowgraph.
        mSource->setData(inputBuffer, numFrames);
        while (true) {
            // Pull and read some data in app format into a small buffer.
            int32_t framesRead = mSink->read(mAppBuffer.get(), flowgraph::kDefaultBufferSize);
            if (framesRead <= 0) break;
            // Write to a block adapter, which will call the destination whenever it has enough data.
            int32_t bytesRead = mBlockWriter.write(mAppBuffer.get(),
                                                   framesRead * mFilterStream->getBytesPerFrame());
            if (bytesRead < 0) return bytesRead; // TODO review
        }
        return numFrames;
        */
    }
    
}
