crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/bitreader.c]

/**
  | Things should be fastest when this matches
  | the machine word size
  | 
  | -----------
  | @warning
  | 
  | if you change this you must also change
  | the following #defines down to clz_uint32
  | below to match
  | ----------
  | @warning
  | 
  | there are a few places where the code
  | will not work unless uint32_t is >= 32
  | bits wide also, some sections currently
  | only have fast versions for 4 or 8 bytes
  | per word
  |
  */
pub const BYTES_PER_WORD: u32 = 4; /* sizeof uint32_t */
pub const BITS_PER_WORD:  u32 = 8 * BYTES_PER_WORD;
pub const WORD_ALL_ONES:  u32 = 0xffffffff;

/**
  | SWAP_BE_WORD_TO_HOST swaps bytes
  | in a uint32_t (which is always big-endian)
  | if necessary to match host byte order
  |
  */
#[cfg(WORDS_BIGENDIAN)]
macro_rules! swap_be_word_to_host {
    ($x:ident) => {
        /*
                (x)
        */
    }
}

#[cfg(not(WORDS_BIGENDIAN))]
macro_rules! swap_be_word_to_host {
    ($x:ident) => {
        /*
                ENDSWAP_32(x)
        */
    }
}

/**
  | This should be at least twice as large
  | as the largest number of words required
  | to represent any 'number' (in any encoding)
  | you are going to read. With FLAC this
  | is on the order of maybe a few hundred
  | bits.
  | 
  | If the buffer is smaller than that, the
  | decoder won't be able to read in a whole
  | number that is in a variable length encoding
  | (e.g. Rice).
  | 
  | But to be practical it should be at least
  | 1K bytes.
  | 
  | Increase this number to decrease the
  | number of read callbacks, at the expense
  | of using more memory. Or decrease for
  | the reverse effect, keeping in mind
  | the limit from the first paragraph.
  | The optimal size also depends on the
  | CPU cache size and other factors; some
  | twiddling may be necessary to squeeze
  | out the best performance.
  |
  */
pub const flac_bitreader_default_capacity: u32 = 65536 / BITS_PER_WORD; /* in words */

pub struct BitReader {

    /**
      | any partially-consumed word at the
      | head will stay right-justified as bits
      | are consumed from the left
      | 
      | any incomplete word at the tail will
      | be left-justified, and bytes from the
      | read callback are added on the right
      |
      */
    buffer:         u32,

    /**
      | in words
      |
      */
    capacity:       u32,


    /**
      | # of completed words in buffer
      |
      */
    words:          u32,


    /**
      | # of bytes in incomplete word at buffer[words]
      |
      */
    bytes:          u32,


    /**
      | #words ...
      |
      */
    consumed_words: u32,


    /**
      | ... + (#bits of head word) already consumed
      | from the front of buffer
      |
      */
    consumed_bits:  u32,


    /**
      | the running frame CRC
      |
      */
    read_crc16:     u32,


    /**
      | the number of bits in the current consumed
      | word that should not be CRC'd
      |
      */
    crc16_align:    u32,

    read_callback:  BitReaderReadCallback,
    client_data:    c_void,
}

#[inline] pub fn crc16_update_word(
        br:   *mut BitReader,
        word: u32)  {
    
    todo!();
        /*
            unsigned crc = br->read_crc16;
    #if BYTES_PER_WORD == 4
        switch(br->crc16_align) {
            case  0: crc = CRC16_UPDATE((unsigned)(word >> 24), crc);
            case  8: crc = CRC16_UPDATE((unsigned)((word >> 16) & 0xff), crc);
            case 16: crc = CRC16_UPDATE((unsigned)((word >> 8) & 0xff), crc);
            case 24: br->read_crc16 = CRC16_UPDATE((unsigned)(word & 0xff), crc);
        }
    #elif BYTES_PER_WORD == 8
        switch(br->crc16_align) {
            case  0: crc = CRC16_UPDATE((unsigned)(word >> 56), crc);
            case  8: crc = CRC16_UPDATE((unsigned)((word >> 48) & 0xff), crc);
            case 16: crc = CRC16_UPDATE((unsigned)((word >> 40) & 0xff), crc);
            case 24: crc = CRC16_UPDATE((unsigned)((word >> 32) & 0xff), crc);
            case 32: crc = CRC16_UPDATE((unsigned)((word >> 24) & 0xff), crc);
            case 40: crc = CRC16_UPDATE((unsigned)((word >> 16) & 0xff), crc);
            case 48: crc = CRC16_UPDATE((unsigned)((word >> 8) & 0xff), crc);
            case 56: br->read_crc16 = CRC16_UPDATE((unsigned)(word & 0xff), crc);
        }
    #else
        for( ; br->crc16_align < BITS_PER_WORD; br->crc16_align += 8)
            crc = CRC16_UPDATE((unsigned)((word >> (BITS_PER_WORD-8-br->crc16_align)) & 0xff), crc);
        br->read_crc16 = crc;
    #endif
        br->crc16_align = 0;
        */
}

