crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/StabilizedCallback.cpp]

pub const kLoadGenerationStepSizeNanos: i32 = 20000;
pub const kPercentageOfCallbackToUse:   f32 = 0.8;

impl OboeStabilizedCallback {

    pub fn new(callback: *mut dyn AudioStreamCallback) -> Self {
    
        todo!();
        /*
        : callback(callback),

            Trace::initialize();
        */
    }

    /**
      | An audio callback which attempts to
      | do work for a fixed amount of time.
      | 
      | -----------
      | @param oboeStream
      | 
      | -----------
      | @param audioData
      | ----------
      | @param numFrames
      | 
      | @return
      |
      */
    pub fn on_audio_ready(&mut self, 
        oboe_stream: *mut AudioStream,
        audio_data:  *mut c_void,
        num_frames:  i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            int64_t startTimeNanos = AudioClock::getNanoseconds();

        if (mFrameCount == 0){
            mEpochTimeNanos = startTimeNanos;
        }

        int64_t durationSinceEpochNanos = startTimeNanos - mEpochTimeNanos;

        // In an ideal world the callback start time will be exactly the same as the duration of the
        // frames already read/written into the stream. In reality the callback can start early
        // or late. By finding the delta we can calculate the target duration for our stabilized
        // callback.
        int64_t idealStartTimeNanos = (mFrameCount * kNanosPerSecond) / oboeStream->getSampleRate();
        int64_t lateStartNanos = durationSinceEpochNanos - idealStartTimeNanos;

        if (lateStartNanos < 0){
            // This was an early start which indicates that our previous epoch was a late callback.
            // Update our epoch to this more accurate time.
            mEpochTimeNanos = startTimeNanos;
            mFrameCount = 0;
        }

        int64_t numFramesAsNanos = (numFrames * kNanosPerSecond) / oboeStream->getSampleRate();
        int64_t targetDurationNanos = static_cast<int64_t>(
                (numFramesAsNanos * kPercentageOfCallbackToUse) - lateStartNanos);

        Trace::beginSection("Actual load");
        DataCallbackResult result = mCallback->onAudioReady(oboeStream, audioData, numFrames);
        Trace::endSection();

        int64_t executionDurationNanos = AudioClock::getNanoseconds() - startTimeNanos;
        int64_t stabilizingLoadDurationNanos = targetDurationNanos - executionDurationNanos;

        Trace::beginSection("Stabilized load for %lldns", stabilizingLoadDurationNanos);
        generateLoad(stabilizingLoadDurationNanos);
        Trace::endSection();

        // Wraparound: At 48000 frames per second mFrameCount wraparound will occur after 6m years,
        // significantly longer than the average lifetime of an Android phone.
        mFrameCount += numFrames;
        return result;
        */
    }
    
    pub fn generate_load(&mut self, duration_nanos: i64)  {
        
        todo!();
        /*
            int64_t currentTimeNanos = AudioClock::getNanoseconds();
        int64_t deadlineTimeNanos = currentTimeNanos + durationNanos;

        // opsPerStep gives us an estimated number of operations which need to be run to fully utilize
        // the CPU for a fixed amount of time (specified by kLoadGenerationStepSizeNanos).
        // After each step the opsPerStep value is re-calculated based on the actual time taken to
        // execute those operations.
        auto opsPerStep = (int)(mOpsPerNano * kLoadGenerationStepSizeNanos);
        int64_t stepDurationNanos = 0;
        int64_t previousTimeNanos = 0;

        while (currentTimeNanos <= deadlineTimeNanos){

            for (int i = 0; i < opsPerStep; i++) cpu_relax();

            previousTimeNanos = currentTimeNanos;
            currentTimeNanos = AudioClock::getNanoseconds();
            stepDurationNanos = currentTimeNanos - previousTimeNanos;

            // Calculate exponential moving average to smooth out values, this acts as a low pass filter.
            // @see https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average
            static const float kFilterCoefficient = 0.1;
            auto measuredOpsPerNano = (double) opsPerStep / stepDurationNanos;
            mOpsPerNano = kFilterCoefficient * measuredOpsPerNano + (1.0 - kFilterCoefficient) * mOpsPerNano;
            opsPerStep = (int) (mOpsPerNano * kLoadGenerationStepSizeNanos);
        }
        */
    }
}
