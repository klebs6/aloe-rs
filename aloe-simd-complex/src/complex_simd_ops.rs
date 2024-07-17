crate::ix!();

pub trait SIMDNativeOps {
    type VSimdType: Copy;
    type Scalar: Copy + Default;

    fn load(a: &[Self::Scalar]) -> Self::VSimdType;
    fn store(value: Self::VSimdType, dest: &mut [Self::Scalar]);
    fn expand(s: Self::Scalar) -> Self::VSimdType;
    fn get(v: Self::VSimdType, i: usize) -> Self::Scalar;
    fn set(v: Self::VSimdType, i: usize, s: Self::Scalar) -> Self::VSimdType;
    fn sum(a: Self::VSimdType) -> Self::Scalar;
    fn mul(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType;
    fn add(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType;
    fn multiply_add(a: Self::VSimdType, b: Self::VSimdType, c: Self::VSimdType) -> Self::VSimdType;
}

/* This class is used internally by SIMDRegister to abstract away differences
   in operations which are different for complex and pure floating point types. */

// the pure floating-point version
pub struct CmplxSIMDOps<Ops>
where
    Ops: SIMDNativeOps,
{
    _marker: std::marker::PhantomData<Ops>,
}

impl<Ops> CmplxSIMDOps<Ops>
where
    Ops: SIMDNativeOps,
{
    pub fn load(a: &[<Ops as SIMDNativeOps>::Scalar]) -> Ops::VSimdType {

        Ops::load(a)
    }

    pub fn store(value: Ops::VSimdType, dest: &mut [<Ops as SIMDNativeOps>::Scalar]) {
        Ops::store(value, dest);
    }

    pub fn expand(s: <Ops as SIMDNativeOps>::Scalar) -> Ops::VSimdType {
        Ops::expand(s)
    }

    pub fn get(v: Ops::VSimdType, i: usize) -> <Ops as SIMDNativeOps>::Scalar {
        Ops::get(v, i)
    }

    pub fn set(v: Ops::VSimdType, i: usize, s: <Ops as SIMDNativeOps>::Scalar) -> Ops::VSimdType {
        Ops::set(v, i, s)
    }

    pub fn sum(a: Ops::VSimdType) -> <Ops as SIMDNativeOps>::Scalar {
        Ops::sum(a)
    }

    pub fn mul(a: Ops::VSimdType, b: Ops::VSimdType) -> Ops::VSimdType {
        Ops::mul(a, b)
    }

    pub fn multiply_add(a: Ops::VSimdType, b: Ops::VSimdType, c: Ops::VSimdType) -> Ops::VSimdType {
        Ops::multiply_add(a, b, c)
    }
}
