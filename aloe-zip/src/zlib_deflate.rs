/*!
  | ALGORITHM
  | 
  | The "deflation" process depends on
  | being able to identify portions of the
  | input text which are identical to earlier
  | input (within a sliding window trailing
  | behind the input currently being processed).
  | 
  | The most straightforward technique
  | turns out to be the fastest for most input
  | files: try all possible matches and
  | select the longest. The key feature
  | of this algorithm is that insertions
  | into the string dictionary are very
  | simple and thus fast, and deletions
  | are avoided completely. Insertions
  | are performed at each input character,
  | whereas string matches are performed
  | only when the previous match ends. So
  | it is preferable to spend more time in
  | matches to allow very fast string insertions
  | and avoid deletions. The matching algorithm
  | for small strings is inspired from that
  | of Rabin & Karp. A brute force approach
  | is used to find longer strings when a
  | small match has been found. A similar
  | algorithm is used in comic (by Jan-Mark
  | Wams) and freeze (by Leonid Broukhis).
  | 
  | A previous version of this file used
  | a more sophisticated algorithm (by
  | Fiala and Greene) which is guaranteed
  | to run in linear amortized time, but
  | has a larger average cost, uses more
  | memory and is patented. However the
  | F&G algorithm may be faster for some
  | highly redundant files if the parameter
  | max_chain_length (described below)
  | is too large.
  | 
  | ACKNOWLEDGEMENTS
  | 
  | The idea of lazy evaluation of matches
  | is due to Jan-Mark Wams, and I found it
  | in 'freeze' written by Leonid Broukhis.
  | Thanks to many people for bug reports
  | and testing.
  | 
  | REFERENCES
  | 
  | Deutsch, L.P.,"DEFLATE Compressed
  | Data Format Specification". Available
  | in http://www.ietf.org/rfc/rfc1951.txt
  | 
  | A description of the Rabin and Karp algorithm
  | is given in the book "Algorithms" by
  | R. Sedgewick, Addison-Wesley, p252.
  | 
  | Fiala,E.R., and Greene,D.H. Data Compression
  | with Finite Windows, Comm.ACM, 32,4
  | (1989) 490-595
  | 
  | @(#) $Id: deflate.c,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  |------------------------------------
  | deflate.h -- internal compression
  | state Copyright (C) 1995-2004 Jean-loup
  | Gailly For conditions of distribution
  | and use, see copyright notice in zlib.h
  | 
  | WARNING: this file should *not* be used
  | by applications. It is part of the implementation
  | of the compression library and is subject
  | to change. Applications should only
  | use zlib.h.
  | 
  | @(#) $Id: deflate.h,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  |____________________________________
  | deflate.c -- compress data using the
  | deflation algorithm
  | 
  | Copyright (C) 1995-2005 Jean-loup
  | Gailly.
  | 
  | For conditions of distribution and
  | use, see copyright notice in zlib.h
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/deflate.h]

/**
  | define NO_GZIP when compiling if you
  | want to disable gzip header and trailer
  | creation by deflate(). NO_GZIP would
  | be used to avoid linking in the crc code
  | when it is not needed. For shared libraries,
  | gzip encoding should be left enabled.
  |
  */
#[cfg(not(NO_GZIP))]
pub const GZIP: bool = true;

pub const NO_DUMMY_DECL: bool = true;

/* ---------- Internal compression state.   ---------- */

/**
  | number of length codes, not counting
  | the special END_BLOCK code
  |
  */
pub const LENGTH_CODES: usize = 29;

/**
  | number of literal bytes 0..255
  |
  */
pub const LITERALS: usize = 256;

/**
  | number of Literal or Length codes, including
  | the END_BLOCK code
  |
  */
pub const L_CODES: usize = LITERALS+1+LENGTH_CODES;

/**
  | number of distance codes
  |
  */
pub const D_CODES: usize = 30;

/**
  | number of codes used to transfer the
  | bit lengths
  |
  */
pub const BL_CODES: usize = 19;

/**
  | maximum heap size
  |
  */
pub const HEAP_SIZE: usize = 2*L_CODES+1;

/**
  | All codes must not exceed MAX_BITS bits
  |
  */
pub const MAX_BITS: usize = 15;

/**
  | Stream status
  |
  */
pub const INIT_STATE:    usize = 42;
pub const EXTRA_STATE:   usize = 69;
pub const NAME_STATE:    usize = 73;
pub const COMMENT_STATE: usize = 91;
pub const HCRC_STATE:    usize = 103;
pub const BUSY_STATE:    usize = 113;
pub const FINISH_STATE:  usize = 666;

/**
  | Data structure describing a single
  | value and its code string.
  |
  */
pub mod ct_data {
    use super::*;

    pub union FC {

        /**
          frequency count
          */
        freq: u16,

        /**
          bit string
          */
        code: u16,
    }

    pub union DL {

        /**
          father node in Huffman tree
          */
        dad: u16,

        /**
          length of bit string
          */
        len: u16,
    }
}

pub struct CtData {
    fc: ct_data::FC,
    dl: ct_data::DL,
}

macro_rules! freq { () => { /* fc.freq */ } }
macro_rules! code { () => { /* fc.code */ } }
macro_rules! dad  { () => { /* dl.dad */ } }
macro_rules! len  { () => { /* dl.len */ } }

pub struct TreeDesc {

    /**
       the dynamic tree
      */
    dyn_tree:  *mut CtData,

    /**
       largest code with non zero frequency
      */
    max_code:  i32,

    /**
       the corresponding static tree
      */
    stat_desc: *mut StaticTreeDesc,
}

pub type Pos  = u16;
pub type Posf = Pos;
pub type IPos = u32;

/**
  | Insert new strings in the hash table
  | only if the match length is not greater
  | than this length. This saves time but
  | degrades compression. max_insert_length
  | is used only for compression levels
  | <= 3.
  |
  */
macro_rules! max_insert_length {
    () => {
        /*
                max_lazy_match
        */
    }
}

/**
  | A Pos is an index in the character window.
  | We use short instead of int to save space
  | in the various tables. IPos is used only
  | for parameter passing.
  |
  */
pub struct DeflateState {

    /**
       pointer back to this zlib stream
      */
    strm:             z_streamp,

    /**
       as the name implies
      */
    status:           i32,

    /**
       output still pending
      */
    pending_buf:      *mut u8,

    /**
       size of pending_buf
      */
    pending_buf_size: u64,

    /**
       next pending byte to output to the stream
      */
    pending_out:      *mut u8,

    /**
       nb of bytes in the pending buffer
      */
    pending:          u32,

    /**
       bit 0 true for zlib, bit 1 true for gzip
      */
    wrap:             i32,

    /**
       gzip header information to write
      */
    gzhead:           gz_headerp,

    /**
       where in extra, name, or comment
      */
    gzindex:          u32,

    /**
       STORED (for zip only) or DEFLATED
      */
    method:           u8,

    /**
       value of flush param for previous deflate
       call
      */
    last_flush:       i32,

    /* --------------- used by deflate.c:  --------------- */

    /**
      LZ77 window size (32K by default)
      */
    w_size: u32,

    /**
      log2(w_size) (8..16)
      */
    w_bits: u32,

    /**
      w_size - 1
      */
    w_mask: u32,

    /**
      | Sliding window. Input bytes are read into the second
      | half of the window, and move to the first half
      | later to keep a dictionary of at least wSize bytes.
      | With this organization, matches are limited to
      | a distance of wSize-MAX_MATCH bytes, but this ensures
      | that IO is always performed with a length multiple
      | of the block size. Also, it limits the window size
      | to 64K, which is quite useful on MSDOS. To do:
      | use the user input buffer as sliding window.
      */
    window: *mut u8,

    /**
       Actual size of window: 2*wSize, except when
       the user input buffer is directly used as
       sliding window.
      */
    window_size:      u64,

    /**
      | Link to older string with same hash index. To limit
      | the size of this array to 64K, this link is maintained
      | only for the last 32K strings. An index in this
      | array is thus a window index modulo 32K.
      */
    prev:             *mut Pos,

    /**
       Heads of the hash chains or NIL.
      */
    head:             *mut Pos,

    /**
       hash index of string to be inserted
      */
    ins_h:            u32,

    /**
       number of elements in hash table
      */
    hash_size:        u32,

    /**
       log2(hash_size)
      */
    hash_bits:        u32,

    /**
       hash_size-1
      */
    hash_mask:        u32,

    /**
      | Number of bits by which ins_h must be shifted at
      | each input step. It must be such that after MIN_MATCH
      | steps, the oldest byte no longer takes part in
      | the hash key, that is: hash_shift * MIN_MATCH >=
      | hash_bits
      */
    hash_shift:       u32,

    /**
      | Window position at the beginning of the current
      | output block. Gets negative when the window is
      | moved backwards.
      */
    block_start:      i64,

    /**
       length of best match
      */
    match_length:     u32,

    /**
       previous match
      */
    prev_match:       IPos,

    /**
       set if previous match exists
      */
    match_available:  i32,

    /**
       start of string to insert
      */
    strstart:         u32,

    /**
       start of matching string
      */
    match_start:      u32,

    /**
       number of valid bytes ahead in window
      */
    lookahead:        u32,

    /**
      | Length of the best match at previous step. Matches
      | not greater than this are discarded. This is used
      | in the lazy match evaluation.
      */
    prev_length:      u32,

    /**
      | To speed up deflation, hash chains are never searched
      | beyond this length. A higher limit improves compression
      | ratio but degrades the speed.
      */
    max_chain_length: u32,

