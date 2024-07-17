crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/bitwriter.c]

/**
  | Things should be fastest when this matches
  | the machine word size
  | 
  | -----------
  | @warning
  | 
  | if you change this you must also change
  | the following #defines down to SWAP_BE_WORD_TO_HOST
  | below to match
  | ----------
  | @warning
  | 
  | there are a few places where the code
  | will not work unless uint32_t is >= 32
  | bits wide
  |
  */
pub const BYTES_PER_WORD: u32 = 4;
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
  | The default capacity here doesn't matter
  | too much. The buffer always grows to
  | hold whatever is written to it. Usually
  | the encoder will stop adding at a frame
  | or metadata block, then write that out
  | and clear the buffer for the next one.
  |
  */
pub const BITWRITER_DEFAULT_CAPACITY: usize = 32768 / size_of::<u32>(); /* size in words */

/**
  | When growing, increment 4K at a time
  |
  */
pub const BITWRITER_DEFAULT_INCREMENT: usize = 4096 / size_of::<u32>(); /* size in words */

macro_rules! words_to_bits {
    ($words:ident) => {
        /*
                ((words) * BITS_PER_WORD)
        */
    }
}

macro_rules! total_bits {
    ($bw:ident) => {
        /*
                (WORDS_TO_BITS((bw)->words) + (bw)->bits)
        */
    }
}

pub struct BitWriter {

    buffer:   u32,

    /**
      | accumulator; bits are right-justified;
      | when full, accum is appended to buffer
      |
      */
    accum:    u32,


    /**
      | capacity of buffer in words
      |
      */
    capacity: u32,


    /**
      | # of complete words in buffer
      |
      */
    words:    u32,


    /**
      | # of used bits in accum
      |
      */
    bits:     u32,
}

/**
  | WATCHOUT: The current implementation
  | only grows the buffer.
  |
  */
pub fn bitwriter_grow(
        bw:          *mut BitWriter,
        bits_to_add: u32) -> bool {
    
    todo!();
        /*
            unsigned new_capacity;
        uint32_t *new_buffer;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);

        /* calculate total words needed to store 'bits_to_add' additional bits */
        new_capacity = bw->words + ((bw->bits + bits_to_add + BITS_PER_WORD - 1) / BITS_PER_WORD);

        /* it's possible (due to pessimism in the growth estimation that
         * leads to this call) that we don't actually need to grow
         */
        if(bw->capacity >= new_capacity)
            return true;

        /* round up capacity increase to the nearest BITWRITER_DEFAULT_INCREMENT */
        if((new_capacity - bw->capacity) % BITWRITER_DEFAULT_INCREMENT)
            new_capacity += BITWRITER_DEFAULT_INCREMENT - ((new_capacity - bw->capacity) % BITWRITER_DEFAULT_INCREMENT);
        /* make sure we got everything right */
        ASSERT(0 == (new_capacity - bw->capacity) % BITWRITER_DEFAULT_INCREMENT);
        ASSERT(new_capacity > bw->capacity);
        ASSERT(new_capacity >= bw->words + ((bw->bits + bits_to_add + BITS_PER_WORD - 1) / BITS_PER_WORD));

        new_buffer = (uint32_t*) safe_realloc_mul_2op_(bw->buffer, sizeof(uint32_t), /*times*/new_capacity);
        if(new_buffer == 0)
            return false;
        bw->buffer = new_buffer;
        bw->capacity = new_capacity;
        return true;
        */
}

pub fn bitwriter_new() -> *mut BitWriter {
    
    todo!();
        /*
            BitWriter *bw = (BitWriter*) calloc(1, sizeof(BitWriter));
        /* note that calloc() sets all members to 0 for us */
        return bw;
        */
}

pub fn bitwriter_delete(bw: *mut BitWriter)  {
    
    todo!();
        /*
            ASSERT(0 != bw);

        bitwriter_free(bw);
        free(bw);
        */
}

pub fn bitwriter_init(bw: *mut BitWriter) -> bool {
    
    todo!();
        /*
            ASSERT(0 != bw);

        bw->words = bw->bits = 0;
        bw->capacity = BITWRITER_DEFAULT_CAPACITY;
        bw->buffer = (uint32_t*) malloc(sizeof(uint32_t) * bw->capacity);
        if(bw->buffer == 0)
            return false;

        return true;
        */
}

