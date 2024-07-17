#[macro_use] mod imports; use imports::*;

#[cfg(target_arch = "x86_64")] x!{simd_constants}
#[cfg(target_arch = "x86_64")] x!{simd_internal}
