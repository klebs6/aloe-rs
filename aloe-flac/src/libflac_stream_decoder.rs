crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/stream_decoder.c]

/**
  | technically this should be in an "export.c"
  | but this is convenient enough
  |
  */
pub const SUPPORTS_OGG_FLAC: bool = cfg![HAS_OGG];

pub const id3v2_tag: [u8; 3] = [ b'I', b'D', b'3' ];

pub struct StreamDecoderPrivate {

    #[cfg(HAS_OGG)]
    is_ogg:                         bool,

    read_callback:                  StreamDecoderReadCallback,
    seek_callback:                  StreamDecoderSeekCallback,
    tell_callback:                  StreamDecoderTellCallback,
    length_callback:                StreamDecoderLengthCallback,
    eof_callback:                   StreamDecoderEofCallback,
    write_callback:                 StreamDecoderWriteCallback,
    metadata_callback:              StreamDecoderMetadataCallback,
    error_callback:                 StreamDecoderErrorCallback,

    local_lpc_restore_signal:       fn(
            residual:        &[i32],
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            data:            &[i32]
    ) -> c_void,

    local_lpc_restore_signal_64bit: fn(
            residual:        &[i32],
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            data:            &[i32]
    ) -> c_void,

    local_lpc_restore_signal_16bit: fn(
            residual:        &[i32],
            data_len:        u32,
            qlp_coeff:       &[i32],
            order:           u32,
            lp_quantization: i32,
            data:            &[i32]
    ) -> c_void,

    client_data:                    c_void,

    /**
      | only used if
      | stream_decoder_init_file()/stream_decoder_init_file()
      | called, else NULL
      |
      */
    file:                           libc::FILE,

    input:                          BitReader,
    output:                         [i32; MAX_CHANNELS],

    /**
      | WATCHOUT: these are the aligned pointers;
      | the real pointers that should be free()'d
      | are residual_unaligned[] below
      |
      */
    residual:                       [i32; MAX_CHANNELS],

    partitioned_rice_contents:      [EntropyCodingMethod_PartitionedRiceContents; MAX_CHANNELS],
    output_capacity:                u32,
    output_channels:                u32,
    fixed_block_size:               u32,
    next_fixed_block_size:          u32,
    samples_decoded:                u64,
    has_stream_info:                bool,
    has_seek_table:                 bool,
    stream_info:                    StreamMetadata,
    seek_table:                     StreamMetadata,

    /**
      | MAGIC number 128 == total number of metadata
      | block types == 1 << 7
      |
      */
    metadata_filter:                [bool; 128],

    metadata_filter_ids:            u8,

    /**
      | units for both are IDs, not bytes
      |
      */
    metadata_filter_ids_count:      usize,


    /**
      | units for both are IDs, not bytes
      |
      */
    metadata_filter_ids_capacity:   usize,

    frame:                          Frame,

    /**
      | true if there is a byte in lookahead
      |
      */
    cached:                         bool,

    cpuinfo:                        CPUInfo,

    /**
      | contains the sync code and reserved
      | bits
      |
      */
    header_warmup:                  [u8; 2],


    /**
      | temp storage when we need to look ahead
      | one byte in the stream
      |
      */
    lookahead:                      u8,


    /**
      | unaligned (original) pointers to allocated
      | data
      |
      */
    residual_unaligned:             [i32; MAX_CHANNELS],


    /**
      | initially gets protected_->md5_checking
      | but is turned off after a seek or if the
      | metadata has a zero MD5
      |
      */
    do_md5_checking:                bool,


    /**
      | used only during init() so we can call
      | reset to set up the decoder without rewinding
      | the input
      |
      */
    internal_reset_hack:            bool,

    is_seeking:                     bool,
    md5context:                     MD5Context,

    /**
      | this is the sum we computed from the decoded
      | data
      |
      */
    computed_md5sum:                [u8; 16],


    /**
      | (the rest of these are only used for seeking)
      | */ /* holds the info of the last frame
      | we seeked to
      |
      */
    last_frame:                     Frame,


    /**
      | hint to the seek routine of where in the
      | stream the first audio frame starts
      |
      */
    first_frame_offset:             u64,

    target_sample:                  u64,

    /**
      | used to tell whether we're decoding
      | a future version of FLAC or just got a
      | bad sync
      |
      */
    unparseable_frame_count:        u32,


    /**
      | hack needed in Ogg FLAC seek routine
      | to check when process_single() actually
      | writes a frame
      |
      */
    #[cfg(HAS_OGG)]
    got_a_frame: bool,
}

pub const flac_stream_decoder_state_string: &[&'static str] = &[ 
    "STREAM_DECODER_SEARCH_FOR_METADATA", 
    "STREAM_DECODER_READ_METADATA", 
    "STREAM_DECODER_SEARCH_FOR_FRAME_SYNC", 
    "STREAM_DECODER_READ_FRAME", 
    "STREAM_DECODER_END_OF_STREAM", 
    "STREAM_DECODER_OGG_ERROR", 
    "STREAM_DECODER_SEEK_ERROR", 
    "STREAM_DECODER_ABORTED", 
    "STREAM_DECODER_MEMORY_ALLOCATION_ERROR", 
    "STREAM_DECODER_UNINITIALIZED" 
];

pub const flac_stream_decoder_init_status_string: &[&'static str] = &[
    "STREAM_DECODER_INIT_STATUS_OK", 
    "STREAM_DECODER_INIT_STATUS_UNSUPPORTED_CONTAINER", 
    "STREAM_DECODER_INIT_STATUS_INVALID_CALLBACKS", 
    "STREAM_DECODER_INIT_STATUS_MEMORY_ALLOCATION_ERROR", 
    "STREAM_DECODER_INIT_STATUS_ERROR_OPENING_FILE", 
    "STREAM_DECODER_INIT_STATUS_ALREADY_INITIALIZED" 
];

pub const flac_stream_decoder_read_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_READ_STATUS_CONTINUE", 
    "STREAM_DECODER_READ_STATUS_END_OF_STREAM", 
    "STREAM_DECODER_READ_STATUS_ABORT" 
];

pub const flac_stream_decoder_seek_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_SEEK_STATUS_OK", 
    "STREAM_DECODER_SEEK_STATUS_ERROR", 
    "STREAM_DECODER_SEEK_STATUS_UNSUPPORTED" 
];

pub const flac_stream_decoder_tell_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_TELL_STATUS_OK", 
    "STREAM_DECODER_TELL_STATUS_ERROR", 
    "STREAM_DECODER_TELL_STATUS_UNSUPPORTED" 
];

pub const flac_stream_decoder_length_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_LENGTH_STATUS_OK", 
    "STREAM_DECODER_LENGTH_STATUS_ERROR", 
    "STREAM_DECODER_LENGTH_STATUS_UNSUPPORTED" 
];

pub const flac_stream_decoder_write_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_WRITE_STATUS_CONTINUE", 
    "STREAM_DECODER_WRITE_STATUS_ABORT" 
];

pub const flac_stream_decoder_error_status_string: &[&'static str] =  &[ 
    "STREAM_DECODER_ERROR_STATUS_LOST_SYNC", 
    "STREAM_DECODER_ERROR_STATUS_BAD_HEADER", 
    "STREAM_DECODER_ERROR_STATUS_FRAME_CRC_MISMATCH", 
    "STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM" 
];

pub fn flac_stream_decoder_new() -> *mut StreamDecoder {
    
    todo!();
        /*
            StreamDecoder *decoder;
        unsigned i;

        ASSERT(sizeof(int) >= 4); /* we want to die right away if this is not true */

        decoder = (StreamDecoder*) calloc(1, sizeof(StreamDecoder));
        if(decoder == 0) {
            return 0;
        
        }

        decoder->protected_ = (StreamDecoderProtected*) calloc(1, sizeof(StreamDecoderProtected));
        if(decoder->protected_ == 0) {
            free(decoder);
            return 0;
        }

        decoder->private_ = (StreamDecoderPrivate*) calloc(1, sizeof(StreamDecoderPrivate));
        if(decoder->private_ == 0) {
            free(decoder->protected_);
            free(decoder);
            return 0;
        }

        decoder->private_->input = bitreader_new();
        if(decoder->private_->input == 0) {
            free(decoder->private_);
            free(decoder->protected_);
            free(decoder);
            return 0;
        }

        decoder->private_->metadata_filter_ids_capacity = 16;
        if(0 == (decoder->private_->metadata_filter_ids = (byte*) malloc((STREAM_METADATA_APPLICATION_ID_LEN/8) * decoder->private_->metadata_filter_ids_capacity))) {
            bitreader_delete(decoder->private_->input);
            free(decoder->private_);
            free(decoder->protected_);
            free(decoder);
            return 0;
        }

        for(i = 0; i < MAX_CHANNELS; i++) {
            decoder->private_->output[i] = 0;
            decoder->private_->residual_unaligned[i] = decoder->private_->residual[i] = 0;
        }

        decoder->private_->output_capacity = 0;
        decoder->private_->output_channels = 0;
        decoder->private_->has_seek_table = false;

        for(i = 0; i < MAX_CHANNELS; i++)
            format_entropy_coding_method_partitioned_rice_contents_init(&decoder->private_->partitioned_rice_contents[i]);

        decoder->private_->file = 0;

        set_defaults_(decoder);

        decoder->protected_->state = STREAM_DECODER_UNINITIALIZED;

        return decoder;
        */
}

pub fn flac_stream_decoder_delete(decoder: *mut StreamDecoder)  {
    
    todo!();
        /*
            unsigned i;

        if (decoder == NULL)
            return ;

        ASSERT(0 != decoder->protected_);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->private_->input);

        (void)stream_decoder_finish(decoder);

        if(0 != decoder->private_->metadata_filter_ids)
            free(decoder->private_->metadata_filter_ids);

        bitreader_delete(decoder->private_->input);

        for(i = 0; i < MAX_CHANNELS; i++)
            format_entropy_coding_method_partitioned_rice_contents_clear(&decoder->private_->partitioned_rice_contents[i]);

        free(decoder->private_);
        free(decoder->protected_);
        free(decoder);
        */
}

pub fn init_stream_internal(
        decoder:           *mut StreamDecoder,
        read_callback:     StreamDecoderReadCallback,
        seek_callback:     StreamDecoderSeekCallback,
        tell_callback:     StreamDecoderTellCallback,
        length_callback:   StreamDecoderLengthCallback,
        eof_callback:      StreamDecoderEofCallback,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            ASSERT(0 != decoder);

        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return STREAM_DECODER_INIT_STATUS_ALREADY_INITIALIZED;

    #if !HAS_OGG
        if(is_ogg)
            return STREAM_DECODER_INIT_STATUS_UNSUPPORTED_CONTAINER;
    #endif

        if(
            0 == read_callback ||
            0 == write_callback ||
            0 == error_callback ||
            (seek_callback && (0 == tell_callback || 0 == length_callback || 0 == eof_callback))
        )
            return STREAM_DECODER_INIT_STATUS_INVALID_CALLBACKS;

    #if HAS_OGG
        decoder->private_->is_ogg = is_ogg;
        if(is_ogg && !ogg_decoder_aspect_init(&decoder->protected_->ogg_decoder_aspect))
            return decoder->protected_->initstate = STREAM_DECODER_INIT_STATUS_ERROR_OPENING_FILE;
    #endif

        /*
         * get the CPU info and set the function pointers
         */
        cpu_info(&decoder->private_->cpuinfo);
        /* first default to the non-asm routines */
        decoder->private_->local_lpc_restore_signal = lpc_restore_signal;
        decoder->private_->local_lpc_restore_signal_64bit = lpc_restore_signal_wide;
        decoder->private_->local_lpc_restore_signal_16bit = lpc_restore_signal;
        /* now override with asm where appropriate */
    #ifndef NO_ASM
        if(decoder->private_->cpuinfo.use_asm) {
    #ifdef CPU_IA32
            ASSERT(decoder->private_->cpuinfo.type == CPUINFO_TYPE_IA32);
    #ifdef HAS_NASM
            decoder->private_->local_lpc_restore_signal_64bit = lpc_restore_signal_wide_asm_ia32; /* OPT_IA32: was really necessary for GCC < 4.9 */
            if(decoder->private_->cpuinfo.ia32.mmx) {
                decoder->private_->local_lpc_restore_signal = lpc_restore_signal_asm_ia32;
                decoder->private_->local_lpc_restore_signal_16bit = lpc_restore_signal_asm_ia32_mmx;
            }
            else {
                decoder->private_->local_lpc_restore_signal = lpc_restore_signal_asm_ia32;
                decoder->private_->local_lpc_restore_signal_16bit = lpc_restore_signal_asm_ia32;
            }
    #endif
    #ifdef HAS_X86INTRIN
    # if defined SSE2_SUPPORTED && !defined HAS_NASM /* OPT_SSE: not better than MMX asm */
            if(decoder->private_->cpuinfo.ia32.sse2) {
                decoder->private_->local_lpc_restore_signal_16bit = lpc_restore_signal_16_intrin_sse2;
            }
    # endif
    # if defined SSE4_1_SUPPORTED
            if(decoder->private_->cpuinfo.ia32.sse41) {
                decoder->private_->local_lpc_restore_signal_64bit = lpc_restore_signal_wide_intrin_sse41;
            }
    # endif
    #endif
    #elif defined CPU_X86_64
            ASSERT(decoder->private_->cpuinfo.type == CPUINFO_TYPE_X86_64);
            /* No useful SSE optimizations yet */
    #endif
        }
    #endif

        /* from here on, errors are fatal */

        if(!bitreader_init(decoder->private_->input, read_callback_, decoder)) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return STREAM_DECODER_INIT_STATUS_MEMORY_ALLOCATION_ERROR;
        }

        decoder->private_->read_callback = read_callback;
        decoder->private_->seek_callback = seek_callback;
        decoder->private_->tell_callback = tell_callback;
        decoder->private_->length_callback = length_callback;
        decoder->private_->eof_callback = eof_callback;
        decoder->private_->write_callback = write_callback;
        decoder->private_->metadata_callback = metadata_callback;
        decoder->private_->error_callback = error_callback;
        decoder->private_->client_data = client_data;
        decoder->private_->fixed_block_size = decoder->private_->next_fixed_block_size = 0;
        decoder->private_->samples_decoded = 0;
        decoder->private_->has_stream_info = false;
        decoder->private_->cached = false;

        decoder->private_->do_md5_checking = decoder->protected_->md5_checking;
        decoder->private_->is_seeking = false;

        decoder->private_->internal_reset_hack = true; /* so the following reset does not try to rewind the input */
        if(!stream_decoder_reset(decoder)) {
            /* above call sets the state for us */
            return STREAM_DECODER_INIT_STATUS_MEMORY_ALLOCATION_ERROR;
        }

        return STREAM_DECODER_INIT_STATUS_OK;
        */
}

