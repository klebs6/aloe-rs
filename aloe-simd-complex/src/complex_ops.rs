crate::ix!();

/// Trait for complex operations
pub trait CmplxExpand<T, S> {
    fn expand(scalar: S) -> T;
}

pub trait CmplxMul<T> {
    fn mul(a: T, b: T) -> T;
}
