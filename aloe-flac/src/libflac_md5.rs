/*!
  | This code implements the MD5 message-digest
  | algorithm.
  | 
  | The algorithm is due to Ron Rivest. This
  | code was written by Colin Plumb in 1993,
  | no copyright is claimed.
  | 
  | This code is in the public domain; do
  | with it what you wish.
  | 
  | Equivalent code is available from RSA
  | Data Security, Inc.
  | 
  | This code has been tested against that,
  | and is equivalent, except that you don't
  | need to include two pages of legalese
  | with every copy.
  | 
  | To compute the message digest of a chunk
  | of bytes, declare an
  | 
  | MD5Context structure, pass it to MD5Init,
  | call MD5Update as needed on buffers
  | full of bytes, and then call MD5Final,
  | which will fill a supplied 16-byte array
  | with the digest.
  | 
  | Changed so as no longer to depend on Colin
  | Plumb's `usual.h' header definitions;
  | now uses stuff from dpkg's config.h.
  | 
  | - Ian Jackson <ijackson@nyx.cs.du.edu>.
  | 
  | Still in the public domain.
  | 
  | Josh Coalson: made some changes to integrate
  | with libFLAC.
  | 
  | Still in the public domain.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/md5.c]

/*
  | The four core functions - F1 is optimized
  | somewhat
  |
  */

/**
  | #define F1(x, y, z) (x & y | ~x & z)
  |
  */
macro_rules! f1 {
    ($x:ident, $y:ident, $z:ident) => {
        /*
                (z ^ (x & (y ^ z)))
        */
    }
}

macro_rules! f2 {
    ($x:ident, $y:ident, $z:ident) => {
        /*
                F1(z, x, y)
        */
    }
}

macro_rules! f3 {
    ($x:ident, $y:ident, $z:ident) => {
        /*
                (x ^ y ^ z)
        */
    }
}

macro_rules! f4 {
    ($x:ident, $y:ident, $z:ident) => {
        /*
                (y ^ (x | ~z))
        */
    }
}

/**
  | This is the central step in the MD5 algorithm.
  |
  */
macro_rules! md5step {
    ($f:ident, 
     $w:ident, 
     $x:ident, 
     $y:ident, 
     $z:ident, 
     $in:ident, 
     $s:ident) => {
        /*
        
             (w += f(x,y,z) + in, w = (w<<s | w>>(32-s)) + x)
        */
    }
}

/**
  | The core of the MD5 algorithm, this alters
  | an existing MD5 hash to reflect the addition
  | of 16 longwords of new data. MD5Update
  | blocks the data and converts bytes into
  | longwords for this routine.
  |
  */
