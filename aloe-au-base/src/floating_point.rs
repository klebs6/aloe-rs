crate::ix!();

/* --------------------------
   our compiler does ALL floating point with SSE
   */

#[cfg(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64)))]
#[inline] pub fn getcsr() -> i32 {
    
    todo!();
        /*
            int _result; asm volatile ("stmxcsr %0" : "=m" (*&_result) ); return _result;
        */
}


#[cfg(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64)))]
#[inline] pub fn setcsr(a: i32)  {
    
    todo!();
        /*
            int _temp = a; asm volatile( "ldmxcsr %0" : : "m" (*&_temp ) );
        */
}


#[cfg(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64)))]
macro_rules! disable_denormals {
    () => {
        /*
                int _savemxcsr = GETCSR(); SETCSR(_savemxcsr | 0x8040);
        */
    }
}

#[cfg(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64)))]
macro_rules! restore_denormals {
    () => {
        /*
                SETCSR(_savemxcsr);
        */
    }
}

///--------------------------
#[cfg(not(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64))))]
macro_rules! disable_denormals { () => { } }

#[cfg(not(all(TARGET_OS_MAC,any(TARGET_CPU_X86,TARGET_CPU_X86_64))))]
macro_rules! restore_denormals { () => { } }

