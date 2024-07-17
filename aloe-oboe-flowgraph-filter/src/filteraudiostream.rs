crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FilterAudioStream.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FilterAudioStream.cpp]

/**
  | An AudioStream that wraps another AudioStream
  | and provides audio data conversion.
  | 
  | Operations may include channel conversion,
  | data format conversion and/or sample
  | rate conversion.
  |
  */
pub struct FilterAudioStream<'a> {
    base:            AudioStream,

    /**
      | this stream wraps the child stream
      |
      */
    child_stream:    Box<AudioStream>,

    /**
      | for converting data
      |
      */
    flow_graph:      Box<dyn DataConversionFlowGraphInterface>,

    /**
      | temp buffer for write()
      |
      */
    blocking_buffer: Box<&'a [u8]>,

    /**
      | ratio parent/child sample rates
      |
      */
    rate_scaler:     f64, // default = 1.0
}

impl<'a> AudioStreamCallback for FilterAudioStream<'a> {

}

impl<'a> AudioStreamErrorCallback for FilterAudioStream<'a> {

}

impl<'a> AudioStreamDataCallback for FilterAudioStream<'a> {

    fn on_audio_ready(
        &mut self, 
        _: *mut AudioStream, 
        _: *mut c_void, 
        _: i32

    ) -> OboeDataCallbackResult { 

        todo!() 
    }
}

impl<'a> FilterAudioStream<'a> {

    /**
      | Construct an `AudioStream` using the
      | given `AudioStreamBuilder` and a child
      | AudioStream.
      | 
      | This should only be called internally
      | by AudioStreamBuilder.
      | 
      | Ownership of childStream will be passed
      | to this object.
      | 
      | -----------
      | @param builder
      | 
      | containing all the stream's attributes
      |
      */
    pub fn new(
        builder:      &AudioStreamBuilder,
        child_stream: *mut AudioStream) -> Self {
    
        todo!();
        /*
        : audio_stream(builder),
        : child_stream(childStream),

            // Intercept the callback if used.
            if (builder.isErrorCallbackSpecified()) {
                mErrorCallback = mChildStream->swapErrorCallback(this);
            }
            if (builder.isDataCallbackSpecified()) {
                mDataCallback = mChildStream->swapDataCallback(this);
            } else {
                const int size = childStream->getFramesPerBurst() * childStream->getBytesPerFrame();
                mBlockingBuffer = std::make_unique<uint8_t[]>(size);
            }

            // Copy parameters that may not match builder.
            mBufferCapacityInFrames = mChildStream->getBufferCapacityInFrames();
            mPerformanceMode = mChildStream->getPerformanceMode();
            mInputPreset = mChildStream->getInputPreset();
            mFramesPerBurst = mChildStream->getFramesPerBurst();
            mDeviceId = mChildStream->getDeviceId();
        */
    }
    
    pub fn get_child_stream(&self) -> *mut AudioStream {
        
        todo!();
        /*
            return mChildStream.get();
        */
    }
    
    /**
      | Close child and parent.
      |
      */
    pub fn close(&mut self) -> OboeResult {
        
        todo!();
        /*
            const OboeResult result1 = mChildStream->close();
            const OboeResult result2 = AudioStream::close();
            return (result1 != OboeResult::OK ? result1 : result2);
        */
    }

