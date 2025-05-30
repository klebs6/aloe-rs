crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/filter_design/aloe_FilterDesign.h]

/**
  | This class provides a set of functions
  | which generates FIR::Coefficients
  | and IIR::Coefficients, of high-order
  | low-pass filters. They can be used for
  | processing directly audio as an equalizer,
  | in resampling algorithms etc.
  | 
  | see FIRFilter::Coefficients, FIRFilter,
  | WindowingFunction, IIRFilter::Coefficients,
  | IIRFilter
  | 
  | @tags{DSP}
  |
  */
pub struct FilterDesign<FloatType: num::Float> {
    _p0: PhantomData<FloatType>,
}

//-------------------------------------
pub trait HasFIRCoefficients {
    type Coefficients;
}

pub trait HasFIRCoefficientsPtr: HasFIRCoefficients {
    type Ptr = Rc<RefCell<<Self as HasFIRCoefficients>::Coefficients>>;
}

impl<FloatType: num::Float> HasFIRCoefficients for FilterDesign<FloatType> {
    type Coefficients = FIRCoefficients<FloatType>;
}

impl<FloatType: num::Float> HasFIRCoefficientsPtr for FilterDesign<FloatType> {}

//-------------------------------------
pub trait HasIIRCoefficients {
    type Coefficients;
}

pub trait HasIIRCoefficientsPtr: HasIIRCoefficients {
    type Ptr = Rc<RefCell<<Self as HasIIRCoefficients>::Coefficients>>;
}

/**
  | The IIRCoefficients structure is ref-counted,
  | so this is a handy type that can be used
  | as a pointer to one.
  |
  */
pub type IIRCoefficientsPtr<T: HasIIRCoefficients> = Rc<RefCell<<T as HasIIRCoefficients>::Coefficients>>;

impl<FloatType: num::Float> HasIIRCoefficients for FilterDesign<FloatType> {
    type Coefficients = IIRCoefficients<FloatType>;
}

impl<FloatType: num::Float> HasIIRCoefficientsPtr for FilterDesign<FloatType> {}

//-------------------------------------
impl<FloatType: num::Float> HasWindowingMethod for FilterDesign<FloatType> {
    type WindowingMethod = <WindowingFunction<FloatType> as HasWindowingMethod>::WindowingMethod;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/filter_design/aloe_FilterDesign.cpp]
impl<FloatType: num::Float> FilterDesign<FloatType> {

    /**
      | This method generates a FIR::Coefficients
      | for a low-pass filter, using the windowing
      | design method, applied to a sinc impulse
      | response. It is one of the simplest method
      | used to generate a high order low-pass
      | filter, which has the downside of needing
      | more coefficients than more complex
      | method to perform a given attenuation
      | in the stop band.
      | 
      | It generates linear phase filters coefficients.
      | 
      | Note: The flatTop WindowingMethod
      | generates an impulse response with
      | a maximum amplitude higher than one,
      | and might be normalised if necessary
      | depending on the applications.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param order
      | 
      | the order of the filter
      | ----------
      | @param type
      | 
      | the type, must be a WindowingFunction::WindowingType
      | ----------
      | @param beta
      | 
      | an optional additional parameter useful
      | for the Kaiser windowing function
      |
      */
    pub fn design_fir_lowpass_window_method(
        &mut self, 
        frequency:   FloatType,
        sample_rate: f64,
        order:       usize,
        ty:          WindowingMethod,
        beta:        FloatType

    ) -> <Self as HasFIRCoefficientsPtr>::Ptr {

        let beta = FloatType::from(2.0);
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);

        auto* result = new typename FIR::Coefficients<FloatType> (order + 1u);

        auto* c = result->getRawCoefficients();
        auto normalisedFrequency = frequency / sampleRate;

        for (size_t i = 0; i <= order; ++i)
        {
            if (i == order / 2)
            {
                c[i] = static_cast<FloatType> (normalisedFrequency * 2);
            }
            else
            {
                auto indice = MathConstants<double>::pi * (static_cast<double> (i) - 0.5 * static_cast<double> (order));
                c[i] = static_cast<FloatType> (std::sin (2.0 * indice * normalisedFrequency) / indice);
            }
        }

        WindowingFunction<FloatType> theWindow (order + 1, type, false, beta);
        theWindow.multiplyWithWindowingTable (c, order + 1);

