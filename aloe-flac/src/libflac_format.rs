crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/format.c]

/**
  | In the C code, VERSION should come from configure
  |
  */
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const version_string: &'static str = VERSION;

pub fn vendor_string() -> String { format!("reference libFLAC {} 20141125", VERSION) }

pub const stream_sync_string: [char; 4] = [ 'f','L','a','C' ];
pub const stream_sync:                                              u32 = 0x664C6143;
pub const stream_sync_len:                                          u32 = 32; // bits 

pub const stream_metadata_streaminfo_min_block_size_len:            u32 = 16; // bits
pub const stream_metadata_streaminfo_max_block_size_len:            u32 = 16; // bits
pub const stream_metadata_streaminfo_min_frame_size_len:            u32 = 24; // bits
pub const stream_metadata_streaminfo_max_frame_size_len:            u32 = 24; // bits
pub const stream_metadata_streaminfo_sample_rate_len:               u32 = 20; // bits
pub const stream_metadata_streaminfo_channels_len:                  u32 = 3; // bits
pub const stream_metadata_streaminfo_bits_per_sample_len:           u32 = 5; // bits
pub const stream_metadata_streaminfo_total_samples_len:             u32 = 36; // bits
pub const stream_metadata_streaminfo_md5sum_len:                    u32 = 128; // bits
pub const stream_metadata_application_id_len:                       u32 = 32; // bits
pub const stream_metadata_seekpoint_sample_number_len:              u32 = 64; // bits
pub const stream_metadata_seekpoint_stream_offset_len:              u32 = 64; // bits
pub const stream_metadata_seekpoint_frame_samples_len:              u32 = 16; // bits

pub const stream_metadata_seekpoint_placeholder: u64 =  0xffffffffffffffff;

pub const stream_metadata_vorbis_comment_entry_length_len:          u32 = 32; // bits
pub const stream_metadata_vorbis_comment_num_comments_len:          u32 = 32; // bits
pub const stream_metadata_cuesheet_index_offset_len:                u32 = 64; // bits
pub const stream_metadata_cuesheet_index_number_len:                u32 = 8; // bits
pub const stream_metadata_cuesheet_index_reserved_len:              u32 = 3*8; // bits
pub const stream_metadata_cuesheet_track_offset_len:                u32 = 64; // bits
pub const stream_metadata_cuesheet_track_number_len:                u32 = 8; // bits
pub const stream_metadata_cuesheet_track_isrc_len:                  u32 = 12*8; // bits
pub const stream_metadata_cuesheet_track_type_len:                  u32 = 1; // bit
pub const stream_metadata_cuesheet_track_pre_emphasis_len:          u32 = 1; // bit
pub const stream_metadata_cuesheet_track_reserved_len:              u32 = 6+13*8; // bits
pub const stream_metadata_cuesheet_track_num_indices_len:           u32 = 8; // bits
pub const stream_metadata_cuesheet_media_catalog_number_len:        u32 = 128*8; // bits
pub const stream_metadata_cuesheet_lead_in_len:                     u32 = 64; // bits
pub const stream_metadata_cuesheet_is_cd_len:                       u32 = 1; // bit
pub const stream_metadata_cuesheet_reserved_len:                    u32 = 7+258*8; // bits
pub const stream_metadata_cuesheet_num_tracks_len:                  u32 = 8; // bits
pub const stream_metadata_picture_type_len:                         u32 = 32; // bits
pub const stream_metadata_picture_mime_type_length_len:             u32 = 32; // bits
pub const stream_metadata_picture_description_length_len:           u32 = 32; // bits
pub const stream_metadata_picture_width_len:                        u32 = 32; // bits
pub const stream_metadata_picture_height_len:                       u32 = 32; // bits
pub const stream_metadata_picture_depth_len:                        u32 = 32; // bits
pub const stream_metadata_picture_colors_len:                       u32 = 32; // bits
pub const stream_metadata_picture_data_length_len:                  u32 = 32; // bits
pub const stream_metadata_is_last_len:                              u32 = 1; // bits
pub const stream_metadata_type_len:                                 u32 = 7; // bits
pub const stream_metadata_length_len:                               u32 = 24; // bits
pub const frame_header_sync:                                        u32 = 0x3ffe;
pub const frame_header_sync_len:                                    u32 = 14; // bits
pub const frame_header_reserved_len:                                u32 = 1; // bits
pub const frame_header_blocking_strategy_len:                       u32 = 1; // bits
pub const frame_header_block_size_len:                              u32 = 4; // bits
pub const frame_header_sample_rate_len:                             u32 = 4; // bits
pub const frame_header_channel_assignment_len:                      u32 = 4; // bits
pub const frame_header_bits_per_sample_len:                         u32 = 3; // bits
pub const frame_header_zero_pad_len:                                u32 = 1; // bits
pub const frame_header_crc_len:                                     u32 = 8; // bits
pub const frame_footer_crc_len:                                     u32 = 16; // bits
pub const entropy_coding_method_type_len:                           u32 = 2; // bits
pub const entropy_coding_method_partitioned_rice_order_len:         u32 = 4; // bits
pub const entropy_coding_method_partitioned_rice_parameter_len:     u32 = 4; // bits
pub const entropy_coding_method_partitioned_rice2_parameter_len:    u32 = 5; // bits
pub const entropy_coding_method_partitioned_rice_raw_len:           u32 = 5; // bits
pub const entropy_coding_method_partitioned_rice_escape_parameter:  u32 = 15; // == (1<<ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN)-1
pub const entropy_coding_method_partitioned_rice2_escape_parameter: u32 = 31; // == (1<<ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN)-1