pub fn flac_md5transform(
        buf: [u32; 4],
        in_: [u32; 16])  {
    
    todo!();
        /*
            u32 a, b, c, d;

        a = buf[0];
        b = buf[1];
        c = buf[2];
        d = buf[3];

        MD5STEP(F1, a, b, c, d, in[0] + 0xd76aa478, 7);
        MD5STEP(F1, d, a, b, c, in[1] + 0xe8c7b756, 12);
        MD5STEP(F1, c, d, a, b, in[2] + 0x242070db, 17);
        MD5STEP(F1, b, c, d, a, in[3] + 0xc1bdceee, 22);
        MD5STEP(F1, a, b, c, d, in[4] + 0xf57c0faf, 7);
        MD5STEP(F1, d, a, b, c, in[5] + 0x4787c62a, 12);
        MD5STEP(F1, c, d, a, b, in[6] + 0xa8304613, 17);
        MD5STEP(F1, b, c, d, a, in[7] + 0xfd469501, 22);
        MD5STEP(F1, a, b, c, d, in[8] + 0x698098d8, 7);
        MD5STEP(F1, d, a, b, c, in[9] + 0x8b44f7af, 12);
        MD5STEP(F1, c, d, a, b, in[10] + 0xffff5bb1, 17);
        MD5STEP(F1, b, c, d, a, in[11] + 0x895cd7be, 22);
        MD5STEP(F1, a, b, c, d, in[12] + 0x6b901122, 7);
        MD5STEP(F1, d, a, b, c, in[13] + 0xfd987193, 12);
        MD5STEP(F1, c, d, a, b, in[14] + 0xa679438e, 17);
        MD5STEP(F1, b, c, d, a, in[15] + 0x49b40821, 22);

        MD5STEP(F2, a, b, c, d, in[1] + 0xf61e2562, 5);
        MD5STEP(F2, d, a, b, c, in[6] + 0xc040b340, 9);
        MD5STEP(F2, c, d, a, b, in[11] + 0x265e5a51, 14);
        MD5STEP(F2, b, c, d, a, in[0] + 0xe9b6c7aa, 20);
        MD5STEP(F2, a, b, c, d, in[5] + 0xd62f105d, 5);
        MD5STEP(F2, d, a, b, c, in[10] + 0x02441453, 9);
        MD5STEP(F2, c, d, a, b, in[15] + 0xd8a1e681, 14);
        MD5STEP(F2, b, c, d, a, in[4] + 0xe7d3fbc8, 20);
        MD5STEP(F2, a, b, c, d, in[9] + 0x21e1cde6, 5);
        MD5STEP(F2, d, a, b, c, in[14] + 0xc33707d6, 9);
        MD5STEP(F2, c, d, a, b, in[3] + 0xf4d50d87, 14);
        MD5STEP(F2, b, c, d, a, in[8] + 0x455a14ed, 20);
        MD5STEP(F2, a, b, c, d, in[13] + 0xa9e3e905, 5);
        MD5STEP(F2, d, a, b, c, in[2] + 0xfcefa3f8, 9);
        MD5STEP(F2, c, d, a, b, in[7] + 0x676f02d9, 14);
        MD5STEP(F2, b, c, d, a, in[12] + 0x8d2a4c8a, 20);

        MD5STEP(F3, a, b, c, d, in[5] + 0xfffa3942, 4);
        MD5STEP(F3, d, a, b, c, in[8] + 0x8771f681, 11);
        MD5STEP(F3, c, d, a, b, in[11] + 0x6d9d6122, 16);
        MD5STEP(F3, b, c, d, a, in[14] + 0xfde5380c, 23);
        MD5STEP(F3, a, b, c, d, in[1] + 0xa4beea44, 4);
        MD5STEP(F3, d, a, b, c, in[4] + 0x4bdecfa9, 11);
        MD5STEP(F3, c, d, a, b, in[7] + 0xf6bb4b60, 16);
        MD5STEP(F3, b, c, d, a, in[10] + 0xbebfbc70, 23);
        MD5STEP(F3, a, b, c, d, in[13] + 0x289b7ec6, 4);
        MD5STEP(F3, d, a, b, c, in[0] + 0xeaa127fa, 11);
        MD5STEP(F3, c, d, a, b, in[3] + 0xd4ef3085, 16);
        MD5STEP(F3, b, c, d, a, in[6] + 0x04881d05, 23);
        MD5STEP(F3, a, b, c, d, in[9] + 0xd9d4d039, 4);
        MD5STEP(F3, d, a, b, c, in[12] + 0xe6db99e5, 11);
        MD5STEP(F3, c, d, a, b, in[15] + 0x1fa27cf8, 16);
        MD5STEP(F3, b, c, d, a, in[2] + 0xc4ac5665, 23);

        MD5STEP(F4, a, b, c, d, in[0] + 0xf4292244, 6);
        MD5STEP(F4, d, a, b, c, in[7] + 0x432aff97, 10);
        MD5STEP(F4, c, d, a, b, in[14] + 0xab9423a7, 15);
        MD5STEP(F4, b, c, d, a, in[5] + 0xfc93a039, 21);
        MD5STEP(F4, a, b, c, d, in[12] + 0x655b59c3, 6);
        MD5STEP(F4, d, a, b, c, in[3] + 0x8f0ccc92, 10);
        MD5STEP(F4, c, d, a, b, in[10] + 0xffeff47d, 15);
        MD5STEP(F4, b, c, d, a, in[1] + 0x85845dd1, 21);
        MD5STEP(F4, a, b, c, d, in[8] + 0x6fa87e4f, 6);
        MD5STEP(F4, d, a, b, c, in[15] + 0xfe2ce6e0, 10);
        MD5STEP(F4, c, d, a, b, in[6] + 0xa3014314, 15);
        MD5STEP(F4, b, c, d, a, in[13] + 0x4e0811a1, 21);
        MD5STEP(F4, a, b, c, d, in[4] + 0xf7537e82, 6);
        MD5STEP(F4, d, a, b, c, in[11] + 0xbd3af235, 10);
        MD5STEP(F4, c, d, a, b, in[2] + 0x2ad7d2bb, 15);
        MD5STEP(F4, b, c, d, a, in[9] + 0xeb86d391, 21);

        buf[0] += a;
        buf[1] += b;
        buf[2] += c;
        buf[3] += d;
        */
}


