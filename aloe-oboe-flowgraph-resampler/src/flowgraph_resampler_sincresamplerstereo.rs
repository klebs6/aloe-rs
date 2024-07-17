crate::ix!();

pub const STEREO: usize = 2;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/SincResamplerStereo.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/SincResamplerStereo.cpp]

pub struct SincResamplerStereo {
    base: SincResampler,
}

impl SincResamplerStereo {

    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*
        : sinc_resampler(builder),

            assert(builder.getChannelCount() == STEREO);
        */
    }
    
    pub fn write_frame(&mut self, frame: *const f32)  {
        
        todo!();
        /*
            // Move cursor before write so that cursor points to last written frame in read.
        if (--mCursor < 0) {
            mCursor = getNumTaps() - 1;
        }
        float *dest = &mX[mCursor * STEREO];
        const int offset = mNumTaps * STEREO;
        // Write each channel twice so we avoid having to wrap when running the FIR.
        const float left =  frame[0];
        const float right = frame[1];
        // Put ordered writes together.
        dest[0] = left;
        dest[1] = right;
        dest[offset] = left;
        dest[1 + offset] = right;
        */
    }

    /**
      | Multiply input times windowed sinc
      | function.
      |
      */
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            // Clear accumulator for mixing.
        std::fill(mSingleFrame.begin(), mSingleFrame.end(), 0.0);
        std::fill(mSingleFrame2.begin(), mSingleFrame2.end(), 0.0);

        // Determine indices into coefficients table.
        double tablePhase = getIntegerPhase() * mPhaseScaler;
        int index1 = static_cast<int>(floor(tablePhase));
        float *coefficients1 = &mCoefficients[index1 * getNumTaps()];
        int index2 = (index1 + 1);
        if (index2 >= mNumRows) { // no guard row needed because we wrap the indices
            index2 = 0;
        }
        float *coefficients2 = &mCoefficients[index2 * getNumTaps()];
        float *xFrame = &mX[mCursor * getChannelCount()];
        for (int i = 0; i < mNumTaps; i++) {
            float coefficient1 = *coefficients1++;
            float coefficient2 = *coefficients2++;
            for (int channel = 0; channel < getChannelCount(); channel++) {
                float sample = *xFrame++;
                mSingleFrame[channel] +=  sample * coefficient1;
                mSingleFrame2[channel] += sample * coefficient2;
            }
        }

        // Interpolate and copy to output.
        float fraction = tablePhase - index1;
        for (int channel = 0; channel < getChannelCount(); channel++) {
            float low = mSingleFrame[channel];
            float high = mSingleFrame2[channel];
            frame[channel] = low + (fraction * (high - low));
        }
        */
    }
}