pub fn bitreader_read_from_client(br: *mut BitReader) -> bool {
    
    todo!();
        /*
            unsigned start, end;
        size_t bytes;
        byte *target;

        /* first shift the unconsumed buffer data toward the front as much as possible */
        if(br->consumed_words > 0) {
            start = br->consumed_words;
            end = br->words + (br->bytes? 1:0);
            memmove(br->buffer, br->buffer+start, BYTES_PER_WORD * (end - start));

            br->words -= start;
            br->consumed_words = 0;
        }

        /*
         * set the target for reading, taking into account word alignment and endianness
         */
        bytes = (br->capacity - br->words) * BYTES_PER_WORD - br->bytes;
        if(bytes == 0)
            return false; /* no space left, buffer is too small; see note for BITREADER_DEFAULT_CAPACITY  */
        target = ((byte*)(br->buffer+br->words)) + br->bytes;

        /* before reading, if the existing reader looks like this (say uint32_t is 32 bits wide)
         *   bitstream :  11 22 33 44 55            br->words=1 br->bytes=1 (partial tail word is left-justified)
         *   buffer[BE]:  11 22 33 44 55 ?? ?? ??   (shown layed out as bytes sequentially in memory)
         *   buffer[LE]:  44 33 22 11 ?? ?? ?? 55   (?? being don't-care)
         *                               ^^-------target, bytes=3
         * on LE machines, have to byteswap the odd tail word so nothing is
         * overwritten:
         */
    #if WORDS_BIGENDIAN
    #else
        if(br->bytes)
            br->buffer[br->words] = SWAP_BE_WORD_TO_HOST(br->buffer[br->words]);
    #endif

        /* now it looks like:
         *   bitstream :  11 22 33 44 55            br->words=1 br->bytes=1
         *   buffer[BE]:  11 22 33 44 55 ?? ?? ??
         *   buffer[LE]:  44 33 22 11 55 ?? ?? ??
         *                               ^^-------target, bytes=3
         */

        /* read in the data; note that the callback may return a smaller number of bytes */
        if(!br->read_callback(target, &bytes, br->client_data))
            return false;

        /* after reading bytes 66 77 88 99 AA BB CC DD EE FF from the client:
         *   bitstream :  11 22 33 44 55 66 77 88 99 AA BB CC DD EE FF
         *   buffer[BE]:  11 22 33 44 55 66 77 88 99 AA BB CC DD EE FF ??
         *   buffer[LE]:  44 33 22 11 55 66 77 88 99 AA BB CC DD EE FF ??
         * now have to byteswap on LE machines:
         */
    #if WORDS_BIGENDIAN
    #else
        end = (br->words*BYTES_PER_WORD + br->bytes + bytes + (BYTES_PER_WORD-1)) / BYTES_PER_WORD;
        for(start = br->words; start < end; start++)
            br->buffer[start] = SWAP_BE_WORD_TO_HOST(br->buffer[start]);
    #endif

        /* now it looks like:
         *   bitstream :  11 22 33 44 55 66 77 88 99 AA BB CC DD EE FF
         *   buffer[BE]:  11 22 33 44 55 66 77 88 99 AA BB CC DD EE FF ??
         *   buffer[LE]:  44 33 22 11 88 77 66 55 CC BB AA 99 ?? FF EE DD
         * finally we'll update the reader values:
         */
        end = br->words*BYTES_PER_WORD + br->bytes + bytes;
        br->words = end / BYTES_PER_WORD;
        br->bytes = end % BYTES_PER_WORD;

        return true;
        */
}

pub fn flac_bitreader_new() -> *mut BitReader {
    
    todo!();
        /*
            BitReader *br = (BitReader*) calloc(1, sizeof(BitReader));

        /* calloc() implies:
            memset(br, 0, sizeof(BitReader));
            br->buffer = 0;
            br->capacity = 0;
            br->words = br->bytes = 0;
            br->consumed_words = br->consumed_bits = 0;
            br->read_callback = 0;
            br->client_data = 0;
        */
        return br;
        */
}

pub fn flac_bitreader_delete(br: *mut BitReader)  {
    
    todo!();
        /*
            ASSERT(0 != br);

        bitreader_free(br);
        free(br);
        */
}

