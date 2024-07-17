/*!
  | jinclude.h
  | 
  | This file exists to provide a single
  | place to fix any problems with including
  | the wrong system include files. (Common
  | problems are taken care of by the standard
  | jconfig symbols, but on really weird
  | systems you may have to edit this file.)
  | 
  | -----------
  | @note
  | 
  | this file is NOT intended to be included
  | by applications using the
  | 
  | JPEG library. Most applications need
  | only include jpeglib.h.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jinclude.h]

/**
  | We need the NULL macro and size_t typedef.
  | 
  | On an ANSI-conforming system it is sufficient
  | to include <stddef.h>.
  | 
  | Otherwise, we get them from <stdlib.h>
  | or <stdio.h>; we may have to pull in <sys/types.h>
  | as well.
  | 
  | -----------
  | @note
  | 
  | the core JPEG library does not require
  | <stdio.h>; only the default error handler
  | and data source/destination modules
  | do.
  | 
  | But we must pull it in because of the references
  | to FILE in jpeglib.h.
  | 
  | You can remove those references if you
  | want to compile without <stdio.h>.
  |
  ------------------------------
  | We need memory copying and zeroing functions,
  | plus strncpy().
  | 
  | ANSI and System V implementations declare
  | these in <string.h>.
  | 
  | BSD doesn't have the mem() functions,
  | but it does have bcopy()/bzero().
  | 
  | Some systems may declare memset and
  | memcpy in <memory.h>.
  | 
  | -----------
  | @note
  | 
  | we assume the size parameters to these
  | functions are of type size_t.
  | 
  | Change the casts in these macros if not!
  |
  */
#[cfg(NEED_BSD_STRINGS)]
macro_rules! memzero {
    ($target:ident, $size:ident) => {
        /*
                bzero((c_void *)(target), (size_t)(size))
        */
    }
}

#[cfg(NEED_BSD_STRINGS)]
macro_rules! memcopy {
    ($dest:ident, $src:ident, $size:ident) => {
        /*
                bcopy((const c_void *)(src), (c_void *)(dest), (size_t)(size))
        */
    }
}


/**
  | not BSD, assume ANSI/SysV string lib
  |
  */
#[cfg(not(NEED_BSD_STRINGS))]
macro_rules! memzero {
    ($target:ident, $size:ident) => {
        /*
                memset((c_void *)(target), 0, (size_t)(size))
        */
    }
}

#[cfg(not(NEED_BSD_STRINGS))]
macro_rules! memcopy {
    ($dest:ident, $src:ident, $size:ident) => {
        /*
                memcpy((c_void *)(dest), (const c_void *)(src), (size_t)(size))
        */
    }
}

/**
  | In ANSI C, and indeed any rational implementation,
  | size_t is also the type returned by sizeof().
  | However, it seems there are some irrational
  | implementations out there, in which
  | sizeof() returns an int even though
  | size_t is defined as long or unsigned
  | long. To ensure consistent results
  | we always use this SIZEOF() macro in
  | place of using sizeof() directly.
  |
  */
macro_rules! sizeof {
    ($object:ident) => {
        /*
                ((size_t) sizeof(object))
        */
    }
}

/**
  | The modules that use fread() and fwrite()
  | always invoke them through these macros.
  | On some systems you may need to twiddle
  | the argument casts.
  | 
  | CAUTION: argument order is different
  | from underlying functions!
  |
  */
macro_rules! jfread {
    ($file:ident, 
     $buf:ident, 
     $sizeofbuf:ident) => {
        /*
        
          ((size_t) fread((c_void *) (buf), (size_t) 1, (size_t) (sizeofbuf), (file)))
        */
    }
}

macro_rules! jfwrite {
    ($file:ident, 
     $buf:ident, 
     $sizeofbuf:ident) => {
        /*
        
          ((size_t) fwrite((const c_void *) (buf), (size_t) 1, (size_t) (sizeofbuf), (file)))
        */
    }
}

/**
  | JPEG marker codes
  |
  */
