crate::ix!();

/**
  | A set of coefficients for use in an IIRFilter
  | object.
  | 
  | @see IIRFilter
  | 
  | @tags{Audio}
  |
  */
pub struct IIRCoefficientsF32 {

    /**
      | The raw coefficients. You should leave
      | these numbers alone unless you really
      | know what you're doing.
      |
      */
    coefficients: [f32; 5],
}

impl Default for IIRCoefficientsF32 {
    
    /**
      | Creates a null set of coefficients (which
      | will produce silence).
      |
      */
    fn default() -> Self {

        todo!();

        /*
            zeromem (coefficients, sizeof (coefficients));
        */
    }
}

/**
  | A set of coefficients for use in an Filter
  | object. @see IIR::Filter
  | 
  | @tags{DSP}
  |
  */
pub struct IIRCoefficients<NumericType> {

    /**
      | The raw coefficients.
      | 
      | You should leave these numbers alone
      | unless you really know what you're doing.
      |
      */
    coefficients: Vec<NumericType>,
}

impl<NumericType> HasIIRCoefficients for IIRCoefficients<NumericType> {
    type Coefficients = Self;
}

impl<NumericType> ProcessorState for IIRCoefficients<NumericType> {}

pub trait HasArrayCoeffs {
    type ArrayCoeffs;
}

impl<NumericType> HasArrayCoeffs for IIRCoefficients<NumericType> {

    type ArrayCoeffs = IIRArrayCoefficients<NumericType>;
}

impl<NumericType> HasElementType for IIRCoefficients<NumericType> {
    type Type = NumericType;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_IIRFilter.cpp]

impl<NumericType> Default for IIRCoefficients<NumericType> {
    
    /**
      | Creates a null set of coefficients (which
      | will produce silence).
      |
      */
    fn default() -> Self {

        todo!();

        /*
            assign ({ NumericType(), NumericType(), NumericType(),
                  NumericType(), NumericType(), NumericType() })
        */
    }
}

impl<NumericType> IIRCoefficients<NumericType> {

    /**
      | Constructs from an array.
      |
      */
    pub fn new_from_array<const LEN: usize>(values: &[NumericType; LEN]) -> Self {
    
        todo!();
        /*

            assignImpl<LEN> (values.data());
        */
    }

    /**
      | Assigns contents from an array.
      |
      */
    pub fn assign_from_array<const LEN: usize>(
        &mut self, 
        values: &[NumericType; LEN]

    ) -> &mut IIRCoefficients<NumericType> {
    
        todo!();
        /*
            return assignImpl<LEN> (values.data());
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
    
    pub fn assign<const LEN: usize>(&mut self, values: &[NumericType; LEN]) -> &mut IIRCoefficients<NumericType> {
    
        todo!();
        /*
            return assignImpl<LEN> (values);
        */
    }

    pub fn assign_impl<const LEN: usize>(&mut self, values: *const NumericType) -> &mut IIRCoefficients<NumericType> {
    
        todo!();
        /*
            static_assert (LEN % 2 == 0, "Must supply an even number of coefficients");
        const auto a0Index = LEN / 2;
        const auto a0 = values[a0Index];
        const auto a0Inv = a0 != NumericType() ? static_cast<NumericType> (1) / values[a0Index]
                                               : NumericType();

        coefficients.clearQuick();
        coefficients.ensureStorageAllocated ((int) jmax ((size_t) 8, LEN));

        for (size_t i = 0; i < LEN; ++i)
            if (i != a0Index)
                coefficients.add (values[i] * a0Inv);

        return *this;
        */
    }
    
    /**
      | Creates a copy of another filter.
      |
      */
    pub fn new_from_other(other: &IIRCoefficients<NumericType>) -> Self {
    
        todo!();
        /*


            memcpy (coefficients, other.coefficients, sizeof (coefficients));
        */
    }
    
    /**
      | Creates a copy of another filter.
      |
      */
    pub fn assign_from(&mut self, other: &IIRCoefficients<NumericType>) -> &mut IIRCoefficients<NumericType> {
        
        todo!();
        /*
            memcpy (coefficients, other.coefficients, sizeof (coefficients));
        return *this;
        */
    }
    
