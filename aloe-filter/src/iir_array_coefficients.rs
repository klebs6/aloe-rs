crate::ix!();

/**
  | A set of coefficients for use in an Filter
  | object.
  | 
  | @tags{DSP}
  |
  */
pub struct IIRArrayCoefficients<NumericType> {
    _p0: PhantomData<NumericType>,
}

impl<NumericType> IIRArrayCoefficients<NumericType> 
where NumericType: From<f32>
{

    /**
      | Returns the coefficients for a first
      | order low-pass filter.
      |
      */
    pub fn make_first_order_low_pass(
        &mut self, 
        sample_rate: f64,
        frequency:   NumericType

    ) -> [NumericType; 4] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));

        auto n = std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));

        return { { n, n, n + 1, n - 1 } };
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

    ) -> [NumericType; 4] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));

        auto n = std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));

        return { { 1, -1, n + 1, n - 1 } };
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

    ) -> [NumericType; 4] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));

        auto n = std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));

        return { { n - 1, n + 1, n + 1, n - 1 } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            return makeLowPass (sampleRate, frequency, inverseRootTwo);
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));
        jassert (Q > 0.0);

        auto n = 1 / std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));
        auto nSquared = n * n;
        auto invQ = 1 / Q;
        auto c1 = 1 / (1 + invQ * n + nSquared);

        return { { c1, c1 * 2, c1,
                   1, c1 * 2 * (1 - nSquared),
                   c1 * (1 - invQ * n + nSquared) } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            return makeHighPass (sampleRate, frequency, inverseRootTwo);
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));
        jassert (Q > 0.0);

        auto n = std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));
        auto nSquared = n * n;
        auto invQ = 1 / Q;
        auto c1 = 1 / (1 + invQ * n + nSquared);

        return { { c1, c1 * -2, c1,
                   1, c1 * 2 * (nSquared - 1),
                   c1 * (1 - invQ * n + nSquared) } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            return makeBandPass (sampleRate, frequency, inverseRootTwo);
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));
        jassert (Q > 0.0);

        auto n = 1 / std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));
        auto nSquared = n * n;
        auto invQ = 1 / Q;
        auto c1 = 1 / (1 + invQ * n + nSquared);

        return { { c1 * n * invQ, 0,
                   -c1 * n * invQ, 1,
                   c1 * 2 * (1 - nSquared),
                   c1 * (1 - invQ * n + nSquared) } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            return makeNotch (sampleRate, frequency, inverseRootTwo);
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (frequency > 0 && frequency <= static_cast<float> (sampleRate * 0.5));
        jassert (Q > 0.0);

        auto n = 1 / std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));
        auto nSquared = n * n;
        auto invQ = 1 / Q;
        auto c1 = 1 / (1 + n * invQ + nSquared);
        auto b0 = c1 * (1 + nSquared);
        auto b1 = 2 * c1 * (1 - nSquared);

        return { { b0, b1, b0, 1, b1, c1 * (1 - n * invQ + nSquared) } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            return makeAllPass (sampleRate, frequency, inverseRootTwo);
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (Q > 0);

        auto n = 1 / std::tan (MathConstants<NumericType>::pi * frequency / static_cast<NumericType> (sampleRate));
        auto nSquared = n * n;
        auto invQ = 1 / Q;
        auto c1 = 1 / (1 + invQ * n + nSquared);
        auto b0 = c1 * (1 - n * invQ + nSquared);
        auto b1 = c1 * 2 * (1 - nSquared);

        return { { b0, b1, 1, 1, b1, b0 } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0.0);
        jassert (cutOffFrequency > 0.0 && cutOffFrequency <= sampleRate * 0.5);
        jassert (Q > 0.0);

        auto A = jmax (static_cast<NumericType> (0.0), std::sqrt (gainFactor));
        auto aminus1 = A - 1;
        auto aplus1 = A + 1;
        auto omega = (2 * MathConstants<NumericType>::pi * jmax (cutOffFrequency, static_cast<NumericType> (2.0))) / static_cast<NumericType> (sampleRate);
        auto coso = std::cos (omega);
        auto beta = std::sin (omega) * std::sqrt (A) / Q;
        auto aminus1TimesCoso = aminus1 * coso;

        return { { A * (aplus1 - aminus1TimesCoso + beta),
                   A * 2 * (aminus1 - aplus1 * coso),
                   A * (aplus1 - aminus1TimesCoso - beta),
                   aplus1 + aminus1TimesCoso + beta,
                   -2 * (aminus1 + aplus1 * coso),
                   aplus1 + aminus1TimesCoso - beta } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (cutOffFrequency > 0 && cutOffFrequency <= static_cast<NumericType> (sampleRate * 0.5));
        jassert (Q > 0);

        auto A = jmax (static_cast<NumericType> (0.0), std::sqrt (gainFactor));
        auto aminus1 = A - 1;
        auto aplus1 = A + 1;
        auto omega = (2 * MathConstants<NumericType>::pi * jmax (cutOffFrequency, static_cast<NumericType> (2.0))) / static_cast<NumericType> (sampleRate);
        auto coso = std::cos (omega);
        auto beta = std::sin (omega) * std::sqrt (A) / Q;
        auto aminus1TimesCoso = aminus1 * coso;

        return { { A * (aplus1 + aminus1TimesCoso + beta),
                   A * -2 * (aminus1 + aplus1 * coso),
                   A * (aplus1 + aminus1TimesCoso - beta),
                   aplus1 - aminus1TimesCoso + beta,
                   2 * (aminus1 - aplus1 * coso),
                   aplus1 - aminus1TimesCoso - beta } };
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

    ) -> [NumericType; 6] {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= static_cast<NumericType> (sampleRate * 0.5));
        jassert (Q > 0);
        jassert (gainFactor > 0);

        auto A = jmax (static_cast<NumericType> (0.0), std::sqrt (gainFactor));
        auto omega = (2 * MathConstants<NumericType>::pi * jmax (frequency, static_cast<NumericType> (2.0))) / static_cast<NumericType> (sampleRate);
        auto alpha = std::sin (omega) / (Q * 2);
        auto c2 = -2 * std::cos (omega);
        auto alphaTimesA = alpha * A;
        auto alphaOverA = alpha / A;

        return { { 1 + alphaTimesA, c2, 1 - alphaTimesA, 1 + alphaOverA, c2, 1 - alphaOverA } };
        */
    }
}