pub const entropy_coding_method_type_string: &[&'static str] = &[ 
    "PARTITIONED_RICE", 
    "PARTITIONED_RICE2" 
];

pub const subframe_lpc_qlp_coeff_precision_len:     u32 = 4; // bits
pub const subframe_lpc_qlp_shift_len:               u32 = 5; // bits
pub const subframe_zero_pad_len:                    u32 = 1; // bits
pub const subframe_type_len:                        u32 = 6; // bits
pub const subframe_wasted_bits_flag_len:            u32 = 1; // bits
pub const subframe_type_constant_byte_aligned_mask: u32 = 0x00;
pub const subframe_type_verbatim_byte_aligned_mask: u32 = 0x02;
pub const subframe_type_fixed_byte_aligned_mask:    u32 = 0x10;
pub const subframe_type_lpc_byte_aligned_mask:      u32 = 0x40;

pub const subframe_type_string:      &[&'static str] =  &[ 
    "CONSTANT", 
    "VERBATIM", 
    "FIXED", 
    "LPC" 
];

pub const channel_assignment_string: &[&'static str] =  &[ 
    "INDEPENDENT", 
    "LEFT_SIDE", 
    "RIGHT_SIDE", 
    "MID_SIDE" 
];

pub const frame_number_type_string:  &[&'static str] =  &[ 
    "FRAME_NUMBER_TYPE_FRAME_NUMBER", 
    "FRAME_NUMBER_TYPE_SAMPLE_NUMBER" 
];

pub const metadata_type_string: &[&'static str] =  &[ 
    "STREAMINFO", 
    "PADDING", 
    "APPLICATION", 
    "SEEKTABLE", 
    "VORBIS_COMMENT", 
    "CUESHEET", 
    "PICTURE" 
];

pub const stream_metadata_picture_type_string: &[&'static str] =  &[ 
    "Other", 
    "32x32 pixels 'file icon' (PNG only)", 
    "Other file icon", 
    "Cover (front)", 
    "Cover (back)", 
    "Leaflet page", 
    "Media (e.g. label side of CD)", 
    "Lead artist/lead performer/soloist", 
    "Artist/performer", 
    "Conductor", 
    "Band/Orchestra", 
    "Composer", 
    "Lyricist/text writer", 
    "Recording Location", 
    "During recording", 
    "During performance", 
    "Movie/video screen capture", 
    "A bright coloured fish", 
    "Illustration", 
    "Band/artist logotype", 
    "Publisher/Studio logotype" 
];

pub fn format_sample_rate_is_valid(sample_rate: u32) -> bool {
    
    todo!();
        /*
            if(sample_rate == 0 || sample_rate > MAX_SAMPLE_RATE) {
            return false;
        }
        else
            return true;
        */
}


pub fn format_blocksize_is_subset(
        blocksize:   u32,
        sample_rate: u32) -> bool {
    
    todo!();
        /*
            if(blocksize > 16384)
            return false;
        else if(sample_rate <= 48000 && blocksize > 4608)
            return false;
        else
            return true;
        */
}

pub fn format_sample_rate_is_subset(sample_rate: u32) -> bool {
    
    todo!();
        /*
            if(
            !format_sample_rate_is_valid(sample_rate) ||
            (
                sample_rate >= (1u << 16) &&
                !(sample_rate % 1000 == 0 || sample_rate % 10 == 0)
            )
        ) {
            return false;
        }
        else
            return true;
        */
}

/**
  | @@@@ add to unit tests; it is already
  | indirectly tested by the metadata_object
  | tests
  |
  */
