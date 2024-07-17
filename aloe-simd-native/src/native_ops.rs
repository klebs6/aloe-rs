crate::ix!();

/// Trait for native operations
pub trait NativeOps<T, M=T> {
    fn add(a: T, b: T) -> T;
    fn sub(a: T, b: T) -> T;
    fn bit_and(a: T, b: M) -> T;
    fn bit_or(a: T, b: M) -> T;
    fn bit_xor(a: T, b: M) -> T;
    fn to_vec_type(mask: M) -> T;
    fn to_vec_type_scalar(scalar: M) -> T;
}