        return *result;
        */
    }
    
    /**
      | This a variant of the function designFIRLowpassWindowMethod,
      | which allows the user to specify a transition
      | width and a negative gain in dB, to get
      | a low-pass filter using the Kaiser windowing
      | function, with calculated values of
      | the filter order and of the beta parameter,
      | to satisfy the constraints.
      | 
      | It generates linear phase filters coefficients.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param amplitudedB
      | 
      | the maximum amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_fir_lowpass_kaiser_method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        amplitudedb:                 FloatType

    ) -> <Self as HasFIRCoefficientsPtr>::Ptr {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (amplitudedB >= -100 && amplitudedB <= 0);

        FloatType beta = 0;

        if (amplitudedB < -50)
            beta = static_cast<FloatType> (0.1102 * (-amplitudedB - 8.7));
        else if (amplitudedB <= -21)
            beta = static_cast<FloatType> (0.5842 * std::pow (-amplitudedB - 21, 0.4) + 0.07886 * (-amplitudedB - 21));

        int order = amplitudedB < -21 ? roundToInt (std::ceil ((-amplitudedB - 7.95) / (2.285 * normalisedTransitionWidth * MathConstants<double>::twoPi)))
                                        : roundToInt (std::ceil (5.79 / (normalisedTransitionWidth * MathConstants<double>::twoPi)));

        jassert (order >= 0);

        return designFIRLowpassWindowMethod (frequency, sampleRate, static_cast<size_t> (order),
                                             WindowingFunction<FloatType>::kaiser, beta);
        */
    }
    
    /**
      | This method is also a variant of the function
      | designFIRLowpassWindowMethod, using
      | a rectangular window as a basis, and
      | a spline transition between the pass
      | band and the stop band, to reduce the
      | Gibbs phenomenon.
      | 
      | It generates linear phase filters coefficients.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param order
      | 
      | the order of the filter
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param spline
      | 
      | between 1.0 and 4.0, indicates how much
      | the transition is curved, with 1.0 meaning
      | a straight line
      |
      */
    pub fn design_fir_lowpass_transition_method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        order:                       usize,
        normalised_transition_width: FloatType,
        spline:                      FloatType

    ) -> <Self as HasFIRCoefficientsPtr>::Ptr {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (spline >= 1.0 && spline <= 4.0);

        auto normalisedFrequency = frequency / static_cast<FloatType> (sampleRate);

        auto* result = new typename FIR::Coefficients<FloatType> (order + 1u);
        auto* c = result->getRawCoefficients();

        for (size_t i = 0; i <= order; ++i)
        {
            if (i == order / 2 && order % 2 == 0)
            {
                c[i] = static_cast<FloatType> (2 * normalisedFrequency);
            }
            else
            {
                auto indice  = MathConstants<double>::pi * ((double) i - 0.5 * (double) order);
                auto indice2 = MathConstants<double>::pi * normalisedTransitionWidth * ((double) i - 0.5 * (double) order) / spline;
                c[i] = static_cast<FloatType> (std::sin (2 * indice * normalisedFrequency)
                                                / indice * std::pow (std::sin (indice2) / indice2, spline));
            }
        }

        return *result;
        */
    }
    
    /**
      | This method generates a FIR::Coefficients
      | for a low-pass filter, by minimizing
      | the average error between the generated
      | filter and an ideal one using the least
      | squares error criterion and matrices
      | operations.
      | 
      | It generates linear phase filters coefficients.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param order
      | 
      | the order of the filter
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param stopBandWeight
      | 
      | between 1.0 and 100.0, indicates how
      | much we want attenuation in the stop
      | band, against some oscillation in the
      | pass band
      |
      */
    pub fn design_fir_lowpass_least_squares_method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        order:                       usize,
        normalised_transition_width: FloatType,
        stop_band_weight:            FloatType

    ) -> <Self as HasFIRCoefficientsPtr>::Ptr {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (stopBandWeight >= 1.0 && stopBandWeight <= 100.0);

        auto normalisedFrequency = static_cast<double> (frequency) / sampleRate;

        auto wp = MathConstants<double>::twoPi * (static_cast<double> (normalisedFrequency - normalisedTransitionWidth / 2.0));
        auto ws = MathConstants<double>::twoPi * (static_cast<double> (normalisedFrequency + normalisedTransitionWidth / 2.0));

        auto N = order + 1;

        auto* result = new typename FIR::Coefficients<FloatType> (static_cast<size_t> (N));
        auto* c = result->getRawCoefficients();

        if (N % 2 == 1)
        {
            // Type I
            auto M = (N - 1) / 2;

            Matrix<double> b (M + 1, 1),
                           q (2 * M + 1, 1);

            auto sinc = [] (double x) { return x == 0 ? 1 : std::sin (x * MathConstants<double>::pi)
                                                              / (MathConstants<double>::pi * x); };

            auto factorp = wp / MathConstants<double>::pi;
            auto factors = ws / MathConstants<double>::pi;

            for (size_t i = 0; i <= M; ++i)
                b (i, 0) = factorp * sinc (factorp * (double) i);

            q (0, 0) = factorp + stopBandWeight * (1.0 - factors);

            for (size_t i = 1; i <= 2 * M; ++i)
                q (i, 0) = factorp * sinc (factorp * (double) i) - stopBandWeight * factors * sinc (factors * (double) i);

            auto Q1 = Matrix<double>::toeplitz (q, M + 1);
            auto Q2 = Matrix<double>::hankel (q, M + 1, 0);

            Q1 += Q2; Q1 *= 0.5;

            Q1.solve (b);

            c[M] = static_cast<FloatType> (b (0, 0));

            for (size_t i = 1; i <= M; ++i)
            {
                c[M - i] = static_cast<FloatType> (b (i, 0) * 0.5);
                c[M + i] = static_cast<FloatType> (b (i, 0) * 0.5);
            }
        }
        else
        {
            // Type II
            auto M = N / 2;

            Matrix<double> b (M, 1);
            Matrix<double> qp (2 * M, 1);
            Matrix<double> qs (2 * M, 1);

            auto sinc = [] (double x) { return x == 0 ? 1 : std::sin (x * MathConstants<double>::pi)
                                                              / (MathConstants<double>::pi * x); };

            auto factorp = wp / MathConstants<double>::pi;
            auto factors = ws / MathConstants<double>::pi;

            for (size_t i = 0; i < M; ++i)
                b (i, 0) = factorp * sinc (factorp * ((double) i + 0.5));

            for (size_t i = 0; i < 2 * M; ++i)
            {
                qp (i, 0) = 0.25 * factorp * sinc (factorp * (double) i);
                qs (i, 0) = -0.25 * stopBandWeight * factors * sinc (factors * (double) i);
            }

            auto Q1p = Matrix<double>::toeplitz (qp, M);
            auto Q2p = Matrix<double>::hankel (qp, M, 1);
            auto Q1s = Matrix<double>::toeplitz (qs, M);
            auto Q2s = Matrix<double>::hankel (qs, M, 1);

            auto Id = Matrix<double>::identity (M);
            Id *= (0.25 * stopBandWeight);

            Q1p += Q2p;
            Q1s += Q2s;
            Q1s += Id;

            auto& Q = Q1s;
            Q += Q1p;

            Q.solve (b);

            for (size_t i = 0; i < M; ++i)
            {
                c[M - i - 1] = static_cast<FloatType> (b (i, 0) * 0.25);
                c[M + i]     = static_cast<FloatType> (b (i, 0) * 0.25);
            }
        }

        return *result;
        */
    }
    
    /**
      | This method generates a FIR::Coefficients
      | for a low-pass filter, with a cutoff
      | frequency at half band, using an algorithm
      | described in the article "Design of
      | Half-Band FIR Filters for Signal Compression"
      | from Pavel Zahradnik, to get an equiripple
      | like high order FIR filter, without
      | the need of an iterative method and convergence
      | failure risks.
      | 
      | It generates linear phase filters coefficients.
      | 
      | -----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param amplitudedB
      | 
      | the maximum amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_fir_lowpass_half_band_equiripple_method(
        &mut self, 
        normalised_transition_width: FloatType,
        amplitudedb:                 FloatType

    ) -> <Self as HasFIRCoefficientsPtr>::Ptr {
        
        todo!();
        /*
            jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (amplitudedB >= -300 && amplitudedB <= -10);

        auto wpT = (0.5 - normalisedTransitionWidth) * MathConstants<double>::pi;

        auto n = roundToInt (std::ceil ((amplitudedB - 18.18840664 * wpT + 33.64775300) / (18.54155181 * wpT - 29.13196871)));
        auto kp = (n * wpT - 1.57111377 * n + 0.00665857) / (-1.01927560 * n + 0.37221484);
        auto A = (0.01525753 * n + 0.03682344 + 9.24760314 / (double) n) * kp + 1.01701407 + 0.73512298 / (double) n;
        auto B = (0.00233667 * n - 1.35418408 + 5.75145813 / (double) n) * kp + 1.02999650 - 0.72759508 / (double) n;

        auto hn  = FilterDesign<FloatType>::getPartialImpulseResponseHn (n, kp);
        auto hnm = FilterDesign<FloatType>::getPartialImpulseResponseHn (n - 1, kp);

        auto diff = (hn.size() - hnm.size()) / 2;

        for (int i = 0; i < diff; ++i)
        {
            hnm.add (0.0);
            hnm.insert (0, 0.0);
        }

        auto hh = hn;

        for (int i = 0; i < hn.size(); ++i)
            hh.setUnchecked (i, A * hh[i] + B * hnm[i]);

        auto* result = new typename FIR::Coefficients<FloatType> (static_cast<size_t> (hh.size()));
        auto* c = result->getRawCoefficients();

        for (int i = 0; i < hh.size(); ++i)
            c[i] = (float) hh[i];

        auto NN = [&]
        {
            if (n % 2 == 0)
                return 2.0 * result->getMagnitudeForFrequency (0.5, 1.0);

            auto w01 = std::sqrt (kp * kp + (1 - kp * kp) * std::pow (std::cos (MathConstants<double>::pi / (2.0 * n + 1.0)), 2.0));

            if (std::abs (w01) > 1.0)
                return 2.0 * result->getMagnitudeForFrequency (0.5, 1.0);

            auto om01 = std::acos (-w01);
            return -2.0 * result->getMagnitudeForFrequency (om01 / MathConstants<double>::twoPi, 1.0);
        }();

        for (int i = 0; i < hh.size(); ++i)
            c[i] = static_cast<FloatType> ((A * hn[i] + B * hnm[i]) / NN);

        c[2 * n + 1] = static_cast<FloatType> (0.5);

        return *result;
        */
    }
    
    pub fn get_partial_impulse_response_hn(&mut self, n: i32, kp: f64) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> alpha;
        alpha.resize (2 * n + 1);

        alpha.setUnchecked (2 * n, 1.0 / std::pow (1.0 - kp * kp, n));

        if (n > 0)
            alpha.setUnchecked (2 * n - 2, -(2 * n * kp * kp + 1) * alpha[2 * n]);

        if (n > 1)
            alpha.setUnchecked (2 * n - 4, -(4 * n + 1 + (n - 1) * (2 * n - 1) * kp * kp) / (2.0 * n) * alpha[2 * n - 2]
                                 - (2 * n + 1) * ((n + 1) * kp * kp + 1) / (2.0 * n) * alpha[2 * n]);

        for (int k = n; k >= 3; --k)
        {
            auto c1 = (3 * (n*(n + 2) - k * (k - 2)) + 2 * k - 3 + 2 * (k - 2)*(2 * k - 3) * kp * kp) * alpha[2 * k - 4];
            auto c2 = (3 * (n*(n + 2) - (k - 1) * (k + 1)) + 2 * (2 * k - 1) + 2 * k*(2 * k - 1) * kp * kp) * alpha[2 * k - 2];
            auto c3 = (n * (n + 2) - (k - 1) * (k + 1)) * alpha[2 * k];
            auto c4 = (n * (n + 2) - (k - 3) * (k - 1));

            alpha.setUnchecked (2 * k - 6, -(c1 + c2 + c3) / c4);
        }

        Vec<double> ai;
        ai.resize (2 * n + 1 + 1);

        for (int k = 0; k <= n; ++k)
            ai.setUnchecked (2 * k + 1, alpha[2 * k] / (2.0 * k + 1.0));

        Vec<double> hn;
        hn.resize (2 * n + 1 + 2 * n + 1 + 1);

        for (int k = 0; k <= n; ++k)
        {
            hn.setUnchecked (2 * n + 1 + (2 * k + 1), 0.5 * ai[2 * k + 1]);
            hn.setUnchecked (2 * n + 1 - (2 * k + 1), 0.5 * ai[2 * k + 1]);
        }

        return hn;
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIRFilters,
      | providing a minimum phase low-pass
      | filter without any ripple in the pass
      | band and in the stop band.
      | 
      | The algorithms are based on "Lecture
      | Notes on Elliptic Filter Design" by
      | Sophocles J. Orfanidis.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param passbandAmplitudedB
      | 
      | the highest gain in dB expected in the
      | pass band (must be negative)
      | ----------
      | @param stopbandAmplitudedB
      | 
      | the gain in dB expected in the stop band
      | (must be negative)
      |
      */
    pub fn design_iir_lowpass_high_order_butterworth_method_with_bands(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        passband_amplitudedb:        FloatType,
        stopband_amplitudedb:        FloatType

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            return designIIRLowpassHighOrderGeneralMethod (0, frequency, sampleRate, normalisedTransitionWidth,
                                                       passbandAmplitudedB, stopbandAmplitudedB);
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIRFilters,
      | providing a minimum phase low-pass
      | filter without any ripple in the stop
      | band only.
      | 
      | The algorithms are based on "Lecture
      | Notes on Elliptic Filter Design" by
      | Sophocles J. Orfanidis.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param passbandAmplitudedB
      | 
      | the highest amplitude in dB expected
      | in the pass band (must be negative)
      | ----------
      | @param stopbandAmplitudedB
      | 
      | the lowest amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_iir_lowpass_high_order_chebyshev_1method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        passband_amplitudedb:        FloatType,
        stopband_amplitudedb:        FloatType

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            return designIIRLowpassHighOrderGeneralMethod (1, frequency, sampleRate, normalisedTransitionWidth,
                                                       passbandAmplitudedB, stopbandAmplitudedB);
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIRFilters,
      | providing a minimum phase low-pass
      | filter without any ripple in the pass
      | band only.
      | 
      | The algorithms are based on "Lecture
      | Notes on Elliptic Filter Design" by
      | Sophocles J. Orfanidis.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param passbandAmplitudedB
      | 
      | the highest amplitude in dB expected
      | in the pass band (must be negative)
      | ----------
      | @param stopbandAmplitudedB
      | 
      | the lowest amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_iir_lowpass_high_order_chebyshev_2method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        passband_amplitudedb:        FloatType,
        stopband_amplitudedb:        FloatType

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            return designIIRLowpassHighOrderGeneralMethod (2, frequency, sampleRate, normalisedTransitionWidth,
                                                       passbandAmplitudedB, stopbandAmplitudedB);
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIR::Filters,
      | providing a minimum phase low-pass
      | filter with ripples in both the pass
      | band and in the stop band.
      | 
      | The algorithms are based on "Lecture
      | Notes on Elliptic Filter Design" by
      | Sophocles J. Orfanidis.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param passbandAmplitudedB
      | 
      | the highest amplitude in dB expected
      | in the pass band (must be negative)
      | ----------
      | @param stopbandAmplitudedB
      | 
      | the lowest amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_iir_lowpass_high_order_elliptic_method(
        &mut self, 
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        passband_amplitudedb:        FloatType,
        stopband_amplitudedb:        FloatType

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            return designIIRLowpassHighOrderGeneralMethod (3, frequency, sampleRate, normalisedTransitionWidth,
                                                       passbandAmplitudedB, stopbandAmplitudedB);
        */
    }
    
    pub fn design_iir_lowpass_high_order_general_method(
        &mut self, 
        ty:                          i32,
        frequency:                   FloatType,
        sample_rate:                 f64,
        normalised_transition_width: FloatType,
        passband_amplitudedb:        FloatType,
        stopband_amplitudedb:        FloatType

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (passbandAmplitudedB > -20 && passbandAmplitudedB < 0);
        jassert (stopbandAmplitudedB > -300 && stopbandAmplitudedB < -20);

        auto normalisedFrequency = frequency / sampleRate;

        auto fp = normalisedFrequency - normalisedTransitionWidth / 2;
        auto fs = normalisedFrequency + normalisedTransitionWidth / 2;

        double Ap = passbandAmplitudedB;
        double As = stopbandAmplitudedB;
        auto Gp = Decibels::decibelsToGain (Ap, -300.0);
        auto Gs = Decibels::decibelsToGain (As, -300.0);
        auto epsp = std::sqrt (1.0 / (Gp * Gp) - 1.0);
        auto epss = std::sqrt (1.0 / (Gs * Gs) - 1.0);

        auto omegap = std::tan (MathConstants<double>::pi * fp);
        auto omegas = std::tan (MathConstants<double>::pi * fs);
        constexpr auto halfPi = MathConstants<double>::halfPi;

        auto k = omegap / omegas;
        auto k1 = epsp / epss;

        int N;

        if (type == 0)
        {
            N = roundToInt (std::ceil (std::log (1.0 / k1) / std::log (1.0 / k)));
        }
        else if (type == 1 || type == 2)
        {
            N = roundToInt (std::ceil (std::acosh (1.0 / k1) / std::acosh (1.0 / k)));
        }
        else
        {
            double K, Kp, K1, K1p;

            SpecialFunctions::ellipticIntegralK (k, K, Kp);
            SpecialFunctions::ellipticIntegralK (k1, K1, K1p);

            N = roundToInt (std::ceil ((K1p * K) / (K1 * Kp)));
        }

        const int r = N % 2;
        const int L = (N - r) / 2;
        const double H0 = (type == 1 || type == 3) ? std::pow (Gp, 1.0 - r) : 1.0;

        Vec<Complex<double>> pa, za;
        Complex<double> j (0, 1);

        if (type == 0)
        {
            if (r == 1)
                pa.add (-omegap * std::pow (epsp, -1.0 / (double) N));

            for (int i = 1; i <= L; ++i)
            {
                auto ui = (2 * i - 1.0) / (double) N;
                pa.add (omegap * std::pow (epsp, -1.0 / (double) N) * j * exp (ui * halfPi * j));
            }
        }
        else if (type == 1)
        {
            auto v0 = std::asinh (1.0 / epsp) / (N * halfPi);

            if (r == 1)
                pa.add (-omegap * std::sinh (v0 * halfPi));

            for (int i = 1; i <= L; ++i)
            {
                auto ui = (2 * i - 1.0) / (double) N;
                pa.add (omegap * j * std::cos ((ui - j * v0) * halfPi));
            }
        }
        else if (type == 2)
        {
            auto v0 = std::asinh (epss) / (N * halfPi);

            if (r == 1)
                pa.add(-1.0 / (k / omegap * std::sinh (v0 * halfPi)));

            for (int i = 1; i <= L; ++i)
            {
                auto ui = (2 * i - 1.0) / (double) N;

                pa.add (1.0 / (k / omegap * j * std::cos ((ui - j * v0) * halfPi)));
                za.add (1.0 / (k / omegap * j * std::cos (ui * halfPi)));
            }
        }
        else
        {
            auto v0 = -j * (SpecialFunctions::asne (j / epsp, k1) / (double) N);

            if (r == 1)
                pa.add (omegap * j * SpecialFunctions::sne (j * v0, k));

            for (int i = 1; i <= L; ++i)
            {
                auto ui = (2 * i - 1.0) / (double) N;
                auto zetai = SpecialFunctions::cde (ui, k);

                pa.add (omegap * j * SpecialFunctions::cde (ui - j * v0, k));
                za.add (omegap * j / (k * zetai));
            }
        }

        Vec<Complex<double>> p, z, g;

        if (r == 1)
        {
            p.add ((1.0 + pa[0]) / (1.0 - pa[0]));
            g.add (0.5 * (1.0 - p[0]));
        }

        for (int i = 0; i < L; ++i)
        {
            p.add ((1.0 + pa[i + r]) / (1.0 - pa[i + r]));
            z.add (za.size() == 0 ? -1.0 : (1.0 + za[i]) / (1.0 - za[i]));
            g.add ((1.0 - p[i + r]) / (1.0 - z[i]));
        }

        ReferenceCountedArray<IIR::Coefficients<FloatType>> cascadedCoefficients;

        if (r == 1)
        {
            auto b0 = static_cast<FloatType> (H0 * std::real (g[0]));
            auto b1 = b0;
            auto a1 = static_cast<FloatType> (-std::real (p[0]));

            cascadedCoefficients.add (new IIR::Coefficients<FloatType> (b0, b1, 1.0f, a1));
        }

        for (int i = 0; i < L; ++i)
        {
            auto gain = std::pow (std::abs (g[i + r]), 2.0);

            auto b0 = static_cast<FloatType> (gain);
            auto b1 = static_cast<FloatType> (std::real (-z[i] - std::conj (z[i])) * gain);
            auto b2 = static_cast<FloatType> (std::real ( z[i] * std::conj (z[i])) * gain);

            auto a1 = static_cast<FloatType> (std::real (-p[i+r] - std::conj (p[i + r])));
            auto a2 = static_cast<FloatType> (std::real ( p[i+r] * std::conj (p[i + r])));

            cascadedCoefficients.add (new IIR::Coefficients<FloatType> (b0, b1, b2, 1, a1, a2));
        }

        return cascadedCoefficients;
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIRFilters,
      | providing a minimum phase low-pass
      | filter without any ripple in the pass
      | band and in the stop band.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the low-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param order
      | 
      | the order of the resulting IIR filter,
      | providing an attenuation of -6 dB times
      | order / octave
      |
      */
    pub fn design_iir_lowpass_high_order_butterworth_method(
        &mut self, 
        frequency:   FloatType,
        sample_rate: f64,
        order:       i32

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (order > 0);

        ReferenceCountedArray<IIR::Coefficients<FloatType>> arrayFilters;

        if (order % 2 == 1)
        {
            arrayFilters.add (*IIR::Coefficients<FloatType>::makeFirstOrderLowPass (sampleRate, frequency));

            for (int i = 0; i < order / 2; ++i)
            {
                auto Q = 1.0 / (2.0 * std::cos ((i + 1.0) * MathConstants<double>::pi / order));
                arrayFilters.add (*IIR::Coefficients<FloatType>::makeLowPass (sampleRate, frequency,
                                                                              static_cast<FloatType> (Q)));
            }
        }
        else
        {
            for (int i = 0; i < order / 2; ++i)
            {
                auto Q = 1.0 / (2.0 * std::cos ((2.0 * i + 1.0) * MathConstants<double>::pi / (order * 2.0)));
                arrayFilters.add (*IIR::Coefficients<FloatType>::makeLowPass (sampleRate, frequency,
                                                                              static_cast<FloatType> (Q)));
            }
        }

        return arrayFilters;
        */
    }
    
    /**
      | This method returns an array of IIR::Coefficients,
      | made to be used in cascaded IIRFilters,
      | providing a minimum phase high-pass
      | filter without any ripple in the pass
      | band and in the stop band.
      | 
      | -----------
      | @param frequency
      | 
      | the cutoff frequency of the high-pass
      | filter
      | ----------
      | @param sampleRate
      | 
      | the sample rate being used in the filter
      | design
      | ----------
      | @param order
      | 
      | the order of the resulting IIR filter,
      | providing an attenuation of -6 dB times
      | order / octave
      |
      */
    pub fn design_iir_highpass_high_order_butterworth_method(
        &mut self, 
        frequency:   FloatType,
        sample_rate: f64,
        order:       i32

    ) -> Vec<Rc<RefCell<IIRCoefficients<FloatType>>>> {
        
        todo!();
        /*
            jassert (sampleRate > 0);
        jassert (frequency > 0 && frequency <= sampleRate * 0.5);
        jassert (order > 0);

        ReferenceCountedArray<IIR::Coefficients<FloatType>> arrayFilters;

        if (order % 2 == 1)
        {
            arrayFilters.add (*IIR::Coefficients<FloatType>::makeFirstOrderHighPass (sampleRate, frequency));

            for (int i = 0; i < order / 2; ++i)
            {
                auto Q = 1.0 / (2.0 * std::cos ((i + 1.0) * MathConstants<double>::pi / order));
                arrayFilters.add (*IIR::Coefficients<FloatType>::makeHighPass (sampleRate, frequency,
                                                                               static_cast<FloatType> (Q)));
            }
        }
        else
        {
            for (int i = 0; i < order / 2; ++i)
            {
                auto Q = 1.0 / (2.0 * std::cos ((2.0 * i + 1.0) * MathConstants<double>::pi / (order * 2.0)));
                arrayFilters.add (*IIR::Coefficients<FloatType>::makeHighPass (sampleRate, frequency,
                                                                               static_cast<FloatType> (Q)));
            }
        }

        return arrayFilters;
        */
    }
    
    /**
      | This method generates arrays of IIR::Coefficients
      | for a low-pass filter, with a cutoff
      | frequency at half band, using an algorithm
      | described in the article "Digital Signal
      | Processing Schemes for efficient interpolation
      | and decimation" from Pavel Valenzuela
      | and Constantinides.
      | 
      | The result is a FilterDesignIIRPolyphaseAllpassStructure
      | object.
      | 
      | The two members of this structure directPath
      | and delayedPath are arrays of IIR::Coefficients,
      | made of polyphase second order allpass
      | filters and an additional delay in the
      | second array, that can be used in cascaded
      | filters processed in two parallel paths,
      | which must be summed at the end to get
      | the high order efficient low-pass filtering.
      | 
      | The gain of the resulting pass-band
      | is 6 dB, so don't forget to compensate
      | it if you want to use that method for something
      | else than two times oversampling.
      | 
      | -----------
      | @param normalisedTransitionWidth
      | 
      | the normalised size between 0 and 0.5
      | of the transition between the pass band
      | and the stop band
      | ----------
      | @param stopbandAmplitudedB
      | 
      | the maximum amplitude in dB expected
      | in the stop band (must be negative)
      |
      */
    pub fn design_iir_lowpass_half_band_polyphase_allpass_method(
        &mut self, 
        normalised_transition_width: FloatType,
        stopband_amplitudedb:        FloatType

    ) -> IIRPolyphaseAllpassStructure<FloatType> {
        
        todo!();
        /*
            jassert (normalisedTransitionWidth > 0 && normalisedTransitionWidth <= 0.5);
        jassert (stopbandAmplitudedB > -300 && stopbandAmplitudedB < -10);

        const double wt = MathConstants<double>::twoPi * normalisedTransitionWidth;
        const double ds = Decibels::decibelsToGain (stopbandAmplitudedB, static_cast<FloatType> (-300.0));

        auto k = std::pow (std::tan ((MathConstants<double>::pi - wt) / 4), 2.0);
        auto kp = std::sqrt (1.0 - k * k);
        auto e = (1 - std::sqrt (kp)) / (1 + std::sqrt (kp)) * 0.5;
        auto q = e + 2 * std::pow (e, 5.0) + 15 * std::pow (e, 9.0) + 150 * std::pow (e, 13.0);

        auto k1 = ds * ds / (1 - ds * ds);
        int n = roundToInt (std::ceil (std::log (k1 * k1 / 16) / std::log (q)));

        if (n % 2 == 0)
            ++n;

        if (n == 1)
            n = 3;

        auto q1 = std::pow (q, (double) n);
        k1 = 4 * std::sqrt (q1);

        const int N = (n - 1) / 2;
        Vec<double> ai;

        for (int i = 1; i <= N; ++i)
        {
            double num = 0.0;
            double delta = 1.0;
            int m = 0;

            while (std::abs (delta) > 1e-100)
            {
                delta = std::pow (-1, m) * std::pow (q, m * (m + 1))
                         * std::sin ((2 * m + 1) * MathConstants<double>::pi * i / (double) n);
                num += delta;
                m++;
            }

            num *= 2 * std::pow (q, 0.25);

            double den = 0.0;
            delta = 1.0;
            m = 1;

            while (std::abs (delta) > 1e-100)
            {
                delta = std::pow (-1, m) * std::pow (q, m * m)
                         * std::cos (m * MathConstants<double>::twoPi * i / (double) n);
                den += delta;
                ++m;
            }

            den = 1 + 2 * den;

            auto wi = num / den;
            auto api = std::sqrt ((1 - wi * wi * k) * (1 - wi * wi / k)) / (1 + wi * wi);

            ai.add ((1 - api) / (1 + api));
        }

        FilterDesignIIRPolyphaseAllpassStructure structure;

        for (int i = 0; i < N; i += 2)
            structure.directPath.add (new IIR::Coefficients<FloatType> (static_cast<FloatType> (ai[i]),
                                                                        0, 1, 1, 0, static_cast<FloatType> (ai[i])));

        structure.delayedPath.add (new IIR::Coefficients<FloatType> (0, 1, 1, 0));

        for (int i = 1; i < N; i += 2)
            structure.delayedPath.add (new IIR::Coefficients<FloatType> (static_cast<FloatType> (ai[i]),
                                                                         0, 1, 1, 0, static_cast<FloatType> (ai[i])));

        structure.alpha.addArray (ai);

        return structure;
        */
    }
}