    /**
      | Directly constructs an object from
      | the raw coefficients. Most people will
      | want to use the static methods instead
      | of this, but the constructor is public
      | to allow tinkerers to create their own
      | custom filters!
      |
      */
    pub fn new_direct(
        c1: f64,
        c2: f64,
        c3: f64,
        c4: f64,
        c5: f64,
        c6: f64

    ) -> Self {
    
        todo!();
        /*


            auto a = 1.0 / c4;

        coefficients[0] = (float) (c1 * a);
        coefficients[1] = (float) (c2 * a);
        coefficients[2] = (float) (c3 * a);
        coefficients[3] = (float) (c5 * a);
        coefficients[4] = (float) (c6 * a);
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | filter.
      |
      */
    pub fn make_low_pass_f64(
        &mut self, 
        sample_rate: f64,
        frequency:   f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return makeLowPass (sampleRate, frequency, 1.0 / MathConstants<double>::sqrt2);
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | filter with variable Q.
      |
      */
    pub fn make_low_pass_f64_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto n = 1.0 / std::tan (MathConstants<double>::pi * frequency / sampleRate);
        auto nSquared = n * n;
        auto c1 = 1.0 / (1.0 + 1.0 / Q * n + nSquared);

        return IIRCoefficients (c1,
                                c1 * 2.0,
                                c1,
                                1.0,
                                c1 * 2.0 * (1.0 - nSquared),
                                c1 * (1.0 - 1.0 / Q * n + nSquared));
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | filter.
      |
      */
    pub fn make_high_pass_f64(
        &mut self, 
        sample_rate: f64,
        frequency:   f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return makeHighPass (sampleRate, frequency, 1.0 / std::sqrt(2.0));
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | filter with variable Q.
      |
      */
    pub fn make_high_pass_f64_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto n = std::tan (MathConstants<double>::pi * frequency / sampleRate);
        auto nSquared = n * n;
        auto c1 = 1.0 / (1.0 + 1.0 / Q * n + nSquared);

        return IIRCoefficients (c1,
                                c1 * -2.0,
                                c1,
                                1.0,
                                c1 * 2.0 * (nSquared - 1.0),
                                c1 * (1.0 - 1.0 / Q * n + nSquared));
        */
    }
    
    /**
      | Returns the coefficients for a band-pass
      | filter.
      |
      */
    pub fn make_band_pass_f64(
        &mut self, 
        sample_rate: f64,
        frequency:   f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return makeBandPass (sampleRate, frequency, 1.0 / MathConstants<double>::sqrt2);
        */
    }
    
    /**
      | Returns the coefficients for a band-pass
      | filter with variable Q.
      |
      */
    pub fn make_band_pass_f64_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto n = 1.0 / std::tan (MathConstants<double>::pi * frequency / sampleRate);
        auto nSquared = n * n;
        auto c1 = 1.0 / (1.0 + 1.0 / Q * n + nSquared);

        return IIRCoefficients (c1 * n / Q,
                                0.0,
                                -c1 * n / Q,
                                1.0,
                                c1 * 2.0 * (1.0 - nSquared),
                                c1 * (1.0 - 1.0 / Q * n + nSquared));
        */
    }
    
    /**
      | Returns the coefficients for a notch
      | filter.
      |
      */
    pub fn make_notch_filter(
        &mut self, 
        sample_rate: f64,
        frequency:   f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return makeNotchFilter (sampleRate, frequency, 1.0 / MathConstants<double>::sqrt2);
        */
    }
    
    /**
      | Returns the coefficients for a notch
      | filter with variable Q.
      |
      */
    pub fn make_notch_filter_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto n = 1.0 / std::tan (MathConstants<double>::pi * frequency / sampleRate);
        auto nSquared = n * n;
        auto c1 = 1.0 / (1.0 + n / Q + nSquared);

        return IIRCoefficients (c1 * (1.0 + nSquared),
                                2.0 * c1 * (1.0 - nSquared),
                                c1 * (1.0 + nSquared),
                                1.0,
                                c1 * 2.0 * (1.0 - nSquared),
                                c1 * (1.0 - n / Q + nSquared));
        */
    }
    
    /**
      | Returns the coefficients for an all-pass
      | filter.
      |
      */
    pub fn make_all_pass_f64(
        &mut self, 
        sample_rate: f64,
        frequency:   f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return makeAllPass (sampleRate, frequency, 1.0 / MathConstants<double>::sqrt2);
        */
    }
    
