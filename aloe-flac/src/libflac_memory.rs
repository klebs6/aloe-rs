crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/memory.c]

pub fn flac_memory_alloc_aligned(
        bytes:           usize,
        aligned_address: *mut *mut c_void)  {
    
    todo!();
        /*
            void *x;

        ASSERT(0 != aligned_address);

    #ifdef ALIGN_MALLOC_DATA
        /* align on 32-byte (256-bit) boundary */
        x = safe_malloc_add_2op_(bytes, /*+*/31L);
        *aligned_address = (void*)(((uintptr_t)x + 31L) & -32L);
    #else
        x = safe_malloc_(bytes);
        *aligned_address = x;
    #endif
        return x;
        */
}

pub fn flac_memory_alloc_aligned_int32_array(
        elements:          usize,
        unaligned_pointer: *mut *mut i32,
        aligned_pointer:   *mut *mut i32) -> bool {
    
    todo!();
        /*
            i32 *pu; /* unaligned pointer */
        union { /* union needed to comply with C99 pointer aliasing rules */
            i32 *pa; /* aligned pointer */
            void        *pv; /* aligned pointer alias */
        } u;

        ASSERT(elements > 0);
        ASSERT(0 != unaligned_pointer);
        ASSERT(0 != aligned_pointer);
        ASSERT(unaligned_pointer != aligned_pointer);

        if(elements > SIZE_MAX / sizeof(*pu)) /* overflow check */
            return false;

        pu = (i32*) memory_alloc_aligned(sizeof(*pu) * elements, &u.pv);
        if(0 == pu) {
            return false;
        }
        else {
            if(*unaligned_pointer != 0)
                free(*unaligned_pointer);
            *unaligned_pointer = pu;
            *aligned_pointer = u.pa;
            return true;
        }
        */
}

pub fn flac_memory_alloc_aligned_uint32_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u32,
        aligned_pointer:   *mut *mut u32) -> bool {
    
    todo!();
        /*
            u32 *pu; /* unaligned pointer */
        union { /* union needed to comply with C99 pointer aliasing rules */
            u32 *pa; /* aligned pointer */
            void         *pv; /* aligned pointer alias */
        } u;

        ASSERT(elements > 0);
        ASSERT(0 != unaligned_pointer);
        ASSERT(0 != aligned_pointer);
        ASSERT(unaligned_pointer != aligned_pointer);

        if(elements > SIZE_MAX / sizeof(*pu)) /* overflow check */
            return false;

        pu = (u32*) memory_alloc_aligned(sizeof(*pu) * elements, &u.pv);
        if(0 == pu) {
            return false;
        }
        else {
            if(*unaligned_pointer != 0)
                free(*unaligned_pointer);
            *unaligned_pointer = pu;
            *aligned_pointer = u.pa;
            return true;
        }
        */
}

pub fn flac_memory_alloc_aligned_uint64_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u64,
        aligned_pointer:   *mut *mut u64) -> bool {
    
    todo!();
        /*
            u64 *pu; /* unaligned pointer */
        union { /* union needed to comply with C99 pointer aliasing rules */
            u64 *pa; /* aligned pointer */
            void         *pv; /* aligned pointer alias */
        } u;

        ASSERT(elements > 0);
        ASSERT(0 != unaligned_pointer);
        ASSERT(0 != aligned_pointer);
        ASSERT(unaligned_pointer != aligned_pointer);

        if(elements > SIZE_MAX / sizeof(*pu)) /* overflow check */
            return false;

        pu = (u64*) memory_alloc_aligned(sizeof(*pu) * elements, &u.pv);
        if(0 == pu) {
            return false;
        }
        else {
            if(*unaligned_pointer != 0)
                free(*unaligned_pointer);
            *unaligned_pointer = pu;
            *aligned_pointer = u.pa;
            return true;
        }
        */
}

pub fn flac_memory_alloc_aligned_unsigned_array(
        elements:          usize,
        unaligned_pointer: *mut *mut u32,
        aligned_pointer:   *mut *mut u32) -> bool {
    
    todo!();
        /*
            unsigned *pu; /* unaligned pointer */
        union { /* union needed to comply with C99 pointer aliasing rules */
            unsigned *pa; /* aligned pointer */
            void     *pv; /* aligned pointer alias */
        } u;

        ASSERT(elements > 0);
        ASSERT(0 != unaligned_pointer);
        ASSERT(0 != aligned_pointer);
        ASSERT(unaligned_pointer != aligned_pointer);

        if(elements > SIZE_MAX / sizeof(*pu)) /* overflow check */
            return false;

        pu = (unsigned int*) memory_alloc_aligned(sizeof(*pu) * elements, &u.pv);
        if(0 == pu) {
            return false;
        }
        else {
            if(*unaligned_pointer != 0)
                free(*unaligned_pointer);
            *unaligned_pointer = pu;
            *aligned_pointer = u.pa;
            return true;
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_memory_alloc_aligned_real_array(
        elements:          usize,
        unaligned_pointer: *mut *mut real,
        aligned_pointer:   *mut *mut real) -> bool {
    
    todo!();
        /*
            real *pu; /* unaligned pointer */
        union { /* union needed to comply with C99 pointer aliasing rules */
            real *pa; /* aligned pointer */
            void       *pv; /* aligned pointer alias */
        } u;

        ASSERT(elements > 0);
        ASSERT(0 != unaligned_pointer);
        ASSERT(0 != aligned_pointer);
        ASSERT(unaligned_pointer != aligned_pointer);

        if(elements > SIZE_MAX / sizeof(*pu)) /* overflow check */
            return false;

        pu = (real*) memory_alloc_aligned(sizeof(*pu) * elements, &u.pv);
        if(0 == pu) {
            return false;
        }
        else {
            if(*unaligned_pointer != 0)
                free(*unaligned_pointer);
            *unaligned_pointer = pu;
            *aligned_pointer = u.pa;
            return true;
        }
        */
}

pub fn safe_malloc_mul_2op_p(
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
            if(!size1 || !size2)
            return malloc(1); /* malloc(0) is undefined; FLAC src convention is to always allocate */
        if(size1 > SIZE_MAX / size2)
            return 0;
        return malloc(size1*size2);
        */
}