pub fn flac_bitreader_init(
        br:  *mut BitReader,
        rcb: BitReaderReadCallback,
        cd:  *mut c_void) -> bool {
    
    todo!();
        /*
            ASSERT(0 != br);

        br->words = br->bytes = 0;
        br->consumed_words = br->consumed_bits = 0;
        br->capacity = BITREADER_DEFAULT_CAPACITY;
        br->buffer = (uint32_t*) malloc(sizeof(uint32_t) * br->capacity);
        if(br->buffer == 0)
            return false;
        br->read_callback = rcb;
        br->client_data = cd;

        return true;
        */
}

pub fn flac_bitreader_free(br: *mut BitReader)  {
    
    todo!();
        /*
            ASSERT(0 != br);

        if(0 != br->buffer)
            free(br->buffer);
        br->buffer = 0;
        br->capacity = 0;
        br->words = br->bytes = 0;
        br->consumed_words = br->consumed_bits = 0;
        br->read_callback = 0;
        br->client_data = 0;
        */
}

pub fn flac_bitreader_clear(br: *mut BitReader) -> bool {
    
    todo!();
        /*
            br->words = br->bytes = 0;
        br->consumed_words = br->consumed_bits = 0;
        return true;
        */
}

pub fn flac_bitreader_dump(
        br:  *const BitReader,
        out: *mut libc::FILE)  {
    
    todo!();
        /*
            unsigned i, j;
        if(br == 0) {
            fprintf(out, "bitreader is NULL\n");
        }
        else {
            fprintf(out, "bitreader: capacity=%u words=%u bytes=%u consumed: words=%u, bits=%u\n", br->capacity, br->words, br->bytes, br->consumed_words, br->consumed_bits);

            for(i = 0; i < br->words; i++) {
                fprintf(out, "%08X: ", i);
                for(j = 0; j < BITS_PER_WORD; j++)
                    if(i < br->consumed_words || (i == br->consumed_words && j < br->consumed_bits))
                        fprintf(out, ".");
                    else
                        fprintf(out, "%01u", br->buffer[i] & (1 << (BITS_PER_WORD-j-1)) ? 1:0);
                fprintf(out, "\n");
            }
            if(br->bytes > 0) {
                fprintf(out, "%08X: ", i);
                for(j = 0; j < br->bytes*8; j++)
                    if(i < br->consumed_words || (i == br->consumed_words && j < br->consumed_bits))
                        fprintf(out, ".");
                    else
                        fprintf(out, "%01u", br->buffer[i] & (1 << (br->bytes*8-j-1)) ? 1:0);
                fprintf(out, "\n");
            }
        }
        */
}

pub fn flac_bitreader_reset_read_crc16(
        br:   *mut BitReader,
        seed: u16)  {
    
    todo!();
        /*
            ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        ASSERT((br->consumed_bits & 7) == 0);

        br->read_crc16 = (unsigned)seed;
        br->crc16_align = br->consumed_bits;
        */
}

pub fn flac_bitreader_get_read_crc16(br: *mut BitReader) -> u16 {
    
    todo!();
        /*
            ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        ASSERT((br->consumed_bits & 7) == 0);
        ASSERT(br->crc16_align <= br->consumed_bits);

        /* CRC any tail bytes in a partially-consumed word */
        if(br->consumed_bits) {
            const uint32_t tail = br->buffer[br->consumed_words];
            for( ; br->crc16_align < br->consumed_bits; br->crc16_align += 8)
                br->read_crc16 = CRC16_UPDATE((unsigned)((tail >> (BITS_PER_WORD-8-br->crc16_align)) & 0xff), br->read_crc16);
        }
        return br->read_crc16;
        */
}

#[inline] pub fn flac_bitreader_is_consumed_byte_aligned(br: *const BitReader) -> bool {
    
    todo!();
        /*
            return ((br->consumed_bits & 7) == 0);
        */
}

#[inline] pub fn flac_bitreader_bits_left_for_byte_alignment(br: *const BitReader) -> u32 {
    
    todo!();
        /*
            return 8 - (br->consumed_bits & 7);
        */
}

#[inline] pub fn flac_bitreader_get_input_bits_unconsumed(br: *const BitReader) -> u32 {
    
    todo!();
        /*
            return (br->words-br->consumed_words)*BITS_PER_WORD + br->bytes*8 - br->consumed_bits;
        */
}

