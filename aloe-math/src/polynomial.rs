crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_Polynomial.h]

/**
  | A class representing a polynomial
  | 
  | @tags{DSP}
  |
  */
#[leak_detector]
pub struct Polynomial<FloatingType> {
    coeffs: Vec<FloatingType>,
}

impl<FloatingType> Default for Polynomial<FloatingType> {
    
    /**
      | Creates a new polynomial which will
      | always evaluate to zero.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            coeffs.add (0)
        */
    }
}

impl<FloatingType> Index<i32> for Polynomial<FloatingType> {
    type Output = FloatingType;
    
    /**
      | Returns a single coefficient of the
      | receiver for reading
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            return coeffs.getUnchecked (index);
        */
    }
}

impl<FloatingType> IndexMut<i32> for Polynomial<FloatingType> {
    
    /**
      | Returns a single coefficient of the
      | receiver for modifying.
      |
      */
    #[inline] fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        todo!();
        /*
            return coeffs.getReference (index);
        */
    }
}

impl<FloatingType> Polynomial<FloatingType> {

    /**
      | Creates a new polynomial with given
      | coefficients.
      | 
      | -----------
      | @param numCoefficients
      | 
      | The number of coefficients stored in
      | coefficients. This is also the order
      | of the returned polynomial.
      | ----------
      | @param coefficients
      | 
      | The coefficients which will be used
      | by the newly created polynomial. The
      | Polynomial class will keep a private
      | copy of the coefficients.
      |
      */
    pub fn new(
        coefficients:     *const FloatingType,
        num_coefficients: i32) -> Self {
    
        todo!();
        /*
        : coeffs(coefficients, numCoefficients),

            jassert (! coeffs.isEmpty());
        */
    }

    /**
      | Creates a new polynomial with coefficients
      | by a C++11 initializer list.
      | 
      | This function can be used in the following
      | way:
      | 
      | Polynomial<float> p ({0.5f, -0.3f,
      | 0.2f});
      |
      */
    pub fn new_with_items<Values>(items: Values) -> Self {
    
        todo!();
        /*


            : coeffs (items...)
            jassert (! coeffs.isEmpty());
        */
    }

    /**
      | Evaluates the value of the polynomial
      | at a single point x.
      |
      */
    pub fn invoke(&self, x: FloatingType) -> FloatingType {
        
        todo!();
        /*
            // Horner's method
            FloatingType y (0);

            for (int i = coeffs.size(); --i >= 0;)
                y = (x * y) + coeffs.getUnchecked(i);

            return y;
        */
    }

    /**
      | Returns the order of the polynomial.
      |
      */
    pub fn get_order(&mut self) -> i32 {
        
        todo!();
        /*
            return coeffs.size() - 1;
        */
    }
    
    /**
      | Returns the polynomial with all its
      | coefficients multiplied with a gain
      | factor
      |
      */
    pub fn with_gain(&self, gain: f64) -> Polynomial<FloatingType> {
        
        todo!();
        /*
            auto result = *this;

            for (auto& c : result.coeffs)
                c *= gain;

            return result;
        */
    }

    /**
      | Returns the sum of this polynomial with
      | another
      |
      */
    pub fn get_sum_with(&self, other: &Polynomial<FloatingType>) -> Polynomial<FloatingType> {
        
        todo!();
        /*
            if (coeffs.size() < other.coeffs.size())
                return other.getSumWith (*this);

            auto result = *this;

            for (int i = 0; i < other.coeffs.size(); ++i)
                result[i] += other[i];

            return result;
        */
    }

    /**
      | computes the product of two polynomials
      | and return the result
      |
      */
    pub fn get_product_with(&self, other: &Polynomial<FloatingType>) -> Polynomial<FloatingType> {
        
        todo!();
        /*
            Polynomial<FloatingType> result;
            result.coeffs.clearQuick();

            auto N1 = coeffs.size();
            auto N2 = other.coeffs.size();
            auto Nmax = jmax (N1, N2);

            auto N = N1 + N2 - 1;

            for (int i = 0; i < N; ++i)
            {
                FloatingType value (0);

                for (int j = 0; j < Nmax; ++j)
                    if (j >= 0 && j < N1 && i - j >= 0 && i - j < N2)
                        value = value + (*this)[j] * other[i - j];

                result.coeffs.add (value);
            }

            return result;
        */
    }
}
