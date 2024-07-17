crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_sse_SIMDNativeOps.cpp]

#[repr(align(16))]
pub struct Align16<T>(pub T);

// int32_t and float
pub struct I32FloatSimd4(pub Align16<[i32; 4]>);

impl I32FloatSimd4 {
    pub const ALL_BITS_SET: Align16<[i32; 4]> = Align16([-1, -1, -1, -1]);
    pub const EVEN_HIGH_BIT: Align16<[i32; 4]> = Align16(
        [
            i32::from_ne_bytes([0x80, 0, 0, 0]),
            0,
            i32::from_ne_bytes([0x80, 0, 0, 0]),
            0,
        ]
    );
}

//----------------------------------------------------------
pub struct F32Simd4(pub Align16<[f32; 4]>);
impl F32Simd4 {
    pub const ONE: Align16<[f32; 4]> = Align16([1.0; 4]);
}

// int64_t and double
pub struct I64DoubleSimd2(pub Align16<[i64; 2]>);
impl I64DoubleSimd2 {
    pub const ALL_BITS_SET: Align16<[i64; 2]> = Align16([-1, -1]);
    pub const EVEN_HIGH_BIT: Align16<[i64; 2]> = Align16(
        [
        i64::from_ne_bytes([0x80, 0, 0, 0, 0, 0, 0, 0]),
        0
        ]
    );
}
pub struct F64Simd2(pub Align16<[f64; 2]>);
impl F64Simd2 {
    pub const ONE: Align16<[f64; 2]> = Align16([1.0; 2]);
}

// int8_t
pub struct I8Simd16(pub Align16<[i8; 16]>);
impl I8Simd16 {
    pub const ALL_BITS_SET: Align16<[i8; 16]> = Align16([-1; 16]);
}

// uint8_t
pub struct U8Simd16(pub Align16<[u8; 16]>);
impl U8Simd16 {
    pub const ALL_BITS_SET: Align16<[u8; 16]> = Align16([0xff; 16]);
    pub const HIGH_BIT: Align16<[u8; 16]> = Align16([0x80; 16]);
}

// int16_t
pub struct I16Simd8(pub Align16<[i16; 8]>);
impl I16Simd8 {
    pub const ALL_BITS_SET: Align16<[i16; 8]> = Align16([-1; 8]);
}

// uint16_t
pub struct U16Simd8(pub Align16<[u16; 8]>);
impl U16Simd8 {
    pub const ALL_BITS_SET: Align16<[u16; 8]> = Align16([0xffff; 8]);
    pub const HIGH_BIT: Align16<[u16; 8]> = Align16([0x8000; 8]);
}

// int32_t
pub struct I32Simd4(pub Align16<[i32; 4]>);
impl I32Simd4 {
    pub const ALL_BITS_SET: Align16<[i32; 4]> = Align16([-1; 4]);
}

// uint32_t
pub struct U32Simd4(pub Align16<[u32; 4]>);
impl U32Simd4 {
    pub const ALL_BITS_SET: Align16<[u32; 4]> = Align16([0xffffffff; 4]);
    pub const HIGH_BIT: Align16<[u32; 4]> = Align16([0x80000000; 4]);
}

// int64_t
pub struct I64Simd2(pub Align16<[i64; 2]>);
impl I64Simd2 {
    pub const ALL_BITS_SET: Align16<[i64; 2]> = Align16([-1; 2]);
}

// uint64_t
pub struct U64Simd2(pub Align16<[u64; 2]>);
impl U64Simd2 {
    pub const ALL_BITS_SET: Align16<[u64; 2]> = Align16([0xffffffffffffffff; 2]);
    pub const HIGH_BIT: Align16<[u64; 2]> = Align16([0x8000000000000000; 2]);
}
