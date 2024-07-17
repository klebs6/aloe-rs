crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/OboeLatencyTuner.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/LatencyTuner.cpp]

pub enum OboeLatencyTunerState {
    Idle,
    Active,
    AtMax,
    Unsupported
}

/**
  | arbitrary number of calls to wait before
  | bumping up the latency
  |
  */
pub const LATENCY_TUNER_IDLE_COUNT:         i32 = 8;
pub const LATENCY_TUNER_DEFAULT_NUM_BURSTS: i32 = 2;

/**
  | OboeLatencyTuner can be used to dynamically
  | tune the latency of an output stream.
  | 
  | It adjusts the stream's bufferSize
  | by monitoring the number of underruns.
  | 
  | This only affects the latency associated
  | with the first level of buffering that
  | is closest to the application. It does
  | not affect low latency in the HAL, or
  | touch latency in the UI.
  | 
  | Call tune() right before returning
  | from your data callback function if
  | using callbacks.
  | 
  | Call tune() right before calling write()
  | if using blocking writes.
  | 
  | If you want to see the ongoing results
  | of this tuning process then call stream->getBufferSize()
  | periodically.
  |
  */
pub struct OboeLatencyTuner<'a> {
    stream:                    &'a mut OboeAAudioStream,
    state:                     OboeLatencyTunerState, // default = OboeLatencyTunerState::Idle
    max_buffer_size:           i32, // default = 0
    previousx_runs:            i32, // default = 0
    idle_count_down:           i32, // default = 0
    minimum_buffer_size:       i32,
    buffer_size_increment:     i32,

    /**
      | TODO user atomic requester from AAudio
      |
      */
    latency_trigger_requests:  Atomic<i32>, // default = 0

    latency_trigger_responses: Atomic<i32>, // default = 0
}

impl<'a> From<&mut OboeAAudioStream> for OboeLatencyTuner<'a> {

    /**
      | Construct a new OboeLatencyTuner object
      | which will act on the given audio stream
      | 
      | -----------
      | @param stream
      | 
      | the stream who's latency will be tuned
      |
      */
    fn from(stream: &mut OboeAAudioStream) -> Self {
    
        todo!();
        /*
        
        */
    }
}

impl<'a> OboeLatencyTuner<'a> {

    /**
      | Set the minimum bufferSize in frames
      | that is used when the tuner is reset.
      | 
      | You may wish to call requestReset()
      | after calling this. @param bufferSize
      |
      */
    pub fn set_minimum_buffer_size(&mut self, buffer_size: i32)  {
        
        todo!();
        /*
            mMinimumBufferSize = bufferSize;
        */
    }
    
    pub fn get_minimum_buffer_size(&self) -> i32 {
        
        todo!();
        /*
            return mMinimumBufferSize;
        */
    }

    /**
      | Set the amount the bufferSize will be
      | incremented while tuning.
      | 
      | By default, this will be one burst.
      | 
      | -----------
      | @note
      | 
      | AAudio will quantize the buffer size
      | to a multiple of the burstSize. So the
      | final buffer sizes may not be a multiple
      | of this increment.
      | 
      | @param sizeIncrement
      |
      */
    pub fn set_buffer_size_increment(&mut self, size_increment: i32)  {
        
        todo!();
        /*
            mBufferSizeIncrement = sizeIncrement;
        */
    }
    
    pub fn get_buffer_size_increment(&self) -> i32 {
        
        todo!();
        /*
            return mBufferSizeIncrement;
        */
    }

    pub fn new_from_aaudio_stream(stream: &mut OboeAAudioStream) -> Self {
    
        todo!();
        /*
        : latency_tuner(stream, stream.getBufferCapacityInFrames()),

        
        */
    }
    
    /**
      | Construct a new OboeLatencyTuner object
      | which will act on the given audio stream.
      | 
      | -----------
      | @param stream
      | 
      | the stream who's latency will be tuned
      | @param the maximum buffer size which
      | the tune() operation will set the buffer
      | size to
      |
      */
    pub fn new(
        stream:              &mut OboeAAudioStream,
        maximum_buffer_size: i32) -> Self {
    
        todo!();
        /*
        : stream(stream),
        : max_buffer_size(maximumBufferSize),

            int32_t burstSize = stream.getFramesPerBurst();
        setMinimumBufferSize(kDefaultNumBursts * burstSize);
        setBufferSizeIncrement(burstSize);
        reset();
        */
    }
    
