crate::ix!();

pub const kMaxCoefficients:         i32 = 8 * 1024;
pub const kDefaultNormalizedCutoff: f32 = 0.70;

pub trait MultiChannelResamplerInterface {

    /**
      | Read a frame containing N samples using
      | interpolation.
      | 
      | Call advanceRead() after calling this.
      | 
      | -----------
      | @param frame
      | 
      | pointer to the first sample in a frame
      |
      */
    fn read_frame(&mut self, frame: *mut f32);
}

#[repr(i32)]
pub enum MultiChannelResamplerQuality {
    Fastest,
    Low,
    Medium,
    High,
    Best,
}

pub struct MultiChannelSamplerBuilder {
    channel_count:     i32, // default = 1
    num_taps:          i32, // default = 16
    input_rate:        i32, // default = 48000
    output_rate:       i32, // default = 48000
    normalized_cutoff: f32, // default = kDefaultNormalizedCutoff
}

impl MultiChannelSamplerBuilder {

    /**
      | The number of taps in the resampling
      | filter.
      | 
      | More taps gives better MultiChannelResamplerquality but uses
      | more CPU time.
      | 
      | This typically ranges from 4 to 64. Default
      | is 16.
      | 
      | For polyphase filters, numTaps must
      | be a multiple of four for loop unrolling.
      | 
      | -----------
      | @param numTaps
      | 
      | number of taps for the filter
      | 
      | -----------
      | @return
      | 
      | address of this MultiChannelSamplerbuilder for chaining
      | calls
      |
      */
    pub fn set_num_taps(&mut self, num_taps: i32) -> *mut MultiChannelSamplerBuilder {
        
        todo!();
        /*
            mNumTaps = numTaps;
                return this;
        */
    }

    /**
      | Use 1 for mono, 2 for stereo, etc. Default
      | is 1.
      | 
      | -----------
      | @param channelCount
      | 
      | number of channels
      | 
      | -----------
      | @return
      | 
      | address of this MultiChannelSamplerbuilder for chaining
      | calls
      |
      */
    pub fn set_channel_count(&mut self, channel_count: i32) -> *mut MultiChannelSamplerBuilder {
        
        todo!();
        /*
            mChannelCount = channelCount;
                return this;
        */
    }

    /**
      | Default is 48000.
      | 
      | -----------
      | @param inputRate
      | 
      | sample rate of the input stream
      | 
      | -----------
      | @return
      | 
      | address of this MultiChannelSamplerbuilder for chaining
      | calls
      |
      */
    pub fn set_input_rate(&mut self, input_rate: i32) -> *mut MultiChannelSamplerBuilder {
        
        todo!();
        /*
            mInputRate = inputRate;
                return this;
        */
    }

    /**
      | Default is 48000.
      | 
      | -----------
      | @param outputRate
      | 
      | sample rate of the output stream
      | 
      | -----------
      | @return
      | 
      | address of this MultiChannelSamplerbuilder for chaining
      | calls
      |
      */
    pub fn set_output_rate(&mut self, output_rate: i32) -> *mut MultiChannelSamplerBuilder {
        
        todo!();
        /*
            mOutputRate = outputRate;
                return this;
        */
    }

    /**
      | Set cutoff frequency relative to the
      | Nyquist rate of the output sample rate.
      | 
      | Set to 1.0 to match the Nyquist frequency.
      | 
      | Set lower to reduce aliasing.
      | 
      | Default is 0.70.
      | 
      | -----------
      | @param normalizedCutoff
      | 
      | anti-aliasing filter cutoff
      | 
      | -----------
      | @return
      | 
      | address of this MultiChannelSamplerbuilder for chaining
      | calls
      |
      */
    pub fn set_normalized_cutoff(&mut self, normalized_cutoff: f32) -> *mut MultiChannelSamplerBuilder {
        
        todo!();
        /*
            mNormalizedCutoff = normalizedCutoff;
                return this;
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
    
    pub fn get_input_rate(&self) -> i32 {
        
        todo!();
        /*
            return mInputRate;
        */
    }
    
    pub fn get_output_rate(&self) -> i32 {
        
        todo!();
        /*
            return mOutputRate;
        */
    }
    
    pub fn get_normalized_cutoff(&self) -> f32 {
        
        todo!();
        /*
            return mNormalizedCutoff;
        */
    }
    
    /**
      | Construct an optimal resampler based
      | on the specified parameters. @return
      | address of a resampler
      |
      */
    pub fn build(&mut self) -> *mut MultiChannelResampler {
        
        todo!();
        /*
            if (getNumTaps() == 2) {
            // Note that this does not do low pass filteringh.
            return new LinearResampler(*this);
        }
        IntegerRatio ratio(getInputRate(), getOutputRate());
        ratio.reduce();
        bool usePolyphase = (getNumTaps() * ratio.getDenominator()) <= kMaxCoefficients;
        if (usePolyphase) {
            if (getChannelCount() == 1) {
                return new PolyphaseResamplerMono(*this);
            } else if (getChannelCount() == 2) {
                return new PolyphaseResamplerStereo(*this);
            } else {
                return new PolyphaseResampler(*this);
            }
        } else {
            // Use less optimized resampler that uses a float phaseIncrement.
            // TODO mono resampler
            if (getChannelCount() == 2) {
                return new SincResamplerStereo(*this);
            } else {
                return new SincResampler(*this);
            }
        }
        */
    }
}
