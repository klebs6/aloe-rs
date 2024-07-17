crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/bitmath.h]

/**
  | Will never be emitted for MSVC, GCC,
  | Intel compilers
  |
  */
#[inline] pub fn flac_clz_soft_uint32(word: u32) -> u32 {
    
    todo!();
        /*
            static const unsigned char byte_to_unary_table[] = {
        8, 7, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        };

        return (word) > 0xffffff ? byte_to_unary_table[(word) >> 24] :
        (word) > 0xffff ? byte_to_unary_table[(word) >> 16] + 8 :
        (word) > 0xff ? byte_to_unary_table[(word) >> 8] + 16 :
        byte_to_unary_table[(word)] + 24;
        */
}

#[inline] pub fn flac_clz_uint32(v: u32) -> u32 {
    
    todo!();
        /*
            /* Never used with input 0 */
        ASSERT(v > 0);
    #if defined(__INTEL_COMPILER)
        return _bit_scan_reverse(v) ^ 31U;
    #elif defined(__GNUC__) && (__GNUC__ >= 4 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 4))
    /* This will translate either to (bsr ^ 31U), clz , ctlz, cntlz, lzcnt depending on
     * -march= setting or to a software routine in exotic machines. */
        return __builtin_clz(v);
    #elif defined(_MSC_VER) && (_MSC_VER >= 1400)
        {
            unsigned long idx;
            _BitScanReverse(&idx, v);
            return idx ^ 31U;
        }
    #else
        return clz_soft_uint32(v);
    #endif
        */
}

/**
  | This one works with input 0
  |
  */
#[inline] pub fn flac_clz2_uint32(v: u32) -> u32 {
    
    todo!();
        /*
            if (!v)
            return 32;
        return clz_uint32(v);
        */
}

/** 
 | An example of what bitmath_ilog2()
 | computes:
 |
 | ilog2( 0) = assertion failure
 | ilog2( 1) = 0
 | ilog2( 2) = 1
 | ilog2( 3) = 1
 | ilog2( 4) = 2
 | ilog2( 5) = 2
 | ilog2( 6) = 2
 | ilog2( 7) = 2
 | ilog2( 8) = 3
 | ilog2( 9) = 3
 | ilog2(10) = 3
 | ilog2(11) = 3
 | ilog2(12) = 3
 | ilog2(13) = 3
 | ilog2(14) = 3
 | ilog2(15) = 3
 | ilog2(16) = 4
 | ilog2(17) = 4
 | ilog2(18) = 4
 */
#[inline] pub fn flac_bitmath_ilog2(v: u32) -> u32 {
    
    todo!();
        /*
            ASSERT(v > 0);
    #if defined(__INTEL_COMPILER)
        return _bit_scan_reverse(v);
    #elif defined(_MSC_VER) && (_MSC_VER >= 1400)
        {
            unsigned long idx;
            _BitScanReverse(&idx, v);
            return idx;
        }
    #else
        return sizeof(u32) * CHAR_BIT  - 1 - clz_uint32(v);
    #endif
        */
}

/**
  | Unused otherwise
  |
  */
#[cfg(INTEGER_ONLY_LIBRARY)]
#[inline] pub fn flac_bitmath_ilog2_wide(v: u64) -> u32 {
    
    todo!();
        /*
            ASSERT(v > 0);
    #if defined(__GNUC__) && (__GNUC__ >= 4 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 4))
        return sizeof(u64) * CHAR_BIT - 1 - __builtin_clzll(v);
    /* Sorry, only supported in x64/Itanium.. and both have fast FPU which makes integer-only encoder pointless */
    #elif (defined(_MSC_VER) && (_MSC_VER >= 1400)) && (defined(_M_IA64) || defined(_M_X64))
        {
            unsigned long idx;
            _BitScanReverse64(&idx, v);
            return idx;
        }
    #else
    /*  Brain-damaged compilers will use the fastest possible way that is,
        de Bruijn sequences (http://supertech.csail.mit.edu/papers/debruijn.pdf)
        (C) Timothy B. Terriberry (tterribe@xiph.org) 2001-2009 CC0 (Public domain).
    */
        {
            static const unsigned char DEBRUIJN_IDX64[64]={
                0, 1, 2, 7, 3,13, 8,19, 4,25,14,28, 9,34,20,40,
                5,17,26,38,15,46,29,48,10,31,35,54,21,50,41,57,
                63, 6,12,18,24,27,33,39,16,37,45,47,30,53,49,56,
                62,11,23,32,36,44,52,55,61,22,43,51,60,42,59,58
            };
            v|= v>>1;
            v|= v>>2;
            v|= v>>4;
            v|= v>>8;
            v|= v>>16;
            v|= v>>32;
            v= (v>>1)+1;
            return DEBRUIJN_IDX64[v*0x218A392CD3D5DBF>>58&0x3F];
        }
    #endif
        */
}

pub fn flac_bitmath_silog2(v: i32) -> u32 {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitmath_silog2_wide(v: i64) -> u32 {
    
    todo!();
        /*
        
        */
}