    /**
      | Attempt to find a better match only when the current
      | match is strictly smaller than this value. This
      | mechanism is used only for compression levels >=
      | 4.
      */
    max_lazy_match:   u32,

    /**
       compression level (1..9)
      */
    level:      i32,

    /**
       favor or force Huffman coding
      */
    strategy:   i32,

    /**
       Use a faster search when the previous match
       is longer than this
      */
    good_match: u32,

    /**
       Stop searching when current match exceeds
       this
      */
    nice_match: i32,

    /* ---------------- used by trees.c:  ---------------- */

    /* Didn't use ct_data typedef below to supress compiler warning */

    /**
       literal and length tree
      */
    dyn_ltree: [CtData; HEAP_SIZE],

    /**
       distance tree
      */
    dyn_dtree: [CtData; 2*D_CODES+1],

    /**
       Huffman tree for bit lengths
      */
    bl_tree:   [CtData; 2*BL_CODES+1],

    /**
       desc. for literal tree
      */
    l_desc:    TreeDesc,

    /**
       desc. for distance tree
      */
    d_desc:    TreeDesc,

    /**
       desc. for bit length tree
      */
    bl_desc:   TreeDesc,

    /**
       number of codes at each bit length for an
       optimal tree
      */
    bl_count: [u16; MAX_BITS+1],

    /**
       heap used to build the Huffman trees
      */
    heap:     [i32; 2*L_CODES+1],

    /**
       number of elements in the heap
      */
    heap_len: i32,

    /**
       element of largest frequency
      */
    heap_max: i32,

    /*
      | The sons of heap[n] are heap[2*n] and
      | heap[2*n+1]. heap[0] is not used.
      | 
      | The same heap array is used to build all
      | trees.
      |
      */

    /**
       Depth of each subtree used as tie breaker
       for trees of equal frequency
      */
    depth: [u8; 2*L_CODES+1],

    /**
       buffer for literals or lengths
      */
    l_buf: *mut u8,


    /**
      | Size of match buffer for literals/lengths.
      | There are 4 reasons for limiting lit_bufsize
      | to 64K:
      | 
      | - frequencies can be kept in 16 bit counters
      | 
      | - if compression is not successful for
      | the first block, all input data is still
      | in the window so we can still emit a stored
      | block even when input comes from standard
      | input. (This can also be done for all
      | blocks if lit_bufsize is not greater
      | than 32K.)
      | 
      | - if compression is not successful for
      | a file smaller than 64K, we can even emit
      | a stored file instead of a stored block
      | (saving 5 bytes). This is applicable
      | only for zip (not gzip or zlib).
      | 
      | - creating new Huffman trees less frequently
      | may not provide fast adaptation to changes
      | in the input data statistics. (Take
      | for example a binary file with poorly
      | compressible code followed by a highly
      | compressible string table.) Smaller
      | buffer sizes give fast adaptation but
      | have of course the overhead of transmitting
      | trees more frequently.
      | 
      | - I can't count above 4
      |
      */
    lit_bufsize: u32,

    /**
       running index in l_buf
      */
    last_lit:     u32,

    /**
      | Buffer for distances. To simplify the code, d_buf
      | and l_buf have the same number of elements. To
      | use different lengths, an extra flag array would
      | be necessary.
      */
    d_buf:        *mut u16,

    /**
       bit length of current block with optimal
       trees
      */
    opt_len:      u64,

    /**
       bit length of current block with static
       trees
      */
    static_len:   u64,

    /**
       number of string matches in current block
      */
    matches:      u32,

    /**
       bit length of EOB code for last block
      */
    last_eob_len: i32,

    /**
       total bit length of compressed file mod
       2^32
      */
    #[cfg(DEBUG)]
    compressed_len: u64,

    /**
       bit length of compressed data sent mod 2^32
      */
    #[cfg(DEBUG)]
    bits_sent:      u64,

    /**
       Output buffer. bits are inserted starting
       at the bottom (least significant bits).
      */
    bi_buf:   u16,

    /**
       Number of valid bits in bi_buf. All bits
       above the last valid bit are always zero.
      */
    bi_valid: i32,

}

/**
  | Output a byte on the stream.
  | 
  | IN assertion: there is enough room in
  | pending_buf.
  |
  */
macro_rules! put_byte {
    ($s:ident, $c:ident) => {
        /*
                {s->pending_buf[s->pending++] = (c);}
        */
    }
}

/**
  | Minimum amount of lookahead, except
  | at the end of the input file. See deflate.c
  | for comments about the MIN_MATCH+1.
  |
  */
macro_rules! min_lookahead {
    () => {
        /*
                (MAX_MATCH+MIN_MATCH+1)
        */
    }
}

/**
  | In order to simplify the code, particularly
  | on 16 bit machines, match distances
  | are limited to MAX_DIST instead of WSIZE.
  |
  */
macro_rules! max_dist {
    ($s:ident) => {
        /*
                ((s)->w_size-MIN_LOOKAHEAD)
        */
    }
}

/* ------------------- in trees.c  ------------------- */
macro_rules! d_code {
    ($dist:ident) => {
        /*
        
           ((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)])
        */
    }
}

/**
  | Mapping from a distance to a distance
  | code. dist is the distance - 1 and must
  | not have side effects. _dist_code[256]
  | and _dist_code[257] are never used.
  |
  */

/* ----- Inline versions of _tr_tally for speed:  ----- */

#[cfg(not(DEBUG))]
lazy_static!{
    /*
    #if defined(GEN_TREES_H) || !defined(STDC)
      extern uch _length_code[];
      extern uch _dist_code[];
    #else
      extern const uch _length_code[];
      extern const uch _dist_code[];
    #endif
    */
}

#[cfg(not(DEBUG))]
macro_rules! _tr_tally_lit {
    ($s:ident, 
     $c:ident, 
     $flush:ident) => {
        /*
        
          { uch cc = (c); 
            s->d_buf[s->last_lit] = 0; 
            s->l_buf[s->last_lit++] = cc; 
            s->dyn_ltree[cc].Freq++; 
            flush = (s->last_lit == s->lit_bufsize-1); 
           }
        */
    }
}

#[cfg(not(DEBUG))]
macro_rules! _tr_tally_dist {
    ($s:ident, 
     $distance:ident, 
     $length:ident, 
     $flush:ident) => {
        /*
        
          { uch len = (length); 
            u16 dist = (distance); 
            s->d_buf[s->last_lit] = dist; 
            s->l_buf[s->last_lit++] = len; 
            dist--; 
            s->dyn_ltree[_length_code[len]+LITERALS+1].Freq++; 
            s->dyn_dtree[d_code(dist)].Freq++; 
            flush = (s->last_lit == s->lit_bufsize-1); 
          }
        */
    }
}

#[cfg(DEBUG)]
macro_rules! _tr_tally_lit {
    ($s:ident, 
     $c:ident, 
     $flush:ident) => {
        /*
          flush = _tr_tally(s, 0, c)
        */
    }
}

#[cfg(DEBUG)]
macro_rules! _tr_tally_dist {
    ($s:ident, 
     $distance:ident, 
     $length:ident, 
     $flush:ident) => {
        /*
          flush = _tr_tally(s, distance, length)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/deflate.c]

/**
  | If you use the zlib library in a product,
  | an acknowledgment is welcome in the
  | documentation of your product. If for
  | some reason you cannot include such
  | an acknowledgment, I would appreciate
  | that you keep this copyright string
  | in the executable of your product.
  |
  | tk: ^you got it!! thank you for the humanity!
  */
pub const deflate_copyright: &'static str =
" deflate 1.2.3 Copyright 1995-2005 Jean-loup Gailly ";

/* -------------- Function prototypes.   -------------- */

pub enum BlockState {

    /**
      | block not completed, need more input
      | or more output
      |
      */
    need_more,      

    /**
      | block flush performed
      |
      */
    block_done,     

    /**
      | finish started, need only more output
      | at next deflate
      |
      */
    finish_started, 

    /**
      | finish done, accept no more input or
      | output
      |
      */
    finish_done     
}

/**
  | Compression function. Returns the
  | block state after the call.
  |
  */
pub type CompressFunc = fn(s: *mut DeflateState, flush: i32) -> BlockState;

/* ------------------- Local data  ------------------- */

/**
  | Tail of hash chains
  |
  */
pub const NIL: usize = 0;

/**
  | Matches of length 3 are discarded if
  | their distance exceeds TOO_FAR
  |
  */
#[cfg(not(TOO_FAR))]
pub const TOO_FAR: usize = 4096;

/**
  | Minimum amount of lookahead, except
  | at the end of the input file.
  | 
  | See deflate.c for comments about the
  | MIN_MATCH+1.
  |
  */
pub const MIN_LOOKAHEAD: usize = MAX_MATCH+MIN_MATCH+1;

/**
  | Values for max_lazy_match, good_match
  | and max_chain_length, depending on
  | the desired pack level (0..9). The values
  | given below have been tuned to exclude
  | worst case performance for pathological
  | files. Better values may be found for
  | specific files.
  |
  */
pub struct Config {

    /**
       reduce lazy search above this match length
      */
    good_length: u16,

    /**
       do not perform lazy search above this match
       length
      */
    max_lazy:    u16,

    /**
       quit search above this match length
      */
    nice_length: u16,

    max_chain:   u16,

    func:        CompressFunc,
} 

/**
  | max speed, no lazy matches
  |
  */
#[cfg(FASTEST)]
lazy_static!{
    /*
    static const config configuration_table[2] = {
    /*      good lazy nice chain */
    /* 0 */ {0,    0,  0,    0, deflate_stored},  /* store only */
    /* 1 */ {4,    4,  8,    4, deflate_fast}};
    */
}

/**
  | max compression
  |
  */
#[cfg(not(FASTEST))]
lazy_static!{
    /*
    static const config configuration_table[10] = {
    /*      good lazy nice chain */
    /* 0 */ {0,    0,  0,    0, deflate_stored},  /* store only */
    /* 1 */ {4,    4,  8,    4, deflate_fast}, /* max speed, no lazy matches */
    /* 2 */ {4,    5, 16,    8, deflate_fast},
    /* 3 */ {4,    6, 32,   32, deflate_fast},

    /* 4 */ {4,    4, 16,   16, deflate_slow},  /* lazy matches */
    /* 5 */ {8,   16, 32,   32, deflate_slow},
    /* 6 */ {8,   16, 128, 128, deflate_slow},
    /* 7 */ {8,   32, 128, 256, deflate_slow},
    /* 8 */ {32, 128, 258, 1024, deflate_slow},
    /* 9 */ {32, 258, 258, 4096, deflate_slow}};
    */
}

