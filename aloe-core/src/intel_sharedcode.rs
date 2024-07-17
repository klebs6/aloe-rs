crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_intel_SharedCode.h]

#[cfg(all(ALOE_INTEL,not(ALOE_NO_INLINE_ASM)))]
pub mod system_stats_helpers
{
    use super::*;

    pub fn docpuid(
            a:  &mut u32,
            b:  &mut u32,
            c:  &mut u32,
            d:  &mut u32,
            ty: u32)  {
        
        todo!();
        /*
            uint32 la = a, lb = b, lc = c, ld = d;

           #if ALOE_32BIT && defined (__pic__)
            asm ("mov %%ebx, %%edi\n"
                 "cpuid\n"
                 "xchg %%edi, %%ebx\n"
                   : "=a" (la), "=D" (lb), "=c" (lc), "=d" (ld)
                   : "a" (type), "c" (0));
           #else
            asm ("cpuid\n"
                   : "=a" (la), "=b" (lb), "=c" (lc), "=d" (ld)
                   : "a" (type), "c" (0));
           #endif

            a = la; b = lb; c = lc; d = ld;
        */
    }


    pub fn get_cpu_info(
            hasmmx:             &mut bool,
            hassse:             &mut bool,
            hassse2:            &mut bool,
            has3d_now:          &mut bool,
            hassse3:            &mut bool,
            hasssse3:           &mut bool,
            hasfma3:            &mut bool,
            hassse41:           &mut bool,
            hassse42:           &mut bool,
            hasavx:             &mut bool,
            hasfma4:            &mut bool,
            hasavx2:            &mut bool,
            hasavx512f:         &mut bool,
            hasavx512dq:        &mut bool,
            hasavx512ifma:      &mut bool,
            hasavx512pf:        &mut bool,
            hasavx512er:        &mut bool,
            hasavx512cd:        &mut bool,
            hasavx512bw:        &mut bool,
            hasavx512vl:        &mut bool,
            hasavx512vbmi:      &mut bool,
            hasavx512vpopcntdq: &mut bool)  {
        
        todo!();
        /*
            uint32 a = 0, b = 0, d = 0, c = 0;
            SystemStatsHelpers::doCPUID (a, b, c, d, 1);

            hasMMX   = (d & (1u << 23)) != 0;
            hasSSE   = (d & (1u << 25)) != 0;
            hasSSE2  = (d & (1u << 26)) != 0;
            has3DNow = (b & (1u << 31)) != 0;
            hasSSE3  = (c & (1u <<  0)) != 0;
            hasSSSE3 = (c & (1u <<  9)) != 0;
            hasFMA3  = (c & (1u << 12)) != 0;
            hasSSE41 = (c & (1u << 19)) != 0;
            hasSSE42 = (c & (1u << 20)) != 0;
            hasAVX   = (c & (1u << 28)) != 0;

            SystemStatsHelpers::doCPUID (a, b, c, d, 0x80000001);
            hasFMA4  = (c & (1u << 16)) != 0;

            SystemStatsHelpers::doCPUID (a, b, c, d, 7);
            hasAVX2            = (b & (1u <<  5)) != 0;
            hasAVX512F         = (b & (1u << 16)) != 0;
            hasAVX512DQ        = (b & (1u << 17)) != 0;
            hasAVX512IFMA      = (b & (1u << 21)) != 0;
            hasAVX512PF        = (b & (1u << 26)) != 0;
            hasAVX512ER        = (b & (1u << 27)) != 0;
            hasAVX512CD        = (b & (1u << 28)) != 0;
            hasAVX512BW        = (b & (1u << 30)) != 0;
            hasAVX512VL        = (b & (1u << 31)) != 0;
            hasAVX512VBMI      = (c & (1u <<  1)) != 0;
            hasAVX512VPOPCNTDQ = (c & (1u << 14)) != 0;
        */
    }
}

