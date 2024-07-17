/*!
  | jidctred.c
  | 
  | This file contains inverse-DCT routines
  | that produce reduced-size output:
  | either 4x4, 2x2, or 1x1 pixels from an
  | 8x8 DCT block.
  | 
  | The implementation is based on the Loeffler,
  | Ligtenberg and Moschytz (LL&M) algorithm
  | used in jidctint.c. We simply replace
  | each 8-to-8 1-D IDCT step with an 8-to-4
  | step that produces the four averages
  | of two adjacent outputs (or an 8-to-2
  | step producing two averages of four
  | outputs, for 2x2 output).
  | 
  | These steps were derived by computing
  | the corresponding values at the end
  | of the normal LL&M code, then simplifying
  | as much as possible. 1x1 is trivial:
  | just take the DC coefficient divided
  | by 8.
  | 
  | See jidctint.c for additional comments.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jidctred.c]

#[cfg(IDCT_SCALING_SUPPORTED)]
pub mod idct_scaling_supported {
    use super::*;

    /**
      | This module is specialized to the case
      | DCTSIZE = 8.
      |
      */
    static_assert!{ DCTSIZE == 8 }

    /**
      | Scaling is the same as in jidctint.c.
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
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_211164243: i32 = 1730;    /* FIX(0.211164243) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_509795579: i32 = 4176;    /* FIX(0.509795579) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_601344887: i32 = 4926;    /* FIX(0.601344887) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_720959822: i32 = 5906;    /* FIX(0.720959822) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_765366865: i32 = 6270;    /* FIX(0.765366865) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_850430095: i32 = 6967;    /* FIX(0.850430095) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_0_899976223: i32 = 7373;    /* FIX(0.899976223) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_061594337: i32 = 8697;    /* FIX(1.061594337) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_272758580: i32 = 10426;   /* FIX(1.272758580) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_451774981: i32 = 11893;   /* FIX(1.451774981) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_1_847759065: i32 = 15137;   /* FIX(1.847759065) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_2_172734803: i32 = 17799;   /* FIX(2.172734803) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_2_562915447: i32 = 20995;   /* FIX(2.562915447) */
    #[cfg(CONST_BITS_EQ_13)] pub const FIX_3_624509785: i32 = 29692;   /* FIX(3.624509785) */

    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_211164243: i32 = FIX(0.211164243);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_509795579: i32 = FIX(0.509795579);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_601344887: i32 = FIX(0.601344887);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_720959822: i32 = FIX(0.720959822);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_765366865: i32 = FIX(0.765366865);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_850430095: i32 = FIX(0.850430095);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_0_899976223: i32 = FIX(0.899976223);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_061594337: i32 = FIX(1.061594337);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_272758580: i32 = FIX(1.272758580);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_451774981: i32 = FIX(1.451774981);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_1_847759065: i32 = FIX(1.847759065);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_2_172734803: i32 = FIX(2.172734803);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_2_562915447: i32 = FIX(2.562915447);
    #[cfg(not(CONST_BITS_EQ_13))] pub const FIX_3_624509785: i32 = FIX(3.624509785);

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
                    (((ISlowMultType) (coef)) * (quantval))
            */
        }
    }

    /**
      | Perform dequantization and inverse
      | DCT on one block of coefficients, producing
      | a reduced-size 4x4 output block.
      |
      */
    pub fn jpeg_idct_4x4(
            cinfo:      JDecompressPtr,
            compptr:    *mut JpegComponentInfo,
            coef_block: JCoefPtr,
            output_buf: JSampArray,
            output_col: JDimension)  {
        
        todo!();
            /*
                i32 tmp0, tmp2, tmp10, tmp12;
          i32 z1, z2, z3, z4;
          JCoefPtr inptr;
          ISlowMultType * quantptr;
          int * wsptr;
          JSAMPROW outptr;
          JSAMPLE *range_limit = IDCT_range_limit(cinfo);
          int ctr;
          int workspace[DCTSIZE*4]; /* buffers data between passes */
          SHIFT_TEMPS

          /* Pass 1: process columns from input, store into work array. */

          inptr = coef_block;
          quantptr = (ISlowMultType *) compptr->dct_table;
          wsptr = workspace;
          for (ctr = DCTSIZE; ctr > 0; inptr++, quantptr++, wsptr++, ctr--) {
            /* Don't bother to process column 4, because second pass won't use it */
            if (ctr == DCTSIZE-4)
              continue;
            if (inptr[DCTSIZE*1] == 0 && inptr[DCTSIZE*2] == 0 &&
            inptr[DCTSIZE*3] == 0 && inptr[DCTSIZE*5] == 0 &&
            inptr[DCTSIZE*6] == 0 && inptr[DCTSIZE*7] == 0) {
              /* AC terms all zero; we need not examine term 4 for 4x4 output */
              int dcval = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]) << PASS1_BITS;

              wsptr[DCTSIZE*0] = dcval;
              wsptr[DCTSIZE*1] = dcval;
              wsptr[DCTSIZE*2] = dcval;
              wsptr[DCTSIZE*3] = dcval;

              continue;
            }

            /* Even part */

            tmp0 = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]);
            tmp0 <<= (CONST_BITS+1);

            z2 = DEQUANTIZE(inptr[DCTSIZE*2], quantptr[DCTSIZE*2]);
            z3 = DEQUANTIZE(inptr[DCTSIZE*6], quantptr[DCTSIZE*6]);

            tmp2 = MULTIPLY(z2, FIX_1_847759065) + MULTIPLY(z3, - FIX_0_765366865);

            tmp10 = tmp0 + tmp2;
            tmp12 = tmp0 - tmp2;

            /* Odd part */

            z1 = DEQUANTIZE(inptr[DCTSIZE*7], quantptr[DCTSIZE*7]);
            z2 = DEQUANTIZE(inptr[DCTSIZE*5], quantptr[DCTSIZE*5]);
            z3 = DEQUANTIZE(inptr[DCTSIZE*3], quantptr[DCTSIZE*3]);
            z4 = DEQUANTIZE(inptr[DCTSIZE*1], quantptr[DCTSIZE*1]);

            tmp0 = MULTIPLY(z1, - FIX_0_211164243) /* sqrt(2) * (c3-c1) */
             + MULTIPLY(z2, FIX_1_451774981) /* sqrt(2) * (c3+c7) */
             + MULTIPLY(z3, - FIX_2_172734803) /* sqrt(2) * (-c1-c5) */
             + MULTIPLY(z4, FIX_1_061594337); /* sqrt(2) * (c5+c7) */

            tmp2 = MULTIPLY(z1, - FIX_0_509795579) /* sqrt(2) * (c7-c5) */
             + MULTIPLY(z2, - FIX_0_601344887) /* sqrt(2) * (c5-c1) */
             + MULTIPLY(z3, FIX_0_899976223) /* sqrt(2) * (c3-c7) */
             + MULTIPLY(z4, FIX_2_562915447); /* sqrt(2) * (c1+c3) */

            /* Final output stage */

            wsptr[DCTSIZE*0] = (int) DESCALE(tmp10 + tmp2, CONST_BITS-PASS1_BITS+1);
            wsptr[DCTSIZE*3] = (int) DESCALE(tmp10 - tmp2, CONST_BITS-PASS1_BITS+1);
            wsptr[DCTSIZE*1] = (int) DESCALE(tmp12 + tmp0, CONST_BITS-PASS1_BITS+1);
            wsptr[DCTSIZE*2] = (int) DESCALE(tmp12 - tmp0, CONST_BITS-PASS1_BITS+1);
          }

          /* Pass 2: process 4 rows from work array, store into output array. */

          wsptr = workspace;
          for (ctr = 0; ctr < 4; ctr++) {
            outptr = output_buf[ctr] + output_col;
            /* It's not clear whether a zero row test is worthwhile here ... */

        #ifndef NO_ZERO_ROW_TEST
            if (wsptr[1] == 0 && wsptr[2] == 0 && wsptr[3] == 0 &&
            wsptr[5] == 0 && wsptr[6] == 0 && wsptr[7] == 0) {
              /* AC terms all zero */
              JSAMPLE dcval = range_limit[(int) DESCALE((i32) wsptr[0], PASS1_BITS+3)
                          & RANGE_MASK];

              outptr[0] = dcval;
              outptr[1] = dcval;
              outptr[2] = dcval;
              outptr[3] = dcval;

              wsptr += DCTSIZE;     /* advance pointer to next row */
              continue;
            }
        #endif

            /* Even part */

            tmp0 = ((i32) wsptr[0]) << (CONST_BITS+1);

            tmp2 = MULTIPLY((i32) wsptr[2], FIX_1_847759065)
             + MULTIPLY((i32) wsptr[6], - FIX_0_765366865);

            tmp10 = tmp0 + tmp2;
            tmp12 = tmp0 - tmp2;

            /* Odd part */

            z1 = (i32) wsptr[7];
            z2 = (i32) wsptr[5];
            z3 = (i32) wsptr[3];
            z4 = (i32) wsptr[1];

            tmp0 = MULTIPLY(z1, - FIX_0_211164243) /* sqrt(2) * (c3-c1) */
             + MULTIPLY(z2, FIX_1_451774981) /* sqrt(2) * (c3+c7) */
             + MULTIPLY(z3, - FIX_2_172734803) /* sqrt(2) * (-c1-c5) */
             + MULTIPLY(z4, FIX_1_061594337); /* sqrt(2) * (c5+c7) */

            tmp2 = MULTIPLY(z1, - FIX_0_509795579) /* sqrt(2) * (c7-c5) */
             + MULTIPLY(z2, - FIX_0_601344887) /* sqrt(2) * (c5-c1) */
             + MULTIPLY(z3, FIX_0_899976223) /* sqrt(2) * (c3-c7) */
             + MULTIPLY(z4, FIX_2_562915447); /* sqrt(2) * (c1+c3) */

            /* Final output stage */

            outptr[0] = range_limit[(int) DESCALE(tmp10 + tmp2,
                              CONST_BITS+PASS1_BITS+3+1)
                        & RANGE_MASK];
            outptr[3] = range_limit[(int) DESCALE(tmp10 - tmp2,
                              CONST_BITS+PASS1_BITS+3+1)
                        & RANGE_MASK];
            outptr[1] = range_limit[(int) DESCALE(tmp12 + tmp0,
                              CONST_BITS+PASS1_BITS+3+1)
                        & RANGE_MASK];
            outptr[2] = range_limit[(int) DESCALE(tmp12 - tmp0,
                              CONST_BITS+PASS1_BITS+3+1)
                        & RANGE_MASK];

            wsptr += DCTSIZE;       /* advance pointer to next row */
          }
            */
    }

    /**
      | Perform dequantization and inverse
      | DCT on one block of coefficients, producing
      | a reduced-size 2x2 output block.
      |
      */
    pub fn jpeg_idct_2x2(
            cinfo:      JDecompressPtr,
            compptr:    *mut JpegComponentInfo,
            coef_block: JCoefPtr,
            output_buf: JSampArray,
            output_col: JDimension)  {
        
        todo!();
            /*
                i32 tmp0, tmp10, z1;
          JCoefPtr inptr;
          ISlowMultType * quantptr;
          int * wsptr;
          JSAMPROW outptr;
          JSAMPLE *range_limit = IDCT_range_limit(cinfo);
          int ctr;
          int workspace[DCTSIZE*2]; /* buffers data between passes */
          SHIFT_TEMPS

          /* Pass 1: process columns from input, store into work array. */

          inptr = coef_block;
          quantptr = (ISlowMultType *) compptr->dct_table;
          wsptr = workspace;
          for (ctr = DCTSIZE; ctr > 0; inptr++, quantptr++, wsptr++, ctr--) {
            /* Don't bother to process columns 2,4,6 */
            if (ctr == DCTSIZE-2 || ctr == DCTSIZE-4 || ctr == DCTSIZE-6)
              continue;
            if (inptr[DCTSIZE*1] == 0 && inptr[DCTSIZE*3] == 0 &&
            inptr[DCTSIZE*5] == 0 && inptr[DCTSIZE*7] == 0) {
              /* AC terms all zero; we need not examine terms 2,4,6 for 2x2 output */
              int dcval = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]) << PASS1_BITS;

              wsptr[DCTSIZE*0] = dcval;
              wsptr[DCTSIZE*1] = dcval;

              continue;
            }

            /* Even part */

            z1 = DEQUANTIZE(inptr[DCTSIZE*0], quantptr[DCTSIZE*0]);
            tmp10 = z1 << (CONST_BITS+2);

            /* Odd part */

            z1 = DEQUANTIZE(inptr[DCTSIZE*7], quantptr[DCTSIZE*7]);
            tmp0 = MULTIPLY(z1, - FIX_0_720959822); /* sqrt(2) * (c7-c5+c3-c1) */
            z1 = DEQUANTIZE(inptr[DCTSIZE*5], quantptr[DCTSIZE*5]);
            tmp0 += MULTIPLY(z1, FIX_0_850430095); /* sqrt(2) * (-c1+c3+c5+c7) */
            z1 = DEQUANTIZE(inptr[DCTSIZE*3], quantptr[DCTSIZE*3]);
            tmp0 += MULTIPLY(z1, - FIX_1_272758580); /* sqrt(2) * (-c1+c3-c5-c7) */
            z1 = DEQUANTIZE(inptr[DCTSIZE*1], quantptr[DCTSIZE*1]);
            tmp0 += MULTIPLY(z1, FIX_3_624509785); /* sqrt(2) * (c1+c3+c5+c7) */

            /* Final output stage */

            wsptr[DCTSIZE*0] = (int) DESCALE(tmp10 + tmp0, CONST_BITS-PASS1_BITS+2);
            wsptr[DCTSIZE*1] = (int) DESCALE(tmp10 - tmp0, CONST_BITS-PASS1_BITS+2);
          }

          /* Pass 2: process 2 rows from work array, store into output array. */

          wsptr = workspace;
          for (ctr = 0; ctr < 2; ctr++) {
            outptr = output_buf[ctr] + output_col;
            /* It's not clear whether a zero row test is worthwhile here ... */

        #ifndef NO_ZERO_ROW_TEST
            if (wsptr[1] == 0 && wsptr[3] == 0 && wsptr[5] == 0 && wsptr[7] == 0) {
              /* AC terms all zero */
              JSAMPLE dcval = range_limit[(int) DESCALE((i32) wsptr[0], PASS1_BITS+3)
                          & RANGE_MASK];

              outptr[0] = dcval;
              outptr[1] = dcval;

              wsptr += DCTSIZE;     /* advance pointer to next row */
              continue;
            }
        #endif

            /* Even part */

            tmp10 = ((i32) wsptr[0]) << (CONST_BITS+2);

            /* Odd part */

            tmp0 = MULTIPLY((i32) wsptr[7], - FIX_0_720959822) /* sqrt(2) * (c7-c5+c3-c1) */
             + MULTIPLY((i32) wsptr[5], FIX_0_850430095) /* sqrt(2) * (-c1+c3+c5+c7) */
             + MULTIPLY((i32) wsptr[3], - FIX_1_272758580) /* sqrt(2) * (-c1+c3-c5-c7) */
             + MULTIPLY((i32) wsptr[1], FIX_3_624509785); /* sqrt(2) * (c1+c3+c5+c7) */

            /* Final output stage */

            outptr[0] = range_limit[(int) DESCALE(tmp10 + tmp0,
                              CONST_BITS+PASS1_BITS+3+2)
                        & RANGE_MASK];
            outptr[1] = range_limit[(int) DESCALE(tmp10 - tmp0,
                              CONST_BITS+PASS1_BITS+3+2)
                        & RANGE_MASK];

            wsptr += DCTSIZE;       /* advance pointer to next row */
          }
            */
    }

    /**
      | Perform dequantization and inverse
      | DCT on one block of coefficients, producing
      | a reduced-size 1x1 output block.
      |
      */
    pub fn jpeg_idct_1x1(
            cinfo:      JDecompressPtr,
            compptr:    *mut JpegComponentInfo,
            coef_block: JCoefPtr,
            output_buf: JSampArray,
            output_col: JDimension)  {
        
        todo!();
            /*
                int dcval;
          ISlowMultType * quantptr;
          JSAMPLE *range_limit = IDCT_range_limit(cinfo);
          SHIFT_TEMPS

          /* We hardly need an inverse DCT routine for this: just take the
           * average pixel value, which is one-eighth of the DC coefficient.
           */
          quantptr = (ISlowMultType *) compptr->dct_table;
          dcval = DEQUANTIZE(coef_block[0], quantptr[0]);
          dcval = (int) DESCALE((i32) dcval, 3);

          output_buf[0][output_col] = range_limit[dcval & RANGE_MASK];
            */
    }
}