/*
  | Note: the deflate() code requires max_lazy
  | >= MIN_MATCH and max_chain >= 4
  | 
  | For deflate_fast() (levels <= 3) good
  | is ignored and lazy has a different meaning.
  |
  */

/**
  | result of memcmp for equal strings
  |
  */
pub const EQUAL: usize = 0;

/**
  | for buggy compilers
  |
  */
#[cfg(not(NO_DUMMY_DECL))]
pub struct static_tree_desc_s {
    dummy: i32,
}

/**
  | Update a hash value with the given input
  | byte
  | 
  | IN assertion: all calls to to UPDATE_HASH
  | are made with consecutive input characters,
  | so that a running hash key can be computed
  | from the previous key instead of complete
  | recalculation each time.
  |
  */
macro_rules! update_hash {
    ($s:ident, $h:ident, $c:ident) => {
        /*
                (h = (((h)<<s->hash_shift) ^ (c)) & s->hash_mask)
        */
    }
}

/**
  | Insert string str in the dictionary
  | and set match_head to the previous head
  | of the hash chain (the most recent string
  | with same hash key). Return the previous
  | length of the hash chain.
  | 
  | If this file is compiled with -DFASTEST,
  | the compression level is forced to 1,
  | and no hash chains are maintained.
  | 
  | IN assertion: all calls to to INSERT_STRING
  | are made with consecutive input characters
  | and the first MIN_MATCH bytes of str
  | are valid (except for the last MIN_MATCH-1
  | bytes of the input file).
  |
  */
#[cfg(FASTEST)]
macro_rules! insert_string {
    ($s:ident, 
     $str:ident, 
     $match_head:ident) => {
        /*
        
           (UPDATE_HASH(s, s->ins_h, s->window[(str) + (MIN_MATCH-1)]), 
            match_head = s->head[s->ins_h], 
            s->head[s->ins_h] = (Pos)(str))
        */
    }
}

#[cfg(not(FASTEST))]
macro_rules! insert_string {
    ($s:ident, 
     $str:ident, 
     $match_head:ident) => {
        /*
        
           (UPDATE_HASH(s, s->ins_h, s->window[(str) + (MIN_MATCH-1)]), 
            match_head = s->prev[(str) & s->w_mask] = s->head[s->ins_h], 
            s->head[s->ins_h] = (Pos)(str))
        */
    }
}

/**
  | Initialize the hash table (avoiding
  | 64K overflow for 16 bit systems). prev[]
  | will be initialized on the fly.
  |
  */
macro_rules! clear_hash {
    ($s:ident) => {
        /*
        
            s->head[s->hash_size-1] = NIL; 
            zmemzero((Bytef *)s->head, (unsigned)(s->hash_size-1)*sizeof(*s->head));
        */
    }
}

pub fn deflate_init(
        strm:        z_streamp,
        level:       i32,
        version:     *const u8,
        stream_size: i32) -> i32 {
    
    todo!();
    /*
        return deflateInit2_(strm, level, Z_DEFLATED, MAX_WBITS, DEF_MEM_LEVEL,
                             Z_DEFAULT_STRATEGY, version, stream_size);
        /* To do: ignore strm->next_in if we use it as window */
    */
}

pub fn deflate_init2(
        strm:        z_streamp,
        level:       i32,
        method:      i32,
        window_bits: i32,
        mem_level:   i32,
        strategy:    i32,
        version:     *const u8,
        stream_size: i32) -> i32 {
    
    todo!();
    /*
        deflate_state *s;
        int wrap = 1;
        static const char my_version[] = ZLIB_VERSION;

        ushf *overlay;
        /* We overlay pending_buf and d_buf+l_buf. This works since the average
         * output size for (length,distance) codes is <= 24 bits.
         */

        if (version == Z_NULL || version[0] != my_version[0] ||
            stream_size != sizeof(z_stream)) {
            return Z_VERSION_ERROR;
        }
        if (strm == Z_NULL) return Z_STREAM_ERROR;

        strm->msg = Z_NULL;
        if (strm->zalloc == (alloc_func)0) {
            strm->zalloc = zcalloc;
            strm->opaque = (voidpf)0;
        }
        if (strm->zfree == (free_func)0) strm->zfree = zcfree;

    #ifdef FASTEST
        if (level != 0) level = 1;
    #else
        if (level == Z_DEFAULT_COMPRESSION) level = 6;
    #endif

        if (windowBits < 0) { /* suppress zlib wrapper */
            wrap = 0;
            windowBits = -windowBits;
        }
    #ifdef GZIP
        else if (windowBits > 15) {
            wrap = 2;       /* write gzip wrapper instead */
            windowBits -= 16;
        }
    #endif
        if (memLevel < 1 || memLevel > MAX_MEM_LEVEL || method != Z_DEFLATED ||
            windowBits < 8 || windowBits > 15 || level < 0 || level > 9 ||
            strategy < 0 || strategy > Z_FIXED) {
            return Z_STREAM_ERROR;
        }
        if (windowBits == 8) windowBits = 9;  /* until 256-byte window bug fixed */
        s = (deflate_state *) ZALLOC(strm, 1, sizeof(deflate_state));
        if (s == Z_NULL) return Z_MEM_ERROR;
        strm->state = (struct internal_state FAR *)s;
        s->strm = strm;

        s->wrap = wrap;
        s->gzhead = Z_NULL;
        s->w_bits = windowBits;
        s->w_size = 1 << s->w_bits;
        s->w_mask = s->w_size - 1;

        s->hash_bits = memLevel + 7;
        s->hash_size = 1 << s->hash_bits;
        s->hash_mask = s->hash_size - 1;
        s->hash_shift =  ((s->hash_bits+MIN_MATCH-1)/MIN_MATCH);

        s->window = (Bytef *) ZALLOC(strm, s->w_size, 2*sizeof(Byte));
        s->prev   = (Posf *)  ZALLOC(strm, s->w_size, sizeof(Pos));
        s->head   = (Posf *)  ZALLOC(strm, s->hash_size, sizeof(Pos));

        s->lit_bufsize = 1 << (memLevel + 6); /* 16K elements by default */

        overlay = (ushf *) ZALLOC(strm, s->lit_bufsize, sizeof(u16)+2);
        s->pending_buf = (uchf *) overlay;
        s->pending_buf_size = (ulg)s->lit_bufsize * (sizeof(u16)+2L);

        if (s->window == Z_NULL || s->prev == Z_NULL || s->head == Z_NULL ||
            s->pending_buf == Z_NULL) {
            s->status = FINISH_STATE;
            strm->msg = (char*)ERR_MSG(Z_MEM_ERROR);
            deflateEnd (strm);
            return Z_MEM_ERROR;
        }
        s->d_buf = overlay + s->lit_bufsize/sizeof(u16);
        s->l_buf = s->pending_buf + (1+sizeof(u16))*s->lit_bufsize;

        s->level = level;
        s->strategy = strategy;
        s->method = (Byte)method;

        return deflateReset(strm);
    */
}

pub fn deflate_set_dictionary(
        strm:        z_streamp,
        dictionary:  *const u8,
        dict_length: u32) -> i32 {
    
    todo!();
    /*
        deflate_state *s;
        uInt length = dictLength;
        uInt n;
        IPos hash_head = 0;

        if (strm == Z_NULL || strm->state == Z_NULL || dictionary == Z_NULL ||
            strm->state->wrap == 2 ||
            (strm->state->wrap == 1 && strm->state->status != INIT_STATE))
            return Z_STREAM_ERROR;

        s = strm->state;
        if (s->wrap)
            strm->adler = adler32(strm->adler, dictionary, dictLength);

        if (length < MIN_MATCH) return Z_OK;
        if (length > MAX_DIST(s)) {
            length = MAX_DIST(s);
            dictionary += dictLength - length; /* use the tail of the dictionary */
        }
        zmemcpy(s->window, dictionary, length);
        s->strstart = length;
        s->block_start = (long)length;

        /* Insert all strings in the hash table (except for the last two bytes).
         * s->lookahead stays null, so s->ins_h will be recomputed at the next
         * call of fill_window.
         */
        s->ins_h = s->window[0];
        UPDATE_HASH(s, s->ins_h, s->window[1]);
        for (n = 0; n <= length - MIN_MATCH; n++) {
            INSERT_STRING(s, n, hash_head);
        }

        (void) hash_head;  /* to make compiler happy */
        return Z_OK;
    */
}

