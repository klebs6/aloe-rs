crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/stream_encoder_framing.c]

pub fn flac_add_metadata_block(
        metadata: *const StreamMetadata,
        bw:       *mut BitWriter) -> bool {
    
    todo!();
        /*
            unsigned i, j;
        const unsigned vendor_string_length = (unsigned)strlen(VENDOR_STRING);

        if(!bitwriter_write_raw_uint32(bw, metadata->is_last, STREAM_METADATA_IS_LAST_LEN))
            return false;

        if(!bitwriter_write_raw_uint32(bw, metadata->type, STREAM_METADATA_TYPE_LEN))
            return false;

        /*
         * First, for VORBIS_COMMENTs, adjust the length to reflect our vendor string
         */
        i = metadata->length;
        if(metadata->type == METADATA_TYPE_VORBIS_COMMENT) {
            ASSERT(metadata->data.vorbis_comment.vendor_string.length == 0 || 0 != metadata->data.vorbis_comment.vendor_string.entry);
            i -= metadata->data.vorbis_comment.vendor_string.length;
            i += vendor_string_length;
        }
        ASSERT(i < (1u << STREAM_METADATA_LENGTH_LEN));
        if(!bitwriter_write_raw_uint32(bw, i, STREAM_METADATA_LENGTH_LEN))
            return false;

        switch(metadata->type) {
            case METADATA_TYPE_STREAMINFO:
                ASSERT(metadata->data.stream_info.min_blocksize < (1u << STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.min_blocksize, STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN))
                    return false;
                ASSERT(metadata->data.stream_info.max_blocksize < (1u << STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.max_blocksize, STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN))
                    return false;
                ASSERT(metadata->data.stream_info.min_framesize < (1u << STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.min_framesize, STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN))
                    return false;
                ASSERT(metadata->data.stream_info.max_framesize < (1u << STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.max_framesize, STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN))
                    return false;
                ASSERT(format_sample_rate_is_valid(metadata->data.stream_info.sample_rate));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.sample_rate, STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN))
                    return false;
                ASSERT(metadata->data.stream_info.channels > 0);
                ASSERT(metadata->data.stream_info.channels <= (1u << STREAM_METADATA_STREAMINFO_CHANNELS_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.channels-1, STREAM_METADATA_STREAMINFO_CHANNELS_LEN))
                    return false;
                ASSERT(metadata->data.stream_info.bits_per_sample > 0);
                ASSERT(metadata->data.stream_info.bits_per_sample <= (1u << STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN));
                if(!bitwriter_write_raw_uint32(bw, metadata->data.stream_info.bits_per_sample-1, STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN))
                    return false;
                if(!bitwriter_write_raw_uint64(bw, metadata->data.stream_info.total_samples, STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN))
                    return false;
                if(!bitwriter_write_byte_block(bw, metadata->data.stream_info.md5sum, 16))
                    return false;
                break;
            case METADATA_TYPE_PADDING:
                if(!bitwriter_write_zeroes(bw, metadata->length * 8))
                    return false;
                break;
            case METADATA_TYPE_APPLICATION:
                if(!bitwriter_write_byte_block(bw, metadata->data.application.id, STREAM_METADATA_APPLICATION_ID_LEN / 8))
                    return false;
                if(!bitwriter_write_byte_block(bw, metadata->data.application.data, metadata->length - (STREAM_METADATA_APPLICATION_ID_LEN / 8)))
                    return false;
                break;
            case METADATA_TYPE_SEEKTABLE:
                for(i = 0; i < metadata->data.seek_table.num_points; i++) {
                    if(!bitwriter_write_raw_uint64(bw, metadata->data.seek_table.points[i].sample_number, STREAM_METADATA_SEEKPOINT_SAMPLE_NUMBER_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint64(bw, metadata->data.seek_table.points[i].stream_offset, STREAM_METADATA_SEEKPOINT_STREAM_OFFSET_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.seek_table.points[i].frame_samples, STREAM_METADATA_SEEKPOINT_FRAME_SAMPLES_LEN))
                        return false;
                }
                break;
            case METADATA_TYPE_VORBIS_COMMENT:
                if(!bitwriter_write_raw_uint32_little_endian(bw, vendor_string_length))
                    return false;
                if(!bitwriter_write_byte_block(bw, (const byte*)VENDOR_STRING, vendor_string_length))
                    return false;
                if(!bitwriter_write_raw_uint32_little_endian(bw, metadata->data.vorbis_comment.num_comments))
                    return false;
                for(i = 0; i < metadata->data.vorbis_comment.num_comments; i++) {
                    if(!bitwriter_write_raw_uint32_little_endian(bw, metadata->data.vorbis_comment.comments[i].length))
                        return false;
                    if(!bitwriter_write_byte_block(bw, metadata->data.vorbis_comment.comments[i].entry, metadata->data.vorbis_comment.comments[i].length))
                        return false;
                }
                break;
            case METADATA_TYPE_CUESHEET:
                ASSERT(STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN % 8 == 0);
                if(!bitwriter_write_byte_block(bw, (const byte*)metadata->data.cue_sheet.media_catalog_number, STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN/8))
                    return false;
                if(!bitwriter_write_raw_uint64(bw, metadata->data.cue_sheet.lead_in, STREAM_METADATA_CUESHEET_LEAD_IN_LEN))
                    return false;
                if(!bitwriter_write_raw_uint32(bw, metadata->data.cue_sheet.is_cd? 1 : 0, STREAM_METADATA_CUESHEET_IS_CD_LEN))
                    return false;
                if(!bitwriter_write_zeroes(bw, STREAM_METADATA_CUESHEET_RESERVED_LEN))
                    return false;
                if(!bitwriter_write_raw_uint32(bw, metadata->data.cue_sheet.num_tracks, STREAM_METADATA_CUESHEET_NUM_TRACKS_LEN))
                    return false;
                for(i = 0; i < metadata->data.cue_sheet.num_tracks; i++) {
                    const StreamMetadata_CueSheet_Track *track = metadata->data.cue_sheet.tracks + i;

                    if(!bitwriter_write_raw_uint64(bw, track->offset, STREAM_METADATA_CUESHEET_TRACK_OFFSET_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, track->number, STREAM_METADATA_CUESHEET_TRACK_NUMBER_LEN))
                        return false;
                    ASSERT(STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN % 8 == 0);
                    if(!bitwriter_write_byte_block(bw, (const byte*)track->isrc, STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN/8))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, track->type, STREAM_METADATA_CUESHEET_TRACK_TYPE_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, track->pre_emphasis, STREAM_METADATA_CUESHEET_TRACK_PRE_EMPHASIS_LEN))
                        return false;
                    if(!bitwriter_write_zeroes(bw, STREAM_METADATA_CUESHEET_TRACK_RESERVED_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, track->num_indices, STREAM_METADATA_CUESHEET_TRACK_NUM_INDICES_LEN))
                        return false;
                    for(j = 0; j < track->num_indices; j++) {
                        const StreamMetadata_CueSheet_Index *indx = track->indices + j;

                        if(!bitwriter_write_raw_uint64(bw, indx->offset, STREAM_METADATA_CUESHEET_INDEX_OFFSET_LEN))
                            return false;
                        if(!bitwriter_write_raw_uint32(bw, indx->number, STREAM_METADATA_CUESHEET_INDEX_NUMBER_LEN))
                            return false;
                        if(!bitwriter_write_zeroes(bw, STREAM_METADATA_CUESHEET_INDEX_RESERVED_LEN))
                            return false;
                    }
                }
                break;
            case METADATA_TYPE_PICTURE:
                {
                    size_t len;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.type, STREAM_METADATA_PICTURE_TYPE_LEN))
                        return false;
                    len = strlen(metadata->data.picture.mime_type);
                    if(!bitwriter_write_raw_uint32(bw, len, STREAM_METADATA_PICTURE_MIME_TYPE_LENGTH_LEN))
                        return false;
                    if(!bitwriter_write_byte_block(bw, (const byte*)metadata->data.picture.mime_type, len))
                        return false;
                    len = strlen((const char *)metadata->data.picture.description);
                    if(!bitwriter_write_raw_uint32(bw, len, STREAM_METADATA_PICTURE_DESCRIPTION_LENGTH_LEN))
                        return false;
                    if(!bitwriter_write_byte_block(bw, metadata->data.picture.description, len))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.width, STREAM_METADATA_PICTURE_WIDTH_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.height, STREAM_METADATA_PICTURE_HEIGHT_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.depth, STREAM_METADATA_PICTURE_DEPTH_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.colors, STREAM_METADATA_PICTURE_COLORS_LEN))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, metadata->data.picture.data_length, STREAM_METADATA_PICTURE_DATA_LENGTH_LEN))
                        return false;
                    if(!bitwriter_write_byte_block(bw, metadata->data.picture.data, metadata->data.picture.data_length))
                        return false;
                }
                break;
            default:
                if(!bitwriter_write_byte_block(bw, metadata->data.unknown.data, metadata->length))
                    return false;
                break;
        }

        ASSERT(bitwriter_is_byte_aligned(bw));
        return true;
        */
}