pub fn flac_stream_decoder_init_stream(
        decoder:           *mut StreamDecoder,
        read_callback:     StreamDecoderReadCallback,
        seek_callback:     StreamDecoderSeekCallback,
        tell_callback:     StreamDecoderTellCallback,
        length_callback:   StreamDecoderLengthCallback,
        eof_callback:      StreamDecoderEofCallback,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_stream_internal_(
            decoder,
            read_callback,
            seek_callback,
            tell_callback,
            length_callback,
            eof_callback,
            write_callback,
            metadata_callback,
            error_callback,
            client_data,
            /*is_ogg=*/false
        );
        */
}

pub fn flac_stream_decoder_init_ogg_stream(
        decoder:           *mut StreamDecoder,
        read_callback:     StreamDecoderReadCallback,
        seek_callback:     StreamDecoderSeekCallback,
        tell_callback:     StreamDecoderTellCallback,
        length_callback:   StreamDecoderLengthCallback,
        eof_callback:      StreamDecoderEofCallback,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_stream_internal_(
            decoder,
            read_callback,
            seek_callback,
            tell_callback,
            length_callback,
            eof_callback,
            write_callback,
            metadata_callback,
            error_callback,
            client_data,
            /*is_ogg=*/true
        );
        */
}

#[ignore]
pub fn init_file_internal(
        decoder:           *mut StreamDecoder,
        file:              *mut libc::FILE,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != file);

        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return decoder->protected_->initstate = STREAM_DECODER_INIT_STATUS_ALREADY_INITIALIZED;

        if(0 == write_callback || 0 == error_callback)
            return decoder->protected_->initstate = STREAM_DECODER_INIT_STATUS_INVALID_CALLBACKS;

        /*
         * To make sure that our file does not go unclosed after an error, we
         * must assign the FILE pointer before any further error can occur in
         * this routine.
         */
        if(file == stdin)
            file = get_binary_stdin_(); /* just to be safe */

        decoder->private_->file = file;

        return init_stream_internal_(
            decoder,
            file_read_callback_,
            decoder->private_->file == stdin? 0: file_seek_callback_,
            decoder->private_->file == stdin? 0: file_tell_callback_,
            decoder->private_->file == stdin? 0: file_length_callback_,
            file_eof_callback_,
            write_callback,
            metadata_callback,
            error_callback,
            client_data,
            is_ogg
        );
        */
}

#[ignore]
pub fn flac_stream_decoder_init_file(
        decoder:           *mut StreamDecoder,
        file:              *mut libc::FILE,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_FILE_internal_(decoder, file, write_callback, metadata_callback, error_callback, client_data, /*is_ogg=*/false);
        */
}


#[ignore]
pub fn flac_stream_decoder_init_ogg_file(
        decoder:           *mut StreamDecoder,
        file:              *mut libc::FILE,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_FILE_internal_(decoder, file, write_callback, metadata_callback, error_callback, client_data, /*is_ogg=*/true);
        */
}


#[ignore]
pub fn init_file_internal_with_filename(
        decoder:           *mut StreamDecoder,
        filename:          *const u8,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void,
        is_ogg:            bool) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            FILE *file;

        ASSERT(0 != decoder);

        /*
         * To make sure that our file does not go unclosed after an error, we
         * have to do the same entrance checks here that are later performed
         * in stream_decoder_init_FILE() before the FILE* is assigned.
         */
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return decoder->protected_->initstate = STREAM_DECODER_INIT_STATUS_ALREADY_INITIALIZED;

        if(0 == write_callback || 0 == error_callback)
            return decoder->protected_->initstate = STREAM_DECODER_INIT_STATUS_INVALID_CALLBACKS;

        file = filename? flac_fopen(filename, "rb") : stdin;

        if(0 == file)
            return STREAM_DECODER_INIT_STATUS_ERROR_OPENING_FILE;

        return init_FILE_internal_(decoder, file, write_callback, metadata_callback, error_callback, client_data, is_ogg);
        */
}

#[ignore]
pub fn flac_stream_decoder_init_file_with_filename(
        decoder:           *mut StreamDecoder,
        filename:          *const u8,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_file_internal_(decoder, filename, write_callback, metadata_callback, error_callback, client_data, /*is_ogg=*/false);
        */
}

#[ignore]
pub fn flac_stream_decoder_init_ogg_file_with_filename(
        decoder:           *mut StreamDecoder,
        filename:          *const u8,
        write_callback:    StreamDecoderWriteCallback,
        metadata_callback: StreamDecoderMetadataCallback,
        error_callback:    StreamDecoderErrorCallback,
        client_data:       *mut c_void) -> StreamDecoderInitStatus {
    
    todo!();
        /*
            return init_file_internal_(decoder, filename, write_callback, metadata_callback, error_callback, client_data, /*is_ogg=*/true);
        */
}

pub fn flac_stream_decoder_finish(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            bool md5_failed = false;
        unsigned i;

        ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);

        if(decoder->protected_->state == STREAM_DECODER_UNINITIALIZED)
            return true;

        /* see the comment in stream_decoder_reset() as to why we
         * always call MD5Final()
         */
        MD5Final(decoder->private_->computed_md5sum, &decoder->private_->md5context);

        if(decoder->private_->has_seek_table && 0 != decoder->private_->seek_table.data.seek_table.points) {
            free(decoder->private_->seek_table.data.seek_table.points);
            decoder->private_->seek_table.data.seek_table.points = 0;
            decoder->private_->has_seek_table = false;
        }
        bitreader_free(decoder->private_->input);
        for(i = 0; i < MAX_CHANNELS; i++) {
            /* WATCHOUT:
             * lpc_restore_signal_asm_ia32_mmx() requires that the
             * output arrays have a buffer of up to 3 zeroes in front
             * (at negative indices) for alignment purposes; we use 4
             * to keep the data well-aligned.
             */
            if(0 != decoder->private_->output[i]) {
                free(decoder->private_->output[i]-4);
                decoder->private_->output[i] = 0;
            }
            if(0 != decoder->private_->residual_unaligned[i]) {
                free(decoder->private_->residual_unaligned[i]);
                decoder->private_->residual_unaligned[i] = decoder->private_->residual[i] = 0;
            }
        }
        decoder->private_->output_capacity = 0;
        decoder->private_->output_channels = 0;

    #if HAS_OGG
        if(decoder->private_->is_ogg)
            ogg_decoder_aspect_finish(&decoder->protected_->ogg_decoder_aspect);
    #endif

        if(0 != decoder->private_->file) {
            if(decoder->private_->file != stdin)
                fclose(decoder->private_->file);
            decoder->private_->file = 0;
        }

        if(decoder->private_->do_md5_checking) {
            if(memcmp(decoder->private_->stream_info.data.stream_info.md5sum, decoder->private_->computed_md5sum, 16))
                md5_failed = true;
        }
        decoder->private_->is_seeking = false;

        set_defaults_(decoder);

        decoder->protected_->state = STREAM_DECODER_UNINITIALIZED;

        return !md5_failed;
        */
}

pub fn flac_stream_decoder_set_ogg_serial_number(
        decoder: *mut StreamDecoder,
        value:   i64) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
    #if HAS_OGG
        /* can't check decoder->private_->is_ogg since that's not set until init time */
        ogg_decoder_aspect_set_serial_number(&decoder->protected_->ogg_decoder_aspect, value);
        return true;
    #else
        (void)value;
        return false;
    #endif
        */
}

pub fn flac_stream_decoder_set_md5_checking(
        decoder: *mut StreamDecoder,
        value:   bool) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
        decoder->protected_->md5_checking = value;
        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_respond(
        decoder: *mut StreamDecoder,
        ty:      MetadataType) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        ASSERT((unsigned)type <= MAX_METADATA_TYPE_CODE);
        /* double protection */
        if((unsigned)type > MAX_METADATA_TYPE_CODE)
            return false;
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
        decoder->private_->metadata_filter[type] = true;
        if(type == METADATA_TYPE_APPLICATION)
            decoder->private_->metadata_filter_ids_count = 0;
        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_respond_application(
        decoder: *mut StreamDecoder,
        id:      [u8; 4]) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        ASSERT(0 != id);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;

        if(decoder->private_->metadata_filter[METADATA_TYPE_APPLICATION])
            return true;

        ASSERT(0 != decoder->private_->metadata_filter_ids);

        if(decoder->private_->metadata_filter_ids_count == decoder->private_->metadata_filter_ids_capacity) {
            if(0 == (decoder->private_->metadata_filter_ids = (byte*) safe_realloc_mul_2op_(decoder->private_->metadata_filter_ids, decoder->private_->metadata_filter_ids_capacity, /*times*/2))) {
                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                return false;
            }
            decoder->private_->metadata_filter_ids_capacity *= 2;
        }

        memcpy(decoder->private_->metadata_filter_ids + decoder->private_->metadata_filter_ids_count * (STREAM_METADATA_APPLICATION_ID_LEN/8), id, (STREAM_METADATA_APPLICATION_ID_LEN/8));
        decoder->private_->metadata_filter_ids_count++;

        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_respond_all(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            unsigned i;
        ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
        for(i = 0; i < sizeof(decoder->private_->metadata_filter) / sizeof(decoder->private_->metadata_filter[0]); i++)
            decoder->private_->metadata_filter[i] = true;
        decoder->private_->metadata_filter_ids_count = 0;
        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_ignore(
        decoder: *mut StreamDecoder,
        ty:      MetadataType) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        ASSERT((unsigned)type <= MAX_METADATA_TYPE_CODE);
        /* double protection */
        if((unsigned)type > MAX_METADATA_TYPE_CODE)
            return false;
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
        decoder->private_->metadata_filter[type] = false;
        if(type == METADATA_TYPE_APPLICATION)
            decoder->private_->metadata_filter_ids_count = 0;
        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_ignore_application(
        decoder: *mut StreamDecoder,
        id:      [u8; 4]) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        ASSERT(0 != id);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;

        if(!decoder->private_->metadata_filter[METADATA_TYPE_APPLICATION])
            return true;

        ASSERT(0 != decoder->private_->metadata_filter_ids);

        if(decoder->private_->metadata_filter_ids_count == decoder->private_->metadata_filter_ids_capacity) {
            if(0 == (decoder->private_->metadata_filter_ids = (byte*) safe_realloc_mul_2op_(decoder->private_->metadata_filter_ids, decoder->private_->metadata_filter_ids_capacity, /*times*/2))) {
                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                return false;
            }
            decoder->private_->metadata_filter_ids_capacity *= 2;
        }

        memcpy(decoder->private_->metadata_filter_ids + decoder->private_->metadata_filter_ids_count * (STREAM_METADATA_APPLICATION_ID_LEN/8), id, (STREAM_METADATA_APPLICATION_ID_LEN/8));
        decoder->private_->metadata_filter_ids_count++;

        return true;
        */
}

pub fn flac_stream_decoder_set_metadata_ignore_all(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);
        if(decoder->protected_->state != STREAM_DECODER_UNINITIALIZED)
            return false;
        memset(decoder->private_->metadata_filter, 0, sizeof(decoder->private_->metadata_filter));
        decoder->private_->metadata_filter_ids_count = 0;
        return true;
        */
}

pub fn flac_stream_decoder_get_state(decoder: *const StreamDecoder) -> StreamDecoderState {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->state;
        */
}

pub fn flac_stream_decoder_get_resolved_state_string(decoder: *const StreamDecoder) -> *const u8 {
    
    todo!();
        /*
            return StreamDecoderStateString[decoder->protected_->state];
        */
}

pub fn flac_stream_decoder_get_md5_checking(decoder: *const StreamDecoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->md5_checking;
        */
}

pub fn flac_stream_decoder_get_total_samples(decoder: *const StreamDecoder) -> u64 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->private_->has_stream_info? decoder->private_->stream_info.data.stream_info.total_samples : 0;
        */
}

pub fn flac_stream_decoder_get_channels(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->channels;
        */
}

pub fn flac_stream_decoder_get_channel_assignment(decoder: *const StreamDecoder) -> ChannelAssignment {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->channel_assignment;
        */
}

pub fn flac_stream_decoder_get_bits_per_sample(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->bits_per_sample;
        */
}

pub fn flac_stream_decoder_get_sample_rate(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->sample_rate;
        */
}

pub fn flac_stream_decoder_get_blocksize(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);
        return decoder->protected_->blocksize;
        */
}

pub fn flac_stream_decoder_get_decode_position(
        decoder:  *const StreamDecoder,
        position: *mut u64) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != position);

    #if HAS_OGG
        if(decoder->private_->is_ogg)
            return false;
    #endif
        if(0 == decoder->private_->tell_callback)
            return false;
        if(decoder->private_->tell_callback(decoder, position, decoder->private_->client_data) != STREAM_DECODER_TELL_STATUS_OK)
            return false;
        /* should never happen since all FLAC frames and metadata blocks are byte aligned, but check just in case */
        if(!bitreader_is_consumed_byte_aligned(decoder->private_->input))
            return false;
        ASSERT(*position >= stream_decoder_get_input_bytes_unconsumed(decoder));
        *position -= stream_decoder_get_input_bytes_unconsumed(decoder);
        return true;
        */
}

pub fn flac_stream_decoder_flush(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);

        decoder->private_->samples_decoded = 0;
        decoder->private_->do_md5_checking = false;

    #if HAS_OGG
        if(decoder->private_->is_ogg)
            ogg_decoder_aspect_flush(&decoder->protected_->ogg_decoder_aspect);
    #endif

        if(!bitreader_clear(decoder->private_->input)) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }
        decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;

        return true;
        */
}

pub fn flac_stream_decoder_reset(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);
        ASSERT(0 != decoder->protected_);

        if(!stream_decoder_flush(decoder)) {
            /* above call sets the state for us */
            return false;
        }

    #if HAS_OGG
        /*@@@ could go in !internal_reset_hack block below */
        if(decoder->private_->is_ogg)
            ogg_decoder_aspect_reset(&decoder->protected_->ogg_decoder_aspect);
    #endif

        /* Rewind if necessary.  If stream_decoder_init() is calling us,
         * (internal_reset_hack) don't try to rewind since we are already at
         * the beginning of the stream and don't want to fail if the input is
         * not seekable.
         */
        if(!decoder->private_->internal_reset_hack) {
            if(decoder->private_->file == stdin)
                return false; /* can't rewind stdin, reset fails */
            if(decoder->private_->seek_callback && decoder->private_->seek_callback(decoder, 0, decoder->private_->client_data) == STREAM_DECODER_SEEK_STATUS_ERROR)
                return false; /* seekable and seek fails, reset fails */
        }
        else
            decoder->private_->internal_reset_hack = false;

        decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_METADATA;

        decoder->private_->has_stream_info = false;
        if(decoder->private_->has_seek_table && 0 != decoder->private_->seek_table.data.seek_table.points) {
            free(decoder->private_->seek_table.data.seek_table.points);
            decoder->private_->seek_table.data.seek_table.points = 0;
            decoder->private_->has_seek_table = false;
        }
        decoder->private_->do_md5_checking = decoder->protected_->md5_checking;
        /*
         * This goes in reset() and not flush() because according to the spec, a
         * fixed-blocksize stream must stay that way through the whole stream.
         */
        decoder->private_->fixed_block_size = decoder->private_->next_fixed_block_size = 0;

        /* We initialize the MD5Context even though we may never use it.  This
         * is because md5 checking may be turned on to start and then turned off if
         * a seek occurs.  So we init the context here and finalize it in
         * stream_decoder_finish() to make sure things are always cleaned up
         * properly.
         */
        MD5Init(&decoder->private_->md5context);

        decoder->private_->first_frame_offset = 0;
        decoder->private_->unparseable_frame_count = 0;

        return true;
        */
}

