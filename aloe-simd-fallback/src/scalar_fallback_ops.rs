crate::ix!();

pub trait ScalarOperation {
    type ScalarType;
    fn op(a: Self::ScalarType, b: Self::ScalarType) -> Self::ScalarType;
}

pub trait MaskOperation {
    type MaskType;
    fn op(a: Self::MaskType, b: Self::MaskType) -> Self::MaskType;
}

pub trait ComparisonOperation {
    type ScalarType;
    fn op(a: Self::ScalarType, b: Self::ScalarType) -> bool;
}

pub struct ScalarAdd;
impl ScalarOperation for ScalarAdd {
    type ScalarType = f64; // Or any other scalar type
    #[inline(always)]
    fn op(a: f64, b: f64) -> f64 {
        a + b
    }
}

pub struct ScalarSub;
impl ScalarOperation for ScalarSub {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> f64 {
        a - b
    }
}

pub struct ScalarAnd;
impl MaskOperation for ScalarAnd {
    type MaskType = u64;
    #[inline(always)]
    fn op(a: u64, b: u64) -> u64 {
        a & b
    }
}

pub struct ScalarEq;
impl ComparisonOperation for ScalarEq {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> bool {
        a == b
    }
}

// Scalar Operations
pub struct ScalarMul;
impl ScalarOperation for ScalarMul {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> f64 {
        a * b
    }
}

pub struct ScalarMin;
impl ScalarOperation for ScalarMin {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> f64 {
        a.min(b)
    }
}

pub struct ScalarMax;
impl ScalarOperation for ScalarMax {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> f64 {
        a.max(b)
    }
}

// Mask Operations
pub struct ScalarOr;
impl MaskOperation for ScalarOr {
    type MaskType = u64;
    #[inline(always)]
    fn op(a: u64, b: u64) -> u64 {
        a | b
    }
}

pub struct ScalarXor;
impl MaskOperation for ScalarXor {
    type MaskType = u64;
    #[inline(always)]
    fn op(a: u64, b: u64) -> u64 {
        a ^ b
    }
}

pub struct ScalarNot;
impl MaskOperation for ScalarNot {
    type MaskType = u64;
    #[inline(always)]
    fn op(a: u64, b: u64) -> u64 {
        !a & b
    }
}

// Comparison Operations
pub struct ScalarNeq;
impl ComparisonOperation for ScalarNeq {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> bool {
        a != b
    }
}

pub struct ScalarGt;
impl ComparisonOperation for ScalarGt {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> bool {
        a > b
    }
}

pub struct ScalarGeq;
impl ComparisonOperation for ScalarGeq {
    type ScalarType = f64;
    #[inline(always)]
    fn op(a: f64, b: f64) -> bool {
        a >= b
    }
}