pub fn bitwriter_free(bw: *mut BitWriter)  {
    
    todo!();
        /*
            ASSERT(0 != bw);

        if(0 != bw->buffer)
            free(bw->buffer);
        bw->buffer = 0;
        bw->capacity = 0;
        bw->words = bw->bits = 0;
        */
}

pub fn bitwriter_clear(bw: *mut BitWriter)  {
    
    todo!();
        /*
            bw->words = bw->bits = 0;
        */
}

pub fn bitwriter_dump(
        bw:  *const BitWriter,
        out: *mut libc::FILE)  {
    
    todo!();
        /*
            unsigned i, j;
        if(bw == 0) {
            fprintf(out, "bitwriter is NULL\n");
        }
        else {
            fprintf(out, "bitwriter: capacity=%u words=%u bits=%u total_bits=%u\n", bw->capacity, bw->words, bw->bits, TOTAL_BITS(bw));

            for(i = 0; i < bw->words; i++) {
                fprintf(out, "%08X: ", i);
                for(j = 0; j < BITS_PER_WORD; j++)
                    fprintf(out, "%01u", bw->buffer[i] & (1 << (BITS_PER_WORD-j-1)) ? 1:0);
                fprintf(out, "\n");
            }
            if(bw->bits > 0) {
                fprintf(out, "%08X: ", i);
                for(j = 0; j < bw->bits; j++)
                    fprintf(out, "%01u", bw->accum & (1 << (bw->bits-j-1)) ? 1:0);
                fprintf(out, "\n");
            }
        }
        */
}

pub fn bitwriter_get_write_crc16(
        bw:  *mut BitWriter,
        crc: *mut u16) -> bool {
    
    todo!();
        /*
            const byte *buffer;
        size_t bytes;

        ASSERT((bw->bits & 7) == 0); /* assert that we're byte-aligned */

        if(!bitwriter_get_buffer(bw, &buffer, &bytes))
            return false;

        *crc = (u16)crc16(buffer, bytes);
        bitwriter_release_buffer(bw);
        return true;
        */
}

pub fn bitwriter_get_write_crc8(
        bw:  *mut BitWriter,
        crc: *mut u8) -> bool {
    
    todo!();
        /*
            const byte *buffer;
        size_t bytes;

        ASSERT((bw->bits & 7) == 0); /* assert that we're byte-aligned */

        if(!bitwriter_get_buffer(bw, &buffer, &bytes))
            return false;

        *crc = crc8(buffer, bytes);
        bitwriter_release_buffer(bw);
        return true;
        */
}

pub fn bitwriter_is_byte_aligned(bw: *const BitWriter) -> bool {
    
    todo!();
        /*
            return ((bw->bits & 7) == 0);
        */
}

pub fn bitwriter_get_input_bits_unconsumed(bw: *const BitWriter) -> u32 {
    
    todo!();
        /*
            return TOTAL_BITS(bw);
        */
}

pub fn bitwriter_get_buffer(
        bw:     *mut BitWriter,
        buffer: *const *const u8,
        bytes:  *mut usize) -> bool {
    
    todo!();
        /*
            ASSERT((bw->bits & 7) == 0);
        /* double protection */
        if(bw->bits & 7)
            return false;
        /* if we have bits in the accumulator we have to flush those to the buffer first */
        if(bw->bits) {
            ASSERT(bw->words <= bw->capacity);
            if(bw->words == bw->capacity && !bitwriter_grow_(bw, BITS_PER_WORD))
                return false;
            /* append bits as complete word to buffer, but don't change bw->accum or bw->bits */
            bw->buffer[bw->words] = SWAP_BE_WORD_TO_HOST(bw->accum << (BITS_PER_WORD-bw->bits));
        }
        /* now we can just return what we have */
        *buffer = (byte*)bw->buffer;
        *bytes = (BYTES_PER_WORD * bw->words) + (bw->bits >> 3);
        return true;
        */
}

pub fn bitwriter_release_buffer(bw: *mut BitWriter)  {
    
    todo!();
        /*
            /* nothing to do.  in the future, strict checking of a 'writer-is-in-
         * get-mode' flag could be added everywhere and then cleared here
         */
        (void)bw;
        */
}