pub fn format_seektable_is_legal(seek_table: *const StreamMetadata_SeekTable) -> bool {
    
    todo!();
        /*
            unsigned i;
        u64 prev_sample_number = 0;
        bool got_prev = false;

        ASSERT(0 != seek_table);

        for(i = 0; i < seek_table->num_points; i++) {
            if(got_prev) {
                if(
                    seek_table->points[i].sample_number != STREAM_METADATA_SEEKPOINT_PLACEHOLDER &&
                    seek_table->points[i].sample_number <= prev_sample_number
                )
                    return false;
            }
            prev_sample_number = seek_table->points[i].sample_number;
            got_prev = true;
        }

        return true;
        */
}

/**
  | used as the sort predicate for qsort()
  |
  */
pub fn seekpoint_compare(
        l: *const StreamMetadata_SeekPoint,
        r: *const StreamMetadata_SeekPoint) -> i32 {
    
    todo!();
        /*
            /* we don't just 'return l->sample_number - r->sample_number' since the result (i64) might overflow an 'int' */
        if(l->sample_number == r->sample_number)
            return 0;
        else if(l->sample_number < r->sample_number)
            return -1;
        else
            return 1;
        */
}

/**
  | @@@@ add to unit tests; it is already
  | indirectly tested by the metadata_object
  | tests
  |
  */
pub fn format_seektable_sort(seek_table: *mut StreamMetadata_SeekTable) -> u32 {
    
    todo!();
        /*
            unsigned i, j;
        bool first;

        ASSERT(0 != seek_table);

        /* sort the seekpoints */
        qsort(seek_table->points, seek_table->num_points, sizeof(StreamMetadata_SeekPoint), (int (*)(const void *, const void *))seekpoint_compare_);

        /* uniquify the seekpoints */
        first = true;
        for(i = j = 0; i < seek_table->num_points; i++) {
            if(seek_table->points[i].sample_number != STREAM_METADATA_SEEKPOINT_PLACEHOLDER) {
                if(!first) {
                    if(seek_table->points[i].sample_number == seek_table->points[j-1].sample_number)
                        continue;
                }
            }
            first = false;
            seek_table->points[j++] = seek_table->points[i];
        }

        for(i = j; i < seek_table->num_points; i++) {
            seek_table->points[i].sample_number = STREAM_METADATA_SEEKPOINT_PLACEHOLDER;
            seek_table->points[i].stream_offset = 0;
            seek_table->points[i].frame_samples = 0;
        }

        return j;
        */
}

/**
 | also disallows non-shortest-form encodings,
 | c.f.
 |
 |   http://www.unicode.org/versions/corrigendum1.html
 |
 | and a more clear explanation at the end of this
 | section:
 |
 |   http://www.cl.cam.ac.uk/~mgk25/unicode.html#utf-8
 */
pub fn utf8len(utf8: *const u8) -> u32 {
    
    todo!();
        /*
            ASSERT(0 != utf8);
        if ((utf8[0] & 0x80) == 0) {
            return 1;
        }
        else if ((utf8[0] & 0xE0) == 0xC0 && (utf8[1] & 0xC0) == 0x80) {
            if ((utf8[0] & 0xFE) == 0xC0) /* overlong sequence check */
                return 0;
            return 2;
        }
        else if ((utf8[0] & 0xF0) == 0xE0 && (utf8[1] & 0xC0) == 0x80 && (utf8[2] & 0xC0) == 0x80) {
            if (utf8[0] == 0xE0 && (utf8[1] & 0xE0) == 0x80) /* overlong sequence check */
                return 0;
            /* illegal surrogates check (U+D800...U+DFFF and U+FFFE...U+FFFF) */
            if (utf8[0] == 0xED && (utf8[1] & 0xE0) == 0xA0) /* D800-DFFF */
                return 0;
            if (utf8[0] == 0xEF && utf8[1] == 0xBF && (utf8[2] & 0xFE) == 0xBE) /* FFFE-FFFF */
                return 0;
            return 3;
        }
        else if ((utf8[0] & 0xF8) == 0xF0 && (utf8[1] & 0xC0) == 0x80 && (utf8[2] & 0xC0) == 0x80 && (utf8[3] & 0xC0) == 0x80) {
            if (utf8[0] == 0xF0 && (utf8[1] & 0xF0) == 0x80) /* overlong sequence check */
                return 0;
            return 4;
        }
        else if ((utf8[0] & 0xFC) == 0xF8 && (utf8[1] & 0xC0) == 0x80 && (utf8[2] & 0xC0) == 0x80 && (utf8[3] & 0xC0) == 0x80 && (utf8[4] & 0xC0) == 0x80) {
            if (utf8[0] == 0xF8 && (utf8[1] & 0xF8) == 0x80) /* overlong sequence check */
                return 0;
            return 5;
        }
        else if ((utf8[0] & 0xFE) == 0xFC && (utf8[1] & 0xC0) == 0x80 && (utf8[2] & 0xC0) == 0x80 && (utf8[3] & 0xC0) == 0x80 && (utf8[4] & 0xC0) == 0x80 && (utf8[5] & 0xC0) == 0x80) {
            if (utf8[0] == 0xFC && (utf8[1] & 0xFC) == 0x80) /* overlong sequence check */
                return 0;
            return 6;
        }
        else {
            return 0;
        }
        */
}