pub fn deflate_reset(strm: z_streamp) -> i32 {
    
    todo!();
    /*
        deflate_state *s;

        if (strm == Z_NULL || strm->state == Z_NULL ||
            strm->zalloc == (alloc_func)0 || strm->zfree == (free_func)0) {
            return Z_STREAM_ERROR;
        }

        strm->total_in = strm->total_out = 0;
        strm->msg = Z_NULL; /* use zfree if we ever allocate msg dynamically */
        strm->data_type = Z_UNKNOWN;

        s = (deflate_state *)strm->state;
        s->pending = 0;
        s->pending_out = s->pending_buf;

        if (s->wrap < 0) {
            s->wrap = -s->wrap; /* was made negative by deflate(..., Z_FINISH); */
        }
        s->status = s->wrap ? INIT_STATE : BUSY_STATE;
        strm->adler =
    #ifdef GZIP
            s->wrap == 2 ? crc32(0L, Z_NULL, 0) :
    #endif
            adler32(0L, Z_NULL, 0);
        s->last_flush = Z_NO_FLUSH;

        _tr_init(s);
        lm_init(s);

        return Z_OK;
    */
}

pub fn deflate_set_header(
        strm: z_streamp,
        head: gz_headerp) -> i32 {
    
    todo!();
    /*
        if (strm == Z_NULL || strm->state == Z_NULL) return Z_STREAM_ERROR;
        if (strm->state->wrap != 2) return Z_STREAM_ERROR;
        strm->state->gzhead = head;
        return Z_OK;
    */
}

pub fn deflate_prime(
        strm:  z_streamp,
        bits:  i32,
        value: i32) -> i32 {
    
    todo!();
    /*
        if (strm == Z_NULL || strm->state == Z_NULL) return Z_STREAM_ERROR;
        strm->state->bi_valid = bits;
        strm->state->bi_buf = (u16)(value & ((1 << bits) - 1));
        return Z_OK;
    */
}

pub fn deflate_params(
        strm:     z_streamp,
        level:    i32,
        strategy: i32) -> i32 {
    
    todo!();
    /*
        deflate_state *s;
        compress_func func;
        int err = Z_OK;

        if (strm == Z_NULL || strm->state == Z_NULL) return Z_STREAM_ERROR;
        s = strm->state;

    #ifdef FASTEST
        if (level != 0) level = 1;
    #else
        if (level == Z_DEFAULT_COMPRESSION) level = 6;
    #endif
        if (level < 0 || level > 9 || strategy < 0 || strategy > Z_FIXED) {
            return Z_STREAM_ERROR;
        }
        func = configuration_table[s->level].func;

        if (func != configuration_table[level].func && strm->total_in != 0) {
            /* Flush the last buffer: */
            err = deflate(strm, Z_PARTIAL_FLUSH);
        }
        if (s->level != level) {
            s->level = level;
            s->max_lazy_match   = configuration_table[level].max_lazy;
            s->good_match       = configuration_table[level].good_length;
            s->nice_match       = configuration_table[level].nice_length;
            s->max_chain_length = configuration_table[level].max_chain;
        }
        s->strategy = strategy;
        return err;
    */
}

pub fn deflate_tune(
        strm:        z_streamp,
        good_length: i32,
        max_lazy:    i32,
        nice_length: i32,
        max_chain:   i32) -> i32 {
    
    todo!();
    /*
        deflate_state *s;

        if (strm == Z_NULL || strm->state == Z_NULL) return Z_STREAM_ERROR;
        s = strm->state;
        s->good_match = good_length;
        s->max_lazy_match = max_lazy;
        s->nice_match = nice_length;
        s->max_chain_length = max_chain;
        return Z_OK;
    */
}

/**
  | For the default windowBits of 15 and
  | memLevel of 8, this function returns
  | a close to exact, as well as small, upper
  | bound on the compressed size. They are
  | coded as constants here for a reason--if
  | the #define's are changed, then this
  | function needs to be changed as well.
  | The return value for 15 and 8 only works
  | for those exact settings.
  | 
  | For any setting other than those defaults
  | for windowBits and memLevel, the value
  | returned is a conservative worst case
  | for the maximum expansion resulting
  | from using fixed blocks instead of stored
  | blocks, which deflate can emit on compressed
  | data for some combinations of the parameters.
  | 
  | This function could be more sophisticated
  | to provide closer upper bounds for every
  | combination of windowBits and memLevel,
  | as well as wrap. But even the conservative
  | upper bound of about 14% expansion does
  | not seem onerous for output buffer allocation.
  |
  */
pub fn deflate_bound(
        strm:       z_streamp,
        source_len: u64) -> u64 {
    
    todo!();
    /*
        deflate_state *s;
        uLong destLen;

        /* conservative upper bound */
        destLen = sourceLen +
                  ((sourceLen + 7) >> 3) + ((sourceLen + 63) >> 6) + 11;

        /* if can't get parameters, return conservative bound */
        if (strm == Z_NULL || strm->state == Z_NULL)
            return destLen;

        /* if not default parameters, return conservative bound */
        s = strm->state;
        if (s->w_bits != 15 || s->hash_bits != 8 + 7)
            return destLen;

        /* default settings: return tight bound for that case */
        return compressBound(sourceLen);
    */
}

/**
  | Put a short in the pending buffer. The
  | 16-bit value is put in MSB order. IN assertion:
  | the stream state is correct and there
  | is enough room in pending_buf.
  |
  */
pub fn put_shortmsb(
        s: *mut DeflateState,
        b: u32)  {
    
    todo!();
    /*
        put_byte(s, (Byte)(b >> 8));
        put_byte(s, (Byte)(b & 0xff));
    */
}

/**
  | Flush as much pending output as possible.
  | All deflate() output goes through this
  | function so some applications may wish
  | to modify it to avoid allocating a large
  | strm->next_out buffer and copying
  | into it. (See also read_buf()).
  |
  */
pub fn flush_pending(strm: z_streamp)  {
    
    todo!();
    /*
        unsigned len = strm->state->pending;

        if (len > strm->avail_out) len = strm->avail_out;
        if (len == 0) return;

        zmemcpy(strm->next_out, strm->state->pending_out, len);
        strm->next_out  += len;
        strm->state->pending_out  += len;
        strm->total_out += len;
        strm->avail_out  -= len;
        strm->state->pending -= len;
        if (strm->state->pending == 0) {
            strm->state->pending_out = strm->state->pending_buf;
        }
    */
}

