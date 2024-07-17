#[macro_use] mod imports; use imports::*;

// NOTE: the translation machine is x86_64 
//
// there are probably a few compiler errors in here, but
// nothing too major
//
#[cfg(target_arch = "aarch64")] x!{neon_constants}
#[cfg(target_arch = "aarch64")] x!{neon_f32}
#[cfg(target_arch = "aarch64")] x!{neon_f64}
#[cfg(target_arch = "aarch64")] x!{neon_i16}
#[cfg(target_arch = "aarch64")] x!{neon_i32}
#[cfg(target_arch = "aarch64")] x!{neon_i64}
#[cfg(target_arch = "aarch64")] x!{neon_i8}
#[cfg(target_arch = "aarch64")] x!{neon_u16}
#[cfg(target_arch = "aarch64")] x!{neon_u32}
#[cfg(target_arch = "aarch64")] x!{neon_u64}
#[cfg(target_arch = "aarch64")] x!{neon_u8}