pub enum JpegMarker {
  M_SOF0  = 0xc0,
  M_SOF1  = 0xc1,
  M_SOF2  = 0xc2,
  M_SOF3  = 0xc3,

  M_SOF5  = 0xc5,
  M_SOF6  = 0xc6,
  M_SOF7  = 0xc7,

  M_JPG   = 0xc8,
  M_SOF9  = 0xc9,
  M_SOF10 = 0xca,
  M_SOF11 = 0xcb,

  M_SOF13 = 0xcd,
  M_SOF14 = 0xce,
  M_SOF15 = 0xcf,

  M_DHT   = 0xc4,

  M_DAC   = 0xcc,

  M_RST0  = 0xd0,
  M_RST1  = 0xd1,
  M_RST2  = 0xd2,
  M_RST3  = 0xd3,
  M_RST4  = 0xd4,
  M_RST5  = 0xd5,
  M_RST6  = 0xd6,
  M_RST7  = 0xd7,

  M_SOI   = 0xd8,
  M_EOI   = 0xd9,
  M_SOS   = 0xda,
  M_DQT   = 0xdb,
  M_DNL   = 0xdc,
  M_DRI   = 0xdd,
  M_DHP   = 0xde,
  M_EXP   = 0xdf,

  M_APP0  = 0xe0,
  M_APP1  = 0xe1,
  M_APP2  = 0xe2,
  M_APP3  = 0xe3,
  M_APP4  = 0xe4,
  M_APP5  = 0xe5,
  M_APP6  = 0xe6,
  M_APP7  = 0xe7,
  M_APP8  = 0xe8,
  M_APP9  = 0xe9,
  M_APP10 = 0xea,
  M_APP11 = 0xeb,
  M_APP12 = 0xec,
  M_APP13 = 0xed,
  M_APP14 = 0xee,
  M_APP15 = 0xef,

  M_JPG0  = 0xf0,
  M_JPG13 = 0xfd,
  M_COM   = 0xfe,

  M_TEM   = 0x01,

  M_ERROR = 0x100
}

/**
  | Figure F.12: extend sign bit.
  | 
  | On some machines, a shift and add will
  | be faster than a table lookup.
  |
  */
#[cfg(AVOID_TABLES)]
macro_rules! huff_extend {
    ($x:ident, $s:ident) => {
        /*
                ((x) < (1<<((s)-1)) ? (x) + (((-1)<<(s)) + 1) : (x))
        */
    }
}

#[cfg(not(AVOID_TABLES))]
macro_rules! huff_extend {
    ($x:ident, $s:ident) => {
        /*
                ((x) < extend_test[s] ? (x) + extend_offset[s] : (x))
        */
    }
}

/**
  | entry n is 2**(n-1)
  |
  */
#[cfg(not(AVOID_TABLES))]
pub const extend_test: [i32; 16] = [ 
    0      , 0x0001 , 0x0002 , 0x0004 , 0x0008 , 0x0010 , 0x0020 , 0x0040 , 
    0x0080 , 0x0100 , 0x0200 , 0x0400 , 0x0800 , 0x1000 , 0x2000 , 0x4000
];

#[cfg(not(AVOID_TABLES))]
macro_rules! shifted_bits_plus_one {
    ($n:expr) => {
        (u32::MAX << $n) as i32 + 1
    }
}

/**
  | entry n is (-1 << n) + 1
  |
  */
#[cfg(not(AVOID_TABLES))]
pub const extend_offset: [i32; 16] = [ 
    0                          , shifted_bits_plus_one!(1)  , shifted_bits_plus_one!(2)  , shifted_bits_plus_one!(3)  , 
    shifted_bits_plus_one!(4)  , shifted_bits_plus_one!(5)  , shifted_bits_plus_one!(6)  , shifted_bits_plus_one!(7)  , 
    shifted_bits_plus_one!(8)  , shifted_bits_plus_one!(9)  , shifted_bits_plus_one!(10) , shifted_bits_plus_one!(11) , 
    shifted_bits_plus_one!(12) , shifted_bits_plus_one!(13) , shifted_bits_plus_one!(14) , shifted_bits_plus_one!(15)
];