pub fn deflate(
        strm:  z_streamp,
        flush: i32) -> i32 {
    
    todo!();
    /*
        int old_flush; /* value of flush param for previous deflate call */
        deflate_state *s;

        if (strm == Z_NULL || strm->state == Z_NULL ||
            flush > Z_FINISH || flush < 0) {
            return Z_STREAM_ERROR;
        }
        s = strm->state;

        if (strm->next_out == Z_NULL ||
            (strm->next_in == Z_NULL && strm->avail_in != 0) ||
            (s->status == FINISH_STATE && flush != Z_FINISH)) {
            ERR_RETURN(strm, Z_STREAM_ERROR);
        }
        if (strm->avail_out == 0) ERR_RETURN(strm, Z_BUF_ERROR);

        s->strm = strm; /* just in case */
        old_flush = s->last_flush;
        s->last_flush = flush;

        /* Write the header */
        if (s->status == INIT_STATE) {
    #ifdef GZIP
            if (s->wrap == 2) {
                strm->adler = crc32(0L, Z_NULL, 0);
                put_byte(s, 31);
                put_byte(s, 139);
                put_byte(s, 8);
                if (s->gzhead == NULL) {
                    put_byte(s, 0);
                    put_byte(s, 0);
                    put_byte(s, 0);
                    put_byte(s, 0);
                    put_byte(s, 0);
                    put_byte(s, s->level == 9 ? 2 :
                                (s->strategy >= Z_HUFFMAN_ONLY || s->level < 2 ?
                                 4 : 0));
                    put_byte(s, OS_CODE);
                    s->status = BUSY_STATE;
                }
                else {
                    put_byte(s, (s->gzhead->text ? 1 : 0) +
                                (s->gzhead->hcrc ? 2 : 0) +
                                (s->gzhead->extra == Z_NULL ? 0 : 4) +
                                (s->gzhead->name == Z_NULL ? 0 : 8) +
                                (s->gzhead->comment == Z_NULL ? 0 : 16)
                            );
                    put_byte(s, (Byte)(s->gzhead->time & 0xff));
                    put_byte(s, (Byte)((s->gzhead->time >> 8) & 0xff));
                    put_byte(s, (Byte)((s->gzhead->time >> 16) & 0xff));
                    put_byte(s, (Byte)((s->gzhead->time >> 24) & 0xff));
                    put_byte(s, s->level == 9 ? 2 :
                                (s->strategy >= Z_HUFFMAN_ONLY || s->level < 2 ?
                                 4 : 0));
                    put_byte(s, s->gzhead->os & 0xff);
                    if (s->gzhead->extra != NULL) {
                        put_byte(s, s->gzhead->extra_len & 0xff);
                        put_byte(s, (s->gzhead->extra_len >> 8) & 0xff);
                    }
                    if (s->gzhead->hcrc)
                        strm->adler = crc32(strm->adler, s->pending_buf,
                                            s->pending);
                    s->gzindex = 0;
                    s->status = EXTRA_STATE;
                }
            }
            else
    #endif
            {
                uInt header = (Z_DEFLATED + ((s->w_bits-8)<<4)) << 8;
                uInt level_flags;

                if (s->strategy >= Z_HUFFMAN_ONLY || s->level < 2)
                    level_flags = 0;
                else if (s->level < 6)
                    level_flags = 1;
                else if (s->level == 6)
                    level_flags = 2;
                else
                    level_flags = 3;
                header |= (level_flags << 6);
                if (s->strstart != 0) header |= PRESET_DICT;
                header += 31 - (header % 31);

                s->status = BUSY_STATE;
                putShortMSB(s, header);

                /* Save the adler32 of the preset dictionary: */
                if (s->strstart != 0) {
                    putShortMSB(s, (uInt)(strm->adler >> 16));
                    putShortMSB(s, (uInt)(strm->adler & 0xffff));
                }
                strm->adler = adler32(0L, Z_NULL, 0);
            }
        }
    #ifdef GZIP
        if (s->status == EXTRA_STATE) {
            if (s->gzhead->extra != NULL) {
                uInt beg = s->pending;  /* start of bytes to update crc */

                while (s->gzindex < (s->gzhead->extra_len & 0xffff)) {
                    if (s->pending == s->pending_buf_size) {
                        if (s->gzhead->hcrc && s->pending > beg)
                            strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                                s->pending - beg);
                        flush_pending(strm);
                        beg = s->pending;
                        if (s->pending == s->pending_buf_size)
                            break;
                    }
                    put_byte(s, s->gzhead->extra[s->gzindex]);
                    s->gzindex++;
                }
                if (s->gzhead->hcrc && s->pending > beg)
                    strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                        s->pending - beg);
                if (s->gzindex == s->gzhead->extra_len) {
                    s->gzindex = 0;
                    s->status = NAME_STATE;
                }
            }
            else
                s->status = NAME_STATE;
        }
        if (s->status == NAME_STATE) {
            if (s->gzhead->name != NULL) {
                uInt beg = s->pending;  /* start of bytes to update crc */
                int val;

                do {
                    if (s->pending == s->pending_buf_size) {
                        if (s->gzhead->hcrc && s->pending > beg)
                            strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                                s->pending - beg);
                        flush_pending(strm);
                        beg = s->pending;
                        if (s->pending == s->pending_buf_size) {
                            val = 1;
                            break;
                        }
                    }
                    val = s->gzhead->name[s->gzindex++];
                    put_byte(s, val);
                } while (val != 0);
                if (s->gzhead->hcrc && s->pending > beg)
                    strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                        s->pending - beg);
                if (val == 0) {
                    s->gzindex = 0;
                    s->status = COMMENT_STATE;
                }
            }
            else
                s->status = COMMENT_STATE;
        }
        if (s->status == COMMENT_STATE) {
            if (s->gzhead->comment != NULL) {
                uInt beg = s->pending;  /* start of bytes to update crc */
                int val;

                do {
                    if (s->pending == s->pending_buf_size) {
                        if (s->gzhead->hcrc && s->pending > beg)
                            strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                                s->pending - beg);
                        flush_pending(strm);
                        beg = s->pending;
                        if (s->pending == s->pending_buf_size) {
                            val = 1;
                            break;
                        }
                    }
                    val = s->gzhead->comment[s->gzindex++];
                    put_byte(s, val);
                } while (val != 0);
                if (s->gzhead->hcrc && s->pending > beg)
                    strm->adler = crc32(strm->adler, s->pending_buf + beg,
                                        s->pending - beg);
                if (val == 0)
                    s->status = HCRC_STATE;
            }
            else
                s->status = HCRC_STATE;
        }
        if (s->status == HCRC_STATE) {
            if (s->gzhead->hcrc) {
                if (s->pending + 2 > s->pending_buf_size)
                    flush_pending(strm);
                if (s->pending + 2 <= s->pending_buf_size) {
                    put_byte(s, (Byte)(strm->adler & 0xff));
                    put_byte(s, (Byte)((strm->adler >> 8) & 0xff));
                    strm->adler = crc32(0L, Z_NULL, 0);
                    s->status = BUSY_STATE;
                }
            }
            else
                s->status = BUSY_STATE;
        }
    #endif

        /* Flush as much pending output as possible */
        if (s->pending != 0) {
            flush_pending(strm);
            if (strm->avail_out == 0) {
                /* Since avail_out is 0, deflate will be called again with
                 * more output space, but possibly with both pending and
                 * avail_in equal to zero. There won't be anything to do,
                 * but this is not an error situation so make sure we
                 * return OK instead of BUF_ERROR at next call of deflate:
                 */
                s->last_flush = -1;
                return Z_OK;
            }

        /* Make sure there is something to do and avoid duplicate consecutive
         * flushes. For repeated and useless calls with Z_FINISH, we keep
         * returning Z_STREAM_END instead of Z_BUF_ERROR.
         */
        } else if (strm->avail_in == 0 && flush <= old_flush &&
                   flush != Z_FINISH) {
            ERR_RETURN(strm, Z_BUF_ERROR);
        }

        /* User must not provide more input after the first FINISH: */
        if (s->status == FINISH_STATE && strm->avail_in != 0) {
            ERR_RETURN(strm, Z_BUF_ERROR);
        }

        /* Start a new block or continue the current one.
         */
        if (strm->avail_in != 0 || s->lookahead != 0 ||
            (flush != Z_NO_FLUSH && s->status != FINISH_STATE)) {
            block_state bstate;

            bstate = (*(configuration_table[s->level].func))(s, flush);

            if (bstate == finish_started || bstate == finish_done) {
                s->status = FINISH_STATE;
            }
            if (bstate == need_more || bstate == finish_started) {
                if (strm->avail_out == 0) {
                    s->last_flush = -1; /* avoid BUF_ERROR next call, see above */
                }
                return Z_OK;
                /* If flush != Z_NO_FLUSH && avail_out == 0, the next call
                 * of deflate should use the same flush parameter to make sure
                 * that the flush is complete. So we don't have to output an
                 * empty block here, this will be done at next call. This also
                 * ensures that for a very small output buffer, we emit at most
                 * one empty block.
                 */
            }
            if (bstate == block_done) {
                if (flush == Z_PARTIAL_FLUSH) {
                    _tr_align(s);
                } else { /* FULL_FLUSH or SYNC_FLUSH */
                    _tr_stored_block(s, (char*)0, 0L, 0);
                    /* For a full flush, this empty block will be recognized
                     * as a special marker by inflate_sync().
                     */
                    if (flush == Z_FULL_FLUSH) {
                        CLEAR_HASH(s);             /* forget history */
                    }
                }
                flush_pending(strm);
                if (strm->avail_out == 0) {
                  s->last_flush = -1; /* avoid BUF_ERROR at next call, see above */
                  return Z_OK;
                }
            }
        }
        Assert(strm->avail_out > 0, "bug2");

        if (flush != Z_FINISH) return Z_OK;
        if (s->wrap <= 0) return Z_STREAM_END;

        /* Write the trailer */
    #ifdef GZIP
        if (s->wrap == 2) {
            put_byte(s, (Byte)(strm->adler & 0xff));
            put_byte(s, (Byte)((strm->adler >> 8) & 0xff));
            put_byte(s, (Byte)((strm->adler >> 16) & 0xff));
            put_byte(s, (Byte)((strm->adler >> 24) & 0xff));
            put_byte(s, (Byte)(strm->total_in & 0xff));
            put_byte(s, (Byte)((strm->total_in >> 8) & 0xff));
            put_byte(s, (Byte)((strm->total_in >> 16) & 0xff));
            put_byte(s, (Byte)((strm->total_in >> 24) & 0xff));
        }
        else
    #endif
        {
            putShortMSB(s, (uInt)(strm->adler >> 16));
            putShortMSB(s, (uInt)(strm->adler & 0xffff));
        }
        flush_pending(strm);
        /* If avail_out is zero, the application will call deflate again
         * to flush the rest.
         */
        if (s->wrap > 0) s->wrap = -s->wrap; /* write the trailer only once! */
        return s->pending != 0 ? Z_OK : Z_STREAM_END;
    */
}

pub fn deflate_end(strm: z_streamp) -> i32 {
    
    todo!();
    /*
        int status;

        if (strm == Z_NULL || strm->state == Z_NULL) return Z_STREAM_ERROR;

        status = strm->state->status;
        if (status != INIT_STATE &&
            status != EXTRA_STATE &&
            status != NAME_STATE &&
            status != COMMENT_STATE &&
            status != HCRC_STATE &&
            status != BUSY_STATE &&
            status != FINISH_STATE) {
          return Z_STREAM_ERROR;
        }

        /* Deallocate in reverse order of allocations: */
        TRY_FREE(strm, strm->state->pending_buf);
        TRY_FREE(strm, strm->state->head);
        TRY_FREE(strm, strm->state->prev);
        TRY_FREE(strm, strm->state->window);

        ZFREE(strm, strm->state);
        strm->state = Z_NULL;

        return status == BUSY_STATE ? Z_DATA_ERROR : Z_OK;
    */
}

/**
  | Copy the source state to the destination
  | state.
  | 
  | To simplify the source, this is not supported
  | for 16-bit MSDOS (which doesn't have
  | enough memory anyway to duplicate compression
  | states).
  |
  */
