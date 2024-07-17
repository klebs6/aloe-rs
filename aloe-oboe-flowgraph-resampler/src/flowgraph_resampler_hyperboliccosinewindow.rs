crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/ResamplerHyperbolicCosineWindow.h]

/**
  | Calculate a HyperbolicCosineWindow
  | window centered at 0.
  | 
  | This can be used in place of a Kaiser window.
  | 
  | The code is based on an anonymous contribution
  | by "a concerned citizen": https://dsp.stackexchange.com/questions/37714/kaiser-window-approximation
  |
  */
pub struct HyperbolicCosineWindow {
    alpha:              f64, // default = 0.0
    inverse_cosh_alpha: f64, // default = 1.0
}

impl Default for HyperbolicCosineWindow {
    
    fn default() -> Self {
        todo!();
        /*


            setStopBandAttenuation(60)
        */
    }
}

impl HyperbolicCosineWindow {

    /**
      | @param attenuation
      | 
      | typical values range from 30 to 90 dB
      | 
      | -----------
      | @return
      | 
      | beta
      |
      */
    pub fn set_stop_band_attenuation(&mut self, attenuation: f64) -> f64 {
        
        todo!();
        /*
            double alpha = ((-325.1e-6 * attenuation + 0.1677) * attenuation) - 3.149;
            setAlpha(alpha);
            return alpha;
        */
    }
    
    pub fn set_alpha(&mut self, alpha: f64)  {
        
        todo!();
        /*
            mAlpha = alpha;
            mInverseCoshAlpha = 1.0 / cosh(alpha);
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
            double w = mAlpha * sqrt(1.0 - x2);
            return cosh(w) * mInverseCoshAlpha;
        */
    }
}
