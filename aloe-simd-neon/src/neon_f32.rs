crate::ix!();

/// Struct SimdNeonFloat encapsulating single-precision
/// floating-point NEON intrinsics.
///
pub struct SimdNeonFloat;

impl SimdNeonFloat {

    // Constants instead of static variables
    pub const ALLBITSSET: i32x4 = i32x4::new(-1, -1, -1, -1);
    pub const EVENHIGHBIT: i32x4 = i32x4::new(0x80000000, 0, 0x80000000, 0);
    pub const ONE: f32x4 = f32x4::new(1.0, 1.0, 1.0, 1.0);

    pub type VSIMDType = f32x4;
    pub type VMASKType = u32x4;

    /// Expand scalar into SIMD register
    #[inline(always)]
    pub fn expand(s: f32) -> Self::VSIMDType {
        Self::VSIMDType::splat(s)
    }

    /// Load data from memory into SIMD register
    #[inline(always)]
    pub fn load(a: &[f32; 4]) -> Self::VSIMDType {
        Self::VSIMDType::from_array(*a)
    }

    /// Retrieve element from SIMD register
    #[inline(always)]
    pub fn get(v: Self::VSIMDType, i: usize) -> f32 {
        v.extract(i)
    }

    /// Set element of SIMD register
    #[inline(always)]
    pub fn set(v: Self::VSIMDType, i: usize, s: f32) -> Self::VSIMDType {
        v.replace(i, s)
    }

    /// Store SIMD register into memory
    #[inline(always)]
    pub fn store(v: Self::VSIMDType, a: &mut [f32; 4]) {
        v.write_to_slice_unaligned(a);
    }

    /// Sum the elements of a SIMD register
    #[inline(always)]
    pub fn sum(a: Self::VSIMDType) -> f32 {
        a.sum()
    }

    /// Arithmetic Operations
    #[inline(always)]
    pub fn add(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        a + b
    }

    #[inline(always)]
    pub fn sub(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        a - b
    }

    #[inline(always)]
    pub fn mul(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        a * b
    }

    /// Complex multiplication for SIMD vectors
    #[inline(always)]
    pub fn cmplxmul(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let rr_ir = a * b.shuffle([0, 0, 2, 2]);
        let ii_ri = a.shuffle([1, 0, 3, 2]) * b.shuffle([1, 1, 3, 3]);
        rr_ir + (ii_ri ^ unsafe { transmute(KEVENHIGHBIT_I32_F32) })
    }

    // Bitwise Operations
    #[inline(always)]
    pub fn bit_and(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask_a: Self::VMASKType = unsafe { transmute(a) };
        let mask_b: Self::VMASKType = unsafe { transmute(b) };
        unsafe { transmute(mask_a & mask_b) }
    }

    #[inline(always)]
    pub fn bit_or(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask_a: Self::VMASKType = unsafe { transmute(a) };
        let mask_b: Self::VMASKType = unsafe { transmute(b) };
        unsafe { transmute(mask_a | mask_b) }
    }

    #[inline(always)]
    pub fn bit_xor(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask_a: Self::VMASKType = unsafe { transmute(a) };
        let mask_b: Self::VMASKType = unsafe { transmute(b) };
        unsafe { transmute(mask_a ^ mask_b) }
    }

    #[inline(always)]
    pub fn bit_notand(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask_a: Self::VMASKType = unsafe { transmute(a) };
        let mask_b: Self::VMASKType = unsafe { transmute(b) };
        unsafe { transmute(mask_a & !mask_b) }
    }

    #[inline(always)]
    pub fn bit_not(a: Self::VSIMDType) -> Self::VSIMDType {
        let mask_a: Self::VMASKType = unsafe { transmute(a) };
        let mask_not: Self::VMASKType = !mask_a;
        unsafe { transmute(mask_not) }
    }

    // Comparison Operations
    #[inline(always)]
    pub fn min(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        a.min(b)
    }

    #[inline(always)]
    pub fn max(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        a.max(b)
    }

    #[inline(always)]
    pub fn equal(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask = a.eq(b);
        unsafe { transmute(mask) }
    }

    #[inline(always)]
    pub fn not_equal(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask = a.ne(b);
        unsafe { transmute(mask) }
    }

    #[inline(always)]
    pub fn greater_than(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask = a.gt(b);
        unsafe { transmute(mask) }
    }

    #[inline(always)]
    pub fn greater_than_or_equal(a: Self::VSIMDType, b: Self::VSIMDType) -> Self::VSIMDType {
        let mask = a.ge(b);
        unsafe { transmute(mask) }
    }

    // Bitwise Operations
    #[inline(always)]
    pub fn bit_and(a: f32x4, b: f32x4) -> f32x4 {
        let mask_a: u32x4 = unsafe { transmute(a) };
        let mask_b: u32x4 = unsafe { transmute(b) };
        unsafe { transmute(mask_a & mask_b) }
    }

    // Comparison Operations
    #[inline(always)]
    pub fn min(a: f32x4, b: f32x4) -> f32x4 {
        a.min(b)
    }

    // Utility Functions
    #[inline(always)]
    pub fn load(ptr: *const f32) -> f32x4 {
        unsafe { f32x4::from_slice_unaligned_unchecked(&*ptr) }
    }

    #[inline(always)]
    pub fn store(self, ptr: *mut f32) {
        unsafe { f32x4::write_to_slice_unaligned_unchecked(self, &mut *ptr); }
    }

    #[inline(always)]
    pub fn sum(self) -> f32 {
        let arr: [f32; 4] = self.into();
        arr.iter().sum()
    }
}