pub fn deflate_copy(
        dest:   z_streamp,
        source: z_streamp) -> i32 {
    
    todo!();
    /*
        #ifdef MAXSEG_64K
        return Z_STREAM_ERROR;
    #else
        deflate_state *ds;
        deflate_state *ss;
        ushf *overlay;

        if (source == Z_NULL || dest == Z_NULL || source->state == Z_NULL) {
            return Z_STREAM_ERROR;
        }

        ss = source->state;

        zmemcpy(dest, source, sizeof(z_stream));

        ds = (deflate_state *) ZALLOC(dest, 1, sizeof(deflate_state));
        if (ds == Z_NULL) return Z_MEM_ERROR;
        dest->state = (struct internal_state FAR *) ds;
        zmemcpy(ds, ss, sizeof(deflate_state));
        ds->strm = dest;

        ds->window = (Bytef *) ZALLOC(dest, ds->w_size, 2*sizeof(Byte));
        ds->prev   = (Posf *)  ZALLOC(dest, ds->w_size, sizeof(Pos));
        ds->head   = (Posf *)  ZALLOC(dest, ds->hash_size, sizeof(Pos));
        overlay = (ushf *) ZALLOC(dest, ds->lit_bufsize, sizeof(u16)+2);
        ds->pending_buf = (uchf *) overlay;

        if (ds->window == Z_NULL || ds->prev == Z_NULL || ds->head == Z_NULL ||
            ds->pending_buf == Z_NULL) {
            deflateEnd (dest);
            return Z_MEM_ERROR;
        }
        /* following zmemcpy do not work for 16-bit MSDOS */
        zmemcpy(ds->window, ss->window, ds->w_size * 2 * sizeof(Byte));
        zmemcpy(ds->prev, ss->prev, ds->w_size * sizeof(Pos));
        zmemcpy(ds->head, ss->head, ds->hash_size * sizeof(Pos));
        zmemcpy(ds->pending_buf, ss->pending_buf, (uInt)ds->pending_buf_size);

        ds->pending_out = ds->pending_buf + (ss->pending_out - ss->pending_buf);
        ds->d_buf = overlay + ds->lit_bufsize/sizeof(u16);
        ds->l_buf = ds->pending_buf + (1+sizeof(u16))*ds->lit_bufsize;

        ds->l_desc.dyn_tree = ds->dyn_ltree;
        ds->d_desc.dyn_tree = ds->dyn_dtree;
        ds->bl_desc.dyn_tree = ds->bl_tree;

        return Z_OK;
    #endif /* MAXSEG_64K */
    */
}

/**
  | Read a new buffer from the current input
  | stream, update the adler32 and total
  | number of bytes read. All deflate()
  | input goes through this function so
  | some applications may wish to modify
  | it to avoid allocating a large strm->next_in
  | buffer and copying from it. (See also
  | flush_pending()).
  |
  */
pub fn read_buf(
        strm: z_streamp,
        buf:  *mut u8,
        size: u32) -> i32 {
    
    todo!();
    /*
        unsigned len = strm->avail_in;

        if (len > size) len = size;
        if (len == 0) return 0;

        strm->avail_in  -= len;

        if (strm->state->wrap == 1) {
            strm->adler = adler32(strm->adler, strm->next_in, len);
        }
    #ifdef GZIP
        else if (strm->state->wrap == 2) {
            strm->adler = crc32(strm->adler, strm->next_in, len);
        }
    #endif
        zmemcpy(buf, strm->next_in, len);
        strm->next_in  += len;
        strm->total_in += len;

        return (int)len;
    */
}

/**
  | Initialize the "longest match" routines
  | for a new zlib stream
  |
  */
pub fn lm_init(s: *mut DeflateState)  {
    
    todo!();
    /*
        s->window_size = (ulg)2L*s->w_size;

        CLEAR_HASH(s);

        /* Set the default configuration parameters:
         */
        s->max_lazy_match   = configuration_table[s->level].max_lazy;
        s->good_match       = configuration_table[s->level].good_length;
        s->nice_match       = configuration_table[s->level].nice_length;
        s->max_chain_length = configuration_table[s->level].max_chain;

        s->strstart = 0;
        s->block_start = 0L;
        s->lookahead = 0;
        s->match_length = s->prev_length = MIN_MATCH-1;
        s->match_available = 0;
        s->ins_h = 0;
    #ifndef FASTEST
    #ifdef ASMV
        match_init(); /* initialize the asm code */
    #endif
    #endif
    */
}

/**
  | Set match_start to the longest match
  | starting at the given string and return
  | its length. Matches shorter or equal
  | to prev_length are discarded, in which
  | case the result is equal to prev_length
  | and match_start is garbage.
  | 
  | IN assertions: cur_match is the head
  | of the hash chain for the current string
  | (strstart) and its distance is <= MAX_DIST,
  | and prev_length >= 1
  | 
  | OUT assertion: the match length is not
  | greater than s->lookahead.
  |
  | For 80x86 and 680x0, an optimized version
  | will be provided in match.asm or match.S.
  | The code will be functionally equivalent.
  |
  */
#[cfg(not(FASTEST))]
#[cfg(not(ASMV))]
pub fn longest_match(
        s:         *mut DeflateState,
        cur_match: IPos) -> u32 {
    
    todo!();
    /*
        unsigned chain_length = s->max_chain_length;/* max hash chain length */
        Bytef *scan = s->window + s->strstart; /* current string */
        Bytef *match;                       /* matched string */
        int len;                           /* length of current match */
        int best_len = s->prev_length;              /* best match length so far */
        int nice_match = s->nice_match;             /* stop if match long enough */
        IPos limit = s->strstart > (IPos)MAX_DIST(s) ?
            s->strstart - (IPos)MAX_DIST(s) : NIL;
        /* Stop when cur_match becomes <= limit. To simplify the code,
         * we prevent matches with the string of window index 0.
         */
        Posf *prev = s->prev;
        uInt wmask = s->w_mask;

    #ifdef UNALIGNED_OK
        /* Compare two bytes at a time. Note: this is not always beneficial.
         * Try with and without -DUNALIGNED_OK to check.
         */
        Bytef *strend = s->window + s->strstart + MAX_MATCH - 1;
        u16 scan_start = *(ushf*)scan;
        u16 scan_end   = *(ushf*)(scan+best_len-1);
    #else
        Bytef *strend = s->window + s->strstart + MAX_MATCH;
        Byte scan_end1  = scan[best_len-1];
        Byte scan_end   = scan[best_len];
    #endif

        /* The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
         * It is easy to get rid of this optimization if necessary.
         */
        Assert(s->hash_bits >= 8 && MAX_MATCH == 258, "Code too clever");

        /* Do not waste too much time if we already have a good match: */
        if (s->prev_length >= s->good_match) {
            chain_length >>= 2;
        }
        /* Do not look for matches beyond the end of the input. This is necessary
         * to make deflate deterministic.
         */
        if ((uInt)nice_match > s->lookahead) nice_match = s->lookahead;

        Assert((ulg)s->strstart <= s->window_size-MIN_LOOKAHEAD, "need lookahead");

        do {
            Assert(cur_match < s->strstart, "no future");
            match = s->window + cur_match;

            /* Skip to next match if the match length cannot increase
             * or if the match length is less than 2.  Note that the checks below
             * for insufficient lookahead only occur occasionally for performance
             * reasons.  Therefore uninitialized memory will be accessed, and
             * conditional jumps will be made that depend on those values.
             * However the length of the match is limited to the lookahead, so
             * the output of deflate is not affected by the uninitialized values.
             */
    #if (defined(UNALIGNED_OK) && MAX_MATCH == 258)
            /* This code assumes sizeof(unsigned short) == 2. Do not use
             * UNALIGNED_OK if your compiler uses a different size.
             */
            if (*(ushf*)(match+best_len-1) != scan_end ||
                *(ushf*)match != scan_start) continue;

            /* It is not necessary to compare scan[2] and match[2] since they are
             * always equal when the other bytes match, given that the hash keys
             * are equal and that HASH_BITS >= 8. Compare 2 bytes at a time at
             * strstart+3, +5, ... up to strstart+257. We check for insufficient
             * lookahead only every 4th comparison; the 128th check will be made
             * at strstart+257. If MAX_MATCH-2 is not a multiple of 8, it is
             * necessary to put more guard bytes at the end of the window, or
             * to check more often for insufficient lookahead.
             */
            Assert(scan[2] == match[2], "scan[2]?");
            scan++, match++;
            do {
            } while (*(ushf*)(scan+=2) == *(ushf*)(match+=2) &&
                     *(ushf*)(scan+=2) == *(ushf*)(match+=2) &&
                     *(ushf*)(scan+=2) == *(ushf*)(match+=2) &&
                     *(ushf*)(scan+=2) == *(ushf*)(match+=2) &&
                     scan < strend);
            /* The funny "do {}" generates better code on most compilers */

            /* Here, scan <= window+strstart+257 */
            Assert(scan <= s->window+(unsigned)(s->window_size-1), "wild scan");
            if (*scan == *match) scan++;

            len = (MAX_MATCH - 1) - (int)(strend-scan);
            scan = strend - (MAX_MATCH-1);

    #else /* UNALIGNED_OK */

            if (match[best_len]   != scan_end  ||
                match[best_len-1] != scan_end1 ||
                *match            != *scan     ||
                *++match          != scan[1])      continue;

            /* The check at best_len-1 can be removed because it will be made
             * again later. (This heuristic is not always a win.)
             * It is not necessary to compare scan[2] and match[2] since they
             * are always equal when the other bytes match, given that
             * the hash keys are equal and that HASH_BITS >= 8.
             */
            scan += 2, match++;
            Assert(*scan == *match, "match[2]?");

            /* We check for insufficient lookahead only every 8th comparison;
             * the 256th check will be made at strstart+258.
             */
            do {
            } while (*++scan == *++match && *++scan == *++match &&
                     *++scan == *++match && *++scan == *++match &&
                     *++scan == *++match && *++scan == *++match &&
                     *++scan == *++match && *++scan == *++match &&
                     scan < strend);

            Assert(scan <= s->window+(unsigned)(s->window_size-1), "wild scan");

            len = MAX_MATCH - (int)(strend - scan);
            scan = strend - MAX_MATCH;

    #endif /* UNALIGNED_OK */

            if (len > best_len) {
                s->match_start = cur_match;
                best_len = len;
                if (len >= nice_match) break;
    #ifdef UNALIGNED_OK
                scan_end = *(ushf*)(scan+best_len-1);
    #else
                scan_end1  = scan[best_len-1];
                scan_end   = scan[best_len];
    #endif
            }
        } while ((cur_match = prev[cur_match & wmask]) > limit
                 && --chain_length != 0);

        if ((uInt)best_len <= s->lookahead) return (uInt)best_len;
        return s->lookahead;
    */
}

