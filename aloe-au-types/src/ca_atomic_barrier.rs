crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAtomic.h]

/**
  | This file implements all Atomic operations
  | using Interlocked functions specified
  | in
  | 
  | Winbase.h
  | 
  | -----------
  | @note
  | 
  | According to Microsoft documentation,
  | all Interlocked functions generates
  | a full barrier.
  | 
  | On Windows:
  | 
  | As the Interlocked functions returns
  | the Old value, Extra checks and operations
  | are made after the atomic operation
  | to return value consistent with OSX
  | counterparts.
  |
  */
#[inline] pub fn ca_memory_barrier()  {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        MemoryBarrier();
    #else
        OSMemoryBarrier();
    #endif
        */
}

#[inline] pub fn ca_atomic_add_32barrier(
        the_amt:   i32,
        the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        long lRetVal = InterlockedExchangeAdd((volatile long*)theValue, theAmt);
        // InterlockedExchangeAdd returns the original value which differs from OSX version.
        // At this point the addition would have occured and hence returning the new value
        // to keep it sync with OSX.
        return lRetVal + theAmt;
    #else
        return OSAtomicAdd32Barrier(theAmt, (volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_or_32barrier(
        the_mask:  u32,
        the_value: Volatile<*mut u32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        // InterlockedAnd macro is not defined in x86 platform, and hence using the intrinsic
        // function instead.
        long j = _InterlockedOr((volatile long*)theValue, theMask);
        // _InterlockedOr returns the original value which differs from OSX version.
        // Returning the new value similar to OSX
        return (SInt32)(j | theMask);
    #else
        return OSAtomicOr32Barrier(theMask, (volatile uint32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_and_32barrier(
        the_mask:  u32,
        the_value: Volatile<*mut u32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
    // InterlockedAnd macro is not defined in x86 platform, and hence using the intrinsic
    // function instead.
        long j = _InterlockedAnd((volatile long*)theValue, theMask);
        // _InterlockedAnd returns the original value which differs from OSX version.
        // Returning the new value similar to OSX
        return (SInt32)(j & theMask);
    #else
        return OSAtomicAnd32Barrier(theMask, (volatile uint32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_compare_and_swap_32barrier(
        old_value: i32,
        new_value: i32,
        the_value: Volatile<*mut i32>) -> bool {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        // InterlockedCompareExchange returns the old value. But we need to return bool value.
        long lRetVal = InterlockedCompareExchange((volatile long*)theValue, newValue, oldValue);
    // Hence we check if the new value is set and if it is we return true else false.
    // If theValue is equal to oldValue then the swap happens. Otherwise swap doesn't happen.
        return (oldValue == lRetVal);
    #else
        return OSAtomicCompareAndSwap32Barrier(oldValue, newValue, (volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_increment32(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        return (SInt32)InterlockedIncrement((volatile long*)theValue);
    #else
        return OSAtomicIncrement32((volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_decrement32(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        return (SInt32)InterlockedDecrement((volatile long*)theValue);
    #else
        return OSAtomicDecrement32((volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_increment_32barrier(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        return CAAtomicIncrement32(theValue);
    #else
        return OSAtomicIncrement32Barrier((volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_decrement_32barrier(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        return CAAtomicDecrement32(theValue);
    #else
        return OSAtomicDecrement32Barrier((volatile int32_t *)theValue);
    #endif
        */
}

#[inline] pub fn ca_atomic_test_and_clear_barrier(
        bit_to_clear: i32,
        the_address:  *mut c_void) -> bool {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        BOOL bOldVal = InterlockedBitTestAndReset((long*)theAddress, bitToClear);
        return (bOldVal ? true : false);
    #else
        return OSAtomicTestAndClearBarrier(bitToClear, (volatile void *)theAddress);
    #endif
        */
}

#[inline] pub fn ca_atomic_test_and_clear(
        bit_to_clear: i32,
        the_address:  *mut c_void) -> bool {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        BOOL bOldVal = CAAtomicTestAndClearBarrier(bitToClear, (long*)theAddress);
        return (bOldVal ? true : false);
    #else
        return OSAtomicTestAndClear(bitToClear, (volatile void *)theAddress);
    #endif
        */
}

#[inline] pub fn ca_atomic_test_and_set_barrier(
        bit_to_set:  i32,
        the_address: *mut c_void) -> bool {
    
    todo!();
        /*
            #if TARGET_OS_WIN32
        BOOL bOldVal = InterlockedBitTestAndSet((long*)theAddress, bitToSet);
        return (bOldVal ? true : false);
    #else
        return OSAtomicTestAndSetBarrier(bitToSet, (volatile void *)theAddress);
    #endif
        */
}

/**
  | int32_t flavors -- for C++ only since we can't
  | overload in C CFBase.h defines SInt32 as signed
  | int which is similar to int32_t. If CFBase.h is
  | included, then this will generate redefinition
  | error. But on Mac, CFBase.h, still includes
  | MacTypes.h where SInt32 is defined as signed
  | long so this would work there.
  |
  | So in order to fix the redefinition errors, we
  | define these functions only if MacTypes.h is
  | included.
  */
#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_add_32barrier(
        the_amt:   i32,
        the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicAdd32Barrier(theAmt, (volatile SInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_or_32barrier(
        the_mask:  u32,
        the_value: Volatile<*mut u32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicOr32Barrier(theMask, (volatile UInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_and_32barrier(
        the_mask:  u32,
        the_value: Volatile<*mut u32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicAnd32Barrier(theMask, (volatile UInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_compare_and_swap_32barrier(
        old_value: i32,
        new_value: i32,
        the_value: Volatile<*mut i32>) -> bool {
    
    todo!();
        /*
            return CAAtomicCompareAndSwap32Barrier(oldValue, newValue, (volatile SInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_increment32(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicIncrement32((volatile SInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_decrement32(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicDecrement32((volatile SInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_increment_32barrier(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicIncrement32Barrier((volatile SInt32 *)theValue);
        */
}

#[cfg(all(all(__cplusplus,__MACTYPES__),not(__LP64__)))]
#[inline] pub fn ca_atomic_decrement_32barrier(the_value: Volatile<*mut i32>) -> i32 {
    
    todo!();
        /*
            return CAAtomicDecrement32Barrier((volatile SInt32 *)theValue);
        */
}

#[cfg(__LP64__)]
#[inline] pub fn ca_atomic_compare_and_swap_64barrier(
        old_value: i64,
        new_value: i64,
        the_value: Volatile<*mut i64>) -> bool {
    
    todo!();
        /*
            return OSAtomicCompareAndSwap64Barrier(__oldValue, __newValue, __theValue);
        */
}

#[inline] pub fn ca_atomic_compare_and_swap_ptr_barrier(
        old_value: *mut c_void,
        new_value: *mut c_void,
        the_value: Volatile<*mut *mut c_void>) -> bool {
    
    todo!();
        /*
            #if __LP64__
        return CAAtomicCompareAndSwap64Barrier((int64_t)__oldValue, (int64_t)__newValue, (int64_t *)__theValue);
    #else
        return CAAtomicCompareAndSwap32Barrier((int32_t)__oldValue, (int32_t)__newValue, (int32_t *)__theValue);
    #endif
        */
}