    /**
      | Start the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `start(0)`.
      |
      */
    pub fn request_start(&mut self) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->requestStart();
        */
    }

    /**
      | Pause the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `pause(0)`.
      |
      */
    pub fn request_pause(&mut self) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->requestPause();
        */
    }

    /**
      | Flush the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `flush(0)`.
      |
      */
    pub fn request_flush(&mut self) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->requestFlush();
        */
    }

    /**
      | Stop the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `stop(0)`.
      |
      */
    pub fn request_stop(&mut self) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->requestStop();
        */
    }
    
    pub fn get_state(&mut self) -> OboeStreamState {
        
        todo!();
        /*
            return mChildStream->getState();
        */
    }
    
    pub fn wait_for_state_change(&mut self, 
        input_state:         OboeStreamState,
        next_state:          *mut OboeStreamState,
        timeout_nanoseconds: i64) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->waitForStateChange(inputState, nextState, timeoutNanoseconds);
        */
    }
    
    pub fn is_xrun_count_supported(&self) -> bool {
        
        todo!();
        /*
            return mChildStream->isXRunCountSupported();
        */
    }
    
    pub fn get_audio_api(&self) -> OboeAudioApi {
        
        todo!();
        /*
            return mChildStream->getAudioApi();
        */
    }
    
    pub fn update_frames_written(&mut self)  {
        
        todo!();
        /*
            // TODO for output, just count local writes?
            mFramesWritten = static_cast<int64_t>(mChildStream->getFramesWritten() * mRateScaler);
        */
    }
    
    pub fn update_frames_read(&mut self)  {
        
        todo!();
        /*
            // TODO for input, just count local reads?
            mFramesRead = static_cast<int64_t>(mChildStream->getFramesRead() * mRateScaler);
        */
    }
    
    pub fn get_underlying_stream(&self)  {
        
        todo!();
        /*
            return mChildStream->getUnderlyingStream();
        */
    }
    
    pub fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return mChildStream->setBufferSizeInFrames(requestedFrames);
        */
    }
    
    pub fn get_buffer_size_in_frames(&mut self) -> i32 {
        
        todo!();
        /*
            mBufferSizeInFrames = mChildStream->getBufferSizeInFrames();
            return mBufferSizeInFrames;
        */
    }
    
    pub fn get_xrun_count(&mut self) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            return mChildStream->getXRunCount();
        */
    }
    
    pub fn calculate_latency_millis(&mut self) -> OboeResultWithValue<f64> {
        
        todo!();
        /*
            // This will automatically include the latency of the flowgraph?
            return mChildStream->calculateLatencyMillis();
        */
    }
    
    pub fn get_timestamp(
        &mut self, 
        clock_id:         ClockId,
        frame_position:   *mut i64,
        time_nanoseconds: *mut i64

    ) -> OboeResult {
        
        todo!();
        /*
            int64_t childPosition = 0;
            OboeResult result = mChildStream->getTimestamp(clockId, &childPosition, timeNanoseconds);
            // It is OK if framePosition is null.
            if (framePosition) {
                *framePosition = childPosition * mRateScaler;
            }
            return result;
        */
    }
    
    pub fn on_error(
        &mut self, 
        audio_stream: *mut AudioStream,
        error:        OboeResult

    ) -> bool {
        
        todo!();
        /*
            if (mErrorCallback != nullptr) {
                return mErrorCallback->onError(this, error);
            }
            return false;
        */
    }
    
    pub fn on_error_before_close(&mut self, 
        oboe_stream: *mut AudioStream,
        error:       OboeResult)  {
        
        todo!();
        /*
            if (mErrorCallback != nullptr) {
                mErrorCallback->onErrorBeforeClose(this, error);
            }
        */
    }
    
    pub fn on_error_after_close(&mut self, 
        oboe_stream: *mut AudioStream,
        error:       OboeResult)  {
        
        todo!();
        /*
            // Close this parent stream because the callback will only close the child.
            AudioStream::close();
            if (mErrorCallback != nullptr) {
                mErrorCallback->onErrorAfterClose(this, error);
            }
        */
    }

    /**
      | @return
      | 
      | last result passed from an error callback
      |
      */
    pub fn get_last_error_callback_result(&self) -> OboeResult {
        
        todo!();
        /*
            return mChildStream->getLastErrorCallbackResult();
        */
    }

    /**
      | Output callback uses FixedBlockReader::read()
      |                <= SourceFloatCaller::onProcess()
      |                <=== DataConversionFlowGraph::read()
      |                <== FilterAudioStream::onAudioReady()
      |
      | Output blocking uses no block adapter because
      | AAudio can accept writes of any size. It
      | uses DataConversionFlowGraph::read()
      | <== FilterAudioStream::write() <= app
      |
      | Input callback uses FixedBlockWriter::write()
      |                <= DataConversionFlowGraph::write()
      |                <= FilterAudioStream::onAudioReady()
      |
      |// TODO may not need block adapter
      | Input blocking uses FixedBlockReader::read()
      |
      |                <= SourceFloatCaller::onProcess()
      |                <=== SinkFloat::read()
      |                <= DataConversionFlowGraph::read()
      |                <== FilterAudioStream::read()
      |                <= app
      */
    pub fn configure_flow_graph(&mut self) -> OboeResult {
        
        todo!();
        /*
            mFlowGraph = std::make_unique<DataConversionFlowGraph>();
        bool isOutput = getDirection() == Direction::Output;

        AudioStream *sourceStream =  isOutput ? this : mChildStream.get();
        AudioStream *sinkStream =  isOutput ? mChildStream.get() : this;

        mRateScaler = ((double) getSampleRate()) / mChildStream->getSampleRate();

        return mFlowGraph->configure(sourceStream, sinkStream);
        */
    }

    /**
      | Put the data to be written at the source end of
      | the flowgraph.
      |
      | Then read (pull) the data from the flowgraph
      | and write it to the child stream.
      */
    pub fn write(&mut self, 
        buffer:              *const c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            int32_t framesWritten = 0;
        mFlowGraph->setSource(buffer, numFrames);
        while (true) {
            int32_t numRead = mFlowGraph->read(mBlockingBuffer.get(),
                    getFramesPerBurst(),
                    timeoutNanoseconds);
            if (numRead < 0) {
                return ResultWithValue<int32_t>::createBasedOnSign(numRead);
            }
            if (numRead == 0) {
                break; // finished processing the source buffer
            }
            auto writeResult = mChildStream->write(mBlockingBuffer.get(),
                                                   numRead,
                                                   timeoutNanoseconds);
            if (!writeResult) {
                return writeResult;
            }
            framesWritten += writeResult.value();
        }
        return ResultWithValue<int32_t>::createBasedOnSign(framesWritten);
        */
    }

    /**
      | Read (pull) the data we want from the sink end
      | of the flowgraph.
      |
      | The necessary data will be read from the child
      | stream using a flowgraph callback.
      */
    pub fn read(&mut self, 
        buffer:              *mut c_void,
        num_frames:          i32,
        timeout_nanoseconds: i64) -> OboeResultWithValue<i32> {
        
        todo!();
        /*
            int32_t framesRead = mFlowGraph->read(buffer, numFrames, timeoutNanoseconds);
        return ResultWithValue<int32_t>::createBasedOnSign(framesRead);
        */
    }
    
    pub fn on_audio_ready(&mut self, 
        oboe_stream: *mut AudioStream,
        audio_data:  *mut c_void,
        num_frames:  i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            int32_t framesProcessed;
        if (oboeStream->getDirection() == Direction::Output) {
            framesProcessed = mFlowGraph->read(audioData, numFrames, 0 /* timeout */);
        } else {
            framesProcessed = mFlowGraph->write(audioData, numFrames);
        }
        return (framesProcessed < numFrames)
               ? DataCallbackResult::Stop
               : mFlowGraph->getDataCallbackResult();
        */
    }
}
