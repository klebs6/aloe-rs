crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUScopeElement.h]

/**
  | represents a parameter's value (either
  | constant or ramped)
  |
  */
pub struct ParameterMapEvent {

    event_type:            AUParameterEventType,

    /**
      | ramp start offset relative to start
      | of this slice (may be negative)
      |
      */
    buffer_offset:         i32,

    /**
      | total duration of ramp parameter
      |
      */
    duration_in_frames:    u32,

    /**
      | value if immediate : startValue if ramp
      |
      */
    value1:                AudioUnitParameterValue,

    /**
      | endValue (only used for ramp)
      |
      */
    value2:                AudioUnitParameterValue,

    /**
      | duration of this processing slice
      |
      */
    slice_duration_frames: u32,
}

impl Default for ParameterMapEvent {
    
    fn default() -> Self {
        todo!();
        /*


            : mEventType(kParameterEvent_Immediate), mBufferOffset(0), mDurationInFrames(0), mValue1(0.0f), mValue2(0.0f), mSliceDurationFrames(0)
        */
    }
}

impl ParameterMapEvent {

    pub fn new_from_value(in_value: AudioUnitParameterValue) -> Self {
    
        todo!();
        /*


            : mEventType(kParameterEvent_Immediate), mBufferOffset(0), mDurationInFrames(0), mValue1(inValue), mValue2(inValue), mSliceDurationFrames(0)
        */
    }

    /**
       constructor for scheduled event
      */
    pub fn new(
        in_event:                  &AudioUnitParameterEvent,
        in_slice_offset_in_buffer: u32,
        in_slice_duration_frames:  u32) -> Self {
    
        todo!();
        /*


            SetScheduledEvent(inEvent, inSliceOffsetInBuffer, inSliceDurationFrames );
        }{
        */
    }
    
    pub fn set_scheduled_event(&mut self, 
        in_event:                  &AudioUnitParameterEvent,
        in_slice_offset_in_buffer: u32,
        in_slice_duration_frames:  u32)  {
        
        todo!();
        /*
            mEventType = inEvent.eventType;
            mSliceDurationFrames = inSliceDurationFrames;

            if(mEventType == kParameterEvent_Immediate )
            {
                // constant immediate value for the whole slice
                mValue1 = inEvent.eventValues.immediate.value;
                mValue2 = mValue1;
                mDurationInFrames = inSliceDurationFrames;
                mBufferOffset = 0;
            }
            else
            {
                mDurationInFrames   =   inEvent.eventValues.ramp.durationInFrames;
                mBufferOffset       =   inEvent.eventValues.ramp.startBufferOffset - inSliceOffsetInBuffer; // shift over for this slice
                mValue1             =   inEvent.eventValues.ramp.startValue;
                mValue2             =   inEvent.eventValues.ramp.endValue;
            }
        }{
        */
    }
    
    pub fn get_event_type(&self) -> AUParameterEventType {
        
        todo!();
        /*
            return mEventType;}{
        */
    }
    
    pub fn get_value(&self) -> AudioUnitParameterValue {
        
        todo!();
        /*
            return mValue1;
        */
    }
    
    pub fn get_end_value(&self) -> AudioUnitParameterValue {
        
        todo!();
        /*
            return mValue2;
        */
    }
    
    pub fn set_value(&mut self, in_value: AudioUnitParameterValue)  {
        
        todo!();
        /*
            mEventType = kParameterEvent_Immediate;
                                        mValue1 = inValue;
                                        mValue2 = inValue;
        */
    }

    /**
      | interpolates the start and end values
      | corresponding to the current processing
      | slice most ramp parameter implementations
      | will want to use this method the start
      | value will correspond to the start of the
      | slice the end value will correspond to the
      | end of the slice
      */
    pub fn get_ramp_slice_start_end(&mut self, 
        out_start_value:           &mut AudioUnitParameterValue,
        out_end_value:             &mut AudioUnitParameterValue,
        out_value_per_frame_delta: &mut AudioUnitParameterValue)  {
        
        todo!();
        /*
            if (mEventType == kParameterEvent_Ramped) {
                outValuePerFrameDelta = (mValue2 - mValue1) / mDurationInFrames;

                outStartValue = mValue1 + outValuePerFrameDelta * (-mBufferOffset); // corresponds to frame 0 of this slice
                outEndValue = outStartValue +  outValuePerFrameDelta * mSliceDurationFrames;
            } else {
                outValuePerFrameDelta = 0;
                outStartValue = outEndValue = mValue1;
            }
        }{
        */
    }

    /**
      | Some ramp parameter implementations will
      | want to interpret the ramp using their own
      | interpolation method (perhaps non-linear)
      |
      | This method gives the raw ramp information,
      | relative to this processing slice for the
      | client to interpret as desired
      */
    pub fn get_ramp_info(&mut self, 
        out_buffer_offset:      &mut i32,
        out_duration_in_frames: &mut u32,
        out_start_value:        &mut AudioUnitParameterValue,
        out_end_value:          &mut AudioUnitParameterValue)  {
        
        todo!();
        /*
            outBufferOffset = mBufferOffset;
            outDurationInFrames = mDurationInFrames;
            outStartValue = mValue1;
            outEndValue = mValue2;
        }{
        */
    }

    #[cfg(DEBUG)]
    pub fn print(&mut self)  {
        
        todo!();
        /*
            printf("ParameterEvent @ %p\n", this);
            printf("    mEventType = %d\n", (int)mEventType);
            printf("    mBufferOffset = %d\n", (int)mBufferOffset);
            printf("    mDurationInFrames = %d\n", (int)mDurationInFrames);
            printf("    mSliceDurationFrames = %d\n", (int)mSliceDurationFrames);
            printf("    mValue1 = %.5f\n", mValue1);
            printf("    mValue2 = %.5f\n", mValue2);
        */
    }
}