#[inline] pub fn bitwriter_write_zeroes(
        bw:   *mut BitWriter,
        bits: u32) -> bool {
    
    todo!();
        /*
            unsigned n;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);

        if(bits == 0)
            return true;
        /* slightly pessimistic size check but faster than "<= bw->words + (bw->bits+bits+BITS_PER_WORD-1)/BITS_PER_WORD" */
        if(bw->capacity <= bw->words + bits && !bitwriter_grow_(bw, bits))
            return false;
        /* first part gets to word alignment */
        if(bw->bits) {
            n = min(BITS_PER_WORD - bw->bits, bits);
            bw->accum <<= n;
            bits -= n;
            bw->bits += n;
            if(bw->bits == BITS_PER_WORD) {
                bw->buffer[bw->words++] = SWAP_BE_WORD_TO_HOST(bw->accum);
                bw->bits = 0;
            }
            else
                return true;
        }
        /* do whole words */
        while(bits >= BITS_PER_WORD) {
            bw->buffer[bw->words++] = 0;
            bits -= BITS_PER_WORD;
        }
        /* do any leftovers */
        if(bits > 0) {
            bw->accum = 0;
            bw->bits = bits;
        }
        return true;
        */
}

#[inline] pub fn bitwriter_write_raw_uint32(
        bw:   *mut BitWriter,
        val:  u32,
        bits: u32) -> bool {
    
    todo!();
        /*
            unsigned left;

        /* WATCHOUT: code does not work with <32bit words; we can make things much faster with this assertion */
        ASSERT(BITS_PER_WORD >= 32);

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);

        ASSERT(bits <= 32);
        if(bits == 0)
            return true;

        /* slightly pessimistic size check but faster than "<= bw->words + (bw->bits+bits+BITS_PER_WORD-1)/BITS_PER_WORD" */
        if(bw->capacity <= bw->words + bits && !bitwriter_grow_(bw, bits))
            return false;

        left = BITS_PER_WORD - bw->bits;
        if(bits < left) {
            bw->accum <<= bits;
            bw->accum |= val;
            bw->bits += bits;
        }
        else if(bw->bits) { /* WATCHOUT: if bw->bits == 0, left==BITS_PER_WORD and bw->accum<<=left is a NOP instead of setting to 0 */
            bw->accum <<= left;
            bw->accum |= val >> (bw->bits = bits - left);
            bw->buffer[bw->words++] = SWAP_BE_WORD_TO_HOST(bw->accum);
            bw->accum = val;
        }
        else {
            bw->accum = val;
            bw->bits = 0;
            bw->buffer[bw->words++] = SWAP_BE_WORD_TO_HOST(val);
        }

        return true;
        */
}

#[inline] pub fn bitwriter_write_raw_int32(
        bw:   *mut BitWriter,
        val:  i32,
        bits: u32) -> bool {
    
    todo!();
        /*
            /* zero-out unused bits */
        if(bits < 32)
            val &= (~(0xffffffff << bits));

        return bitwriter_write_raw_uint32(bw, (u32)val, bits);
        */
}

#[inline] pub fn bitwriter_write_raw_uint64(
        bw:   *mut BitWriter,
        val:  u64,
        bits: u32) -> bool {
    
    todo!();
        /*
            /* this could be a little faster but it's not used for much */
        if(bits > 32) {
            return
                bitwriter_write_raw_uint32(bw, (u32)(val>>32), bits-32) &&
                bitwriter_write_raw_uint32(bw, (u32)val, 32);
        }
        else
            return bitwriter_write_raw_uint32(bw, (u32)val, bits);
        */
}

#[inline] pub fn bitwriter_write_raw_uint32_little_endian(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
            /* this doesn't need to be that fast as currently it is only used for vorbis comments */

        if(!bitwriter_write_raw_uint32(bw, val & 0xff, 8))
            return false;
        if(!bitwriter_write_raw_uint32(bw, (val>>8) & 0xff, 8))
            return false;
        if(!bitwriter_write_raw_uint32(bw, (val>>16) & 0xff, 8))
            return false;
        if(!bitwriter_write_raw_uint32(bw, val>>24, 8))
            return false;

        return true;
        */
}

#[inline] pub fn bitwriter_write_byte_block(
        bw:    *mut BitWriter,
        vals:  &[u8],
        nvals: u32) -> bool {
    
    todo!();
        /*
            unsigned i;

        /* this could be faster but currently we don't need it to be since it's only used for writing metadata */
        for(i = 0; i < nvals; i++) {
            if(!bitwriter_write_raw_uint32(bw, (u32)(vals[i]), 8))
                return false;
        }

        return true;
        */
}

pub fn bitwriter_write_unary_unsigned(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
            if(val < 32)
            return bitwriter_write_raw_uint32(bw, 1, ++val);
        else
            return
                bitwriter_write_zeroes(bw, val) &&
                bitwriter_write_raw_uint32(bw, 1, 1);
        */
}

