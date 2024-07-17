crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/ResamplerKaiserWindow.h]

/**
  | Calculate a Kaiser window centered
  | at 0.
  |
  */
pub struct ResamplerKaiserWindow {
    beta:                f64, // default = 0.0
    inverse_bessel_beta: f64, // default = 1.0
}

impl Default for ResamplerKaiserWindow {
    
    fn default() -> Self {
        todo!();
        /*
            setStopBandAttenuation(60)
        */
    }
}

impl ResamplerKaiserWindow {

    /**
      | @param attenuation
      | 
      | typical values range from 30 to 90 dB
      | @return beta
      |
      */
    pub fn set_stop_band_attenuation(&mut self, attenuation: f64) -> f64 {
        
        todo!();
        /*
            double beta = 0.0;
            if (attenuation > 50) {
                beta = 0.1102 * (attenuation - 8.7);
            } else if (attenuation >= 21) {
                double a21 = attenuation - 21;
                beta = 0.5842 * pow(a21, 0.4) + (0.07886 * a21);
            }
            setBeta(beta);
            return beta;
        */
    }
    
    pub fn set_beta(&mut self, beta: f64)  {
        
        todo!();
        /*
            mBeta = beta;
            mInverseBesselBeta = 1.0 / bessel(beta);
        */
    }

    /**
      | @param x
      | 
      | ranges from -1.0 to +1.0
      |
      */
    pub fn invoke(&mut self, x: f64) -> f64 {
        
        todo!();
        /*
            double x2 = x * x;
            if (x2 >= 1.0) return 0.0;
            double w = mBeta * sqrt(1.0 - x2);
            return bessel(w) * mInverseBesselBeta;
        */
    }

    /**
      | Approximation of a modified zero order
      | Bessel function of the first kind.
      |
      | Based on a discussion at:
      | https://dsp.stackexchange.com/questions/37714/kaiser-window-approximation
      */
    pub fn bessel(x: f64) -> f64 {
        
        todo!();
        /*
            double y = cosh(0.970941817426052 * x);
            y += cosh(0.8854560256532099 * x);
            y += cosh(0.7485107481711011 * x);
            y += cosh(0.5680647467311558 * x);
            y += cosh(0.3546048870425356 * x);
            y += cosh(0.120536680255323 * x);
            y *= 2;
            y += cosh(x);
            y /= 13;
            return y;
        */
    }
}
