crate::ix!();

pub const STEINBERG_MAX_LONG:    i32 = 0x7fffffff;
pub const STEINBERG_MIN_LONG:    i32 = -0x7fffffff - 1;
pub const STEINBERG_MAX_INT32:   i32 = STEINBERG_MAX_LONG;
pub const STEINBERG_MIN_INT32:   i32 = STEINBERG_MIN_LONG;
pub const STEINBERG_MAX_INT_32U: u32 = 0xffffffff;

#[cfg(all(SMTG_OS_WINDOWS,not(__GNUC__)))] 
pub const STEINBERG_MAX_INT64: i64 = 9223372036854775807;

#[cfg(all(SMTG_OS_WINDOWS,not(__GNUC__)))] 
pub const STEINBERG_MIN_INT64: i64 = -9223372036854775807 - 1;

#[cfg(not(all(SMTG_OS_WINDOWS,not(__GNUC__))))] 
pub const STEINBERG_MAX_INT64: i64 = 0x7fffffffffffffff;

#[cfg(not(all(SMTG_OS_WINDOWS,not(__GNUC__))))] 
pub const STEINBERG_MIN_INT64: i64 = -0x7fffffffffffffff - 1;

///--------------------------
pub const STEINBERG_MAX_INT64U: u64 = (0xffffffff as u64) | ((0xffffffff as u64) << 32);

pub const STEINBERG_MAX_FLOAT:  f32 = 3.40282346638528860E38;
pub const STEINBERG_MAX_DOUBLE: f64 = 1.7976931348623158E308;