pub fn flac_bitreader_read_raw_uint32(
        br:   *mut BitReader,
        val:  *mut u32,
        bits: u32) -> bool {
    
    todo!();
        /*
            ASSERT(0 != br);
        ASSERT(0 != br->buffer);

        ASSERT(bits <= 32);
        ASSERT((br->capacity*BITS_PER_WORD) * 2 >= bits);
        ASSERT(br->consumed_words <= br->words);

        /* WATCHOUT: code does not work with <32bit words; we can make things much faster with this assertion */
        ASSERT(BITS_PER_WORD >= 32);

        if(bits == 0) { /* OPT: investigate if this can ever happen, maybe change to assertion */
            *val = 0;
            return true;
        }

        while((br->words-br->consumed_words)*BITS_PER_WORD + br->bytes*8 - br->consumed_bits < bits) {
            if(!bitreader_read_from_client_(br))
                return false;
        }
        if(br->consumed_words < br->words) { /* if we've not consumed up to a partial tail word... */
            /* OPT: taking out the consumed_bits==0 "else" case below might make things faster if less code allows the compiler to inline this function */
            if(br->consumed_bits) {
                /* this also works when consumed_bits==0, it's just a little slower than necessary for that case */
                const unsigned n = BITS_PER_WORD - br->consumed_bits;
                const uint32_t word = br->buffer[br->consumed_words];
                if(bits < n) {
                    *val = (word & (WORD_ALL_ONES >> br->consumed_bits)) >> (n-bits);
                    br->consumed_bits += bits;
                    return true;
                }
                *val = word & (WORD_ALL_ONES >> br->consumed_bits);
                bits -= n;
                crc16_update_word_(br, word);
                br->consumed_words++;
                br->consumed_bits = 0;
                if(bits) { /* if there are still bits left to read, there have to be less than 32 so they will all be in the next word */
                    *val <<= bits;
                    *val |= (br->buffer[br->consumed_words] >> (BITS_PER_WORD-bits));
                    br->consumed_bits = bits;
                }
                return true;
            }
            else {
                const uint32_t word = br->buffer[br->consumed_words];
                if(bits < BITS_PER_WORD) {
                    *val = word >> (BITS_PER_WORD-bits);
                    br->consumed_bits = bits;
                    return true;
                }
                /* at this point 'bits' must be == BITS_PER_WORD; because of previous assertions, it can't be larger */
                *val = word;
                crc16_update_word_(br, word);
                br->consumed_words++;
                return true;
            }
        }
        else {
            /* in this case we're starting our read at a partial tail word;
             * the reader has guaranteed that we have at least 'bits' bits
             * available to read, which makes this case simpler.
             */
            /* OPT: taking out the consumed_bits==0 "else" case below might make things faster if less code allows the compiler to inline this function */
            if(br->consumed_bits) {
                /* this also works when consumed_bits==0, it's just a little slower than necessary for that case */
                ASSERT(br->consumed_bits + bits <= br->bytes*8);
                *val = (br->buffer[br->consumed_words] & (WORD_ALL_ONES >> br->consumed_bits)) >> (BITS_PER_WORD-br->consumed_bits-bits);
                br->consumed_bits += bits;
                return true;
            }
            else {
                *val = br->buffer[br->consumed_words] >> (BITS_PER_WORD-bits);
                br->consumed_bits += bits;
                return true;
            }
        }
        */
}

pub fn flac_bitreader_read_raw_int32(
        br:   *mut BitReader,
        val:  *mut i32,
        bits: u32) -> bool {
    
    todo!();
        /*
            /* OPT: inline raw u32 code here, or make into a macro if possible in the .h file */
        if(!bitreader_read_raw_uint32(br, (u32*)val, bits))
            return false;
        /* sign-extend: */
        *val <<= (32-bits);
        *val >>= (32-bits);
        return true;
        */
}

pub fn flac_bitreader_read_raw_uint64(
        br:   *mut BitReader,
        val:  *mut u64,
        bits: u32) -> bool {
    
    todo!();
        /*
            u32 hi, lo;

        if(bits > 32) {
            if(!bitreader_read_raw_uint32(br, &hi, bits-32))
                return false;
            if(!bitreader_read_raw_uint32(br, &lo, 32))
                return false;
            *val = hi;
            *val <<= 32;
            *val |= lo;
        }
        else {
            if(!bitreader_read_raw_uint32(br, &lo, bits))
                return false;
            *val = lo;
        }
        return true;
        */
}

#[inline] pub fn flac_bitreader_read_uint32_little_endian(
        br:  *mut BitReader,
        val: *mut u32) -> bool {
    
    todo!();
        /*
            u32 x8, x32 = 0;

        /* this doesn't need to be that fast as currently it is only used for vorbis comments */

        if(!bitreader_read_raw_uint32(br, &x32, 8))
            return false;

        if(!bitreader_read_raw_uint32(br, &x8, 8))
            return false;
        x32 |= (x8 << 8);

        if(!bitreader_read_raw_uint32(br, &x8, 8))
            return false;
        x32 |= (x8 << 16);

        if(!bitreader_read_raw_uint32(br, &x8, 8))
            return false;
        x32 |= (x8 << 24);

        *val = x32;
        return true;
        */
}

