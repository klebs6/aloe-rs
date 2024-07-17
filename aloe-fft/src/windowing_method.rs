crate::ix!();

/**
  | The windowing methods available.
  |
  */
pub enum WindowingMethod
{
    rectangular = 0,
    triangular,
    hann,
    hamming,
    blackman,
    blackmanHarris,
    flatTop,
    kaiser,
    numWindowingMethods
}

pub trait HasWindowingMethod {
    type WindowingMethod;
}

impl<FloatType: num::Float> HasWindowingMethod for WindowingFunction<FloatType> {
    type WindowingMethod = WindowingMethod;
}
