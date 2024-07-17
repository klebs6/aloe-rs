crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/lpc.h]

/**
  | lpc_window_data()
  | 
  | -------------------------------------
  | 
  | Applies the given window to the data.
  | 
  | OPT: asm implementation
  | 
  | IN in[0,data_len-1]
  | 
  | IN window[0,data_len-1]
  | 
  | OUT out[0,lag-1]
  | 
  | IN data_len
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_window_data(
        in_:      &[i32],
        window:   &[real],
        out:      &[real],
        data_len: u32)  {
    
    todo!();
        /*
        
        */
}

/**
  | lpc_compute_autocorrelation()
  | 
  | ----------------------------------------
  | 
  | Compute the autocorrelation for lags
  | between 0 and lag-1.
  | 
  | Assumes data[] outside of [0,data_len-1]
  | == 0.
  | 
  | Asserts that lag > 0.
  | 
  | IN data[0,data_len-1]
  | 
  | IN data_len
  | 
  | IN 0 < lag <= data_len
  | 
  | OUT autoc[0,lag-1]
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_autocorrelation(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_autocorrelation_asm_ia32(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_autocorrelation_asm_ia32_sse_lag_4(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_autocorrelation_asm_ia32_sse_lag_8(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_autocorrelation_asm_ia32_sse_lag_12(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_autocorrelation_asm_ia32_sse_lag_16(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE_SUPPORTED)]
pub fn flac_lpc_compute_autocorrelation_intrin_sse_lag_4(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE_SUPPORTED)]
pub fn flac_lpc_compute_autocorrelation_intrin_sse_lag_8(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE_SUPPORTED)]
pub fn flac_lpc_compute_autocorrelation_intrin_sse_lag_12(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE_SUPPORTED)]
pub fn flac_lpc_compute_autocorrelation_intrin_sse_lag_16(
        data:     &[real],
        data_len: u32,
        lag:      u32,
        autoc:    &[real])  {
    
    todo!();
        /*
        
        */
}

/**
  | lpc_compute_lp_coefficients()
  | 
  | --------------------------------------------
  | 
  | Computes LP coefficients for orders
  | 1..max_order.
  | 
  | Do not call if autoc[0] == 0.0. This means
  | the signal is zero and there is no point
  | in calculating a predictor.
  | 
  | IN autoc[0,max_order] autocorrelation
  | values
  | 
  | IN 0 < max_order <= MAX_LPC_ORDER
  | max
  | 
  | LP order to compute
  | 
  | OUT lp_coeff[0,max_order-1][0,max_order-1]
  | LP coefficients for each order *** IMPORTANT:
  | *** lp_coeff[0,max_order-1][max_order,MAX_LPC_ORDER-1]
  | are untouched
  | 
  | OUT error[0,max_order-1] error for
  | each order (more specifically, the
  | variance of the error signal times #
  | of samples in the signal)
  | 
  | Example: if max_order is 9, the LP coefficients
  | for order 9 will be in lp_coeff[8][0,8],
  | the LP coefficients for order 8 will
  | be in lp_coeff[7][0,7], etc.
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_lp_coefficients(
        autoc:     &[real],
        max_order: *mut u32,
        lp_coeff:  [&[real]; MAX_LPC_ORDER],
        error:     &[f64])  {
    
    todo!();
        /*
        
        */
}

