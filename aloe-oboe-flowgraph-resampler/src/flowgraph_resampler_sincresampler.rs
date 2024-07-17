crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/SincResampler.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/SincResampler.cpp]

/**
  | Resampler that can interpolate between
  | coefficients.
  | 
  | This can be used to support arbitrary
  | ratios.
  |
  */
pub struct SincResampler {

    base:          MultiChannelResampler,

    /**
      | for interpolation
      |
      */
    single_frame2: Vec<f32>,

    num_rows:      i32, // default = 0
    phase_scaler:  f64, // default = 1.0
}

impl SincResampler {

    pub fn new(builder: &MultiChannelResamplerBuilder) -> Self {
    
        todo!();
        /*


            : MultiChannelResampler(builder)
            , mSingleFrame2(builder.getChannelCount()) 

        assert((getNumTaps() % 4) == 0); // Required for loop unrolling.
        mNumRows = kMaxCoefficients / getNumTaps(); // no guard row needed
    //    printf("SincResampler: numRows = %d\n", mNumRows);
        mPhaseScaler = (double) mNumRows / mDenominator;
        double phaseIncrement = 1.0 / mNumRows;
        generateCoefficients(builder.getInputRate(),
                             builder.getOutputRate(),
                             mNumRows,
                             phaseIncrement,
                             builder.getNormalizedCutoff());
        */
    }
    
    pub fn read_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            // Clear accumulator for mixing.
        std::fill(mSingleFrame.begin(), mSingleFrame.end(), 0.0);
        std::fill(mSingleFrame2.begin(), mSingleFrame2.end(), 0.0);

        // Determine indices into coefficients table.
        double tablePhase = getIntegerPhase() * mPhaseScaler;
        int index1 = static_cast<int>(floor(tablePhase));
        if (index1 >= mNumRows) { // no guard row needed because we wrap the indices
            tablePhase -= mNumRows;
            index1 -= mNumRows;
        }

        int index2 = index1 + 1;
        if (index2 >= mNumRows) { // no guard row needed because we wrap the indices
            index2 -= mNumRows;
        }

        float *coefficients1 = &mCoefficients[index1 * getNumTaps()];
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