pub fn format_vorbiscomment_entry_name_is_legal(name: *const u8) -> bool {
    
    todo!();
        /*
            char c;
        for(c = *name; c; c = *(++name))
            if(c < 0x20 || c == 0x3d || c > 0x7d)
                return false;
        return true;
        */
}

pub fn format_vorbiscomment_entry_value_is_legal(
        value:  *const u8,
        length: u32) -> bool {
    
    todo!();
        /*
            if(length == (unsigned)(-1)) {
            while(*value) {
                unsigned n = utf8len_(value);
                if(n == 0)
                    return false;
                value += n;
            }
        }
        else {
            const byte *end = value + length;
            while(value < end) {
                unsigned n = utf8len_(value);
                if(n == 0)
                    return false;
                value += n;
            }
            if(value != end)
                return false;
        }
        return true;
        */
}

pub fn format_vorbiscomment_entry_is_legal(
        entry:  *const u8,
        length: u32) -> bool {
    
    todo!();
        /*
            const byte *s, *end;

        for(s = entry, end = s + length; s < end && *s != '='; s++) {
            if(*s < 0x20 || *s > 0x7D)
                return false;
        }
        if(s == end)
            return false;

        s++; /* skip '=' */

        while(s < end) {
            unsigned n = utf8len_(s);
            if(n == 0)
                return false;
            s += n;
        }
        if(s != end)
            return false;

        return true;
        */
}

/**
  | @@@@ add to unit tests; it is already
  | indirectly tested by the metadata_object
  | tests
  |
  */
pub fn format_cuesheet_is_legal(
        cue_sheet:          *const StreamMetadata_CueSheet,
        check_cd_da_subset: bool,
        violation:          *const *const u8) -> bool {
    
    todo!();
        /*
            unsigned i, j;

        if(check_cd_da_subset) {
            if(cue_sheet->lead_in < 2 * 44100) {
                if(violation) *violation = "CD-DA cue sheet must have a lead-in length of at least 2 seconds";
                return false;
            }
            if(cue_sheet->lead_in % 588 != 0) {
                if(violation) *violation = "CD-DA cue sheet lead-in length must be evenly divisible by 588 samples";
                return false;
            }
        }

        if(cue_sheet->num_tracks == 0) {
            if(violation) *violation = "cue sheet must have at least one track (the lead-out)";
            return false;
        }

        if(check_cd_da_subset && cue_sheet->tracks[cue_sheet->num_tracks-1].number != 170) {
            if(violation) *violation = "CD-DA cue sheet must have a lead-out track number 170 (0xAA)";
            return false;
        }

        for(i = 0; i < cue_sheet->num_tracks; i++) {
            if(cue_sheet->tracks[i].number == 0) {
                if(violation) *violation = "cue sheet may not have a track number 0";
                return false;
            }

            if(check_cd_da_subset) {
                if(!((cue_sheet->tracks[i].number >= 1 && cue_sheet->tracks[i].number <= 99) || cue_sheet->tracks[i].number == 170)) {
                    if(violation) *violation = "CD-DA cue sheet track number must be 1-99 or 170";
                    return false;
                }
            }

            if(check_cd_da_subset && cue_sheet->tracks[i].offset % 588 != 0) {
                if(violation) {
                    if(i == cue_sheet->num_tracks-1) /* the lead-out track... */
                        *violation = "CD-DA cue sheet lead-out offset must be evenly divisible by 588 samples";
                    else
                        *violation = "CD-DA cue sheet track offset must be evenly divisible by 588 samples";
                }
                return false;
            }

            if(i < cue_sheet->num_tracks - 1) {
                if(cue_sheet->tracks[i].num_indices == 0) {
                    if(violation) *violation = "cue sheet track must have at least one index point";
                    return false;
                }

                if(cue_sheet->tracks[i].indices[0].number > 1) {
                    if(violation) *violation = "cue sheet track's first index number must be 0 or 1";
                    return false;
                }
            }

            for(j = 0; j < cue_sheet->tracks[i].num_indices; j++) {
                if(check_cd_da_subset && cue_sheet->tracks[i].indices[j].offset % 588 != 0) {
                    if(violation) *violation = "CD-DA cue sheet track index offset must be evenly divisible by 588 samples";
                    return false;
                }

                if(j > 0) {
                    if(cue_sheet->tracks[i].indices[j].number != cue_sheet->tracks[i].indices[j-1].number + 1) {
                        if(violation) *violation = "cue sheet track index numbers must increase by 1";
                        return false;
                    }
                }
            }
        }

        return true;
        */
}

