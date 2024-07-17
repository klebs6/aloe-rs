crate::ix!();

#[cfg(ALOE_USE_SSE_INTRINSICS)]
pub fn is_aligned(p: *const c_void) -> bool {
    
    todo!();
    /*
        return (((pointer_sized_int) p) & 15) == 0;
    */
}