/**
  | lpc_quantize_coefficients()
  | 
  | ------------------------------------------
  | 
  | Quantizes the LP coefficients.
  | 
  | -----------
  | @note
  | 
  | precision + bits_per_sample must be
  | less than 32 (sizeof(i32)*8).
  | 
  | IN lp_coeff[0,order-1] LP coefficients
  | 
  | IN order LP order
  | 
  | IN MIN_QLP_COEFF_PRECISION
  | < precision desired precision (in bits,
  | including sign bit) of largest coefficient
  | 
  | OUT qlp_coeff[0,order-1] quantized
  | coefficients
  | 
  | OUT shift # of bits to shift right to get
  | approximated
  | 
  | LP coefficients. NOTE: could be negative.
  | 
  | RETURN 0 => quantization OK 1 => coefficients
  | require too much shifting for *shift
  | to fit in the
  | 
  | LPC subframe header. 'shift' is unset.
  | 2 => coefficients are all zero, which
  | is bad. 'shift' is unset.
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_quantize_coefficients(
        lp_coeff:  &[real],
        order:     u32,
        precision: u32,
        qlp_coeff: &[i32],
        shift:     *mut i32) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | lpc_compute_residual_from_qlp_coefficients()
  | 
  | ----------------------------------------------------
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
  | IN qlp_coeff[0,order-1] quantized
  | LP coefficients
  | 
  | IN order > 0 LP order
  | 
  | IN lp_quantization quantization of
  | LP coefficients in bits
  | 
  | OUT residual[0,data_len-1] residual
  | signal
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_residual_from_qlp_coefficients(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_wide(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_asm_ia32(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_asm_ia32_mmx(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_wide_asm_ia32(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_16_intrin_sse2(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_intrin_sse2(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE4_1_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_intrin_sse41(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE4_1_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_wide_intrin_sse41(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(AVX2_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_16_intrin_avx2(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(AVX2_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_intrin_avx2(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(AVX2_SUPPORTED)]
pub fn flac_lpc_compute_residual_from_qlp_coefficients_wide_intrin_avx2(
        data:            *const i32,
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        residual:        &[i32])  {
    
    todo!();
        /*
        
        */
}

/**
 |  lpc_restore_signal()
 |  -------------------------------------------
 |  Restore the original signal by summing the
 |  residual and the predictor.
 |
 |  IN residual[0,data_len-1]  
 |  residual signal
 |
 |  IN data_len                
 |  length of original signal
 |
 |  IN qlp_coeff[0,order-1]    
 |  quantized LP coefficients
 |
 |  IN order > 0               
 |  LP order
 |
 |  IN lp_quantization         
 |  quantization of LP coefficients in bits
 |
 |  *** IMPORTANT: the caller must pass in the historical samples:
 |
 |  IN  data[-order,-1]        
 |  previously-reconstructed historical samples
 |
 |  OUT data[0,data_len-1]     
 |  original signal
 */
pub fn flac_lpc_restore_signal(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

pub fn flac_lpc_restore_signal_wide(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_restore_signal_asm_ia32(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_restore_signal_asm_ia32_mmx(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(CPU_IA32)]
#[cfg(HAS_NASM)]
pub fn flac_lpc_restore_signal_wide_asm_ia32(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
pub fn flac_lpc_restore_signal_16_intrin_sse2(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(NO_ASM))]
#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE4_1_SUPPORTED)]
pub fn flac_lpc_restore_signal_wide_intrin_sse41(
        residual:        &[i32],
        data_len:        u32,
        qlp_coeff:       &[i32],
        order:           u32,
        lp_quantization: i32,
        data:            &[i32])  {
    
    todo!();
        /*
        
        */
}

/**
 |  lpc_compute_expected_bits_per_residual_sample()
 |  --------------------------------------------------------------------
 |  Compute the expected number of bits per
 |  residual signal sample based on the LP error
 |  (which is related to the residual variance).
 |
 |  IN lpc_error >= 0.0   
 |  error returned from calculating LP
 |  coefficients
 |
 |  IN total_samples > 0  
 |  # of samples in residual signal
 |
 |  RETURN                expected bits per sample
 */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_expected_bits_per_residual_sample(
        lpc_error:     f64,
        total_samples: u32) -> f64 {
    
    todo!();
        /*
        
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_expected_bits_per_residual_sample_with_error_scale(
        lpc_error:   f64,
        error_scale: f64) -> f64 {
    
    todo!();
        /*
        
        */
}

/**
 |  lpc_compute_best_order()
 |  ---------------------------------------------
 |  Compute the best order from the array of
 |  signal errors returned during coefficient
 |  computation.
 |
 |  IN lpc_error[0,max_order-1] >= 0.0  
 |  error returned from calculating LP
 |  coefficients
 |
 |  IN max_order > 0                    
 |  max LP order
 |
 |  IN total_samples > 0                
 |  # of samples in residual signal
 |
 |  IN overhead_bits_per_order          
 |  # of bits overhead for each increased LP order
 |  (includes warmup sample size and quantized LP
 |  coefficient)
 |
 |  RETURN [1,max_order]                
 |  best order
 */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_best_order(
        lpc_error:               &[f64],
        max_order:               u32,
        total_samples:           u32,
        overhead_bits_per_order: u32) -> u32 {
    
    todo!();
        /*
        
        */
}