pub fn flac_frame_add_header(
        header: *const FrameHeader,
        bw:     *mut BitWriter) -> bool {
    
    todo!();
        /*
            unsigned u, blocksize_hint, sample_rate_hint;
        byte crc;

        ASSERT(bitwriter_is_byte_aligned(bw));

        if(!bitwriter_write_raw_uint32(bw, FRAME_HEADER_SYNC, FRAME_HEADER_SYNC_LEN))
            return false;

        if(!bitwriter_write_raw_uint32(bw, 0, FRAME_HEADER_RESERVED_LEN))
            return false;

        if(!bitwriter_write_raw_uint32(bw, (header->number_type == FRAME_NUMBER_TYPE_FRAME_NUMBER)? 0 : 1, FRAME_HEADER_BLOCKING_STRATEGY_LEN))
            return false;

        ASSERT(header->blocksize > 0 && header->blocksize <= MAX_BLOCK_SIZE);
        /* when this assertion holds true, any legal blocksize can be expressed in the frame header */
        ASSERT(MAX_BLOCK_SIZE <= 65535u);
        blocksize_hint = 0;
        switch(header->blocksize) {
            case   192: u = 1; break;
            case   576: u = 2; break;
            case  1152: u = 3; break;
            case  2304: u = 4; break;
            case  4608: u = 5; break;
            case   256: u = 8; break;
            case   512: u = 9; break;
            case  1024: u = 10; break;
            case  2048: u = 11; break;
            case  4096: u = 12; break;
            case  8192: u = 13; break;
            case 16384: u = 14; break;
            case 32768: u = 15; break;
            default:
                if(header->blocksize <= 0x100)
                    blocksize_hint = u = 6;
                else
                    blocksize_hint = u = 7;
                break;
        }
        if(!bitwriter_write_raw_uint32(bw, u, FRAME_HEADER_BLOCK_SIZE_LEN))
            return false;

        ASSERT(format_sample_rate_is_valid(header->sample_rate));
        sample_rate_hint = 0;
        switch(header->sample_rate) {
            case  88200: u = 1; break;
            case 176400: u = 2; break;
            case 192000: u = 3; break;
            case   8000: u = 4; break;
            case  16000: u = 5; break;
            case  22050: u = 6; break;
            case  24000: u = 7; break;
            case  32000: u = 8; break;
            case  44100: u = 9; break;
            case  48000: u = 10; break;
            case  96000: u = 11; break;
            default:
                if(header->sample_rate <= 255000 && header->sample_rate % 1000 == 0)
                    sample_rate_hint = u = 12;
                else if(header->sample_rate % 10 == 0)
                    sample_rate_hint = u = 14;
                else if(header->sample_rate <= 0xffff)
                    sample_rate_hint = u = 13;
                else
                    u = 0;
                break;
        }
        if(!bitwriter_write_raw_uint32(bw, u, FRAME_HEADER_SAMPLE_RATE_LEN))
            return false;

        ASSERT(header->channels > 0 && header->channels <= (1u << STREAM_METADATA_STREAMINFO_CHANNELS_LEN) && header->channels <= MAX_CHANNELS);
        switch(header->channel_assignment) {
            case CHANNEL_ASSIGNMENT_INDEPENDENT:
                u = header->channels - 1;
                break;
            case CHANNEL_ASSIGNMENT_LEFT_SIDE:
                ASSERT(header->channels == 2);
                u = 8;
                break;
            case CHANNEL_ASSIGNMENT_RIGHT_SIDE:
                ASSERT(header->channels == 2);
                u = 9;
                break;
            case CHANNEL_ASSIGNMENT_MID_SIDE:
                ASSERT(header->channels == 2);
                u = 10;
                break;
            default:
                ASSERT(0);
        }
        if(!bitwriter_write_raw_uint32(bw, u, FRAME_HEADER_CHANNEL_ASSIGNMENT_LEN))
            return false;

        ASSERT(header->bits_per_sample > 0 && header->bits_per_sample <= (1u << STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN));
        switch(header->bits_per_sample) {
            case 8 : u = 1; break;
            case 12: u = 2; break;
            case 16: u = 4; break;
            case 20: u = 5; break;
            case 24: u = 6; break;
            default: u = 0; break;
        }
        if(!bitwriter_write_raw_uint32(bw, u, FRAME_HEADER_BITS_PER_SAMPLE_LEN))
            return false;

        if(!bitwriter_write_raw_uint32(bw, 0, FRAME_HEADER_ZERO_PAD_LEN))
            return false;

        if(header->number_type == FRAME_NUMBER_TYPE_FRAME_NUMBER) {
            if(!bitwriter_write_utf8_uint32(bw, header->number.frame_number))
                return false;
        }
        else {
            if(!bitwriter_write_utf8_uint64(bw, header->number.sample_number))
                return false;
        }

        if(blocksize_hint)
            if(!bitwriter_write_raw_uint32(bw, header->blocksize-1, (blocksize_hint==6)? 8:16))
                return false;

        switch(sample_rate_hint) {
            case 12:
                if(!bitwriter_write_raw_uint32(bw, header->sample_rate / 1000, 8))
                    return false;
                break;
            case 13:
                if(!bitwriter_write_raw_uint32(bw, header->sample_rate, 16))
                    return false;
                break;
            case 14:
                if(!bitwriter_write_raw_uint32(bw, header->sample_rate / 10, 16))
                    return false;
                break;
        }

        /* write the CRC */
        if(!bitwriter_get_write_crc8(bw, &crc))
            return false;
        if(!bitwriter_write_raw_uint32(bw, crc, FRAME_HEADER_CRC_LEN))
            return false;

        return true;
        */
}

