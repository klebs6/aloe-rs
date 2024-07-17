/*!
  | jutils.c
  | 
  | This file contains tables and miscellaneous
  | utility routines needed for both compression
  | and decompression.
  | 
  | Note we prefix all global names with
  | "j" to minimize conflicts with a surrounding
  | application.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jutils.c]

/**
  | jpeg_zigzag_order[i] is the zigzag-order
  | position of the i'th element of a DCT
  | block read in natural order (left to
  | right, top to bottom).
  |
  | This table is not actually needed in
  | v6a
  |
  */
#[ignore]
pub const jpeg_zigzag_order: [i32; DCTSIZE2] = [
   0,  1,  5,  6, 14, 15, 27, 28,
   2,  4,  7, 13, 16, 26, 29, 42,
   3,  8, 12, 17, 25, 30, 41, 43,
   9, 11, 18, 24, 31, 40, 44, 53,
  10, 19, 23, 32, 39, 45, 52, 54,
  20, 22, 33, 38, 46, 51, 55, 60,
  21, 34, 37, 47, 50, 56, 59, 61,
  35, 36, 48, 49, 57, 58, 62, 63
];

/**
  | jpeg_natural_order[i] is the natural-order
  | position of the i'th element of zigzag
  | order.
  | 
  | When reading corrupted data, the Huffman
  | decoders could attempt to reference
  | an entry beyond the end of this array
  | (if the decoded zero run length reaches
  | past the end of the block). To prevent
  | wild stores without adding an inner-loop
  | test, we put some extra "63"s after the
  | real entries. This will cause the extra
  | coefficient to be stored in location
  | 63 of the block, not somewhere random.
  | 
  | The worst case would be a run-length
  | of 15, which means we need 16 fake entries.
  |
  */
pub const jpeg_natural_order: [i32; DCTSIZE2+16] = [
    0,  1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63,
    63, 63, 63, 63, 63, 63, 63, 63, /* extra entries for safety in decoder */
    63, 63, 63, 63, 63, 63, 63, 63
];

/**
  | Arithmetic utilities
  |
  | Compute a/b rounded up to next integer,
  | ie, ceil(a/b)
  | 
  | Assumes a >= 0, b > 0
  |
  */
pub fn jdiv_round_up(a: i64, b: i64) -> i64 {
    
    todo!();
        /*
            return (a + b - 1L) / b;
        */
}

/**
  | Compute a rounded up to next multiple
  | of b, ie, ceil(a/b)*b
  | 
  | Assumes a >= 0, b > 0
  |
  */
pub fn jround_up(a: i64, b: i64) -> i64 {
    
    todo!();
        /*
            a += b - 1L;
      return a - (a % b);
        */
}

/**
  | On normal machines we can apply MEMCOPY()
  | and MEMZERO() to sample arrays and coefficient-block
  | arrays. This won't work on 80x86 because
  | the arrays are FAR and we're assuming
  | a small-pointer memory model. However,
  | some
  | 
  | DOS compilers provide far-pointer
  | versions of memcpy() and memset() even
  | in the small-model libraries. These
  | will be used if USE_FMEM is defined.
  | 
  | Otherwise, the routines below do it
  | the hard way. (The performance cost
  | is not all that great, because these
  | routines aren't very heavily used.)
  |
  | normal case, same as regular macros
  |
  */
#[cfg(not(NEED_FAR_POINTERS))]
macro_rules! fmemcopy {
    ($dest:ident, $src:ident, $size:ident) => {
        /*
                MEMCOPY(dest,src,size)
        */
    }
}

#[cfg(not(NEED_FAR_POINTERS))]
macro_rules! fmemzero {
    ($target:ident, $size:ident) => {
        /*
                MEMZERO(target,size)
        */
    }
}

/**
  | 80x86 case, define if we can
  |
  */
#[cfg(NEED_FAR_POINTERS)]
#[cfg(USE_FMEM)]
macro_rules! fmemcopy {
    ($dest:ident, $src:ident, $size:ident) => {
        /*
                _fmemcpy((c_void FAR *)(dest), (const c_void FAR *)(src), (size_t)(size))
        */
    }
}

#[cfg(NEED_FAR_POINTERS)]
#[cfg(USE_FMEM)]
macro_rules! fmemzero {
    ($target:ident, $size:ident) => {
        /*
                _fmemset((c_void FAR *)(target), 0, (size_t)(size))
        */
    }
}

/**
  | Copy some rows of samples from one place
  | to another. num_rows rows are copied
  | from input_array[source_row++] to
  | output_array[dest_row++]; these
  | areas may overlap for duplication.
  | 
  | The source and destination arrays must
  | be at least as wide as num_cols.
  |
  */
pub fn jcopy_sample_rows(
        input_array:  JSampArray,
        source_row:   i32,
        output_array: JSampArray,
        dest_row:     i32,
        num_rows:     i32,
        num_cols:     JDimension)  {
    
    todo!();
        /*
            JSAMPROW inptr, outptr;
    #ifdef FMEMCOPY
      size_t count = (size_t) (num_cols * SIZEOF(JSAMPLE));
    #else
      JDimension count;
    #endif
      int row;

      input_array += source_row;
      output_array += dest_row;

      for (row = num_rows; row > 0; row--) {
        inptr = *input_array++;
        outptr = *output_array++;
    #ifdef FMEMCOPY
        FMEMCOPY(outptr, inptr, count);
    #else
        for (count = num_cols; count > 0; count--)
          *outptr++ = *inptr++; /* needn't bother with GETJSAMPLE() here */
    #endif
      }
        */
}

/**
  | Copy a row of coefficient blocks from
  | one place to another.
  |
  */
pub fn jcopy_block_row(
        input_row:  JBlockRow,
        output_row: JBlockRow,
        num_blocks: JDimension)  {
    
    todo!();
        /*
            #ifdef FMEMCOPY
      FMEMCOPY(output_row, input_row, num_blocks * (DCTSIZE2 * SIZEOF(JCoef)));
    #else
      JCoefPtr inptr, outptr;
      long count;

      inptr = (JCoefPtr) input_row;
      outptr = (JCoefPtr) output_row;
      for (count = (long) num_blocks * DCTSIZE2; count > 0; count--) {
        *outptr++ = *inptr++;
      }
    #endif
        */
}

/**
  | Zero out a chunk of FAR memory.
  | 
  | This might be sample-array data, block-array
  | data, or alloc_large data.
  |
  */
pub fn jzero_far(
        target:      *mut c_void,
        bytestozero: usize)  {
    
    todo!();
        /*
            #ifdef FMEMZERO
      FMEMZERO(target, bytestozero);
    #else
      char FAR * ptr = (char FAR *) target;
      size_t count;

      for (count = bytestozero; count > 0; count--) {
        *ptr++ = 0;
      }
    #endif
        */
}
