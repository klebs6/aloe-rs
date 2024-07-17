crate::ix!();

/**
  | The type of filter that can be used for
  | the oversampling processing.
  |
  */
pub enum OversamplingFilterType
{
    filterHalfBandFIREquiripple = 0,
    filterHalfBandPolyphaseIIR,
    numFilterTypes
}
