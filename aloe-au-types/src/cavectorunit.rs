crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAVectorUnit.h]

pub fn ca_vector_unit_get_vector_unit_type() -> i32 {
    
    todo!();
    /*
        return CAVectorUnit_GetType();
    */
}

pub fn ca_vector_unit_has_vector_unit() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() > kVecNone;
    */

    todo!();
        /*
            return CAVectorUnit_GetType() > kVecNone;
        */
}

pub fn ca_vector_unit_has_altivec() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() == kVecAltivec;
    */
}

pub fn ca_vector_unit_hassse2() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() >= kVecSSE2;
    */
}

pub fn ca_vector_unit_hassse3() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() >= kVecSSE3;
    */
}

pub fn ca_vector_unit_hasavx1() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() >= kVecAVX1;
    */
}

pub fn ca_vector_unit_has_neon() -> bool {
    
    todo!();
    /*
        return GetVectorUnitType() == kVecNeon;
    */
}

/**
  | Unify checks for vector units.
  |
  | Allow setting an environment variable
  | "CA_NoVector" to turn off vectorized code at
  | runtime (very useful for performance testing).
  */
lazy_static!{
    /*
    extern int gCAVectorUnitType;
    */
}

/**
   expensive. use GetType() for lazy
   initialization and caching.
  */
lazy_static!{
    /*
    extern SInt32 CAVectorUnit_Examine();
    */
}

#[inline] pub fn ca_vector_unit_get_type() -> i32 {
    
    todo!();
        /*
            int x = gCAVectorUnitType;
        return (x != kVecUninitialized) ? x : CAVectorUnit_Examine();
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAVectorUnit.cpp]

lazy_static!{
    /*
    int gCAVectorUnitType = kVecUninitialized;
    */
}

/**
  | Use cpuid to check if SSE2 is available.
  |
  | Before calling this function make sure cpuid is
  | available
  */
#[cfg(TARGET_OS_WIN32)]
pub fn is_sse2available() -> i32 {
    
    todo!();
        /*
            int return_value;

        {
            int r_edx;
            _asm
            {
                mov eax, 0x01
                cpuid
                mov r_edx, edx
            }
            return_value = (r_edx >> 26) & 0x1;
        }
        return return_value;
        */
}

/**
  | Use cpuid to check if SSE3 is available.
  |
  | Before calling this function make sure cpuid is
  | available
  */
#[cfg(TARGET_OS_WIN32)]
pub fn is_sse3available() -> i32 {
    
    todo!();
        /*
            SInt32 return_value;

        {
            SInt32 r_ecx;
            _asm
            {
                mov eax, 0x01
                cpuid
                mov r_ecx, ecx
            }
            return_value = r_ecx & 0x1;
        }
        return return_value;
        */
}

/**
  | Return true if the cpuid instruction is
  | available.
  |
  | The cpuid instruction is available if bit 21 in
  | the EFLAGS register can be changed
  |
  | This function may not work on Intel CPUs prior
  | to Pentium (didn't test)
  */
#[cfg(TARGET_OS_WIN32)]
pub fn is_cpuid_available() -> bool {
    
    todo!();
        /*
            SInt32 return_value = 0x0;
        _asm{
            pushfd    ;         //push original EFLAGS
            pop eax   ;         //get original EFLAGS
            mov ecx, eax   ;    //save original EFLAGS
            xor eax, 200000h  ; //flip ID bit in EFLAGS
            push eax   ;        //save new EFLAGS value on stack
            popfd    ;          //replace current EFLAGS value
            pushfd    ;         //get new EFLAGS
            pop eax   ;         //store new EFLAGS in EAX
            xor eax, ecx   ;
            je end_cpuid_identify  ; //can't toggle ID bit
            mov return_value, 0x1;
    end_cpuid_identify:
            nop;
            }
            return return_value;
        */
}

pub fn ca_vector_unit_examine() -> i32 {
    
    todo!();
        /*
            int result = kVecNone;

    #if TARGET_OS_WIN32
    #if HAS_IPP
        // Initialize the static IPP library! This needs to be done before
        // any IPP function calls, otherwise we may have a performance penalty
        int status = ippStaticInit();
        if ( status == ippStsNonIntelCpu )
        {
            IppCpuType cpuType = ippGetCpuType();
            if ( cpuType >= ippCpuSSE || cpuType <= ippCpuSSE42 )
                ippStaticInitCpu( cpuType );
        }
    #endif
        {
            // On Windows we use cpuid to detect the vector unit because it works on Intel and AMD.
            // The IPP library does not detect SSE on AMD processors.
            if (IsCpuidAvailable())
            {
                if(IsSSE3Available())
                {
                    result = kVecSSE3;
                }
                else if(IsSSE2Available())
                {
                    result = kVecSSE2;
                }
            }
        }
    #elif TARGET_OS_MAC
    #if DEBUG
        if (getenv("CA_NoVector")) {
            fprintf(stderr, "CA_NoVector set; Vector unit optimized routines will be bypassed\n");
            return result;
        }
        else
    #endif
        {
        #if (TARGET_CPU_PPC || TARGET_CPU_PPC64)
            int sels[2] = { CTL_HW, HW_VECTORUNIT };
            int vType = 0; //0 == scalar only
            size_t length = sizeof(vType);
            int error = sysctl(sels, 2, &vType, &length, NULL, 0);
            if (!error && vType > 0)
                result = kVecAltivec;
        #elif (TARGET_CPU_X86 || TARGET_CPU_X86_64)
            static const struct { const char* kName; const int kVectype; } kStringVectypes[] = {
                { "hw.optional.avx1_0", kVecAVX1 }, { "hw.optional.sse3", kVecSSE3 }, { "hw.optional.sse2", kVecSSE2 }
            };
            static const size_t kNumStringVectypes = sizeof(kStringVectypes)/sizeof(kStringVectypes[0]);
            int i = 0, answer = 0;
            while(i != kNumStringVectypes)
            {
                size_t length = sizeof(answer);
                int error = sysctlbyname(kStringVectypes[i].kName, &answer, &length, NULL, 0);
                if (!error && answer)
                {
                    result = kStringVectypes[i].kVectype;
                    break;
                }
                ++i;
            };
        #elif CA_ARM_NEON
            result = kVecNeon;
        #endif
        }
    #endif
        gCAVectorUnitType = result;
        return result;
        */
}