/**
  | Optimized version for level == 1 or strategy
  | == Z_RLE only
  |
  */
pub fn longest_match_fast(
        s:         *mut DeflateState,
        cur_match: IPos) -> u32 {
    
    todo!();
    /*
        Bytef *scan = s->window + s->strstart; /* current string */
        Bytef *match;                       /* matched string */
        int len;                           /* length of current match */
        Bytef *strend = s->window + s->strstart + MAX_MATCH;

        /* The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
         * It is easy to get rid of this optimization if necessary.
         */
        Assert(s->hash_bits >= 8 && MAX_MATCH == 258, "Code too clever");

        Assert((ulg)s->strstart <= s->window_size-MIN_LOOKAHEAD, "need lookahead");

        Assert(cur_match < s->strstart, "no future");

        match = s->window + cur_match;

        /* Return failure if the match length is less than 2:
         */
        if (match[0] != scan[0] || match[1] != scan[1]) return MIN_MATCH-1;

        /* The check at best_len-1 can be removed because it will be made
         * again later. (This heuristic is not always a win.)
         * It is not necessary to compare scan[2] and match[2] since they
         * are always equal when the other bytes match, given that
         * the hash keys are equal and that HASH_BITS >= 8.
         */
        scan += 2, match += 2;
        Assert(*scan == *match, "match[2]?");

        /* We check for insufficient lookahead only every 8th comparison;
         * the 256th check will be made at strstart+258.
         */
        do {
        } while (*++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 scan < strend);

        Assert(scan <= s->window+(unsigned)(s->window_size-1), "wild scan");

        len = MAX_MATCH - (int)(strend - scan);

        if (len < MIN_MATCH) return MIN_MATCH - 1;

        s->match_start = cur_match;
        return (uInt)len <= s->lookahead ? (uInt)len : s->lookahead;
    */
}

/**
  | Check that the match at match_start
  | is indeed a match.
  |
  */
#[cfg(DEBUG)]
pub fn check_match(
        s:      *mut DeflateState,
        start:  IPos,
        match_: IPos,
        length: i32)  {
    
    todo!();
    /*
        /* check that the match is indeed a match */
        if (zmemcmp(s->window + match,
                    s->window + start, length) != EQUAL) {
            fprintf(stderr, " start %u, match %u, length %d\n",
                    start, match, length);
            do {
                fprintf(stderr, "%c%c", s->window[match++], s->window[start++]);
            } while (--length != 0);
            z_error("invalid match");
        }
        if (z_verbose > 1) {
            fprintf(stderr,"\\[%d,%d]", start-match, length);
            do { putc(s->window[start++], stderr); } while (--length != 0);
        }
    */
}

#[cfg(not(DEBUG))]
macro_rules! check_match {
    ($s:ident, 
     $start:ident, 
     $match:ident, 
     $length:ident) => { }
}

/**
  | Fill the window when the lookahead becomes
  | insufficient. Updates strstart and
  | lookahead.
  | 
  | IN assertion: lookahead < MIN_LOOKAHEAD
  | 
  | OUT assertions: strstart <= window_size-MIN_LOOKAHEAD
  | 
  | At least one byte has been read, or avail_in
  | == 0; reads are performed for at least
  | two bytes (required for the zip translate_eol
  | option -- not supported here).
  |
  */
pub fn fill_window(s: *mut DeflateState)  {
    
    todo!();
    /*
        unsigned n, m;
        Posf *p;
        unsigned more;    /* Amount of free space at the end of the window. */
        uInt wsize = s->w_size;

        do {
            more = (unsigned)(s->window_size -(ulg)s->lookahead -(ulg)s->strstart);

            /* Deal with !@#$% 64K limit: */
            if (sizeof(int) <= 2) {
                if (more == 0 && s->strstart == 0 && s->lookahead == 0) {
                    more = wsize;

                } else if (more == (unsigned)(-1)) {
                    /* Very unlikely, but possible on 16 bit machine if
                     * strstart == 0 && lookahead == 1 (input done a byte at time)
                     */
                    more--;
                }
            }

            /* If the window is almost full and there is insufficient lookahead,
             * move the upper half to the lower one to make room in the upper half.
             */
            if (s->strstart >= wsize+MAX_DIST(s)) {

                zmemcpy(s->window, s->window+wsize, (unsigned)wsize);
                s->match_start -= wsize;
                s->strstart    -= wsize; /* we now have strstart >= MAX_DIST */
                s->block_start -= (long) wsize;

                /* Slide the hash table (could be avoided with 32 bit values
                   at the expense of memory usage). We slide even when level == 0
                   to keep the hash table consistent if we switch back to level > 0
                   later. (Using level 0 permanently is not an optimal usage of
                   zlib, so we don't care about this pathological case.)
                 */
                /* %%% avoid this when Z_RLE */
                n = s->hash_size;
                p = &s->head[n];
                do {
                    m = *--p;
                    *p = (Pos)(m >= wsize ? m-wsize : NIL);
                } while (--n);

                n = wsize;
    #ifndef FASTEST
                p = &s->prev[n];
                do {
                    m = *--p;
                    *p = (Pos)(m >= wsize ? m-wsize : NIL);
                    /* If n is not on any hash chain, prev[n] is garbage but
                     * its value will never be used.
                     */
                } while (--n);
    #endif
                more += wsize;
            }
            if (s->strm->avail_in == 0) return;

            /* If there was no sliding:
             *    strstart <= WSIZE+MAX_DIST-1 && lookahead <= MIN_LOOKAHEAD - 1 &&
             *    more == window_size - lookahead - strstart
             * => more >= window_size - (MIN_LOOKAHEAD-1 + WSIZE + MAX_DIST-1)
             * => more >= window_size - 2*WSIZE + 2
             * In the BIG_MEM or MMAP case (not yet supported),
             *   window_size == input_size + MIN_LOOKAHEAD  &&
             *   strstart + s->lookahead <= input_size => more >= MIN_LOOKAHEAD.
             * Otherwise, window_size == 2*WSIZE so more >= 2.
             * If there was sliding, more >= WSIZE. So in all cases, more >= 2.
             */
            Assert(more >= 2, "more < 2");

            n = read_buf(s->strm, s->window + s->strstart + s->lookahead, more);
            s->lookahead += n;

            /* Initialize the hash value now that we have some input: */
            if (s->lookahead >= MIN_MATCH) {
                s->ins_h = s->window[s->strstart];
                UPDATE_HASH(s, s->ins_h, s->window[s->strstart+1]);
    #if MIN_MATCH != 3
                Call UPDATE_HASH() MIN_MATCH-3 more times
    #endif
            }
            /* If the whole input has less than MIN_MATCH bytes, ins_h is garbage,
             * but this is not important since only literal bytes will be emitted.
             */

        } while (s->lookahead < MIN_LOOKAHEAD && s->strm->avail_in != 0);
    */
}

/**
  | Flush the current block, with given
  | end-of-file flag. IN assertion: strstart
  | is set to the end of the current match.
  |
  */
macro_rules! flush_block_only {
    ($s:ident, $eof:ident) => {
        /*
                { 
           _tr_flush_block(s, (s->block_start >= 0L ? 
                           (charf *)&s->window[(unsigned)s->block_start] : 
                           (charf *)Z_NULL), 
                        (ulg)((long)s->strstart - s->block_start), 
                        (eof)); 
           s->block_start = s->strstart; 
           flush_pending(s->strm); 
           Tracev((stderr,"[FLUSH]")); 
        }
        */
    }
}

/**
  | Same but force premature exit if necessary.
  |
  */
macro_rules! flush_block {
    ($s:ident, 
     $eof:ident) => {
        /*
                { 
           FLUSH_BLOCK_ONLY(s, eof); 
           if (s->strm->avail_out == 0) return (eof) ? finish_started : need_more; 
        }
        */
    }
}

/**
  | Copy without compression as much as
  | possible from the input stream, return
  | the current block state. This function
  | does not insert new strings in the dictionary
  | since uncompressible data is probably
  | not useful. This function is used only
  | for the level=0 compression option.
  | 
  | -----------
  | @note
  | 
  | this function should be optimized to
  | avoid extra copying from window to pending_buf.
  |
  */
pub fn deflate_stored(
        s:     *mut DeflateState,
        flush: i32) -> BlockState {
    
    todo!();
    /*
        /* Stored blocks are limited to 0xffff bytes, pending_buf is limited
         * to pending_buf_size, and each stored block has a 5 byte header:
         */
        ulg max_block_size = 0xffff;
        ulg max_start;

        if (max_block_size > s->pending_buf_size - 5) {
            max_block_size = s->pending_buf_size - 5;
        }

        /* Copy as much as possible from input to output: */
        for (;;) {
            /* Fill the window as much as possible: */
            if (s->lookahead <= 1) {

                Assert(s->strstart < s->w_size+MAX_DIST(s) ||
                       s->block_start >= (long)s->w_size, "slide too late");

                fill_window(s);
                if (s->lookahead == 0 && flush == Z_NO_FLUSH) return need_more;

                if (s->lookahead == 0) break; /* flush the current block */
            }
            Assert(s->block_start >= 0L, "block gone");

            s->strstart += s->lookahead;
            s->lookahead = 0;

            /* Emit a stored block if pending_buf will be full: */
            max_start = s->block_start + max_block_size;
            if (s->strstart == 0 || (ulg)s->strstart >= max_start) {
                /* strstart == 0 is possible when wraparound on 16-bit machine */
                s->lookahead = (uInt)(s->strstart - max_start);
                s->strstart = (uInt)max_start;
                FLUSH_BLOCK(s, 0);
            }
            /* Flush if we may have to slide, otherwise block_start may become
             * negative and the data will be gone:
             */
            if (s->strstart - (uInt)s->block_start >= MAX_DIST(s)) {
                FLUSH_BLOCK(s, 0);
            }
        }
        FLUSH_BLOCK(s, flush == Z_FINISH);
        return flush == Z_FINISH ? finish_done : block_done;
    */
}