pub fn flac_stream_decoder_process_single(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            bool got_a_frame;
        ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);

        while(1) {
            switch(decoder->protected_->state) {
                case STREAM_DECODER_SEARCH_FOR_METADATA:
                    if(!find_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_METADATA:
                    if(!read_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    else
                        return true;
                case STREAM_DECODER_SEARCH_FOR_FRAME_SYNC:
                    if(!frame_sync_(decoder))
                        return true; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_FRAME:
                    if(!read_frame_(decoder, &got_a_frame, /*do_full_decode=*/true))
                        return false; /* above function sets the status for us */
                    if(got_a_frame)
                        return true; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_END_OF_STREAM:
                case STREAM_DECODER_ABORTED:
                    return true;
                default:
                    ASSERT(0);
                    return false;
            }
        }
        */
}

pub fn flac_stream_decoder_process_until_end_of_metadata(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);

        while(1) {
            switch(decoder->protected_->state) {
                case STREAM_DECODER_SEARCH_FOR_METADATA:
                    if(!find_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_METADATA:
                    if(!read_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_SEARCH_FOR_FRAME_SYNC:
                case STREAM_DECODER_READ_FRAME:
                case STREAM_DECODER_END_OF_STREAM:
                case STREAM_DECODER_ABORTED:
                    return true;
                default:
                    ASSERT(0);
                    return false;
            }
        }
        */
}

pub fn flac_stream_decoder_process_until_end_of_stream(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            bool dummy;
        ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);

        while(1) {
            switch(decoder->protected_->state) {
                case STREAM_DECODER_SEARCH_FOR_METADATA:
                    if(!find_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_METADATA:
                    if(!read_metadata_(decoder))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_SEARCH_FOR_FRAME_SYNC:
                    if(!frame_sync_(decoder))
                        return true; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_FRAME:
                    if(!read_frame_(decoder, &dummy, /*do_full_decode=*/true))
                        return false; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_END_OF_STREAM:
                case STREAM_DECODER_ABORTED:
                    return true;
                default:
                    ASSERT(0);
                    return false;
            }
        }
        */
}

pub fn flac_stream_decoder_skip_single_frame(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            bool got_a_frame;
        ASSERT(0 != decoder);
        ASSERT(0 != decoder->protected_);

        while(1) {
            switch(decoder->protected_->state) {
                case STREAM_DECODER_SEARCH_FOR_METADATA:
                case STREAM_DECODER_READ_METADATA:
                    return false; /* above function sets the status for us */
                case STREAM_DECODER_SEARCH_FOR_FRAME_SYNC:
                    if(!frame_sync_(decoder))
                        return true; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_READ_FRAME:
                    if(!read_frame_(decoder, &got_a_frame, /*do_full_decode=*/false))
                        return false; /* above function sets the status for us */
                    if(got_a_frame)
                        return true; /* above function sets the status for us */
                    break;
                case STREAM_DECODER_END_OF_STREAM:
                case STREAM_DECODER_ABORTED:
                    return true;
                default:
                    ASSERT(0);
                    return false;
            }
        }
        */
}

pub fn flac_stream_decoder_seek_absolute(
        decoder: *mut StreamDecoder,
        sample:  u64) -> bool {
    
    todo!();
        /*
            u64 length;

        ASSERT(0 != decoder);

        if(
            decoder->protected_->state != STREAM_DECODER_SEARCH_FOR_METADATA &&
            decoder->protected_->state != STREAM_DECODER_READ_METADATA &&
            decoder->protected_->state != STREAM_DECODER_SEARCH_FOR_FRAME_SYNC &&
            decoder->protected_->state != STREAM_DECODER_READ_FRAME &&
            decoder->protected_->state != STREAM_DECODER_END_OF_STREAM
        )
            return false;

        if(0 == decoder->private_->seek_callback)
            return false;

        ASSERT(decoder->private_->seek_callback);
        ASSERT(decoder->private_->tell_callback);
        ASSERT(decoder->private_->length_callback);
        ASSERT(decoder->private_->eof_callback);

        if(stream_decoder_get_total_samples(decoder) > 0 && sample >= stream_decoder_get_total_samples(decoder))
            return false;

        decoder->private_->is_seeking = true;

        /* turn off md5 checking if a seek is attempted */
        decoder->private_->do_md5_checking = false;

        /* get the file length (currently our algorithm needs to know the length so it's also an error to get STREAM_DECODER_LENGTH_STATUS_UNSUPPORTED) */
        if(decoder->private_->length_callback(decoder, &length, decoder->private_->client_data) != STREAM_DECODER_LENGTH_STATUS_OK) {
            decoder->private_->is_seeking = false;
            return false;
        }

        /* if we haven't finished processing the metadata yet, do that so we have the STREAMINFO, SEEK_TABLE, and first_frame_offset */
        if(
            decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_METADATA ||
            decoder->protected_->state == STREAM_DECODER_READ_METADATA
        ) {
            if(!stream_decoder_process_until_end_of_metadata(decoder)) {
                /* above call sets the state for us */
                decoder->private_->is_seeking = false;
                return false;
            }
            /* check this again in case we didn't know total_samples the first time */
            if(stream_decoder_get_total_samples(decoder) > 0 && sample >= stream_decoder_get_total_samples(decoder)) {
                decoder->private_->is_seeking = false;
                return false;
            }
        }

        {
            const bool ok =
    #if HAS_OGG
                decoder->private_->is_ogg?
                seek_to_absolute_sample_ogg_(decoder, length, sample) :
    #endif
                seek_to_absolute_sample_(decoder, length, sample)
            ;
            decoder->private_->is_seeking = false;
            return ok;
        }
        */
}

pub fn flac_stream_decoder_get_input_bytes_unconsumed(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != decoder);
        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));
        ASSERT(!(bitreader_get_input_bits_unconsumed(decoder->private_->input) & 7));
        return bitreader_get_input_bits_unconsumed(decoder->private_->input) / 8;
        */
}

pub fn set_defaults(decoder: *mut StreamDecoder)  {
    
    todo!();
        /*
            #if HAS_OGG
        decoder->private_->is_ogg = false;
    #endif
        decoder->private_->read_callback = 0;
        decoder->private_->seek_callback = 0;
        decoder->private_->tell_callback = 0;
        decoder->private_->length_callback = 0;
        decoder->private_->eof_callback = 0;
        decoder->private_->write_callback = 0;
        decoder->private_->metadata_callback = 0;
        decoder->private_->error_callback = 0;
        decoder->private_->client_data = 0;

        memset(decoder->private_->metadata_filter, 0, sizeof(decoder->private_->metadata_filter));
        decoder->private_->metadata_filter[METADATA_TYPE_STREAMINFO] = true;
        decoder->private_->metadata_filter_ids_count = 0;

        decoder->protected_->md5_checking = false;

    #if HAS_OGG
        ogg_decoder_aspect_set_defaults(&decoder->protected_->ogg_decoder_aspect);
    #endif
        */
}

/**
  | This will forcibly set stdin to binary
  | mode (for OSes that require it)
  |
  */
#[ignore]
pub fn get_binary_stdin() -> *mut libc::FILE {
    
    todo!();
        /*
            /* if something breaks here it is probably due to the presence or
         * absence of an underscore before the identifiers 'setmode',
         * 'fileno', and/or 'O_BINARY'; check your system header files.
         */
    #if defined _MSC_VER || defined __MINGW32__
        _setmode(_fileno(stdin), _O_BINARY);
    #elif defined __CYGWIN__
        /* almost certainly not needed for any modern Cygwin, but let's be safe... */
        setmode(_fileno(stdin), _O_BINARY);
    #elif defined __EMX__
        setmode(fileno(stdin), O_BINARY);
    #endif

        return stdin;
        */
}

pub fn allocate_output(
        decoder:  *mut StreamDecoder,
        size:     u32,
        channels: u32) -> bool {
    
    todo!();
        /*
            unsigned i;
        i32 *tmp;

        if(size <= decoder->private_->output_capacity && channels <= decoder->private_->output_channels)
            return true;

        /* simply using realloc() is not practical because the number of channels may change mid-stream */

        for(i = 0; i < MAX_CHANNELS; i++) {
            if(0 != decoder->private_->output[i]) {
                free(decoder->private_->output[i]-4);
                decoder->private_->output[i] = 0;
            }
            if(0 != decoder->private_->residual_unaligned[i]) {
                free(decoder->private_->residual_unaligned[i]);
                decoder->private_->residual_unaligned[i] = decoder->private_->residual[i] = 0;
            }
        }

        for(i = 0; i < channels; i++) {
            /* WATCHOUT:
             * lpc_restore_signal_asm_ia32_mmx() requires that the
             * output arrays have a buffer of up to 3 zeroes in front
             * (at negative indices) for alignment purposes; we use 4
             * to keep the data well-aligned.
             */
            tmp = (i32*) safe_malloc_muladd2_(sizeof(i32), /*times (*/size, /*+*/4/*)*/);
            if(tmp == 0) {
                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                return false;
            }
            memset(tmp, 0, sizeof(i32)*4);
            decoder->private_->output[i] = tmp + 4;

            if(!memory_alloc_aligned_int32_array(size, &decoder->private_->residual_unaligned[i], &decoder->private_->residual[i])) {
                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                return false;
            }
        }

        decoder->private_->output_capacity = size;
        decoder->private_->output_channels = channels;

        return true;
        */
}

pub fn has_id_filtered(
        decoder: *mut StreamDecoder,
        id:      *mut u8) -> bool {
    
    todo!();
        /*
            size_t i;

        ASSERT(0 != decoder);
        ASSERT(0 != decoder->private_);

        for(i = 0; i < decoder->private_->metadata_filter_ids_count; i++)
            if(0 == memcmp(decoder->private_->metadata_filter_ids + i * (STREAM_METADATA_APPLICATION_ID_LEN/8), id, (STREAM_METADATA_APPLICATION_ID_LEN/8)))
                return true;

        return false;
        */
}

pub fn find_metadata(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            u32 x;
        unsigned i, id_;
        bool first = true;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        for(i = id_ = 0; i < 4; ) {
            if(decoder->private_->cached) {
                x = (u32)decoder->private_->lookahead;
                decoder->private_->cached = false;
            }
            else {
                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                    return false; /* read_callback_ sets the state for us */
            }
            if(x == STREAM_SYNC_STRING[i]) {
                first = true;
                i++;
                id_ = 0;
                continue;
            }

            if(id_ >= 3)
                return false;

            if(x == ID3V2_TAG_[id_]) {
                id_++;
                i = 0;
                if(id_ == 3) {
                    if(!skip_id3v2_tag_(decoder))
                        return false; /* skip_id3v2_tag_ sets the state for us */
                }
                continue;
            }
            id_ = 0;
            if(x == 0xff) { /* MAGIC NUMBER for the first 8 frame sync bits */
                decoder->private_->header_warmup[0] = (byte)x;
                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                    return false; /* read_callback_ sets the state for us */

                /* we have to check if we just read two 0xff's in a row; the second may actually be the beginning of the sync code */
                /* else we have to check if the second byte is the end of a sync code */
                if(x == 0xff) { /* MAGIC NUMBER for the first 8 frame sync bits */
                    decoder->private_->lookahead = (byte)x;
                    decoder->private_->cached = true;
                }
                else if(x >> 1 == 0x7c) { /* MAGIC NUMBER for the last 6 sync bits and reserved 7th bit */
                    decoder->private_->header_warmup[1] = (byte)x;
                    decoder->protected_->state = STREAM_DECODER_READ_FRAME;
                    return true;
                }
            }
            i = 0;
            if(first) {
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
                first = false;
            }
        }

        decoder->protected_->state = STREAM_DECODER_READ_METADATA;
        return true;
        */
}

