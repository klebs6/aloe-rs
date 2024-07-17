#![feature(inherent_associated_types)]
#[macro_use] mod imports; use imports::*;

x!{native_ops}
#[cfg(target_arch = "x86_64")] x!{simd_native_double}
#[cfg(target_arch = "x86_64")] x!{simd_native_float}
#[cfg(target_arch = "x86_64")] x!{simd_native_i16}
#[cfg(target_arch = "x86_64")] x!{simd_native_i32}
#[cfg(target_arch = "x86_64")] x!{simd_native_i64}
#[cfg(target_arch = "x86_64")] x!{simd_native_i8}
#[cfg(target_arch = "x86_64")] x!{simd_native_u16}
#[cfg(target_arch = "x86_64")] x!{simd_native_u32}
#[cfg(target_arch = "x86_64")] x!{simd_native_u64}
#[cfg(target_arch = "x86_64")] x!{simd_native_u8}