pub fn flac_subframe_add_constant(
        subframe:     *const Subframe_Constant,
        subframe_bps: u32,
        wasted_bits:  u32,
        bw:           *mut BitWriter) -> bool {
    
    todo!();
        /*
            bool ok;

        ok =
            bitwriter_write_raw_uint32(bw, SUBFRAME_TYPE_CONSTANT_BYTE_ALIGNED_MASK | (wasted_bits? 1:0), SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN) &&
            (wasted_bits? bitwriter_write_unary_unsigned(bw, wasted_bits-1) : true) &&
            bitwriter_write_raw_int32(bw, subframe->value, subframe_bps)
        ;

        return ok;
        */
}

pub fn flac_subframe_add_fixed(
        subframe:         *const Subframe_Fixed,
        residual_samples: u32,
        subframe_bps:     u32,
        wasted_bits:      u32,
        bw:               *mut BitWriter) -> bool {
    
    todo!();
        /*
            unsigned i;

        if(!bitwriter_write_raw_uint32(bw, SUBFRAME_TYPE_FIXED_BYTE_ALIGNED_MASK | (subframe->order<<1) | (wasted_bits? 1:0), SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN))
            return false;
        if(wasted_bits)
            if(!bitwriter_write_unary_unsigned(bw, wasted_bits-1))
                return false;

        for(i = 0; i < subframe->order; i++)
            if(!bitwriter_write_raw_int32(bw, subframe->warmup[i], subframe_bps))
                return false;

        if(!add_entropy_coding_method_(bw, &subframe->entropy_coding_method))
            return false;
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!add_residual_partitioned_rice_(
                    bw,
                    subframe->residual,
                    residual_samples,
                    subframe->order,
                    subframe->entropy_coding_method.data.partitioned_rice.contents->parameters,
                    subframe->entropy_coding_method.data.partitioned_rice.contents->raw_bits,
                    subframe->entropy_coding_method.data.partitioned_rice.order,
                    /*is_extended=*/subframe->entropy_coding_method.type == ENTROPY_CODING_METHOD_PARTITIONED_RICE2
                ))
                    return false;
                break;
            default:
                ASSERT(0);
        }

        return true;
        */
}

