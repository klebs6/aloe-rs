crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/LinearResampler.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/LinearResampler.cpp]

/**
  | Simple resampler that uses bi-linear
  | interpolation.
  |
  */
pub struct LinearResampler<'a> {
    base:           MultiChannelResampler,
    previous_frame: Box<&'a [f32]>,
    current_frame:  Box<&'a [f32]>,
}

impl<'a> LinearResampler<'a> {

    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*


            : MultiChannelResampler(builder) 
        mPreviousFrame = std::make_unique<float[]>(getChannelCount());
        mCurrentFrame = std::make_unique<float[]>(getChannelCount());
        */
    }
    
    pub fn write_frame(&mut self, frame: *const f32)  {
        
        todo!();
        /*
            memcpy(mPreviousFrame.get(), mCurrentFrame.get(), sizeof(float) * getChannelCount());
        memcpy(mCurrentFrame.get(), frame, sizeof(float) * getChannelCount());
        */
    }
    
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            float *previous = mPreviousFrame.get();
        float *current = mCurrentFrame.get();
        float phase = (float) getIntegerPhase() / mDenominator;
        // iterate across samples in the frame
        for (int channel = 0; channel < getChannelCount(); channel++) {
            float f0 = *previous++;
            float f1 = *current++;
            *frame++ = f0 + (phase * (f1 - f0));
        }
        */
    }
}