/**
  | @@@@ add to unit tests; it is already
  | indirectly tested by the metadata_object
  | tests
  |
  */
pub fn format_picture_is_legal(
        picture:   *const StreamMetadata_Picture,
        violation: *const *const u8) -> bool {
    
    todo!();
        /*
            char *p;
        byte *b;

        for(p = picture->mime_type; *p; p++) {
            if(*p < 0x20 || *p > 0x7e) {
                if(violation) *violation = "MIME type string must contain only printable ASCII characters (0x20-0x7e)";
                return false;
            }
        }

        for(b = picture->description; *b; ) {
            unsigned n = utf8len_(b);
            if(n == 0) {
                if(violation) *violation = "description string must be valid UTF-8";
                return false;
            }
            b += n;
        }

        return true;
        */
}

/**
  | These routines are private to libFLAC
  |
  */
pub fn format_get_max_rice_partition_order(
        blocksize:       u32,
        predictor_order: u32) -> u32 {
    
    todo!();
        /*
            return
            format_get_max_rice_partition_order_from_blocksize_limited_max_and_predictor_order(
                format_get_max_rice_partition_order_from_blocksize(blocksize),
                blocksize,
                predictor_order
            );
        */
}

pub fn format_get_max_rice_partition_order_from_blocksize(blocksize: u32) -> u32 {
    
    todo!();
        /*
            unsigned max_rice_partition_order = 0;
        while(!(blocksize & 1)) {
            max_rice_partition_order++;
            blocksize >>= 1;
        }
        return min(MAX_RICE_PARTITION_ORDER, max_rice_partition_order);
        */
}

pub fn format_get_max_rice_partition_order_from_blocksize_limited_max_and_predictor_order(
        limit:           u32,
        blocksize:       u32,
        predictor_order: u32) -> u32 {
    
    todo!();
        /*
            unsigned max_rice_partition_order = limit;

        while(max_rice_partition_order > 0 && (blocksize >> max_rice_partition_order) <= predictor_order)
            max_rice_partition_order--;

        ASSERT(
            (max_rice_partition_order == 0 && blocksize >= predictor_order) ||
            (max_rice_partition_order > 0 && blocksize >> max_rice_partition_order > predictor_order)
        );

        return max_rice_partition_order;
        */
}

pub fn format_entropy_coding_method_partitioned_rice_contents_init(object: *mut EntropyCodingMethod_PartitionedRiceContents)  {
    
    todo!();
        /*
            ASSERT(0 != object);

        object->parameters = 0;
        object->raw_bits = 0;
        object->capacity_by_order = 0;
        */
}

pub fn format_entropy_coding_method_partitioned_rice_contents_clear(object: *mut EntropyCodingMethod_PartitionedRiceContents)  {
    
    todo!();
        /*
            ASSERT(0 != object);

        if(0 != object->parameters)
            free(object->parameters);
        if(0 != object->raw_bits)
            free(object->raw_bits);
        format_entropy_coding_method_partitioned_rice_contents_init(object);
        */
}

pub fn format_entropy_coding_method_partitioned_rice_contents_ensure_size(
        object:              *mut EntropyCodingMethod_PartitionedRiceContents,
        max_partition_order: u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != object);

        ASSERT(object->capacity_by_order > 0 || (0 == object->parameters && 0 == object->raw_bits));

        if(object->capacity_by_order < max_partition_order) {
            if(0 == (object->parameters = (unsigned int*) realloc(object->parameters, sizeof(unsigned)*(1 << max_partition_order))))
                return false;
            if(0 == (object->raw_bits = (unsigned int*) realloc(object->raw_bits, sizeof(unsigned)*(1 << max_partition_order))))
                return false;
            memset(object->raw_bits, 0, sizeof(unsigned)*(1 << max_partition_order));
            object->capacity_by_order = max_partition_order;
        }

        return true;
        */
}
