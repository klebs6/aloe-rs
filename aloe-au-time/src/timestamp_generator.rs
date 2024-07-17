crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUTimestampGenerator.h]

pub const TSGFMT: &'static str = "0x%10qx";

/**
  | This class generates a continuously increasing
  | series of timestamps based on a series of
  | potentially discontinuous timestamps (as can be
  | delivered from CoreAudio in the event of an
  | overload or major engine change).
  |
  | N.B.: "output" = downstream (source) timestamp
  |       "input"  = upstream (derived) timestamp
  */
pub struct AUTimestampGenerator {

    state:      AUTimestampGeneratorState,
    first_time: bool,

    #[cfg(DEBUG)] verbosity:  i32,
    #[cfg(DEBUG)] debug_name: [u8; 64],
}

impl AUTimestampGenerator {

    pub fn new(host_time_discontinuity_correction: Option<bool>) -> Self {

        let host_time_discontinuity_correction: bool =
                 host_time_discontinuity_correction.unwrap_or(false);

        todo!();
        /*

            mState.mStartInputAtZero = true;
            mState.mBypassed = false;
            mState.mHostTimeDiscontinuityCorrection = hostTimeDiscontinuityCorrection;
    #if DEBUG
            mVerbosity = 0;
            snprintf(mDebugName, sizeof(mDebugName), "tsg @ %p", this);
    #endif
            // CAHostTimeBase should be used instead of the calls in <CoreAudio/HostTime.h>
            // we make this call here to ensure that this is initialized, otherwise the first time
            // you do actually call CAHostTimeBase to do work, can be on the render thread, and lead to unwanted VM faults
            CAHostTimeBase::GetFrequency();
            Reset();
        */
    }
    
    pub fn set_start_input_at_zero(&mut self, b: bool)  {
        
        todo!();
        /*
            mState.mStartInputAtZero = b;
        */
    }
    
    pub fn get_start_input_at_zero(&self) -> bool {
        
        todo!();
        /*
            return mState.mStartInputAtZero;
        */
    }

    /**
       | bypassing is intended for a narrow special
       | case. the upstream sample time will always
       | be the same as the downstream time.
      */
    pub fn set_bypassed(&mut self, b: bool)  {
        
        todo!();
        /*
            mState.mBypassed = b;
        */
    }
    
    pub fn get_bypassed(&self) -> bool {
        
        todo!();
        /*
            return mState.mBypassed;
        */
    }

    /**
       Call this to reset the timeline.
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mState.mCurrentInputTime.mSampleTime = 0.;
            mState.mNextInputSampleTime = 0.;
            mState.mCurrentOutputTime.mSampleTime = 0.;
            mState.mNextOutputSampleTime = 0.;
            mState.mLastOutputTime.mFlags = 0;
            mState.mRateScalarAdj = 1.;

            mFirstTime = true;
    #if DEBUG
            if (mVerbosity)
                printf("%-20.20s: Reset\n", mDebugName);
    #endif
        */
    }

    /**
      | Call this once per render cycle with the
      | downstream timestamp.  expectedDeltaFrames
      | is the expected difference between the
      | current and NEXT downstream timestamps.
      |
      | sampleRate is the OUTPUT sample rate.
      */
    pub fn add_output_time(
        &mut self, 
        in_time_stamp:         &AudioTimeStamp,
        expected_delta_frames: f64,
        output_sample_rate:    f64,
        rate_scalar_adj:       Option<f64>

    ) {

        let rate_scalar_adj: f64 = rate_scalar_adj.unwrap_or(1.0);

        todo!();
        /*
        
        */
    }

    /**
      | Call this once per render cycle to obtain
      | the upstream timestamp.  framesToAdvance is
      | the number of frames the input timeline is
      | to be advanced during this render cycle.
      |
      | sampleRate is the INPUT sample rate.
      */
    pub fn generate_input_time(
        &mut self, 
        frames_to_advance: f64,
        input_sample_rate: f64,
        advance_host_time: Option<bool>

    ) -> &AudioTimeStamp {

        let advance_host_time: bool = advance_host_time.unwrap_or(false);

        todo!();
        /*
        
        */
    }

    /**
      | this can be called to override the setting
      | of the next input sample time in
      | GenerateInputTime
      */
    pub fn advance(&mut self, frames_to_advance: f64)  {
        
        todo!();
        /*
            #if DEBUG
            if (mVerbosity > 1)
                printf("%-20.20s:   ADVANCE         in = " TSGFMT "                    advance = " TSGFMT "\n", mDebugName, (SInt64)mState.mCurrentInputTime.mSampleTime, (SInt64)framesToAdvance);
    #endif
            mState.mNextInputSampleTime = mState.mCurrentInputTime.mSampleTime + framesToAdvance;
        */
    }
    
    pub fn get_state(&self, out_state: &mut AUTimestampGeneratorState)  {
        
        todo!();
        /*
            outState = mState;
        */
    }
    
    pub fn set_state(&mut self, in_state: &AUTimestampGeneratorState)  {
        
        todo!();
        /*
            mState = inState; mFirstTime = false;
        */
    }
}