pub fn read_metadata(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            bool is_last;
        u32 i, x, type, length;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_IS_LAST_LEN))
            return false; /* read_callback_ sets the state for us */
        is_last = x? true : false;

        if(!bitreader_read_raw_uint32(decoder->private_->input, &type, STREAM_METADATA_TYPE_LEN))
            return false; /* read_callback_ sets the state for us */

        if(!bitreader_read_raw_uint32(decoder->private_->input, &length, STREAM_METADATA_LENGTH_LEN))
            return false; /* read_callback_ sets the state for us */

        if(type == METADATA_TYPE_STREAMINFO) {
            if(!read_metadata_streaminfo_(decoder, is_last, length))
                return false;

            decoder->private_->has_stream_info = true;
            if(0 == memcmp(decoder->private_->stream_info.data.stream_info.md5sum, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", 16))
                decoder->private_->do_md5_checking = false;
            if(!decoder->private_->is_seeking && decoder->private_->metadata_filter[METADATA_TYPE_STREAMINFO] && decoder->private_->metadata_callback)
                decoder->private_->metadata_callback(decoder, &decoder->private_->stream_info, decoder->private_->client_data);
        }
        else if(type == METADATA_TYPE_SEEKTABLE) {
            if(!read_metadata_seektable_(decoder, is_last, length))
                return false;

            decoder->private_->has_seek_table = true;
            if(!decoder->private_->is_seeking && decoder->private_->metadata_filter[METADATA_TYPE_SEEKTABLE] && decoder->private_->metadata_callback)
                decoder->private_->metadata_callback(decoder, &decoder->private_->seek_table, decoder->private_->client_data);
        }
        else {
            bool skip_it = !decoder->private_->metadata_filter[type];
            unsigned real_length = length;
            StreamMetadata block;

            memset(&block, 0, sizeof(block));
            block.is_last = is_last;
            block.type = (MetadataType)type;
            block.length = length;

            if(type == METADATA_TYPE_APPLICATION) {
                if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, block.data.application.id, STREAM_METADATA_APPLICATION_ID_LEN/8))
                    return false; /* read_callback_ sets the state for us */

                if(real_length < STREAM_METADATA_APPLICATION_ID_LEN/8) { /* underflow check */
                    decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;/*@@@@@@ maybe wrong error? need to resync?*/
                    return false;
                }

                real_length -= STREAM_METADATA_APPLICATION_ID_LEN/8;

                if(decoder->private_->metadata_filter_ids_count > 0 && has_id_filtered_(decoder, block.data.application.id))
                    skip_it = !skip_it;
            }

            if(skip_it) {
                if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, real_length))
                    return false; /* read_callback_ sets the state for us */
            }
            else {
                bool ok = true;
                switch(type) {
                    case METADATA_TYPE_PADDING:
                        /* skip the padding bytes */
                        if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, real_length))
                            ok = false; /* read_callback_ sets the state for us */
                        break;
                    case METADATA_TYPE_APPLICATION:
                        /* remember, we read the ID already */
                        if(real_length > 0) {
                            if(0 == (block.data.application.data = (byte*) malloc(real_length))) {
                                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                                ok = false;
                            }
                            else if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, block.data.application.data, real_length))
                                ok = false; /* read_callback_ sets the state for us */
                        }
                        else
                            block.data.application.data = 0;
                        break;
                    case METADATA_TYPE_VORBIS_COMMENT:
                        if(!read_metadata_vorbiscomment_(decoder, &block.data.vorbis_comment, real_length))
                            ok = false;
                        break;
                    case METADATA_TYPE_CUESHEET:
                        if(!read_metadata_cuesheet_(decoder, &block.data.cue_sheet))
                            ok = false;
                        break;
                    case METADATA_TYPE_PICTURE:
                        if(!read_metadata_picture_(decoder, &block.data.picture))
                            ok = false;
                        break;
                    case METADATA_TYPE_STREAMINFO:
                    case METADATA_TYPE_SEEKTABLE:
                        ASSERT(0);
                        break;
                    default:
                        if(real_length > 0) {
                            if(0 == (block.data.unknown.data = (byte*) malloc(real_length))) {
                                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                                ok = false;
                            }
                            else if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, block.data.unknown.data, real_length))
                                ok = false; /* read_callback_ sets the state for us */
                        }
                        else
                            block.data.unknown.data = 0;
                        break;
                }
                if(ok && !decoder->private_->is_seeking && decoder->private_->metadata_callback)
                    decoder->private_->metadata_callback(decoder, &block, decoder->private_->client_data);

                /* now we have to free any malloc()ed data in the block */
                switch(type) {
                    case METADATA_TYPE_PADDING:
                        break;
                    case METADATA_TYPE_APPLICATION:
                        if(0 != block.data.application.data)
                            free(block.data.application.data);
                        break;
                    case METADATA_TYPE_VORBIS_COMMENT:
                        if(0 != block.data.vorbis_comment.vendor_string.entry)
                            free(block.data.vorbis_comment.vendor_string.entry);
                        if(block.data.vorbis_comment.num_comments > 0)
                            for(i = 0; i < block.data.vorbis_comment.num_comments; i++)
                                if(0 != block.data.vorbis_comment.comments[i].entry)
                                    free(block.data.vorbis_comment.comments[i].entry);
                        if(0 != block.data.vorbis_comment.comments)
                            free(block.data.vorbis_comment.comments);
                        break;
                    case METADATA_TYPE_CUESHEET:
                        if(block.data.cue_sheet.num_tracks > 0)
                            for(i = 0; i < block.data.cue_sheet.num_tracks; i++)
                                if(0 != block.data.cue_sheet.tracks[i].indices)
                                    free(block.data.cue_sheet.tracks[i].indices);
                        if(0 != block.data.cue_sheet.tracks)
                            free(block.data.cue_sheet.tracks);
                        break;
                    case METADATA_TYPE_PICTURE:
                        if(0 != block.data.picture.mime_type)
                            free(block.data.picture.mime_type);
                        if(0 != block.data.picture.description)
                            free(block.data.picture.description);
                        if(0 != block.data.picture.data)
                            free(block.data.picture.data);
                        break;
                    case METADATA_TYPE_STREAMINFO:
                    case METADATA_TYPE_SEEKTABLE:
                        ASSERT(0);
                    default:
                        if(0 != block.data.unknown.data)
                            free(block.data.unknown.data);
                        break;
                }

                if(!ok) /* anything that unsets "ok" should also make sure decoder->protected_->state is updated */
                    return false;
            }
        }

        if(is_last) {
            /* if this fails, it's OK, it's just a hint for the seek routine */
            if(!stream_decoder_get_decode_position(decoder, &decoder->private_->first_frame_offset))
                decoder->private_->first_frame_offset = 0;
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
        }

        return true;
        */
}

pub fn read_metadata_streaminfo(
        decoder: *mut StreamDecoder,
        is_last: bool,
        length:  u32) -> bool {
    
    todo!();
        /*
            u32 x;
        unsigned bits, used_bits = 0;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        decoder->private_->stream_info.type = METADATA_TYPE_STREAMINFO;
        decoder->private_->stream_info.is_last = is_last;
        decoder->private_->stream_info.length = length;

        bits = STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, bits))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.min_blocksize = x;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.max_blocksize = x;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.min_framesize = x;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.max_framesize = x;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.sample_rate = x;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_CHANNELS_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_CHANNELS_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.channels = x+1;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN;
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN))
            return false; /* read_callback_ sets the state for us */
        decoder->private_->stream_info.data.stream_info.bits_per_sample = x+1;
        used_bits += bits;

        bits = STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN;
        if(!bitreader_read_raw_uint64(decoder->private_->input, &decoder->private_->stream_info.data.stream_info.total_samples, STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN))
            return false; /* read_callback_ sets the state for us */
        used_bits += bits;

        if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, decoder->private_->stream_info.data.stream_info.md5sum, 16))
            return false; /* read_callback_ sets the state for us */
        used_bits += 16*8;

        /* skip the rest of the block */
        ASSERT(used_bits % 8 == 0);
        length -= (used_bits / 8);
        if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, length))
            return false; /* read_callback_ sets the state for us */

        return true;
        */
}

pub fn read_metadata_seektable(
        decoder: *mut StreamDecoder,
        is_last: bool,
        length:  u32) -> bool {
    
    todo!();
        /*
            u32 i, x;
        u64 xx;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        decoder->private_->seek_table.type = METADATA_TYPE_SEEKTABLE;
        decoder->private_->seek_table.is_last = is_last;
        decoder->private_->seek_table.length = length;

        decoder->private_->seek_table.data.seek_table.num_points = length / STREAM_METADATA_SEEKPOINT_LENGTH;

        /* use realloc since we may pass through here several times (e.g. after seeking) */
        if(0 == (decoder->private_->seek_table.data.seek_table.points = (StreamMetadata_SeekPoint*) safe_realloc_mul_2op_(decoder->private_->seek_table.data.seek_table.points, decoder->private_->seek_table.data.seek_table.num_points, /*times*/sizeof(StreamMetadata_SeekPoint)))) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }
        for(i = 0; i < decoder->private_->seek_table.data.seek_table.num_points; i++) {
            if(!bitreader_read_raw_uint64(decoder->private_->input, &xx, STREAM_METADATA_SEEKPOINT_SAMPLE_NUMBER_LEN))
                return false; /* read_callback_ sets the state for us */
            decoder->private_->seek_table.data.seek_table.points[i].sample_number = xx;

            if(!bitreader_read_raw_uint64(decoder->private_->input, &xx, STREAM_METADATA_SEEKPOINT_STREAM_OFFSET_LEN))
                return false; /* read_callback_ sets the state for us */
            decoder->private_->seek_table.data.seek_table.points[i].stream_offset = xx;

            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_SEEKPOINT_FRAME_SAMPLES_LEN))
                return false; /* read_callback_ sets the state for us */
            decoder->private_->seek_table.data.seek_table.points[i].frame_samples = x;
        }
        length -= (decoder->private_->seek_table.data.seek_table.num_points * STREAM_METADATA_SEEKPOINT_LENGTH);
        /* if there is a partial point left, skip over it */
        if(length > 0) {
            /*@@@ do a send_error_to_client_() here?  there's an argument for either way */
            if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, length))
                return false; /* read_callback_ sets the state for us */
        }

        return true;
        */
}

pub fn read_metadata_vorbiscomment(
        decoder: *mut StreamDecoder,
        obj:     *mut StreamMetadata_VorbisComment,
        length:  u32) -> bool {
    
    todo!();
        /*
            u32 i;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        /* read vendor string */
        if (length >= 8) {
            length -= 8; /* vendor string length + num comments entries alone take 8 bytes */
            ASSERT(STREAM_METADATA_VORBIS_COMMENT_ENTRY_LENGTH_LEN == 32);
            if (!bitreader_read_uint32_little_endian(decoder->private_->input, &obj->vendor_string.length))
                return false; /* read_callback_ sets the state for us */
            if (obj->vendor_string.length > 0) {
                if (length < obj->vendor_string.length) {
                    obj->vendor_string.length = 0;
                    obj->vendor_string.entry = 0;
                    goto skip;
                }
                else
                    length -= obj->vendor_string.length;
                if (0 == (obj->vendor_string.entry = (byte*) safe_malloc_add_2op_(obj->vendor_string.length, /*+*/1))) {
                    decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                    return false;
                }
                if (!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, obj->vendor_string.entry, obj->vendor_string.length))
                    return false; /* read_callback_ sets the state for us */
                obj->vendor_string.entry[obj->vendor_string.length] = '\0';
            }
            else
                obj->vendor_string.entry = 0;

            /* read num comments */
            ASSERT(STREAM_METADATA_VORBIS_COMMENT_NUM_COMMENTS_LEN == 32);
            if (!bitreader_read_uint32_little_endian(decoder->private_->input, &obj->num_comments))
                return false; /* read_callback_ sets the state for us */

            /* read comments */
            if (obj->num_comments > 0) {
                if (0 == (obj->comments = (StreamMetadata_VorbisComment_Entry*) safe_malloc_mul_2op_p(obj->num_comments, /*times*/sizeof(StreamMetadata_VorbisComment_Entry)))) {
                    decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                    return false;
                }
                for (i = 0; i < obj->num_comments; i++) {
                    ASSERT(STREAM_METADATA_VORBIS_COMMENT_ENTRY_LENGTH_LEN == 32);
                    if (length < 4) {
                        obj->num_comments = i;
                        goto skip;
                    }
                    else
                        length -= 4;
                    if (!bitreader_read_uint32_little_endian(decoder->private_->input, &obj->comments[i].length))
                        return false; /* read_callback_ sets the state for us */
                    if (obj->comments[i].length > 0) {
                        if (length < obj->comments[i].length) {
                            obj->comments[i].length = 0;
                            obj->comments[i].entry = 0;
                            obj->num_comments = i;
                            goto skip;
                        }
                        else
                            length -= obj->comments[i].length;
                        if (0 == (obj->comments[i].entry = (byte*) safe_malloc_add_2op_(obj->comments[i].length, /*+*/1))) {
                            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                            return false;
                        }
                        if (!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, obj->comments[i].entry, obj->comments[i].length))
                            return false; /* read_callback_ sets the state for us */
                        obj->comments[i].entry[obj->comments[i].length] = '\0';
                    }
                    else
                        obj->comments[i].entry = 0;
                }
            }
            else
                obj->comments = 0;
        }

      skip:
        if (length > 0) {
            /* This will only happen on files with invalid data in comments */
            if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, length))
                return false; /* read_callback_ sets the state for us */
        }

        return true;
        */
}

pub fn read_metadata_cuesheet(
        decoder: *mut StreamDecoder,
        obj:     *mut StreamMetadata_CueSheet) -> bool {
    
    todo!();
        /*
            u32 i, j, x;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        memset(obj, 0, sizeof(StreamMetadata_CueSheet));

        ASSERT(STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN % 8 == 0);
        if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, (byte*)obj->media_catalog_number, STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN/8))
            return false; /* read_callback_ sets the state for us */

        if(!bitreader_read_raw_uint64(decoder->private_->input, &obj->lead_in, STREAM_METADATA_CUESHEET_LEAD_IN_LEN))
            return false; /* read_callback_ sets the state for us */

        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_IS_CD_LEN))
            return false; /* read_callback_ sets the state for us */
        obj->is_cd = x? true : false;

        if(!bitreader_skip_bits_no_crc(decoder->private_->input, STREAM_METADATA_CUESHEET_RESERVED_LEN))
            return false; /* read_callback_ sets the state for us */

        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_NUM_TRACKS_LEN))
            return false; /* read_callback_ sets the state for us */
        obj->num_tracks = x;

        if(obj->num_tracks > 0) {
            if(0 == (obj->tracks = (StreamMetadata_CueSheet_Track*) safe_calloc_(obj->num_tracks, sizeof(StreamMetadata_CueSheet_Track)))) {
                decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                return false;
            }
            for(i = 0; i < obj->num_tracks; i++) {
                StreamMetadata_CueSheet_Track *track = &obj->tracks[i];
                if(!bitreader_read_raw_uint64(decoder->private_->input, &track->offset, STREAM_METADATA_CUESHEET_TRACK_OFFSET_LEN))
                    return false; /* read_callback_ sets the state for us */

                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_TRACK_NUMBER_LEN))
                    return false; /* read_callback_ sets the state for us */
                track->number = (byte)x;

                ASSERT(STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN % 8 == 0);
                if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, (byte*)track->isrc, STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN/8))
                    return false; /* read_callback_ sets the state for us */

                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_TRACK_TYPE_LEN))
                    return false; /* read_callback_ sets the state for us */
                track->type = x;

                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_TRACK_PRE_EMPHASIS_LEN))
                    return false; /* read_callback_ sets the state for us */
                track->pre_emphasis = x;

                if(!bitreader_skip_bits_no_crc(decoder->private_->input, STREAM_METADATA_CUESHEET_TRACK_RESERVED_LEN))
                    return false; /* read_callback_ sets the state for us */

                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_TRACK_NUM_INDICES_LEN))
                    return false; /* read_callback_ sets the state for us */
                track->num_indices = (byte)x;

                if(track->num_indices > 0) {
                    if(0 == (track->indices = (StreamMetadata_CueSheet_Index*) safe_calloc_(track->num_indices, sizeof(StreamMetadata_CueSheet_Index)))) {
                        decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
                        return false;
                    }
                    for(j = 0; j < track->num_indices; j++) {
                        StreamMetadata_CueSheet_Index *indx = &track->indices[j];
                        if(!bitreader_read_raw_uint64(decoder->private_->input, &indx->offset, STREAM_METADATA_CUESHEET_INDEX_OFFSET_LEN))
                            return false; /* read_callback_ sets the state for us */

                        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_CUESHEET_INDEX_NUMBER_LEN))
                            return false; /* read_callback_ sets the state for us */
                        indx->number = (byte)x;

                        if(!bitreader_skip_bits_no_crc(decoder->private_->input, STREAM_METADATA_CUESHEET_INDEX_RESERVED_LEN))
                            return false; /* read_callback_ sets the state for us */
                    }
                }
            }
        }

        return true;
        */
}