pub fn flac_bitreader_skip_bits_no_crc(
        br:   *mut BitReader,
        bits: u32) -> bool {
    
    todo!();
        /*
            /*
         * OPT: a faster implementation is possible but probably not that useful
         * since this is only called a couple of times in the metadata readers.
         */
        ASSERT(0 != br);
        ASSERT(0 != br->buffer);

        if(bits > 0) {
            const unsigned n = br->consumed_bits & 7;
            unsigned m;
            u32 x;

            if(n != 0) {
                m = flac_min(8-n, bits);
                if(!bitreader_read_raw_uint32(br, &x, m))
                    return false;
                bits -= m;
            }
            m = bits / 8;
            if(m > 0) {
                if(!bitreader_skip_byte_block_aligned_no_crc(br, m))
                    return false;
                bits %= 8;
            }
            if(bits > 0) {
                if(!bitreader_read_raw_uint32(br, &x, bits))
                    return false;
            }
        }

        return true;
        */
}

pub fn flac_bitreader_skip_byte_block_aligned_no_crc(
        br:    *mut BitReader,
        nvals: u32) -> bool {
    
    todo!();
        /*
            u32 x;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        ASSERT(bitreader_is_consumed_byte_aligned(br));

        /* step 1: skip over partial head word to get word aligned */
        while(nvals && br->consumed_bits) { /* i.e. run until we read 'nvals' bytes or we hit the end of the head word */
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            nvals--;
        }
        if(0 == nvals)
            return true;
        /* step 2: skip whole words in chunks */
        while(nvals >= BYTES_PER_WORD) {
            if(br->consumed_words < br->words) {
                br->consumed_words++;
                nvals -= BYTES_PER_WORD;
            }
            else if(!bitreader_read_from_client_(br))
                return false;
        }
        /* step 3: skip any remainder from partial tail bytes */
        while(nvals) {
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            nvals--;
        }

        return true;
        */
}

pub fn flac_bitreader_read_byte_block_aligned_no_crc(
        br:    *mut BitReader,
        val:   *mut u8,
        nvals: u32) -> bool {
    
    todo!();
        /*
            u32 x;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        ASSERT(bitreader_is_consumed_byte_aligned(br));

        /* step 1: read from partial head word to get word aligned */
        while(nvals && br->consumed_bits) { /* i.e. run until we read 'nvals' bytes or we hit the end of the head word */
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            *val++ = (byte)x;
            nvals--;
        }
        if(0 == nvals)
            return true;
        /* step 2: read whole words in chunks */
        while(nvals >= BYTES_PER_WORD) {
            if(br->consumed_words < br->words) {
                const uint32_t word = br->buffer[br->consumed_words++];
    #if BYTES_PER_WORD == 4
                val[0] = (byte)(word >> 24);
                val[1] = (byte)(word >> 16);
                val[2] = (byte)(word >> 8);
                val[3] = (byte)word;
    #elif BYTES_PER_WORD == 8
                val[0] = (byte)(word >> 56);
                val[1] = (byte)(word >> 48);
                val[2] = (byte)(word >> 40);
                val[3] = (byte)(word >> 32);
                val[4] = (byte)(word >> 24);
                val[5] = (byte)(word >> 16);
                val[6] = (byte)(word >> 8);
                val[7] = (byte)word;
    #else
                for(x = 0; x < BYTES_PER_WORD; x++)
                    val[x] = (byte)(word >> (8*(BYTES_PER_WORD-x-1)));
    #endif
                val += BYTES_PER_WORD;
                nvals -= BYTES_PER_WORD;
            }
            else if(!bitreader_read_from_client_(br))
                return false;
        }
        /* step 3: read any remainder from partial tail bytes */
        while(nvals) {
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            *val++ = (byte)x;
            nvals--;
        }

        return true;
        */
}

