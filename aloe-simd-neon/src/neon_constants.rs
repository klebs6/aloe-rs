crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_neon_SIMDNativeOps.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_neon_SIMDNativeOps.cpp]

/// Struct to encapsulate float-related constants.
pub struct NeonFloatOps;

/// Struct to encapsulate int8-related constants.
pub struct NeonInt8Ops;

/// Struct to encapsulate uint8-related constants.
pub struct NeonUInt8Ops;

/// Struct to encapsulate int16-related constants.
pub struct NeonInt16Ops;

/// Struct to encapsulate uint16-related constants.
pub struct NeonUInt16Ops;

/// Struct to encapsulate int32-related constants.
pub struct NeonInt32Ops;

/// Struct to encapsulate uint32-related constants.
pub struct NeonUInt32Ops;

/// Struct to encapsulate int64-related constants.
pub struct NeonInt64Ops;

/// Struct to encapsulate uint64-related constants.
pub struct NeonUInt64Ops;

impl NeonFloatOps {
    /// All bits set (-1) constant
    pub const ALL_BITS_SET: float32x4_t = unsafe { mem::transmute([0xFFFFFFFFu32; 4]) };
    /// Even high bit constant
    pub const EVEN_HIGH_BIT: float32x4_t = unsafe { mem::transmute([0x80000000, 0, 0x80000000, 0]) };
    /// Constant representing 1.0
    pub const ONE: float32x4_t = float32x4_t::new(1.0, 1.0, 1.0, 1.0);
}

impl NeonInt8Ops {
    /// All bits set (-1) constant
    pub const ALL_BITS_SET: int8x16_t = int8x16_t::new(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
}

impl NeonUInt8Ops {
    /// All bits set (0xFF) constant
    pub const ALL_BITS_SET: uint8x16_t = uint8x16_t::new(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF);
}

impl NeonInt16Ops {
    /// All bits set (-1) constant
    pub const ALL_BITS_SET: int16x8_t = int16x8_t::new(-1, -1, -1, -1, -1, -1, -1, -1);
}

impl NeonUInt16Ops {
    /// All bits set (0xFFFF) constant
    pub const ALL_BITS_SET: uint16x8_t = uint16x8_t::new(0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF);
}

impl NeonInt32Ops {
    /// All bits set (-1) constant
    pub const ALL_BITS_SET: int32x4_t = int32x4_t::new(-1, -1, -1, -1);
}

impl NeonUInt32Ops {
    /// All bits set (0xFFFFFFFF) constant
    pub const ALL_BITS_SET: uint32x4_t = uint32x4_t::new(0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF);
}

impl NeonInt64Ops {
    /// All bits set (-1) constant
    pub const ALL_BITS_SET: int64x2_t = int64x2_t::new(-1, -1);
}

impl NeonUInt64Ops {
    /// All bits set (0xFFFFFFFFFFFFFFFF) constant
    pub const ALL_BITS_SET: uint64x2_t = uint64x2_t::new(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF);
}
