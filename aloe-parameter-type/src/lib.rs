#![feature(associated_type_defaults)]

#[macro_use] mod imports; use imports::*;

pub trait HasParameters {

    type Parameters;
}

pub trait HasParametersPtr: HasParameters {

    type ParametersPtr = Rc<RefCell<<Self as HasParameters>::Parameters>>;
}

pub trait ParameterType {
    type Type;
}

/*
impl<T> ParameterType for T {
    type Type = &'static T;
}
*/

// For specific primitive types:
impl ParameterType for char {
    type Type = char;
}

impl ParameterType for u8 {
    type Type = u8;
}

impl ParameterType for i16 {
    type Type = i16;
}

impl ParameterType for u16 {
    type Type = u16;
}

impl ParameterType for i32 {
    type Type = i32;
}

impl ParameterType for u32 {
    type Type = u32;
}

impl ParameterType for i64 {
    type Type = i64;
}

impl ParameterType for u64 {
    type Type = u64;
}

impl ParameterType for bool {
    type Type = bool;
}

impl ParameterType for f32 {
    type Type = f32;
}

impl ParameterType for f64 {
    type Type = f64;
}

/*
impl<T> ParameterType for &T {
    type Type = &T;
}

impl<T> ParameterType for *const T {
    type Type = *const T;
}
*/

//-----------------------------------
pub trait HasFloatType {
    type FloatType;
}

pub trait SmallestFloatType {
    type Type;
}

impl SmallestFloatType for f32 {
    type Type = f32;
}

impl SmallestFloatType for f64 {
    type Type = f64;
}

//-----------------------------------
pub trait UnsignedTypeWithSize {
    type Type;
}

impl UnsignedTypeWithSize for i8 {
    type Type = u8;
}

impl UnsignedTypeWithSize for i16 {
    type Type = u16;
}

impl UnsignedTypeWithSize for i32 {
    type Type = u32;
}

impl UnsignedTypeWithSize for i64 {
    type Type = u64;
}
