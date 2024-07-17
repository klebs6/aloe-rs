crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_FFT.h]

/**
  | Performs a fast fourier transform.
  |
  | This is only a simple low-footprint
  | implementation and isn't tuned for speed - it
  | may be useful for simple applications where
  | one of the more complex FFT libraries would be
  | overkill. (But in the future it may end up
  | becoming optimised of course...)
  |
  | The FFT class itself contains lookup tables,
  | so there's some overhead in creating one, you
  | should create and cache an FFT object for each
  | size/direction of transform that you need, and
  | re-use them to perform the actual operation.
  |
  | @tags{DSP}
  */
#[no_copy]
#[leak_detector]
pub struct FFT {
    engine: Box<dyn FftInstance>,
    size:   i32,
}

impl FFT {

    /**
      | Returns the number of data points that
      | this FFT was created to work with.
      |
      */
    pub fn get_size(&self) -> i32 {
        
        todo!();
        /*
            return size;
        */
    }
    
    /**
      | Initialises an object for performing
      | forward and inverse FFT with the given
      | size. The number of points the FFT will
      | operate on will be 2 ^ order.
      |
      */
    pub fn new(order: i32) -> Self {
    
        todo!();
        /*
           : engine (FFT::FftEngine::createBestEngineForPlatform (order)),
           size (1 << order)
        */
    }
    
    /** 
      | Performs an out-of-place FFT, either forward
      | or inverse.  The arrays must contain at least
      | getSize() elements.
      */
    pub fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool)  {
        
        todo!();
        /*
            if (engine != nullptr)
            engine->perform (input, output, inverse);
        */
    }
    
    /** 
      | Performs an in-place forward transform on
      | a block of real data.
      |
      | As the coefficients of the negative
      | frequencies (frequencies higher than N/2
      | or pi) are the complex conjugate of their
      | positive counterparts, it may not be
      | necessary to calculate them for your
      | particular application.  You can use
      | dontCalculateNegativeFrequencies to let
      | the FFT engine know that you do not plan
      | on using them. Note that this is only
      | a hint: some FFT engines (currently only
      | the Fallback engine), will still calculate
      | the negative frequencies even if
      | dontCalculateNegativeFrequencies is true.
      |
      | The size of the array passed in must be
      | 2 * getSize(), and the first half should
      | contain your raw input sample data. On
      | return, if dontCalculateNegativeFrequencies 
      | is false, the array will contain size complex 
      | real + imaginary parts data interleaved. If
      | dontCalculateNegativeFrequencies is true,
      | the array will contain at least 
      | (size / 2) + 1 complex numbers. Both outputs 
      | can be passed to
      | performRealOnlyInverseTransform() in order
      | to convert it back to reals.
      */
    pub fn perform_real_only_forward_transform(
        &self, 
        input_output_data:     *mut f32,
        ignore_negative_freqs: Option<bool>

    ) {

        let ignore_negative_freqs = ignore_negative_freqs.unwrap_or(false);
        
        todo!();
        /*
            if (engine != nullptr)
            engine->performRealOnlyForwardTransform (inputOutputData, ignoreNeagtiveFreqs);
        */
    }
    
    /** 
      | Performs a reverse operation to data created
      | in performRealOnlyForwardTransform().
      |
      | Although performRealOnlyInverseTransform
      | will only use the first ((size / 2) + 1)
      | complex numbers, the size of the array
      | passed in must still be 2 * getSize(), as
      | some FFT engines require the extra space
      | for the calculation. On return, the first
      | half of the array will contain the
      | reconstituted samples.
      */
    pub fn perform_real_only_inverse_transform(&self, input_output_data: *mut f32)  {
        
        todo!();
        /*
            if (engine != nullptr)
            engine->performRealOnlyInverseTransform (inputOutputData);
        */
    }
    
    /** 
      | Takes an array and simply transforms it to
      | the magnitude frequency response
      | spectrum. This may be handy for things like
      | frequency displays or analysis.  The size of
      | the array passed in must be 2 * getSize().
      */
    pub fn perform_frequency_only_forward_transform(&self, input_output_data: *mut f32)  {
        
        todo!();
        /*
            if (size == 1)
            return;

        performRealOnlyForwardTransform (inputOutputData);
        auto* out = reinterpret_cast<Complex<float>*> (inputOutputData);

        for (int i = 0; i < size; ++i)
            inputOutputData[i] = std::abs (out[i]);

        zeromem (&inputOutputData[size], static_cast<size_t> (size) * sizeof (float));
        */
    }
}
