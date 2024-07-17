crate::ix!();

/**
  | A namespace containing a set of types
  | used for specifying the smoothing behaviour
  | of the SmoothedValue class.
  | 
  | For example: @code SmoothedValue<float,
  | ValueSmoothingTypes::Multiplicative>
  | frequency (1.0f); @endcode
  |
  */
pub mod value_smoothing_types {

    use super::*;

    /**
      | Used to indicate a linear smoothing
      | between values.
      | 
      | @tags{Audio}
      |
      */
    pub struct Linear {}

    /**
      | Used to indicate a smoothing between
      | multiplicative values.
      | 
      | @tags{Audio}
      |
      */
    pub struct Multiplicative {}
}

pub type LinearSmoothedValue<FloatType: num::Float> = SmoothedValue<FloatType, value_smoothing_types::Linear>;
pub type MultiplicativeSmoothedValue<FloatType: num::Float> = SmoothedValue<FloatType, value_smoothing_types::Multiplicative>;
