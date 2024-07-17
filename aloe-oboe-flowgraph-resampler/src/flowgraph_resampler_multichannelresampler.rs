crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/MultiChannelResampler.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/MultiChannelResampler.cpp]

/**
  | It appears from the spectrogram that
  | the HyperbolicCosine window leads
  | to fewer artifacts. And it is faster
  | to calculate.
  |
  */
#[cfg(not(MCR_USE_KAISER))]
pub const MCR_USE_KAISER: usize = 0;

pub struct MultiChannelResampler {

    coefficients:  Vec<f32>,
    num_taps:      i32,
    cursor:        i32, // default = 0

    /**
      | delayed input values for the FIR
      |
      */
    mx:            Vec<f32>,

    /**
      | one frame for temporary use
      |
      */
    single_frame:  Vec<f32>,

    integer_phase: i32, // default = 0
    numerator:     i32, // default = 0
    denominator:   i32, // default = 0

    #[cfg(MCR_USE_KAISER)]
    kaiser_window:  KaiserWindow,

    #[cfg(not(MCR_USE_KAISER))]
    cosh_window:    HyperbolicCosineWindow,

    channel_count:  i32,
}

impl MultiChannelResampler {

    pub fn is_write_needed(&self) -> bool {
        
        todo!();
        /*
            return mIntegerPhase >= mDenominator;
        */
    }

    /**
      | Write a frame containing N samples.
      | 
      | -----------
      | @param frame
      | 
      | pointer to the first sample in a frame
      |
      */
    pub fn write_next_frame(&mut self, frame: *const f32)  {
        
        todo!();
        /*
            writeFrame(frame);
            advanceWrite();
        */
    }

    /**
      | Read a frame containing N samples.
      | 
      | -----------
      | @param frame
      | 
      | pointer to the first sample in a frame
      |
      */
    pub fn read_next_frame(&mut self, frame: *mut f32)  {
        
        todo!();
        /*
            readFrame(frame);
            advanceRead();
        */
    }
    
    pub fn get_num_taps(&self) -> i32 {
        
        todo!();
        /*
            return mNumTaps;
        */
    }
    
    pub fn get_channel_count(&self) -> i32 {
        
        todo!();
        /*
            return mChannelCount;
        */
    }
    
    pub fn hamming_window(
        radians: f32,
        spread:  f32) -> f32 {
        
        todo!();
        /*
        
        */
    }
    
    pub fn advance_write(&mut self)  {
        
        todo!();
        /*
            mIntegerPhase -= mDenominator;
        */
    }
    
    pub fn advance_read(&mut self)  {
        
        todo!();
        /*
            mIntegerPhase += mNumerator;
        */
    }

    pub fn get_integer_phase(&mut self) -> i32 {
        
        todo!();
        /*
            return mIntegerPhase;
        */
    }
    
    pub fn new(MultiChannelSamplerbuilder: &MultiChannelSamplerBuilder) -> Self {
    
        todo!();
        /*


            : mNumTaps(MultiChannelSamplerbuilder.getNumTaps())
            , mX(MultiChannelSamplerbuilder.getChannelCount() * MultiChannelSamplerbuilder.getNumTaps() * 2)
            , mSingleFrame(MultiChannelSamplerbuilder.getChannelCount())
            , mChannelCount(MultiChannelSamplerbuilder.getChannelCount())

        // Reduce sample rates to the smallest ratio.
        // For example 44100/48000 would become 147/160.
        IntegerRatio ratio(MultiChannelSamplerbuilder.getInputRate(), MultiChannelSamplerbuilder.getOutputRate());
        ratio.reduce();
        mNumerator = ratio.getNumerator();
        mDenominator = ratio.getDenominator();
        mIntegerPhase = mDenominator;
        */
    }

