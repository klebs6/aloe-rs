crate::ix!();

/// Calculate the logarithm base 2 at compile-time.
pub const fn log2_helper(n: usize) -> usize {
    if n <= 1 {
        0
    } else {
        log2_helper(n / 2) + 1
    }
}

/// Define the SIMD-related type parameters and associated helper types.
pub struct SIMDTraits<ScalarType: Copy, VSIMDType: Copy> {
    _marker: std::marker::PhantomData<(ScalarType, VSIMDType)>,
}

impl<ScalarType: Copy, VSIMDType: Copy> SIMDTraits<ScalarType, VSIMDType> {
    /// Number of scalar elements that fit into the SIMD type.
    pub const N: usize = mem::size_of::<VSIMDType>() / mem::size_of::<ScalarType>();
    
    /// Mask for modular arithmetic with n.
    pub const MASK: usize = Self::N - 1;
    
    /// Number of bits needed to represent `n` as a binary number.
    pub const BITS: usize = log2_helper(Self::N);
}

/// Union type to facilitate operations between SIMD and scalar types.
union UnionType<ScalarType: Copy, VSIMDType: Copy> 
where [(); SIMDTraits::<ScalarType, VSIMDType>::N]:
{
    v: VSIMDType,
    s: [ScalarType; SIMDTraits::<ScalarType, VSIMDType>::N],
}

/// Union type to facilitate operations between SIMD and mask types.
union UnionMaskType<ScalarType: Copy, MaskType: Copy, VSIMDType: Copy> 
where [(); SIMDTraits::<ScalarType, VSIMDType>::N]:
{
    v: VSIMDType,
    m: [MaskType; SIMDTraits::<ScalarType, VSIMDType>::N],
}
