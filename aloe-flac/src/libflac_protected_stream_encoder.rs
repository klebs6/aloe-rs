crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/protected/stream_encoder.h]

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub const MAX_APODIZATION_FUNCTIONS: usize = 32;

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub enum ApodizationFunction {
    APODIZATION_BARTLETT,
    APODIZATION_BARTLETT_HANN,
    APODIZATION_BLACKMAN,
    APODIZATION_BLACKMAN_HARRIS_4TERM_92DB_SIDELOBE,
    APODIZATION_CONNES,
    APODIZATION_FLATTOP,
    APODIZATION_GAUSS,
    APODIZATION_HAMMING,
    APODIZATION_HANN,
    APODIZATION_KAISER_BESSEL,
    APODIZATION_NUTTALL,
    APODIZATION_RECTANGLE,
    APODIZATION_TRIANGLE,
    APODIZATION_TUKEY,
    APODIZATION_PARTIAL_TUKEY,
    APODIZATION_PUNCHOUT_TUKEY,
    APODIZATION_WELCH
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub struct ApodizationSpecification {
    ty:         ApodizationFunction,
    parameters: FlacApodizationSpecificationParameters,
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[derive(Copy,Clone)]
pub struct FlacApodizationSpecificationGauss {
    stddev: real,
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[derive(Copy,Clone)]
pub struct FlacApodizationSpecificationTukey {
    p: real,
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
#[derive(Copy,Clone)]
pub struct FlacApodizationSpecificationMultipleTukey {
    p:     real,
    start: real,
    end:   real,
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub union FlacApodizationSpecificationParameters {
    gauss:          FlacApodizationSpecificationGauss,
    tukey:          FlacApodizationSpecificationTukey,
    multiple_tukey: FlacApodizationSpecificationMultipleTukey,
}

//-------------------------------
pub struct StreamEncoderProtected {
    state:                        StreamEncoderState,
    verify:                       bool,
    streamable_subset:            bool,
    do_md5:                       bool,
    do_mid_side_stereo:           bool,
    loose_mid_side_stereo:        bool,
    channels:                     u32,
    bits_per_sample:              u32,
    sample_rate:                  u32,
    blocksize:                    u32,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    num_apodizations:             u32,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    apodizations:                 [ApodizationSpecification; MAX_APODIZATION_FUNCTIONS],

    max_lpc_order:                u32,
    qlp_coeff_precision:          u32,
    do_qlp_coeff_prec_search:     bool,
    do_exhaustive_model_search:   bool,
    do_escape_coding:             bool,
    min_residual_partition_order: u32,
    max_residual_partition_order: u32,
    rice_parameter_search_dist:   u32,
    total_samples_estimate:       u64,
    metadata:                     StreamMetadata,
    num_metadata_blocks:          u32,
    streaminfo_offset:            u64,
    seektable_offset:             u64,
    audio_offset:                 u64,

    #[cfg(HAS_OGG)]
    ogg_encoder_aspect:           OggEncoderAspect,
}
