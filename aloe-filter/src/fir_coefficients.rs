crate::ix!();

/**
  | The FIRCoefficients structure is ref-counted,
  | so this is a handy type that can be used
  | as a pointer to one.
  |
  */
pub type FIRCoefficientsPtr<NumericType> = Rc<RefCell<FIRCoefficients<NumericType>>>;

/**
  | A set of coefficients for use in an FIRFilter
  | object.
  | 
  | @see FIRFilter
  | 
  | @tags{DSP}
  |
  */
pub struct FIRCoefficients<NumericType> {

    /**
      | The raw coefficients.
      | 
      | You should leave these numbers alone
      | unless you really know what you're doing.
      |
      */
    coefficients: Vec<NumericType>,
}

impl<NumericType> ProcessorState for FIRCoefficients<NumericType> {}

impl<NumericType> Default for FIRCoefficients<NumericType> {
    
    /**
      | Creates a null set of coefficients (which
      | will produce silence).
      |
      */
    fn default() -> Self {
        todo!();
        /*


            : coefficients ({ NumericType() }
        */
    }
}

impl<NumericType> FIRCoefficients<NumericType> {

    /**
      | Creates a null set of coefficients of
      | a given size.
      |
      */
    pub fn new(size: usize) -> Self {
    
        todo!();
        /*


            coefficients.resize ((int) size);
        */
    }

    /**
      | Creates a set of coefficients from an
      | array of samples.
      |
      */
    pub fn new_with_samples(
        samples:     *const NumericType,
        num_samples: usize) -> Self {
    
        todo!();
        /*


            : coefficients (samples, (int) numSamples)
        */
    }
        
    /**
      | Returns the filter order associated
      | with the coefficients.
      |
      */
    pub fn get_filter_order(&self) -> usize {
        
        todo!();
        /*
            return static_cast<size_t> (coefficients.size()) - 1;
        */
    }

    /**
      | Returns a raw data pointer to the coefficients.
      |
      */
    pub fn get_raw_coefficients_mut(&mut self) -> *mut NumericType {
        
        todo!();
        /*
            return coefficients.getRawDataPointer();
        */
    }

    /**
      | Returns a raw data pointer to the coefficients.
      |
      */
    pub fn get_raw_coefficients(&self) -> *const NumericType {
        
        todo!();
        /*
            return coefficients.begin();
        */
    }

    /**
      | Returns the magnitude frequency response
      | of the filter for a given frequency and
      | sample rate.
      |
      */
    pub fn get_magnitude_for_frequency(
        &self, 
        frequency:       f64,
        the_sample_rate: f64

    ) -> f64 {
    
        todo!();
        /*
            jassert (theSampleRate > 0.0);
        jassert (frequency >= 0.0 && frequency <= theSampleRate * 0.5);

        constexpr Complex<double> j (0, 1);
        auto order = getFilterOrder();

        Complex<double> numerator = 0.0, factor = 1.0;
        Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequency * j / theSampleRate);

        const auto* coefs = coefficients.begin();

        for (size_t n = 0; n <= order; ++n)
        {
            numerator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        return std::abs (numerator);
        */
    }
    
    /**
      | Returns the magnitude frequency response
      | of the filter for a given frequency array
      | and sample rate.
      |
      */
    pub fn get_magnitude_for_frequency_array(
        &self, 
        frequencies:     *mut f64,
        magnitudes:      *mut f64,
        num_samples:     usize,
        the_sample_rate: f64

    ) {
        
        todo!();
        /*
            jassert (theSampleRate > 0.0);

        constexpr Complex<double> j (0, 1);
        const auto* coefs = coefficients.begin();
        auto order = getFilterOrder();

        for (size_t i = 0; i < numSamples; ++i)
        {
            jassert (frequencies[i] >= 0.0 && frequencies[i] <= theSampleRate * 0.5);

            Complex<double> numerator = 0.0;
            Complex<double> factor = 1.0;
            Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequencies[i] * j / theSampleRate);

            for (size_t n = 0; n <= order; ++n)
            {
                numerator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            magnitudes[i] = std::abs (numerator);
        }
        */
    }
    
    /**
      | Returns the phase frequency response
      | of the filter for a given frequency and
      | sample rate.
      |
      */
    pub fn get_phase_for_frequency(&self, 
        frequency:       f64,
        the_sample_rate: f64) -> f64 {
        
        todo!();
        /*
            jassert (theSampleRate > 0.0);
        jassert (frequency >= 0.0 && frequency <= theSampleRate * 0.5);

        constexpr Complex<double> j (0, 1);

        Complex<double> numerator = 0.0;
        Complex<double> factor = 1.0;
        Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequency * j / theSampleRate);

        const auto* coefs = coefficients.begin();
        auto order = getFilterOrder();

        for (size_t n = 0; n <= order; ++n)
        {
            numerator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        return std::arg (numerator);
        */
    }
    
    /**
      | Returns the phase frequency response
      | of the filter for a given frequency array
      | and sample rate.
      |
      */
    pub fn get_phase_for_frequency_array(&self, 
        frequencies:     *mut f64,
        phases:          *mut f64,
        num_samples:     usize,
        the_sample_rate: f64)  {
        
        todo!();
        /*
            jassert (theSampleRate > 0.0);

        constexpr Complex<double> j (0, 1);
        const auto* coefs = coefficients.begin();
        auto order = getFilterOrder();

        for (size_t i = 0; i < numSamples; ++i)
        {
            jassert (frequencies[i] >= 0.0 && frequencies[i] <= theSampleRate * 0.5);

            Complex<double> numerator = 0.0, factor = 1.0;
            Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequencies[i] * j / theSampleRate);

            for (size_t n = 0; n <= order; ++n)
            {
                numerator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            phases[i] = std::arg (numerator);
        }
        */
    }
    
    /**
      | Scales the values of the FIR filter with
      | the sum of the squared coefficients.
      |
      */
    pub fn normalise(&mut self)  {
        
        todo!();
        /*
            auto magnitude = static_cast<NumericType> (0);

        auto* coefs = coefficients.getRawDataPointer();
        auto n = static_cast<size_t> (coefficients.size());

        for (size_t i = 0; i < n; ++i)
        {
            auto c = coefs[i];
            magnitude += c * c;
        }

        auto magnitudeInv = 1 / (4 * std::sqrt (magnitude));

        FloatVectorOperations::multiply (coefs, magnitudeInv, static_cast<int> (n));
        */
    }
}