    /**
      | Adjust the bufferSizeInFrames to optimize
      | latency.
      | 
      | It will start with a low latency and then
      | raise it if an underrun occurs.
      | 
      | Latency tuning is only supported for
      | AAudio.
      | 
      | -----------
      | @return
      | 
      | OK or negative error, ErrorUnimplemented
      | for OpenSL ES
      |
      */
    pub fn tune(&mut self) -> OboeResult {
        
        todo!();
        /*
            if (mState == State::Unsupported) {
            return OboeResult::ErrorUnimplemented;
        }

        OboeResult result = OboeResult::OK;

        // Process reset requests.
        int32_t numRequests = mLatencyTriggerRequests.load();
        if (numRequests != mLatencyTriggerResponses.load()) {
            mLatencyTriggerResponses.store(numRequests);
            reset();
        }

        // Set state to Active if the idle countdown has reached zero.
        if (mState == State::Idle && --mIdleCountDown <= 0) {
            mState = State::Active;
        }

        // When state is Active attempt to change the buffer size if the number of xRuns has increased.
        if (mState == State::Active) {

            auto xRunCountResult = mStream.getXRunCount();
            if (xRunCountResult == OboeResult::OK) {
                if ((xRunCountResult.value() - mPreviousXRuns) > 0) {
                    mPreviousXRuns = xRunCountResult.value();
                    int32_t oldBufferSize = mStream.getBufferSizeInFrames();
                    int32_t requestedBufferSize = oldBufferSize + getBufferSizeIncrement();

                    // Do not request more than the maximum buffer size (which was either user-specified
                    // or was from stream->getBufferCapacityInFrames())
                    if (requestedBufferSize > mMaxBufferSize) requestedBufferSize = mMaxBufferSize;

                    // Note that this will not allocate more memory. It simply determines
                    // how much of the existing buffer capacity will be used. The size will be
                    // clipped to the bufferCapacity by AAudio.
                    auto setBufferResult = mStream.setBufferSizeInFrames(requestedBufferSize);
                    if (setBufferResult != OboeResult::OK) {
                        result = setBufferResult;
                        mState = State::Unsupported;
                    } else if (setBufferResult.value() == oldBufferSize) {
                        mState = State::AtMax;
                    }
                }
            } else {
                mState = State::Unsupported;
            }
        }

        if (mState == State::Unsupported) {
            result = OboeResult::ErrorUnimplemented;
        }

        if (mState == State::AtMax) {
            result = OboeResult::OK;
        }
        return result;
        */
    }
    
    /**
      | This may be called from another thread.
      | Then tune() will call reset(), which
      | will lower the latency to the minimum
      | and then allow it to rise back up if there
      | are glitches.
      | 
      | This is typically called in response
      | to a user decision to minimize latency.
      | In other words, call this from a button
      | handler.
      |
      */
    pub fn request_reset(&mut self)  {
        
        todo!();
        /*
            if (mState != State::Unsupported) {
            mLatencyTriggerRequests++;
        }
        */
    }
    
    /**
      | Drop the latency down to the minimum
      | and then let it rise back up.
      | 
      | This is useful if a glitch caused the
      | latency to increase and it hasn't gone
      | back down.
      | 
      | This should only be called in the same
      | thread as tune().
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mState = State::Idle;
        mIdleCountDown = kIdleCount;
        // Set to minimal latency
        mStream.setBufferSizeInFrames(getMinimumBufferSize());
        */
    }
    
    /**
      | @return
      | 
      | true if the audio stream's buffer size
      | is at the maximum value. If no maximum
      | value was specified when constructing
      | the OboeLatencyTuner then the value of stream->getBufferCapacityInFrames
      | is used
      |
      */
    pub fn is_at_maximum_buffer_size(&mut self) -> bool {
        
        todo!();
        /*
            return mState == State::AtMax;
        */
    }
}
