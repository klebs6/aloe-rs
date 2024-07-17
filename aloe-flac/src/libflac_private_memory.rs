crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/memory.h]

/**
  | Returns the unaligned address returned
  | by malloc.
  | 
  | Use free() on this address to deallocate.
  |
  */
pub fn flac_memory_alloc_aligned(
        bytes:           usize,
        aligned_address: *mut *mut c_void)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_memory_alloc_aligned_int32_array(
        elements:          usize,
        unaligned_pointer: *mut *mut i32,
        aligned_pointer:   *mut *mut i32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_memory_alloc_aligned_uint32_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u32,
        aligned_pointer:   *mut *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_memory_alloc_aligned_uint64_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u64,
        aligned_pointer:   *mut *mut u64) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_memory_alloc_aligned_unsigned_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u32,
        aligned_pointer:   *mut *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_memory_alloc_aligned_real_array(
        elements:          usize,
        unaligned_pointer: *mut *mut real,
        aligned_pointer:   *mut *mut real) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn safe_malloc_mul_2op_p(
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
        
        */
}