pub fn read_metadata_picture(
        decoder: *mut StreamDecoder,
        obj:     *mut StreamMetadata_Picture) -> bool {
    
    todo!();
        /*
            u32 x;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        /* read type */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_PICTURE_TYPE_LEN))
            return false; /* read_callback_ sets the state for us */
        obj->type = (StreamMetadata_Picture_Type) x;

        /* read MIME type */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_PICTURE_MIME_TYPE_LENGTH_LEN))
            return false; /* read_callback_ sets the state for us */
        if(0 == (obj->mime_type = (char*) safe_malloc_add_2op_(x, /*+*/1))) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }
        if(x > 0) {
            if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, (byte*)obj->mime_type, x))
                return false; /* read_callback_ sets the state for us */
        }
        obj->mime_type[x] = '\0';

        /* read description */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, STREAM_METADATA_PICTURE_DESCRIPTION_LENGTH_LEN))
            return false; /* read_callback_ sets the state for us */
        if(0 == (obj->description = (byte*) safe_malloc_add_2op_(x, /*+*/1))) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }
        if(x > 0) {
            if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, obj->description, x))
                return false; /* read_callback_ sets the state for us */
        }
        obj->description[x] = '\0';

        /* read width */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &obj->width, STREAM_METADATA_PICTURE_WIDTH_LEN))
            return false; /* read_callback_ sets the state for us */

        /* read height */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &obj->height, STREAM_METADATA_PICTURE_HEIGHT_LEN))
            return false; /* read_callback_ sets the state for us */

        /* read depth */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &obj->depth, STREAM_METADATA_PICTURE_DEPTH_LEN))
            return false; /* read_callback_ sets the state for us */

        /* read colors */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &obj->colors, STREAM_METADATA_PICTURE_COLORS_LEN))
            return false; /* read_callback_ sets the state for us */

        /* read data */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &(obj->data_length), STREAM_METADATA_PICTURE_DATA_LENGTH_LEN))
            return false; /* read_callback_ sets the state for us */
        if(0 == (obj->data = (byte*) safe_malloc_(obj->data_length))) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }
        if(obj->data_length > 0) {
            if(!bitreader_read_byte_block_aligned_no_crc(decoder->private_->input, obj->data, obj->data_length))
                return false; /* read_callback_ sets the state for us */
        }

        return true;
        */
}

pub fn skip_id3v2_tag(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            u32 x;
        unsigned i, skip;

        /* skip the version and flags bytes */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 24))
            return false; /* read_callback_ sets the state for us */
        /* get the size (in bytes) to skip */
        skip = 0;
        for(i = 0; i < 4; i++) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                return false; /* read_callback_ sets the state for us */
            skip <<= 7;
            skip |= (x & 0x7f);
        }
        /* skip the rest of the tag */
        if(!bitreader_skip_byte_block_aligned_no_crc(decoder->private_->input, skip))
            return false; /* read_callback_ sets the state for us */
        return true;
        */
}

pub fn frame_sync(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            u32 x;
        bool first = true;

        /* If we know the total number of samples in the stream, stop if we've read that many. */
        /* This will stop us, for example, from wasting time trying to sync on an ID3V1 tag. */
        if(stream_decoder_get_total_samples(decoder) > 0) {
            if(decoder->private_->samples_decoded >= stream_decoder_get_total_samples(decoder)) {
                decoder->protected_->state = STREAM_DECODER_END_OF_STREAM;
                return true;
            }
        }

        /* make sure we're byte aligned */
        if(!bitreader_is_consumed_byte_aligned(decoder->private_->input)) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, bitreader_bits_left_for_byte_alignment(decoder->private_->input)))
                return false; /* read_callback_ sets the state for us */
        }

        while(1) {
            if(decoder->private_->cached) {
                x = (u32)decoder->private_->lookahead;
                decoder->private_->cached = false;
            }
            else {
                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                    return false; /* read_callback_ sets the state for us */
            }
            if(x == 0xff) { /* MAGIC NUMBER for the first 8 frame sync bits */
                decoder->private_->header_warmup[0] = (byte)x;
                if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                    return false; /* read_callback_ sets the state for us */

                /* we have to check if we just read two 0xff's in a row; the second may actually be the beginning of the sync code */
                /* else we have to check if the second byte is the end of a sync code */
                if(x == 0xff) { /* MAGIC NUMBER for the first 8 frame sync bits */
                    decoder->private_->lookahead = (byte)x;
                    decoder->private_->cached = true;
                }
                else if(x >> 1 == 0x7c) { /* MAGIC NUMBER for the last 6 sync bits and reserved 7th bit */
                    decoder->private_->header_warmup[1] = (byte)x;
                    decoder->protected_->state = STREAM_DECODER_READ_FRAME;
                    return true;
                }
            }
            if(first) {
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
                first = false;
            }
        }

        return true;
        */
}

pub fn read_frame(
        decoder:        *mut StreamDecoder,
        got_a_frame:    *mut bool,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            unsigned channel;
        unsigned i;
        i32 mid, side;
        unsigned frame_crc; /* the one we calculate from the input stream */
        u32 x;

        *got_a_frame = false;

        /* init the CRC */
        frame_crc = 0;
        frame_crc = CRC16_UPDATE(decoder->private_->header_warmup[0], frame_crc);
        frame_crc = CRC16_UPDATE(decoder->private_->header_warmup[1], frame_crc);
        bitreader_reset_read_crc16(decoder->private_->input, (uint16)frame_crc);

        if(!read_frame_header_(decoder))
            return false;
        if(decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_FRAME_SYNC) /* means we didn't sync on a valid header */
            return true;
        if(!allocate_output_(decoder, decoder->private_->frame.header.blocksize, decoder->private_->frame.header.channels))
            return false;
        for(channel = 0; channel < decoder->private_->frame.header.channels; channel++) {
            /*
             * first figure the correct bits-per-sample of the subframe
             */
            unsigned bps = decoder->private_->frame.header.bits_per_sample;
            switch(decoder->private_->frame.header.channel_assignment) {
                case CHANNEL_ASSIGNMENT_INDEPENDENT:
                    /* no adjustment needed */
                    break;
                case CHANNEL_ASSIGNMENT_LEFT_SIDE:
                    ASSERT(decoder->private_->frame.header.channels == 2);
                    if(channel == 1)
                        bps++;
                    break;
                case CHANNEL_ASSIGNMENT_RIGHT_SIDE:
                    ASSERT(decoder->private_->frame.header.channels == 2);
                    if(channel == 0)
                        bps++;
                    break;
                case CHANNEL_ASSIGNMENT_MID_SIDE:
                    ASSERT(decoder->private_->frame.header.channels == 2);
                    if(channel == 1)
                        bps++;
                    break;
                default:
                    ASSERT(0);
            }
            /*
             * now read it
             */
            if(!read_subframe_(decoder, channel, bps, do_full_decode))
                return false;
            if(decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_FRAME_SYNC) /* means bad sync or got corruption */
                return true;
        }
        if(!read_zero_padding_(decoder))
            return false;
        if(decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_FRAME_SYNC) /* means bad sync or got corruption (i.e. "zero bits" were not all zeroes) */
            return true;

        /*
         * Read the frame CRC-16 from the footer and check
         */
        frame_crc = bitreader_get_read_crc16(decoder->private_->input);
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, FRAME_FOOTER_CRC_LEN))
            return false; /* read_callback_ sets the state for us */
        if(frame_crc == x) {
            if(do_full_decode) {
                /* Undo any special channel coding */
                switch(decoder->private_->frame.header.channel_assignment) {
                    case CHANNEL_ASSIGNMENT_INDEPENDENT:
                        /* do nothing */
                        break;
                    case CHANNEL_ASSIGNMENT_LEFT_SIDE:
                        ASSERT(decoder->private_->frame.header.channels == 2);
                        for(i = 0; i < decoder->private_->frame.header.blocksize; i++)
                            decoder->private_->output[1][i] = decoder->private_->output[0][i] - decoder->private_->output[1][i];
                        break;
                    case CHANNEL_ASSIGNMENT_RIGHT_SIDE:
                        ASSERT(decoder->private_->frame.header.channels == 2);
                        for(i = 0; i < decoder->private_->frame.header.blocksize; i++)
                            decoder->private_->output[0][i] += decoder->private_->output[1][i];
                        break;
                    case CHANNEL_ASSIGNMENT_MID_SIDE:
                        ASSERT(decoder->private_->frame.header.channels == 2);
                        for(i = 0; i < decoder->private_->frame.header.blocksize; i++) {
    #if 1
                            mid = decoder->private_->output[0][i];
                            side = decoder->private_->output[1][i];
                            mid <<= 1;
                            mid |= (side & 1); /* i.e. if 'side' is odd... */
                            decoder->private_->output[0][i] = (mid + side) >> 1;
                            decoder->private_->output[1][i] = (mid - side) >> 1;
    #else
                            /* OPT: without 'side' temp variable */
                            mid = (decoder->private_->output[0][i] << 1) | (decoder->private_->output[1][i] & 1); /* i.e. if 'side' is odd... */
                            decoder->private_->output[0][i] = (mid + decoder->private_->output[1][i]) >> 1;
                            decoder->private_->output[1][i] = (mid - decoder->private_->output[1][i]) >> 1;
    #endif
                        }
                        break;
                    default:
                        ASSERT(0);
                        break;
                }
            }
        }
        else {
            /* Bad frame, emit error and zero the output signal */
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_FRAME_CRC_MISMATCH);
            if(do_full_decode) {
                for(channel = 0; channel < decoder->private_->frame.header.channels; channel++) {
                    memset(decoder->private_->output[channel], 0, sizeof(i32) * decoder->private_->frame.header.blocksize);
                }
            }
        }

        *got_a_frame = true;

        /* we wait to update fixed_block_size until here, when we're sure we've got a proper frame and hence a correct blocksize */
        if(decoder->private_->next_fixed_block_size)
            decoder->private_->fixed_block_size = decoder->private_->next_fixed_block_size;

        /* put the latest values into the public section of the decoder instance */
        decoder->protected_->channels = decoder->private_->frame.header.channels;
        decoder->protected_->channel_assignment = decoder->private_->frame.header.channel_assignment;
        decoder->protected_->bits_per_sample = decoder->private_->frame.header.bits_per_sample;
        decoder->protected_->sample_rate = decoder->private_->frame.header.sample_rate;
        decoder->protected_->blocksize = decoder->private_->frame.header.blocksize;

        ASSERT(decoder->private_->frame.header.number_type == FRAME_NUMBER_TYPE_SAMPLE_NUMBER);
        decoder->private_->samples_decoded = decoder->private_->frame.header.number.sample_number + decoder->private_->frame.header.blocksize;

        /* write it */
        if(do_full_decode) {
            if(write_audio_frame_to_client_(decoder, &decoder->private_->frame, (const i32 * const *)decoder->private_->output) != STREAM_DECODER_WRITE_STATUS_CONTINUE)
                return false;
        }

        decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
        return true;
        */
}

