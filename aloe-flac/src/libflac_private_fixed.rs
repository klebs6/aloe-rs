crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/fixed.h]

/**
  | fixed_compute_best_predictor()
  | 
  | --------------------------------------------
  | 
  | Compute the best fixed predictor and
  | the expected bits-per-sample of the
  | residual signal for each order. The
  | _wide() version uses 64-bit integers
  | which is statistically necessary when
  | bits-per- sample + log2(blocksize) > 30
  | 
  | IN data[0,data_len-1]
  | 
  | IN data_len
  | 
  | OUT residual_bits_per_sample[0,MAX_FIXED_ORDER]
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_fixed_compute_best_predictor(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_fixed_compute_best_predictor_wide(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
pub fn flac_fixed_compute_best_predictor_intrin_sse2(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER + 1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
pub fn flac_fixed_compute_best_predictor_wide_intrin_sse2(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER + 1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSSE3_SUPPORTED)]
pub fn flac_fixed_compute_best_predictor_intrin_ssse3(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSSE3_SUPPORTED)]
pub fn flac_fixed_compute_best_predictor_wide_intrin_ssse3(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER + 1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(CPU_IA32,HAS_NASM))]
pub fn flac_fixed_compute_best_predictor_asm_ia32_mmx_cmov(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(INTEGER_ONLY_LIBRARY)]
pub fn flac_fixed_compute_best_predictor(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(INTEGER_ONLY_LIBRARY)]
pub fn flac_fixed_compute_best_predictor_wide(
        data:                     &[i32],
        data_len:                 u32,
        residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1]) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | fixed_compute_residual()
  | 
  | -------------------------------------
  | 
  | Compute the residual signal obtained
  | from sutracting the predicted signal
  | from the original.
  | 
  | IN data[-order,data_len-1] original
  | signal (NOTE THE INDICES!)
  | 
  | IN data_len length of original signal
  | 
  | IN order <= MAX_FIXED_ORDER
  | fixed-predictor order
  | 
  | OUT residual[0,data_len-1] residual
  | signal
  |
  */
pub fn flac_fixed_compute_residual(
        data:     &[i32],
        data_len: u32,
        order:    u32,
        residual: &[i32])  {
    
    todo!();
        /*
        
        */
}

/**
  | fixed_restore_signal()
  | 
  | ---------------------------------------------
  | 
  | Restore the original signal by summing
  | the residual and the predictor.
  | 
  | IN residual[0,data_len-1] residual
  | signal
  | 
  | IN data_len length of original signal
  | 
  | IN order <= MAX_FIXED_ORDER
  | fixed-predictor order *** IMPORTANT:
  | the caller must pass in the historical
  | samples:
  | 
  | IN data[-order,-1] previously-reconstructed
  | historical samples
  | 
  | OUT data[0,data_len-1] original signal
  |
  */
pub fn flac_fixed_restore_signal(
        residual: &[i32],
        data_len: u32,
        order:    u32,
        data:     &[i32])  {
    
    todo!();
        /*
        
        */
}
