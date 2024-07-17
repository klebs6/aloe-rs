#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#[macro_use] mod imports; use imports::*;

x!{scalar_fallback_ops}
#[cfg(target_arch = "x86_64")] x!{simd_fallback_helper}
#[cfg(target_arch = "x86_64")] x!{simd_fallback_ops}
#[cfg(target_arch = "x86_64")] x!{simd_fallback_utility}
