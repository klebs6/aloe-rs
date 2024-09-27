crate::ix!();

use std::simd::{Simd, prelude::{SimdPartialEq,SimdPartialOrd,SimdUint}};

pub struct SimdNativeU8;

impl SimdNativeU8 {
    // Constants
    const HIGH_BIT:     u8 = 0x80;
    const ALL_BITS_SET: u8 = 0xFF;

    // Load a SIMD vector from memory
    pub fn vconst(a: &[u8; 16]) -> u8x16 {
        u8x16::from_slice(a)
    }

    // Sign-flip each byte
    pub fn ssign(a: u8x16) -> u8x16 {
        a ^ Self::vconst(&[Self::HIGH_BIT; 16])
    }

    // Store a SIMD vector to memory
    pub fn store(v: u8x16, p: &mut [u8; 16]) {
        p.copy_from_slice(v.as_array());
    }

    // Create a SIMD vector with all lanes set to `s`
    pub fn expand(s: u8) -> u8x16 {
        u8x16::splat(s)
    }

    // Arithmetic and Bitwise Operations
    pub fn add(a: u8x16, b: u8x16) -> u8x16 { a + b }
    pub fn sub(a: u8x16, b: u8x16) -> u8x16 { a - b }
    pub fn bit_and(a: u8x16, b: u8x16) -> u8x16 { a & b }
    pub fn bit_or(a: u8x16, b: u8x16) -> u8x16 { a | b }
    pub fn bit_xor(a: u8x16, b: u8x16) -> u8x16 { a ^ b }
    pub fn bit_andnot(a: u8x16, b: u8x16) -> u8x16 { !a & b }
    pub fn bit_not(a: u8x16) -> u8x16 { !a }

    // Comparison Operations
    pub fn min(a: u8x16, b: u8x16) -> u8x16 { a.min(b) }
    pub fn max(a: u8x16, b: u8x16) -> u8x16 { a.max(b) }

    pub fn equal(a: u8x16, b: u8x16) -> m8x16 {
        a.simd_eq(b)
    }

    pub fn greater_than(a: u8x16, b: u8x16) -> m8x16 {
        a.simd_gt(b)
    }

    pub fn greater_than_or_equal(a: u8x16, b: u8x16) -> m8x16 {
        a.simd_ge(b)
    }

    // Multiply-Add
    pub fn multiply_add(a: u8x16, b: u8x16, c: u8x16) -> u8x16 {
        Self::add(a, Self::mul(b, c))
    }

    pub fn mask_not(a: m8x16) -> m8x16 {
        !a // Logical NOT for mask type
    }

    pub fn not_equal(a: u8x16, b: u8x16) -> m8x16 {
        Self::mask_not(Self::equal(a, b)) // Negate the result of equal comparison
    }

    // All-Equal
    pub fn all_equal(a: u8x16, b: u8x16) -> bool {
        Self::equal(a, b).all() // Returns true if all comparisons are true
    }

    // Get an element from a SIMD vector
    pub fn get(a: u8x16, i: usize) -> u8 {
        a[i]
    }

    // Set an element in a SIMD vector
    pub fn set(a: u8x16, i: usize, s: u8) -> u8x16 {
        let mut result = a;
        result[i] = s;
        result
    }

    // Truncate: in this case, it's a no-op as the vector is already of type u8x16
    pub fn truncate(a: u8x16) -> u8x16 {
        a
    }

    // Sum of elements in a SIMD vector
    pub fn sum(a: u8x16) -> u8 {
        a.to_array().iter().copied().sum()
    }

    // Multiply elements in SIMD vectors
    pub fn mul(a: u8x16, b: u8x16) -> u8x16 {
        // No direct 8-bit multiplication in std-simd, need to handle manually
        let even = (a & u8x16::splat(0xF)) * (b & u8x16::splat(0xF));
        let odd = ((a >> 4) * (b >> 4)) << 4;
        even | odd
    }
}
