crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_FastMathApproximations.h]

/**
  | This class contains various fast mathematical
  | function approximations.
  | 
  | @tags{DSP}
  |
  */
pub struct FastMathApproximations {

}

impl FastMathApproximations {

    /**
      | Provides a fast approximation of the
      | function cosh(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn cosh<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = -(39251520 + x2 * (18471600 + x2 * (1075032 + 14615 * x2)));
            auto denominator = -39251520 + x2 * (1154160 + x2 * (-16632 + 127 * x2));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function cosh(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn cosh_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::cosh (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function sinh(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn sinh<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = -x * (11511339840 + x2 * (1640635920 + x2 * (52785432 + x2 * 479249)));
            auto denominator = -11511339840 + x2 * (277920720 + x2 * (-3177720 + x2 * 18361));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function sinh(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn sinh_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::sinh (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function tanh(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn tanh<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = x * (135135 + x2 * (17325 + x2 * (378 + x2)));
            auto denominator = 135135 + x2 * (62370 + x2 * (3150 + 28 * x2));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function tanh(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -5 and
      | +5 for limiting the error.
      |
      */
    pub fn tanh_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::tanh (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function cos(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi
      | and +pi for limiting the error.
      |
      */
    pub fn cos<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = -(-39251520 + x2 * (18471600 + x2 * (-1075032 + 14615 * x2)));
            auto denominator = 39251520 + x2 * (1154160 + x2 * (16632 + x2 * 127));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function cos(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi
      | and +pi for limiting the error.
      |
      */
    pub fn cos_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::cos (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function sin(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi
      | and +pi for limiting the error.
      |
      */
    pub fn sin<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = -x * (-11511339840 + x2 * (1640635920 + x2 * (-52785432 + x2 * 479249)));
            auto denominator = 11511339840 + x2 * (277920720 + x2 * (3177720 + x2 * 18361));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function sin(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi
      | and +pi for limiting the error.
      |
      */
    pub fn sin_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::sin (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function tan(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi/2
      | and +pi/2 for limiting the error.
      |
      */
    pub fn tan<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto x2 = x * x;
            auto numerator = x * (-135135 + x2 * (17325 + x2 * (-378 + x2)));
            auto denominator = -135135 + x2 * (62370 + x2 * (-3150 + 28 * x2));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function tan(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -pi/2
      | and +pi/2 for limiting the error.
      |
      */
    pub fn tan_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::tan (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function exp(x) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -6 and
      | +4 for limiting the error.
      |
      */
    pub fn exp<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto numerator = 1680 + x * (840 + x * (180 + x * (20 + x)));
            auto denominator = 1680 + x *(-840 + x * (180 + x * (-20 + x)));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function exp(x) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -6 and
      | +4 for limiting the error.
      |
      */
    pub fn exp_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::exp (values[i]);
        */
    }

    /**
      | Provides a fast approximation of the
      | function log(x+1) using a Pade approximant
      | continued fraction, calculated sample
      | by sample.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -0.8
      | and +5 for limiting the error.
      |
      */
    pub fn logn_plus_one<FloatType>(x: FloatType) -> FloatType {
    
        todo!();
        /*
            auto numerator = x * (7560 + x * (15120 + x * (9870 + x * (2310 + x * 137))));
            auto denominator = 7560 + x * (18900 + x * (16800 + x * (6300 + x * (900 + 30 * x))));
            return numerator / denominator;
        */
    }

    /**
      | Provides a fast approximation of the
      | function log(x+1) using a Pade approximant
      | continued fraction, calculated on
      | a whole buffer.
      | 
      | Note: This is an approximation which
      | works on a limited range. You are advised
      | to use input values only between -0.8
      | and +5 for limiting the error.
      |
      */
    pub fn logn_plus_one_for_buffer<FloatType>(
        values:     *mut FloatType,
        num_values: usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < numValues; ++i)
                values[i] = FastMathApproximations::logNPlusOne (values[i]);
        */
    }
}
