crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/stream_encoder.c]

/**
  | Exact Rice codeword length calculation
  | is off by default. The simple (and fast)
  | estimation (of how many bits a residual
  | value will be encoded with) in this encoder
  | is very good, almost always yielding
  | compression within 0.1% of exact calculation.
  |
  */
pub const EXACT_RICE_BITS_CALCULATION: bool = false;

/**
  | Rice parameter searching is off by default.
  | The simple (and fast) parameter estimation
  | in this encoder is very good, almost
  | always yielding compression within
  | 0.1% of the optimal parameters.
  |
  */
pub const ENABLE_RICE_PARAMETER_SEARCH: bool = false;

pub struct VerifyInputFifo {
    data: [*mut i32; MAX_CHANNELS],

    /**
      | of each data[] in samples
      |
      */
    size: u32,

    tail: u32,
}

pub struct VerifyOutput {
    data:     *mut u8,
    capacity: u32,
    bytes:    u32,
}

pub enum EncoderStateHint {
    ENCODER_IN_MAGIC    = 0,
    ENCODER_IN_METADATA = 1,
    ENCODER_IN_AUDIO    = 2
}

pub struct CompressionLevel {
    do_mid_side_stereo:           bool,
    loose_mid_side_stereo:        bool,
    max_lpc_order:                u32,
    qlp_coeff_precision:          u32,
    do_qlp_coeff_prec_search:     bool,
    do_escape_coding:             bool,
    do_exhaustive_model_search:   bool,
    min_residual_partition_order: u32,
    max_residual_partition_order: u32,
    rice_parameter_search_dist:   u32,
    apodization:                  &'static str,
} 

/**
  | here we use locale-independent 5e-1
  | instead of 0.5 or 0,5
  |
  */
pub const COMPRESSION_LEVELS: &[CompressionLevel] = &[
    CompressionLevel {
        do_mid_side_stereo:             false, 
        loose_mid_side_stereo:          false,  
        max_lpc_order:                  0, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   3, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          true ,  
        max_lpc_order:                  0, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   3, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          false,  
        max_lpc_order:                  0, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   3, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             false, 
        loose_mid_side_stereo:          false,  
        max_lpc_order:                  6, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   4, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          true ,  
        max_lpc_order:                  8, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   4, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          false,  
        max_lpc_order:                  8, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   5, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          false,  
        max_lpc_order:                  8, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   6, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1);partial_tukey(2)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          false, 
        max_lpc_order:                  12, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   6, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1);partial_tukey(2)" 
    },
    CompressionLevel { 
        do_mid_side_stereo:             true , 
        loose_mid_side_stereo:          false, 
        max_lpc_order:                  12, 
        qlp_coeff_precision:            0, 
        do_qlp_coeff_prec_search:       false, 
        do_escape_coding:               false, 
        do_exhaustive_model_search:     false, 
        min_residual_partition_order:   0, 
        max_residual_partition_order:   6, 
        rice_parameter_search_dist:     0, 
        apodization:                    "tukey(5e-1);partial_tukey(2);punchout_tukey(3)" 
    },
];

pub struct StreamEncoderPrivate {

    /**
      | current size (in samples) of the signal
      | and residual buffers
      |
      */
    input_capacity:                                   u32,

    /**
      | the integer version of the input signal
      |
      */
    integer_signal:                                   [*mut i32; MAX_CHANNELS],

    /**
      | the integer version of the mid-side
      | input signal (stereo only)
      |
      */
    integer_signal_mid_side:                          [*mut i32; 2],


    /**
      | (@@@ currently unused) the floating-point
      | version of the input signal
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    real_signal:                                      [*mut real; MAX_CHANNELS],


    /**
      | (@@@ currently unused) the floating-point
      | version of the mid-side input signal
      | (stereo only)
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    real_signal_mid_side:                             [*mut real; 2],


    /**
      | the pre-computed floating-point window
      | for each apodization function
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    window:                                           [*mut real; MAX_APODIZATION_FUNCTIONS],

    /**
      | the integer_signal[] * current window[]
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    windowed_signal:                                  *mut real,

    /**
      | the effective bits per sample of the
      | input signal (stream bps - wasted bits)
      |
      */
    subframe_bps:                                     [u32; MAX_CHANNELS],

    /**
      | the effective bits per sample of the
      | mid-side input signal (stream bps -
      | wasted bits + 0/1)
      |
      */
    subframe_bps_mid_side:                            [u32; 2],

    /**
      | each channel has a candidate and best
      | workspace where the subframe residual
      | signals will be stored
      |
      */
    residual_workspace:                               [[*mut i32; MAX_CHANNELS]; 2],
    residual_workspace_mid_side:                      [[*mut i32; 2]; 2],
    subframe_workspace:                               [[*mut Subframe; MAX_CHANNELS]; 2],
    subframe_workspace_mid_side:                      [[*mut Subframe; 2]; 2],
    subframe_workspace_ptr:                           [[*mut Subframe; MAX_CHANNELS]; 2],
    subframe_workspace_ptr_mid_side:                  [[*mut Subframe; 2]; 2],
    partitioned_rice_contents_workspace:              [[EntropyCodingMethod_PartitionedRiceContents; MAX_CHANNELS]; 2],
    partitioned_rice_contents_workspace_mid_side:     [[EntropyCodingMethod_PartitionedRiceContents; MAX_CHANNELS]; 2],
    partitioned_rice_contents_workspace_ptr:          [[*mut EntropyCodingMethod_PartitionedRiceContents; MAX_CHANNELS]; 2],
    partitioned_rice_contents_workspace_ptr_mid_side: [[*mut EntropyCodingMethod_PartitionedRiceContents; MAX_CHANNELS]; 2],

    /**
      | index (0 or 1) into 2nd dimension of the
      | above workspaces
      |
      */
    best_subframe:                                    [u32; MAX_CHANNELS],

    best_subframe_mid_side:                           [u32; 2],

    /**
      | size in bits of the best subframe for
      | each channel
      |
      */
    best_subframe_bits:                               [u32; MAX_CHANNELS],

    best_subframe_bits_mid_side:                      [u32; 2],

    /**
      | workspace where the sum of abs(candidate
      | residual) for each partition is stored
      |
      */
    abs_residual_partition_sums:                      *mut u64,


    /**
      | workspace where the sum of silog2(candidate
      | residual) for each partition is stored
      |
      */
    raw_bits_per_partition:                           *mut u32,


    /**
      | the current frame being worked on
      |
      */
    frame:                                            *mut BitWriter,


    /**
      | rounded number of frames the encoder
      | will use before trying both independent
      | and mid/side frames again
      |
      */
    loose_mid_side_stereo_frames:                     u32,

    /**
      | number of frames using the current channel
      | assignment
      |
      */
    loose_mid_side_stereo_frame_count:                u32,

    last_channel_assignment:                          ChannelAssignment,

    /**
      | scratchpad for STREAMINFO as it is built
      |
      */
    streaminfo:                                       StreamMetadata,

    /**
      | pointer into encoder->protected_->metadata_
      | where the seek table is
      |
      */
    seek_table:                                       *mut StreamMetadata_SeekTable,

    current_sample_number:                            u32,
    current_frame_number:                             u32,
    md5context:                                       MD5Context,
    cpuinfo:                                          CPUInfo,

    local_precompute_partition_info_sums:             fn(
            residual:                    &[i32],
            abs_residual_partition_sums: &[u64],
            residual_samples:            u32,
            predictor_order:             u32,
            min_partition_order:         u32,
            max_partition_order:         u32,
            bps:                         u32
    ) -> c_void,


    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_fixed_compute_best_predictor:      fn(
            data:                     &[i32],
            data_len:                 u32,
            residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]
    ) -> u32,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_fixed_compute_best_predictor_wide: fn(
            data:                     &[i32],
            data_len:                 u32,
            residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]
    ) -> u32,

    #[cfg(INTEGER_ONLY_LIBRARY)]
    local_fixed_compute_best_predictor: fn(
            data:                     &[i32],
            data_len:                 u32,
            residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1]
    ) -> u32,

    #[cfg(INTEGER_ONLY_LIBRARY)]
    local_fixed_compute_best_predictor_wide: fn(
            data:                     &[i32],
            data_len:                 u32,
            residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1]
    ) -> u32,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_lpc_compute_autocorrelation:                      fn(
            data:     &[real],
            data_len: u32,
            lag:      u32,
            autoc:    &[real]
    ) -> c_void,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_lpc_compute_residual_from_qlp_coefficients:       fn(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            residual:        &[i32]
    ) -> c_void,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_lpc_compute_residual_from_qlp_coefficients_64bit: fn(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            residual:        &[i32]
    ) -> c_void,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    local_lpc_compute_residual_from_qlp_coefficients_16bit: fn(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            residual:        &[i32]
    ) -> c_void,

    /**
      | use slow 64-bit versions of some functions
      | because of the block size
      |
      */
    use_wide_by_block:                     bool,

    /**
      | use slow 64-bit versions of some functions
      | because of the min partition order and
      | blocksize
      |
      */
    use_wide_by_partition:                 bool,

    /**
      | use slow 64-bit versions of some functions
      | because of the lpc order
      |
      */
    use_wide_by_order:                     bool,
    disable_constant_subframes:            bool,
    disable_fixed_subframes:               bool,
    disable_verbatim_subframes:            bool,

    #[cfg(HAS_OGG)]
    is_ogg:                                bool,

    /**
      | currently only needed for Ogg FLAC
      |
      */
    read_callback:                         FLAC__StreamEncoderReadCallback,
    seek_callback:                         FLAC__StreamEncoderSeekCallback,
    tell_callback:                         FLAC__StreamEncoderTellCallback,
    write_callback:                        FLAC__StreamEncoderWriteCallback,
    metadata_callback:                     FLAC__StreamEncoderMetadataCallback,
    progress_callback:                     FLAC__StreamEncoderProgressCallback,
    client_data:                           *mut c_void,
    first_seekpoint_to_check:              u32,

    /**
      | only used when encoding to a file
      |
      */
    file:                                  *mut libc::FILE,

    bytes_written:                         u64,
    samples_written:                       u64,
    frames_written:                        u32,
    total_frames_estimate:                 u32,

    /**
      | unaligned (original) pointers to allocated
      | data
      |
      */
    integer_signal_unaligned:              [*mut i32; MAX_CHANNELS],
    integer_signal_mid_side_unaligned:     [*mut i32; 2],


    /**
      | (@@@ currently unused)
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    real_signal_unaligned:                 [*mut real; MAX_CHANNELS],


    /**
      | (@@@ currently unused)
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    real_signal_mid_side_unaligned:        [*mut real; 2],

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    window_unaligned:                      [*mut real; MAX_APODIZATION_FUNCTIONS],

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    windowed_signal_unaligned:             *mut real,

    residual_workspace_unaligned:          [[*mut i32; MAX_CHANNELS]; 2],
    residual_workspace_mid_side_unaligned: [[*mut i32; 2]; 2],
    abs_residual_partition_sums_unaligned: *mut u64,
    raw_bits_per_partition_unaligned:      *mut u32,

    /*
       | These fields have been moved here from
       | private function local declarations
       | merely to save stack space during encoding.
       |
       */

    /**
      | from process_subframe_()
      |
      */
    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    lp_coeff:                              [[real; MAX_LPC_ORDER]; MAX_LPC_ORDER],

    /**
      | from find_best_partition_order_()
      |
      */
    partitioned_rice_contents_extra:       [EntropyCodingMethod_PartitionedRiceContents; 2],

    /**
      | The data for the verify section
      |
      */
    verify:                                FlacStreamEncoderPrivateVerify,

    /**
      | if true, call to ..._finish() from ..._delete()
      | will not call the callbacks
      |
      */
    is_being_deleted:                      bool,
}

pub struct FlacStreamEncoderPrivateVerify {
    decoder:          StreamDecoder,
    state_hint:       EncoderStateHint,
    needs_magic_hack: bool,
    input_fifo:       VerifyInputFifo,
    output:           VerifyOutput,
    error_stats:      FlacStreamEncoderPrivateErrorStats,
}

pub struct FlacStreamEncoderPrivateErrorStats {
    absolute_sample: u64,
    frame_number:    u32,
    channel:         u32,
    sample:          u32,
    expected:        i32,
    got:             i32,
}

pub const FLAC_STREAM_ENCODER_STATE_STRING: &[&'static str] = &[
    "STREAM_ENCODER_OK", 
    "STREAM_ENCODER_UNINITIALIZED", 
    "STREAM_ENCODER_OGG_ERROR", 
    "STREAM_ENCODER_VERIFY_DECODER_ERROR", 
    "STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA", 
    "STREAM_ENCODER_CLIENT_ERROR", 
    "STREAM_ENCODER_IO_ERROR", 
    "STREAM_ENCODER_FRAMING_ERROR", 
    "STREAM_ENCODER_MEMORY_ALLOCATION_ERROR" 
];

pub const FLAC_STREAM_ENCODER_INIT_STATUS_STRING: &[&'static str] = &[
    "STREAM_ENCODER_INIT_STATUS_OK",
    "STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR",
    "STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER",
    "STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS",
    "STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS",
    "STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE",
    "STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE",
    "STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE",
    "STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER",
    "STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION",
    "STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER",
    "STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE",
    "STREAM_ENCODER_INIT_STATUS_INVALID_METADATA",
    "STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED"
];

pub const FLAC_STREAM_ENCODER_READ_STATUS_STRING: &[&'static str] = &[
    "STREAM_ENCODER_READ_STATUS_CONTINUE",
    "STREAM_ENCODER_READ_STATUS_END_OF_STREAM",
    "STREAM_ENCODER_READ_STATUS_ABORT",
    "STREAM_ENCODER_READ_STATUS_UNSUPPORTED"
];

pub const FLAC_STREAM_ENCODER_WRITE_STATUS_STRING: &[&'static str] = &[
    "STREAM_ENCODER_WRITE_STATUS_OK",
    "STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR"
];

pub const FLAC_STREAM_ENCODER_SEEK_STATUS_STRING: &[&'static str] = &[
    "STREAM_ENCODER_SEEK_STATUS_OK",
    "STREAM_ENCODER_SEEK_STATUS_ERROR",
    "STREAM_ENCODER_SEEK_STATUS_UNSUPPORTED"
];

pub const FLAC_STREAM_ENCODER_TELL_STATUS_STRING: &[&'static str] = &[
    "STREAM_ENCODER_TELL_STATUS_OK",
    "STREAM_ENCODER_TELL_STATUS_ERROR",
    "STREAM_ENCODER_TELL_STATUS_UNSUPPORTED"
];

/**
  | Number of samples that will be overread
  | to watch for end of stream. By 'overread',
  | we mean that the stream_encoder_process*()
  | calls will always try to read blocksize+1
  | samples before encoding a block, so
  | that even if the stream has a total sample
  | count that is an integral multiple of
  | the blocksize, we will still notice
  | when we are encoding the last block.
  | This is needed, for example, to correctly
  | set the end-of-stream marker in Ogg
  | FLAC.
  | 
  | WATCHOUT: some parts of the code assert
  | that OVERREAD_ == 1 and there's not really
  | any reason to change it.
  |
  */
pub const OVERREAD: u32 = 1;

pub fn flac_stream_encoder_new() -> *mut StreamEncoder {
    
    todo!();
        /*
            StreamEncoder *encoder;
        unsigned i;

        ASSERT(sizeof(int) >= 4); /* we want to die right away if this is not true */

        encoder = (StreamEncoder*) calloc(1, sizeof(StreamEncoder));
        if(encoder == 0) {
            return 0;
        }

        encoder->protected_ = (StreamEncoderProtected*) calloc(1, sizeof(StreamEncoderProtected));
        if(encoder->protected_ == 0) {
            free(encoder);
            return 0;
        }

        encoder->private_ = (StreamEncoderPrivate*) calloc(1, sizeof(StreamEncoderPrivate));
        if(encoder->private_ == 0) {
            free(encoder->protected_);
            free(encoder);
            return 0;
        }

        encoder->private_->frame = bitwriter_new();
        if(encoder->private_->frame == 0) {
            free(encoder->private_);
            free(encoder->protected_);
            free(encoder);
            return 0;
        }

        encoder->private_->file = 0;

        set_defaults_(encoder);

        encoder->private_->is_being_deleted = false;

        for(i = 0; i < MAX_CHANNELS; i++) {
            encoder->private_->subframe_workspace_ptr[i][0] = &encoder->private_->subframe_workspace[i][0];
            encoder->private_->subframe_workspace_ptr[i][1] = &encoder->private_->subframe_workspace[i][1];
        }
        for(i = 0; i < 2; i++) {
            encoder->private_->subframe_workspace_ptr_mid_side[i][0] = &encoder->private_->subframe_workspace_mid_side[i][0];
            encoder->private_->subframe_workspace_ptr_mid_side[i][1] = &encoder->private_->subframe_workspace_mid_side[i][1];
        }
        for(i = 0; i < MAX_CHANNELS; i++) {
            encoder->private_->partitioned_rice_contents_workspace_ptr[i][0] = &encoder->private_->partitioned_rice_contents_workspace[i][0];
            encoder->private_->partitioned_rice_contents_workspace_ptr[i][1] = &encoder->private_->partitioned_rice_contents_workspace[i][1];
        }
        for(i = 0; i < 2; i++) {
            encoder->private_->partitioned_rice_contents_workspace_ptr_mid_side[i][0] = &encoder->private_->partitioned_rice_contents_workspace_mid_side[i][0];
            encoder->private_->partitioned_rice_contents_workspace_ptr_mid_side[i][1] = &encoder->private_->partitioned_rice_contents_workspace_mid_side[i][1];
        }

        for(i = 0; i < MAX_CHANNELS; i++) {
            format_entropy_coding_method_partitioned_rice_contents_init(&encoder->private_->partitioned_rice_contents_workspace[i][0]);
            format_entropy_coding_method_partitioned_rice_contents_init(&encoder->private_->partitioned_rice_contents_workspace[i][1]);
        }
        for(i = 0; i < 2; i++) {
            format_entropy_coding_method_partitioned_rice_contents_init(&encoder->private_->partitioned_rice_contents_workspace_mid_side[i][0]);
            format_entropy_coding_method_partitioned_rice_contents_init(&encoder->private_->partitioned_rice_contents_workspace_mid_side[i][1]);
        }
        for(i = 0; i < 2; i++)
            format_entropy_coding_method_partitioned_rice_contents_init(&encoder->private_->partitioned_rice_contents_extra[i]);

        encoder->protected_->state = STREAM_ENCODER_UNINITIALIZED;

        return encoder;
        */
}

pub fn flac_stream_encoder_delete(encoder: *mut StreamEncoder)  {
    
    todo!();
        /*
            unsigned i;

        if (encoder == NULL)
            return ;

        ASSERT(0 != encoder->protected_);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->private_->frame);

        encoder->private_->is_being_deleted = true;

        (void)stream_encoder_finish(encoder);

        if(0 != encoder->private_->verify.decoder)
            stream_decoder_delete(encoder->private_->verify.decoder);

        for(i = 0; i < MAX_CHANNELS; i++) {
            format_entropy_coding_method_partitioned_rice_contents_clear(&encoder->private_->partitioned_rice_contents_workspace[i][0]);
            format_entropy_coding_method_partitioned_rice_contents_clear(&encoder->private_->partitioned_rice_contents_workspace[i][1]);
        }
        for(i = 0; i < 2; i++) {
            format_entropy_coding_method_partitioned_rice_contents_clear(&encoder->private_->partitioned_rice_contents_workspace_mid_side[i][0]);
            format_entropy_coding_method_partitioned_rice_contents_clear(&encoder->private_->partitioned_rice_contents_workspace_mid_side[i][1]);
        }
        for(i = 0; i < 2; i++)
            format_entropy_coding_method_partitioned_rice_contents_clear(&encoder->private_->partitioned_rice_contents_extra[i]);

        bitwriter_delete(encoder->private_->frame);
        free(encoder->private_);
        free(encoder->protected_);
        free(encoder);
        */
}

