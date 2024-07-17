crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAutoDisposer.h]

#[inline] pub fn ca_malloc(size: usize)  {
    
    todo!();
        /*
            void* p = malloc(size);
        if (!p && size) throw std::bad_alloc();
        return p;
        */
}

#[inline] pub fn ca_realloc(
        old:  *mut c_void,
        size: usize)  {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        void* p = realloc(old, size);
    #else
        void* p = reallocf(old, size); // reallocf ensures the old pointer is freed if memory is full (p is NULL).
    #endif
        if (!p && size) throw std::bad_alloc();
        return p;
        */
}

#[cfg(not(UINTPTR_MAX))] #[cfg(__LP64__)]      pub const UINTPTR_MAX: usize = 18446744073709551615;
#[cfg(not(UINTPTR_MAX))] #[cfg(not(__LP64__))] pub const UINTPTR_MAX: usize = 4294967295;

#[inline] pub fn ca_calloc(
        n:    usize,
        size: usize)  {
    
    todo!();
        /*
            // ensure that multiplication will not overflow
        if (n && UINTPTR_MAX / n < size) throw std::bad_alloc();

        size_t nsize = n*size;
        void* p = malloc(nsize);
        if (!p && nsize) throw std::bad_alloc();

        memset(p, 0, nsize);
        return p;
        */
}

/**
  | convenience function
  |
  */
pub fn free<T>(p: &mut CAAutoFree<T>)  {

    todo!();
        /*
            p.free();
        */
}