pub fn bitwriter_rice_bits(
        val:       i32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
            u32 uval;

        ASSERT(parameter < sizeof(unsigned)*8);

        /* fold signed to unsigned; actual formula is: negative(v)? -2v-1 : 2v */
        uval = (val<<1) ^ (val>>31);

        return 1 + parameter + (uval >> parameter);
        */
}

#[cfg(UNUSED)]
pub fn bitwriter_golomb_bits_signed(
        val:       i32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
            unsigned bits, msbs, uval;
        unsigned k;

        ASSERT(parameter > 0);

        /* fold signed to unsigned */
        if(val < 0)
            uval = (unsigned)(((-(++val)) << 1) + 1);
        else
            uval = (unsigned)(val << 1);

        k = bitmath_ilog2(parameter);
        if(parameter == 1u<<k) {
            ASSERT(k <= 30);

            msbs = uval >> k;
            bits = 1 + k + msbs;
        }
        else {
            unsigned q, r, d;

            d = (1 << (k+1)) - parameter;
            q = uval / parameter;
            r = uval - (q * parameter);

            bits = 1 + q + k;
            if(r >= d)
                bits++;
        }
        return bits;
        */
}

#[cfg(UNUSED)]
pub fn bitwriter_golomb_bits_unsigned(
        uval:      u32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
            unsigned bits, msbs;
        unsigned k;

        ASSERT(parameter > 0);

        k = bitmath_ilog2(parameter);
        if(parameter == 1u<<k) {
            ASSERT(k <= 30);

            msbs = uval >> k;
            bits = 1 + k + msbs;
        }
        else {
            unsigned q, r, d;

            d = (1 << (k+1)) - parameter;
            q = uval / parameter;
            r = uval - (q * parameter);

            bits = 1 + q + k;
            if(r >= d)
                bits++;
        }
        return bits;
        */
}

pub fn bitwriter_write_rice_signed(
        bw:        *mut BitWriter,
        val:       i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            unsigned total_bits, interesting_bits, msbs;
        u32 uval, pattern;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);
        ASSERT(parameter < 8*sizeof(uval));

        /* fold signed to unsigned; actual formula is: negative(v)? -2v-1 : 2v */
        uval = (val<<1) ^ (val>>31);

        msbs = uval >> parameter;
        interesting_bits = 1 + parameter;
        total_bits = interesting_bits + msbs;
        pattern = 1 << parameter; /* the unary end bit */
        pattern |= (uval & ((1<<parameter)-1)); /* the binary LSBs */

        if(total_bits <= 32)
            return bitwriter_write_raw_uint32(bw, pattern, total_bits);
        else
            return
                bitwriter_write_zeroes(bw, msbs) && /* write the unary MSBs */
                bitwriter_write_raw_uint32(bw, pattern, interesting_bits); /* write the unary end bit and binary LSBs */
        */
}

