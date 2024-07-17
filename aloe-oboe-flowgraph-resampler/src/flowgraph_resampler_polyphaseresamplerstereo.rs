crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResamplerStereo.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResamplerStereo.cpp]

pub const STEREO: usize = 2;

pub struct PolyphaseResamplerStereo {
    base: PolyphaseResampler,
}

impl PolyphaseResamplerStereo {

    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*
        : polyphase_resampler(builder),

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
    
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            // Clear accumulators.
        float left = 0.0;
        float right = 0.0;

        // Multiply input times precomputed windowed sinc function.
        const float *coefficients = &mCoefficients[mCoefficientCursor];
        float *xFrame = &mX[mCursor * STEREO];
        const int numLoops = mNumTaps >> 2; // n/4
        for (int i = 0; i < numLoops; i++) {
            // Manual loop unrolling, might get converted to SIMD.
            float coefficient = *coefficients++;
            left += *xFrame++ * coefficient;
            right += *xFrame++ * coefficient;

            coefficient = *coefficients++; // next tap
            left += *xFrame++ * coefficient;
            right += *xFrame++ * coefficient;

            coefficient = *coefficients++;  // next tap
            left += *xFrame++ * coefficient;
            right += *xFrame++ * coefficient;

            coefficient = *coefficients++;  // next tap
            left += *xFrame++ * coefficient;
            right += *xFrame++ * coefficient;
        }

        mCoefficientCursor = (mCoefficientCursor + mNumTaps) % mCoefficients.size();

        // Copy accumulators to output.
        frame[0] = left;
        frame[1] = right;
        */
    }
}