    /**
      | Factory method for making a resampler
      | that is optimal for the given inputs.
      | 
      | -----------
      | @param channelCount
      | 
      | number of channels, 2 for stereo
      | ----------
      | @param inputRate
      | 
      | sample rate of the input stream
      | ----------
      | @param outputRate
      | 
      | sample rate of the output stream
      | ----------
      | @param MultiChannelResamplerquality
      | 
      | higher MultiChannelResamplerquality sounds better but uses
      | more CPU
      | 
      | -----------
      | @return
      | 
      | an optimal resampler
      |
      */
    pub fn make(&mut self, 
        channel_count: i32,
        input_rate:    i32,
        output_rate:   i32,
        MultiChannelResamplerquality:       MultiChannelResamplerQuality) -> *mut MultiChannelResampler {
        
        todo!();
        /*
            MultiChannelSamplerBuilder MultiChannelSamplerbuilder;
        MultiChannelSamplerbuilder.setInputRate(inputRate);
        MultiChannelSamplerbuilder.setOutputRate(outputRate);
        MultiChannelSamplerbuilder.setChannelCount(channelCount);

        switch (MultiChannelResamplerquality) {
            case MultiChannelResamplerQuality::Fastest:
                MultiChannelSamplerbuilder.setNumTaps(2);
                break;
            case MultiChannelResamplerQuality::Low:
                MultiChannelSamplerbuilder.setNumTaps(4);
                break;
            case MultiChannelResamplerQuality::Medium:
            default:
                MultiChannelSamplerbuilder.setNumTaps(8);
                break;
            case MultiChannelResamplerQuality::High:
                MultiChannelSamplerbuilder.setNumTaps(16);
                break;
            case MultiChannelResamplerQuality::Best:
                MultiChannelSamplerbuilder.setNumTaps(32);
                break;
        }

        // Set the cutoff frequency so that we do not get aliasing when down-sampling.
        if (inputRate > outputRate) {
            MultiChannelSamplerbuilder.setNormalizedCutoff(kDefaultNormalizedCutoff);
        }
        return MultiChannelSamplerbuilder.build();
        */
    }
    
    /**
      | Write a frame containing N samples.
      | 
      | Call advanceWrite() after calling
      | this.
      | 
      | -----------
      | @param frame
      | 
      | pointer to the first sample in a frame
      |
      */
    pub fn write_frame(&mut self, frame: *const f32)  {
        
        todo!();
        /*
            // Move cursor before write so that cursor points to last written frame in read.
        if (--mCursor < 0) {
            mCursor = getNumTaps() - 1;
        }
        float *dest = &mX[mCursor * getChannelCount()];
        int offset = getNumTaps() * getChannelCount();
        for (int channel = 0; channel < getChannelCount(); channel++) {
            // Write twice so we avoid having to wrap when reading.
            dest[channel] = dest[channel + offset] = frame[channel];
        }
        */
    }
    
    pub fn sinc(&mut self, radians: f32) -> f32 {
        
        todo!();
        /*
            if (abs(radians) < 1.0e-9) return 1.0f;   // avoid divide by zero
        return sinf(radians) / radians;   // Sinc function
        */
    }

    /**
      | Generate coefficients in the order they will be
      | used by readFrame().
      |
      | This is more complicated but readFrame() is
      | called repeatedly and should be optimized.
      |
      |____________________
      |
      | Generate the filter coefficients in
      | optimal order.
      | 
      | -----------
      | @param inputRate
      | 
      | sample rate of the input stream
      | ----------
      | @param outputRate
      | 
      | sample rate of the output stream
      | ----------
      | @param numRows
      | 
      | number of rows in the array that contain
      | a set of tap coefficients
      | ----------
      | @param phaseIncrement
      | 
      | how much to increment the phase between
      | rows
      | ----------
      | @param normalizedCutoff
      | 
      | filter cutoff frequency normalized
      | to Nyquist rate of output
      |
      */
    pub fn generate_coefficients(&mut self, 
        input_rate:        i32,
        output_rate:       i32,
        num_rows:          i32,
        phase_increment:   f64,
        normalized_cutoff: f32)  {
        
        todo!();
        /*
            mCoefficients.resize(getNumTaps() * numRows);
        int coefficientIndex = 0;
        double phase = 0.0; // ranges from 0.0 to 1.0, fraction between samples
        // Stretch the sinc function for low pass filtering.
        const float cutoffScaler = normalizedCutoff *
                ((outputRate < inputRate)
                 ? ((float)outputRate / inputRate)
                 : ((float)inputRate / outputRate));
        const int numTapsHalf = getNumTaps() / 2; // numTaps must be even.
        const float numTapsHalfInverse = 1.0f / numTapsHalf;
        for (int i = 0; i < numRows; i++) {
            float tapPhase = phase - numTapsHalf;
            float gain = 0.0; // sum of raw coefficients
            int gainCursor = coefficientIndex;
            for (int tap = 0; tap < getNumTaps(); tap++) {
                float radians = tapPhase * M_PI;

    #if MCR_USE_KAISER
                float window = mKaiserWindow(tapPhase * numTapsHalfInverse);
    #else
                float window = mCoshWindow(tapPhase * numTapsHalfInverse);
    #endif
                float coefficient = sinc(radians * cutoffScaler) * window;
                mCoefficients.at(coefficientIndex++) = coefficient;
                gain += coefficient;
                tapPhase += 1.0;
            }
            phase += phaseIncrement;
            while (phase >= 1.0) {
                phase -= 1.0;
            }

            // Correct for gain variations.
            float gainCorrection = 1.0 / gain; // normalize the gain
            for (int tap = 0; tap < getNumTaps(); tap++) {
                mCoefficients.at(gainCursor + tap) *= gainCorrection;
            }
        }
        */
    }
}