pub fn init_stream_internal(
        encoder:           *mut StreamEncoder,
        read_callback:     FLAC__StreamEncoderReadCallback,
        write_callback:    FLAC__StreamEncoderWriteCallback,
        seek_callback:     FLAC__StreamEncoderSeekCallback,
        tell_callback:     FLAC__StreamEncoderTellCallback,
        metadata_callback: FLAC__StreamEncoderMetadataCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            unsigned i;
        bool metadata_has_seektable, metadata_has_vorbis_comment, metadata_picture_has_type1, metadata_picture_has_type2;

        ASSERT(0 != encoder);

        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED;

    #if !HAS_OGG
        if(is_ogg)
            return STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER;
    #endif

        if(0 == write_callback || (seek_callback && 0 == tell_callback))
            return STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS;

        if(encoder->protected_->channels == 0 || encoder->protected_->channels > MAX_CHANNELS)
            return STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS;

        if(encoder->protected_->channels != 2) {
            encoder->protected_->do_mid_side_stereo = false;
            encoder->protected_->loose_mid_side_stereo = false;
        }
        else if(!encoder->protected_->do_mid_side_stereo)
            encoder->protected_->loose_mid_side_stereo = false;

        if(encoder->protected_->bits_per_sample >= 32)
            encoder->protected_->do_mid_side_stereo = false; /* since we currenty do 32-bit math, the side channel would have 33 bps and overflow */

        if(encoder->protected_->bits_per_sample < MIN_BITS_PER_SAMPLE || encoder->protected_->bits_per_sample > REFERENCE_CODEC_MAX_BITS_PER_SAMPLE)
            return STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE;

        if(!format_sample_rate_is_valid(encoder->protected_->sample_rate))
            return STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE;

        if(encoder->protected_->blocksize == 0) {
            if(encoder->protected_->max_lpc_order == 0)
                encoder->protected_->blocksize = 1152;
            else
                encoder->protected_->blocksize = 4096;
        }

        if(encoder->protected_->blocksize < MIN_BLOCK_SIZE || encoder->protected_->blocksize > MAX_BLOCK_SIZE)
            return STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE;

        if(encoder->protected_->max_lpc_order > MAX_LPC_ORDER)
            return STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER;

        if(encoder->protected_->blocksize < encoder->protected_->max_lpc_order)
            return STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER;

        if(encoder->protected_->qlp_coeff_precision == 0) {
            if(encoder->protected_->bits_per_sample < 16) {
                /* @@@ need some data about how to set this here w.r.t. blocksize and sample rate */
                /* @@@ until then we'll make a guess */
                encoder->protected_->qlp_coeff_precision = flac_max(MIN_QLP_COEFF_PRECISION, 2 + encoder->protected_->bits_per_sample / 2);
            }
            else if(encoder->protected_->bits_per_sample == 16) {
                if(encoder->protected_->blocksize <= 192)
                    encoder->protected_->qlp_coeff_precision = 7;
                else if(encoder->protected_->blocksize <= 384)
                    encoder->protected_->qlp_coeff_precision = 8;
                else if(encoder->protected_->blocksize <= 576)
                    encoder->protected_->qlp_coeff_precision = 9;
                else if(encoder->protected_->blocksize <= 1152)
                    encoder->protected_->qlp_coeff_precision = 10;
                else if(encoder->protected_->blocksize <= 2304)
                    encoder->protected_->qlp_coeff_precision = 11;
                else if(encoder->protected_->blocksize <= 4608)
                    encoder->protected_->qlp_coeff_precision = 12;
                else
                    encoder->protected_->qlp_coeff_precision = 13;
            }
            else {
                if(encoder->protected_->blocksize <= 384)
                    encoder->protected_->qlp_coeff_precision = MAX_QLP_COEFF_PRECISION-2;
                else if(encoder->protected_->blocksize <= 1152)
                    encoder->protected_->qlp_coeff_precision = MAX_QLP_COEFF_PRECISION-1;
                else
                    encoder->protected_->qlp_coeff_precision = MAX_QLP_COEFF_PRECISION;
            }
            ASSERT(encoder->protected_->qlp_coeff_precision <= MAX_QLP_COEFF_PRECISION);
        }
        else if(encoder->protected_->qlp_coeff_precision < MIN_QLP_COEFF_PRECISION || encoder->protected_->qlp_coeff_precision > MAX_QLP_COEFF_PRECISION)
            return STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION;

        if(encoder->protected_->streamable_subset) {
            if(!format_blocksize_is_subset(encoder->protected_->blocksize, encoder->protected_->sample_rate))
                return STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE;
            if(!format_sample_rate_is_subset(encoder->protected_->sample_rate))
                return STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE;
            if(
                encoder->protected_->bits_per_sample != 8 &&
                encoder->protected_->bits_per_sample != 12 &&
                encoder->protected_->bits_per_sample != 16 &&
                encoder->protected_->bits_per_sample != 20 &&
                encoder->protected_->bits_per_sample != 24
            )
                return STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE;
            if(encoder->protected_->max_residual_partition_order > SUBSET_MAX_RICE_PARTITION_ORDER)
                return STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE;
            if(
                encoder->protected_->sample_rate <= 48000 &&
                (
                    encoder->protected_->blocksize > SUBSET_MAX_BLOCK_SIZE_48000HZ ||
                    encoder->protected_->max_lpc_order > SUBSET_MAX_LPC_ORDER_48000HZ
                )
            ) {
                return STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE;
            }
        }

        if(encoder->protected_->max_residual_partition_order >= (1u << ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN))
            encoder->protected_->max_residual_partition_order = (1u << ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN) - 1;
        if(encoder->protected_->min_residual_partition_order >= encoder->protected_->max_residual_partition_order)
            encoder->protected_->min_residual_partition_order = encoder->protected_->max_residual_partition_order;

    #if HAS_OGG
        /* reorder metadata if necessary to ensure that any VORBIS_COMMENT is the first, according to the mapping spec */
        if(is_ogg && 0 != encoder->protected_->metadata && encoder->protected_->num_metadata_blocks > 1) {
            unsigned i1;
            for(i1 = 1; i1 < encoder->protected_->num_metadata_blocks; i1++) {
                if(0 != encoder->protected_->metadata[i1] && encoder->protected_->metadata[i1]->type == METADATA_TYPE_VORBIS_COMMENT) {
                    StreamMetadata *vc = encoder->protected_->metadata[i1];
                    for( ; i1 > 0; i1--)
                        encoder->protected_->metadata[i1] = encoder->protected_->metadata[i1-1];
                    encoder->protected_->metadata[0] = vc;
                    break;
                }
            }
        }
    #endif
        /* keep track of any SEEKTABLE block */
        if(0 != encoder->protected_->metadata && encoder->protected_->num_metadata_blocks > 0) {
            unsigned i2;
            for(i2 = 0; i2 < encoder->protected_->num_metadata_blocks; i2++) {
                if(0 != encoder->protected_->metadata[i2] && encoder->protected_->metadata[i2]->type == METADATA_TYPE_SEEKTABLE) {
                    encoder->private_->seek_table = &encoder->protected_->metadata[i2]->data.seek_table;
                    break; /* take only the first one */
                }
            }
        }

        /* validate metadata */
        if(0 == encoder->protected_->metadata && encoder->protected_->num_metadata_blocks > 0)
            return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
        metadata_has_seektable = false;
        metadata_has_vorbis_comment = false;
        metadata_picture_has_type1 = false;
        metadata_picture_has_type2 = false;
        for(i = 0; i < encoder->protected_->num_metadata_blocks; i++) {
            const StreamMetadata *m = encoder->protected_->metadata[i];
            if(m->type == METADATA_TYPE_STREAMINFO)
                return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
            else if(m->type == METADATA_TYPE_SEEKTABLE) {
                if(metadata_has_seektable) /* only one is allowed */
                    return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                metadata_has_seektable = true;
                if(!format_seektable_is_legal(&m->data.seek_table))
                    return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
            }
            else if(m->type == METADATA_TYPE_VORBIS_COMMENT) {
                if(metadata_has_vorbis_comment) /* only one is allowed */
                    return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                metadata_has_vorbis_comment = true;
            }
            else if(m->type == METADATA_TYPE_CUESHEET) {
                if(!format_cuesheet_is_legal(&m->data.cue_sheet, m->data.cue_sheet.is_cd, /*violation=*/0))
                    return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
            }
            else if(m->type == METADATA_TYPE_PICTURE) {
                if(!format_picture_is_legal(&m->data.picture, /*violation=*/0))
                    return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                if(m->data.picture.type == STREAM_METADATA_PICTURE_TYPE_FILE_ICON_STANDARD) {
                    if(metadata_picture_has_type1) /* there should only be 1 per stream */
                        return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                    metadata_picture_has_type1 = true;
                    /* standard icon must be 32x32 pixel PNG */
                    if(
                        m->data.picture.type == STREAM_METADATA_PICTURE_TYPE_FILE_ICON_STANDARD &&
                        (
                            (strcmp(m->data.picture.mime_type, "image/png") && strcmp(m->data.picture.mime_type, "-->")) ||
                            m->data.picture.width != 32 ||
                            m->data.picture.height != 32
                        )
                    )
                        return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                }
                else if(m->data.picture.type == STREAM_METADATA_PICTURE_TYPE_FILE_ICON) {
                    if(metadata_picture_has_type2) /* there should only be 1 per stream */
                        return STREAM_ENCODER_INIT_STATUS_INVALID_METADATA;
                    metadata_picture_has_type2 = true;
                }
            }
        }

        encoder->private_->input_capacity = 0;
        for(i = 0; i < encoder->protected_->channels; i++) {
            encoder->private_->integer_signal_unaligned[i] = encoder->private_->integer_signal[i] = 0;
    #ifndef INTEGER_ONLY_LIBRARY
            encoder->private_->real_signal_unaligned[i] = encoder->private_->real_signal[i] = 0;
    #endif
        }
        for(i = 0; i < 2; i++) {
            encoder->private_->integer_signal_mid_side_unaligned[i] = encoder->private_->integer_signal_mid_side[i] = 0;
    #ifndef INTEGER_ONLY_LIBRARY
            encoder->private_->real_signal_mid_side_unaligned[i] = encoder->private_->real_signal_mid_side[i] = 0;
    #endif
        }
    #ifndef INTEGER_ONLY_LIBRARY
        for(i = 0; i < encoder->protected_->num_apodizations; i++)
            encoder->private_->window_unaligned[i] = encoder->private_->window[i] = 0;
        encoder->private_->windowed_signal_unaligned = encoder->private_->windowed_signal = 0;
    #endif
        for(i = 0; i < encoder->protected_->channels; i++) {
            encoder->private_->residual_workspace_unaligned[i][0] = encoder->private_->residual_workspace[i][0] = 0;
            encoder->private_->residual_workspace_unaligned[i][1] = encoder->private_->residual_workspace[i][1] = 0;
            encoder->private_->best_subframe[i] = 0;
        }
        for(i = 0; i < 2; i++) {
            encoder->private_->residual_workspace_mid_side_unaligned[i][0] = encoder->private_->residual_workspace_mid_side[i][0] = 0;
            encoder->private_->residual_workspace_mid_side_unaligned[i][1] = encoder->private_->residual_workspace_mid_side[i][1] = 0;
            encoder->private_->best_subframe_mid_side[i] = 0;
        }
        encoder->private_->abs_residual_partition_sums_unaligned = encoder->private_->abs_residual_partition_sums = 0;
        encoder->private_->raw_bits_per_partition_unaligned = encoder->private_->raw_bits_per_partition = 0;
    #ifndef INTEGER_ONLY_LIBRARY
        encoder->private_->loose_mid_side_stereo_frames = (unsigned)((double)encoder->protected_->sample_rate * 0.4 / (double)encoder->protected_->blocksize + 0.5);
    #else
        /* 26214 is the approximate fixed-point equivalent to 0.4 (0.4 * 2^16) */
        /* sample rate can be up to 655350 Hz, and thus use 20 bits, so we do the multiply&divide by hand */
        ASSERT(MAX_SAMPLE_RATE <= 655350);
        ASSERT(MAX_BLOCK_SIZE <= 65535);
        ASSERT(encoder->protected_->sample_rate <= 655350);
        ASSERT(encoder->protected_->blocksize <= 65535);
        encoder->private_->loose_mid_side_stereo_frames = (unsigned)fixedpoint_trunc((((u64)(encoder->protected_->sample_rate) * (u64)(26214)) << 16) / (encoder->protected_->blocksize<<16) + FP_ONE_HALF);
    #endif
        if(encoder->private_->loose_mid_side_stereo_frames == 0)
            encoder->private_->loose_mid_side_stereo_frames = 1;
        encoder->private_->loose_mid_side_stereo_frame_count = 0;
        encoder->private_->current_sample_number = 0;
        encoder->private_->current_frame_number = 0;

        encoder->private_->use_wide_by_block = (encoder->protected_->bits_per_sample + bitmath_ilog2(encoder->protected_->blocksize)+1 > 30);
        encoder->private_->use_wide_by_order = (encoder->protected_->bits_per_sample + bitmath_ilog2(flac_max(encoder->protected_->max_lpc_order, MAX_FIXED_ORDER))+1 > 30); /*@@@ need to use this? */
        encoder->private_->use_wide_by_partition = (false); /*@@@ need to set this */

        /*
         * get the CPU info and set the function pointers
         */
        cpu_info(&encoder->private_->cpuinfo);
        /* first default to the non-asm routines */
    #ifndef INTEGER_ONLY_LIBRARY
        encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation;
    #endif
        encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_;
        encoder->private_->local_fixed_compute_best_predictor = fixed_compute_best_predictor;
        encoder->private_->local_fixed_compute_best_predictor_wide = fixed_compute_best_predictor_wide;
    #ifndef INTEGER_ONLY_LIBRARY
        encoder->private_->local_lpc_compute_residual_from_qlp_coefficients = lpc_compute_residual_from_qlp_coefficients;
        encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit = lpc_compute_residual_from_qlp_coefficients_wide;
        encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients;
    #endif
        /* now override with asm where appropriate */
    #ifndef INTEGER_ONLY_LIBRARY
    # ifndef NO_ASM
        if(encoder->private_->cpuinfo.use_asm) {
    #  ifdef CPU_IA32
            ASSERT(encoder->private_->cpuinfo.type == CPUINFO_TYPE_IA32);
    #   ifdef HAS_NASM
            if(encoder->private_->cpuinfo.ia32.sse) {
                if(encoder->protected_->max_lpc_order < 4)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32_sse_lag_4;
                else if(encoder->protected_->max_lpc_order < 8)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32_sse_lag_8;
                else if(encoder->protected_->max_lpc_order < 12)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32_sse_lag_12;
                else if(encoder->protected_->max_lpc_order < 16)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32_sse_lag_16;
                else
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32;
            }
            else
                encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_asm_ia32;

            encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit = lpc_compute_residual_from_qlp_coefficients_wide_asm_ia32; /* OPT_IA32: was really necessary for GCC < 4.9 */
            if(encoder->private_->cpuinfo.ia32.mmx) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients = lpc_compute_residual_from_qlp_coefficients_asm_ia32;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_asm_ia32_mmx;
            }
            else {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients = lpc_compute_residual_from_qlp_coefficients_asm_ia32;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_asm_ia32;
            }

            if(encoder->private_->cpuinfo.ia32.mmx && encoder->private_->cpuinfo.ia32.cmov)
                encoder->private_->local_fixed_compute_best_predictor = fixed_compute_best_predictor_asm_ia32_mmx_cmov;
    #   endif /* HAS_NASM */
    #   ifdef HAS_X86INTRIN
    #    if defined SSE_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.sse) {
                if(encoder->protected_->max_lpc_order < 4)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_4;
                else if(encoder->protected_->max_lpc_order < 8)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_8;
                else if(encoder->protected_->max_lpc_order < 12)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_12;
                else if(encoder->protected_->max_lpc_order < 16)
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_16;
                else
                    encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation;
            }
    #    endif

    #    ifdef SSE2_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.sse2) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients       = lpc_compute_residual_from_qlp_coefficients_intrin_sse2;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_16_intrin_sse2;
            }
    #    endif
    #    ifdef SSE4_1_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.sse41) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients       = lpc_compute_residual_from_qlp_coefficients_intrin_sse41;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit = lpc_compute_residual_from_qlp_coefficients_wide_intrin_sse41;
            }
    #    endif
    #    ifdef AVX2_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.avx2) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_16_intrin_avx2;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients       = lpc_compute_residual_from_qlp_coefficients_intrin_avx2;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit = lpc_compute_residual_from_qlp_coefficients_wide_intrin_avx2;
            }
    #    endif

    #    ifdef SSE2_SUPPORTED
            if (encoder->private_->cpuinfo.ia32.sse2) {
                encoder->private_->local_fixed_compute_best_predictor      = fixed_compute_best_predictor_intrin_sse2;
                encoder->private_->local_fixed_compute_best_predictor_wide = fixed_compute_best_predictor_wide_intrin_sse2;
            }
    #    endif
    #    ifdef SSSE3_SUPPORTED
            if (encoder->private_->cpuinfo.ia32.ssse3) {
                encoder->private_->local_fixed_compute_best_predictor      = fixed_compute_best_predictor_intrin_ssse3;
                encoder->private_->local_fixed_compute_best_predictor_wide = fixed_compute_best_predictor_wide_intrin_ssse3;
            }
    #    endif
    #   endif /* HAS_X86INTRIN */
    #  elif defined CPU_X86_64
            ASSERT(encoder->private_->cpuinfo.type == CPUINFO_TYPE_X86_64);
    #   ifdef HAS_X86INTRIN
    #    ifdef SSE_SUPPORTED
            if(encoder->protected_->max_lpc_order < 4)
                encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_4;
            else if(encoder->protected_->max_lpc_order < 8)
                encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_8;
            else if(encoder->protected_->max_lpc_order < 12)
                encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_12;
            else if(encoder->protected_->max_lpc_order < 16)
                encoder->private_->local_lpc_compute_autocorrelation = lpc_compute_autocorrelation_intrin_sse_lag_16;
    #    endif

    #    ifdef SSE2_SUPPORTED
            encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_16_intrin_sse2;
    #    endif
    #    ifdef SSE4_1_SUPPORTED
            if(encoder->private_->cpuinfo.x86.sse41) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients = lpc_compute_residual_from_qlp_coefficients_intrin_sse41;
            }
    #    endif
    #    ifdef AVX2_SUPPORTED
            if(encoder->private_->cpuinfo.x86.avx2) {
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit = lpc_compute_residual_from_qlp_coefficients_16_intrin_avx2;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients       = lpc_compute_residual_from_qlp_coefficients_intrin_avx2;
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit = lpc_compute_residual_from_qlp_coefficients_wide_intrin_avx2;
            }
    #    endif

    #    ifdef SSE2_SUPPORTED
            encoder->private_->local_fixed_compute_best_predictor      = fixed_compute_best_predictor_intrin_sse2;
            encoder->private_->local_fixed_compute_best_predictor_wide = fixed_compute_best_predictor_wide_intrin_sse2;
    #    endif
    #    ifdef SSSE3_SUPPORTED
            if (encoder->private_->cpuinfo.x86.ssse3) {
                encoder->private_->local_fixed_compute_best_predictor      = fixed_compute_best_predictor_intrin_ssse3;
                encoder->private_->local_fixed_compute_best_predictor_wide = fixed_compute_best_predictor_wide_intrin_ssse3;
            }
    #    endif
    #   endif /* HAS_X86INTRIN */
    #  endif /* CPU_... */
        }
    # endif /* !NO_ASM */
    #endif /* !INTEGER_ONLY_LIBRARY */
    #if !defined NO_ASM && defined HAS_X86INTRIN
        if(encoder->private_->cpuinfo.use_asm) {
    # if defined CPU_IA32
    #  ifdef SSE2_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.sse2)
                encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_sse2;
    #  endif
    #  ifdef SSSE3_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.ssse3)
                encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_ssse3;
    #  endif
    #  ifdef AVX2_SUPPORTED
            if(encoder->private_->cpuinfo.ia32.avx2)
                encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_avx2;
    #  endif
    # elif defined CPU_X86_64
    #  ifdef SSE2_SUPPORTED
            encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_sse2;
    #  endif
    #  ifdef SSSE3_SUPPORTED
            if(encoder->private_->cpuinfo.x86.ssse3)
                encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_ssse3;
    #  endif
    #  ifdef AVX2_SUPPORTED
            if(encoder->private_->cpuinfo.x86.avx2)
                encoder->private_->local_precompute_partition_info_sums = precompute_partition_info_sums_intrin_avx2;
    #  endif
    # endif /* CPU_... */
        }
    #endif /* !NO_ASM && HAS_X86INTRIN */
        /* finally override based on wide-ness if necessary */
        if(encoder->private_->use_wide_by_block) {
            encoder->private_->local_fixed_compute_best_predictor = encoder->private_->local_fixed_compute_best_predictor_wide;
        }

        /* set state to OK; from here on, errors are fatal and we'll override the state then */
        encoder->protected_->state = STREAM_ENCODER_OK;

    #if HAS_OGG
        encoder->private_->is_ogg = is_ogg;
        if(is_ogg && !ogg_encoder_aspect_init(&encoder->protected_->ogg_encoder_aspect)) {
            encoder->protected_->state = STREAM_ENCODER_OGG_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }
    #endif

        encoder->private_->read_callback = read_callback;
        encoder->private_->write_callback = write_callback;
        encoder->private_->seek_callback = seek_callback;
        encoder->private_->tell_callback = tell_callback;
        encoder->private_->metadata_callback = metadata_callback;
        encoder->private_->client_data = client_data;

        if(!resize_buffers_(encoder, encoder->protected_->blocksize)) {
            /* the above function sets the state for us in case of an error */
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        if(!bitwriter_init(encoder->private_->frame)) {
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        /*
         * Set up the verify stuff if necessary
         */
        if(encoder->protected_->verify) {
            /*
             * First, set up the fifo which will hold the
             * original signal to compare against
             */
            encoder->private_->verify.input_fifo.size = encoder->protected_->blocksize+OVERREAD_;
            for(i = 0; i < encoder->protected_->channels; i++) {
                if(0 == (encoder->private_->verify.input_fifo.data[i] = (i32*) safe_malloc_mul_2op_p(sizeof(i32), /*times*/encoder->private_->verify.input_fifo.size))) {
                    encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
                    return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
                }
            }
            encoder->private_->verify.input_fifo.tail = 0;

            /*
             * Now set up a stream decoder for verification
             */
            if(0 == encoder->private_->verify.decoder) {
                encoder->private_->verify.decoder = stream_decoder_new();
                if(0 == encoder->private_->verify.decoder) {
                    encoder->protected_->state = STREAM_ENCODER_VERIFY_DECODER_ERROR;
                    return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
                }
            }

            if(stream_decoder_init_stream(encoder->private_->verify.decoder, verify_read_callback_, /*seek_callback=*/0, /*tell_callback=*/0, /*length_callback=*/0, /*eof_callback=*/0, verify_write_callback_, verify_metadata_callback_, verify_error_callback_, /*client_data=*/encoder) != STREAM_DECODER_INIT_STATUS_OK) {
                encoder->protected_->state = STREAM_ENCODER_VERIFY_DECODER_ERROR;
                return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
            }
        }
        encoder->private_->verify.error_stats.absolute_sample = 0;
        encoder->private_->verify.error_stats.frame_number = 0;
        encoder->private_->verify.error_stats.channel = 0;
        encoder->private_->verify.error_stats.sample = 0;
        encoder->private_->verify.error_stats.expected = 0;
        encoder->private_->verify.error_stats.got = 0;

        /*
         * These must be done before we write any metadata, because that
         * calls the write_callback, which uses these values.
         */
        encoder->private_->first_seekpoint_to_check = 0;
        encoder->private_->samples_written = 0;
        encoder->protected_->streaminfo_offset = 0;
        encoder->protected_->seektable_offset = 0;
        encoder->protected_->audio_offset = 0;

        /*
         * write the stream header
         */
        if(encoder->protected_->verify)
            encoder->private_->verify.state_hint = ENCODER_IN_MAGIC;
        if(!bitwriter_write_raw_uint32(encoder->private_->frame, STREAM_SYNC, STREAM_SYNC_LEN)) {
            encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }
        if(!write_bitbuffer_(encoder, 0, /*is_last_block=*/false)) {
            /* the above function sets the state for us in case of an error */
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        /*
         * write the STREAMINFO metadata block
         */
        if(encoder->protected_->verify)
            encoder->private_->verify.state_hint = ENCODER_IN_METADATA;
        encoder->private_->streaminfo.type = METADATA_TYPE_STREAMINFO;
        encoder->private_->streaminfo.is_last = false; /* we will have at a minimum a VORBIS_COMMENT afterwards */
        encoder->private_->streaminfo.length = STREAM_METADATA_STREAMINFO_LENGTH;
        encoder->private_->streaminfo.data.stream_info.min_blocksize = encoder->protected_->blocksize; /* this encoder uses the same blocksize for the whole stream */
        encoder->private_->streaminfo.data.stream_info.max_blocksize = encoder->protected_->blocksize;
        encoder->private_->streaminfo.data.stream_info.min_framesize = 0; /* we don't know this yet; have to fill it in later */
        encoder->private_->streaminfo.data.stream_info.max_framesize = 0; /* we don't know this yet; have to fill it in later */
        encoder->private_->streaminfo.data.stream_info.sample_rate = encoder->protected_->sample_rate;
        encoder->private_->streaminfo.data.stream_info.channels = encoder->protected_->channels;
        encoder->private_->streaminfo.data.stream_info.bits_per_sample = encoder->protected_->bits_per_sample;
        encoder->private_->streaminfo.data.stream_info.total_samples = encoder->protected_->total_samples_estimate; /* we will replace this later with the real total */
        memset(encoder->private_->streaminfo.data.stream_info.md5sum, 0, 16); /* we don't know this yet; have to fill it in later */
        if(encoder->protected_->do_md5)
            MD5Init(&encoder->private_->md5context);
        if(!add_metadata_block(&encoder->private_->streaminfo, encoder->private_->frame)) {
            encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }
        if(!write_bitbuffer_(encoder, 0, /*is_last_block=*/false)) {
            /* the above function sets the state for us in case of an error */
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        /*
         * Now that the STREAMINFO block is written, we can init this to an
         * absurdly-high value...
         */
        encoder->private_->streaminfo.data.stream_info.min_framesize = (1u << STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN) - 1;
        /* ... and clear this to 0 */
        encoder->private_->streaminfo.data.stream_info.total_samples = 0;

        /*
         * Check to see if the supplied metadata contains a VORBIS_COMMENT;
         * if not, we will write an empty one (add_metadata_block()
         * automatically supplies the vendor string).
         *
         * WATCHOUT: the Ogg FLAC mapping requires us to write this block after
         * the STREAMINFO.  (In the case that metadata_has_vorbis_comment is
         * true it will have already insured that the metadata list is properly
         * ordered.)
         */
        if(!metadata_has_vorbis_comment) {
            StreamMetadata vorbis_comment;
            vorbis_comment.type = METADATA_TYPE_VORBIS_COMMENT;
            vorbis_comment.is_last = (encoder->protected_->num_metadata_blocks == 0);
            vorbis_comment.length = 4 + 4; /* MAGIC NUMBER */
            vorbis_comment.data.vorbis_comment.vendor_string.length = 0;
            vorbis_comment.data.vorbis_comment.vendor_string.entry = 0;
            vorbis_comment.data.vorbis_comment.num_comments = 0;
            vorbis_comment.data.vorbis_comment.comments = 0;
            if(!add_metadata_block(&vorbis_comment, encoder->private_->frame)) {
                encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
            }
            if(!write_bitbuffer_(encoder, 0, /*is_last_block=*/false)) {
                /* the above function sets the state for us in case of an error */
                return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
            }
        }

        /*
         * write the user's metadata blocks
         */
        for(i = 0; i < encoder->protected_->num_metadata_blocks; i++) {
            encoder->protected_->metadata[i]->is_last = (i == encoder->protected_->num_metadata_blocks - 1);
            if(!add_metadata_block(encoder->protected_->metadata[i], encoder->private_->frame)) {
                encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
            }
            if(!write_bitbuffer_(encoder, 0, /*is_last_block=*/false)) {
                /* the above function sets the state for us in case of an error */
                return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
            }
        }

        /* now that all the metadata is written, we save the stream offset */
        if(encoder->private_->tell_callback && encoder->private_->tell_callback(encoder, &encoder->protected_->audio_offset, encoder->private_->client_data) == STREAM_ENCODER_TELL_STATUS_ERROR) { /* STREAM_ENCODER_TELL_STATUS_UNSUPPORTED just means we didn't get the offset; no error */
            encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        if(encoder->protected_->verify)
            encoder->private_->verify.state_hint = ENCODER_IN_AUDIO;

        return STREAM_ENCODER_INIT_STATUS_OK;
        */
}

pub fn flac_stream_encoder_init_stream(
        encoder:           *mut StreamEncoder,
        write_callback:    FLAC__StreamEncoderWriteCallback,
        seek_callback:     FLAC__StreamEncoderSeekCallback,
        tell_callback:     FLAC__StreamEncoderTellCallback,
        metadata_callback: FLAC__StreamEncoderMetadataCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_stream_internal_(
            encoder,
            /*read_callback=*/0,
            write_callback,
            seek_callback,
            tell_callback,
            metadata_callback,
            client_data,
            /*is_ogg=*/false
        );
        */
}

pub fn flac_stream_encoder_init_ogg_stream(
        encoder:           *mut StreamEncoder,
        read_callback:     FLAC__StreamEncoderReadCallback,
        write_callback:    FLAC__StreamEncoderWriteCallback,
        seek_callback:     FLAC__StreamEncoderSeekCallback,
        tell_callback:     FLAC__StreamEncoderTellCallback,
        metadata_callback: FLAC__StreamEncoderMetadataCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_stream_internal_(
            encoder,
            read_callback,
            write_callback,
            seek_callback,
            tell_callback,
            metadata_callback,
            client_data,
            /*is_ogg=*/true
        );
        */
}

#[ignore]
pub fn init_file_internal(
        encoder:           *mut StreamEncoder,
        file:              *mut libc::FILE,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            StreamEncoderInitStatus init_status;

        ASSERT(0 != encoder);
        ASSERT(0 != file);

        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED;

        /* double protection */
        if(file == 0) {
            encoder->protected_->state = STREAM_ENCODER_IO_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        /*
         * To make sure that our file does not go unclosed after an error, we
         * must assign the FILE pointer before any further error can occur in
         * this routine.
         */
        if(file == stdout)
            file = get_binary_stdout_(); /* just to be safe */

    #ifdef _WIN32
        /*
         * Windows can suffer quite badly from disk fragmentation. This can be
         * reduced significantly by setting the output buffer size to be 10MB.
         */
        setvbuf(file, NULL, _IOFBF, 10*1024*1024);
    #endif
        encoder->private_->file = file;

        encoder->private_->progress_callback = progress_callback;
        encoder->private_->bytes_written = 0;
        encoder->private_->samples_written = 0;
        encoder->private_->frames_written = 0;

        init_status = init_stream_internal_(
            encoder,
            encoder->private_->file == stdout? 0 : is_ogg? file_read_callback_ : 0,
            file_write_callback_,
            encoder->private_->file == stdout? 0 : file_seek_callback_,
            encoder->private_->file == stdout? 0 : file_tell_callback_,
            /*metadata_callback=*/0,
            client_data,
            is_ogg
        );
        if(init_status != STREAM_ENCODER_INIT_STATUS_OK) {
            /* the above function sets the state for us in case of an error */
            return init_status;
        }

        {
            unsigned blocksize = stream_encoder_get_blocksize(encoder);

            ASSERT(blocksize != 0);
            encoder->private_->total_frames_estimate = (unsigned)((stream_encoder_get_total_samples_estimate(encoder) + blocksize - 1) / blocksize);
        }

        return init_status;
        */
}

#[ignore]
pub fn flac_stream_encoder_init_file(
        encoder:           *mut StreamEncoder,
        file:              *mut libc::FILE,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_FILE_internal_(encoder, file, progress_callback, client_data, /*is_ogg=*/false);
        */
}

#[ignore]
pub fn flac_stream_encoder_init_ogg_file(
        encoder:           *mut StreamEncoder,
        file:              *mut libc::FILE,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_FILE_internal_(encoder, file, progress_callback, client_data, /*is_ogg=*/true);
        */
}

#[ignore]
pub fn init_file_internal_with_filename(
        encoder:           *mut StreamEncoder,
        filename:          *const u8,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            FILE *file;

        ASSERT(0 != encoder);

        /*
         * To make sure that our file does not go unclosed after an error, we
         * have to do the same entrance checks here that are later performed
         * in stream_encoder_init_FILE() before the FILE* is assigned.
         */
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED;

        file = filename? flac_fopen(filename, "w+b") : stdout;

        if(file == 0) {
            encoder->protected_->state = STREAM_ENCODER_IO_ERROR;
            return STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR;
        }

        return init_FILE_internal_(encoder, file, progress_callback, client_data, is_ogg);
        */
}

#[ignore]
pub fn flac_stream_encoder_init_file_with_filename(
        encoder:           *mut StreamEncoder,
        filename:          *const u8,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_file_internal_(encoder, filename, progress_callback, client_data, /*is_ogg=*/false);
        */
}

#[ignore]
pub fn flac_stream_encoder_init_ogg_file_with_filename(
        encoder:           *mut StreamEncoder,
        filename:          *const u8,
        progress_callback: FLAC__StreamEncoderProgressCallback,
        client_data:       *mut c_void) -> StreamEncoderInitStatus {
    
    todo!();
        /*
            return init_file_internal_(encoder, filename, progress_callback, client_data, /*is_ogg=*/true);
        */
}

pub fn flac_stream_encoder_finish(encoder: *mut StreamEncoder) -> bool {
    
    todo!();
        /*
            bool error = false;

        ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);

        if(encoder->protected_->state == STREAM_ENCODER_UNINITIALIZED)
            return true;

        if(encoder->protected_->state == STREAM_ENCODER_OK && !encoder->private_->is_being_deleted) {
            if(encoder->private_->current_sample_number != 0) {
                const bool is_fractional_block = encoder->protected_->blocksize != encoder->private_->current_sample_number;
                encoder->protected_->blocksize = encoder->private_->current_sample_number;
                if(!process_frame_(encoder, is_fractional_block, /*is_last_block=*/true))
                    error = true;
            }
        }

        if(encoder->protected_->do_md5)
            MD5Final(encoder->private_->streaminfo.data.stream_info.md5sum, &encoder->private_->md5context);

        if(!encoder->private_->is_being_deleted) {
            if(encoder->protected_->state == STREAM_ENCODER_OK) {
                if(encoder->private_->seek_callback) {
    #if HAS_OGG
                    if(encoder->private_->is_ogg)
                        update_ogg_metadata_(encoder);
                    else
    #endif
                    update_metadata_(encoder);

                    /* check if an error occurred while updating metadata */
                    if(encoder->protected_->state != STREAM_ENCODER_OK)
                        error = true;
                }
                if(encoder->private_->metadata_callback)
                    encoder->private_->metadata_callback(encoder, &encoder->private_->streaminfo, encoder->private_->client_data);
            }

            if(encoder->protected_->verify && 0 != encoder->private_->verify.decoder && !stream_decoder_finish(encoder->private_->verify.decoder)) {
                if(!error)
                    encoder->protected_->state = STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA;
                error = true;
            }
        }

        if(0 != encoder->private_->file) {
            if(encoder->private_->file != stdout)
                fclose(encoder->private_->file);
            encoder->private_->file = 0;
        }

    #if HAS_OGG
        if(encoder->private_->is_ogg)
            ogg_encoder_aspect_finish(&encoder->protected_->ogg_encoder_aspect);
    #endif

        free_(encoder);
        set_defaults_(encoder);

        if(!error)
            encoder->protected_->state = STREAM_ENCODER_UNINITIALIZED;

        return !error;
        */
}

