crate::ix!();

/// Double-precision floating point NEON intrinsics
/// emulation in Rust
///
/// Tags: DSP (Digital Signal Processing)
///
pub struct SimdNeonF64;

impl SimdNeonF64 {

    /// The vectorized type represented as a 128-bit vector
    pub type VSimdType = [f64; 2];

    /// Expand a scalar to all elements of the vector
    pub fn expand(s: f64) -> Self::VSimdType {
        [s, s]
    }

    /// Load data from a pointer into a vector
    pub fn load(a: &[f64]) -> Self::VSimdType {
        [a[0], a[1]]
    }

    /// Store a vector's data into a pointer
    pub fn store(v: Self::VSimdType, a: &mut [f64]) {
        a[0] = v[0];
        a[1] = v[1];
    }

    /// Retrieve element at index `i` from the vector
    pub fn get(v: Self::VSimdType, i: usize) -> f64 {
        v[i]
    }

    /// Set element at index `i` in the vector
    pub fn set(mut v: Self::VSimdType, i: usize, s: f64) -> Self::VSimdType {
        v[i] = s;
        v
    }

    // Implement basic operations (add, sub, mul) directly

    /// Element-wise addition
    pub fn add(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        [a[0] + b[0], a[1] + b[1]]
    }

    /// Element-wise subtraction
    pub fn sub(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        [a[0] - b[0], a[1] - b[1]]
    }

    /// Element-wise multiplication
    pub fn mul(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        [a[0] * b[0], a[1] * b[1]]
    }

    // Additional operations can use transmute to bit-wise operations
    // For demonstration, however, I'll leave this out for now due to Rust's
    // safety concerns related to transmuting floating-point types to integers for bit-wise ops.

    /// Sum of all elements
    pub fn sum(a: Self::VSimdType) -> f64 {
        a[0] + a[1]
    }

    /// Bitwise AND operation
    pub unsafe fn bit_and(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        let ai: [u64; 2] = std::mem::transmute(a);
        let bi: [u64; 2] = std::mem::transmute(b);
        std::mem::transmute([ai[0] & bi[0], ai[1] & bi[1]])
    }

    /// Bitwise OR operation
    pub unsafe fn bit_or(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        let ai: [u64; 2] = std::mem::transmute(a);
        let bi: [u64; 2] = std::mem::transmute(b);
        std::mem::transmute([ai[0] | bi[0], ai[1] | bi[1]])
    }

    /// Bitwise XOR operation
    pub unsafe fn bit_xor(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        let ai: [u64; 2] = std::mem::transmute(a);
        let bi: [u64; 2] = std::mem::transmute(b);
        std::mem::transmute([ai[0] ^ bi[0], ai[1] ^ bi[1]])
    }

    /// Bitwise NOT operation
    pub unsafe fn bit_not(a: Self::VSimdType) -> Self::VSimdType {
        let ai: [u64; 2] = std::mem::transmute(a);
        std::mem::transmute([!ai[0], !ai[1]])
    }

    /// Bitwise NOT AND operation
    pub unsafe fn bit_notand(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        let ai: [u64; 2] = std::mem::transmute(a);
        let bi: [u64; 2] = std::mem::transmute(b);
        std::mem::transmute([!ai[0] & bi[0], !ai[1] & bi[1]])
    }

    /// Element-wise min
    pub fn min(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        [a[0].min(b[0]), a[1].min(b[1])]
    }

    /// Element-wise max
    pub fn max(a: Self::VSimdType, b: Self::VSimdType) -> Self::VSimdType {
        [a[0].max(b[0]), a[1].max(b[1])]
    }
}