pub fn read_frame_header(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            u32 x;
        u64 xx;
        unsigned i, blocksize_hint = 0, sample_rate_hint = 0;
        byte crc8, raw_header[16]; /* MAGIC NUMBER based on the maximum frame header size, including CRC */
        unsigned raw_header_len;
        bool is_unparseable = false;

        ASSERT(bitreader_is_consumed_byte_aligned(decoder->private_->input));

        /* init the raw header with the saved bits from synchronization */
        raw_header[0] = decoder->private_->header_warmup[0];
        raw_header[1] = decoder->private_->header_warmup[1];
        raw_header_len = 2;

        /* check to make sure that reserved bit is 0 */
        if(raw_header[1] & 0x02) /* MAGIC NUMBER */
            is_unparseable = true;

        /*
         * Note that along the way as we read the header, we look for a sync
         * code inside.  If we find one it would indicate that our original
         * sync was bad since there cannot be a sync code in a valid header.
         *
         * Three kinds of things can go wrong when reading the frame header:
         *  1) We may have sync'ed incorrectly and not landed on a frame header.
         *     If we don't find a sync code, it can end up looking like we read
         *     a valid but unparseable header, until getting to the frame header
         *     CRC.  Even then we could get a false positive on the CRC.
         *  2) We may have sync'ed correctly but on an unparseable frame (from a
         *     future encoder).
         *  3) We may be on a damaged frame which appears valid but unparseable.
         *
         * For all these reasons, we try and read a complete frame header as
         * long as it seems valid, even if unparseable, up until the frame
         * header CRC.
         */

        /*
         * read in the raw header as bytes so we can CRC it, and parse it on the way
         */
        for(i = 0; i < 2; i++) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                return false; /* read_callback_ sets the state for us */
            if(x == 0xff) { /* MAGIC NUMBER for the first 8 frame sync bits */
                /* if we get here it means our original sync was erroneous since the sync code cannot appear in the header */
                decoder->private_->lookahead = (byte)x;
                decoder->private_->cached = true;
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_BAD_HEADER);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
            }
            raw_header[raw_header_len++] = (byte)x;
        }

        switch(x = raw_header[2] >> 4) {
            case 0:
                is_unparseable = true;
                break;
            case 1:
                decoder->private_->frame.header.blocksize = 192;
                break;
            case 2:
            case 3:
            case 4:
            case 5:
                decoder->private_->frame.header.blocksize = 576 << (x-2);
                break;
            case 6:
            case 7:
                blocksize_hint = x;
                break;
            case 8:
            case 9:
            case 10:
            case 11:
            case 12:
            case 13:
            case 14:
            case 15:
                decoder->private_->frame.header.blocksize = 256 << (x-8);
                break;
            default:
                ASSERT(0);
                break;
        }

        switch(x = raw_header[2] & 0x0f) {
            case 0:
                if(decoder->private_->has_stream_info)
                    decoder->private_->frame.header.sample_rate = decoder->private_->stream_info.data.stream_info.sample_rate;
                else
                    is_unparseable = true;
                break;
            case 1:
                decoder->private_->frame.header.sample_rate = 88200;
                break;
            case 2:
                decoder->private_->frame.header.sample_rate = 176400;
                break;
            case 3:
                decoder->private_->frame.header.sample_rate = 192000;
                break;
            case 4:
                decoder->private_->frame.header.sample_rate = 8000;
                break;
            case 5:
                decoder->private_->frame.header.sample_rate = 16000;
                break;
            case 6:
                decoder->private_->frame.header.sample_rate = 22050;
                break;
            case 7:
                decoder->private_->frame.header.sample_rate = 24000;
                break;
            case 8:
                decoder->private_->frame.header.sample_rate = 32000;
                break;
            case 9:
                decoder->private_->frame.header.sample_rate = 44100;
                break;
            case 10:
                decoder->private_->frame.header.sample_rate = 48000;
                break;
            case 11:
                decoder->private_->frame.header.sample_rate = 96000;
                break;
            case 12:
            case 13:
            case 14:
                sample_rate_hint = x;
                break;
            case 15:
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_BAD_HEADER);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
            default:
                ASSERT(0);
        }

        x = (unsigned)(raw_header[3] >> 4);
        if(x & 8) {
            decoder->private_->frame.header.channels = 2;
            switch(x & 7) {
                case 0:
                    decoder->private_->frame.header.channel_assignment = CHANNEL_ASSIGNMENT_LEFT_SIDE;
                    break;
                case 1:
                    decoder->private_->frame.header.channel_assignment = CHANNEL_ASSIGNMENT_RIGHT_SIDE;
                    break;
                case 2:
                    decoder->private_->frame.header.channel_assignment = CHANNEL_ASSIGNMENT_MID_SIDE;
                    break;
                default:
                    is_unparseable = true;
                    break;
            }
        }
        else {
            decoder->private_->frame.header.channels = (unsigned)x + 1;
            decoder->private_->frame.header.channel_assignment = CHANNEL_ASSIGNMENT_INDEPENDENT;
        }

        switch(x = (unsigned)(raw_header[3] & 0x0e) >> 1) {
            case 0:
                if(decoder->private_->has_stream_info)
                    decoder->private_->frame.header.bits_per_sample = decoder->private_->stream_info.data.stream_info.bits_per_sample;
                else
                    is_unparseable = true;
                break;
            case 1:
                decoder->private_->frame.header.bits_per_sample = 8;
                break;
            case 2:
                decoder->private_->frame.header.bits_per_sample = 12;
                break;
            case 4:
                decoder->private_->frame.header.bits_per_sample = 16;
                break;
            case 5:
                decoder->private_->frame.header.bits_per_sample = 20;
                break;
            case 6:
                decoder->private_->frame.header.bits_per_sample = 24;
                break;
            case 3:
            case 7:
                is_unparseable = true;
                break;
            default:
                ASSERT(0);
                break;
        }

        /* check to make sure that reserved bit is 0 */
        if(raw_header[3] & 0x01) /* MAGIC NUMBER */
            is_unparseable = true;

        /* read the frame's starting sample number (or frame number as the case may be) */
        if(
            raw_header[1] & 0x01 ||
            /*@@@ this clause is a concession to the old way of doing variable blocksize; the only known implementation is flake and can probably be removed without inconveniencing anyone */
            (decoder->private_->has_stream_info && decoder->private_->stream_info.data.stream_info.min_blocksize != decoder->private_->stream_info.data.stream_info.max_blocksize)
        ) { /* variable blocksize */
            if(!bitreader_read_utf8_uint64(decoder->private_->input, &xx, raw_header, &raw_header_len))
                return false; /* read_callback_ sets the state for us */
            if(xx == U64L(0xffffffffffffffff)) { /* i.e. non-UTF8 code... */
                decoder->private_->lookahead = raw_header[raw_header_len-1]; /* back up as much as we can */
                decoder->private_->cached = true;
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_BAD_HEADER);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
            }
            decoder->private_->frame.header.number_type = FRAME_NUMBER_TYPE_SAMPLE_NUMBER;
            decoder->private_->frame.header.number.sample_number = xx;
        }
        else { /* fixed blocksize */
            if(!bitreader_read_utf8_uint32(decoder->private_->input, &x, raw_header, &raw_header_len))
                return false; /* read_callback_ sets the state for us */
            if(x == 0xffffffff) { /* i.e. non-UTF8 code... */
                decoder->private_->lookahead = raw_header[raw_header_len-1]; /* back up as much as we can */
                decoder->private_->cached = true;
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_BAD_HEADER);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
            }
            decoder->private_->frame.header.number_type = FRAME_NUMBER_TYPE_FRAME_NUMBER;
            decoder->private_->frame.header.number.frame_number = x;
        }

        if(blocksize_hint) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                return false; /* read_callback_ sets the state for us */
            raw_header[raw_header_len++] = (byte)x;
            if(blocksize_hint == 7) {
                u32 _x;
                if(!bitreader_read_raw_uint32(decoder->private_->input, &_x, 8))
                    return false; /* read_callback_ sets the state for us */
                raw_header[raw_header_len++] = (byte)_x;
                x = (x << 8) | _x;
            }
            decoder->private_->frame.header.blocksize = x+1;
        }

        if(sample_rate_hint) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
                return false; /* read_callback_ sets the state for us */
            raw_header[raw_header_len++] = (byte)x;
            if(sample_rate_hint != 12) {
                u32 _x;
                if(!bitreader_read_raw_uint32(decoder->private_->input, &_x, 8))
                    return false; /* read_callback_ sets the state for us */
                raw_header[raw_header_len++] = (byte)_x;
                x = (x << 8) | _x;
            }
            if(sample_rate_hint == 12)
                decoder->private_->frame.header.sample_rate = x*1000;
            else if(sample_rate_hint == 13)
                decoder->private_->frame.header.sample_rate = x;
            else
                decoder->private_->frame.header.sample_rate = x*10;
        }

        /* read the CRC-8 byte */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8))
            return false; /* read_callback_ sets the state for us */
        crc8 = (byte)x;

        if(crc8(raw_header, raw_header_len) != crc8) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_BAD_HEADER);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }

        /* calculate the sample number from the frame number if needed */
        decoder->private_->next_fixed_block_size = 0;
        if(decoder->private_->frame.header.number_type == FRAME_NUMBER_TYPE_FRAME_NUMBER) {
            x = decoder->private_->frame.header.number.frame_number;
            decoder->private_->frame.header.number_type = FRAME_NUMBER_TYPE_SAMPLE_NUMBER;
            if(decoder->private_->fixed_block_size)
                decoder->private_->frame.header.number.sample_number = (u64)decoder->private_->fixed_block_size * (u64)x;
            else if(decoder->private_->has_stream_info) {
                if(decoder->private_->stream_info.data.stream_info.min_blocksize == decoder->private_->stream_info.data.stream_info.max_blocksize) {
                    decoder->private_->frame.header.number.sample_number = (u64)decoder->private_->stream_info.data.stream_info.min_blocksize * (u64)x;
                    decoder->private_->next_fixed_block_size = decoder->private_->stream_info.data.stream_info.max_blocksize;
                }
                else
                    is_unparseable = true;
            }
            else if(x == 0) {
                decoder->private_->frame.header.number.sample_number = 0;
                decoder->private_->next_fixed_block_size = decoder->private_->frame.header.blocksize;
            }
            else {
                /* can only get here if the stream has invalid frame numbering and no STREAMINFO, so assume it's not the last (possibly short) frame */
                decoder->private_->frame.header.number.sample_number = (u64)decoder->private_->frame.header.blocksize * (u64)x;
            }
        }

        if(is_unparseable) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }

        return true;
        */
}

pub fn read_subframe(
        decoder:        *mut StreamDecoder,
        channel:        u32,
        bps:            u32,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            u32 x;
        bool wasted_bits;
        unsigned i;

        if(!bitreader_read_raw_uint32(decoder->private_->input, &x, 8)) /* MAGIC NUMBER */
            return false; /* read_callback_ sets the state for us */

        wasted_bits = (x & 1);
        x &= 0xfe;

        if(wasted_bits) {
            unsigned u;
            if(!bitreader_read_unary_unsigned(decoder->private_->input, &u))
                return false; /* read_callback_ sets the state for us */
            decoder->private_->frame.subframes[channel].wasted_bits = u+1;
            bps -= decoder->private_->frame.subframes[channel].wasted_bits;
        }
        else
            decoder->private_->frame.subframes[channel].wasted_bits = 0;

        /*
         * Lots of magic numbers here
         */
        if(x & 0x80) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }
        else if(x == 0) {
            if(!read_subframe_constant_(decoder, channel, bps, do_full_decode))
                return false;
        }
        else if(x == 2) {
            if(!read_subframe_verbatim_(decoder, channel, bps, do_full_decode))
                return false;
        }
        else if(x < 16) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }
        else if(x <= 24) {
            if(!read_subframe_fixed_(decoder, channel, bps, (x>>1)&7, do_full_decode))
                return false;
            if(decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_FRAME_SYNC) /* means bad sync or got corruption */
                return true;
        }
        else if(x < 64) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }
        else {
            if(!read_subframe_lpc_(decoder, channel, bps, ((x>>1)&31)+1, do_full_decode))
                return false;
            if(decoder->protected_->state == STREAM_DECODER_SEARCH_FOR_FRAME_SYNC) /* means bad sync or got corruption */
                return true;
        }

        if(wasted_bits && do_full_decode) {
            x = decoder->private_->frame.subframes[channel].wasted_bits;
            for(i = 0; i < decoder->private_->frame.header.blocksize; i++)
                decoder->private_->output[channel][i] <<= x;
        }

        return true;
        */
}

pub fn read_subframe_constant(
        decoder:        *mut StreamDecoder,
        channel:        u32,
        bps:            u32,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            Subframe_Constant *subframe = &decoder->private_->frame.subframes[channel].data.constant;
        i32 x;
        unsigned i;
        i32 *output = decoder->private_->output[channel];

        decoder->private_->frame.subframes[channel].type = SUBFRAME_TYPE_CONSTANT;

        if(!bitreader_read_raw_int32(decoder->private_->input, &x, bps))
            return false; /* read_callback_ sets the state for us */

        subframe->value = x;

        /* decode the subframe */
        if(do_full_decode) {
            for(i = 0; i < decoder->private_->frame.header.blocksize; i++)
                output[i] = x;
        }

        return true;
        */
}

pub fn read_subframe_fixed(
        decoder:        *mut StreamDecoder,
        channel:        u32,
        bps:            u32,
        order:          u32,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            Subframe_Fixed *subframe = &decoder->private_->frame.subframes[channel].data.fixed;
        i32 i32;
        u32 u32;
        unsigned u;

        decoder->private_->frame.subframes[channel].type = SUBFRAME_TYPE_FIXED;

        subframe->residual = decoder->private_->residual[channel];
        subframe->order = order;

        /* read warm-up samples */
        for(u = 0; u < order; u++) {
            if(!bitreader_read_raw_int32(decoder->private_->input, &i32, bps))
                return false; /* read_callback_ sets the state for us */
            subframe->warmup[u] = i32;
        }

        /* read entropy coding method info */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &u32, ENTROPY_CODING_METHOD_TYPE_LEN))
            return false; /* read_callback_ sets the state for us */
        subframe->entropy_coding_method.type = (EntropyCodingMethodType)u32;
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!bitreader_read_raw_uint32(decoder->private_->input, &u32, ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN))
                    return false; /* read_callback_ sets the state for us */
                subframe->entropy_coding_method.data.partitioned_rice.order = u32;
                subframe->entropy_coding_method.data.partitioned_rice.contents = &decoder->private_->partitioned_rice_contents[channel];
                break;
            default:
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
        }

        /* read residual */
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!read_residual_partitioned_rice_(decoder, order, subframe->entropy_coding_method.data.partitioned_rice.order, &decoder->private_->partitioned_rice_contents[channel], decoder->private_->residual[channel], /*is_extended=*/subframe->entropy_coding_method.type == ENTROPY_CODING_METHOD_PARTITIONED_RICE2))
                    return false;
                break;
            default:
                ASSERT(0);
        }

        /* decode the subframe */
        if(do_full_decode) {
            memcpy(decoder->private_->output[channel], subframe->warmup, sizeof(i32) * order);
            fixed_restore_signal(decoder->private_->residual[channel], decoder->private_->frame.header.blocksize-order, order, decoder->private_->output[channel]+order);
        }

        return true;
        */
}

pub fn read_subframe_lpc(
        decoder:        *mut StreamDecoder,
        channel:        u32,
        bps:            u32,
        order:          u32,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            Subframe_LPC *subframe = &decoder->private_->frame.subframes[channel].data.lpc;
        i32 i32;
        u32 u32;
        unsigned u;

        decoder->private_->frame.subframes[channel].type = SUBFRAME_TYPE_LPC;

        subframe->residual = decoder->private_->residual[channel];
        subframe->order = order;

        /* read warm-up samples */
        for(u = 0; u < order; u++) {
            if(!bitreader_read_raw_int32(decoder->private_->input, &i32, bps))
                return false; /* read_callback_ sets the state for us */
            subframe->warmup[u] = i32;
        }

        /* read qlp coeff precision */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &u32, SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN))
            return false; /* read_callback_ sets the state for us */
        if(u32 == (1u << SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN) - 1) {
            send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
            decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            return true;
        }
        subframe->qlp_coeff_precision = u32+1;

        /* read qlp shift */
        if(!bitreader_read_raw_int32(decoder->private_->input, &i32, SUBFRAME_LPC_QLP_SHIFT_LEN))
            return false; /* read_callback_ sets the state for us */
        subframe->quantization_level = i32;

        /* read quantized lp coefficiencts */
        for(u = 0; u < order; u++) {
            if(!bitreader_read_raw_int32(decoder->private_->input, &i32, subframe->qlp_coeff_precision))
                return false; /* read_callback_ sets the state for us */
            subframe->qlp_coeff[u] = i32;
        }

        /* read entropy coding method info */
        if(!bitreader_read_raw_uint32(decoder->private_->input, &u32, ENTROPY_CODING_METHOD_TYPE_LEN))
            return false; /* read_callback_ sets the state for us */
        subframe->entropy_coding_method.type = (EntropyCodingMethodType)u32;
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!bitreader_read_raw_uint32(decoder->private_->input, &u32, ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN))
                    return false; /* read_callback_ sets the state for us */
                subframe->entropy_coding_method.data.partitioned_rice.order = u32;
                subframe->entropy_coding_method.data.partitioned_rice.contents = &decoder->private_->partitioned_rice_contents[channel];
                break;
            default:
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                return true;
        }

        /* read residual */
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!read_residual_partitioned_rice_(decoder, order, subframe->entropy_coding_method.data.partitioned_rice.order, &decoder->private_->partitioned_rice_contents[channel], decoder->private_->residual[channel], /*is_extended=*/subframe->entropy_coding_method.type == ENTROPY_CODING_METHOD_PARTITIONED_RICE2))
                    return false;
                break;
            default:
                ASSERT(0);
        }

        /* decode the subframe */
        if(do_full_decode) {
            memcpy(decoder->private_->output[channel], subframe->warmup, sizeof(i32) * order);
            /*@@@@@@ technically not pessimistic enough, should be more like
            if( (u64)order * ((((u64)1)<<bps)-1) * ((1<<subframe->qlp_coeff_precision)-1) < (((u64)-1) << 32) )
            */
            if(bps + subframe->qlp_coeff_precision + bitmath_ilog2(order) <= 32)
                if(bps <= 16 && subframe->qlp_coeff_precision <= 16)
                    decoder->private_->local_lpc_restore_signal_16bit(decoder->private_->residual[channel], decoder->private_->frame.header.blocksize-order, subframe->qlp_coeff, order, subframe->quantization_level, decoder->private_->output[channel]+order);
                else
                    decoder->private_->local_lpc_restore_signal(decoder->private_->residual[channel], decoder->private_->frame.header.blocksize-order, subframe->qlp_coeff, order, subframe->quantization_level, decoder->private_->output[channel]+order);
            else
                decoder->private_->local_lpc_restore_signal_64bit(decoder->private_->residual[channel], decoder->private_->frame.header.blocksize-order, subframe->qlp_coeff, order, subframe->quantization_level, decoder->private_->output[channel]+order);
        }

        return true;
        */
}

pub fn read_subframe_verbatim(
        decoder:        *mut StreamDecoder,
        channel:        u32,
        bps:            u32,
        do_full_decode: bool) -> bool {
    
    todo!();
        /*
            Subframe_Verbatim *subframe = &decoder->private_->frame.subframes[channel].data.verbatim;
        i32 x, *residual = decoder->private_->residual[channel];
        unsigned i;

        decoder->private_->frame.subframes[channel].type = SUBFRAME_TYPE_VERBATIM;

        subframe->data = residual;

        for(i = 0; i < decoder->private_->frame.header.blocksize; i++) {
            if(!bitreader_read_raw_int32(decoder->private_->input, &x, bps))
                return false; /* read_callback_ sets the state for us */
            residual[i] = x;
        }

        /* decode the subframe */
        if(do_full_decode)
            memcpy(decoder->private_->output[channel], subframe->data, sizeof(i32) * decoder->private_->frame.header.blocksize);

        return true;
        */
}

