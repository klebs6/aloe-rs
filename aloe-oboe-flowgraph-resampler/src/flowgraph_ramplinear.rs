crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphRampLinear.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphRampLinear.cpp]

/**
  | When the target is modified then the
  | output will ramp smoothly between the
  | original and the new target value.
  | 
  | This can be used to smooth out control
  | values and reduce pops.
  | 
  | The target may be updated while a ramp
  | is in progress, which will trigger a
  | new ramp from the current value.
  |
  */
pub struct FlowgraphRampLinear<'a> {
    base:             FlowGraphFilter<'a>,
    target:           Atomic<f32>,

    /**
       10 msec at 48000 Hz;
      */
    length_in_frames: i32, // default = 48000.0 / 100.0 ;
    remaining:        i32, // default = 0
    scaler:           f32, // default = 0.0f
    level_from:       f32, // default = 0.0f
    level_to:         f32, // default = 0.0f
}

impl<'a> FlowgraphRampLinear<'a> {

    pub fn get_length_in_frames(&self) -> i32 {
        
        todo!();
        /*
            return mLengthInFrames;
        */
    }

    pub fn get_target(&self) -> f32 {
        
        todo!();
        /*
            return mTarget.load();
        */
    }

    /**
      | Force the nextSegment to start from
      | this level.
      | 
      | -----------
      | @warning
      | 
      | this can cause a discontinuity if called
      | while the ramp is being used. Only call
      | this when setting the initial ramp.
      | 
      | @param level
      |
      */
    pub fn force_current(&mut self, level: f32)  {
        
        todo!();
        /*
            mLevelFrom = level;
            mLevelTo = level;
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphRampLinear";
        */
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_filter(channelCount),

            mTarget.store(1.0f);
        */
    }
    
    /**
      | This is used for the next ramp.
      | 
      | Calling this does not affect a ramp that
      | is in progress.
      |
      */
    pub fn set_length_in_frames(&mut self, frames: i32)  {
        
        todo!();
        /*
            mLengthInFrames = frames;
        */
    }
    
    /**
      | This may be safely called by another
      | thread.
      | 
      | @param target
      |
      */
    pub fn set_target(&mut self, target: f32)  {
        
        todo!();
        /*
            mTarget.store(target);
        // If the ramp has not been used then start immediately at this level.
        if (mLastCallCount == kInitialCallCount) {
            forceCurrent(target);
        }
        */
    }
    
    pub fn interpolate_current(&mut self) -> f32 {
        
        todo!();
        /*
            return mLevelTo - (mRemaining * mScaler);
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        float *outputBuffer = output.getBuffer();
        int32_t channelCount = output.getSamplesPerFrame();

        float target = getTarget();
        if (target != mLevelTo) {
            // Start new ramp. Continue from previous level.
            mLevelFrom = interpolateCurrent();
            mLevelTo = target;
            mRemaining = mLengthInFrames;
            mScaler = (mLevelTo - mLevelFrom) / mLengthInFrames; // for interpolation
        }

        int32_t framesLeft = numFrames;

        if (mRemaining > 0) { // Ramping? This doesn't happen very often.
            int32_t framesToRamp = std::min(framesLeft, mRemaining);
            framesLeft -= framesToRamp;
            while (framesToRamp > 0) {
                float currentLevel = interpolateCurrent();
                for (int ch = 0; ch < channelCount; ch++) {
                    *outputBuffer++ = *inputBuffer++ * currentLevel;
                }
                mRemaining--;
                framesToRamp--;
            }
        }

        // Process any frames after the ramp.
        int32_t samplesLeft = framesLeft * channelCount;
        for (int i = 0; i < samplesLeft; i++) {
            *outputBuffer++ = *inputBuffer++ * mLevelTo;
        }

        return numFrames;
        */
    }
}
