crate::ix!();

// Define a trait for scalar operations
pub trait ScalarOp<T> {
    fn apply(a: T, b: T) -> T;
}

/*
// Implement ScalarOp for f32x4 addition
impl ScalarOp<f32x4> for f32x4 {
    fn apply(a: f32x4, b: f32x4) -> f32x4 {
        a + b
    }
}

// Implement ScalarOp for f32x4 subtraction
impl ScalarOp<f32x4> for f32x4 {
    fn apply(a: f32x4, b: f32x4) -> f32x4 {
        a - b
    }
}

// Implement ScalarOp for f32x4 multiplication
impl ScalarOp<f32x4> for f32x4 {
    fn apply(a: f32x4, b: f32x4) -> f32x4 {
        a * b
    }
}

// Implement ScalarOp for u32x4 bitwise AND
impl ScalarOp<u32x4> for u32x4 {
    fn apply(a: u32x4, b: u32x4) -> u32x4 {
        a & b
    }
}

// Implement ScalarOp for u32x4 bitwise OR
impl ScalarOp<u32x4> for u32x4 {
    fn apply(a: u32x4, b: u32x4) -> u32x4 {
        a | b
    }
}

// Implement ScalarOp for u32x4 bitwise XOR
impl ScalarOp<u32x4> for u32x4 {
    fn apply(a: u32x4, b: u32x4) -> u32x4 {
        a ^ b
    }
}
*/

// Utility function to apply operations
pub fn apply<T, O: ScalarOp<T>>(a: T, b: T) -> T {
    O::apply(a, b)
}

// The main SIMD utility struct
pub struct VSIMDUtility;

impl VSIMDUtility {
    /// Add two SIMD vectors
    pub fn add(a: f32x4, b: f32x4) -> f32x4 {
        a.add(b)
    }

    /// Subtract two SIMD vectors
    pub fn sub(a: f32x4, b: f32x4) -> f32x4 {
        a.sub(b)
    }

    /// Multiply two SIMD vectors
    pub fn mul(a: f32x4, b: f32x4) -> f32x4 {
        a.mul(b)
    }

    /// Bitwise AND two SIMD vectors
    pub fn bit_and(a: u32x4, b: u32x4) -> u32x4 {
        a.bitand(b)
    }

    /// Bitwise OR two SIMD vectors
    pub fn bit_or(a: u32x4, b: u32x4) -> u32x4 {
        a.bitor(b)
    }

    /// Bitwise XOR two SIMD vectors
    pub fn bit_xor(a: u32x4, b: u32x4) -> u32x4 {
        a.bitxor(b)
    }

    /// Minimum of two SIMD vectors
    pub fn min(a: f32x4, b: f32x4) -> f32x4 {
        a.simd_min(b)
    }

    /// Maximum of two SIMD vectors
    pub fn max(a: f32x4, b: f32x4) -> f32x4 {
        a.simd_max(b)
    }
}

#[test] fn test_simd_fallback_utility() {
    let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
    let b = f32x4::new(5.0, 6.0, 7.0, 8.0);

    let result = VSIMDUtility::add(a, b);
    println!("{:?}", result);
}