pub fn read_residual_partitioned_rice(
        decoder:                   *mut StreamDecoder,
        predictor_order:           u32,
        partition_order:           u32,
        partitioned_rice_contents: *mut EntropyCodingMethod_PartitionedRiceContents,
        residual:                  *mut i32,
        is_extended:               bool) -> bool {
    
    todo!();
        /*
            u32 rice_parameter;
        int i;
        unsigned partition, sample, u;
        const unsigned partitions = 1u << partition_order;
        const unsigned partition_samples = partition_order > 0? decoder->private_->frame.header.blocksize >> partition_order : decoder->private_->frame.header.blocksize - predictor_order;
        const unsigned plen = is_extended? ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN : ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN;
        const unsigned pesc = is_extended? ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER : ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER;

        /* sanity checks */
        if(partition_order == 0) {
            if(decoder->private_->frame.header.blocksize < predictor_order) {
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                /* We have received a potentially malicious bit stream. All we can do is error out to avoid a heap overflow. */
                return false;
            }
        }
        else {
            if(partition_samples < predictor_order) {
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
                /* We have received a potentially malicious bit stream. All we can do is error out to avoid a heap overflow. */
                return false;
            }
        }

        if(!format_entropy_coding_method_partitioned_rice_contents_ensure_size(partitioned_rice_contents, flac_max(6u, partition_order))) {
            decoder->protected_->state = STREAM_DECODER_MEMORY_ALLOCATION_ERROR;
            return false;
        }

        sample = 0;
        for(partition = 0; partition < partitions; partition++) {
            if(!bitreader_read_raw_uint32(decoder->private_->input, &rice_parameter, plen))
                return false; /* read_callback_ sets the state for us */
            partitioned_rice_contents->parameters[partition] = rice_parameter;
            if(rice_parameter < pesc) {
                partitioned_rice_contents->raw_bits[partition] = 0;
                u = (partition_order == 0 || partition > 0)? partition_samples : partition_samples - predictor_order;
                if(!bitreader_read_rice_signed_block(decoder->private_->input, residual + sample, u, rice_parameter))
                    return false; /* read_callback_ sets the state for us */
                sample += u;
            }
            else {
                if(!bitreader_read_raw_uint32(decoder->private_->input, &rice_parameter, ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN))
                    return false; /* read_callback_ sets the state for us */
                partitioned_rice_contents->raw_bits[partition] = rice_parameter;
                for(u = (partition_order == 0 || partition > 0)? 0 : predictor_order; u < partition_samples; u++, sample++) {
                    if(!bitreader_read_raw_int32(decoder->private_->input, &i, rice_parameter))
                        return false; /* read_callback_ sets the state for us */
                    residual[sample] = i;
                }
            }
        }

        return true;
        */
}

pub fn read_zero_padding(decoder: *mut StreamDecoder) -> bool {
    
    todo!();
        /*
            if(!bitreader_is_consumed_byte_aligned(decoder->private_->input)) {
            u32 zero = 0;
            if(!bitreader_read_raw_uint32(decoder->private_->input, &zero, bitreader_bits_left_for_byte_alignment(decoder->private_->input)))
                return false; /* read_callback_ sets the state for us */
            if(zero != 0) {
                send_error_to_client_(decoder, STREAM_DECODER_ERROR_STATUS_LOST_SYNC);
                decoder->protected_->state = STREAM_DECODER_SEARCH_FOR_FRAME_SYNC;
            }
        }
        return true;
        */
}

pub fn read_callback(
        buffer:      &[u8],
        bytes:       *mut usize,
        client_data: *mut c_void) -> bool {
    
    todo!();
        /*
            StreamDecoder *decoder = (StreamDecoder *)client_data;

        if(
    #if HAS_OGG
            /* see [1] HACK NOTE below for why we don't call the eof_callback when decoding Ogg FLAC */
            !decoder->private_->is_ogg &&
    #endif
            decoder->private_->eof_callback && decoder->private_->eof_callback(decoder, decoder->private_->client_data)
        ) {
            *bytes = 0;
            decoder->protected_->state = STREAM_DECODER_END_OF_STREAM;
            return false;
        }
        else if(*bytes > 0) {
            /* While seeking, it is possible for our seek to land in the
             * middle of audio data that looks exactly like a frame header
             * from a future version of an encoder.  When that happens, our
             * error callback will get an
             * STREAM_DECODER_UNPARSEABLE_STREAM and increment its
             * unparseable_frame_count.  But there is a remote possibility
             * that it is properly synced at such a "future-codec frame",
             * so to make sure, we wait to see many "unparseable" errors in
             * a row before bailing out.
             */
            if(decoder->private_->is_seeking && decoder->private_->unparseable_frame_count > 20) {
                decoder->protected_->state = STREAM_DECODER_ABORTED;
                return false;
            }
            else {
                const StreamDecoderReadStatus status =
    #if HAS_OGG
                    decoder->private_->is_ogg?
                    read_callback_ogg_aspect_(decoder, buffer, bytes) :
    #endif
                    decoder->private_->read_callback(decoder, buffer, bytes, decoder->private_->client_data)
                ;
                if(status == STREAM_DECODER_READ_STATUS_ABORT) {
                    decoder->protected_->state = STREAM_DECODER_ABORTED;
                    return false;
                }
                else if(*bytes == 0) {
                    if(
                        status == STREAM_DECODER_READ_STATUS_END_OF_STREAM ||
                        (
    #if HAS_OGG
                            /* see [1] HACK NOTE below for why we don't call the eof_callback when decoding Ogg FLAC */
                            !decoder->private_->is_ogg &&
    #endif
                            decoder->private_->eof_callback && decoder->private_->eof_callback(decoder, decoder->private_->client_data)
                        )
                    ) {
                        decoder->protected_->state = STREAM_DECODER_END_OF_STREAM;
                        return false;
                    }
                    else
                        return true;
                }
                else
                    return true;
            }
        }
        else {
            /* abort to avoid a deadlock */
            decoder->protected_->state = STREAM_DECODER_ABORTED;
            return false;
        }
        /* [1] @@@ HACK NOTE: The end-of-stream checking has to be hacked around
         * for Ogg FLAC.  This is because the ogg decoder aspect can lose sync
         * and at the same time hit the end of the stream (for example, seeking
         * to a point that is after the beginning of the last Ogg page).  There
         * is no way to report an Ogg sync loss through the callbacks (see note
         * in read_callback_ogg_aspect_()) so it returns CONTINUE with *bytes==0.
         * So to keep the decoder from stopping at this point we gate the call
         * to the eof_callback and let the Ogg decoder aspect set the
         * end-of-stream state when it is needed.
         */
        */
}

#[cfg(HAS_OGG)]
pub fn read_callback_ogg_aspect(
        decoder: *const StreamDecoder,
        buffer:  &[u8],
        bytes:   *mut usize) -> StreamDecoderReadStatus {
    
    todo!();
        /*
            switch(ogg_decoder_aspect_read_callback_wrapper(&decoder->protected_->ogg_decoder_aspect, buffer, bytes, read_callback_proxy_, decoder, decoder->private_->client_data)) {
            case OGG_DECODER_ASPECT_READ_STATUS_OK:
                return STREAM_DECODER_READ_STATUS_CONTINUE;
            /* we don't really have a way to handle lost sync via read
             * callback so we'll let it pass and let the underlying
             * FLAC decoder catch the error
             */
            case OGG_DECODER_ASPECT_READ_STATUS_LOST_SYNC:
                return STREAM_DECODER_READ_STATUS_CONTINUE;
            case OGG_DECODER_ASPECT_READ_STATUS_END_OF_STREAM:
                return STREAM_DECODER_READ_STATUS_END_OF_STREAM;
            case OGG_DECODER_ASPECT_READ_STATUS_NOT_FLAC:
            case OGG_DECODER_ASPECT_READ_STATUS_UNSUPPORTED_MAPPING_VERSION:
            case OGG_DECODER_ASPECT_READ_STATUS_ABORT:
            case OGG_DECODER_ASPECT_READ_STATUS_ERROR:
            case OGG_DECODER_ASPECT_READ_STATUS_MEMORY_ALLOCATION_ERROR:
                return STREAM_DECODER_READ_STATUS_ABORT;
            default:
                ASSERT(0);
                /* double protection */
                return STREAM_DECODER_READ_STATUS_ABORT;
        }
        */
}

#[cfg(HAS_OGG)]
pub fn read_callback_proxy(
        void_decoder: *const c_void,
        buffer:       &[u8],
        bytes:        *mut usize,
        client_data:  *mut c_void) -> OggDecoderAspectReadStatus {
    
    todo!();
        /*
            StreamDecoder *decoder = (StreamDecoder*)void_decoder;

        switch(decoder->private_->read_callback(decoder, buffer, bytes, client_data)) {
            case STREAM_DECODER_READ_STATUS_CONTINUE:
                return OGG_DECODER_ASPECT_READ_STATUS_OK;
            case STREAM_DECODER_READ_STATUS_END_OF_STREAM:
                return OGG_DECODER_ASPECT_READ_STATUS_END_OF_STREAM;
            case STREAM_DECODER_READ_STATUS_ABORT:
                return OGG_DECODER_ASPECT_READ_STATUS_ABORT;
            default:
                /* double protection: */
                ASSERT(0);
                return OGG_DECODER_ASPECT_READ_STATUS_ABORT;
        }
        */
}

pub fn write_audio_frame_to_client(
        decoder: *mut StreamDecoder,
        frame:   *const Frame,
        buffer:  *const &[i32]) -> StreamDecoderWriteStatus {
    
    todo!();
        /*
            if(decoder->private_->is_seeking) {
            u64 this_frame_sample = frame->header.number.sample_number;
            u64 next_frame_sample = this_frame_sample + (u64)frame->header.blocksize;
            u64 target_sample = decoder->private_->target_sample;

            ASSERT(frame->header.number_type == FRAME_NUMBER_TYPE_SAMPLE_NUMBER);

    #if HAS_OGG
            decoder->private_->got_a_frame = true;
    #endif
            decoder->private_->last_frame = *frame; /* save the frame */
            if(this_frame_sample <= target_sample && target_sample < next_frame_sample) { /* we hit our target frame */
                unsigned delta = (unsigned)(target_sample - this_frame_sample);
                /* kick out of seek mode */
                decoder->private_->is_seeking = false;
                /* shift out the samples before target_sample */
                if(delta > 0) {
                    unsigned channel;
                    const i32 *newbuffer[MAX_CHANNELS];
                    for(channel = 0; channel < frame->header.channels; channel++)
                        newbuffer[channel] = buffer[channel] + delta;
                    decoder->private_->last_frame.header.blocksize -= delta;
                    decoder->private_->last_frame.header.number.sample_number += (u64)delta;
                    /* write the relevant samples */
                    return decoder->private_->write_callback(decoder, &decoder->private_->last_frame, newbuffer, decoder->private_->client_data);
                }
                else {
                    /* write the relevant samples */
                    return decoder->private_->write_callback(decoder, frame, buffer, decoder->private_->client_data);
                }
            }
            else {
                return STREAM_DECODER_WRITE_STATUS_CONTINUE;
            }
        }
        else {
            /*
             * If we never got STREAMINFO, turn off MD5 checking to save
             * cycles since we don't have a sum to compare to anyway
             */
            if(!decoder->private_->has_stream_info)
                decoder->private_->do_md5_checking = false;
            if(decoder->private_->do_md5_checking) {
                if(!MD5Accumulate(&decoder->private_->md5context, buffer, frame->header.channels, frame->header.blocksize, (frame->header.bits_per_sample+7) / 8))
                    return STREAM_DECODER_WRITE_STATUS_ABORT;
            }
            return decoder->private_->write_callback(decoder, frame, buffer, decoder->private_->client_data);
        }
        */
}

pub fn send_error_to_client(
        decoder: *const StreamDecoder,
        status:  StreamDecoderErrorStatus)  {
    
    todo!();
        /*
            if(!decoder->private_->is_seeking)
            decoder->private_->error_callback(decoder, status, decoder->private_->client_data);
        else if(status == STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM)
            decoder->private_->unparseable_frame_count++;
        */
}