/**
  | Compress as much as possible from the
  | input stream, return the current block
  | state.
  | 
  | This function does not perform lazy
  | evaluation of matches and inserts new
  | strings in the dictionary only for unmatched
  | strings or for short matches. It is used
  | only for the fast compression options.
  |
  */
pub fn deflate_fast(
        s:     *mut DeflateState,
        flush: i32) -> BlockState {
    
    todo!();
    /*
        IPos hash_head = NIL; /* head of the hash chain */
        int bflush;           /* set if current block must be flushed */

        for (;;) {
            /* Make sure that we always have enough lookahead, except
             * at the end of the input file. We need MAX_MATCH bytes
             * for the next match, plus MIN_MATCH bytes to insert the
             * string following the next match.
             */
            if (s->lookahead < MIN_LOOKAHEAD) {
                fill_window(s);
                if (s->lookahead < MIN_LOOKAHEAD && flush == Z_NO_FLUSH) {
                    return need_more;
                }
                if (s->lookahead == 0) break; /* flush the current block */
            }

            /* Insert the string window[strstart .. strstart+2] in the
             * dictionary, and set hash_head to the head of the hash chain:
             */
            if (s->lookahead >= MIN_MATCH) {
                INSERT_STRING(s, s->strstart, hash_head);
            }

            /* Find the longest match, discarding those <= prev_length.
             * At this point we have always match_length < MIN_MATCH
             */
            if (hash_head != NIL && s->strstart - hash_head <= MAX_DIST(s)) {
                /* To simplify the code, we prevent matches with the string
                 * of window index 0 (in particular we have to avoid a match
                 * of the string with itself at the start of the input file).
                 */
    #ifdef FASTEST
                if ((s->strategy != Z_HUFFMAN_ONLY && s->strategy != Z_RLE) ||
                    (s->strategy == Z_RLE && s->strstart - hash_head == 1)) {
                    s->match_length = longest_match_fast (s, hash_head);
                }
    #else
                if (s->strategy != Z_HUFFMAN_ONLY && s->strategy != Z_RLE) {
                    s->match_length = longest_match (s, hash_head);
                } else if (s->strategy == Z_RLE && s->strstart - hash_head == 1) {
                    s->match_length = longest_match_fast (s, hash_head);
                }
    #endif
                /* longest_match() or longest_match_fast() sets match_start */
            }
            if (s->match_length >= MIN_MATCH) {
                check_match(s, s->strstart, s->match_start, s->match_length);

                _tr_tally_dist(s, s->strstart - s->match_start,
                               s->match_length - MIN_MATCH, bflush);

                s->lookahead -= s->match_length;

                /* Insert new strings in the hash table only if the match length
                 * is not too large. This saves time but degrades compression.
                 */
    #ifndef FASTEST
                if (s->match_length <= s->max_insert_length &&
                    s->lookahead >= MIN_MATCH) {
                    s->match_length--; /* string at strstart already in table */
                    do {
                        s->strstart++;
                        INSERT_STRING(s, s->strstart, hash_head);
                        /* strstart never exceeds WSIZE-MAX_MATCH, so there are
                         * always MIN_MATCH bytes ahead.
                         */
                    } while (--s->match_length != 0);
                    s->strstart++;
                } else
    #endif
                {
                    s->strstart += s->match_length;
                    s->match_length = 0;
                    s->ins_h = s->window[s->strstart];
                    UPDATE_HASH(s, s->ins_h, s->window[s->strstart+1]);
    #if MIN_MATCH != 3
                    Call UPDATE_HASH() MIN_MATCH-3 more times
    #endif
                    /* If lookahead < MIN_MATCH, ins_h is garbage, but it does not
                     * matter since it will be recomputed at next deflate call.
                     */
                }
            } else {
                /* No match, output a literal byte */
                Tracevv((stderr,"%c", s->window[s->strstart]));
                _tr_tally_lit (s, s->window[s->strstart], bflush);
                s->lookahead--;
                s->strstart++;
            }
            if (bflush) FLUSH_BLOCK(s, 0);
        }
        FLUSH_BLOCK(s, flush == Z_FINISH);
        return flush == Z_FINISH ? finish_done : block_done;
    */
}

/**
  | Same as above, but achieves better compression.
  | We use a lazy evaluation for matches:
  | a match is finally adopted only if there
  | is no better match at the next window
  | position.
  |
  */
#[cfg(not(FASTEST))]
pub fn deflate_slow(
        s:     *mut DeflateState,
        flush: i32) -> BlockState {
    
    todo!();
    /*
        IPos hash_head = NIL;    /* head of hash chain */
        int bflush;              /* set if current block must be flushed */

        /* Process the input block. */
        for (;;) {
            /* Make sure that we always have enough lookahead, except
             * at the end of the input file. We need MAX_MATCH bytes
             * for the next match, plus MIN_MATCH bytes to insert the
             * string following the next match.
             */
            if (s->lookahead < MIN_LOOKAHEAD) {
                fill_window(s);
                if (s->lookahead < MIN_LOOKAHEAD && flush == Z_NO_FLUSH) {
                    return need_more;
                }
                if (s->lookahead == 0) break; /* flush the current block */
            }

            /* Insert the string window[strstart .. strstart+2] in the
             * dictionary, and set hash_head to the head of the hash chain:
             */
            if (s->lookahead >= MIN_MATCH) {
                INSERT_STRING(s, s->strstart, hash_head);
            }

            /* Find the longest match, discarding those <= prev_length.
             */
            s->prev_length = s->match_length, s->prev_match = s->match_start;
            s->match_length = MIN_MATCH-1;

            if (hash_head != NIL && s->prev_length < s->max_lazy_match &&
                s->strstart - hash_head <= MAX_DIST(s)) {
                /* To simplify the code, we prevent matches with the string
                 * of window index 0 (in particular we have to avoid a match
                 * of the string with itself at the start of the input file).
                 */
                if (s->strategy != Z_HUFFMAN_ONLY && s->strategy != Z_RLE) {
                    s->match_length = longest_match (s, hash_head);
                } else if (s->strategy == Z_RLE && s->strstart - hash_head == 1) {
                    s->match_length = longest_match_fast (s, hash_head);
                }
                /* longest_match() or longest_match_fast() sets match_start */

                if (s->match_length <= 5 && (s->strategy == Z_FILTERED
    #if TOO_FAR <= 32767
                    || (s->match_length == MIN_MATCH &&
                        s->strstart - s->match_start > TOO_FAR)
    #endif
                    )) {

                    /* If prev_match is also MIN_MATCH, match_start is garbage
                     * but we will ignore the current match anyway.
                     */
                    s->match_length = MIN_MATCH-1;
                }
            }
            /* If there was a match at the previous step and the current
             * match is not better, output the previous match:
             */
            if (s->prev_length >= MIN_MATCH && s->match_length <= s->prev_length) {
                uInt max_insert = s->strstart + s->lookahead - MIN_MATCH;
                /* Do not insert strings in hash table beyond this. */

                check_match(s, s->strstart-1, s->prev_match, s->prev_length);

                _tr_tally_dist(s, s->strstart -1 - s->prev_match,
                               s->prev_length - MIN_MATCH, bflush);

                /* Insert in hash table all strings up to the end of the match.
                 * strstart-1 and strstart are already inserted. If there is not
                 * enough lookahead, the last two strings are not inserted in
                 * the hash table.
                 */
                s->lookahead -= s->prev_length-1;
                s->prev_length -= 2;
                do {
                    if (++s->strstart <= max_insert) {
                        INSERT_STRING(s, s->strstart, hash_head);
                    }
                } while (--s->prev_length != 0);
                s->match_available = 0;
                s->match_length = MIN_MATCH-1;
                s->strstart++;

                if (bflush) FLUSH_BLOCK(s, 0);

            } else if (s->match_available) {
                /* If there was no match at the previous position, output a
                 * single literal. If there was a match but the current match
                 * is longer, truncate the previous match to a single literal.
                 */
                Tracevv((stderr,"%c", s->window[s->strstart-1]));
                _tr_tally_lit(s, s->window[s->strstart-1], bflush);
                if (bflush) {
                    FLUSH_BLOCK_ONLY(s, 0);
                }
                s->strstart++;
                s->lookahead--;
                if (s->strm->avail_out == 0) return need_more;
            } else {
                /* There is no previous match to compare with, wait for
                 * the next step to decide.
                 */
                s->match_available = 1;
                s->strstart++;
                s->lookahead--;
            }
        }
        Assert (flush != Z_NO_FLUSH, "no flush?");
        if (s->match_available) {
            Tracevv((stderr,"%c", s->window[s->strstart-1]));
            _tr_tally_lit(s, s->window[s->strstart-1], bflush);
            s->match_available = 0;
        }
        FLUSH_BLOCK(s, flush == Z_FINISH);
        return flush == Z_FINISH ? finish_done : block_done;
    */
}
