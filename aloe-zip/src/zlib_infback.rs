/*!
  | infback.c -- inflate using a call-back
  | interface Copyright (C) 1995-2005
  | Mark Adler For conditions of distribution
  | and use, see copyright notice in zlib.h
  | 
  | This code is largely copied from inflate.c.
  | Normally either infback.o or inflate.o
  | would be linked into an application--not
  | both. The interface with inffast.c
  | is retained so that optimized assembler-coded
  | versions of inflate_fast() can be used
  | with either inflate.c or infback.c.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/infback.c]

/**
  | strm provides memory allocation functions
  | in zalloc and zfree, or Z_NULL to use
  | the library memory allocation functions.
  | 
  | windowBits is in the range 8..15, and
  | window is a user-supplied window and
  | output buffer that is 2**windowBits
  | bytes.
  |
  */
pub fn inflate_back_init(
        strm:        z_streamp,
        window_bits: i32,
        window:      *mut u8,
        version:     *const u8,
        stream_size: i32) -> i32 {
    
    todo!();
    /*
        struct inflate_state   *state;

        if (version == Z_NULL || version[0] != ZLIB_VERSION[0] ||
            stream_size != (int)(sizeof(z_stream)))
            return Z_VERSION_ERROR;
        if (strm == Z_NULL || window == Z_NULL ||
            windowBits < 8 || windowBits > 15)
            return Z_STREAM_ERROR;
        strm->msg = Z_NULL;                 /* in case we return an error */
        if (strm->zalloc == (alloc_func)0) {
            strm->zalloc = zcalloc;
            strm->opaque = (voidpf)0;
        }
        if (strm->zfree == (free_func)0) strm->zfree = zcfree;
        state = (struct inflate_state   *)ZALLOC(strm, 1,
                                                   sizeof(struct inflate_state));
        if (state == Z_NULL) return Z_MEM_ERROR;
        Tracev((stderr, "inflate: allocated\n"));
        strm->state = (struct internal_state   *)state;
        state->dmax = 32768U;
        state->wbits = windowBits;
        state->wsize = 1U << windowBits;
        state->window = window;
        state->write = 0;
        state->whave = 0;
        return Z_OK;
    */
}

/**
  | Return state with length and distance
  | decoding tables and index sizes set
  | to fixed code decoding. Normally this
  | returns fixed tables from inffixed.h.
  | 
  | If BUILDFIXED is defined, then instead
  | this routine builds the tables the first
  | time it's called, and returns those
  | tables the first time and thereafter.
  | This reduces the size of the code by about
  | 2K bytes, in exchange for a little execution
  | time. However, BUILDFIXED should not
  | be used for threaded applications,
  | since the rewriting of the tables and
  | virgin may not be thread-safe.
  |
  */
pub fn fixedtables1(state: *mut InflateState)  {
    
    todo!();
    /*
        #ifdef BUILDFIXED
        static int virgin = 1;
        static code *lenfix, *distfix;
        static code fixed[544];

        /* build fixed huffman tables if first call (may not be thread safe) */
        if (virgin) {
            unsigned sym, bits;
            static code *next;

            /* literal/length table */
            sym = 0;
            while (sym < 144) state->lens[sym++] = 8;
            while (sym < 256) state->lens[sym++] = 9;
            while (sym < 280) state->lens[sym++] = 7;
            while (sym < 288) state->lens[sym++] = 8;
            next = fixed;
            lenfix = next;
            bits = 9;
            inflate_table(LENS, state->lens, 288, &(next), &(bits), state->work);

            /* distance table */
            sym = 0;
            while (sym < 32) state->lens[sym++] = 5;
            distfix = next;
            bits = 5;
            inflate_table(DISTS, state->lens, 32, &(next), &(bits), state->work);

            /* do this just once */
            virgin = 0;
        }
    #else /* !BUILDFIXED */
    #   include "inffixed.h"
    #endif /* BUILDFIXED */
        state->lencode = lenfix;
        state->lenbits = 9;
        state->distcode = distfix;
        state->distbits = 5;
    */
}

/* ------------ Macros for inflateBack():  ------------ */

/**
  | Load returned state from inflate_fast()
  |
  */
