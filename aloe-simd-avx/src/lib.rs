#[macro_use] mod imports; use imports::*;

// NOTE: the translation machine is x86_64 
//
// there are probably a few compiler errors in here, but
// nothing too major
//
#[cfg(target_feature = "avx")] x!{avx_constants}
#[cfg(target_feature = "avx")] x!{avx_double}
#[cfg(target_feature = "avx")] x!{avx_float}
#[cfg(target_feature = "avx")] x!{avx_i16}
#[cfg(target_feature = "avx")] x!{avx_i32}
#[cfg(target_feature = "avx")] x!{avx_i64}
#[cfg(target_feature = "avx")] x!{avx_i8}
#[cfg(target_feature = "avx")] x!{avx_u16}
#[cfg(target_feature = "avx")] x!{avx_u32}
#[cfg(target_feature = "avx")] x!{avx_u64}
#[cfg(target_feature = "avx")] x!{avx_u8}
