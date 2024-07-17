crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/cpu.h]

pub enum CPUInfo_Type {
    CPUINFO_TYPE_IA32,
    CPUINFO_TYPE_X86_64,
    CPUINFO_TYPE_UNKNOWN
}

#[cfg(CPU_IA32)]
pub struct CPUInfo_IA32 {
    cmov:  bool,
    mmx:   bool,
    sse:   bool,
    sse2:  bool,
    sse3:  bool,
    ssse3: bool,
    sse41: bool,
    sse42: bool,
    avx:   bool,
    avx2:  bool,
    fma:   bool,
}

#[cfg(CPU_X86_64)]
pub struct CPUInfo_x86 {
    sse3:  bool,
    ssse3: bool,
    sse41: bool,
    sse42: bool,
    avx:   bool,
    avx2:  bool,
    fma:   bool,
}

pub struct CPUInfo {

    use_asm: bool,
    ty:      CPUInfo_Type,

    #[cfg(CPU_IA32)]
    ia32: CPUInfo_IA32,

    #[cfg(CPU_X86_64)]
    x86: CPUInfo_x86,
}

pub fn flac_cpu_info(info: *mut CPUInfo)  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(CPU_IA32,HAS_NASM))]
pub fn flac_cpu_have_cpuid_asm_ia32() -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(CPU_IA32,HAS_NASM))]
pub fn flac_cpu_info_asm_ia32(
        flags_edx: *mut u32,
        flags_ecx: *mut u32)  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
pub fn flac_cpu_have_cpuid_x86() -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
pub fn flac_cpu_info_x86(
        level: u32,
        eax:   *mut u32,
        ebx:   *mut u32,
        ecx:   *mut u32,
        edx:   *mut u32)  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
pub fn flac_cpu_xgetbv_x86() -> u32 {
    
    todo!();
        /*
        
        */
}