pub fn bitwriter_write_rice_signed_block(
        bw:        *mut BitWriter,
        vals:      *const i32,
        nvals:     u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            const u32 mask1 = WORD_ALL_ONES << parameter; /* we val|=mask1 to set the stop bit above it... */
        const u32 mask2 = WORD_ALL_ONES >> (31-parameter); /* ...then mask off the bits above the stop bit with val&=mask2*/
        u32 uval;
        unsigned left;
        const unsigned lsbits = 1 + parameter;
        unsigned msbits;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);
        ASSERT(parameter < 8*sizeof(uint32_t)-1);
        /* WATCHOUT: code does not work with <32bit words; we can make things much faster with this assertion */
        ASSERT(BITS_PER_WORD >= 32);

        while(nvals) {
            /* fold signed to unsigned; actual formula is: negative(v)? -2v-1 : 2v */
            uval = (*vals<<1) ^ (*vals>>31);

            msbits = uval >> parameter;

            if(bw->bits && bw->bits + msbits + lsbits < BITS_PER_WORD) { /* i.e. if the whole thing fits in the current uint32_t */
                /* ^^^ if bw->bits is 0 then we may have filled the buffer and have no free uint32_t to work in */
                bw->bits = bw->bits + msbits + lsbits;
                uval |= mask1; /* set stop bit */
                uval &= mask2; /* mask off unused top bits */
                bw->accum <<= msbits + lsbits;
                bw->accum |= uval;
            }
            else {
                /* slightly pessimistic size check but faster than "<= bw->words + (bw->bits+msbits+lsbits+BITS_PER_WORD-1)/BITS_PER_WORD" */
                /* OPT: pessimism may cause flurry of false calls to grow_ which eat up all savings before it */
                if(bw->capacity <= bw->words + bw->bits + msbits + 1/*lsbits always fit in 1 uint32_t*/ && !bitwriter_grow_(bw, msbits+lsbits))
                    return false;

                if(msbits) {
                    /* first part gets to word alignment */
                    if(bw->bits) {
                        left = BITS_PER_WORD - bw->bits;
                        if(msbits < left) {
                            bw->accum <<= msbits;
                            bw->bits += msbits;
                            goto break1;
                        }
                        else {
                            bw->accum <<= left;
                            msbits -= left;
                            bw->buffer[bw->words++] = SWAP_BE_WORD_TO_HOST(bw->accum);
                            bw->bits = 0;
                        }
                    }
                    /* do whole words */
                    while(msbits >= BITS_PER_WORD) {
                        bw->buffer[bw->words++] = 0;
                        msbits -= BITS_PER_WORD;
                    }
                    /* do any leftovers */
                    if(msbits > 0) {
                        bw->accum = 0;
                        bw->bits = msbits;
                    }
                }
    break1:
                uval |= mask1; /* set stop bit */
                uval &= mask2; /* mask off unused top bits */

                left = BITS_PER_WORD - bw->bits;
                if(lsbits < left) {
                    bw->accum <<= lsbits;
                    bw->accum |= uval;
                    bw->bits += lsbits;
                }
                else {
                    /* if bw->bits == 0, left==BITS_PER_WORD which will always
                     * be > lsbits (because of previous assertions) so it would have
                     * triggered the (lsbits<left) case above.
                     */
                    ASSERT(bw->bits);
                    ASSERT(left < BITS_PER_WORD);
                    bw->accum <<= left;
                    bw->accum |= uval >> (bw->bits = lsbits - left);
                    bw->buffer[bw->words++] = SWAP_BE_WORD_TO_HOST(bw->accum);
                    bw->accum = uval;
                }
            }
            vals++;
            nvals--;
        }
        return true;
        */
}

#[cfg(UNUSED)]
pub fn bitwriter_write_golomb_signed(
        bw:        *mut BitWriter,
        val:       i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            unsigned total_bits, msbs, uval;
        unsigned k;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);
        ASSERT(parameter > 0);

        /* fold signed to unsigned */
        if(val < 0)
            uval = (unsigned)(((-(++val)) << 1) + 1);
        else
            uval = (unsigned)(val << 1);

        k = bitmath_ilog2(parameter);
        if(parameter == 1u<<k) {
            unsigned pattern;

            ASSERT(k <= 30);

            msbs = uval >> k;
            total_bits = 1 + k + msbs;
            pattern = 1 << k; /* the unary end bit */
            pattern |= (uval & ((1u<<k)-1)); /* the binary LSBs */

            if(total_bits <= 32) {
                if(!bitwriter_write_raw_uint32(bw, pattern, total_bits))
                    return false;
            }
            else {
                /* write the unary MSBs */
                if(!bitwriter_write_zeroes(bw, msbs))
                    return false;
                /* write the unary end bit and binary LSBs */
                if(!bitwriter_write_raw_uint32(bw, pattern, k+1))
                    return false;
            }
        }
        else {
            unsigned q, r, d;

            d = (1 << (k+1)) - parameter;
            q = uval / parameter;
            r = uval - (q * parameter);
            /* write the unary MSBs */
            if(!bitwriter_write_zeroes(bw, q))
                return false;
            /* write the unary end bit */
            if(!bitwriter_write_raw_uint32(bw, 1, 1))
                return false;
            /* write the binary LSBs */
            if(r >= d) {
                if(!bitwriter_write_raw_uint32(bw, r+d, k+1))
                    return false;
            }
            else {
                if(!bitwriter_write_raw_uint32(bw, r, k))
                    return false;
            }
        }
        return true;
        */
}

