crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_SpecialFunctions.h]

/**
  | Contains miscellaneous filter design
  | and windowing functions.
  | 
  | @tags{DSP}
  |
  */
pub struct SpecialFunctions {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_SpecialFunctions.cpp]
impl SpecialFunctions {

    /**
      | Computes the modified Bessel function
      | of the first kind I0 for a given double
      | value x. Modified Bessel functions
      | are useful to solve various mathematical
      | problems involving differential equations.
      |
      */
    pub fn besseli0(&mut self, x: f64) -> f64 {
        
        todo!();
        /*
            auto ax = std::abs (x);

        if (ax < 3.75)
        {
            auto y = x / 3.75;
            y *= y;

            return 1.0 + y * (3.5156229 + y * (3.0899424 + y * (1.2067492
                    + y * (0.2659732 + y * (0.360768e-1 + y * 0.45813e-2)))));
        }

        auto y = 3.75 / ax;

        return (std::exp (ax) / std::sqrt (ax))
                 * (0.39894228 + y * (0.1328592e-1 + y * (0.225319e-2 + y * (-0.157565e-2 + y * (0.916281e-2
                     + y * (-0.2057706e-1 + y * (0.2635537e-1 + y * (-0.1647633e-1 + y * 0.392377e-2))))))));
        */
    }
    
    /**
      | Computes the complete elliptic integral
      | of the first kind K for a given double
      | value k, and the associated complete
      | elliptic integral of the first kind
      | Kp for the complementary modulus of
      | k.
      |
      */
    pub fn elliptic_integralk(&mut self, 
        k0: f64,
        k1: &mut f64,
        kp: &mut f64)  {
        
        todo!();
        /*
            constexpr int M = 4;

        K = MathConstants<double>::halfPi;
        auto lastK = k;

        for (int i = 0; i < M; ++i)
        {
            lastK = std::pow (lastK / (1 + std::sqrt (1 - std::pow (lastK, 2.0))), 2.0);
            K *= 1 + lastK;
        }

        Kp = MathConstants<double>::halfPi;
        auto last = std::sqrt (1 - k * k);

        for (int i = 0; i < M; ++i)
        {
            last = std::pow (last / (1.0 + std::sqrt (1.0 - std::pow (last, 2.0))), 2.0);
            Kp *= 1 + last;
        }
        */
    }
    
    /**
      | Computes the Jacobian elliptic function
      | cd for the elliptic modulus k and the
      | quarter-period units u.
      |
      */
    pub fn cde(&mut self, 
        u: Complex<f64>,
        k: f64) -> Complex<f64> {
        
        todo!();
        /*
            constexpr int M = 4;

        double ke[M + 1];
        double* kei = ke;
        *kei = k;

        for (int i = 0; i < M; ++i)
        {
            auto next = std::pow (*kei / (1.0 + std::sqrt (1.0 - std::pow (*kei, 2.0))), 2.0);
            *++kei = next;
        }

        // NB: the spurious cast to double here is a workaround for a very odd link-time failure
        std::complex<double> last = std::cos (u * (double) MathConstants<double>::halfPi);

        for (int i = M - 1; i >= 0; --i)
            last = (1.0 + ke[i + 1]) / (1.0 / last + ke[i + 1] * last);

        return last;
        */
    }
    
    /**
      | Computes the Jacobian elliptic function
      | sn for the elliptic modulus k and the
      | quarter-period units u.
      |
      */
    pub fn sne(&mut self, 
        u: Complex<f64>,
        k: f64) -> Complex<f64> {
        
        todo!();
        /*
            constexpr int M = 4;

        double ke[M + 1];
        double* kei = ke;
        *kei = k;

        for (int i = 0; i < M; ++i)
        {
            auto next = std::pow (*kei / (1 + std::sqrt (1 - std::pow (*kei, 2.0))), 2.0);
            *++kei = next;
        }

        // NB: the spurious cast to double here is a workaround for a very odd link-time failure
        std::complex<double> last = std::sin (u * (double) MathConstants<double>::halfPi);

        for (int i = M - 1; i >= 0; --i)
            last = (1.0 + ke[i + 1]) / (1.0 / last + ke[i + 1] * last);

        return last;
        */
    }
    
    /**
      | Computes the inverse of the Jacobian
      | elliptic function sn for the elliptic
      | modulus k and the quarter-period units
      | u.
      |
      */
    pub fn asne(&mut self, 
        w: Complex<f64>,
        k: f64) -> Complex<f64> {
        
        todo!();
        /*
            constexpr int M = 4;

        double ke[M + 1];
        double* kei = ke;
        *kei = k;

        for (int i = 0; i < M; ++i)
        {
            auto next = std::pow (*kei / (1.0 + std::sqrt (1.0 - std::pow (*kei, 2.0))), 2.0);
            *++kei = next;
        }

        std::complex<double> last = w;

        for (int i = 1; i <= M; ++i)
            last = 2.0 * last / ((1.0 + ke[i]) * (1.0 + std::sqrt (1.0 - std::pow (ke[i - 1] * last, 2.0))));

        return 2.0 / MathConstants<double>::pi * std::asin (last);
        */
    }
}