/**
  @@@@@@ OPT: use bswap/intrinsics
  */
#[cfg(WORDS_BIGENDIAN)]
pub fn byte_swap(
        buf:   *mut u32,
        words: u32)  {
    
    todo!();
        /*
            u32 x;
        do {
            x = *buf;
            x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff);
            *buf++ = (x >> 16) | (x << 16);
        } while (--words);
        */
}

#[cfg(WORDS_BIGENDIAN)]
pub fn byte_swapx16(buf: *mut u32)  {
    
    todo!();
        /*
            u32 x;

        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf++ = (x >> 16) | (x << 16);
        x = *buf; x = ((x << 8) & 0xff00ff00) | ((x >> 8) & 0x00ff00ff); *buf   = (x >> 16) | (x << 16);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
macro_rules! byteswap    { ($buf:ident, $words:ident) => { } }

#[cfg(not(WORDS_BIGENDIAN))]
macro_rules! byteswapx16 { ($buf:ident) => { } }

/**
  | Update context to reflect the concatenation
  | of another buffer full of bytes.
  |
  */
pub fn flac_md5update(
        ctx: *mut MD5Context,
        buf: *const u8,
        len: u32)  {
    
    todo!();
        /*
            u32 t;

        /* Update byte count */

        t = ctx->bytes[0];
        if ((ctx->bytes[0] = t + len) < t)
            ctx->bytes[1]++;    /* Carry from low to high */

        t = 64 - (t & 0x3f);    /* Space available in ctx->in (at least 1) */
        if (t > len) {
            memcpy((byte *)ctx->in + 64 - t, buf, len);
            return;
        }
        /* First chunk is an odd size */
        memcpy((byte *)ctx->in + 64 - t, buf, t);
        byteSwapX16(ctx->in);
        MD5Transform(ctx->buf, ctx->in);
        buf += t;
        len -= t;

        /* Process data in 64-byte chunks */
        while (len >= 64) {
            memcpy(ctx->in, buf, 64);
            byteSwapX16(ctx->in);
            MD5Transform(ctx->buf, ctx->in);
            buf += 64;
            len -= 64;
        }

        /* Handle any remaining bytes of data. */
        memcpy(ctx->in, buf, len);
        */
}

/**
  | Start MD5 accumulation. Set bit count
  | to 0 and buffer to mysterious initialization
  | constants.
  |
  */
pub fn flac_md5init(ctx: *mut MD5Context)  {
    
    todo!();
        /*
            ctx->buf[0] = 0x67452301;
        ctx->buf[1] = 0xefcdab89;
        ctx->buf[2] = 0x98badcfe;
        ctx->buf[3] = 0x10325476;

        ctx->bytes[0] = 0;
        ctx->bytes[1] = 0;

        ctx->internal_buf.p8= 0;
        ctx->capacity = 0;
        */
}

/**
  | Final wrapup - pad to 64-byte boundary
  | with the bit pattern 1 0* (64-bit count
  | of bits processed, MSB-first)
  |
  */
pub fn flac_md5final(
        digest: [u8; 16],
        ctx:    *mut MD5Context)  {
    
    todo!();
        /*
            int count = ctx->bytes[0] & 0x3f;   /* Number of bytes in ctx->in */
        byte *p = (byte *)ctx->in + count;

        /* Set the first char of padding to 0x80.  There is always room. */
        *p++ = 0x80;

        /* Bytes of padding needed to make 56 bytes (-8..55) */
        count = 56 - 1 - count;

        if (count < 0) {    /* Padding forces an extra block */
            memset(p, 0, count + 8);
            byteSwapX16(ctx->in);
            MD5Transform(ctx->buf, ctx->in);
            p = (byte *)ctx->in;
            count = 56;
        }
        memset(p, 0, count);
        byteSwap(ctx->in, 14);

        /* Append length in bits and transform */
        ctx->in[14] = ctx->bytes[0] << 3;
        ctx->in[15] = ctx->bytes[1] << 3 | ctx->bytes[0] >> 29;
        MD5Transform(ctx->buf, ctx->in);

        byteSwap(ctx->buf, 4);
        memcpy(digest, ctx->buf, 16);
        if (0 != ctx->internal_buf.p8) {
            free(ctx->internal_buf.p8);
            ctx->internal_buf.p8= 0;
            ctx->capacity = 0;
        }
        memset(ctx, 0, sizeof(*ctx));   /* In case it's sensitive */
        */
}

/**
  | Convert the incoming audio signal to
  | a byte stream
  |
  */
pub fn format_input(
        mbuf:             *mut multibyte,
        signal:           *const &[i32],
        channels:         u32,
        samples:          u32,
        bytes_per_sample: u32)  {
    
    todo!();
        /*
            byte *buf_ = mbuf->p8;
        int16 *buf16 = mbuf->p16;
        i32 *buf32 = mbuf->p32;
        i32 a_word;
        unsigned channel, sample;

        /* Storage in the output buffer, buf, is little endian. */

    #define BYTES_CHANNEL_SELECTOR(bytes, channels)   (bytes * 100 + channels)

        /* First do the most commonly used combinations. */
        switch (BYTES_CHANNEL_SELECTOR (bytes_per_sample, channels)) {
            /* One byte per sample. */
            case (BYTES_CHANNEL_SELECTOR (1, 1)):
                for (sample = 0; sample < samples; sample++)
                    *buf_++ = signal[0][sample];
                return;

            case (BYTES_CHANNEL_SELECTOR (1, 2)):
                for (sample = 0; sample < samples; sample++) {
                    *buf_++ = signal[0][sample];
                    *buf_++ = signal[1][sample];
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (1, 4)):
                for (sample = 0; sample < samples; sample++) {
                    *buf_++ = signal[0][sample];
                    *buf_++ = signal[1][sample];
                    *buf_++ = signal[2][sample];
                    *buf_++ = signal[3][sample];
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (1, 6)):
                for (sample = 0; sample < samples; sample++) {
                    *buf_++ = signal[0][sample];
                    *buf_++ = signal[1][sample];
                    *buf_++ = signal[2][sample];
                    *buf_++ = signal[3][sample];
                    *buf_++ = signal[4][sample];
                    *buf_++ = signal[5][sample];
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (1, 8)):
                for (sample = 0; sample < samples; sample++) {
                    *buf_++ = signal[0][sample];
                    *buf_++ = signal[1][sample];
                    *buf_++ = signal[2][sample];
                    *buf_++ = signal[3][sample];
                    *buf_++ = signal[4][sample];
                    *buf_++ = signal[5][sample];
                    *buf_++ = signal[6][sample];
                    *buf_++ = signal[7][sample];
                }
                return;

            /* Two bytes per sample. */
            case (BYTES_CHANNEL_SELECTOR (2, 1)):
                for (sample = 0; sample < samples; sample++)
                    *buf16++ = H2LE_16(signal[0][sample]);
                return;

            case (BYTES_CHANNEL_SELECTOR (2, 2)):
                for (sample = 0; sample < samples; sample++) {
                    *buf16++ = H2LE_16(signal[0][sample]);
                    *buf16++ = H2LE_16(signal[1][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (2, 4)):
                for (sample = 0; sample < samples; sample++) {
                    *buf16++ = H2LE_16(signal[0][sample]);
                    *buf16++ = H2LE_16(signal[1][sample]);
                    *buf16++ = H2LE_16(signal[2][sample]);
                    *buf16++ = H2LE_16(signal[3][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (2, 6)):
                for (sample = 0; sample < samples; sample++) {
                    *buf16++ = H2LE_16(signal[0][sample]);
                    *buf16++ = H2LE_16(signal[1][sample]);
                    *buf16++ = H2LE_16(signal[2][sample]);
                    *buf16++ = H2LE_16(signal[3][sample]);
                    *buf16++ = H2LE_16(signal[4][sample]);
                    *buf16++ = H2LE_16(signal[5][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (2, 8)):
                for (sample = 0; sample < samples; sample++) {
                    *buf16++ = H2LE_16(signal[0][sample]);
                    *buf16++ = H2LE_16(signal[1][sample]);
                    *buf16++ = H2LE_16(signal[2][sample]);
                    *buf16++ = H2LE_16(signal[3][sample]);
                    *buf16++ = H2LE_16(signal[4][sample]);
                    *buf16++ = H2LE_16(signal[5][sample]);
                    *buf16++ = H2LE_16(signal[6][sample]);
                    *buf16++ = H2LE_16(signal[7][sample]);
                }
                return;

            /* Three bytes per sample. */
            case (BYTES_CHANNEL_SELECTOR (3, 1)):
                for (sample = 0; sample < samples; sample++) {
                    a_word = signal[0][sample];
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word;
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (3, 2)):
                for (sample = 0; sample < samples; sample++) {
                    a_word = signal[0][sample];
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word;
                    a_word = signal[1][sample];
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word; a_word >>= 8;
                    *buf_++ = (byte)a_word;
                }
                return;

            /* Four bytes per sample. */
            case (BYTES_CHANNEL_SELECTOR (4, 1)):
                for (sample = 0; sample < samples; sample++)
                    *buf32++ = H2LE_32(signal[0][sample]);
                return;

            case (BYTES_CHANNEL_SELECTOR (4, 2)):
                for (sample = 0; sample < samples; sample++) {
                    *buf32++ = H2LE_32(signal[0][sample]);
                    *buf32++ = H2LE_32(signal[1][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (4, 4)):
                for (sample = 0; sample < samples; sample++) {
                    *buf32++ = H2LE_32(signal[0][sample]);
                    *buf32++ = H2LE_32(signal[1][sample]);
                    *buf32++ = H2LE_32(signal[2][sample]);
                    *buf32++ = H2LE_32(signal[3][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (4, 6)):
                for (sample = 0; sample < samples; sample++) {
                    *buf32++ = H2LE_32(signal[0][sample]);
                    *buf32++ = H2LE_32(signal[1][sample]);
                    *buf32++ = H2LE_32(signal[2][sample]);
                    *buf32++ = H2LE_32(signal[3][sample]);
                    *buf32++ = H2LE_32(signal[4][sample]);
                    *buf32++ = H2LE_32(signal[5][sample]);
                }
                return;

            case (BYTES_CHANNEL_SELECTOR (4, 8)):
                for (sample = 0; sample < samples; sample++) {
                    *buf32++ = H2LE_32(signal[0][sample]);
                    *buf32++ = H2LE_32(signal[1][sample]);
                    *buf32++ = H2LE_32(signal[2][sample]);
                    *buf32++ = H2LE_32(signal[3][sample]);
                    *buf32++ = H2LE_32(signal[4][sample]);
                    *buf32++ = H2LE_32(signal[5][sample]);
                    *buf32++ = H2LE_32(signal[6][sample]);
                    *buf32++ = H2LE_32(signal[7][sample]);
                }
                return;

            default:
                break;
        }

        /* General version. */
        switch (bytes_per_sample) {
            case 1:
                for (sample = 0; sample < samples; sample++)
                    for (channel = 0; channel < channels; channel++)
                        *buf_++ = signal[channel][sample];
                return;

            case 2:
                for (sample = 0; sample < samples; sample++)
                    for (channel = 0; channel < channels; channel++)
                        *buf16++ = H2LE_16(signal[channel][sample]);
                return;

            case 3:
                for (sample = 0; sample < samples; sample++)
                    for (channel = 0; channel < channels; channel++) {
                        a_word = signal[channel][sample];
                        *buf_++ = (byte)a_word; a_word >>= 8;
                        *buf_++ = (byte)a_word; a_word >>= 8;
                        *buf_++ = (byte)a_word;
                    }
                return;

            case 4:
                for (sample = 0; sample < samples; sample++)
                    for (channel = 0; channel < channels; channel++)
                        *buf32++ = H2LE_32(signal[channel][sample]);
                return;

            default:
                break;
        }
        */
}

/**
  | Convert the incoming audio signal to
  | a byte stream and MD5Update it.
  |
  */
pub fn flac_md5accumulate(
        ctx:              *mut MD5Context,
        signal:           *const &[i32],
        channels:         u32,
        samples:          u32,
        bytes_per_sample: u32) -> bool {
    
    todo!();
        /*
            const size_t bytes_needed = (size_t)channels * (size_t)samples * (size_t)bytes_per_sample;

        /* overflow check */
        if ((size_t)channels > SIZE_MAX / (size_t)bytes_per_sample)
            return false;
        if ((size_t)channels * (size_t)bytes_per_sample > SIZE_MAX / (size_t)samples)
            return false;

        if (ctx->capacity < bytes_needed) {
            byte *tmp = (byte*) realloc(ctx->internal_buf.p8, bytes_needed);
            if (0 == tmp) {
                free(ctx->internal_buf.p8);
                if (0 == (ctx->internal_buf.p8= (byte*) safe_malloc_(bytes_needed)))
                    return false;
            }
            else
                ctx->internal_buf.p8= tmp;
            ctx->capacity = bytes_needed;
        }

        format_input_(&ctx->internal_buf, signal, channels, samples, bytes_per_sample);

        MD5Update(ctx, ctx->internal_buf.p8, bytes_needed);

        return true;
        */
}