pub fn flac_subframe_add_lpc(
        subframe:         *const Subframe_LPC,
        residual_samples: u32,
        subframe_bps:     u32,
        wasted_bits:      u32,
        bw:               *mut BitWriter) -> bool {
    
    todo!();
        /*
            unsigned i;

        if(!bitwriter_write_raw_uint32(bw, SUBFRAME_TYPE_LPC_BYTE_ALIGNED_MASK | ((subframe->order-1)<<1) | (wasted_bits? 1:0), SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN))
            return false;
        if(wasted_bits)
            if(!bitwriter_write_unary_unsigned(bw, wasted_bits-1))
                return false;

        for(i = 0; i < subframe->order; i++)
            if(!bitwriter_write_raw_int32(bw, subframe->warmup[i], subframe_bps))
                return false;

        if(!bitwriter_write_raw_uint32(bw, subframe->qlp_coeff_precision-1, SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN))
            return false;
        if(!bitwriter_write_raw_int32(bw, subframe->quantization_level, SUBFRAME_LPC_QLP_SHIFT_LEN))
            return false;
        for(i = 0; i < subframe->order; i++)
            if(!bitwriter_write_raw_int32(bw, subframe->qlp_coeff[i], subframe->qlp_coeff_precision))
                return false;

        if(!add_entropy_coding_method_(bw, &subframe->entropy_coding_method))
            return false;
        switch(subframe->entropy_coding_method.type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!add_residual_partitioned_rice_(
                    bw,
                    subframe->residual,
                    residual_samples,
                    subframe->order,
                    subframe->entropy_coding_method.data.partitioned_rice.contents->parameters,
                    subframe->entropy_coding_method.data.partitioned_rice.contents->raw_bits,
                    subframe->entropy_coding_method.data.partitioned_rice.order,
                    /*is_extended=*/subframe->entropy_coding_method.type == ENTROPY_CODING_METHOD_PARTITIONED_RICE2
                ))
                    return false;
                break;
            default:
                ASSERT(0);
        }

        return true;
        */
}