    /**
      | Returns the coefficients for an all-pass
      | filter with variable Q.
      |
      */
    pub fn make_all_pass_f64_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto n = 1.0 / std::tan (MathConstants<double>::pi * frequency / sampleRate);
        auto nSquared = n * n;
        auto c1 = 1.0 / (1.0 + 1.0 / Q * n + nSquared);

        return IIRCoefficients (c1 * (1.0 - n / Q + nSquared),
                                c1 * 2.0 * (1.0 - nSquared),
                                1.0,
                                1.0,
                                c1 * 2.0 * (1.0 - nSquared),
                                c1 * (1.0 - n / Q + nSquared));
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | shelf filter with variable Q and gain.
      | 
      | The gain is a scale factor that the low
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the low frequencies,
      | values less than 1.0 will attenuate
      | them.
      |
      */
    pub fn make_low_shelf_f64(
        &mut self, 
        sample_rate:       f64,
        cut_off_frequency: f64,
        q:                 f64,
        gain_factor:       f32

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (cutOffFrequency > 0.0 && cutOffFrequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto A = jmax (0.0f, std::sqrt (gainFactor));
        auto aminus1 = A - 1.0;
        auto aplus1 = A + 1.0;
        auto omega = (MathConstants<double>::twoPi * jmax (cutOffFrequency, 2.0)) / sampleRate;
        auto coso = std::cos (omega);
        auto beta = std::sin (omega) * std::sqrt (A) / Q;
        auto aminus1TimesCoso = aminus1 * coso;

        return IIRCoefficients (A * (aplus1 - aminus1TimesCoso + beta),
                                A * 2.0 * (aminus1 - aplus1 * coso),
                                A * (aplus1 - aminus1TimesCoso - beta),
                                aplus1 + aminus1TimesCoso + beta,
                                -2.0 * (aminus1 + aplus1 * coso),
                                aplus1 + aminus1TimesCoso - beta);
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | shelf filter with variable Q and gain.
      | 
      | The gain is a scale factor that the high
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the high
      | frequencies, values less than 1.0 will
      | attenuate them.
      |
      */
    pub fn make_high_shelf_f64(
        &mut self, 
        sample_rate:       f64,
        cut_off_frequency: f64,
        q:                 f64,
        gain_factor:       f32

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (cutOffFrequency > 0.0 && cutOffFrequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto A = jmax (0.0f, std::sqrt (gainFactor));
        auto aminus1 = A - 1.0;
        auto aplus1 = A + 1.0;
        auto omega = (MathConstants<double>::twoPi * jmax (cutOffFrequency, 2.0)) / sampleRate;
        auto coso = std::cos (omega);
        auto beta = std::sin (omega) * std::sqrt (A) / Q;
        auto aminus1TimesCoso = aminus1 * coso;

        return IIRCoefficients (A * (aplus1 + aminus1TimesCoso + beta),
                                A * -2.0 * (aminus1 + aplus1 * coso),
                                A * (aplus1 + aminus1TimesCoso - beta),
                                aplus1 - aminus1TimesCoso + beta,
                                2.0 * (aminus1 - aplus1 * coso),
                                aplus1 - aminus1TimesCoso - beta);
        */
    }
    
    /**
      | Returns the coefficients for a peak
      | filter centred around a given frequency,
      | with a variable Q and gain.
      | 
      | The gain is a scale factor that the centre
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the centre
      | frequencies, values less than 1.0 will
      | attenuate them.
      |
      */
    pub fn make_peak_filter_f64(
        &mut self, 
        sample_rate: f64,
        frequency:   f64,
        q:           f64,
        gain_factor: f32

    ) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0.0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto A = jmax (0.0f, std::sqrt (gainFactor));
        auto omega = (MathConstants<double>::twoPi * jmax (frequency, 2.0)) / sampleRate;
        auto alpha = 0.5 * std::sin (omega) / Q;
        auto c2 = -2.0 * std::cos (omega);
        auto alphaTimesA = alpha * A;
        auto alphaOverA = alpha / A;

        return IIRCoefficients (1.0 + alphaTimesA,
                                c2,
                                1.0 - alphaTimesA,
                                1.0 + alphaOverA,
                                c2,
                                1.0 - alphaOverA);
        */
    }

    /**
      | Directly constructs an object from
      | the raw coefficients.
      | 
      | Most people will want to use the static
      | methods instead of this, but the constructor
      | is public to allow tinkerers to create
      | their own custom filters!
      |
      */
    pub fn new_from_raw2(
        b0: NumericType,
        b1: NumericType,
        a0: NumericType,
        a1: NumericType

    ) -> Self {
    
        todo!();
        /*
            assign ({ b0, b1,
                  a0, a1 });
        */
    }
    
    pub fn new_from_raw3(
        b0: NumericType,
        b1: NumericType,
        b2: NumericType,
        a0: NumericType,
        a1: NumericType,
        a2: NumericType

    ) -> Self {
    
        todo!();
        /*
            assign ({ b0, b1, b2,
                  a0, a1, a2 });
        */
    }
    
    pub fn new_from_raw4(
        b0: NumericType,
        b1: NumericType,
        b2: NumericType,
        b3: NumericType,
        a0: NumericType,
        a1: NumericType,
        a2: NumericType,
        a3: NumericType

    ) -> Self {
    
        todo!();
        /*
            assign ({ b0, b1, b2, b3,
                  a0, a1, a2, a3 });
        */
    }
    
    /**
      | Returns the coefficients for a first
      | order low-pass filter.
      |
      */
    pub fn make_first_order_low_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeFirstOrderLowPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a first
      | order high-pass filter.
      |
      */
    pub fn make_first_order_high_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeFirstOrderHighPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a first
      | order all-pass filter.
      |
      */
    pub fn make_first_order_all_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeFirstOrderAllPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | filter.
      |
      */
    pub fn make_low_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeLowPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | filter with variable Q.
      |
      */
    pub fn make_low_pass_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeLowPass (sampleRate, frequency, Q));
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | filter.
      |
      */
    pub fn make_high_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeHighPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | filter with variable Q.
      |
      */
    pub fn make_high_pass_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeHighPass (sampleRate, frequency, Q));
        */
    }
    
    /**
      | Returns the coefficients for a band-pass
      | filter.
      |
      */
    pub fn make_band_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeBandPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a band-pass
      | filter with variable Q.
      |
      */
    pub fn make_band_pass_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeBandPass (sampleRate, frequency, Q));
        */
    }
    
    /**
      | Returns the coefficients for a notch
      | filter.
      |
      */
    pub fn make_notch(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeNotch (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for a notch
      | filter with variable Q.
      |
      */
    pub fn make_notch_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeNotch (sampleRate, frequency, Q));
        */
    }
    
    /**
      | Returns the coefficients for an all-pass
      | filter.
      |
      */
    pub fn make_all_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeAllPass (sampleRate, frequency));
        */
    }
    
    /**
      | Returns the coefficients for an all-pass
      | filter with variable Q.
      |
      */
    pub fn make_all_pass_with_q(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeAllPass (sampleRate, frequency, Q));
        */
    }
    
    /**
      | Returns the coefficients for a low-pass
      | shelf filter with variable Q and gain.
      | 
      | The gain is a scale factor that the low
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the low frequencies,
      | values less than 1.0 will attenuate
      | them.
      |
      */
    pub fn make_low_shelf(
        &mut self, 
        sample_rate:       f64,
        cut_off_frequency: NumericType,
        q:                 NumericType,
        gain_factor:       NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeLowShelf (sampleRate, cutOffFrequency, Q, gainFactor));
        */
    }
    
    /**
      | Returns the coefficients for a high-pass
      | shelf filter with variable Q and gain.
      | 
      | The gain is a scale factor that the high
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the high
      | frequencies, values less than 1.0 will
      | attenuate them.
      |
      */
    pub fn make_high_shelf(
        &mut self, 
        sample_rate:       f64,
        cut_off_frequency: NumericType,
        q:                 NumericType,
        gain_factor:       NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makeHighShelf (sampleRate, cutOffFrequency, Q, gainFactor));
        */
    }
    
    /**
      | Returns the coefficients for a peak
      | filter centred around a given frequency,
      | with a variable Q and gain.
      | 
      | The gain is a scale factor that the centre
      | frequencies are multiplied by, so values
      | greater than 1.0 will boost the centre
      | frequencies, values less than 1.0 will
      | attenuate them.
      |
      */
    pub fn make_peak_filter(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType,
        q:           NumericType,
        gain_factor: NumericType

    ) -> IIRCoefficientsPtr<Self> {
        
        todo!();
        /*
            return *new IIRCoefficients (ArrayCoeffs::makePeakFilter (sampleRate, frequency, Q, gainFactor));
        */
    }
    
    /**
      | Returns the filter order associated
      | with the coefficients
      |
      */
    pub fn get_filter_order(&self) -> usize {
        
        todo!();
        /*
            return (static_cast<size_t> (coefficients.size()) - 1) / 2;
        */
    }
    
    /**
      | Returns the magnitude frequency response
      | of the filter for a given frequency and
      | sample rate
      |
      */
    pub fn get_magnitude_for_frequency(
        &self, 
        frequency:   f64,
        sample_rate: f64

    ) -> f64 {
        
        todo!();
        /*
            constexpr Complex<double> j (0, 1);
        const auto order = getFilterOrder();
        const auto* coefs = coefficients.begin();

        jassert (frequency >= 0 && frequency <= sampleRate * 0.5);

        Complex<double> numerator = 0.0, denominator = 0.0, factor = 1.0;
        Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequency * j / sampleRate);

        for (size_t n = 0; n <= order; ++n)
        {
            numerator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        denominator = 1.0;
        factor = jw;

        for (size_t n = order + 1; n <= 2 * order; ++n)
        {
            denominator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        return std::abs (numerator / denominator);
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
        frequencies: *const f64,
        magnitudes:  *mut f64,
        num_samples: usize,
        sample_rate: f64

    )  {
        
        todo!();
        /*
            constexpr Complex<double> j (0, 1);
        const auto order = getFilterOrder();
        const auto* coefs = coefficients.begin();

        jassert (order >= 0);

        for (size_t i = 0; i < numSamples; ++i)
        {
            jassert (frequencies[i] >= 0 && frequencies[i] <= sampleRate * 0.5);

            Complex<double> numerator = 0.0, denominator = 0.0, factor = 1.0;
            Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequencies[i] * j / sampleRate);

            for (size_t n = 0; n <= order; ++n)
            {
                numerator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            denominator = 1.0;
            factor = jw;

            for (size_t n = order + 1; n <= 2 * order; ++n)
            {
                denominator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            magnitudes[i] = std::abs(numerator / denominator);
        }
        */
    }
    
    /**
      | Returns the phase frequency response
      | of the filter for a given frequency and
      | sample rate
      |
      */
    pub fn get_phase_for_frequency(
        &self, 
        frequency:   f64,
        sample_rate: f64

    ) -> f64 {
        
        todo!();
        /*
            constexpr Complex<double> j (0, 1);
        const auto order = getFilterOrder();
        const auto* coefs = coefficients.begin();

        jassert (frequency >= 0 && frequency <= sampleRate * 0.5);

        Complex<double> numerator = 0.0, denominator = 0.0, factor = 1.0;
        Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequency * j / sampleRate);

        for (size_t n = 0; n <= order; ++n)
        {
            numerator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        denominator = 1.0;
        factor = jw;

        for (size_t n = order + 1; n <= 2 * order; ++n)
        {
            denominator += static_cast<double> (coefs[n]) * factor;
            factor *= jw;
        }

        return std::arg (numerator / denominator);
        */
    }
    
    /**
      | Returns the phase frequency response
      | of the filter for a given frequency array
      | and sample rate.
      |
      */
    pub fn get_phase_for_frequency_array(
        &self, 
        frequencies: *mut f64,
        phases:      *mut f64,
        num_samples: usize,
        sample_rate: f64

    )  {
        
        todo!();
        /*
            jassert (sampleRate > 0);

        constexpr Complex<double> j (0, 1);
        const auto order = getFilterOrder();
        const auto* coefs = coefficients.begin();
        auto invSampleRate = 1 / sampleRate;

        jassert (order >= 0);

        for (size_t i = 0; i < numSamples; ++i)
        {
            jassert (frequencies[i] >= 0 && frequencies[i] <= sampleRate * 0.5);

            Complex<double> numerator = 0.0, denominator = 0.0, factor = 1.0;
            Complex<double> jw = std::exp (-MathConstants<double>::twoPi * frequencies[i] * j * invSampleRate);

            for (size_t n = 0; n <= order; ++n)
            {
                numerator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            denominator = 1.0;
            factor = jw;

            for (size_t n = order + 1; n <= 2 * order; ++n)
            {
                denominator += static_cast<double> (coefs[n]) * factor;
                factor *= jw;
            }

            phases[i] = std::arg (numerator / denominator);
        }
        */
    }
}
