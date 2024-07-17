/*!
  | jidctint.c
  | 
  | This file contains a slow-but-accurate
  | integer implementation of the inverse
  | DCT (Discrete Cosine Transform). In
  | the IJG code, this routine must also
  | perform dequantization of the input
  | coefficients.
  | 
  | A 2-D IDCT can be done by 1-D IDCT on each
  | column followed by 1-D IDCT on each row
  | (or vice versa, but it's more convenient
  | to emit a row at a time). Direct algorithms
  | are also available, but they are much
  | more complex and seem not to be any faster
  | when reduced to code.
  | 
  | This implementation is based on an algorithm
  | described in
  | 
  | C. Loeffler, A. Ligtenberg and G. Moschytz,
  | "Practical Fast 1-D DCT
  | 
  | Algorithms with 11 Multiplications",
  | Proc. Int'l. Conf. on Acoustics,
  | 
  | Speech, and Signal Processing 1989
  | (ICASSP '89), pp. 988-991.
  | 
  | The primary algorithm described there
  | uses 11 multiplies and 29 adds.
  | 
  | We use their alternate method with 12
  | multiplies and 32 adds.
  | 
  | The advantage of this method is that
  | no data path contains more than one multiplication;
  | this allows a very simple and accurate
  | implementation in scaled fixed-point
  | arithmetic, with a minimal number of
  | shifts.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jidctint.c]

#[cfg(DCT_ISLOW_SUPPORTED)]
pub mod dct_islow_supported {
    use super::*;

    /**
      | This module is specialized to the case
      | DCTSIZE = 8.
      |
      */
    static_assert!{ DCTSIZE == 8 }

    /**
      | The poop on this scaling stuff is as follows:
      | 
      | Each 1-D IDCT step produces outputs
      | which are a factor of sqrt(N) larger
      | than the true IDCT outputs. The final
      | outputs are therefore a factor of N larger
      | than desired; since N=8 this can be cured
      | by a simple right shift at the end of the
      | algorithm. The advantage of this arrangement
      | is that we save two multiplications
      | per 1-D IDCT, because the y0 and y4 inputs
      | need not be divided by sqrt(N).
      | 
      | We have to do addition and subtraction
      | of the integer inputs, which is no problem,
      | and multiplication by fractional constants,
      | which is a problem to do in integer arithmetic.
      | We multiply all the constants by CONST_SCALE
      | and convert them to integer constants
      | (thus retaining CONST_BITS bits of precision 
      | in the constants). After doing a multiplication
      | we have to divide the product by CONST_SCALE,
      | with proper rounding, to produce the
      | correct output. This division can be
      | done cheaply as a right shift of CONST_BITS
      | bits. We postpone shifting as long as
      | possible so that partial sums can be
      | added together with full fractional
      | precision.
      | 
      | The outputs of the first pass are scaled
      | up by PASS1_BITS bits so that they are
      | represented to better-than-integral
      | precision. These outputs require BITS_IN_JSAMPLE
      | + PASS1_BITS + 3 bits; this fits in a 16-bit
      | word with the recommended scaling.
      | (To scale up 12-bit sample data further,
      | an intermediate i32 array would be
      | needed.)
      | 
      | To avoid overflow of the 32-bit intermediate
      | results in pass 2, we must have BITS_IN_JSAMPLE
      | + CONST_BITS + PASS1_BITS <= 26. Error
      | analysis shows that the values given
      | below are the most effective.
      |
      */
    #[cfg(BITS_IN_JSAMPLE_EQ_8)] pub const CONST_BITS: usize = 13;
    #[cfg(BITS_IN_JSAMPLE_EQ_8)] pub const PASS1_BITS: usize = 2;

    #[cfg(not(BITS_IN_JSAMPLE_EQ_8))] pub const CONST_BITS: usize = 13;
    #[cfg(not(BITS_IN_JSAMPLE_EQ_8))] pub const PASS1_BITS: usize = 1; /* lose a little precision to avoid overflow */

    /**
      | Some C compilers fail to reduce "FIX(constant)"
      | at compile time, thus causing a lot of
      | useless floating-point operations
      | at run time.
      | 
      | To get around this we use the following
      | pre-calculated constants.
      | 
      | If you change CONST_BITS you may want
      | to add appropriate values. (With a reasonable
      | C compiler, you can just rely on the FIX()
      | macro...)
      |
      */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_298631336: i32 = 2446 as i32;   /* FIX(0.298631336) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_390180644: i32 = 3196 as i32;   /* FIX(0.390180644) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_541196100: i32 = 4433 as i32;   /* FIX(0.541196100) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_765366865: i32 = 6270 as i32;   /* FIX(0.765366865) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_899976223: i32 = 7373 as i32;   /* FIX(0.899976223) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_175875602: i32 = 9633 as i32;   /* FIX(1.175875602) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_501321110: i32 = 12299 as i32;  /* FIX(1.501321110) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_847759065: i32 = 15137 as i32;  /* FIX(1.847759065) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_961570560: i32 = 16069 as i32;  /* FIX(1.961570560) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_2_053119869: i32 = 16819 as i32;  /* FIX(2.053119869) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_2_562915447: i32 = 20995 as i32;  /* FIX(2.562915447) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_3_072711026: i32 = 25172 as i32;  /* FIX(3.072711026) */

    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_298631336: i32 = FIX(0.298631336);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_390180644: i32 = FIX(0.390180644);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_541196100: i32 = FIX(0.541196100);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_765366865: i32 = FIX(0.765366865);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_899976223: i32 = FIX(0.899976223);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_175875602: i32 = FIX(1.175875602);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_501321110: i32 = FIX(1.501321110);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_847759065: i32 = FIX(1.847759065);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_961570560: i32 = FIX(1.961570560);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_2_053119869: i32 = FIX(2.053119869);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_2_562915447: i32 = FIX(2.562915447);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_3_072711026: i32 = FIX(3.072711026);

    /**
      | Multiply an i32 variable by an i32
      | constant to yield an i32 result.
      | 
      | For 8-bit samples with the recommended
      | scaling, all the variable and constant
      | values involved are no more than 16 bits
      | wide, so a 16x16->32 bit multiply can
      | be used instead of a full 32x32 multiply.
      | 
      | For 12-bit samples, a full 32-bit multiplication
      | will be needed.
      |
      */
    #[cfg(BITS_IN_JSAMPLE_EQ_8)]
    macro_rules! multiply {
        ($var:ident, $const:ident) => {
            /*
                    MULTIPLY16C16(var,const)
            */
        }
    }

    #[cfg(not(BITS_IN_JSAMPLE_EQ_8))]
    macro_rules! multiply {
        ($var:ident, $const:ident) => {
            /*
                    ((var) * (const))
            */
        }
    }

    /**
      | Dequantize a coefficient by multiplying
      | it by the multiplier-table entry; produce
      | an int result. In this module, both inputs
      | and result are 16 bits or less, so either
      | int or short multiply will work.
      |
      */
    macro_rules! dequantize {
        ($coef:ident, $quantval:ident) => {
            /*
                    (((ISLOW_MULT_TYPE) (coef)) * (quantval))
            */
        }
    }

    /**
      | Perform dequantization and inverse
      | DCT on one block of coefficients.
      |
      */
    pub fn jpeg_idct_islow(
            cinfo:      JDecompressPtr,
            compptr:    *mut JpegComponentInfo,
            coef_block: JCoefPtr,
            output_buf: JSampArray,
            output_col: JDimension)  {
        
        todo!();
            /*
                i32 tmp0, tmp1, tmp2, tmp3;
          i32 tmp10, tmp11, tmp12, tmp13;
          i32 z1, z2, z3, z4, z5;
          JCoefPtr inptr;
          ISLOW_MULT_TYPE * quantptr;
          int * wsptr;
          JSAMPROW outptr;
          JSAMPLE *range_limit = IDCT_range_limit(cinfo);
          int ctr;
          int workspace[DCTSIZE2];  /* buffers data between passes */
          SHIFT_TEMPS

          /* Pass 1: process columns from input, store into work array. */
          /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
          /* furthermore, we scale the results by 2**PASS1_BITS. */

          inptr = coef_block;
          quantptr = (ISLOW_MULT_TYPE *) compptr->dct_table;
          wsptr = workspace;
          for (ctr = DCTSIZE; ctr > 0; ctr--) {
            /* Due to quantization, we will usually find that many of the input
             * coefficients are zero, especially the AC terms.  We can exploit this
             * by short-circuiting the IDCT calculation for any column in which all
             * the AC terms are zero.  In that case each output is equal to the
             * DC coefficient (with scale factor as needed).
             * With typical images and quantization tables, half or more of the
             * column DCT calculations can be simplified this way.
             */

            if (inptr[DCTSIZE*1] == 0 && inptr[DCTSIZE*2] == 0 &&
            inptr[DCTSIZE*3] == 0 && inptr[DCTSIZE*4] == 0 &&
            inptr[DCTSIZE*5] == 0 && inptr[DCTSIZE*6] == 0 &&
            inptr[DCTSIZE*7] == 0) {
              /* AC terms all zero */
              int dcval = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]) << PASS1_BITS;

              wsptr[DCTSIZE*0] = dcval;
              wsptr[DCTSIZE*1] = dcval;
              wsptr[DCTSIZE*2] = dcval;
              wsptr[DCTSIZE*3] = dcval;
              wsptr[DCTSIZE*4] = dcval;
              wsptr[DCTSIZE*5] = dcval;
              wsptr[DCTSIZE*6] = dcval;
              wsptr[DCTSIZE*7] = dcval;

              inptr++;          /* advance pointers to next column */
              quantptr++;
              wsptr++;
              continue;
            }

            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */

            z2 = DEQUANTIZE(inptr[DCTSIZE*2], quantptr[DCTSIZE*2]);
            z3 = DEQUANTIZE(inptr[DCTSIZE*6], quantptr[DCTSIZE*6]);

            z1 = MULTIPLY(z2 + z3, FIX_0_541196100);
            tmp2 = z1 + MULTIPLY(z3, - FIX_1_847759065);
            tmp3 = z1 + MULTIPLY(z2, FIX_0_765366865);

            z2 = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]);
            z3 = DEQUANTIZE(inptr[DCTSIZE*4], quantptr[DCTSIZE*4]);

            tmp0 = (z2 + z3) << CONST_BITS;
            tmp1 = (z2 - z3) << CONST_BITS;

            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;

            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */

            tmp0 = DEQUANTIZE(inptr[DCTSIZE*7], quantptr[DCTSIZE*7]);
            tmp1 = DEQUANTIZE(inptr[DCTSIZE*5], quantptr[DCTSIZE*5]);
            tmp2 = DEQUANTIZE(inptr[DCTSIZE*3], quantptr[DCTSIZE*3]);
            tmp3 = DEQUANTIZE(inptr[DCTSIZE*1], quantptr[DCTSIZE*1]);

            z1 = tmp0 + tmp3;
            z2 = tmp1 + tmp2;
            z3 = tmp0 + tmp2;
            z4 = tmp1 + tmp3;
            z5 = MULTIPLY(z3 + z4, FIX_1_175875602); /* sqrt(2) * c3 */

            tmp0 = MULTIPLY(tmp0, FIX_0_298631336); /* sqrt(2) * (-c1+c3+c5-c7) */
            tmp1 = MULTIPLY(tmp1, FIX_2_053119869); /* sqrt(2) * ( c1+c3-c5+c7) */
            tmp2 = MULTIPLY(tmp2, FIX_3_072711026); /* sqrt(2) * ( c1+c3+c5-c7) */
            tmp3 = MULTIPLY(tmp3, FIX_1_501321110); /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = MULTIPLY(z1, - FIX_0_899976223); /* sqrt(2) * (c7-c3) */
            z2 = MULTIPLY(z2, - FIX_2_562915447); /* sqrt(2) * (-c1-c3) */
            z3 = MULTIPLY(z3, - FIX_1_961570560); /* sqrt(2) * (-c3-c5) */
            z4 = MULTIPLY(z4, - FIX_0_390180644); /* sqrt(2) * (c5-c3) */

            z3 += z5;
            z4 += z5;

            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;

            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */

            wsptr[DCTSIZE*0] = (int) DESCALE(tmp10 + tmp3, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*7] = (int) DESCALE(tmp10 - tmp3, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*1] = (int) DESCALE(tmp11 + tmp2, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*6] = (int) DESCALE(tmp11 - tmp2, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*2] = (int) DESCALE(tmp12 + tmp1, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*5] = (int) DESCALE(tmp12 - tmp1, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*3] = (int) DESCALE(tmp13 + tmp0, CONST_BITS-PASS1_BITS);
            wsptr[DCTSIZE*4] = (int) DESCALE(tmp13 - tmp0, CONST_BITS-PASS1_BITS);

            inptr++;            /* advance pointers to next column */
            quantptr++;
            wsptr++;
          }

          /* Pass 2: process rows from work array, store into output array. */
          /* Note that we must descale the results by a factor of 8 == 2**3, */
          /* and also undo the PASS1_BITS scaling. */

          wsptr = workspace;
          for (ctr = 0; ctr < DCTSIZE; ctr++) {
            outptr = output_buf[ctr] + output_col;
            /* Rows of zeroes can be exploited in the same way as we did with columns.
             * However, the column calculation has created many nonzero AC terms, so
             * the simplification applies less often (typically 5% to 10% of the time).
             * On machines with very fast multiplication, it's possible that the
             * test takes more time than it's worth.  In that case this section
             * may be commented out.
             */

        #ifndef NO_ZERO_ROW_TEST
            if (wsptr[1] == 0 && wsptr[2] == 0 && wsptr[3] == 0 && wsptr[4] == 0 &&
            wsptr[5] == 0 && wsptr[6] == 0 && wsptr[7] == 0) {
              /* AC terms all zero */
              JSAMPLE dcval = range_limit[(int) DESCALE((i32) wsptr[0], PASS1_BITS+3)
                          & RANGE_MASK];

              outptr[0] = dcval;
              outptr[1] = dcval;
              outptr[2] = dcval;
              outptr[3] = dcval;
              outptr[4] = dcval;
              outptr[5] = dcval;
              outptr[6] = dcval;
              outptr[7] = dcval;

              wsptr += DCTSIZE;     /* advance pointer to next row */
              continue;
            }
        #endif

            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */

            z2 = (i32) wsptr[2];
            z3 = (i32) wsptr[6];

            z1 = MULTIPLY(z2 + z3, FIX_0_541196100);
            tmp2 = z1 + MULTIPLY(z3, - FIX_1_847759065);
            tmp3 = z1 + MULTIPLY(z2, FIX_0_765366865);

            tmp0 = ((i32) wsptr[0] + (i32) wsptr[4]) << CONST_BITS;
            tmp1 = ((i32) wsptr[0] - (i32) wsptr[4]) << CONST_BITS;

            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;

            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */

            tmp0 = (i32) wsptr[7];
            tmp1 = (i32) wsptr[5];
            tmp2 = (i32) wsptr[3];
            tmp3 = (i32) wsptr[1];

            z1 = tmp0 + tmp3;
            z2 = tmp1 + tmp2;
            z3 = tmp0 + tmp2;
            z4 = tmp1 + tmp3;
            z5 = MULTIPLY(z3 + z4, FIX_1_175875602); /* sqrt(2) * c3 */

            tmp0 = MULTIPLY(tmp0, FIX_0_298631336); /* sqrt(2) * (-c1+c3+c5-c7) */
            tmp1 = MULTIPLY(tmp1, FIX_2_053119869); /* sqrt(2) * ( c1+c3-c5+c7) */
            tmp2 = MULTIPLY(tmp2, FIX_3_072711026); /* sqrt(2) * ( c1+c3+c5-c7) */
            tmp3 = MULTIPLY(tmp3, FIX_1_501321110); /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = MULTIPLY(z1, - FIX_0_899976223); /* sqrt(2) * (c7-c3) */
            z2 = MULTIPLY(z2, - FIX_2_562915447); /* sqrt(2) * (-c1-c3) */
            z3 = MULTIPLY(z3, - FIX_1_961570560); /* sqrt(2) * (-c3-c5) */
            z4 = MULTIPLY(z4, - FIX_0_390180644); /* sqrt(2) * (c5-c3) */

            z3 += z5;
            z4 += z5;

            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;

            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */

            outptr[0] = range_limit[(int) DESCALE(tmp10 + tmp3,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[7] = range_limit[(int) DESCALE(tmp10 - tmp3,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[1] = range_limit[(int) DESCALE(tmp11 + tmp2,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[6] = range_limit[(int) DESCALE(tmp11 - tmp2,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[2] = range_limit[(int) DESCALE(tmp12 + tmp1,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[5] = range_limit[(int) DESCALE(tmp12 - tmp1,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[3] = range_limit[(int) DESCALE(tmp13 + tmp0,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];
            outptr[4] = range_limit[(int) DESCALE(tmp13 - tmp0,
                              CONST_BITS+PASS1_BITS+3)
                        & RANGE_MASK];

            wsptr += DCTSIZE;       /* advance pointer to next row */
          }
            */
    }
}
