crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResamplerMono.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResamplerMono.cpp]

pub const MONO: usize = 1;

pub struct PolyphaseResamplerMono {
    base: PolyphaseResampler,
}

impl PolyphaseResamplerMono {

    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*


            : PolyphaseResampler(builder) 

        assert(builder.getChannelCount() == MONO);
        */
    }
    
    pub fn write_frame(&mut self, frame: *const f32)  {
        
        todo!();
        /*
            // Move cursor before write so that cursor points to last written frame in read.
        if (--mCursor < 0) {
            mCursor = getNumTaps() - 1;
        }
        float *dest = &mX[mCursor * MONO];
        const int offset = mNumTaps * MONO;
        // Write each channel twice so we avoid having to wrap when running the FIR.
        const float sample =  frame[0];
        // Put ordered writes together.
        dest[0] = sample;
        dest[offset] = sample;
        */
    }
    
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            // Clear accumulator.
        float sum = 0.0;

        // Multiply input times precomputed windowed sinc function.
        const float *coefficients = &mCoefficients[mCoefficientCursor];
        float *xFrame = &mX[mCursor * MONO];
        const int numLoops = mNumTaps >> 2; // n/4
        for (int i = 0; i < numLoops; i++) {
            // Manual loop unrolling, might get converted to SIMD.
            sum += *xFrame++ * *coefficients++;
            sum += *xFrame++ * *coefficients++;
            sum += *xFrame++ * *coefficients++;
            sum += *xFrame++ * *coefficients++;
        }

        mCoefficientCursor = (mCoefficientCursor + mNumTaps) % mCoefficients.size();

        // Copy accumulator to output.
        frame[0] = sum;
        */
    }
}