pub fn flac_bitreader_read_unary_unsigned(
        br:  *mut BitReader,
        val: *mut u32) -> bool {
    
    todo!();
        /*
            /*
        #if 0 /* slow but readable version */
        {
            unsigned bit;

            ASSERT(0 != br);
            ASSERT(0 != br->buffer);

            *val = 0;
            while(1) {
                if(!bitreader_read_bit(br, &bit))
                    return false;
                if(bit)
                    break;
                else
                    *val++;
            }
            return true;
        }
        #endif
        */

        unsigned i;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);

        *val = 0;
        while(1) {
            while(br->consumed_words < br->words) { /* if we've not consumed up to a partial tail word... */
                uint32_t b = br->buffer[br->consumed_words] << br->consumed_bits;
                if(b) {
                    i = clz_uint32(b);
                    *val += i;
                    i++;
                    br->consumed_bits += i;
                    if(br->consumed_bits >= BITS_PER_WORD) { /* faster way of testing if(br->consumed_bits == BITS_PER_WORD) */
                        crc16_update_word_(br, br->buffer[br->consumed_words]);
                        br->consumed_words++;
                        br->consumed_bits = 0;
                    }
                    return true;
                }
                else {
                    *val += BITS_PER_WORD - br->consumed_bits;
                    crc16_update_word_(br, br->buffer[br->consumed_words]);
                    br->consumed_words++;
                    br->consumed_bits = 0;
                    /* didn't find stop bit yet, have to keep going... */
                }
            }
            /* at this point we've eaten up all the whole words; have to try
             * reading through any tail bytes before calling the read callback.
             * this is a repeat of the above logic adjusted for the fact we
             * don't have a whole word.  note though if the client is feeding
             * us data a byte at a time (unlikely), br->consumed_bits may not
             * be zero.
             */
            if(br->bytes*8 > br->consumed_bits) {
                const unsigned end = br->bytes * 8;
                uint32_t b = (br->buffer[br->consumed_words] & (WORD_ALL_ONES << (BITS_PER_WORD-end))) << br->consumed_bits;
                if(b) {
                    i = clz_uint32(b);
                    *val += i;
                    i++;
                    br->consumed_bits += i;
                    ASSERT(br->consumed_bits < BITS_PER_WORD);
                    return true;
                }
                else {
                    *val += end - br->consumed_bits;
                    br->consumed_bits = end;
                    ASSERT(br->consumed_bits < BITS_PER_WORD);
                    /* didn't find stop bit yet, have to keep going... */
                }
            }
            if(!bitreader_read_from_client_(br))
                return false;
        }
        */
}

pub fn flac_bitreader_read_rice_signed(
        br:        *mut BitReader,
        val:       *mut i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            u32 lsbs = 0, msbs = 0;
        unsigned uval;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        ASSERT(parameter <= 31);

        /* read the unary MSBs and end bit */
        if(!bitreader_read_unary_unsigned(br, &msbs))
            return false;

        /* read the binary LSBs */
        if(!bitreader_read_raw_uint32(br, &lsbs, parameter))
            return false;

        /* compose the value */
        uval = (msbs << parameter) | lsbs;
        if(uval & 1)
            *val = -((int)(uval >> 1)) - 1;
        else
            *val = (int)(uval >> 1);

        return true;
        */
}

/**
  | this is by far the most heavily used reader
  | call. it ain't pretty but it's fast
  |
  */
