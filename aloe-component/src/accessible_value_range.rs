crate::ix!();

/**
  | Represents the range of this value,
  | if supported.
  | 
  | Return one of these from the `getRange()`
  | method, providing a minimum, maximum,
  | and interval value for the range to indicate
  | that this is a ranged value.
  | 
  | The default state is an "invalid" range,
  | indicating that the accessibility
  | element does not support ranged values.
  | 
  | @see AccessibilityRangedNumericValueInterface
  | 
  | @tags{Accessibility}
  |
  */
pub struct AccessibleValueRange {
    valid:     bool, // default = false
    range:     MinAndMax,
    step_size: f64, // default = 0.0
}


impl Default for AccessibleValueRange {
    
    /**
      | Constructor.
      | 
      | Creates a default, "invalid" range
      | that can be returned from `AccessibilityValueInterface::getRange()`
      | to indicate that the value interface
      | does not support ranged values.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl AccessibleValueRange {

    /**
      | Constructor.
      | 
      | Creates a valid AccessibleValueRange
      | with the provided minimum, maximum,
      | and interval values.
      |
      */
    pub fn new(
        value_range: MinAndMax,
        interval:    f64) -> Self {
    
        todo!();
        /*
            : valid (true),
                  range (valueRange),
                  stepSize (interval)

                jassert (range.min < range.max);
        */
    }

    /**
      | Returns true if this represents a valid
      | range.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return valid;
        */
    }

    /**
      | Returns the minimum value for this range.
      |
      */
    pub fn get_minimum_value(&self) -> f64 {
        
        todo!();
        /*
            return range.min;
        */
    }

    /**
      | Returns the maxiumum value for this
      | range.
      |
      */
    pub fn get_maximum_value(&self) -> f64 {
        
        todo!();
        /*
            return range.max;
        */
    }

    /**
      | Returns the interval for this range.
      |
      */
    pub fn get_interval(&self) -> f64 {
        
        todo!();
        /*
            return stepSize;
        */
    }
}