pub fn seek_to_absolute_sample(
        decoder:       *mut StreamDecoder,
        stream_length: u64,
        target_sample: u64) -> bool {
    
    todo!();
        /*
            u64 first_frame_offset = decoder->private_->first_frame_offset, lower_bound, upper_bound, lower_bound_sample, upper_bound_sample, this_frame_sample;
        i64 pos = -1;
        int i;
        unsigned approx_bytes_per_frame;
        bool first_seek = true;
        const u64 total_samples = stream_decoder_get_total_samples(decoder);
        const unsigned min_blocksize = decoder->private_->stream_info.data.stream_info.min_blocksize;
        const unsigned max_blocksize = decoder->private_->stream_info.data.stream_info.max_blocksize;
        const unsigned max_framesize = decoder->private_->stream_info.data.stream_info.max_framesize;
        const unsigned min_framesize = decoder->private_->stream_info.data.stream_info.min_framesize;
        /* take these from the current frame in case they've changed mid-stream */
        unsigned channels = stream_decoder_get_channels(decoder);
        unsigned bps = stream_decoder_get_bits_per_sample(decoder);
        const StreamMetadata_SeekTable *seek_table = decoder->private_->has_seek_table? &decoder->private_->seek_table.data.seek_table : 0;

        /* use values from stream info if we didn't decode a frame */
        if(channels == 0)
            channels = decoder->private_->stream_info.data.stream_info.channels;
        if(bps == 0)
            bps = decoder->private_->stream_info.data.stream_info.bits_per_sample;

        /* we are just guessing here */
        if(max_framesize > 0)
            approx_bytes_per_frame = (max_framesize + min_framesize) / 2 + 1;
        /*
         * Check if it's a known fixed-blocksize stream.  Note that though
         * the spec doesn't allow zeroes in the STREAMINFO block, we may
         * never get a STREAMINFO block when decoding so the value of
         * min_blocksize might be zero.
         */
        else if(min_blocksize == max_blocksize && min_blocksize > 0) {
            /* note there are no () around 'bps/8' to keep precision up since it's an integer calulation */
            approx_bytes_per_frame = min_blocksize * channels * bps/8 + 64;
        }
        else
            approx_bytes_per_frame = 4096 * channels * bps/8 + 64;

        /*
         * First, we set an upper and lower bound on where in the
         * stream we will search.  For now we assume the worst case
         * scenario, which is our best guess at the beginning of
         * the first frame and end of the stream.
         */
        lower_bound = first_frame_offset;
        lower_bound_sample = 0;
        upper_bound = stream_length;
        upper_bound_sample = total_samples > 0 ? total_samples : target_sample /*estimate it*/;

        /*
         * Now we refine the bounds if we have a seektable with
         * suitable points.  Note that according to the spec they
         * must be ordered by ascending sample number.
         *
         * Note: to protect against invalid seek tables we will ignore points
         * that have frame_samples==0 or sample_number>=total_samples
         */
        if(seek_table) {
            u64 new_lower_bound = lower_bound;
            u64 new_upper_bound = upper_bound;
            u64 new_lower_bound_sample = lower_bound_sample;
            u64 new_upper_bound_sample = upper_bound_sample;

            /* find the closest seek point <= target_sample, if it exists */
            for(i = (int)seek_table->num_points - 1; i >= 0; i--) {
                if(
                    seek_table->points[i].sample_number != STREAM_METADATA_SEEKPOINT_PLACEHOLDER &&
                    seek_table->points[i].frame_samples > 0 && /* defense against bad seekpoints */
                    (total_samples <= 0 || seek_table->points[i].sample_number < total_samples) && /* defense against bad seekpoints */
                    seek_table->points[i].sample_number <= target_sample
                )
                    break;
            }
            if(i >= 0) { /* i.e. we found a suitable seek point... */
                new_lower_bound = first_frame_offset + seek_table->points[i].stream_offset;
                new_lower_bound_sample = seek_table->points[i].sample_number;
            }

            /* find the closest seek point > target_sample, if it exists */
            for(i = 0; i < (int)seek_table->num_points; i++) {
                if(
                    seek_table->points[i].sample_number != STREAM_METADATA_SEEKPOINT_PLACEHOLDER &&
                    seek_table->points[i].frame_samples > 0 && /* defense against bad seekpoints */
                    (total_samples <= 0 || seek_table->points[i].sample_number < total_samples) && /* defense against bad seekpoints */
                    seek_table->points[i].sample_number > target_sample
                )
                    break;
            }
            if(i < (int)seek_table->num_points) { /* i.e. we found a suitable seek point... */
                new_upper_bound = first_frame_offset + seek_table->points[i].stream_offset;
                new_upper_bound_sample = seek_table->points[i].sample_number;
            }
            /* final protection against unsorted seek tables; keep original values if bogus */
            if(new_upper_bound >= new_lower_bound) {
                lower_bound = new_lower_bound;
                upper_bound = new_upper_bound;
                lower_bound_sample = new_lower_bound_sample;
                upper_bound_sample = new_upper_bound_sample;
            }
        }

        ASSERT(upper_bound_sample >= lower_bound_sample);
        /* there are 2 insidious ways that the following equality occurs, which
         * we need to fix:
         *  1) total_samples is 0 (unknown) and target_sample is 0
         *  2) total_samples is 0 (unknown) and target_sample happens to be
         *     exactly equal to the last seek point in the seek table; this
         *     means there is no seek point above it, and upper_bound_samples
         *     remains equal to the estimate (of target_samples) we made above
         * in either case it does not hurt to move upper_bound_sample up by 1
         */
        if(upper_bound_sample == lower_bound_sample)
            upper_bound_sample++;

        decoder->private_->target_sample = target_sample;
        while(1) {
            /* check if the bounds are still ok */
            if (lower_bound_sample >= upper_bound_sample || lower_bound > upper_bound) {
                decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                return false;
            }
    #ifndef INTEGER_ONLY_LIBRARY
            pos = (i64)lower_bound + (i64)((double)(target_sample - lower_bound_sample) / (double)(upper_bound_sample - lower_bound_sample) * (double)(upper_bound - lower_bound)) - approx_bytes_per_frame;
    #else
            /* a little less accurate: */
            if(upper_bound - lower_bound < 0xffffffff)
                pos = (i64)lower_bound + (i64)(((target_sample - lower_bound_sample) * (upper_bound - lower_bound)) / (upper_bound_sample - lower_bound_sample)) - approx_bytes_per_frame;
            else /* @@@ WATCHOUT, ~2TB limit */
                pos = (i64)lower_bound + (i64)((((target_sample - lower_bound_sample)>>8) * ((upper_bound - lower_bound)>>8)) / ((upper_bound_sample - lower_bound_sample)>>16)) - approx_bytes_per_frame;
    #endif
            if(pos >= (i64)upper_bound)
                pos = (i64)upper_bound - 1;
            if(pos < (i64)lower_bound)
                pos = (i64)lower_bound;
            if(decoder->private_->seek_callback(decoder, (u64)pos, decoder->private_->client_data) != STREAM_DECODER_SEEK_STATUS_OK) {
                decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                return false;
            }
            if(!stream_decoder_flush(decoder)) {
                /* above call sets the state for us */
                return false;
            }
            /* Now we need to get a frame.  First we need to reset our
             * unparseable_frame_count; if we get too many unparseable
             * frames in a row, the read callback will return
             * STREAM_DECODER_READ_STATUS_ABORT, causing
             * stream_decoder_process_single() to return false.
             */
            decoder->private_->unparseable_frame_count = 0;
            if(!stream_decoder_process_single(decoder)) {
                decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                return false;
            }
            /* our write callback will change the state when it gets to the target frame */
            /* actually, we could have got_a_frame if our decoder is at STREAM_DECODER_END_OF_STREAM so we need to check for that also */
    #if 0
            /*@@@@@@ used to be the following; not clear if the check for end of stream is needed anymore */
            if(decoder->protected_->state != SEEKABLE_STREAM_DECODER_SEEKING && decoder->protected_->state != STREAM_DECODER_END_OF_STREAM)
                break;
    #endif
            if(!decoder->private_->is_seeking)
                break;

            ASSERT(decoder->private_->last_frame.header.number_type == FRAME_NUMBER_TYPE_SAMPLE_NUMBER);
            this_frame_sample = decoder->private_->last_frame.header.number.sample_number;

            if (0 == decoder->private_->samples_decoded || (this_frame_sample + decoder->private_->last_frame.header.blocksize >= upper_bound_sample && !first_seek)) {
                if (pos == (i64)lower_bound) {
                    /* can't move back any more than the first frame, something is fatally wrong */
                    decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                    return false;
                }
                /* our last move backwards wasn't big enough, try again */
                approx_bytes_per_frame = approx_bytes_per_frame? approx_bytes_per_frame * 2 : 16;
                continue;
            }
            /* allow one seek over upper bound, so we can get a correct upper_bound_sample for streams with unknown total_samples */
            first_seek = false;

            /* make sure we are not seeking in corrupted stream */
            if (this_frame_sample < lower_bound_sample) {
                decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                return false;
            }

            /* we need to narrow the search */
            if(target_sample < this_frame_sample) {
                upper_bound_sample = this_frame_sample + decoder->private_->last_frame.header.blocksize;
    /*@@@@@@ what will decode position be if at end of stream? */
                if(!stream_decoder_get_decode_position(decoder, &upper_bound)) {
                    decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                    return false;
                }
                approx_bytes_per_frame = (unsigned)(2 * (upper_bound - pos) / 3 + 16);
            }
            else { /* target_sample >= this_frame_sample + this frame's blocksize */
                lower_bound_sample = this_frame_sample + decoder->private_->last_frame.header.blocksize;
                if(!stream_decoder_get_decode_position(decoder, &lower_bound)) {
                    decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                    return false;
                }
                approx_bytes_per_frame = (unsigned)(2 * (lower_bound - pos) / 3 + 16);
            }
        }

        return true;
        */
}

#[cfg(HAS_OGG)]
pub fn seek_to_absolute_sample_ogg(
        decoder:       *mut StreamDecoder,
        stream_length: u64,
        target_sample: u64) -> bool {
    
    todo!();
        /*
            u64 left_pos = 0, right_pos = stream_length;
        u64 left_sample = 0, right_sample = stream_decoder_get_total_samples(decoder);
        u64 this_frame_sample = (u64)0 - 1;
        u64 pos = 0; /* only initialized to avoid compiler warning */
        bool did_a_seek;
        unsigned iteration = 0;

        /* In the first iterations, we will calculate the target byte position
         * by the distance from the target sample to left_sample and
         * right_sample (let's call it "proportional search").  After that, we
         * will switch to binary search.
         */
        unsigned BINARY_SEARCH_AFTER_ITERATION = 2;

        /* We will switch to a linear search once our current sample is less
         * than this number of samples ahead of the target sample
         */
        static const u64 LINEAR_SEARCH_WITHIN_SAMPLES = MAX_BLOCK_SIZE * 2;

        /* If the total number of samples is unknown, use a large value, and
         * force binary search immediately.
         */
        if(right_sample == 0) {
            right_sample = (u64)(-1);
            BINARY_SEARCH_AFTER_ITERATION = 0;
        }

        decoder->private_->target_sample = target_sample;
        for( ; ; iteration++) {
            if (iteration == 0 || this_frame_sample > target_sample || target_sample - this_frame_sample > LINEAR_SEARCH_WITHIN_SAMPLES) {
                if (iteration >= BINARY_SEARCH_AFTER_ITERATION) {
                    pos = (right_pos + left_pos) / 2;
                }
                else {
    #ifndef INTEGER_ONLY_LIBRARY
                    pos = (u64)((double)(target_sample - left_sample) / (double)(right_sample - left_sample) * (double)(right_pos - left_pos));
    #else
                    /* a little less accurate: */
                    if ((target_sample-left_sample <= 0xffffffff) && (right_pos-left_pos <= 0xffffffff))
                        pos = (i64)(((target_sample-left_sample) * (right_pos-left_pos)) / (right_sample-left_sample));
                    else /* @@@ WATCHOUT, ~2TB limit */
                        pos = (i64)((((target_sample-left_sample)>>8) * ((right_pos-left_pos)>>8)) / ((right_sample-left_sample)>>16));
    #endif
                    /* @@@ TODO: might want to limit pos to some distance
                     * before EOF, to make sure we land before the last frame,
                     * thereby getting a this_frame_sample and so having a better
                     * estimate.
                     */
                }

                /* physical seek */
                if(decoder->private_->seek_callback((StreamDecoder*)decoder, (u64)pos, decoder->private_->client_data) != STREAM_DECODER_SEEK_STATUS_OK) {
                    decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                    return false;
                }
                if(!stream_decoder_flush(decoder)) {
                    /* above call sets the state for us */
                    return false;
                }
                did_a_seek = true;
            }
            else
                did_a_seek = false;

            decoder->private_->got_a_frame = false;
            if(!stream_decoder_process_single(decoder)) {
                decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                return false;
            }
            if(!decoder->private_->got_a_frame) {
                if(did_a_seek) {
                    /* this can happen if we seek to a point after the last frame; we drop
                     * to binary search right away in this case to avoid any wasted
                     * iterations of proportional search.
                     */
                    right_pos = pos;
                    BINARY_SEARCH_AFTER_ITERATION = 0;
                }
                else {
                    /* this can probably only happen if total_samples is unknown and the
                     * target_sample is past the end of the stream
                     */
                    decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                    return false;
                }
            }
            /* our write callback will change the state when it gets to the target frame */
            else if(!decoder->private_->is_seeking) {
                break;
            }
            else {
                this_frame_sample = decoder->private_->last_frame.header.number.sample_number;
                ASSERT(decoder->private_->last_frame.header.number_type == FRAME_NUMBER_TYPE_SAMPLE_NUMBER);

                if (did_a_seek) {
                    if (this_frame_sample <= target_sample) {
                        /* The 'equal' case should not happen, since
                         * stream_decoder_process_single()
                         * should recognize that it has hit the
                         * target sample and we would exit through
                         * the 'break' above.
                         */
                        ASSERT(this_frame_sample != target_sample);

                        left_sample = this_frame_sample;
                        /* sanity check to avoid infinite loop */
                        if (left_pos == pos) {
                            decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                            return false;
                        }
                        left_pos = pos;
                    }
                    else if(this_frame_sample > target_sample) {
                        right_sample = this_frame_sample;
                        /* sanity check to avoid infinite loop */
                        if (right_pos == pos) {
                            decoder->protected_->state = STREAM_DECODER_SEEK_ERROR;
                            return false;
                        }
                        right_pos = pos;
                    }
                }
            }
        }

        return true;
        */
}

#[ignore]
pub fn file_read_callback(
        decoder:     *const StreamDecoder,
        buffer:      &[u8],
        bytes:       *mut usize,
        client_data: *mut c_void) -> StreamDecoderReadStatus {
    
    todo!();
        /*
            (void)client_data;

        if(*bytes > 0) {
            *bytes = fread(buffer, sizeof(byte), *bytes, decoder->private_->file);
            if(ferror(decoder->private_->file))
                return STREAM_DECODER_READ_STATUS_ABORT;
            else if(*bytes == 0)
                return STREAM_DECODER_READ_STATUS_END_OF_STREAM;
            else
                return STREAM_DECODER_READ_STATUS_CONTINUE;
        }
        else
            return STREAM_DECODER_READ_STATUS_ABORT; /* abort to avoid a deadlock */
        */
}

#[ignore]
pub fn file_seek_callback(
        decoder:              *const StreamDecoder,
        absolute_byte_offset: u64,
        client_data:          *mut c_void) -> StreamDecoderSeekStatus {
    
    todo!();
        /*
            (void)client_data;

        if(decoder->private_->file == stdin)
            return STREAM_DECODER_SEEK_STATUS_UNSUPPORTED;
        else if(fseeko(decoder->private_->file, (off_t)absolute_byte_offset, SEEK_SET) < 0)
            return STREAM_DECODER_SEEK_STATUS_ERROR;
        else
            return STREAM_DECODER_SEEK_STATUS_OK;
        */
}

#[ignore]
pub fn file_tell_callback(
        decoder:              *const StreamDecoder,
        absolute_byte_offset: *mut u64,
        client_data:          *mut c_void) -> StreamDecoderTellStatus {
    
    todo!();
        /*
            off_t pos;
        (void)client_data;

        if(decoder->private_->file == stdin)
            return STREAM_DECODER_TELL_STATUS_UNSUPPORTED;
        else if((pos = ftello(decoder->private_->file)) < 0)
            return STREAM_DECODER_TELL_STATUS_ERROR;
        else {
            *absolute_byte_offset = (u64)pos;
            return STREAM_DECODER_TELL_STATUS_OK;
        }
        */
}

#[ignore]
pub fn file_length_callback(
        decoder:       *const StreamDecoder,
        stream_length: *mut u64,
        client_data:   *mut c_void) -> StreamDecoderLengthStatus {
    
    todo!();
        /*
            struct flac_stat_s filestats;
        (void)client_data;

        if(decoder->private_->file == stdin)
            return STREAM_DECODER_LENGTH_STATUS_UNSUPPORTED;
        else if(flac_fstat(fileno(decoder->private_->file), &filestats) != 0)
            return STREAM_DECODER_LENGTH_STATUS_ERROR;
        else {
            *stream_length = (u64)filestats.st_size;
            return STREAM_DECODER_LENGTH_STATUS_OK;
        }
        */
}

#[ignore]
pub fn file_eof_callback(
        decoder:     *const StreamDecoder,
        client_data: *mut c_void) -> bool {
    
    todo!();
        /*
            (void)client_data;

        return feof(decoder->private_->file)? true : false;
        */
}
