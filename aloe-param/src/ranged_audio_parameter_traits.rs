crate::ix!();

pub trait RangedAudioParameterInterface: GetNormalisableRange {}

pub trait GetNormalisableRange {

    /**
      | Returns the range of values that the
      | parameter can take.
      |
      */
    fn get_normalisable_range(&self) -> &NormalisableRange<f32>;
}
