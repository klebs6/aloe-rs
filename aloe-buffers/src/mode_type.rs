crate::ix!();

///-------------------------
#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
pub struct ModeType<const typeSize: i32> {

}

#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
impl<const typeSize: i32> HasMode for ModeType<typeSize> {
    type Mode = BasicOps32;
}

#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
impl HasMode for ModeType<8> {
    type Mode = BasicOps64;
}