macro_rules! load {
    () => {
        /*
        
            do { 
                put = strm->next_out; 
                left = strm->avail_out; 
                next = strm->next_in; 
                have = strm->avail_in; 
                hold = state->hold; 
                bits = state->bits; 
            } while (0)
        */
    }
}

/**
  | Set state from registers for inflate_fast()
  |
  */
macro_rules! restore {
    () => {
        /*
        
            do { 
                strm->next_out = put; 
                strm->avail_out = left; 
                strm->next_in = next; 
                strm->avail_in = have; 
                state->hold = hold; 
                state->bits = bits; 
            } while (0)
        */
    }
}

/**
  | Clear the input bit accumulator
  |
  */
macro_rules! initbits {
    () => {
        /*
        
            do { 
                hold = 0; 
                bits = 0; 
            } while (0)
        */
    }
}

/**
  | Assure that some input is available.
  | If input is requested, but denied, then
  | return a Z_BUF_ERROR from inflateBack().
  |
  */
macro_rules! pull {
    () => {
        /*
        
            do { 
                if (have == 0) { 
                    have = in(in_desc, &next); 
                    if (have == 0) { 
                        next = Z_NULL; 
                        ret = Z_BUF_ERROR; 
                        goto inf_leave; 
                    } 
                } 
            } while (0)
        */
    }
}

/**
  | Get a byte of input into the bit accumulator,
  | or return from inflateBack() with an
  | error if there is no input available.
  |
  */
macro_rules! pullbyte {
    () => {
        /*
        
            do { 
                PULL(); 
                have--; 
                hold += (unsigned long)(*next++) << bits; 
                bits += 8; 
            } while (0)
        */
    }
}

/**
  | Assure that there are at least n bits
  | in the bit accumulator. If there is not
  | enough available input to do that, then
  | return from inflateBack() with an error.
  |
  */
macro_rules! needbits {
    ($n:ident) => {
        /*
        
            do { 
                while (bits < (unsigned)(n)) 
                    PULLBYTE(); 
            } while (0)
        */
    }
}

/**
  | Return the low n bits of the bit accumulator
  | (n < 16)
  |
  */
macro_rules! bits {
    ($n:ident) => {
        /*
        
            ((unsigned)hold & ((1U << (n)) - 1))
        */
    }
}

/**
  | Remove n bits from the bit accumulator
  |
  */
macro_rules! dropbits {
    ($n:ident) => {
        /*
        
            do { 
                hold >>= (n); 
                bits -= (unsigned)(n); 
            } while (0)
        */
    }
}

/**
  | Remove zero to seven bits as needed to
  | go to a byte boundary
  |
  */
macro_rules! bytebits {
    () => {
        /*
        
            do { 
                hold >>= bits & 7; 
                bits -= bits & 7; 
            } while (0)
        */
    }
}

/**
  | Assure that some output space is available,
  | by writing out the window if it's full.
  | If the write fails, return from inflateBack()
  | with a Z_BUF_ERROR.
  |
  */
macro_rules! room {
    () => {
        /*
        
            do { 
                if (left == 0) { 
                    put = state->window; 
                    left = state->wsize; 
                    state->whave = left; 
                    if (out(out_desc, put, left)) { 
                        ret = Z_BUF_ERROR; 
                        goto inf_leave; 
                    } 
                } 
            } while (0)
        */
    }
}

/**
  | strm provides the memory allocation
  | functions and window buffer on input,
  | and provides information on the unused
  | input on return. For Z_DATA_ERROR returns,
  | strm will also provide an error message.
  | 
  | in() and out() are the call-back input
  | and output functions. When inflateBack()
  | needs more input, it calls in(). When
  | inflateBack() has filled the window
  | with output, or when it completes with
  | data in the window, it calls out() to
  | write out the data. The application
  | must not change the provided input until
  | in() is called again or inflateBack()
  | returns. The application must not change
  | the window/output buffer until inflateBack()
  | returns.
  | 
  | in() and out() are called with a descriptor
  | parameter provided in the inflateBack()
  | call. This parameter can be a structure
  | that provides the information required
  | to do the read or write, as well as accumulated
  | information on the input and output
  | such as totals and check values.
  | 
  | in() should return zero on failure.
  | out() should return non-zero on failure.
  | If either in() or out() fails, than inflateBack()
  | returns a Z_BUF_ERROR. strm->next_in
  | can be checked for Z_NULL to see whether
  | it was in() or out() that caused in the
  | error. Otherwise, inflateBack() returns
  | Z_STREAM_END on success, Z_DATA_ERROR
  | for an deflate format error, or Z_MEM_ERROR
  | if it could not allocate memory for the
  | state. inflateBack() can also return
  | Z_STREAM_ERROR if the input parameters
  | are not correct, i.e. strm is Z_NULL
  | or the state was not initialized.
  |
  */
