crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_StatisticsAccumulator.h]

#[derive(Default)]
pub struct KahanSum<FloatType> {
    sum:   FloatType,
    error: FloatType,
}

impl<FloatType> KahanSum<FloatType> {
    
    #[inline] pub fn into(self) -> FloatType {
        self.sum
    }
}


impl<FloatType> AddAssign<FloatType> for KahanSum<FloatType> {
    
    #[ALOE_NO_ASSOCIATIVE_MATH_OPTIMIZATIONS]
    #[inline] fn add_assign(&mut self, other: FloatType) {
        todo!();
        /*
            FloatType correctedValue = value - error;
                FloatType newSum = sum + correctedValue;
                error = (newSum - sum) - correctedValue;
                sum = newSum;
        */
    }
}

/**
  | A class that measures various statistics
  | about a series of floating point values
  | that it is given.
  | 
  | @tags{Core}
  |
  */
pub struct StatisticsAccumulator<FloatType> {
    count:       usize,     // default = 0 
    sum:         KahanSum<FloatType>,
    sum_squares: KahanSum<FloatType>,
    minimum:     FloatType, // {  std::numeric_limits<FloatType>::infinity() };
    maximum:     FloatType, // { -std::numeric_limits<FloatType>::infinity() };
}

impl<FloatType> Default for StatisticsAccumulator<FloatType> {
    
    /**
      | Constructs a new StatisticsAccumulator.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<FloatType> StatisticsAccumulator<FloatType> {
    
    /**
      | Add a new value to the accumulator.
      | 
      | This will update all running statistics
      | accordingly.
      |
      */
    pub fn add_value(&mut self, v: FloatType)  {
        
        todo!();
        /*
            jassert (aloe_isfinite (v));

            sum += v;
            sumSquares += v * v;
            ++count;

            if (v > maximum) maximum = v;
            if (v < minimum) minimum = v;
        */
    }

    /**
      | Reset the accumulator.
      | 
      | This will reset all currently saved
      | statistcs.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            *this = StatisticsAccumulator<FloatType>();
        */
    }

    /**
      | Returns the average (arithmetic mean)
      | of all previously added values.
      | 
      | If no values have been added yet, this
      | will return zero.
      |
      */
    pub fn get_average(&self) -> FloatType {
        
        todo!();
        /*
            return count > 0 ? sum / (FloatType) count
                             : FloatType();
        */
    }

    /**
      | Returns the variance of all previously
      | added values.
      | 
      | If no values have been added yet, this
      | will return zero.
      |
      */
    pub fn get_variance(&self) -> FloatType {
        
        todo!();
        /*
            return count > 0 ? (sumSquares - sum * sum / (FloatType) count) / (FloatType) count
                             : FloatType();
        */
    }

    /**
      | Returns the standard deviation of all
      | previously added values.
      | 
      | If no values have been added yet, this
      | will return zero.
      |
      */
    pub fn get_standard_deviation(&self) -> FloatType {
        
        todo!();
        /*
            return std::sqrt (getVariance());
        */
    }

    /**
      | Returns the smallest of all previously
      | added values.
      | 
      | If no values have been added yet, this
      | will return positive infinity.
      |
      */
    pub fn get_min_value(&self) -> FloatType {
        
        todo!();
        /*
            return minimum;
        */
    }

    /**
      | Returns the largest of all previously
      | added values.
      | 
      | If no values have been added yet, this
      | will return negative infinity.
      |
      */
    pub fn get_max_value(&self) -> FloatType {
        
        todo!();
        /*
            return maximum;
        */
    }

    /**
      | Returns how many values have been added
      | to this accumulator.
      |
      */
    pub fn get_count(&self) -> usize {
        
        todo!();
        /*
            return count;
        */
    }
}