pub fn flac_stream_encoder_set_ogg_serial_number(
        encoder: *mut StreamEncoder,
        value:   i64) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
    #if HAS_OGG
        /* can't check encoder->private_->is_ogg since that's not set until init time */
        ogg_encoder_aspect_set_serial_number(&encoder->protected_->ogg_encoder_aspect, value);
        return true;
    #else
        (void)value;
        return false;
    #endif
        */
}

pub fn flac_stream_encoder_set_verify(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
    #ifndef MANDATORY_VERIFY_WHILE_ENCODING
        encoder->protected_->verify = value;
    #endif
        return true;
        */
}

pub fn flac_stream_encoder_set_streamable_subset(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->streamable_subset = value;
        return true;
        */
}

#[inline] pub fn flac_stream_encoder_set_do_md5(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->do_md5 = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_channels(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->channels = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_bits_per_sample(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->bits_per_sample = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_sample_rate(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->sample_rate = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_compression_level(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            bool ok = true;
        ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        if(value >= sizeof(compression_levels_)/sizeof(compression_levels_[0]))
            value = sizeof(compression_levels_)/sizeof(compression_levels_[0]) - 1;
        ok &= stream_encoder_set_do_mid_side_stereo          (encoder, compression_levels_[value].do_mid_side_stereo);
        ok &= stream_encoder_set_loose_mid_side_stereo       (encoder, compression_levels_[value].loose_mid_side_stereo);
    #ifndef INTEGER_ONLY_LIBRARY
    #if 1
        ok &= stream_encoder_set_apodization                 (encoder, compression_levels_[value].apodization);
    #else
        /* equivalent to -A tukey(0.5) */
        encoder->protected_->num_apodizations = 1;
        encoder->protected_->apodizations[0].type = APODIZATION_TUKEY;
        encoder->protected_->apodizations[0].parameters.tukey.p = 0.5;
    #endif
    #endif
        ok &= stream_encoder_set_max_lpc_order               (encoder, compression_levels_[value].max_lpc_order);
        ok &= stream_encoder_set_qlp_coeff_precision         (encoder, compression_levels_[value].qlp_coeff_precision);
        ok &= stream_encoder_set_do_qlp_coeff_prec_search    (encoder, compression_levels_[value].do_qlp_coeff_prec_search);
        ok &= stream_encoder_set_do_escape_coding            (encoder, compression_levels_[value].do_escape_coding);
        ok &= stream_encoder_set_do_exhaustive_model_search  (encoder, compression_levels_[value].do_exhaustive_model_search);
        ok &= stream_encoder_set_min_residual_partition_order(encoder, compression_levels_[value].min_residual_partition_order);
        ok &= stream_encoder_set_max_residual_partition_order(encoder, compression_levels_[value].max_residual_partition_order);
        ok &= stream_encoder_set_rice_parameter_search_dist  (encoder, compression_levels_[value].rice_parameter_search_dist);
        return ok;
        */
}

