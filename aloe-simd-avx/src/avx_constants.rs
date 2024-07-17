crate::ix!();

// Float constants
#[repr(align(32))]
pub struct FloatAligned(pub [f32; 8]);
impl FloatAligned {
    pub const ALL_BITS_SET: FloatAligned = FloatAligned([-1.0; 8]);
    pub const EVEN_HIGH_BIT: FloatAligned = FloatAligned([-0.0, 0.0, -0.0, 0.0, -0.0, 0.0, -0.0, 0.0]);
    pub const ONE: FloatAligned = FloatAligned([1.0; 8]);
}

// Double constants
#[repr(align(32))]
pub struct DoubleAligned(pub [f64; 4]);
impl DoubleAligned {
    pub const ALL_BITS_SET: DoubleAligned = DoubleAligned([-1.0; 4]);
    pub const EVEN_HIGH_BIT: DoubleAligned = DoubleAligned([-0.0, 0.0, -0.0, 0.0]);
    pub const ONE: DoubleAligned = DoubleAligned([1.0; 4]);
}

// int8_t constants
#[repr(align(32))]
pub struct Int8Aligned(pub [i8; 32]);
impl Int8Aligned {
    pub const ALL_BITS_SET: Int8Aligned = Int8Aligned([-1; 32]);
}

// uint8_t constants
#[repr(align(32))]
pub struct UInt8Aligned(pub [u8; 32]);
impl UInt8Aligned {
    pub const ALL_BITS_SET: UInt8Aligned = UInt8Aligned([0xFF; 32]);
    pub const HIGH_BIT: UInt8Aligned = UInt8Aligned([0x80; 32]);
}

// int16_t constants
#[repr(align(32))]
pub struct Int16Aligned(pub [i16; 16]);
impl Int16Aligned {
    pub const ALL_BITS_SET: Int16Aligned = Int16Aligned([-1; 16]);
}

// uint16_t constants
#[repr(align(32))]
pub struct UInt16Aligned(pub [u16; 16]);
impl UInt16Aligned {
    pub const ALL_BITS_SET: UInt16Aligned = UInt16Aligned([0xFFFF; 16]);
    pub const HIGH_BIT: UInt16Aligned = UInt16Aligned([0x8000; 16]);
}

// int32_t constants
#[repr(align(32))]
pub struct Int32Aligned(pub [i32; 8]);
impl Int32Aligned {
    pub const ALL_BITS_SET: Int32Aligned = Int32Aligned([-1; 8]);
}

// uint32_t constants
#[repr(align(32))]
pub struct UInt32Aligned(pub [u32; 8]);
impl UInt32Aligned {
    pub const ALL_BITS_SET: UInt32Aligned = UInt32Aligned([0xFFFFFFFF; 8]);
    pub const HIGH_BIT: UInt32Aligned = UInt32Aligned([0x80000000; 8]);
}

// int64_t constants
#[repr(align(32))]
pub struct Int64Aligned(pub [i64; 4]);
impl Int64Aligned {
    pub const ALL_BITS_SET: Int64Aligned = Int64Aligned([-1; 4]);
}

// uint64_t constants
#[repr(align(32))]
pub struct UInt64Aligned(pub [u64; 4]);
impl UInt64Aligned {
    pub const ALL_BITS_SET: UInt64Aligned = UInt64Aligned([0xFFFFFFFFFFFFFFFF; 4]);
    pub const HIGH_BIT: UInt64Aligned = UInt64Aligned([0x8000000000000000; 4]);
}