pub fn flac_subframe_add_verbatim(
        subframe:     *const Subframe_Verbatim,
        samples:      u32,
        subframe_bps: u32,
        wasted_bits:  u32,
        bw:           *mut BitWriter) -> bool {
    
    todo!();
        /*
            unsigned i;
        const i32 *signal = subframe->data;

        if(!bitwriter_write_raw_uint32(bw, SUBFRAME_TYPE_VERBATIM_BYTE_ALIGNED_MASK | (wasted_bits? 1:0), SUBFRAME_ZERO_PAD_LEN + SUBFRAME_TYPE_LEN + SUBFRAME_WASTED_BITS_FLAG_LEN))
            return false;
        if(wasted_bits)
            if(!bitwriter_write_unary_unsigned(bw, wasted_bits-1))
                return false;

        for(i = 0; i < samples; i++)
            if(!bitwriter_write_raw_int32(bw, signal[i], subframe_bps))
                return false;

        return true;
        */
}

pub fn add_entropy_coding_method(
        bw:     *mut BitWriter,
        method: *const EntropyCodingMethod) -> bool {
    
    todo!();
        /*
            if(!bitwriter_write_raw_uint32(bw, method->type, ENTROPY_CODING_METHOD_TYPE_LEN))
            return false;
        switch(method->type) {
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE:
            case ENTROPY_CODING_METHOD_PARTITIONED_RICE2:
                if(!bitwriter_write_raw_uint32(bw, method->data.partitioned_rice.order, ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN))
                    return false;
                break;
            default:
                ASSERT(0);
        }
        return true;
        */
}

pub fn add_residual_partitioned_rice(
        bw:               *mut BitWriter,
        residual:         &[i32],
        residual_samples: u32,
        predictor_order:  u32,
        rice_parameters:  &[u32],
        raw_bits:         &[u32],
        partition_order:  u32,
        is_extended:      bool) -> bool {
    
    todo!();
        /*
            const unsigned plen = is_extended? ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN : ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN;
        const unsigned pesc = is_extended? ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER : ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER;

        if(partition_order == 0) {
            unsigned i;

            if(raw_bits[0] == 0) {
                if(!bitwriter_write_raw_uint32(bw, rice_parameters[0], plen))
                    return false;
                if(!bitwriter_write_rice_signed_block(bw, residual, residual_samples, rice_parameters[0]))
                    return false;
            }
            else {
                ASSERT(rice_parameters[0] == 0);
                if(!bitwriter_write_raw_uint32(bw, pesc, plen))
                    return false;
                if(!bitwriter_write_raw_uint32(bw, raw_bits[0], ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN))
                    return false;
                for(i = 0; i < residual_samples; i++) {
                    if(!bitwriter_write_raw_int32(bw, residual[i], raw_bits[0]))
                        return false;
                }
            }
            return true;
        }
        else {
            unsigned i, j, k = 0, k_last = 0;
            unsigned partition_samples;
            const unsigned default_partition_samples = (residual_samples+predictor_order) >> partition_order;
            for(i = 0; i < (1u<<partition_order); i++) {
                partition_samples = default_partition_samples;
                if(i == 0)
                    partition_samples -= predictor_order;
                k += partition_samples;
                if(raw_bits[i] == 0) {
                    if(!bitwriter_write_raw_uint32(bw, rice_parameters[i], plen))
                        return false;
                    if(!bitwriter_write_rice_signed_block(bw, residual+k_last, k-k_last, rice_parameters[i]))
                        return false;
                }
                else {
                    if(!bitwriter_write_raw_uint32(bw, pesc, plen))
                        return false;
                    if(!bitwriter_write_raw_uint32(bw, raw_bits[i], ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN))
                        return false;
                    for(j = k_last; j < k; j++) {
                        if(!bitwriter_write_raw_int32(bw, residual[j], raw_bits[i]))
                            return false;
                    }
                }
                k_last = k;
            }
            return true;
        }
        */
}