pub fn flac_bitreader_read_rice_signed_block(
        br:        *mut BitReader,
        vals:      &[i32],
        nvals:     u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            /* try and get br->consumed_words and br->consumed_bits into register;
         * must remember to flush them back to *br before calling other
         * bitreader functions that use them, and before returning */
        unsigned cwords, words, lsbs, msbs, x, y;
        unsigned ucbits; /* keep track of the number of unconsumed bits in word */
        uint32_t b;
        int *val, *end;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);
        /* WATCHOUT: code does not work with <32bit words; we can make things much faster with this assertion */
        ASSERT(BITS_PER_WORD >= 32);
        ASSERT(parameter < 32);
        /* the above two asserts also guarantee that the binary part never straddles more than 2 words, so we don't have to loop to read it */

        val = vals;
        end = vals + nvals;

        if(parameter == 0) {
            while(val < end) {
                /* read the unary MSBs and end bit */
                if(!bitreader_read_unary_unsigned(br, &msbs))
                    return false;

                *val++ = (int)(msbs >> 1) ^ -(int)(msbs & 1);
            }

            return true;
        }

        ASSERT(parameter > 0);

        cwords = br->consumed_words;
        words = br->words;

        /* if we've not consumed up to a partial tail word... */
        if(cwords >= words) {
            x = 0;
            goto process_tail;
        }

        ucbits = BITS_PER_WORD - br->consumed_bits;
        b = br->buffer[cwords] << br->consumed_bits;  /* keep unconsumed bits aligned to left */

        while(val < end) {
            /* read the unary MSBs and end bit */
            x = y = clz2_uint32(b);
            if(x == BITS_PER_WORD) {
                x = ucbits;
                do {
                    /* didn't find stop bit yet, have to keep going... */
                    crc16_update_word_(br, br->buffer[cwords++]);
                    if (cwords >= words)
                        goto incomplete_msbs;
                    b = br->buffer[cwords];
                    y = clz2_uint32(b);
                    x += y;
                } while(y == BITS_PER_WORD);
            }
            b <<= y;
            b <<= 1; /* account for stop bit */
            ucbits = (ucbits - x - 1) % BITS_PER_WORD;
            msbs = x;

            /* read the binary LSBs */
            x = b >> (BITS_PER_WORD - parameter);
            if(parameter <= ucbits) {
                ucbits -= parameter;
                b <<= parameter;
            } else {
                /* there are still bits left to read, they will all be in the next word */
                crc16_update_word_(br, br->buffer[cwords++]);
                if (cwords >= words)
                    goto incomplete_lsbs;
                b = br->buffer[cwords];
                ucbits += BITS_PER_WORD - parameter;
                x |= b >> ucbits;
                b <<= BITS_PER_WORD - ucbits;
            }
            lsbs = x;

            /* compose the value */
            x = (msbs << parameter) | lsbs;
            *val++ = (int)(x >> 1) ^ -(int)(x & 1);

            continue;

            /* at this point we've eaten up all the whole words */
    process_tail:
            do {
                if(0) {
    incomplete_msbs:
                    br->consumed_bits = 0;
                    br->consumed_words = cwords;
                }

                /* read the unary MSBs and end bit */
                if(!bitreader_read_unary_unsigned(br, &msbs))
                    return false;
                msbs += x;
                x = ucbits = 0;

                if(0) {
    incomplete_lsbs:
                    br->consumed_bits = 0;
                    br->consumed_words = cwords;
                }

                /* read the binary LSBs */
                if(!bitreader_read_raw_uint32(br, &lsbs, parameter - ucbits))
                    return false;
                lsbs = x | lsbs;

                /* compose the value */
                x = (msbs << parameter) | lsbs;
                *val++ = (int)(x >> 1) ^ -(int)(x & 1);
                x = 0;

                cwords = br->consumed_words;
                words = br->words;
                ucbits = BITS_PER_WORD - br->consumed_bits;
                b = br->buffer[cwords] << br->consumed_bits;
            } while(cwords >= words && val < end);
        }

        if(ucbits == 0 && cwords < words) {
            /* don't leave the head word with no unconsumed bits */
            crc16_update_word_(br, br->buffer[cwords++]);
            ucbits = BITS_PER_WORD;
        }

        br->consumed_bits = BITS_PER_WORD - ucbits;
        br->consumed_words = cwords;

        return true;
        */
}

#[cfg(UNUSED)]
pub fn flac_bitreader_read_golomb_signed(
        br:        *mut BitReader,
        val:       *mut i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            u32 lsbs = 0, msbs = 0;
        unsigned bit, uval, k;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);

        k = bitmath_ilog2(parameter);

        /* read the unary MSBs and end bit */
        if(!bitreader_read_unary_unsigned(br, &msbs))
            return false;

        /* read the binary LSBs */
        if(!bitreader_read_raw_uint32(br, &lsbs, k))
            return false;

        if(parameter == 1u<<k) {
            /* compose the value */
            uval = (msbs << k) | lsbs;
        }
        else {
            unsigned d = (1 << (k+1)) - parameter;
            if(lsbs >= d) {
                if(!bitreader_read_bit(br, &bit))
                    return false;
                lsbs <<= 1;
                lsbs |= bit;
                lsbs -= d;
            }
            /* compose the value */
            uval = msbs * parameter + lsbs;
        }

        /* unfold unsigned to signed */
        if(uval & 1)
            *val = -((int)(uval >> 1)) - 1;
        else
            *val = (int)(uval >> 1);

        return true;
        */
}

#[cfg(UNUSED)]
pub fn flac_bitreader_read_golomb_unsigned(
        br:        *mut BitReader,
        val:       *mut u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            u32 lsbs, msbs = 0;
        unsigned bit, k;

        ASSERT(0 != br);
        ASSERT(0 != br->buffer);

        k = bitmath_ilog2(parameter);

        /* read the unary MSBs and end bit */
        if(!bitreader_read_unary_unsigned(br, &msbs))
            return false;

        /* read the binary LSBs */
        if(!bitreader_read_raw_uint32(br, &lsbs, k))
            return false;

        if(parameter == 1u<<k) {
            /* compose the value */
            *val = (msbs << k) | lsbs;
        }
        else {
            unsigned d = (1 << (k+1)) - parameter;
            if(lsbs >= d) {
                if(!bitreader_read_bit(br, &bit))
                    return false;
                lsbs <<= 1;
                lsbs |= bit;
                lsbs -= d;
            }
            /* compose the value */
            *val = msbs * parameter + lsbs;
        }

        return true;
        */
}