#[cfg(UNUSED)]
pub fn bitwriter_write_golomb_unsigned(
        bw:        *mut BitWriter,
        uval:      u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
            unsigned total_bits, msbs;
        unsigned k;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);
        ASSERT(parameter > 0);

        k = bitmath_ilog2(parameter);
        if(parameter == 1u<<k) {
            unsigned pattern;

            ASSERT(k <= 30);

            msbs = uval >> k;
            total_bits = 1 + k + msbs;
            pattern = 1 << k; /* the unary end bit */
            pattern |= (uval & ((1u<<k)-1)); /* the binary LSBs */

            if(total_bits <= 32) {
                if(!bitwriter_write_raw_uint32(bw, pattern, total_bits))
                    return false;
            }
            else {
                /* write the unary MSBs */
                if(!bitwriter_write_zeroes(bw, msbs))
                    return false;
                /* write the unary end bit and binary LSBs */
                if(!bitwriter_write_raw_uint32(bw, pattern, k+1))
                    return false;
            }
        }
        else {
            unsigned q, r, d;

            d = (1 << (k+1)) - parameter;
            q = uval / parameter;
            r = uval - (q * parameter);
            /* write the unary MSBs */
            if(!bitwriter_write_zeroes(bw, q))
                return false;
            /* write the unary end bit */
            if(!bitwriter_write_raw_uint32(bw, 1, 1))
                return false;
            /* write the binary LSBs */
            if(r >= d) {
                if(!bitwriter_write_raw_uint32(bw, r+d, k+1))
                    return false;
            }
            else {
                if(!bitwriter_write_raw_uint32(bw, r, k))
                    return false;
            }
        }
        return true;
        */
}

pub fn bitwriter_write_utf8_uint32(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
            bool ok = 1;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);

        ASSERT(!(val & 0x80000000)); /* this version only handles 31 bits */

        if(val < 0x80) {
            return bitwriter_write_raw_uint32(bw, val, 8);
        }
        else if(val < 0x800) {
            ok &= bitwriter_write_raw_uint32(bw, 0xC0 | (val>>6), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (val&0x3F), 8);
        }
        else if(val < 0x10000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xE0 | (val>>12), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (val&0x3F), 8);
        }
        else if(val < 0x200000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xF0 | (val>>18), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (val&0x3F), 8);
        }
        else if(val < 0x4000000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xF8 | (val>>24), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>18)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (val&0x3F), 8);
        }
        else {
            ok &= bitwriter_write_raw_uint32(bw, 0xFC | (val>>30), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>24)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>18)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | ((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (val&0x3F), 8);
        }

        return ok;
        */
}

pub fn bitwriter_write_utf8_uint64(
        bw:  *mut BitWriter,
        val: u64) -> bool {
    
    todo!();
        /*
            bool ok = 1;

        ASSERT(0 != bw);
        ASSERT(0 != bw->buffer);

        ASSERT(!(val & U64L(0xFFFFFFF000000000))); /* this version only handles 36 bits */

        if(val < 0x80) {
            return bitwriter_write_raw_uint32(bw, (u32)val, 8);
        }
        else if(val < 0x800) {
            ok &= bitwriter_write_raw_uint32(bw, 0xC0 | (u32)(val>>6), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }
        else if(val < 0x10000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xE0 | (u32)(val>>12), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }
        else if(val < 0x200000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xF0 | (u32)(val>>18), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }
        else if(val < 0x4000000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xF8 | (u32)(val>>24), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>18)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }
        else if(val < 0x80000000) {
            ok &= bitwriter_write_raw_uint32(bw, 0xFC | (u32)(val>>30), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>24)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>18)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }
        else {
            ok &= bitwriter_write_raw_uint32(bw, 0xFE, 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>30)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>24)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>18)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>12)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)((val>>6)&0x3F), 8);
            ok &= bitwriter_write_raw_uint32(bw, 0x80 | (u32)(val&0x3F), 8);
        }

        return ok;
        */
}

pub fn bitwriter_zero_pad_to_byte_boundary(bw: *mut BitWriter) -> bool {
    
    todo!();
        /*
            /* 0-pad to byte boundary */
        if(bw->bits & 7u)
            return bitwriter_write_zeroes(bw, 8 - (bw->bits & 7u));
        else
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
    extern bool bitwriter_write_zeroes(BitWriter *bw, unsigned bits);
    extern bool bitwriter_write_raw_int32(BitWriter *bw, i32 val, unsigned bits);
    extern bool bitwriter_write_raw_uint64(BitWriter *bw, u64 val, unsigned bits);
    extern bool bitwriter_write_raw_uint32_little_endian(BitWriter *bw, u32 val);
    extern bool bitwriter_write_byte_block(BitWriter *bw, const byte vals[], unsigned nvals);
    */
}
