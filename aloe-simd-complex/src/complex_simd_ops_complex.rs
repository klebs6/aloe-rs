crate::ix!();

// The pure complex version
pub trait SIMDComplexOps: SIMDNativeOps {
    fn cmplx_mul(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType;
    fn odd_even_sum(a: Self::VSimdType) -> Self::VSimdType;
}

pub struct CmplxSIMDOpsComplex<Ops>
where
    Ops: SIMDComplexOps,
{
    _marker: std::marker::PhantomData<Ops>,
}

impl<Ops> CmplxSIMDOpsComplex<Ops>
where
    Ops: SIMDComplexOps,
{
    pub fn load(a: &[Complex<<Ops as SIMDNativeOps>::Scalar>]) -> Ops::VSimdType {
        unsafe {
            let ptr = a.as_ptr() as *const <Ops as SIMDNativeOps>::Scalar;
            Ops::load(std::slice::from_raw_parts(ptr, a.len() * 2))
        }
    }

    pub fn store(value: Ops::VSimdType, dest: &mut [Complex<<Ops as SIMDNativeOps>::Scalar>]) {
        unsafe {
            let ptr = dest.as_mut_ptr() as *mut <Ops as SIMDNativeOps>::Scalar;
            Ops::store(value, std::slice::from_raw_parts_mut(ptr, dest.len() * 2));
        }
    }

    pub fn expand(s: Complex<<Ops as SIMDNativeOps>::Scalar>) -> Ops::VSimdType {
        let mut scalars = [s.re; 2];  // Assuming 2-element SIMD for simplicity
        scalars[1] = s.im;
        Ops::load(&scalars)
    }

    pub fn get(v: Ops::VSimdType, i: usize) -> Complex<<Ops as SIMDNativeOps>::Scalar> {

        let j = i << 1;

        let vj  = Ops::get(v, j);
        let vj1 = Ops::get(v, j + 1);

        Complex::new(vj, vj1)
    }

    pub fn set(v: Ops::VSimdType, i: usize, s: Complex<<Ops as SIMDNativeOps>::Scalar>) -> Ops::VSimdType {
        let j = i << 1;
        Ops::set(Ops::set(v, j, s.re), j + 1, s.im)
    }

    pub fn sum(a: Ops::VSimdType) -> Complex<<Ops as SIMDNativeOps>::Scalar> {
        let result = Ops::odd_even_sum(a);
        let mut scalars = [<Ops as SIMDNativeOps>::Scalar::default(); 2]; // Assuming 2-element SIMD
        Ops::store(result, &mut scalars);
        Complex::new(scalars[0], scalars[1])
    }

    pub fn mul(a: Ops::VSimdType, b: Ops::VSimdType) -> Ops::VSimdType {
        Ops::cmplx_mul(a, b)
    }

    pub fn multiply_add(a: Ops::VSimdType, b: Ops::VSimdType, c: Ops::VSimdType) -> Ops::VSimdType {
        Ops::add(a, Self::mul(b, c))
    }
}