/**
  | on return, if *val == 0xffffffff then
  | the utf-8 sequence was invalid, but
  | the return value will be true
  |
  */
pub fn flac_bitreader_read_utf8_uint32(
        br:     *mut BitReader,
        val:    *mut u32,
        raw:    *mut u8,
        rawlen: *mut u32) -> bool {
    
    todo!();
        /*
            u32 v = 0;
        u32 x;
        unsigned i;

        if(!bitreader_read_raw_uint32(br, &x, 8))
            return false;
        if(raw)
            raw[(*rawlen)++] = (byte)x;
        if(!(x & 0x80)) { /* 0xxxxxxx */
            v = x;
            i = 0;
        }
        else if(x & 0xC0 && !(x & 0x20)) { /* 110xxxxx */
            v = x & 0x1F;
            i = 1;
        }
        else if(x & 0xE0 && !(x & 0x10)) { /* 1110xxxx */
            v = x & 0x0F;
            i = 2;
        }
        else if(x & 0xF0 && !(x & 0x08)) { /* 11110xxx */
            v = x & 0x07;
            i = 3;
        }
        else if(x & 0xF8 && !(x & 0x04)) { /* 111110xx */
            v = x & 0x03;
            i = 4;
        }
        else if(x & 0xFC && !(x & 0x02)) { /* 1111110x */
            v = x & 0x01;
            i = 5;
        }
        else {
            *val = 0xffffffff;
            return true;
        }
        for( ; i; i--) {
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            if(raw)
                raw[(*rawlen)++] = (byte)x;
            if(!(x & 0x80) || (x & 0x40)) { /* 10xxxxxx */
                *val = 0xffffffff;
                return true;
            }
            v <<= 6;
            v |= (x & 0x3F);
        }
        *val = v;
        return true;
        */
}

/**
  | on return, if *val == 0xffffffffffffffff
  | then the utf-8 sequence was invalid,
  | but the return value will be true
  |
  */
pub fn flac_bitreader_read_utf8_uint64(
        br:     *mut BitReader,
        val:    *mut u64,
        raw:    *mut u8,
        rawlen: *mut u32) -> bool {
    
    todo!();
        /*
            u64 v = 0;
        u32 x;
        unsigned i;

        if(!bitreader_read_raw_uint32(br, &x, 8))
            return false;
        if(raw)
            raw[(*rawlen)++] = (byte)x;
        if(!(x & 0x80)) { /* 0xxxxxxx */
            v = x;
            i = 0;
        }
        else if(x & 0xC0 && !(x & 0x20)) { /* 110xxxxx */
            v = x & 0x1F;
            i = 1;
        }
        else if(x & 0xE0 && !(x & 0x10)) { /* 1110xxxx */
            v = x & 0x0F;
            i = 2;
        }
        else if(x & 0xF0 && !(x & 0x08)) { /* 11110xxx */
            v = x & 0x07;
            i = 3;
        }
        else if(x & 0xF8 && !(x & 0x04)) { /* 111110xx */
            v = x & 0x03;
            i = 4;
        }
        else if(x & 0xFC && !(x & 0x02)) { /* 1111110x */
            v = x & 0x01;
            i = 5;
        }
        else if(x & 0xFE && !(x & 0x01)) { /* 11111110 */
            v = 0;
            i = 6;
        }
        else {
            *val = U64L(0xffffffffffffffff);
            return true;
        }
        for( ; i; i--) {
            if(!bitreader_read_raw_uint32(br, &x, 8))
                return false;
            if(raw)
                raw[(*rawlen)++] = (byte)x;
            if(!(x & 0x80) || (x & 0x40)) { /* 10xxxxxx */
                *val = U64L(0xffffffffffffffff);
                return true;
            }
            v <<= 6;
            v |= (x & 0x3F);
        }
        *val = v;
        return true;
        */
}

/**
  | These functions are declared inline
  | in this file but are also callable as
  | externs from elsewhere.
  | 
  | According to the C99 spec, section 6.7.4,
  | simply providing a function prototype
  | in a header file without 'inline' and
  | making the function inline in this file
  | should be sufficient.
  | 
  | Unfortunately, the Microsoft VS compiler
  | doesn't pick them up externally. To
  | fix that we add extern declarations
  | here.
  |
  */
lazy_static!{
    /*
    extern bool bitreader_is_consumed_byte_aligned(const BitReader *br);
    extern unsigned bitreader_bits_left_for_byte_alignment(const BitReader *br);
    extern unsigned bitreader_get_input_bits_unconsumed(const BitReader *br);
    extern bool bitreader_read_uint32_little_endian(BitReader *br, u32 *val);
    */
}