pub fn inflate_back(
        strm:     z_streamp,
        in_:      InFunc,
        in_desc:  *mut c_void,
        out:      OutFunc,
        out_desc: *mut c_void) -> i32 {
    
    todo!();
    /*
        struct inflate_state   *state;
        unsigned char   *next;    /* next input */
        unsigned char   *put;     /* next output */
        unsigned have, left;        /* available input and output */
        unsigned long hold;         /* bit buffer */
        unsigned bits;              /* bits in bit buffer */
        unsigned copy;              /* number of stored or match bytes to copy */
        unsigned char   *from;    /* where to copy match bytes from */
        code thisx;                  /* current decoding table entry */
        code last;                  /* parent table entry */
        unsigned len;               /* length to copy for repeats, bits to drop */
        int ret;                    /* return code */
        static const unsigned short order[19] = /* permutation of code lengths */
            {16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15};

        /* Check that the strm exists and that the state was initialized */
        if (strm == Z_NULL || strm->state == Z_NULL)
            return Z_STREAM_ERROR;
        state = (struct inflate_state   *)strm->state;

        /* Reset the state */
        strm->msg = Z_NULL;
        state->mode = TYPE;
        state->last = 0;
        state->whave = 0;
        next = strm->next_in;
        have = next != Z_NULL ? strm->avail_in : 0;
        hold = 0;
        bits = 0;
        put = state->window;
        left = state->wsize;

        /* Inflate until end of block marked as last */
        for (;;)
            switch (state->mode) {
            case TYPE:
                /* determine and dispatch block type */
                if (state->last) {
                    BYTEBITS();
                    state->mode = DONE;
                    break;
                }
                NEEDBITS(3);
                state->last = BITS(1);
                DROPBITS(1);
                switch (BITS(2)) {
                case 0:                             /* stored block */
                    Tracev((stderr, "inflate:     stored block%s\n",
                            state->last ? " (last)" : ""));
                    state->mode = STORED;
                    break;
                case 1:                             /* fixed block */
                    fixedtables1(state);
                    Tracev((stderr, "inflate:     fixed codes block%s\n",
                            state->last ? " (last)" : ""));
                    state->mode = LEN;              /* decode codes */
                    break;
                case 2:                             /* dynamic block */
                    Tracev((stderr, "inflate:     dynamic codes block%s\n",
                            state->last ? " (last)" : ""));
                    state->mode = TABLE;
                    break;
                case 3:
                    strm->msg = (char *)"invalid block type";
                    state->mode = BAD;
                }
                DROPBITS(2);
                break;

            case STORED:
                /* get and verify stored block length */
                BYTEBITS();                         /* go to byte boundary */
                NEEDBITS(32);
                if ((hold & 0xffff) != ((hold >> 16) ^ 0xffff)) {
                    strm->msg = (char *)"invalid stored block lengths";
                    state->mode = BAD;
                    break;
                }
                state->length = (unsigned)hold & 0xffff;
                Tracev((stderr, "inflate:       stored length %u\n",
                        state->length));
                INITBITS();

                /* copy stored block from input to output */
                while (state->length != 0) {
                    copy = state->length;
                    PULL();
                    ROOM();
                    if (copy > have) copy = have;
                    if (copy > left) copy = left;
                    zmemcpy(put, next, copy);
                    have -= copy;
                    next += copy;
                    left -= copy;
                    put += copy;
                    state->length -= copy;
                }
                Tracev((stderr, "inflate:       stored end\n"));
                state->mode = TYPE;
                break;

            case TABLE:
                /* get dynamic table entries descriptor */
                NEEDBITS(14);
                state->nlen = BITS(5) + 257;
                DROPBITS(5);
                state->ndist = BITS(5) + 1;
                DROPBITS(5);
                state->ncode = BITS(4) + 4;
                DROPBITS(4);
    #ifndef PKZIP_BUG_WORKAROUND
                if (state->nlen > 286 || state->ndist > 30) {
                    strm->msg = (char *)"too many length or distance symbols";
                    state->mode = BAD;
                    break;
                }
    #endif
                Tracev((stderr, "inflate:       table sizes ok\n"));

                /* get code length code lengths (not a typo) */
                state->have = 0;
                while (state->have < state->ncode) {
                    NEEDBITS(3);
                    state->lens[order[state->have++]] = (unsigned short)BITS(3);
                    DROPBITS(3);
                }
                while (state->have < 19)
                    state->lens[order[state->have++]] = 0;
                state->next = state->codes;
                state->lencode = (code const   *)(state->next);
                state->lenbits = 7;
                ret = inflate_table(CODES, state->lens, 19, &(state->next),
                                    &(state->lenbits), state->work);
                if (ret) {
                    strm->msg = (char *)"invalid code lengths set";
                    state->mode = BAD;
                    break;
                }
                Tracev((stderr, "inflate:       code lengths ok\n"));

                /* get length and distance code code lengths */
                state->have = 0;
                while (state->have < state->nlen + state->ndist) {
                    for (;;) {
                        thisx = state->lencode[BITS(state->lenbits)];
                        if ((unsigned)(thisx.bits) <= bits) break;
                        PULLBYTE();
                    }
                    if (thisx.val < 16) {
                        NEEDBITS(thisx.bits);
                        DROPBITS(thisx.bits);
                        state->lens[state->have++] = thisx.val;
                    }
                    else {
                        if (thisx.val == 16) {
                            NEEDBITS(thisx.bits + 2);
                            DROPBITS(thisx.bits);
                            if (state->have == 0) {
                                strm->msg = (char *)"invalid bit length repeat";
                                state->mode = BAD;
                                break;
                            }
                            len = (unsigned)(state->lens[state->have - 1]);
                            copy = 3 + BITS(2);
                            DROPBITS(2);
                        }
                        else if (thisx.val == 17) {
                            NEEDBITS(thisx.bits + 3);
                            DROPBITS(thisx.bits);
                            len = 0;
                            copy = 3 + BITS(3);
                            DROPBITS(3);
                        }
                        else {
                            NEEDBITS(thisx.bits + 7);
                            DROPBITS(thisx.bits);
                            len = 0;
                            copy = 11 + BITS(7);
                            DROPBITS(7);
                        }
                        if (state->have + copy > state->nlen + state->ndist) {
                            strm->msg = (char *)"invalid bit length repeat";
                            state->mode = BAD;
                            break;
                        }
                        while (copy--)
                            state->lens[state->have++] = (unsigned short)len;
                    }
                }

                /* handle error breaks in while */
                if (state->mode == BAD) break;

                /* build code tables */
                state->next = state->codes;
                state->lencode = (code const   *)(state->next);
                state->lenbits = 9;
                ret = inflate_table(LENS, state->lens, state->nlen, &(state->next),
                                    &(state->lenbits), state->work);
                if (ret) {
                    strm->msg = (char *)"invalid literal/lengths set";
                    state->mode = BAD;
                    break;
                }
                state->distcode = (code const   *)(state->next);
                state->distbits = 6;
                ret = inflate_table(DISTS, state->lens + state->nlen, state->ndist,
                                &(state->next), &(state->distbits), state->work);
                if (ret) {
                    strm->msg = (char *)"invalid distances set";
                    state->mode = BAD;
                    break;
                }
                Tracev((stderr, "inflate:       codes ok\n"));
                state->mode = LEN;

            case LEN:
                /* use inflate_fast() if we have enough input and output */
                if (have >= 6 && left >= 258) {
                    RESTORE();
                    if (state->whave < state->wsize)
                        state->whave = state->wsize - left;
                    inflate_fast(strm, state->wsize);
                    LOAD();
                    break;
                }

                /* get a literal, length, or end-of-block code */
                for (;;) {
                    thisx = state->lencode[BITS(state->lenbits)];
                    if ((unsigned)(thisx.bits) <= bits) break;
                    PULLBYTE();
                }
                if (thisx.op && (thisx.op & 0xf0) == 0) {
                    last = thisx;
                    for (;;) {
                        thisx = state->lencode[last.val +
                                (BITS(last.bits + last.op) >> last.bits)];
                        if ((unsigned)(last.bits + thisx.bits) <= bits) break;
                        PULLBYTE();
                    }
                    DROPBITS(last.bits);
                }
                DROPBITS(thisx.bits);
                state->length = (unsigned)thisx.val;

                /* process literal */
                if (thisx.op == 0) {
                    Tracevv((stderr, thisx.val >= 0x20 && thisx.val < 0x7f ?
                            "inflate:         literal '%c'\n" :
                            "inflate:         literal 0x%02x\n", thisx.val));
                    ROOM();
                    *put++ = (unsigned char)(state->length);
                    left--;
                    state->mode = LEN;
                    break;
                }

                /* process end of block */
                if (thisx.op & 32) {
                    Tracevv((stderr, "inflate:         end of block\n"));
                    state->mode = TYPE;
                    break;
                }

                /* invalid code */
                if (thisx.op & 64) {
                    strm->msg = (char *)"invalid literal/length code";
                    state->mode = BAD;
                    break;
                }

                /* length code -- get extra bits, if any */
                state->extra = (unsigned)(thisx.op) & 15;
                if (state->extra != 0) {
                    NEEDBITS(state->extra);
                    state->length += BITS(state->extra);
                    DROPBITS(state->extra);
                }
                Tracevv((stderr, "inflate:         length %u\n", state->length));

                /* get distance code */
                for (;;) {
                    thisx = state->distcode[BITS(state->distbits)];
                    if ((unsigned)(thisx.bits) <= bits) break;
                    PULLBYTE();
                }
                if ((thisx.op & 0xf0) == 0) {
                    last = thisx;
                    for (;;) {
                        thisx = state->distcode[last.val +
                                (BITS(last.bits + last.op) >> last.bits)];
                        if ((unsigned)(last.bits + thisx.bits) <= bits) break;
                        PULLBYTE();
                    }
                    DROPBITS(last.bits);
                }
                DROPBITS(thisx.bits);
                if (thisx.op & 64) {
                    strm->msg = (char *)"invalid distance code";
                    state->mode = BAD;
                    break;
                }
                state->offset = (unsigned)thisx.val;

                /* get distance extra bits, if any */
                state->extra = (unsigned)(thisx.op) & 15;
                if (state->extra != 0) {
                    NEEDBITS(state->extra);
                    state->offset += BITS(state->extra);
                    DROPBITS(state->extra);
                }
                if (state->offset > state->wsize - (state->whave < state->wsize ?
                                                    left : 0)) {
                    strm->msg = (char *)"invalid distance too far back";
                    state->mode = BAD;
                    break;
                }
                Tracevv((stderr, "inflate:         distance %u\n", state->offset));

                /* copy match from window to output */
                do {
                    ROOM();
                    copy = state->wsize - state->offset;
                    if (copy < left) {
                        from = put + copy;
                        copy = left - copy;
                    }
                    else {
                        from = put - state->offset;
                        copy = left;
                    }
                    if (copy > state->length) copy = state->length;
                    state->length -= copy;
                    left -= copy;
                    do {
                        *put++ = *from++;
                    } while (--copy);
                } while (state->length != 0);
                break;

            case DONE:
                /* inflate stream terminated properly -- write leftover output */
                ret = Z_STREAM_END;
                if (left < state->wsize) {
                    if (out(out_desc, state->window, state->wsize - left))
                        ret = Z_BUF_ERROR;
                }
                goto inf_leave;

            case BAD:
                ret = Z_DATA_ERROR;
                goto inf_leave;

            default:                /* can't happen, but makes compilers happy */
                ret = Z_STREAM_ERROR;
                goto inf_leave;
            }

        /* Return unused input */
      inf_leave:
        strm->next_in = next;
        strm->avail_in = have;
        return ret;
    */
}

pub fn inflate_back_end(strm: z_streamp) -> i32 {
    
    todo!();
    /*
        if (strm == Z_NULL || strm->state == Z_NULL || strm->zfree == (free_func)0)
            return Z_STREAM_ERROR;
        ZFREE(strm, strm->state);
        strm->state = Z_NULL;
        Tracev((stderr, "inflate: end\n"));
        return Z_OK;
    */
}