pub fn flac_stream_encoder_set_blocksize(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->blocksize = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_do_mid_side_stereo(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->do_mid_side_stereo = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_loose_mid_side_stereo(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->loose_mid_side_stereo = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_apodization(
        encoder:       *mut StreamEncoder,
        specification: *const u8) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        ASSERT(0 != specification);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
    #ifdef INTEGER_ONLY_LIBRARY
        (void)specification; /* silently ignore since we haven't integerized; will always use a rectangular window */
    #else
        encoder->protected_->num_apodizations = 0;
        while(1) {
            const char *s = strchr(specification, ';');
            const size_t n = s? (size_t)(s - specification) : strlen(specification);
            if     (n==8  && 0 == strncmp("bartlett"     , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_BARTLETT;
            else if(n==13 && 0 == strncmp("bartlett_hann", specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_BARTLETT_HANN;
            else if(n==8  && 0 == strncmp("blackman"     , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_BLACKMAN;
            else if(n==26 && 0 == strncmp("blackman_harris_4term_92db", specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_BLACKMAN_HARRIS_4TERM_92DB_SIDELOBE;
            else if(n==6  && 0 == strncmp("connes"       , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_CONNES;
            else if(n==7  && 0 == strncmp("flattop"      , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_FLATTOP;
            else if(n>7   && 0 == strncmp("gauss("       , specification, 6)) {
                real stddev = (real)strtod(specification+6, 0);
                if (stddev > 0.0 && stddev <= 0.5) {
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.gauss.stddev = stddev;
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_GAUSS;
                }
            }
            else if(n==7  && 0 == strncmp("hamming"      , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_HAMMING;
            else if(n==4  && 0 == strncmp("hann"         , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_HANN;
            else if(n==13 && 0 == strncmp("kaiser_bessel", specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_KAISER_BESSEL;
            else if(n==7  && 0 == strncmp("nuttall"      , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_NUTTALL;
            else if(n==9  && 0 == strncmp("rectangle"    , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_RECTANGLE;
            else if(n==8  && 0 == strncmp("triangle"     , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_TRIANGLE;
            else if(n>7   && 0 == strncmp("tukey("       , specification, 6)) {
                real p = (real)strtod(specification+6, 0);
                if (p >= 0.0 && p <= 1.0) {
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.tukey.p = p;
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_TUKEY;
                }
            }
            else if(n>15   && 0 == strncmp("partial_tukey("       , specification, 14)) {
                i32 tukey_parts = (i32)strtod(specification+14, 0);
                const char *si_1 = strchr(specification, '/');
                real overlap = si_1?flac_min((real)strtod(si_1+1, 0),0.99f):0.1f;
                real overlap_units = 1.0f/(1.0f - overlap) - 1.0f;
                const char *si_2 = strchr((si_1?(si_1+1):specification), '/');
                real tukey_p = si_2?(real)strtod(si_2+1, 0):0.2f;

                if (tukey_parts <= 1) {
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.tukey.p = tukey_p;
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_TUKEY;
                }else if (encoder->protected_->num_apodizations + tukey_parts < 32){
                    i32 m;
                    for(m = 0; m < tukey_parts; m++){
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.p = tukey_p;
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.start = m/(tukey_parts+overlap_units);
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.end = (m+1+overlap_units)/(tukey_parts+overlap_units);
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_PARTIAL_TUKEY;
                    }
                }
            }
            else if(n>16   && 0 == strncmp("punchout_tukey("       , specification, 15)) {
                i32 tukey_parts = (i32)strtod(specification+15, 0);
                const char *si_1 = strchr(specification, '/');
                real overlap = si_1?flac_min((real)strtod(si_1+1, 0),0.99f):0.2f;
                real overlap_units = 1.0f/(1.0f - overlap) - 1.0f;
                const char *si_2 = strchr((si_1?(si_1+1):specification), '/');
                real tukey_p = si_2?(real)strtod(si_2+1, 0):0.2f;

                if (tukey_parts <= 1) {
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.tukey.p = tukey_p;
                    encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_TUKEY;
                }else if (encoder->protected_->num_apodizations + tukey_parts < 32){
                    i32 m;
                    for(m = 0; m < tukey_parts; m++){
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.p = tukey_p;
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.start = m/(tukey_parts+overlap_units);
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations].parameters.multiple_tukey.end = (m+1+overlap_units)/(tukey_parts+overlap_units);
                        encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_PUNCHOUT_TUKEY;
                    }
                }
            }
            else if(n==5  && 0 == strncmp("welch"        , specification, n))
                encoder->protected_->apodizations[encoder->protected_->num_apodizations++].type = APODIZATION_WELCH;
            if (encoder->protected_->num_apodizations == 32)
                break;
            if (s)
                specification = s+1;
            else
                break;
        }
        if(encoder->protected_->num_apodizations == 0) {
            encoder->protected_->num_apodizations = 1;
            encoder->protected_->apodizations[0].type = APODIZATION_TUKEY;
            encoder->protected_->apodizations[0].parameters.tukey.p = 0.5;
        }
    #endif
        return true;
        */
}

pub fn flac_stream_encoder_set_max_lpc_order(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->max_lpc_order = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_qlp_coeff_precision(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->qlp_coeff_precision = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_do_qlp_coeff_prec_search(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->do_qlp_coeff_prec_search = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_do_escape_coding(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
    #if 0
        /*@@@ deprecated: */
        encoder->protected_->do_escape_coding = value;
    #else
        (void)value;
    #endif
        return true;
        */
}

pub fn flac_stream_encoder_set_do_exhaustive_model_search(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->do_exhaustive_model_search = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_min_residual_partition_order(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->min_residual_partition_order = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_max_residual_partition_order(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->max_residual_partition_order = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_rice_parameter_search_dist(
        encoder: *mut StreamEncoder,
        value:   u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
    #if 0
        /*@@@ deprecated: */
        encoder->protected_->rice_parameter_search_dist = value;
    #else
        (void)value;
    #endif
        return true;
        */
}

pub fn flac_stream_encoder_set_total_samples_estimate(
        encoder: *mut StreamEncoder,
        value:   u64) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->protected_->total_samples_estimate = value;
        return true;
        */
}

pub fn flac_stream_encoder_set_metadata(
        encoder:    *mut StreamEncoder,
        metadata:   *mut *mut StreamMetadata,
        num_blocks: u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        if(0 == metadata)
            num_blocks = 0;
        if(0 == num_blocks)
            metadata = 0;
        /* realloc() does not do exactly what we want so... */
        if(encoder->protected_->metadata) {
            free(encoder->protected_->metadata);
            encoder->protected_->metadata = 0;
            encoder->protected_->num_metadata_blocks = 0;
        }
        if(num_blocks) {
            StreamMetadata **m;
            if(0 == (m = (StreamMetadata**) safe_malloc_mul_2op_p(sizeof(m[0]), /*times*/num_blocks)))
                return false;
            memcpy(m, metadata, sizeof(m[0]) * num_blocks);
            encoder->protected_->metadata = m;
            encoder->protected_->num_metadata_blocks = num_blocks;
        }
    #if HAS_OGG
        if(!ogg_encoder_aspect_set_num_metadata(&encoder->protected_->ogg_encoder_aspect, num_blocks))
            return false;
    #endif
        return true;
        */
}

/**
  | These three functions are not static,
  | but not publically exposed in include/FLAC/
  | either. They are used by the test suite.
  |
  */
#[inline] pub fn flac_stream_encoder_disable_constant_subframes(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->private_->disable_constant_subframes = value;
        return true;
        */
}

#[inline] pub fn flac_stream_encoder_disable_fixed_subframes(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->private_->disable_fixed_subframes = value;
        return true;
        */
}

#[inline] pub fn flac_stream_encoder_disable_verbatim_subframes(
        encoder: *mut StreamEncoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_UNINITIALIZED)
            return false;
        encoder->private_->disable_verbatim_subframes = value;
        return true;
        */
}

pub fn flac_stream_encoder_get_state(encoder: *const StreamEncoder) -> StreamEncoderState {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->state;
        */
}

pub fn flac_stream_encoder_get_verify_decoder_state(encoder: *const StreamEncoder) -> StreamDecoderState {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->verify)
            return stream_decoder_get_state(encoder->private_->verify.decoder);
        else
            return STREAM_DECODER_UNINITIALIZED;
        */
}

pub fn flac_stream_encoder_get_resolved_state_string(encoder: *const StreamEncoder) -> *const u8 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(encoder->protected_->state != STREAM_ENCODER_VERIFY_DECODER_ERROR)
            return StreamEncoderStateString[encoder->protected_->state];
        else
            return stream_decoder_get_resolved_state_string(encoder->private_->verify.decoder);
        */
}

pub fn flac_stream_encoder_get_verify_decoder_error_stats(
        encoder:         *const StreamEncoder,
        absolute_sample: *mut u64,
        frame_number:    *mut u32,
        channel:         *mut u32,
        sample:          *mut u32,
        expected:        *mut i32,
        got:             *mut i32)  {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        if(0 != absolute_sample)
            *absolute_sample = encoder->private_->verify.error_stats.absolute_sample;
        if(0 != frame_number)
            *frame_number = encoder->private_->verify.error_stats.frame_number;
        if(0 != channel)
            *channel = encoder->private_->verify.error_stats.channel;
        if(0 != sample)
            *sample = encoder->private_->verify.error_stats.sample;
        if(0 != expected)
            *expected = encoder->private_->verify.error_stats.expected;
        if(0 != got)
            *got = encoder->private_->verify.error_stats.got;
        */
}

pub fn flac_stream_encoder_get_verify(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->verify;
        */
}

pub fn flac_stream_encoder_get_streamable_subset(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->streamable_subset;
        */
}

#[inline] pub fn flac_stream_encoder_get_do_md5(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->do_md5;
        */
}

pub fn flac_stream_encoder_get_channels(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->channels;
        */
}

pub fn flac_stream_encoder_get_bits_per_sample(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->bits_per_sample;
        */
}

pub fn flac_stream_encoder_get_sample_rate(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->sample_rate;
        */
}

pub fn flac_stream_encoder_get_blocksize(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->blocksize;
        */
}

pub fn flac_stream_encoder_get_do_mid_side_stereo(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->do_mid_side_stereo;
        */
}

pub fn flac_stream_encoder_get_loose_mid_side_stereo(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->loose_mid_side_stereo;
        */
}

pub fn flac_stream_encoder_get_max_lpc_order(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->max_lpc_order;
        */
}

pub fn flac_stream_encoder_get_qlp_coeff_precision(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->qlp_coeff_precision;
        */
}

pub fn flac_stream_encoder_get_do_qlp_coeff_prec_search(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->do_qlp_coeff_prec_search;
        */
}

pub fn flac_stream_encoder_get_do_escape_coding(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->do_escape_coding;
        */
}

pub fn flac_stream_encoder_get_do_exhaustive_model_search(encoder: *const StreamEncoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->do_exhaustive_model_search;
        */
}

pub fn flac_stream_encoder_get_min_residual_partition_order(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->min_residual_partition_order;
        */
}

pub fn flac_stream_encoder_get_max_residual_partition_order(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->max_residual_partition_order;
        */
}

pub fn flac_stream_encoder_get_rice_parameter_search_dist(encoder: *const StreamEncoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->rice_parameter_search_dist;
        */
}

pub fn flac_stream_encoder_get_total_samples_estimate(encoder: *const StreamEncoder) -> u64 {
    
    todo!();
        /*
            ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        return encoder->protected_->total_samples_estimate;
        */
}

pub fn flac_stream_encoder_process(
        encoder: *mut StreamEncoder,
        buffer:  *const &[i32],
        samples: u32) -> bool {
    
    todo!();
        /*
            unsigned i, j = 0, channel;
        const unsigned channels = encoder->protected_->channels, blocksize = encoder->protected_->blocksize;

        ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        ASSERT(encoder->protected_->state == STREAM_ENCODER_OK);

        do {
            const unsigned n = flac_min(blocksize+OVERREAD_-encoder->private_->current_sample_number, samples-j);

            if(encoder->protected_->verify)
                append_to_verify_fifo_(&encoder->private_->verify.input_fifo, buffer, j, channels, n);

            for(channel = 0; channel < channels; channel++)
                memcpy(&encoder->private_->integer_signal[channel][encoder->private_->current_sample_number], &buffer[channel][j], sizeof(buffer[channel][0]) * n);

            if(encoder->protected_->do_mid_side_stereo) {
                ASSERT(channels == 2);
                /* "i <= blocksize" to overread 1 sample; see comment in OVERREAD_ decl */
                for(i = encoder->private_->current_sample_number; i <= blocksize && j < samples; i++, j++) {
                    encoder->private_->integer_signal_mid_side[1][i] = buffer[0][j] - buffer[1][j];
                    encoder->private_->integer_signal_mid_side[0][i] = (buffer[0][j] + buffer[1][j]) >> 1; /* NOTE: not the same as 'mid = (buffer[0][j] + buffer[1][j]) / 2' ! */
                }
            }
            else
                j += n;

            encoder->private_->current_sample_number += n;

            /* we only process if we have a full block + 1 extra sample; final block is always handled by stream_encoder_finish() */
            if(encoder->private_->current_sample_number > blocksize) {
                ASSERT(encoder->private_->current_sample_number == blocksize+OVERREAD_);
                ASSERT(OVERREAD_ == 1); /* assert we only overread 1 sample which simplifies the rest of the code below */
                if(!process_frame_(encoder, /*is_fractional_block=*/false, /*is_last_block=*/false))
                    return false;
                /* move unprocessed overread samples to beginnings of arrays */
                for(channel = 0; channel < channels; channel++)
                    encoder->private_->integer_signal[channel][0] = encoder->private_->integer_signal[channel][blocksize];
                if(encoder->protected_->do_mid_side_stereo) {
                    encoder->private_->integer_signal_mid_side[0][0] = encoder->private_->integer_signal_mid_side[0][blocksize];
                    encoder->private_->integer_signal_mid_side[1][0] = encoder->private_->integer_signal_mid_side[1][blocksize];
                }
                encoder->private_->current_sample_number = 1;
            }
        } while(j < samples);

        return true;
        */
}

pub fn flac_stream_encoder_process_interleaved(
        encoder: *mut StreamEncoder,
        buffer:  &[i32],
        samples: u32) -> bool {
    
    todo!();
        /*
            unsigned i, j, k, channel;
        i32 x, mid, side;
        const unsigned channels = encoder->protected_->channels, blocksize = encoder->protected_->blocksize;

        ASSERT(0 != encoder);
        ASSERT(0 != encoder->private_);
        ASSERT(0 != encoder->protected_);
        ASSERT(encoder->protected_->state == STREAM_ENCODER_OK);

        j = k = 0;
        /*
         * we have several flavors of the same basic loop, optimized for
         * different conditions:
         */
        if(encoder->protected_->do_mid_side_stereo && channels == 2) {
            /*
             * stereo coding: unroll channel loop
             */
            do {
                if(encoder->protected_->verify)
                    append_to_verify_fifo_interleaved_(&encoder->private_->verify.input_fifo, buffer, j, channels, flac_min(blocksize+OVERREAD_-encoder->private_->current_sample_number, samples-j));

                /* "i <= blocksize" to overread 1 sample; see comment in OVERREAD_ decl */
                for(i = encoder->private_->current_sample_number; i <= blocksize && j < samples; i++, j++) {
                    encoder->private_->integer_signal[0][i] = mid = side = buffer[k++];
                    x = buffer[k++];
                    encoder->private_->integer_signal[1][i] = x;
                    mid += x;
                    side -= x;
                    mid >>= 1; /* NOTE: not the same as 'mid = (left + right) / 2' ! */
                    encoder->private_->integer_signal_mid_side[1][i] = side;
                    encoder->private_->integer_signal_mid_side[0][i] = mid;
                }
                encoder->private_->current_sample_number = i;
                /* we only process if we have a full block + 1 extra sample; final block is always handled by stream_encoder_finish() */
                if(i > blocksize) {
                    if(!process_frame_(encoder, /*is_fractional_block=*/false, /*is_last_block=*/false))
                        return false;
                    /* move unprocessed overread samples to beginnings of arrays */
                    ASSERT(i == blocksize+OVERREAD_);
                    ASSERT(OVERREAD_ == 1); /* assert we only overread 1 sample which simplifies the rest of the code below */
                    encoder->private_->integer_signal[0][0] = encoder->private_->integer_signal[0][blocksize];
                    encoder->private_->integer_signal[1][0] = encoder->private_->integer_signal[1][blocksize];
                    encoder->private_->integer_signal_mid_side[0][0] = encoder->private_->integer_signal_mid_side[0][blocksize];
                    encoder->private_->integer_signal_mid_side[1][0] = encoder->private_->integer_signal_mid_side[1][blocksize];
                    encoder->private_->current_sample_number = 1;
                }
            } while(j < samples);
        }
        else {
            /*
             * independent channel coding: buffer each channel in inner loop
             */
            do {
                if(encoder->protected_->verify)
                    append_to_verify_fifo_interleaved_(&encoder->private_->verify.input_fifo, buffer, j, channels, flac_min(blocksize+OVERREAD_-encoder->private_->current_sample_number, samples-j));

                /* "i <= blocksize" to overread 1 sample; see comment in OVERREAD_ decl */
                for(i = encoder->private_->current_sample_number; i <= blocksize && j < samples; i++, j++) {
                    for(channel = 0; channel < channels; channel++)
                        encoder->private_->integer_signal[channel][i] = buffer[k++];
                }
                encoder->private_->current_sample_number = i;
                /* we only process if we have a full block + 1 extra sample; final block is always handled by stream_encoder_finish() */
                if(i > blocksize) {
                    if(!process_frame_(encoder, /*is_fractional_block=*/false, /*is_last_block=*/false))
                        return false;
                    /* move unprocessed overread samples to beginnings of arrays */
                    ASSERT(i == blocksize+OVERREAD_);
                    ASSERT(OVERREAD_ == 1); /* assert we only overread 1 sample which simplifies the rest of the code below */
                    for(channel = 0; channel < channels; channel++)
                        encoder->private_->integer_signal[channel][0] = encoder->private_->integer_signal[channel][blocksize];
                    encoder->private_->current_sample_number = 1;
                }
            } while(j < samples);
        }

        return true;
        */
}

pub fn set_defaults(encoder: *mut StreamEncoder)  {
    
    todo!();
        /*
            ASSERT(0 != encoder);

    #ifdef MANDATORY_VERIFY_WHILE_ENCODING
        encoder->protected_->verify = true;
    #else
        encoder->protected_->verify = false;
    #endif
        encoder->protected_->streamable_subset = true;
        encoder->protected_->do_md5 = true;
        encoder->protected_->do_mid_side_stereo = false;
        encoder->protected_->loose_mid_side_stereo = false;
        encoder->protected_->channels = 2;
        encoder->protected_->bits_per_sample = 16;
        encoder->protected_->sample_rate = 44100;
        encoder->protected_->blocksize = 0;
    #ifndef INTEGER_ONLY_LIBRARY
        encoder->protected_->num_apodizations = 1;
        encoder->protected_->apodizations[0].type = APODIZATION_TUKEY;
        encoder->protected_->apodizations[0].parameters.tukey.p = 0.5;
    #endif
        encoder->protected_->max_lpc_order = 0;
        encoder->protected_->qlp_coeff_precision = 0;
        encoder->protected_->do_qlp_coeff_prec_search = false;
        encoder->protected_->do_exhaustive_model_search = false;
        encoder->protected_->do_escape_coding = false;
        encoder->protected_->min_residual_partition_order = 0;
        encoder->protected_->max_residual_partition_order = 0;
        encoder->protected_->rice_parameter_search_dist = 0;
        encoder->protected_->total_samples_estimate = 0;
        encoder->protected_->metadata = 0;
        encoder->protected_->num_metadata_blocks = 0;

        encoder->private_->seek_table = 0;
        encoder->private_->disable_constant_subframes = false;
        encoder->private_->disable_fixed_subframes = false;
        encoder->private_->disable_verbatim_subframes = false;
    #if HAS_OGG
        encoder->private_->is_ogg = false;
    #endif
        encoder->private_->read_callback = 0;
        encoder->private_->write_callback = 0;
        encoder->private_->seek_callback = 0;
        encoder->private_->tell_callback = 0;
        encoder->private_->metadata_callback = 0;
        encoder->private_->progress_callback = 0;
        encoder->private_->client_data = 0;

    #if HAS_OGG
        ogg_encoder_aspect_set_defaults(&encoder->protected_->ogg_encoder_aspect);
    #endif

        stream_encoder_set_compression_level(encoder, 5);
        */
}

pub fn free(encoder: *mut StreamEncoder)  {
    
    todo!();
        /*
            unsigned i, channel;

        ASSERT(0 != encoder);
        if(encoder->protected_->metadata) {
            free(encoder->protected_->metadata);
            encoder->protected_->metadata = 0;
            encoder->protected_->num_metadata_blocks = 0;
        }
        for(i = 0; i < encoder->protected_->channels; i++) {
            if(0 != encoder->private_->integer_signal_unaligned[i]) {
                free(encoder->private_->integer_signal_unaligned[i]);
                encoder->private_->integer_signal_unaligned[i] = 0;
            }
    #ifndef INTEGER_ONLY_LIBRARY
            if(0 != encoder->private_->real_signal_unaligned[i]) {
                free(encoder->private_->real_signal_unaligned[i]);
                encoder->private_->real_signal_unaligned[i] = 0;
            }
    #endif
        }
        for(i = 0; i < 2; i++) {
            if(0 != encoder->private_->integer_signal_mid_side_unaligned[i]) {
                free(encoder->private_->integer_signal_mid_side_unaligned[i]);
                encoder->private_->integer_signal_mid_side_unaligned[i] = 0;
            }
    #ifndef INTEGER_ONLY_LIBRARY
            if(0 != encoder->private_->real_signal_mid_side_unaligned[i]) {
                free(encoder->private_->real_signal_mid_side_unaligned[i]);
                encoder->private_->real_signal_mid_side_unaligned[i] = 0;
            }
    #endif
        }
    #ifndef INTEGER_ONLY_LIBRARY
        for(i = 0; i < encoder->protected_->num_apodizations; i++) {
            if(0 != encoder->private_->window_unaligned[i]) {
                free(encoder->private_->window_unaligned[i]);
                encoder->private_->window_unaligned[i] = 0;
            }
        }
        if(0 != encoder->private_->windowed_signal_unaligned) {
            free(encoder->private_->windowed_signal_unaligned);
            encoder->private_->windowed_signal_unaligned = 0;
        }
    #endif
        for(channel = 0; channel < encoder->protected_->channels; channel++) {
            for(i = 0; i < 2; i++) {
                if(0 != encoder->private_->residual_workspace_unaligned[channel][i]) {
                    free(encoder->private_->residual_workspace_unaligned[channel][i]);
                    encoder->private_->residual_workspace_unaligned[channel][i] = 0;
                }
            }
        }
        for(channel = 0; channel < 2; channel++) {
            for(i = 0; i < 2; i++) {
                if(0 != encoder->private_->residual_workspace_mid_side_unaligned[channel][i]) {
                    free(encoder->private_->residual_workspace_mid_side_unaligned[channel][i]);
                    encoder->private_->residual_workspace_mid_side_unaligned[channel][i] = 0;
                }
            }
        }
        if(0 != encoder->private_->abs_residual_partition_sums_unaligned) {
            free(encoder->private_->abs_residual_partition_sums_unaligned);
            encoder->private_->abs_residual_partition_sums_unaligned = 0;
        }
        if(0 != encoder->private_->raw_bits_per_partition_unaligned) {
            free(encoder->private_->raw_bits_per_partition_unaligned);
            encoder->private_->raw_bits_per_partition_unaligned = 0;
        }
        if(encoder->protected_->verify) {
            for(i = 0; i < encoder->protected_->channels; i++) {
                if(0 != encoder->private_->verify.input_fifo.data[i]) {
                    free(encoder->private_->verify.input_fifo.data[i]);
                    encoder->private_->verify.input_fifo.data[i] = 0;
                }
            }
        }
        bitwriter_free(encoder->private_->frame);
        */
}

pub fn resize_buffers(
        encoder:       *mut StreamEncoder,
        new_blocksize: u32) -> bool {
    
    todo!();
        /*
            bool ok;
        unsigned i, channel;

        ASSERT(new_blocksize > 0);
        ASSERT(encoder->protected_->state == STREAM_ENCODER_OK);
        ASSERT(encoder->private_->current_sample_number == 0);

        /* To avoid excessive malloc'ing, we only grow the buffer; no shrinking. */
        if(new_blocksize <= encoder->private_->input_capacity)
            return true;

        ok = true;

        /* WATCHOUT: lpc_compute_residual_from_qlp_coefficients_asm_ia32_mmx() and ..._intrin_sse2()
         * require that the input arrays (in our case the integer signals)
         * have a buffer of up to 3 zeroes in front (at negative indices) for
         * alignment purposes; we use 4 in front to keep the data well-aligned.
         */

        for(i = 0; ok && i < encoder->protected_->channels; i++) {
            ok = ok && memory_alloc_aligned_int32_array(new_blocksize+4+OVERREAD_, &encoder->private_->integer_signal_unaligned[i], &encoder->private_->integer_signal[i]);
            memset(encoder->private_->integer_signal[i], 0, sizeof(i32)*4);
            encoder->private_->integer_signal[i] += 4;
    #ifndef INTEGER_ONLY_LIBRARY
    #if 0 /* @@@ currently unused */
            if(encoder->protected_->max_lpc_order > 0)
                ok = ok && memory_alloc_aligned_real_array(new_blocksize+OVERREAD_, &encoder->private_->real_signal_unaligned[i], &encoder->private_->real_signal[i]);
    #endif
    #endif
        }
        for(i = 0; ok && i < 2; i++) {
            ok = ok && memory_alloc_aligned_int32_array(new_blocksize+4+OVERREAD_, &encoder->private_->integer_signal_mid_side_unaligned[i], &encoder->private_->integer_signal_mid_side[i]);
            memset(encoder->private_->integer_signal_mid_side[i], 0, sizeof(i32)*4);
            encoder->private_->integer_signal_mid_side[i] += 4;
    #ifndef INTEGER_ONLY_LIBRARY
    #if 0 /* @@@ currently unused */
            if(encoder->protected_->max_lpc_order > 0)
                ok = ok && memory_alloc_aligned_real_array(new_blocksize+OVERREAD_, &encoder->private_->real_signal_mid_side_unaligned[i], &encoder->private_->real_signal_mid_side[i]);
    #endif
    #endif
        }
    #ifndef INTEGER_ONLY_LIBRARY
        if(ok && encoder->protected_->max_lpc_order > 0) {
            for(i = 0; ok && i < encoder->protected_->num_apodizations; i++)
                ok = ok && memory_alloc_aligned_real_array(new_blocksize, &encoder->private_->window_unaligned[i], &encoder->private_->window[i]);
            ok = ok && memory_alloc_aligned_real_array(new_blocksize, &encoder->private_->windowed_signal_unaligned, &encoder->private_->windowed_signal);
        }
    #endif
        for(channel = 0; ok && channel < encoder->protected_->channels; channel++) {
            for(i = 0; ok && i < 2; i++) {
                ok = ok && memory_alloc_aligned_int32_array(new_blocksize, &encoder->private_->residual_workspace_unaligned[channel][i], &encoder->private_->residual_workspace[channel][i]);
            }
        }
        for(channel = 0; ok && channel < 2; channel++) {
            for(i = 0; ok && i < 2; i++) {
                ok = ok && memory_alloc_aligned_int32_array(new_blocksize, &encoder->private_->residual_workspace_mid_side_unaligned[channel][i], &encoder->private_->residual_workspace_mid_side[channel][i]);
            }
        }
        /* the *2 is an approximation to the series 1 + 1/2 + 1/4 + ... that sums tree occupies in a flat array */
        /*@@@ new_blocksize*2 is too pessimistic, but to fix, we need smarter logic because a smaller new_blocksize can actually increase the # of partitions; would require moving this out into a separate function, then checking its capacity against the need of the current blocksize&min/max_partition_order (and maybe predictor order) */
        ok = ok && memory_alloc_aligned_uint64_array(new_blocksize * 2, &encoder->private_->abs_residual_partition_sums_unaligned, &encoder->private_->abs_residual_partition_sums);
        if(encoder->protected_->do_escape_coding)
            ok = ok && memory_alloc_aligned_unsigned_array(new_blocksize * 2, &encoder->private_->raw_bits_per_partition_unaligned, &encoder->private_->raw_bits_per_partition);

        /* now adjust the windows if the blocksize has changed */
    #ifndef INTEGER_ONLY_LIBRARY
        if(ok && new_blocksize != encoder->private_->input_capacity && encoder->protected_->max_lpc_order > 0) {
            for(i = 0; ok && i < encoder->protected_->num_apodizations; i++) {
                switch(encoder->protected_->apodizations[i].type) {
                    case APODIZATION_BARTLETT:
                        window_bartlett(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_BARTLETT_HANN:
                        window_bartlett_hann(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_BLACKMAN:
                        window_blackman(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_BLACKMAN_HARRIS_4TERM_92DB_SIDELOBE:
                        window_blackman_harris_4term_92db_sidelobe(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_CONNES:
                        window_connes(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_FLATTOP:
                        window_flattop(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_GAUSS:
                        window_gauss(encoder->private_->window[i], new_blocksize, encoder->protected_->apodizations[i].parameters.gauss.stddev);
                        break;
                    case APODIZATION_HAMMING:
                        window_hamming(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_HANN:
                        window_hann(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_KAISER_BESSEL:
                        window_kaiser_bessel(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_NUTTALL:
                        window_nuttall(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_RECTANGLE:
                        window_rectangle(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_TRIANGLE:
                        window_triangle(encoder->private_->window[i], new_blocksize);
                        break;
                    case APODIZATION_TUKEY:
                        window_tukey(encoder->private_->window[i], new_blocksize, encoder->protected_->apodizations[i].parameters.tukey.p);
                        break;
                    case APODIZATION_PARTIAL_TUKEY:
                        window_partial_tukey(encoder->private_->window[i], new_blocksize, encoder->protected_->apodizations[i].parameters.multiple_tukey.p, encoder->protected_->apodizations[i].parameters.multiple_tukey.start, encoder->protected_->apodizations[i].parameters.multiple_tukey.end);
                        break;
                    case APODIZATION_PUNCHOUT_TUKEY:
                        window_punchout_tukey(encoder->private_->window[i], new_blocksize, encoder->protected_->apodizations[i].parameters.multiple_tukey.p, encoder->protected_->apodizations[i].parameters.multiple_tukey.start, encoder->protected_->apodizations[i].parameters.multiple_tukey.end);
                        break;
                    case APODIZATION_WELCH:
                        window_welch(encoder->private_->window[i], new_blocksize);
                        break;
                    default:
                        ASSERT(0);
                        /* double protection */
                        window_hann(encoder->private_->window[i], new_blocksize);
                        break;
                }
            }
        }
    #endif

        if(ok)
            encoder->private_->input_capacity = new_blocksize;
        else
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;

        return ok;
        */
}

pub fn write_bitbuffer(
        encoder:       *mut StreamEncoder,
        samples:       u32,
        is_last_block: bool) -> bool {
    
    todo!();
        /*
            const byte *buffer;
        size_t bytes;

        ASSERT(bitwriter_is_byte_aligned(encoder->private_->frame));

        if(!bitwriter_get_buffer(encoder->private_->frame, &buffer, &bytes)) {
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }

        if(encoder->protected_->verify) {
            encoder->private_->verify.output.data = buffer;
            encoder->private_->verify.output.bytes = bytes;
            if(encoder->private_->verify.state_hint == ENCODER_IN_MAGIC) {
                encoder->private_->verify.needs_magic_hack = true;
            }
            else {
                if(!stream_decoder_process_single(encoder->private_->verify.decoder)) {
                    bitwriter_release_buffer(encoder->private_->frame);
                    bitwriter_clear(encoder->private_->frame);
                    if(encoder->protected_->state != STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA)
                        encoder->protected_->state = STREAM_ENCODER_VERIFY_DECODER_ERROR;
                    return false;
                }
            }
        }

        if(write_frame_(encoder, buffer, bytes, samples, is_last_block) != STREAM_ENCODER_WRITE_STATUS_OK) {
            bitwriter_release_buffer(encoder->private_->frame);
            bitwriter_clear(encoder->private_->frame);
            encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
            return false;
        }

        bitwriter_release_buffer(encoder->private_->frame);
        bitwriter_clear(encoder->private_->frame);

        if(samples > 0) {
            encoder->private_->streaminfo.data.stream_info.min_framesize = flac_min(bytes, (size_t) encoder->private_->streaminfo.data.stream_info.min_framesize);
            encoder->private_->streaminfo.data.stream_info.max_framesize = flac_max(bytes, (size_t) encoder->private_->streaminfo.data.stream_info.max_framesize);
        }

        return true;
        */
}

pub fn write_frame(
        encoder:       *mut StreamEncoder,
        buffer:        &[u8],
        bytes:         usize,
        samples:       u32,
        is_last_block: bool) -> StreamEncoderWriteStatus {
    
    todo!();
        /*
            StreamEncoderWriteStatus status;
        u64 output_position = 0;

    #if HAS_OGG == 0
        (void)is_last_block;
    #endif

        /* STREAM_ENCODER_TELL_STATUS_UNSUPPORTED just means we didn't get the offset; no error */
        if(encoder->private_->tell_callback && encoder->private_->tell_callback(encoder, &output_position, encoder->private_->client_data) == STREAM_ENCODER_TELL_STATUS_ERROR) {
            encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
            return STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR;
        }

        /*
         * Watch for the STREAMINFO block and first SEEKTABLE block to go by and store their offsets.
         */
        if(samples == 0) {
            MetadataType type = (MetadataType) (buffer[0] & 0x7f);
            if(type == METADATA_TYPE_STREAMINFO)
                encoder->protected_->streaminfo_offset = output_position;
            else if(type == METADATA_TYPE_SEEKTABLE && encoder->protected_->seektable_offset == 0)
                encoder->protected_->seektable_offset = output_position;
        }

        /*
         * Mark the current seek point if hit (if audio_offset == 0 that
         * means we're still writing metadata and haven't hit the first
         * frame yet)
         */
        if(0 != encoder->private_->seek_table && encoder->protected_->audio_offset > 0 && encoder->private_->seek_table->num_points > 0) {
            const unsigned blocksize = stream_encoder_get_blocksize(encoder);
            const u64 frame_first_sample = encoder->private_->samples_written;
            const u64 frame_last_sample = frame_first_sample + (u64)blocksize - 1;
            u64 test_sample;
            unsigned i;
            for(i = encoder->private_->first_seekpoint_to_check; i < encoder->private_->seek_table->num_points; i++) {
                test_sample = encoder->private_->seek_table->points[i].sample_number;
                if(test_sample > frame_last_sample) {
                    break;
                }
                else if(test_sample >= frame_first_sample) {
                    encoder->private_->seek_table->points[i].sample_number = frame_first_sample;
                    encoder->private_->seek_table->points[i].stream_offset = output_position - encoder->protected_->audio_offset;
                    encoder->private_->seek_table->points[i].frame_samples = blocksize;
                    encoder->private_->first_seekpoint_to_check++;
                    /* DO NOT: "break;" and here's why:
                     * The seektable template may contain more than one target
                     * sample for any given frame; we will keep looping, generating
                     * duplicate seekpoints for them, and we'll clean it up later,
                     * just before writing the seektable back to the metadata.
                     */
                }
                else {
                    encoder->private_->first_seekpoint_to_check++;
                }
            }
        }

    #if HAS_OGG
        if(encoder->private_->is_ogg) {
            status = ogg_encoder_aspect_write_callback_wrapper(
                &encoder->protected_->ogg_encoder_aspect,
                buffer,
                bytes,
                samples,
                encoder->private_->current_frame_number,
                is_last_block,
                (OggEncoderAspectWriteCallbackProxy)encoder->private_->write_callback,
                encoder,
                encoder->private_->client_data
            );
        }
        else
    #endif
        status = encoder->private_->write_callback(encoder, buffer, bytes, samples, encoder->private_->current_frame_number, encoder->private_->client_data);

        if(status == STREAM_ENCODER_WRITE_STATUS_OK) {
            encoder->private_->bytes_written += bytes;
            encoder->private_->samples_written += samples;
            /* we keep a high watermark on the number of frames written because
             * when the encoder goes back to write metadata, 'current_frame'
             * will drop back to 0.
             */
            encoder->private_->frames_written = flac_max(encoder->private_->frames_written, encoder->private_->current_frame_number+1);
        }
        else
            encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;

        return status;
        */
}

/**
  | Gets called when the encoding process
  | has finished so that we can update the
  | STREAMINFO and SEEKTABLE blocks.
  |
  */
pub fn update_metadata(encoder: *const StreamEncoder)  {
    
    todo!();
        /*
            byte b[STREAM_METADATA_SEEKPOINT_LENGTH];
        const StreamMetadata *metadata = &encoder->private_->streaminfo;
        const u64 samples = metadata->data.stream_info.total_samples;
        const unsigned min_framesize = metadata->data.stream_info.min_framesize;
        const unsigned max_framesize = metadata->data.stream_info.max_framesize;
        const unsigned bps = metadata->data.stream_info.bits_per_sample;
        StreamEncoderSeekStatus seek_status;

        ASSERT(metadata->type == METADATA_TYPE_STREAMINFO);

        /* All this is based on intimate knowledge of the stream header
         * layout, but a change to the header format that would break this
         * would also break all streams encoded in the previous format.
         */

        /*
         * Write MD5 signature
         */
        {
            const unsigned md5_offset =
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN +
                    STREAM_METADATA_STREAMINFO_CHANNELS_LEN +
                    STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN +
                    STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN
                ) / 8;

            if((seek_status = encoder->private_->seek_callback(encoder, encoder->protected_->streaminfo_offset + md5_offset, encoder->private_->client_data)) != STREAM_ENCODER_SEEK_STATUS_OK) {
                if(seek_status == STREAM_ENCODER_SEEK_STATUS_ERROR)
                    encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
            if(encoder->private_->write_callback(encoder, metadata->data.stream_info.md5sum, 16, 0, 0, encoder->private_->client_data) != STREAM_ENCODER_WRITE_STATUS_OK) {
                encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
        }

        /*
         * Write total samples
         */
        {
            const unsigned total_samples_byte_offset =
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN +
                    STREAM_METADATA_STREAMINFO_CHANNELS_LEN +
                    STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN
                    - 4
                ) / 8;

            b[0] = ((byte)(bps-1) << 4) | (byte)((samples >> 32) & 0x0F);
            b[1] = (byte)((samples >> 24) & 0xFF);
            b[2] = (byte)((samples >> 16) & 0xFF);
            b[3] = (byte)((samples >> 8) & 0xFF);
            b[4] = (byte)(samples & 0xFF);
            if((seek_status = encoder->private_->seek_callback(encoder, encoder->protected_->streaminfo_offset + total_samples_byte_offset, encoder->private_->client_data)) != STREAM_ENCODER_SEEK_STATUS_OK) {
                if(seek_status == STREAM_ENCODER_SEEK_STATUS_ERROR)
                    encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
            if(encoder->private_->write_callback(encoder, b, 5, 0, 0, encoder->private_->client_data) != STREAM_ENCODER_WRITE_STATUS_OK) {
                encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
        }

        /*
         * Write min/max framesize
         */
        {
            const unsigned min_framesize_offset =
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN
                ) / 8;

            b[0] = (byte)((min_framesize >> 16) & 0xFF);
            b[1] = (byte)((min_framesize >> 8) & 0xFF);
            b[2] = (byte)(min_framesize & 0xFF);
            b[3] = (byte)((max_framesize >> 16) & 0xFF);
            b[4] = (byte)((max_framesize >> 8) & 0xFF);
            b[5] = (byte)(max_framesize & 0xFF);
            if((seek_status = encoder->private_->seek_callback(encoder, encoder->protected_->streaminfo_offset + min_framesize_offset, encoder->private_->client_data)) != STREAM_ENCODER_SEEK_STATUS_OK) {
                if(seek_status == STREAM_ENCODER_SEEK_STATUS_ERROR)
                    encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
            if(encoder->private_->write_callback(encoder, b, 6, 0, 0, encoder->private_->client_data) != STREAM_ENCODER_WRITE_STATUS_OK) {
                encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }
        }

        /*
         * Write seektable
         */
        if(0 != encoder->private_->seek_table && encoder->private_->seek_table->num_points > 0 && encoder->protected_->seektable_offset > 0) {
            unsigned i;

            format_seektable_sort(encoder->private_->seek_table);

            ASSERT(format_seektable_is_legal(encoder->private_->seek_table));

            if((seek_status = encoder->private_->seek_callback(encoder, encoder->protected_->seektable_offset + STREAM_METADATA_HEADER_LENGTH, encoder->private_->client_data)) != STREAM_ENCODER_SEEK_STATUS_OK) {
                if(seek_status == STREAM_ENCODER_SEEK_STATUS_ERROR)
                    encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                return;
            }

            for(i = 0; i < encoder->private_->seek_table->num_points; i++) {
                u64 xx;
                unsigned x;
                xx = encoder->private_->seek_table->points[i].sample_number;
                b[7] = (byte)xx; xx >>= 8;
                b[6] = (byte)xx; xx >>= 8;
                b[5] = (byte)xx; xx >>= 8;
                b[4] = (byte)xx; xx >>= 8;
                b[3] = (byte)xx; xx >>= 8;
                b[2] = (byte)xx; xx >>= 8;
                b[1] = (byte)xx; xx >>= 8;
                b[0] = (byte)xx; //xx >>= 8;
                xx = encoder->private_->seek_table->points[i].stream_offset;
                b[15] = (byte)xx; xx >>= 8;
                b[14] = (byte)xx; xx >>= 8;
                b[13] = (byte)xx; xx >>= 8;
                b[12] = (byte)xx; xx >>= 8;
                b[11] = (byte)xx; xx >>= 8;
                b[10] = (byte)xx; xx >>= 8;
                b[9] = (byte)xx; xx >>= 8;
                b[8] = (byte)xx; //xx >>= 8;
                x = encoder->private_->seek_table->points[i].frame_samples;
                b[17] = (byte)x; x >>= 8;
                b[16] = (byte)x; //x >>= 8;
                if(encoder->private_->write_callback(encoder, b, 18, 0, 0, encoder->private_->client_data) != STREAM_ENCODER_WRITE_STATUS_OK) {
                    encoder->protected_->state = STREAM_ENCODER_CLIENT_ERROR;
                    return;
                }
            }
        }
        */
}

/**
  | Gets called when the encoding process
  | has finished so that we can update the
  | STREAMINFO and SEEKTABLE blocks.
  |
  */
#[cfg(HAS_OGG)]
pub fn update_ogg_metadata(encoder: *mut StreamEncoder)  {
    
    todo!();
        /*
            /* the # of bytes in the 1st packet that precede the STREAMINFO */
        static const unsigned FIRST_OGG_PACKET_STREAMINFO_PREFIX_LENGTH =
            OGG_MAPPING_PACKET_TYPE_LENGTH +
            OGG_MAPPING_MAGIC_LENGTH +
            OGG_MAPPING_VERSION_MAJOR_LENGTH +
            OGG_MAPPING_VERSION_MINOR_LENGTH +
            OGG_MAPPING_NUM_HEADERS_LENGTH +
            STREAM_SYNC_LENGTH
        ;
        byte b[flac_max(6u, STREAM_METADATA_SEEKPOINT_LENGTH)];
        const StreamMetadata *metadata = &encoder->private_->streaminfo;
        const u64 samples = metadata->data.stream_info.total_samples;
        const unsigned min_framesize = metadata->data.stream_info.min_framesize;
        const unsigned max_framesize = metadata->data.stream_info.max_framesize;
        ogg_page page;

        ASSERT(metadata->type == METADATA_TYPE_STREAMINFO);
        ASSERT(0 != encoder->private_->seek_callback);

        /* Pre-check that client supports seeking, since we don't want the
         * ogg_helper code to ever have to deal with this condition.
         */
        if(encoder->private_->seek_callback(encoder, 0, encoder->private_->client_data) == STREAM_ENCODER_SEEK_STATUS_UNSUPPORTED)
            return;

        /* All this is based on intimate knowledge of the stream header
         * layout, but a change to the header format that would break this
         * would also break all streams encoded in the previous format.
         */

        /**
         ** Write STREAMINFO stats
         **/
        simple_ogg_page__init(&page);
        if(!simple_ogg_page__get_at(encoder, encoder->protected_->streaminfo_offset, &page, encoder->private_->seek_callback, encoder->private_->read_callback, encoder->private_->client_data)) {
            simple_ogg_page__clear(&page);
            return; /* state already set */
        }

        /*
         * Write MD5 signature
         */
        {
            const unsigned md5_offset =
                FIRST_OGG_PACKET_STREAMINFO_PREFIX_LENGTH +
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN +
                    STREAM_METADATA_STREAMINFO_CHANNELS_LEN +
                    STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN +
                    STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN
                ) / 8;

            if(md5_offset + 16 > (unsigned)page.body_len) {
                encoder->protected_->state = STREAM_ENCODER_OGG_ERROR;
                simple_ogg_page__clear(&page);
                return;
            }
            memcpy(page.body + md5_offset, metadata->data.stream_info.md5sum, 16);
        }

        /*
         * Write total samples
         */
        {
            const unsigned total_samples_byte_offset =
                FIRST_OGG_PACKET_STREAMINFO_PREFIX_LENGTH +
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN +
                    STREAM_METADATA_STREAMINFO_CHANNELS_LEN +
                    STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN
                    - 4
                ) / 8;

            if(total_samples_byte_offset + 5 > (unsigned)page.body_len) {
                encoder->protected_->state = STREAM_ENCODER_OGG_ERROR;
                simple_ogg_page__clear(&page);
                return;
            }
            b[0] = (byte)page.body[total_samples_byte_offset] & 0xF0;
            b[0] |= (byte)((samples >> 32) & 0x0F);
            b[1] = (byte)((samples >> 24) & 0xFF);
            b[2] = (byte)((samples >> 16) & 0xFF);
            b[3] = (byte)((samples >> 8) & 0xFF);
            b[4] = (byte)(samples & 0xFF);
            memcpy(page.body + total_samples_byte_offset, b, 5);
        }

        /*
         * Write min/max framesize
         */
        {
            const unsigned min_framesize_offset =
                FIRST_OGG_PACKET_STREAMINFO_PREFIX_LENGTH +
                STREAM_METADATA_HEADER_LENGTH +
                (
                    STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN +
                    STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN
                ) / 8;

            if(min_framesize_offset + 6 > (unsigned)page.body_len) {
                encoder->protected_->state = STREAM_ENCODER_OGG_ERROR;
                simple_ogg_page__clear(&page);
                return;
            }
            b[0] = (byte)((min_framesize >> 16) & 0xFF);
            b[1] = (byte)((min_framesize >> 8) & 0xFF);
            b[2] = (byte)(min_framesize & 0xFF);
            b[3] = (byte)((max_framesize >> 16) & 0xFF);
            b[4] = (byte)((max_framesize >> 8) & 0xFF);
            b[5] = (byte)(max_framesize & 0xFF);
            memcpy(page.body + min_framesize_offset, b, 6);
        }
        if(!simple_ogg_page__set_at(encoder, encoder->protected_->streaminfo_offset, &page, encoder->private_->seek_callback, encoder->private_->write_callback, encoder->private_->client_data)) {
            simple_ogg_page__clear(&page);
            return; /* state already set */
        }
        simple_ogg_page__clear(&page);

        /*
         * Write seektable
         */
        if(0 != encoder->private_->seek_table && encoder->private_->seek_table->num_points > 0 && encoder->protected_->seektable_offset > 0) {
            unsigned i;
            byte *p;

            format_seektable_sort(encoder->private_->seek_table);

            ASSERT(format_seektable_is_legal(encoder->private_->seek_table));

            simple_ogg_page__init(&page);
            if(!simple_ogg_page__get_at(encoder, encoder->protected_->seektable_offset, &page, encoder->private_->seek_callback, encoder->private_->read_callback, encoder->private_->client_data)) {
                simple_ogg_page__clear(&page);
                return; /* state already set */
            }

            if((STREAM_METADATA_HEADER_LENGTH + 18*encoder->private_->seek_table->num_points) != (unsigned)page.body_len) {
                encoder->protected_->state = STREAM_ENCODER_OGG_ERROR;
                simple_ogg_page__clear(&page);
                return;
            }

            for(i = 0, p = page.body + STREAM_METADATA_HEADER_LENGTH; i < encoder->private_->seek_table->num_points; i++, p += 18) {
                u64 xx;
                unsigned x;
                xx = encoder->private_->seek_table->points[i].sample_number;
                b[7] = (byte)xx; xx >>= 8;
                b[6] = (byte)xx; xx >>= 8;
                b[5] = (byte)xx; xx >>= 8;
                b[4] = (byte)xx; xx >>= 8;
                b[3] = (byte)xx; xx >>= 8;
                b[2] = (byte)xx; xx >>= 8;
                b[1] = (byte)xx; xx >>= 8;
                b[0] = (byte)xx; xx >>= 8;
                xx = encoder->private_->seek_table->points[i].stream_offset;
                b[15] = (byte)xx; xx >>= 8;
                b[14] = (byte)xx; xx >>= 8;
                b[13] = (byte)xx; xx >>= 8;
                b[12] = (byte)xx; xx >>= 8;
                b[11] = (byte)xx; xx >>= 8;
                b[10] = (byte)xx; xx >>= 8;
                b[9] = (byte)xx; xx >>= 8;
                b[8] = (byte)xx; xx >>= 8;
                x = encoder->private_->seek_table->points[i].frame_samples;
                b[17] = (byte)x; x >>= 8;
                b[16] = (byte)x; x >>= 8;
                memcpy(p, b, 18);
            }

            if(!simple_ogg_page__set_at(encoder, encoder->protected_->seektable_offset, &page, encoder->private_->seek_callback, encoder->private_->write_callback, encoder->private_->client_data)) {
                simple_ogg_page__clear(&page);
                return; /* state already set */
            }
            simple_ogg_page__clear(&page);
        }
        */
}

pub fn process_frame(
        encoder:             *mut StreamEncoder,
        is_fractional_block: bool,
        is_last_block:       bool) -> bool {
    
    todo!();
        /*
            uint16 crc;
        ASSERT(encoder->protected_->state == STREAM_ENCODER_OK);

        /*
         * Accumulate raw signal to the MD5 signature
         */
        if(encoder->protected_->do_md5 && !MD5Accumulate(&encoder->private_->md5context, (const i32 * const *)encoder->private_->integer_signal, encoder->protected_->channels, encoder->protected_->blocksize, (encoder->protected_->bits_per_sample+7) / 8)) {
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }

        /*
         * Process the frame header and subframes into the frame bitbuffer
         */
        if(!process_subframes_(encoder, is_fractional_block)) {
            /* the above function sets the state for us in case of an error */
            return false;
        }

        /*
         * Zero-pad the frame to a byte_boundary
         */
        if(!bitwriter_zero_pad_to_byte_boundary(encoder->private_->frame)) {
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }

        /*
         * CRC-16 the whole thing
         */
        ASSERT(bitwriter_is_byte_aligned(encoder->private_->frame));
        if(
            !bitwriter_get_write_crc16(encoder->private_->frame, &crc) ||
            !bitwriter_write_raw_uint32(encoder->private_->frame, crc, FRAME_FOOTER_CRC_LEN)
        ) {
            encoder->protected_->state = STREAM_ENCODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }

        /*
         * Write it
         */
        if(!write_bitbuffer_(encoder, encoder->protected_->blocksize, is_last_block)) {
            /* the above function sets the state for us in case of an error */
            return false;
        }

        /*
         * Get ready for the next frame
         */
        encoder->private_->current_sample_number = 0;
        encoder->private_->current_frame_number++;
        encoder->private_->streaminfo.data.stream_info.total_samples += (u64)encoder->protected_->blocksize;

        return true;
        */
}

pub fn process_subframes(
        encoder:             *mut StreamEncoder,
        is_fractional_block: bool) -> bool {
    
    todo!();
        /*
            FrameHeader frame_header;
        unsigned channel, min_partition_order = encoder->protected_->min_residual_partition_order, max_partition_order;
        bool do_independent, do_mid_side;

        /*
         * Calculate the min,max Rice partition orders
         */
        if(is_fractional_block) {
            max_partition_order = 0;
        }
        else {
            max_partition_order = format_get_max_rice_partition_order_from_blocksize(encoder->protected_->blocksize);
            max_partition_order = flac_min(max_partition_order, encoder->protected_->max_residual_partition_order);
        }
        min_partition_order = flac_min(min_partition_order, max_partition_order);

        /*
         * Setup the frame
         */
        frame_header.blocksize = encoder->protected_->blocksize;
        frame_header.sample_rate = encoder->protected_->sample_rate;
        frame_header.channels = encoder->protected_->channels;
        frame_header.channel_assignment = CHANNEL_ASSIGNMENT_INDEPENDENT; /* the default unless the encoder determines otherwise */
        frame_header.bits_per_sample = encoder->protected_->bits_per_sample;
        frame_header.number_type = FRAME_NUMBER_TYPE_FRAME_NUMBER;
        frame_header.number.frame_number = encoder->private_->current_frame_number;

        /*
         * Figure out what channel assignments to try
         */
        if(encoder->protected_->do_mid_side_stereo) {
            if(encoder->protected_->loose_mid_side_stereo) {
                if(encoder->private_->loose_mid_side_stereo_frame_count == 0) {
                    do_independent = true;
                    do_mid_side = true;
                }
                else {
                    do_independent = (encoder->private_->last_channel_assignment == CHANNEL_ASSIGNMENT_INDEPENDENT);
                    do_mid_side = !do_independent;
                }
            }
            else {
                do_independent = true;
                do_mid_side = true;
            }
        }
        else {
            do_independent = true;
            do_mid_side = false;
        }

        ASSERT(do_independent || do_mid_side);

        /*
         * Check for wasted bits; set effective bps for each subframe
         */
        if(do_independent) {
            for(channel = 0; channel < encoder->protected_->channels; channel++) {
                const unsigned w = get_wasted_bits_(encoder->private_->integer_signal[channel], encoder->protected_->blocksize);
                encoder->private_->subframe_workspace[channel][0].wasted_bits = encoder->private_->subframe_workspace[channel][1].wasted_bits = w;
                encoder->private_->subframe_bps[channel] = encoder->protected_->bits_per_sample - w;
            }
        }
        if(do_mid_side) {
            ASSERT(encoder->protected_->channels == 2);
            for(channel = 0; channel < 2; channel++) {
                const unsigned w = get_wasted_bits_(encoder->private_->integer_signal_mid_side[channel], encoder->protected_->blocksize);
                encoder->private_->subframe_workspace_mid_side[channel][0].wasted_bits = encoder->private_->subframe_workspace_mid_side[channel][1].wasted_bits = w;
                encoder->private_->subframe_bps_mid_side[channel] = encoder->protected_->bits_per_sample - w + (channel==0? 0:1);
            }
        }

        /*
         * First do a normal encoding pass of each independent channel
         */
        if(do_independent) {
            for(channel = 0; channel < encoder->protected_->channels; channel++) {
                if(!
                    process_subframe_(
                        encoder,
                        min_partition_order,
                        max_partition_order,
                        &frame_header,
                        encoder->private_->subframe_bps[channel],
                        encoder->private_->integer_signal[channel],
                        encoder->private_->subframe_workspace_ptr[channel],
                        encoder->private_->partitioned_rice_contents_workspace_ptr[channel],
                        encoder->private_->residual_workspace[channel],
                        encoder->private_->best_subframe+channel,
                        encoder->private_->best_subframe_bits+channel
                    )
                )
                    return false;
            }
        }

        /*
         * Now do mid and side channels if requested
         */
        if(do_mid_side) {
            ASSERT(encoder->protected_->channels == 2);

            for(channel = 0; channel < 2; channel++) {
                if(!
                    process_subframe_(
                        encoder,
                        min_partition_order,
                        max_partition_order,
                        &frame_header,
                        encoder->private_->subframe_bps_mid_side[channel],
                        encoder->private_->integer_signal_mid_side[channel],
                        encoder->private_->subframe_workspace_ptr_mid_side[channel],
                        encoder->private_->partitioned_rice_contents_workspace_ptr_mid_side[channel],
                        encoder->private_->residual_workspace_mid_side[channel],
                        encoder->private_->best_subframe_mid_side+channel,
                        encoder->private_->best_subframe_bits_mid_side+channel
                    )
                )
                    return false;
            }
        }

        /*
         * Compose the frame bitbuffer
         */
        if(do_mid_side) {
            unsigned left_bps = 0, right_bps = 0; /* initialized only to prevent superfluous compiler warning */
            Subframe *left_subframe = 0, *right_subframe = 0; /* initialized only to prevent superfluous compiler warning */
            ChannelAssignment channel_assignment;

            ASSERT(encoder->protected_->channels == 2);

            if(encoder->protected_->loose_mid_side_stereo && encoder->private_->loose_mid_side_stereo_frame_count > 0) {
                channel_assignment = (encoder->private_->last_channel_assignment == CHANNEL_ASSIGNMENT_INDEPENDENT? CHANNEL_ASSIGNMENT_INDEPENDENT : CHANNEL_ASSIGNMENT_MID_SIDE);
            }
            else {
                unsigned bits[4]; /* WATCHOUT - indexed by ChannelAssignment */
                unsigned min_bits;
                int ca;

                ASSERT(CHANNEL_ASSIGNMENT_INDEPENDENT == 0);
                ASSERT(CHANNEL_ASSIGNMENT_LEFT_SIDE   == 1);
                ASSERT(CHANNEL_ASSIGNMENT_RIGHT_SIDE  == 2);
                ASSERT(CHANNEL_ASSIGNMENT_MID_SIDE    == 3);
                ASSERT(do_independent && do_mid_side);

                /* We have to figure out which channel assignent results in the smallest frame */
                bits[CHANNEL_ASSIGNMENT_INDEPENDENT] = encoder->private_->best_subframe_bits         [0] + encoder->private_->best_subframe_bits         [1];
                bits[CHANNEL_ASSIGNMENT_LEFT_SIDE  ] = encoder->private_->best_subframe_bits         [0] + encoder->private_->best_subframe_bits_mid_side[1];
                bits[CHANNEL_ASSIGNMENT_RIGHT_SIDE ] = encoder->private_->best_subframe_bits         [1] + encoder->private_->best_subframe_bits_mid_side[1];
                bits[CHANNEL_ASSIGNMENT_MID_SIDE   ] = encoder->private_->best_subframe_bits_mid_side[0] + encoder->private_->best_subframe_bits_mid_side[1];

                channel_assignment = CHANNEL_ASSIGNMENT_INDEPENDENT;
                min_bits = bits[channel_assignment];
                for(ca = 1; ca <= 3; ca++) {
                    if(bits[ca] < min_bits) {
                        min_bits = bits[ca];
                        channel_assignment = (ChannelAssignment)ca;
                    }
                }
            }

            frame_header.channel_assignment = channel_assignment;

            if(!frame_add_header(&frame_header, encoder->private_->frame)) {
                encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                return false;
            }

            switch(channel_assignment) {
                case CHANNEL_ASSIGNMENT_INDEPENDENT:
                    left_subframe  = &encoder->private_->subframe_workspace         [0][encoder->private_->best_subframe         [0]];
                    right_subframe = &encoder->private_->subframe_workspace         [1][encoder->private_->best_subframe         [1]];
                    break;
                case CHANNEL_ASSIGNMENT_LEFT_SIDE:
                    left_subframe  = &encoder->private_->subframe_workspace         [0][encoder->private_->best_subframe         [0]];
                    right_subframe = &encoder->private_->subframe_workspace_mid_side[1][encoder->private_->best_subframe_mid_side[1]];
                    break;
                case CHANNEL_ASSIGNMENT_RIGHT_SIDE:
                    left_subframe  = &encoder->private_->subframe_workspace_mid_side[1][encoder->private_->best_subframe_mid_side[1]];
                    right_subframe = &encoder->private_->subframe_workspace         [1][encoder->private_->best_subframe         [1]];
                    break;
                case CHANNEL_ASSIGNMENT_MID_SIDE:
                    left_subframe  = &encoder->private_->subframe_workspace_mid_side[0][encoder->private_->best_subframe_mid_side[0]];
                    right_subframe = &encoder->private_->subframe_workspace_mid_side[1][encoder->private_->best_subframe_mid_side[1]];
                    break;
                default:
                    ASSERT(0);
            }

            switch(channel_assignment) {
                case CHANNEL_ASSIGNMENT_INDEPENDENT:
                    left_bps  = encoder->private_->subframe_bps         [0];
                    right_bps = encoder->private_->subframe_bps         [1];
                    break;
                case CHANNEL_ASSIGNMENT_LEFT_SIDE:
                    left_bps  = encoder->private_->subframe_bps         [0];
                    right_bps = encoder->private_->subframe_bps_mid_side[1];
                    break;
                case CHANNEL_ASSIGNMENT_RIGHT_SIDE:
                    left_bps  = encoder->private_->subframe_bps_mid_side[1];
                    right_bps = encoder->private_->subframe_bps         [1];
                    break;
                case CHANNEL_ASSIGNMENT_MID_SIDE:
                    left_bps  = encoder->private_->subframe_bps_mid_side[0];
                    right_bps = encoder->private_->subframe_bps_mid_side[1];
                    break;
                default:
                    ASSERT(0);
            }

            /* note that encoder_add_subframe_ sets the state for us in case of an error */
            if(!add_subframe_(encoder, frame_header.blocksize, left_bps , left_subframe , encoder->private_->frame))
                return false;
            if(!add_subframe_(encoder, frame_header.blocksize, right_bps, right_subframe, encoder->private_->frame))
                return false;
        }
        else {
            if(!frame_add_header(&frame_header, encoder->private_->frame)) {
                encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                return false;
            }

            for(channel = 0; channel < encoder->protected_->channels; channel++) {
                if(!add_subframe_(encoder, frame_header.blocksize, encoder->private_->subframe_bps[channel], &encoder->private_->subframe_workspace[channel][encoder->private_->best_subframe[channel]], encoder->private_->frame)) {
                    /* the above function sets the state for us in case of an error */
                    return false;
                }
            }
        }

        if(encoder->protected_->loose_mid_side_stereo) {
            encoder->private_->loose_mid_side_stereo_frame_count++;
            if(encoder->private_->loose_mid_side_stereo_frame_count >= encoder->private_->loose_mid_side_stereo_frames)
                encoder->private_->loose_mid_side_stereo_frame_count = 0;
        }

        encoder->private_->last_channel_assignment = frame_header.channel_assignment;

        return true;
        */
}

pub fn process_subframe(
        encoder:                   *mut StreamEncoder,
        min_partition_order:       u32,
        max_partition_order:       u32,
        frame_header:              *const FrameHeader,
        subframe_bps:              u32,
        integer_signal:            &[i32],
        subframe:                  *mut [Subframe; 2],
        partitioned_rice_contents: *mut [EntropyCodingMethod_PartitionedRiceContents; 2],
        residual:                  *mut [i32; 2],
        best_subframe:             *mut u32,
        best_bits:                 *mut u32) -> bool {
    
    todo!();
        /*
            #ifndef INTEGER_ONLY_LIBRARY
        float fixed_residual_bits_per_sample[MAX_FIXED_ORDER+1];
    #else
        fixedpoint fixed_residual_bits_per_sample[MAX_FIXED_ORDER+1];
    #endif
    #ifndef INTEGER_ONLY_LIBRARY
        double lpc_residual_bits_per_sample;
        real autoc[MAX_LPC_ORDER+1]; /* WATCHOUT: the size is important even though encoder->protected_->max_lpc_order might be less; some asm and x86 intrinsic routines need all the space */
        double lpc_error[MAX_LPC_ORDER];
        unsigned min_lpc_order, max_lpc_order, lpc_order;
        unsigned min_qlp_coeff_precision, max_qlp_coeff_precision, qlp_coeff_precision;
    #endif
        unsigned min_fixed_order, max_fixed_order, guess_fixed_order, fixed_order;
        unsigned rice_parameter;
        unsigned _candidate_bits, _best_bits;
        unsigned _best_subframe;
        /* only use RICE2 partitions if stream bps > 16 */
        const unsigned rice_parameter_limit = stream_encoder_get_bits_per_sample(encoder) > 16? ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER : ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER;

        ASSERT(frame_header->blocksize > 0);

        /* verbatim subframe is the baseline against which we measure other compressed subframes */
        _best_subframe = 0;
        if(encoder->private_->disable_verbatim_subframes && frame_header->blocksize >= MAX_FIXED_ORDER)
            _best_bits = UINT_MAX;
        else
            _best_bits = evaluate_verbatim_subframe_(encoder, integer_signal, frame_header->blocksize, subframe_bps, subframe[_best_subframe]);

        if(frame_header->blocksize >= MAX_FIXED_ORDER) {
            unsigned signal_is_constant = false;
            guess_fixed_order = encoder->private_->local_fixed_compute_best_predictor(integer_signal+MAX_FIXED_ORDER, frame_header->blocksize-MAX_FIXED_ORDER, fixed_residual_bits_per_sample);
            /* check for constant subframe */
            if(
                !encoder->private_->disable_constant_subframes &&
    #ifndef INTEGER_ONLY_LIBRARY
                fixed_residual_bits_per_sample[1] == 0.0
    #else
                fixed_residual_bits_per_sample[1] == FP_ZERO
    #endif
            ) {
                /* the above means it's possible all samples are the same value; now double-check it: */
                unsigned i;
                signal_is_constant = true;
                for(i = 1; i < frame_header->blocksize; i++) {
                    if(integer_signal[0] != integer_signal[i]) {
                        signal_is_constant = false;
                        break;
                    }
                }
            }
            if(signal_is_constant) {
                _candidate_bits = evaluate_constant_subframe_(encoder, integer_signal[0], frame_header->blocksize, subframe_bps, subframe[!_best_subframe]);
                if(_candidate_bits < _best_bits) {
                    _best_subframe = !_best_subframe;
                    _best_bits = _candidate_bits;
                }
            }
            else {
                if(!encoder->private_->disable_fixed_subframes || (encoder->protected_->max_lpc_order == 0 && _best_bits == UINT_MAX)) {
                    /* encode fixed */
                    if(encoder->protected_->do_exhaustive_model_search) {
                        min_fixed_order = 0;
                        max_fixed_order = MAX_FIXED_ORDER;
                    }
                    else {
                        min_fixed_order = max_fixed_order = guess_fixed_order;
                    }
                    if(max_fixed_order >= frame_header->blocksize)
                        max_fixed_order = frame_header->blocksize - 1;
                    for(fixed_order = min_fixed_order; fixed_order <= max_fixed_order; fixed_order++) {
    #ifndef INTEGER_ONLY_LIBRARY
                        if(fixed_residual_bits_per_sample[fixed_order] >= (float)subframe_bps)
                            continue; /* don't even try */
                        rice_parameter = (fixed_residual_bits_per_sample[fixed_order] > 0.0)? (unsigned)(fixed_residual_bits_per_sample[fixed_order]+0.5) : 0; /* 0.5 is for rounding */
    #else
                        if(fixedpoint_trunc(fixed_residual_bits_per_sample[fixed_order]) >= (int)subframe_bps)
                            continue; /* don't even try */
                        rice_parameter = (fixed_residual_bits_per_sample[fixed_order] > FP_ZERO)? (unsigned)fixedpoint_trunc(fixed_residual_bits_per_sample[fixed_order]+FP_ONE_HALF) : 0; /* 0.5 is for rounding */
    #endif
                        rice_parameter++; /* to account for the signed->unsigned conversion during rice coding */
                        if(rice_parameter >= rice_parameter_limit) {
    #ifdef DEBUG_VERBOSE
                            fprintf(stderr, "clipping rice_parameter (%u -> %u) @0\n", rice_parameter, rice_parameter_limit - 1);
    #endif
                            rice_parameter = rice_parameter_limit - 1;
                        }
                        _candidate_bits =
                            evaluate_fixed_subframe_(
                                encoder,
                                integer_signal,
                                residual[!_best_subframe],
                                encoder->private_->abs_residual_partition_sums,
                                encoder->private_->raw_bits_per_partition,
                                frame_header->blocksize,
                                subframe_bps,
                                fixed_order,
                                rice_parameter,
                                rice_parameter_limit,
                                min_partition_order,
                                max_partition_order,
                                encoder->protected_->do_escape_coding,
                                encoder->protected_->rice_parameter_search_dist,
                                subframe[!_best_subframe],
                                partitioned_rice_contents[!_best_subframe]
                            );
                        if(_candidate_bits < _best_bits) {
                            _best_subframe = !_best_subframe;
                            _best_bits = _candidate_bits;
                        }
                    }
                }

    #ifndef INTEGER_ONLY_LIBRARY
                /* encode lpc */
                if(encoder->protected_->max_lpc_order > 0) {
                    if(encoder->protected_->max_lpc_order >= frame_header->blocksize)
                        max_lpc_order = frame_header->blocksize-1;
                    else
                        max_lpc_order = encoder->protected_->max_lpc_order;
                    if(max_lpc_order > 0) {
                        unsigned a;
                        for (a = 0; a < encoder->protected_->num_apodizations; a++) {
                            lpc_window_data(integer_signal, encoder->private_->window[a], encoder->private_->windowed_signal, frame_header->blocksize);
                            encoder->private_->local_lpc_compute_autocorrelation(encoder->private_->windowed_signal, frame_header->blocksize, max_lpc_order+1, autoc);
                            /* if autoc[0] == 0.0, the signal is constant and we usually won't get here, but it can happen */
                            if(autoc[0] != 0.0) {
                                lpc_compute_lp_coefficients(autoc, &max_lpc_order, encoder->private_->lp_coeff, lpc_error);
                                if(encoder->protected_->do_exhaustive_model_search) {
                                    min_lpc_order = 1;
                                }
                                else {
                                    const unsigned guess_lpc_order =
                                        lpc_compute_best_order(
                                            lpc_error,
                                            max_lpc_order,
                                            frame_header->blocksize,
                                            subframe_bps + (
                                                encoder->protected_->do_qlp_coeff_prec_search?
                                                    MIN_QLP_COEFF_PRECISION : /* have to guess; use the min possible size to avoid accidentally favoring lower orders */
                                                    encoder->protected_->qlp_coeff_precision
                                            )
                                        );
                                    min_lpc_order = max_lpc_order = guess_lpc_order;
                                }
                                if(max_lpc_order >= frame_header->blocksize)
                                    max_lpc_order = frame_header->blocksize - 1;
                                for(lpc_order = min_lpc_order; lpc_order <= max_lpc_order; lpc_order++) {
                                    lpc_residual_bits_per_sample = lpc_compute_expected_bits_per_residual_sample(lpc_error[lpc_order-1], frame_header->blocksize-lpc_order);
                                    if(lpc_residual_bits_per_sample >= (double)subframe_bps)
                                        continue; /* don't even try */
                                    rice_parameter = (lpc_residual_bits_per_sample > 0.0)? (unsigned)(lpc_residual_bits_per_sample+0.5) : 0; /* 0.5 is for rounding */
                                    rice_parameter++; /* to account for the signed->unsigned conversion during rice coding */
                                    if(rice_parameter >= rice_parameter_limit) {
    #ifdef DEBUG_VERBOSE
                                        fprintf(stderr, "clipping rice_parameter (%u -> %u) @1\n", rice_parameter, rice_parameter_limit - 1);
    #endif
                                        rice_parameter = rice_parameter_limit - 1;
                                    }
                                    if(encoder->protected_->do_qlp_coeff_prec_search) {
                                        min_qlp_coeff_precision = MIN_QLP_COEFF_PRECISION;
                                        /* try to keep qlp coeff precision such that only 32-bit math is required for decode of <=16bps streams */
                                        if(subframe_bps <= 16) {
                                            max_qlp_coeff_precision = flac_min(32 - subframe_bps - bitmath_ilog2(lpc_order), MAX_QLP_COEFF_PRECISION);
                                            max_qlp_coeff_precision = flac_max(max_qlp_coeff_precision, min_qlp_coeff_precision);
                                        }
                                        else
                                            max_qlp_coeff_precision = MAX_QLP_COEFF_PRECISION;
                                    }
                                    else {
                                        min_qlp_coeff_precision = max_qlp_coeff_precision = encoder->protected_->qlp_coeff_precision;
                                    }
                                    for(qlp_coeff_precision = min_qlp_coeff_precision; qlp_coeff_precision <= max_qlp_coeff_precision; qlp_coeff_precision++) {
                                        _candidate_bits =
                                            evaluate_lpc_subframe_(
                                                encoder,
                                                integer_signal,
                                                residual[!_best_subframe],
                                                encoder->private_->abs_residual_partition_sums,
                                                encoder->private_->raw_bits_per_partition,
                                                encoder->private_->lp_coeff[lpc_order-1],
                                                frame_header->blocksize,
                                                subframe_bps,
                                                lpc_order,
                                                qlp_coeff_precision,
                                                rice_parameter,
                                                rice_parameter_limit,
                                                min_partition_order,
                                                max_partition_order,
                                                encoder->protected_->do_escape_coding,
                                                encoder->protected_->rice_parameter_search_dist,
                                                subframe[!_best_subframe],
                                                partitioned_rice_contents[!_best_subframe]
                                            );
                                        if(_candidate_bits > 0) { /* if == 0, there was a problem quantizing the lpcoeffs */
                                            if(_candidate_bits < _best_bits) {
                                                _best_subframe = !_best_subframe;
                                                _best_bits = _candidate_bits;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
    #endif /* !defined INTEGER_ONLY_LIBRARY */
            }
        }

        /* under rare circumstances this can happen when all but lpc subframe types are disabled: */
        if(_best_bits == UINT_MAX) {
            ASSERT(_best_subframe == 0);
            _best_bits = evaluate_verbatim_subframe_(encoder, integer_signal, frame_header->blocksize, subframe_bps, subframe[_best_subframe]);
        }

        *best_subframe = _best_subframe;
        *best_bits = _best_bits;

        return true;
        */
}

pub fn add_subframe(
        encoder:      *mut StreamEncoder,
        blocksize:    u32,
        subframe_bps: u32,
        subframe:     *const Subframe,
        frame:        *mut BitWriter) -> bool {
    
    todo!();
        /*
            switch(subframe->type) {
            case SUBFRAME_TYPE_CONSTANT:
                if(!subframe_add_constant(&(subframe->data.constant), subframe_bps, subframe->wasted_bits, frame)) {
                    encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                    return false;
                }
                break;
            case SUBFRAME_TYPE_FIXED:
                if(!subframe_add_fixed(&(subframe->data.fixed), blocksize - subframe->data.fixed.order, subframe_bps, subframe->wasted_bits, frame)) {
                    encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                    return false;
                }
                break;
            case SUBFRAME_TYPE_LPC:
                if(!subframe_add_lpc(&(subframe->data.lpc), blocksize - subframe->data.lpc.order, subframe_bps, subframe->wasted_bits, frame)) {
                    encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                    return false;
                }
                break;
            case SUBFRAME_TYPE_VERBATIM:
                if(!subframe_add_verbatim(&(subframe->data.verbatim), blocksize, subframe_bps, subframe->wasted_bits, frame)) {
                    encoder->protected_->state = STREAM_ENCODER_FRAMING_ERROR;
                    return false;
                }
                break;
            default:
                ASSERT(0);
        }

        return true;
        */
}

pub const SPOTCHECK_ESTIMATE: usize = 0;

#[cfg(SPOTCHECK_ESTIMATE)]
pub fn spotcheck_subframe_estimate(
        encoder:      *mut StreamEncoder,
        blocksize:    u32,
        subframe_bps: u32,
        subframe:     *const Subframe,
        estimate:     u32)  {
    
    todo!();
        /*
            bool ret;
        BitWriter *frame = bitwriter_new();
        if(frame == 0) {
            fprintf(stderr, "EST: can't allocate frame\n");
            return;
        }
        if(!bitwriter_init(frame)) {
            fprintf(stderr, "EST: can't init frame\n");
            return;
        }
        ret = add_subframe_(encoder, blocksize, subframe_bps, subframe, frame);
        ASSERT(ret);
        {
            const unsigned actual = bitwriter_get_input_bits_unconsumed(frame);
            if(estimate != actual)
                fprintf(stderr, "EST: bad, frame#%u sub#%%d type=%8s est=%u, actual=%u, delta=%d\n", encoder->private_->current_frame_number, SubframeTypeString[subframe->type], estimate, actual, (int)actual-(int)estimate);
        }
        bitwriter_delete(frame);
        */
}

pub fn evaluate_constant_subframe(
        encoder:      *mut StreamEncoder,
        signal:       i32,
        blocksize:    u32,
        subframe_bps: u32,
        subframe:     *mut Subframe) -> u32 {
    
    todo!();
        /*
            unsigned estimate;
        subframe->type = SUBFRAME_TYPE_CONSTANT;
        subframe->data.constant.value = signal;

        estimate = SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN + subframe->wasted_bits + subframe_bps;

    #if SPOTCHECK_ESTIMATE
        spotcheck_subframe_estimate_(encoder, blocksize, subframe_bps, subframe, estimate);
    #else
        (void)encoder, (void)blocksize;
    #endif

        return estimate;
        */
}

pub fn evaluate_fixed_subframe(
        encoder:                     *mut StreamEncoder,
        signal:                      &[i32],
        residual:                    &[i32],
        abs_residual_partition_sums: &[u64],
        raw_bits_per_partition:      &[u32],
        blocksize:                   u32,
        subframe_bps:                u32,
        order:                       u32,
        rice_parameter:              u32,
        rice_parameter_limit:        u32,
        min_partition_order:         u32,
        max_partition_order:         u32,
        do_escape_coding:            bool,
        rice_parameter_search_dist:  u32,
        subframe:                    *mut Subframe,
        partitioned_rice_contents:   *mut EntropyCodingMethod_PartitionedRiceContents) -> u32 {
    
    todo!();
        /*
            unsigned i, residual_bits, estimate;
        const unsigned residual_samples = blocksize - order;

        fixed_compute_residual(signal+order, residual_samples, order, residual);

        subframe->type = SUBFRAME_TYPE_FIXED;

        subframe->data.fixed.entropy_coding_method.type = ENTROPY_CODING_METHOD_PARTITIONED_RICE;
        subframe->data.fixed.entropy_coding_method.data.partitioned_rice.contents = partitioned_rice_contents;
        subframe->data.fixed.residual = residual;

        residual_bits =
            find_best_partition_order_(
                encoder->private_,
                residual,
                abs_residual_partition_sums,
                raw_bits_per_partition,
                residual_samples,
                order,
                rice_parameter,
                rice_parameter_limit,
                min_partition_order,
                max_partition_order,
                subframe_bps,
                do_escape_coding,
                rice_parameter_search_dist,
                &subframe->data.fixed.entropy_coding_method
            );

        subframe->data.fixed.order = order;
        for(i = 0; i < order; i++)
            subframe->data.fixed.warmup[i] = signal[i];

        estimate = SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN + subframe->wasted_bits + (order * subframe_bps) + residual_bits;

    #if SPOTCHECK_ESTIMATE
        spotcheck_subframe_estimate_(encoder, blocksize, subframe_bps, subframe, estimate);
    #endif

        return estimate;
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn evaluate_lpc_subframe(
        encoder:                     *mut StreamEncoder,
        signal:                      &[i32],
        residual:                    &[i32],
        abs_residual_partition_sums: &[u64],
        raw_bits_per_partition:      &[u32],
        lp_coeff:                    &[real],
        blocksize:                   u32,
        subframe_bps:                u32,
        order:                       u32,
        qlp_coeff_precision:         u32,
        rice_parameter:              u32,
        rice_parameter_limit:        u32,
        min_partition_order:         u32,
        max_partition_order:         u32,
        do_escape_coding:            bool,
        rice_parameter_search_dist:  u32,
        subframe:                    *mut Subframe,
        partitioned_rice_contents:   *mut EntropyCodingMethod_PartitionedRiceContents) -> u32 {
    
    todo!();
        /*
            i32 qlp_coeff[MAX_LPC_ORDER]; /* WATCHOUT: the size is important; some x86 intrinsic routines need more than lpc order elements */
        unsigned i, residual_bits, estimate;
        int quantization, ret;
        const unsigned residual_samples = blocksize - order;

        /* try to keep qlp coeff precision such that only 32-bit math is required for decode of <=16bps streams */
        if(subframe_bps <= 16) {
            ASSERT(order > 0);
            ASSERT(order <= MAX_LPC_ORDER);
            qlp_coeff_precision = flac_min(qlp_coeff_precision, 32 - subframe_bps - bitmath_ilog2(order));
        }

        ret = lpc_quantize_coefficients(lp_coeff, order, qlp_coeff_precision, qlp_coeff, &quantization);
        if(ret != 0)
            return 0; /* this is a hack to indicate to the caller that we can't do lp at this order on this subframe */

        if(subframe_bps + qlp_coeff_precision + bitmath_ilog2(order) <= 32)
            if(subframe_bps <= 16 && qlp_coeff_precision <= 16)
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_16bit(signal+order, residual_samples, qlp_coeff, order, quantization, residual);
            else
                encoder->private_->local_lpc_compute_residual_from_qlp_coefficients(signal+order, residual_samples, qlp_coeff, order, quantization, residual);
        else
            encoder->private_->local_lpc_compute_residual_from_qlp_coefficients_64bit(signal+order, residual_samples, qlp_coeff, order, quantization, residual);

        subframe->type = SUBFRAME_TYPE_LPC;

        subframe->data.lpc.entropy_coding_method.type = ENTROPY_CODING_METHOD_PARTITIONED_RICE;
        subframe->data.lpc.entropy_coding_method.data.partitioned_rice.contents = partitioned_rice_contents;
        subframe->data.lpc.residual = residual;

        residual_bits =
            find_best_partition_order_(
                encoder->private_,
                residual,
                abs_residual_partition_sums,
                raw_bits_per_partition,
                residual_samples,
                order,
                rice_parameter,
                rice_parameter_limit,
                min_partition_order,
                max_partition_order,
                subframe_bps,
                do_escape_coding,
                rice_parameter_search_dist,
                &subframe->data.lpc.entropy_coding_method
            );

        subframe->data.lpc.order = order;
        subframe->data.lpc.qlp_coeff_precision = qlp_coeff_precision;
        subframe->data.lpc.quantization_level = quantization;
        memcpy(subframe->data.lpc.qlp_coeff, qlp_coeff, sizeof(i32)*MAX_LPC_ORDER);
        for(i = 0; i < order; i++)
            subframe->data.lpc.warmup[i] = signal[i];

        estimate = SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN + subframe->wasted_bits + SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN + SUBFRAME_LPC_QLP_SHIFT_LEN + (order * (qlp_coeff_precision + subframe_bps)) + residual_bits;

    #if SPOTCHECK_ESTIMATE
        spotcheck_subframe_estimate_(encoder, blocksize, subframe_bps, subframe, estimate);
    #endif

        return estimate;
        */
}

pub fn evaluate_verbatim_subframe(
        encoder:      *mut StreamEncoder,
        signal:       &[i32],
        blocksize:    u32,
        subframe_bps: u32,
        subframe:     *mut Subframe) -> u32 {
    
    todo!();
        /*
            unsigned estimate;

        subframe->type = SUBFRAME_TYPE_VERBATIM;

        subframe->data.verbatim.data = signal;

        estimate = SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN + subframe->wasted_bits + (blocksize * subframe_bps);

    #if SPOTCHECK_ESTIMATE
        spotcheck_subframe_estimate_(encoder, blocksize, subframe_bps, subframe, estimate);
    #else
        (void)encoder;
    #endif

        return estimate;
        */
}

pub fn find_best_partition_order(
        private:                     *mut StreamEncoderPrivate,
        residual:                    &[i32],
        abs_residual_partition_sums: &[u64],
        raw_bits_per_partition:      &[u32],
        residual_samples:            u32,
        predictor_order:             u32,
        rice_parameter:              u32,
        rice_parameter_limit:        u32,
        min_partition_order:         u32,
        max_partition_order:         u32,
        bps:                         u32,
        do_escape_coding:            bool,
        rice_parameter_search_dist:  u32,
        best_ecm:                    *mut EntropyCodingMethod) -> u32 {
    
    todo!();
        /*
            unsigned residual_bits, best_residual_bits = 0;
        unsigned best_parameters_index = 0;
        unsigned best_partition_order = 0;
        const unsigned blocksize = residual_samples + predictor_order;

        max_partition_order = format_get_max_rice_partition_order_from_blocksize_limited_max_and_predictor_order(max_partition_order, blocksize, predictor_order);
        min_partition_order = flac_min(min_partition_order, max_partition_order);

        private_->local_precompute_partition_info_sums(residual, abs_residual_partition_sums, residual_samples, predictor_order, min_partition_order, max_partition_order, bps);

        if(do_escape_coding)
            precompute_partition_info_escapes_(residual, raw_bits_per_partition, residual_samples, predictor_order, min_partition_order, max_partition_order);

        {
            int partition_order;
            unsigned sum;

            for(partition_order = (int)max_partition_order, sum = 0; partition_order >= (int)min_partition_order; partition_order--) {
                if(!
                    set_partitioned_rice_(
    #ifdef EXACT_RICE_BITS_CALCULATION
                        residual,
    #endif
                        abs_residual_partition_sums+sum,
                        raw_bits_per_partition+sum,
                        residual_samples,
                        predictor_order,
                        rice_parameter,
                        rice_parameter_limit,
                        rice_parameter_search_dist,
                        (unsigned)partition_order,
                        do_escape_coding,
                        &private_->partitioned_rice_contents_extra[!best_parameters_index],
                        &residual_bits
                    )
                )
                {
                    ASSERT(best_residual_bits != 0);
                    break;
                }
                sum += 1u << partition_order;
                if(best_residual_bits == 0 || residual_bits < best_residual_bits) {
                    best_residual_bits = residual_bits;
                    best_parameters_index = !best_parameters_index;
                    best_partition_order = partition_order;
                }
            }
        }

        best_ecm->data.partitioned_rice.order = best_partition_order;

        {
            /*
             * We are allowed to de-const the pointer based on our special
             * knowledge; it is const to the outside world.
             */
            EntropyCodingMethod_PartitionedRiceContents* prc = (EntropyCodingMethod_PartitionedRiceContents*)best_ecm->data.partitioned_rice.contents;
            unsigned partition;

            /* save best parameters and raw_bits */
            format_entropy_coding_method_partitioned_rice_contents_ensure_size(prc, flac_max(6u, best_partition_order));
            memcpy(prc->parameters, private_->partitioned_rice_contents_extra[best_parameters_index].parameters, sizeof(unsigned)*(1<<(best_partition_order)));
            if(do_escape_coding)
                memcpy(prc->raw_bits, private_->partitioned_rice_contents_extra[best_parameters_index].raw_bits, sizeof(unsigned)*(1<<(best_partition_order)));
            /*
             * Now need to check if the type should be changed to
             * ENTROPY_CODING_METHOD_PARTITIONED_RICE2 based on the
             * size of the rice parameters.
             */
            for(partition = 0; partition < (1u<<best_partition_order); partition++) {
                if(prc->parameters[partition] >= ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER) {
                    best_ecm->type = ENTROPY_CODING_METHOD_PARTITIONED_RICE2;
                    break;
                }
            }
        }

        return best_residual_bits;
        */
}

pub fn precompute_partition_info_sums(
        residual:                    &[i32],
        abs_residual_partition_sums: &[u64],
        residual_samples:            u32,
        predictor_order:             u32,
        min_partition_order:         u32,
        max_partition_order:         u32,
        bps:                         u32)  {
    
    todo!();
        /*
            const unsigned default_partition_samples = (residual_samples + predictor_order) >> max_partition_order;
        unsigned partitions = 1u << max_partition_order;

        ASSERT(default_partition_samples > predictor_order);

        /* first do max_partition_order */
        {
            unsigned partition, residual_sample, end = (unsigned)(-(int)predictor_order);
            /* WATCHOUT: "+ bps + MAX_EXTRA_RESIDUAL_BPS" is the maximum
             * assumed size of the average residual magnitude */
            if(bitmath_ilog2(default_partition_samples) + bps + MAX_EXTRA_RESIDUAL_BPS < 32) {
                u32 abs_residual_partition_sum;

                for(partition = residual_sample = 0; partition < partitions; partition++) {
                    end += default_partition_samples;
                    abs_residual_partition_sum = 0;
                    for( ; residual_sample < end; residual_sample++)
                        abs_residual_partition_sum += abs(residual[residual_sample]); /* abs(INT_MIN) is undefined, but if the residual is INT_MIN we have bigger problems */
                    abs_residual_partition_sums[partition] = abs_residual_partition_sum;
                }
            }
            else { /* have to pessimistically use 64 bits for accumulator */
                u64 abs_residual_partition_sum;

                for(partition = residual_sample = 0; partition < partitions; partition++) {
                    end += default_partition_samples;
                    abs_residual_partition_sum = 0;
                    for( ; residual_sample < end; residual_sample++)
                        abs_residual_partition_sum += abs(residual[residual_sample]); /* abs(INT_MIN) is undefined, but if the residual is INT_MIN we have bigger problems */
                    abs_residual_partition_sums[partition] = abs_residual_partition_sum;
                }
            }
        }

        /* now merge partitions for lower orders */
        {
            unsigned from_partition = 0, to_partition = partitions;
            int partition_order;
            for(partition_order = (int)max_partition_order - 1; partition_order >= (int)min_partition_order; partition_order--) {
                unsigned i;
                partitions >>= 1;
                for(i = 0; i < partitions; i++) {
                    abs_residual_partition_sums[to_partition++] =
                        abs_residual_partition_sums[from_partition  ] +
                        abs_residual_partition_sums[from_partition+1];
                    from_partition += 2;
                }
            }
        }
        */
}

pub fn precompute_partition_info_escapes(
        residual:               &[i32],
        raw_bits_per_partition: &[u32],
        residual_samples:       u32,
        predictor_order:        u32,
        min_partition_order:    u32,
        max_partition_order:    u32)  {
    
    todo!();
        /*
            int partition_order;
        unsigned from_partition, to_partition = 0;
        const unsigned blocksize = residual_samples + predictor_order;

        /* first do max_partition_order */
        for(partition_order = (int)max_partition_order; partition_order >= 0; partition_order--) {
            i32 r;
            u32 rmax;
            unsigned partition, partition_sample, partition_samples, residual_sample;
            const unsigned partitions = 1u << partition_order;
            const unsigned default_partition_samples = blocksize >> partition_order;

            ASSERT(default_partition_samples > predictor_order);

            for(partition = residual_sample = 0; partition < partitions; partition++) {
                partition_samples = default_partition_samples;
                if(partition == 0)
                    partition_samples -= predictor_order;
                rmax = 0;
                for(partition_sample = 0; partition_sample < partition_samples; partition_sample++) {
                    r = residual[residual_sample++];
                    /* OPT: maybe faster: rmax |= r ^ (r>>31) */
                    if(r < 0)
                        rmax |= ~r;
                    else
                        rmax |= r;
                }
                /* now we know all residual values are in the range [-rmax-1,rmax] */
                raw_bits_per_partition[partition] = rmax? bitmath_ilog2(rmax) + 2 : 1;
            }
            to_partition = partitions;
            break; /*@@@ yuck, should remove the 'for' loop instead */
        }

        /* now merge partitions for lower orders */
        for(from_partition = 0, --partition_order; partition_order >= (int)min_partition_order; partition_order--) {
            unsigned m;
            unsigned i;
            const unsigned partitions = 1u << partition_order;
            for(i = 0; i < partitions; i++) {
                m = raw_bits_per_partition[from_partition];
                from_partition++;
                raw_bits_per_partition[to_partition] = flac_max(m, raw_bits_per_partition[from_partition]);
                from_partition++;
                to_partition++;
            }
        }
        */
}

#[cfg(EXACT_RICE_BITS_CALCULATION)]
#[inline] pub fn count_rice_bits_in_partition(
        rice_parameter:    u32,
        partition_samples: u32,
        residual:          *const i32) -> u32 {
    
    todo!();
        /*
            unsigned i, partition_bits =
            ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN + /* actually could end up being ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN but err on side of 16bps */
            (1+rice_parameter) * partition_samples /* 1 for unary stop bit + rice_parameter for the binary portion */
        ;
        for(i = 0; i < partition_samples; i++)
            partition_bits += ( (u32)((residual[i]<<1)^(residual[i]>>31)) >> rice_parameter );
        return partition_bits;
        */
}

#[cfg(not(EXACT_RICE_BITS_CALCULATION))]
#[inline] pub fn count_rice_bits_in_partition(
        rice_parameter:             u32,
        partition_samples:          u32,
        abs_residual_partition_sum: u64) -> u32 {
    
    todo!();
        /*
            return
            ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN + /* actually could end up being ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN but err on side of 16bps */
            (1+rice_parameter) * partition_samples + /* 1 for unary stop bit + rice_parameter for the binary portion */
            (
                rice_parameter?
                    (unsigned)(abs_residual_partition_sum >> (rice_parameter-1)) /* rice_parameter-1 because the real coder sign-folds instead of using a sign bit */
                    : (unsigned)(abs_residual_partition_sum << 1) /* can't shift by negative number, so reverse */
            )
            - (partition_samples >> 1)
            /* -(partition_samples>>1) to subtract out extra contributions to the abs_residual_partition_sum.
             * The actual number of bits used is closer to the sum(for all i in the partition) of  abs(residual[i])>>(rice_parameter-1)
             * By using the abs_residual_partition sum, we also add in bits in the LSBs that would normally be shifted out.
             * So the subtraction term tries to guess how many extra bits were contributed.
             * If the LSBs are randomly distributed, this should average to 0.5 extra bits per sample.
             */
        ;
        */
}

pub fn set_partitioned_rice(

        #[cfg(EXACT_RICE_BITS_CALCULATION)]
        residual:                    &[i32],

        abs_residual_partition_sums: &[u64],
        raw_bits_per_partition:      &[u32],
        residual_samples:            u32,
        predictor_order:             u32,
        suggested_rice_parameter:    u32,
        rice_parameter_limit:        u32,
        rice_parameter_search_dist:  u32,
        partition_order:             u32,
        search_for_escapes:          bool,
        partitioned_rice_contents:   *mut EntropyCodingMethod_PartitionedRiceContents,
        bits:                        *mut u32) -> bool {
    
    todo!();
        /*
            unsigned rice_parameter, partition_bits;
        unsigned best_partition_bits, best_rice_parameter = 0;
        unsigned bits_ = ENTROPY_CODING_METHOD_TYPE_LEN + ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN;
        unsigned *parameters, *raw_bits;
    #ifdef ENABLE_RICE_PARAMETER_SEARCH
        unsigned min_rice_parameter, max_rice_parameter;
    #else
        (void)rice_parameter_search_dist;
    #endif

        ASSERT(suggested_rice_parameter < ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER);
        ASSERT(rice_parameter_limit <= ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER);

        format_entropy_coding_method_partitioned_rice_contents_ensure_size(partitioned_rice_contents, flac_max(6u, partition_order));
        parameters = partitioned_rice_contents->parameters;
        raw_bits = partitioned_rice_contents->raw_bits;

        if(partition_order == 0) {
            best_partition_bits = (unsigned)(-1);
    #ifdef ENABLE_RICE_PARAMETER_SEARCH
            if(rice_parameter_search_dist) {
                if(suggested_rice_parameter < rice_parameter_search_dist)
                    min_rice_parameter = 0;
                else
                    min_rice_parameter = suggested_rice_parameter - rice_parameter_search_dist;
                max_rice_parameter = suggested_rice_parameter + rice_parameter_search_dist;
                if(max_rice_parameter >= rice_parameter_limit) {
    #ifdef DEBUG_VERBOSE
                    fprintf(stderr, "clipping rice_parameter (%u -> %u) @5\n", max_rice_parameter, rice_parameter_limit - 1);
    #endif
                    max_rice_parameter = rice_parameter_limit - 1;
                }
            }
            else
                min_rice_parameter = max_rice_parameter = suggested_rice_parameter;

            for(rice_parameter = min_rice_parameter; rice_parameter <= max_rice_parameter; rice_parameter++) {
    #else
                rice_parameter = suggested_rice_parameter;
    #endif
    #ifdef EXACT_RICE_BITS_CALCULATION
                partition_bits = count_rice_bits_in_partition_(rice_parameter, residual_samples, residual);
    #else
                partition_bits = count_rice_bits_in_partition_(rice_parameter, residual_samples, abs_residual_partition_sums[0]);
    #endif
                if(partition_bits < best_partition_bits) {
                    best_rice_parameter = rice_parameter;
                    best_partition_bits = partition_bits;
                }
    #ifdef ENABLE_RICE_PARAMETER_SEARCH
            }
    #endif
            if(search_for_escapes) {
                partition_bits = ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN + ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN + raw_bits_per_partition[0] * residual_samples;
                if(partition_bits <= best_partition_bits) {
                    raw_bits[0] = raw_bits_per_partition[0];
                    best_rice_parameter = 0; /* will be converted to appropriate escape parameter later */
                    best_partition_bits = partition_bits;
                }
                else
                    raw_bits[0] = 0;
            }
            parameters[0] = best_rice_parameter;
            bits_ += best_partition_bits;
        }
        else {
            unsigned partition, residual_sample;
            unsigned partition_samples;
            u64 mean, k;
            const unsigned partitions = 1u << partition_order;
            for(partition = residual_sample = 0; partition < partitions; partition++) {
                partition_samples = (residual_samples+predictor_order) >> partition_order;
                if(partition == 0) {
                    if(partition_samples <= predictor_order)
                        return false;
                    else
                        partition_samples -= predictor_order;
                }
                mean = abs_residual_partition_sums[partition];
                /* we are basically calculating the size in bits of the
                 * average residual magnitude in the partition:
                 *   rice_parameter = floor(log2(mean/partition_samples))
                 * 'mean' is not a good name for the variable, it is
                 * actually the sum of magnitudes of all residual values
                 * in the partition, so the actual mean is
                 * mean/partition_samples
                 */
    #if 0 /* old simple code */
                for(rice_parameter = 0, k = partition_samples; k < mean; rice_parameter++, k <<= 1)
                    ;
    #else
    #if defined CPU_X86_64 /* and other 64-bit arch, too */
                if(mean <= 0x80000000/512) { /* 512: more or less optimal for both 16- and 24-bit input */
    #else
                if(mean <= 0x80000000/8) { /* 32-bit arch: use 32-bit math if possible */
    #endif
                    u32 k2, mean2 = (u32) mean;
                    rice_parameter = 0; k2 = partition_samples;
                    while(k2*8 < mean2) { /* requires: mean <= (2^31)/8 */
                        rice_parameter += 4; k2 <<= 4; /* tuned for 16-bit input */
                    }
                    while(k2 < mean2) { /* requires: mean <= 2^31 */
                        rice_parameter++; k2 <<= 1;
                    }
                }
                else {
                    rice_parameter = 0; k = partition_samples;
                    if(mean <= U64L(0x8000000000000000)/128) /* usually mean is _much_ smaller than this value */
                        while(k*128 < mean) { /* requires: mean <= (2^63)/128 */
                            rice_parameter += 8; k <<= 8; /* tuned for 24-bit input */
                        }
                    while(k < mean) { /* requires: mean <= 2^63 */
                        rice_parameter++; k <<= 1;
                    }
                }
    #endif
                if(rice_parameter >= rice_parameter_limit) {
    #ifdef DEBUG_VERBOSE
                    fprintf(stderr, "clipping rice_parameter (%u -> %u) @6\n", rice_parameter, rice_parameter_limit - 1);
    #endif
                    rice_parameter = rice_parameter_limit - 1;
                }

                best_partition_bits = (unsigned)(-1);
    #ifdef ENABLE_RICE_PARAMETER_SEARCH
                if(rice_parameter_search_dist) {
                    if(rice_parameter < rice_parameter_search_dist)
                        min_rice_parameter = 0;
                    else
                        min_rice_parameter = rice_parameter - rice_parameter_search_dist;
                    max_rice_parameter = rice_parameter + rice_parameter_search_dist;
                    if(max_rice_parameter >= rice_parameter_limit) {
    #ifdef DEBUG_VERBOSE
                        fprintf(stderr, "clipping rice_parameter (%u -> %u) @7\n", max_rice_parameter, rice_parameter_limit - 1);
    #endif
                        max_rice_parameter = rice_parameter_limit - 1;
                    }
                }
                else
                    min_rice_parameter = max_rice_parameter = rice_parameter;

                for(rice_parameter = min_rice_parameter; rice_parameter <= max_rice_parameter; rice_parameter++) {
    #endif
    #ifdef EXACT_RICE_BITS_CALCULATION
                    partition_bits = count_rice_bits_in_partition_(rice_parameter, partition_samples, residual+residual_sample);
    #else
                    partition_bits = count_rice_bits_in_partition_(rice_parameter, partition_samples, abs_residual_partition_sums[partition]);
    #endif
                    if(partition_bits < best_partition_bits) {
                        best_rice_parameter = rice_parameter;
                        best_partition_bits = partition_bits;
                    }
    #ifdef ENABLE_RICE_PARAMETER_SEARCH
                }
    #endif
                if(search_for_escapes) {
                    partition_bits = ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN + ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN + raw_bits_per_partition[partition] * partition_samples;
                    if(partition_bits <= best_partition_bits) {
                        raw_bits[partition] = raw_bits_per_partition[partition];
                        best_rice_parameter = 0; /* will be converted to appropriate escape parameter later */
                        best_partition_bits = partition_bits;
                    }
                    else
                        raw_bits[partition] = 0;
                }
                parameters[partition] = best_rice_parameter;
                bits_ += best_partition_bits;
                residual_sample += partition_samples;
            }
        }

        *bits = bits_;
        return true;
        */
}

pub fn get_wasted_bits(
        signal:  &[i32],
        samples: u32) -> u32 {
    
    todo!();
        /*
            unsigned i, shift;
        i32 x = 0;

        for(i = 0; i < samples && !(x&1); i++)
            x |= signal[i];

        if(x == 0) {
            shift = 0;
        }
        else {
            for(shift = 0; !(x&1); shift++)
                x >>= 1;
        }

        if(shift > 0) {
            for(i = 0; i < samples; i++)
                 signal[i] >>= shift;
        }

        return shift;
        */
}

pub fn append_to_verify_fifo(
        fifo:         *mut VerifyInputFifo,
        input:        *const &[i32],
        input_offset: u32,
        channels:     u32,
        wide_samples: u32)  {
    
    todo!();
        /*
            unsigned channel;

        for(channel = 0; channel < channels; channel++)
            memcpy(&fifo->data[channel][fifo->tail], &input[channel][input_offset], sizeof(i32) * wide_samples);

        fifo->tail += wide_samples;

        ASSERT(fifo->tail <= fifo->size);
        */
}

pub fn append_to_verify_fifo_interleaved(
        fifo:         *mut VerifyInputFifo,
        input:        &[i32],
        input_offset: u32,
        channels:     u32,
        wide_samples: u32)  {
    
    todo!();
        /*
            unsigned channel;
        unsigned sample, wide_sample;
        unsigned tail = fifo->tail;

        sample = input_offset * channels;
        for(wide_sample = 0; wide_sample < wide_samples; wide_sample++) {
            for(channel = 0; channel < channels; channel++)
                fifo->data[channel][tail] = input[sample++];
            tail++;
        }
        fifo->tail = tail;

        ASSERT(fifo->tail <= fifo->size);
        */
}

pub fn verify_read_callback(
        decoder:     *const StreamDecoder,
        buffer:      &[u8],
        bytes:       *mut usize,
        client_data: *mut c_void) -> StreamDecoderReadStatus {
    
    todo!();
        /*
            StreamEncoder *encoder = (StreamEncoder*)client_data;
        const size_t encoded_bytes = encoder->private_->verify.output.bytes;
        (void)decoder;

        if(encoder->private_->verify.needs_magic_hack) {
            ASSERT(*bytes >= STREAM_SYNC_LENGTH);
            *bytes = STREAM_SYNC_LENGTH;
            memcpy(buffer, STREAM_SYNC_STRING, *bytes);
            encoder->private_->verify.needs_magic_hack = false;
        }
        else {
            if(encoded_bytes == 0) {
                /*
                 * If we get here, a FIFO underflow has occurred,
                 * which means there is a bug somewhere.
                 */
                ASSERT(0);
                return STREAM_DECODER_READ_STATUS_ABORT;
            }
            else if(encoded_bytes < *bytes)
                *bytes = encoded_bytes;
            memcpy(buffer, encoder->private_->verify.output.data, *bytes);
            encoder->private_->verify.output.data += *bytes;
            encoder->private_->verify.output.bytes -= *bytes;
        }

        return STREAM_DECODER_READ_STATUS_CONTINUE;
        */
}

pub fn verify_write_callback(
        decoder:     *const StreamDecoder,
        frame:       *const Frame,
        buffer:      *const &[i32],
        client_data: *mut c_void) -> StreamDecoderWriteStatus {
    
    todo!();
        /*
            StreamEncoder *encoder = (StreamEncoder *)client_data;
        unsigned channel;
        const unsigned channels = frame->header.channels;
        const unsigned blocksize = frame->header.blocksize;
        const unsigned bytes_per_block = sizeof(i32) * blocksize;

        (void)decoder;

        for(channel = 0; channel < channels; channel++) {
            if(0 != memcmp(buffer[channel], encoder->private_->verify.input_fifo.data[channel], bytes_per_block)) {
                unsigned i, sample = 0;
                i32 expect = 0, got = 0;

                for(i = 0; i < blocksize; i++) {
                    if(buffer[channel][i] != encoder->private_->verify.input_fifo.data[channel][i]) {
                        sample = i;
                        expect = (i32)encoder->private_->verify.input_fifo.data[channel][i];
                        got = (i32)buffer[channel][i];
                        break;
                    }
                }
                ASSERT(i < blocksize);
                ASSERT(frame->header.number_type == FRAME_NUMBER_TYPE_SAMPLE_NUMBER);
                encoder->private_->verify.error_stats.absolute_sample = frame->header.number.sample_number + sample;
                encoder->private_->verify.error_stats.frame_number = (unsigned)(frame->header.number.sample_number / blocksize);
                encoder->private_->verify.error_stats.channel = channel;
                encoder->private_->verify.error_stats.sample = sample;
                encoder->private_->verify.error_stats.expected = expect;
                encoder->private_->verify.error_stats.got = got;
                encoder->protected_->state = STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA;
                return STREAM_DECODER_WRITE_STATUS_ABORT;
            }
        }
        /* dequeue the frame from the fifo */
        encoder->private_->verify.input_fifo.tail -= blocksize;
        ASSERT(encoder->private_->verify.input_fifo.tail <= OVERREAD_);
        for(channel = 0; channel < channels; channel++)
            memmove(&encoder->private_->verify.input_fifo.data[channel][0], &encoder->private_->verify.input_fifo.data[channel][blocksize], encoder->private_->verify.input_fifo.tail * sizeof(encoder->private_->verify.input_fifo.data[0][0]));
        return STREAM_DECODER_WRITE_STATUS_CONTINUE;
        */
}

pub fn verify_metadata_callback(
        decoder:     *const StreamDecoder,
        metadata:    *const StreamMetadata,
        client_data: *mut c_void)  {
    
    todo!();
        /*
            (void)decoder, (void)metadata, (void)client_data;
        */
}

pub fn verify_error_callback(
        decoder:     *const StreamDecoder,
        status:      StreamDecoderErrorStatus,
        client_data: *mut c_void)  {
    
    todo!();
        /*
            StreamEncoder *encoder = (StreamEncoder*)client_data;
        (void)decoder, (void)status;
        encoder->protected_->state = STREAM_ENCODER_VERIFY_DECODER_ERROR;
        */
}

#[ignore]
pub fn file_read_callback(
        encoder:     *const StreamEncoder,
        buffer:      &[u8],
        bytes:       *mut usize,
        client_data: *mut c_void) -> StreamEncoderReadStatus {
    
    todo!();
        /*
            (void)client_data;

        *bytes = fread(buffer, 1, *bytes, encoder->private_->file);
        if (*bytes == 0) {
            if (feof(encoder->private_->file))
                return STREAM_ENCODER_READ_STATUS_END_OF_STREAM;
            else if (ferror(encoder->private_->file))
                return STREAM_ENCODER_READ_STATUS_ABORT;
        }
        return STREAM_ENCODER_READ_STATUS_CONTINUE;
        */
}

#[ignore]
pub fn file_seek_callback(
        encoder:              *const StreamEncoder,
        absolute_byte_offset: u64,
        client_data:          *mut c_void) -> StreamEncoderSeekStatus {
    
    todo!();
        /*
            (void)client_data;

        if(fseeko(encoder->private_->file, (off_t)absolute_byte_offset, SEEK_SET) < 0)
            return STREAM_ENCODER_SEEK_STATUS_ERROR;
        else
            return STREAM_ENCODER_SEEK_STATUS_OK;
        */
}

#[ignore]
pub fn file_tell_callback(
        encoder:              *const StreamEncoder,
        absolute_byte_offset: *mut u64,
        client_data:          *mut c_void) -> StreamEncoderTellStatus {
    
    todo!();
        /*
            off_t offset;

        (void)client_data;

        offset = ftello(encoder->private_->file);

        if(offset < 0) {
            return STREAM_ENCODER_TELL_STATUS_ERROR;
        }
        else {
            *absolute_byte_offset = (u64)offset;
            return STREAM_ENCODER_TELL_STATUS_OK;
        }
        */
}

#[ignore]
#[cfg(VALGRIND_TESTING)]
pub fn local_fwrite(
        ptr:    *const c_void,
        size:   usize,
        nmemb:  usize,
        stream: *mut libc::FILE) -> usize {
    
    todo!();
        /*
            size_t ret = fwrite(ptr, size, nmemb, stream);
        if(!ferror(stream))
            fflush(stream);
        return ret;
        */
}

#[ignore]
#[cfg(not(VALGRIND_TESTING))]
macro_rules! local__fwrite {
    () => {
        /*
                fwrite
        */
    }
}

#[ignore]
pub fn file_write_callback(
        encoder:       *const StreamEncoder,
        buffer:        &[u8],
        bytes:         usize,
        samples:       u32,
        current_frame: u32,
        client_data:   *mut c_void) -> StreamEncoderWriteStatus {
    
    todo!();
        /*
            (void)client_data, (void)current_frame;

        if(local__fwrite(buffer, sizeof(byte), bytes, encoder->private_->file) == bytes) {
            bool call_it = 0 != encoder->private_->progress_callback && (
    #if HAS_OGG
                /* We would like to be able to use 'samples > 0' in the
                 * clause here but currently because of the nature of our
                 * Ogg writing implementation, 'samples' is always 0 (see
                 * ogg_encoder_aspect.c).  The downside is extra progress
                 * callbacks.
                 */
                encoder->private_->is_ogg? true :
    #endif
                samples > 0
            );
            if(call_it) {
                /* NOTE: We have to add +bytes, +samples, and +1 to the stats
                 * because at this point in the callback chain, the stats
                 * have not been updated.  Only after we return and control
                 * gets back to write_frame_() are the stats updated
                 */
                encoder->private_->progress_callback(encoder, encoder->private_->bytes_written+bytes, encoder->private_->samples_written+samples, encoder->private_->frames_written+(samples?1:0), encoder->private_->total_frames_estimate, encoder->private_->client_data);
            }
            return STREAM_ENCODER_WRITE_STATUS_OK;
        }
        else
            return STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR;
        */
}

/**
  | This will forcibly set stdout to binary
  | mode (for OSes that require it)
  |
  */
#[ignore]
pub fn get_binary_stdout() -> *mut libc::FILE {
    
    todo!();
        /*
            /* if something breaks here it is probably due to the presence or
         * absence of an underscore before the identifiers 'setmode',
         * 'fileno', and/or 'O_BINARY'; check your system header files.
         */
    #if defined _MSC_VER || defined __MINGW32__
        _setmode(_fileno(stdout), _O_BINARY);
    #elif defined __CYGWIN__
        /* almost certainly not needed for any modern Cygwin, but let's be safe... */
        setmode(_fileno(stdout), _O_BINARY);
    #elif defined __EMX__
        setmode(fileno(stdout), O_BINARY);
    #endif

        return stdout;
        */
}
