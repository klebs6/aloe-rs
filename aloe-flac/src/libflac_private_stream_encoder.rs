crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/stream_encoder.h]

/**
  | This is used to avoid overflow with unusual
  | signals in 32-bit accumulator in the
  | *precompute_partition_info_sums_*
  | functions.
  |
  */
pub const MAX_EXTRA_RESIDUAL_BPS: usize = 4;

#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSE2_SUPPORTED)]
lazy_static!{
    /*
    extern void precompute_partition_info_sums_intrin_sse2(const i32 residual[], u64 abs_residual_partition_sums[],
                unsigned residual_samples, unsigned predictor_order, unsigned min_partition_order, unsigned max_partition_order, unsigned bps);
    */
}

#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(SSSE3_SUPPORTED)]
lazy_static!{
    /*
    extern void precompute_partition_info_sums_intrin_ssse3(const i32 residual[], u64 abs_residual_partition_sums[],
                unsigned residual_samples, unsigned predictor_order, unsigned min_partition_order, unsigned max_partition_order, unsigned bps);
    */
}

#[cfg(all(any(CPU_IA32,CPU_X86_64),HAS_X86INTRIN))]
#[cfg(AVX2_SUPPORTED)]
lazy_static!{
    /*
    extern void precompute_partition_info_sums_intrin_avx2(const i32 residual[], u64 abs_residual_partition_sums[],
                unsigned residual_samples, unsigned predictor_order, unsigned min_partition_order, unsigned max_partition_order, unsigned bps);
    */
}
