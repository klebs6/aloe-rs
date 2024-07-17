crate::ix!();

// Define a trait with an associated type to represent the mask type for a given primitive.
pub trait HasMaskType {
    type Mask;
}

// Implement the trait for each primitive type.
impl HasMaskType for f32 {
    type Mask = u32;
}

impl HasMaskType for f64 {
    type Mask = u64;
}

impl HasMaskType for char {
    type Mask = u8;
}

impl HasMaskType for i8 {
    type Mask = u8;
}

impl HasMaskType for i16 {
    type Mask = u16;
}

impl HasMaskType for i32 {
    type Mask = u32;
}

impl HasMaskType for i64 {
    type Mask = u64;
}

impl HasMaskType for Complex<f32> {
    type Mask = u32;
}

impl HasMaskType for Complex<f64> {
    type Mask = u64;
}

// Define a trait to remove the complex part and find the underlying primitive type.
pub trait PrimitiveType {
    type Primitive;
}

/*
impl<T: Copy> PrimitiveType for T {
    type Primitive = T;
}
*/

impl<T: Copy> PrimitiveType for Complex<T> {
    type Primitive = T;
}

const fn log2_helper(n: usize, acc: usize) -> usize {
    if n == 1 {
        acc
    } else {
        log2_helper(n / 2, acc + 1)
    }
}

pub const fn log2(n: usize) -> usize {
    log2_helper(n, 0)
}
