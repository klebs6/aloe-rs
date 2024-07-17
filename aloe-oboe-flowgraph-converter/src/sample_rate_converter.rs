crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/SampleRateConverter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/SampleRateConverter.cpp]

pub struct SampleRateConverter<'a> {

    base:                   FlowGraphFilter<'a>,

    resampler:              &'a mut MultiChannelResampler,

    /**
      | offset into the input port buffer
      |
      */
    input_cursor:           i32, // default = 0

    /**
      | number of valid frames currently in
      | the input port buffer
      |
      */
    num_valid_input_frames: i32, // default = 0

    /**
      | We need our own callCount for upstream
      | calls because calls occur at a different
      | rate.
      |
      | This means we cannot have cyclic graphs or
      | merges that contain an SRC.
      */
    input_call_count:       i64, // default = 0
}

impl<'a> SampleRateConverter<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "SampleRateConverter";
        */
    }
    
    pub fn new(
        channel_count: i32,
        resampler:     &mut MultiChannelResampler) -> Self {
    
        todo!();
        /*
        : flow_graph_filter(channelCount),
        : resampler(resampler),

            setDataPulledAutomatically(false);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            FlowGraphNode::reset();
        mInputCursor = kInitialCallCount;
        */
    }

    /**
      | Return true if there is a sample available.
      |
      */
    pub fn is_input_available(&mut self) -> bool {
        
        todo!();
        /*
            // If we have consumed all of the input data then go out and get some more.
        if (mInputCursor >= mNumValidInputFrames) {
            mInputCallCount++;
            mNumValidInputFrames = input.pullData(mInputCallCount, input.getFramesPerBuffer());
            mInputCursor = 0;
        }
        return (mInputCursor < mNumValidInputFrames);
        */
    }
    
    /**
      | This assumes data is available. Only
      | call after calling isInputAvailable().
      |
      */
    pub fn get_next_input_frame(&mut self) -> *const f32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        return &inputBuffer[mInputCursor++ * input.getSamplesPerFrame()];
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            float *outputBuffer = output.getBuffer();
        int32_t channelCount = output.getSamplesPerFrame();
        int framesLeft = numFrames;
        while (framesLeft > 0) {
            // Gather input samples as needed.
            if(mResampler.isWriteNeeded()) {
                if (isInputAvailable()) {
                    const float *frame = getNextInputFrame();
                    mResampler.writeNextFrame(frame);
                } else {
                    break;
                }
            } else {
                // Output frame is interpolated from input samples.
                mResampler.readNextFrame(outputBuffer);
                outputBuffer += channelCount;
                framesLeft--;
            }
        }
        return numFrames - framesLeft;
        */
    }
}
