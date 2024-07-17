crate::ix!();

pub trait HasParentType {
    type Type;
}

pub trait HasNumericType {
    type Type;
}

pub trait HasSampleType {
    type SampleType: SampleTypeInterface;
}

//-----------------------
pub trait SampleTypeInterface: HasElementType {}

impl<T: num::Float + HasElementType> SampleTypeInterface for T {}

impl HasElementType for f32 { type Type = f32; }
impl HasElementType for f64 { type Type = f64; }

//-----------------------
pub trait HasElementType {
    type Type;
}

pub trait FloatSample = SampleTypeInterface + num::Float;

/*
impl<T: SampleTypeInterface> HasElementType for T {
    type Type = T;
}
*/

impl<T: HasElementType> HasNumericType for T {

    /**
      | The NumericType is the underlying primitive
      | type used by the SampleType (which could
      | be either a primitive or vector)
      |
      */
    type Type = <Self as HasElementType>::Type;
}

pub type NumericType<T: HasNumericType> = <T as HasNumericType>::Type;

#[cfg(ALOE_USE_SIMD)]
impl<T> HasElementType for SIMDRegister<T> {
    type Type = T;
}
