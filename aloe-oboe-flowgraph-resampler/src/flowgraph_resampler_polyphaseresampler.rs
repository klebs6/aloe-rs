crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResampler.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/PolyphaseResampler.cpp]

/**
  | Resampler that is optimized for a reduced
  | ratio of sample rates.
  | 
  | All of the coefficients for each possible
  | phase value are pre-calculated.
  |
  */
pub struct PolyphaseResampler {
    base:               MultiChannelResampler,
    coefficient_cursor: i32, // default = 0
}

impl PolyphaseResampler {

    /**
      | @param builder
      | 
      | containing lots of parameters
      |
      */
    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*


            : MultiChannelResampler(builder)

        assert((getNumTaps() % 4) == 0); // Required for loop unrolling.

        int32_t inputRate = builder.getInputRate();
        int32_t outputRate = builder.getOutputRate();

        int32_t numRows = mDenominator;
        double phaseIncrement = (double) inputRate / (double) outputRate;
        generateCoefficients(inputRate, outputRate,
                             numRows, phaseIncrement,
                             builder.getNormalizedCutoff());
        */
    }
    
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            // Clear accumulator for mixing.
        std::fill(mSingleFrame.begin(), mSingleFrame.end(), 0.0);

    //    printf("PolyphaseResampler: mCoefficientCursor = %4d\n", mCoefficientCursor);
        // Multiply input times windowed sinc function.
        float *coefficients = &mCoefficients[mCoefficientCursor];
        float *xFrame = &mX[mCursor * getChannelCount()];
        for (int i = 0; i < mNumTaps; i++) {
            float coefficient = *coefficients++;
    //        printf("PolyphaseResampler: coeff = %10.6f, xFrame[0] = %10.6f\n", coefficient, xFrame[0]);
            for (int channel = 0; channel < getChannelCount(); channel++) {
                mSingleFrame[channel] += *xFrame++ * coefficient;
            }
        }

        // Advance and wrap through coefficients.
        mCoefficientCursor = (mCoefficientCursor + mNumTaps) % mCoefficients.size();

        // Copy accumulator to output.
        for (int channel = 0; channel < getChannelCount(); channel++) {
            frame[channel] = mSingleFrame[channel];
        }
        */
    }
}
