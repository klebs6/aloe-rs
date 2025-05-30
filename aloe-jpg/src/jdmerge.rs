/*!
  | jdmerge.c
  | 
  | This file contains code for merged upsampling/color
  | conversion.
  | 
  | This file combines functions from jdsample.c
  | and jdcolor.c; read those files first
  | to understand what's going on.
  | 
  | When the chroma components are to be
  | upsampled by simple replication (ie,
  | box filtering), we can save some work
  | in color conversion by calculating
  | all the output pixels corresponding
  | to a pair of chroma samples at one time.
  | In the conversion equations
  | 
  | R = Y + K1 * Cr
  | 
  | G = Y + K2 * Cb + K3 * Cr
  | 
  | B = Y + K4 * Cb 
  |
  | only the Y term varies among
  | the group of pixels corresponding to
  | a pair of chroma samples, so the rest
  | of the terms can be calculated just once.
  | 
  | At typical sampling ratios, this eliminates
  | half or three-quarters of the multiplications
  | needed for color conversion.
  | 
  | This file currently provides implementations
  | for the following cases:
  | 
  | - YCbCr => RGB color conversion only.
  | 
  | - Sampling ratios of 2h1v or 2h2v.
  | 
  | - No scaling needed at upsample time.
  | 
  | - Corner-aligned (non-CCIR601) sampling
  | alignment.
  | 
  | Other special cases could be added,
  | but in most applications these are the
  | only common cases. (For uncommon cases
  | we fall back on the more general code
  | in jdsample.c and jdcolor.c.)
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdmerge.c]

/**
  | Private subobject
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub struct MyUpsampler {

    /**
      | public fields
      |
      */
    pub_: JpegUpsampler,

    /**
      | Pointer to routine to do actual
      | upsampling/conversion of one row group
      |
      */
    upmethod: fn(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: JDimension,
        output_buf:       JSampArray
    ) -> c_void,

    /* ------ Private state for YCC->RGB conversion  ------ */

    /**
      | => table for Cr to R conversion
      |
      */
    cr_r_tab: *mut i32,

    /**
      | => table for Cb to B conversion
      |
      */
    cb_b_tab: *mut i32,

    /**
      | => table for Cr to G conversion
      |
      */
    cr_g_tab: *mut i32,

    /**
      | => table for Cb to G conversion
      |
      */
    cb_g_tab: *mut i32,

    /**
      | For 2:1 vertical sampling, we produce
      | two output rows at a time.
      | 
      | We need a "spare" row buffer to hold the
      | second output row if the application
      | provides just a one-row buffer; we also
      | use the spare to discard the dummy last
      | row if the image height is odd.
      |
      */
    spare_row:     JSampRow,

    /**
      | T if spare buffer is occupied
      |
      */
    spare_full:    bool,

    /**
      | samples per output row
      |
      */
    out_row_width: JDimension,

    /**
      | counts rows remaining in image
      |
      */
    rows_to_go:    JDimension,
}

#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub type MyUpsamplePtr = *mut MyUpsampler;

#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
macro_rules! scalebits {
    () => {
        /*
                16  /* speediest right-shift on some machines */
        */
    }
}

#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
macro_rules! one_half {
    () => {
        /*
                ((i32) 1 << (SCALEBITS-1))
        */
    }
}

#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
macro_rules! fix {
    ($x:ident) => {
        /*
                ((i32) ((x) * (1L<<SCALEBITS) + 0.5))
        */
    }
}

/**
  | Initialize tables for YCC->RGB colorspace
  | conversion.
  | 
  | This is taken directly from jdcolor.c;
  | see that file for more info.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn build_ycc_rgb_table2(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;
      int i;
      i32 x;
      SHIFT_TEMPS

      upsample->Cr_r_tab = (int *)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    (MAXJSAMPLE+1) * SIZEOF(int));
      upsample->Cb_b_tab = (int *)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    (MAXJSAMPLE+1) * SIZEOF(int));
      upsample->Cr_g_tab = (i32 *)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    (MAXJSAMPLE+1) * SIZEOF(i32));
      upsample->Cb_g_tab = (i32 *)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    (MAXJSAMPLE+1) * SIZEOF(i32));

      for (i = 0, x = -CENTERJSAMPLE; i <= MAXJSAMPLE; i++, x++) {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        upsample->Cr_r_tab[i] = (int)
                RIGHT_SHIFT(FIX(1.40200) * x + ONE_HALF, SCALEBITS);
        /* Cb=>B value is nearest int to 1.77200 * x */
        upsample->Cb_b_tab[i] = (int)
                RIGHT_SHIFT(FIX(1.77200) * x + ONE_HALF, SCALEBITS);
        /* Cr=>G value is scaled-up -0.71414 * x */
        upsample->Cr_g_tab[i] = (- FIX(0.71414)) * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        upsample->Cb_g_tab[i] = (- FIX(0.34414)) * x + ONE_HALF;
      }
        */
}

/**
  | Initialize for an upsampling pass.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn start_pass_merged_upsample(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;

      /* Mark the spare buffer empty */
      upsample->spare_full = FALSE;
      /* Initialize total-height counter for detecting bottom of image */
      upsample->rows_to_go = cinfo->output_height;
        */
}

/**
  | Control routine to do upsampling (and
  | color conversion).
  | 
  | The control routine just handles the
  | row buffering considerations.
  |
  | 2:1 vertical sampling case: may need
  | a spare row.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn merged_2v_upsample(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: *mut JDimension,
        _3:               JDimension,
        output_buf:       JSampArray,
        out_row_ctr:      *mut JDimension,
        out_rows_avail:   JDimension)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;
      JSAMPROW work_ptrs[2];
      JDimension num_rows;      /* number of rows returned to caller */

      if (upsample->spare_full) {
        /* If we have a spare row saved from a previous cycle, just return it. */
        jcopy_sample_rows(& upsample->spare_row, 0, output_buf + *out_row_ctr, 0,
                  1, upsample->out_row_width);
        num_rows = 1;
        upsample->spare_full = FALSE;
      } else {
        /* Figure number of rows to return to caller. */
        num_rows = 2;
        /* Not more than the distance to the end of the image. */
        if (num_rows > upsample->rows_to_go)
          num_rows = upsample->rows_to_go;
        /* And not more than what the client can accept: */
        out_rows_avail -= *out_row_ctr;
        if (num_rows > out_rows_avail)
          num_rows = out_rows_avail;
        /* Create output pointer array for upsampler. */
        work_ptrs[0] = output_buf[*out_row_ctr];
        if (num_rows > 1) {
          work_ptrs[1] = output_buf[*out_row_ctr + 1];
        } else {
          work_ptrs[1] = upsample->spare_row;
          upsample->spare_full = TRUE;
        }
        /* Now do the upsampling. */
        (*upsample->upmethod) (cinfo, input_buf, *in_row_group_ctr, work_ptrs);
      }

      /* Adjust counts */
      *out_row_ctr += num_rows;
      upsample->rows_to_go -= num_rows;
      /* When the buffer is emptied, declare this input row group consumed */
      if (! upsample->spare_full)
        (*in_row_group_ctr)++;
        */
}

/**
  | 1:1 vertical sampling case: much easier,
  | never need a spare row.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn merged_1v_upsample(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: *mut JDimension,
        _3:               JDimension,
        output_buf:       JSampArray,
        out_row_ctr:      *mut JDimension,
        _6:               JDimension)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;

      /* Just do the upsampling. */
      (*upsample->upmethod) (cinfo, input_buf, *in_row_group_ctr,
                 output_buf + *out_row_ctr);
      /* Adjust counts */
      (*out_row_ctr)++;
      (*in_row_group_ctr)++;
        */
}

/*
  | These are the routines invoked by the
  | control routines to do the actual upsampling/conversion.
  | One row group is processed per call.
  | 
  | -----------
  | @note
  | 
  | since we may be writing directly into
  | application-supplied buffers, we
  | have to be honest about the output width;
  | we can't assume the buffer has been rounded
  | up to an even width.
  |
  */

/**
  | Upsample and color convert for the case
  | of 2:1 horizontal and 1:1 vertical.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn h2v1_merged_upsample(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: JDimension,
        output_buf:       JSampArray)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;
      int y, cred, cgreen, cblue;
      int cb, cr;
      JSAMPROW outptr;
      JSAMPROW inptr0, inptr1, inptr2;
      JDimension col;
      /* copy these pointers into registers if possible */
      JSAMPLE * range_limit = cinfo->sample_range_limit;
      int * Crrtab = upsample->Cr_r_tab;
      int * Cbbtab = upsample->Cb_b_tab;
      i32 * Crgtab = upsample->Cr_g_tab;
      i32 * Cbgtab = upsample->Cb_g_tab;
      SHIFT_TEMPS

      inptr0 = input_buf[0][in_row_group_ctr];
      inptr1 = input_buf[1][in_row_group_ctr];
      inptr2 = input_buf[2][in_row_group_ctr];
      outptr = output_buf[0];
      /* Loop for each pair of output pixels */
      for (col = cinfo->output_width >> 1; col > 0; col--) {
        /* Do the chroma part of the calculation */
        cb = GETJSAMPLE(*inptr1++);
        cr = GETJSAMPLE(*inptr2++);
        cred = Crrtab[cr];
        cgreen = (int) RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS);
        cblue = Cbbtab[cb];
        /* Fetch 2 Y values and emit 2 pixels */
        y  = GETJSAMPLE(*inptr0++);
        outptr[RGB_RED] =   range_limit[y + cred];
        outptr[RGB_GREEN] = range_limit[y + cgreen];
        outptr[RGB_BLUE] =  range_limit[y + cblue];
        outptr += RGB_PIXELSIZE;
        y  = GETJSAMPLE(*inptr0++);
        outptr[RGB_RED] =   range_limit[y + cred];
        outptr[RGB_GREEN] = range_limit[y + cgreen];
        outptr[RGB_BLUE] =  range_limit[y + cblue];
        outptr += RGB_PIXELSIZE;
      }
      /* If image width is odd, do the last output column separately */
      if (cinfo->output_width & 1) {
        cb = GETJSAMPLE(*inptr1);
        cr = GETJSAMPLE(*inptr2);
        cred = Crrtab[cr];
        cgreen = (int) RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS);
        cblue = Cbbtab[cb];
        y  = GETJSAMPLE(*inptr0);
        outptr[RGB_RED] =   range_limit[y + cred];
        outptr[RGB_GREEN] = range_limit[y + cgreen];
        outptr[RGB_BLUE] =  range_limit[y + cblue];
      }
        */
}

/**
  | Upsample and color convert for the case
  | of 2:1 horizontal and 2:1 vertical.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn h2v2_merged_upsample(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: JDimension,
        output_buf:       JSampArray)  {
    
    todo!();
        /*
            my_upsample_ptr upsample = (my_upsample_ptr) cinfo->upsample;
      int y, cred, cgreen, cblue;
      int cb, cr;
      JSAMPROW outptr0, outptr1;
      JSAMPROW inptr00, inptr01, inptr1, inptr2;
      JDimension col;
      /* copy these pointers into registers if possible */
      JSAMPLE * range_limit = cinfo->sample_range_limit;
      int * Crrtab = upsample->Cr_r_tab;
      int * Cbbtab = upsample->Cb_b_tab;
      i32 * Crgtab = upsample->Cr_g_tab;
      i32 * Cbgtab = upsample->Cb_g_tab;
      SHIFT_TEMPS

      inptr00 = input_buf[0][in_row_group_ctr*2];
      inptr01 = input_buf[0][in_row_group_ctr*2 + 1];
      inptr1 = input_buf[1][in_row_group_ctr];
      inptr2 = input_buf[2][in_row_group_ctr];
      outptr0 = output_buf[0];
      outptr1 = output_buf[1];
      /* Loop for each group of output pixels */
      for (col = cinfo->output_width >> 1; col > 0; col--) {
        /* Do the chroma part of the calculation */
        cb = GETJSAMPLE(*inptr1++);
        cr = GETJSAMPLE(*inptr2++);
        cred = Crrtab[cr];
        cgreen = (int) RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS);
        cblue = Cbbtab[cb];
        /* Fetch 4 Y values and emit 4 pixels */
        y  = GETJSAMPLE(*inptr00++);
        outptr0[RGB_RED] =   range_limit[y + cred];
        outptr0[RGB_GREEN] = range_limit[y + cgreen];
        outptr0[RGB_BLUE] =  range_limit[y + cblue];
        outptr0 += RGB_PIXELSIZE;
        y  = GETJSAMPLE(*inptr00++);
        outptr0[RGB_RED] =   range_limit[y + cred];
        outptr0[RGB_GREEN] = range_limit[y + cgreen];
        outptr0[RGB_BLUE] =  range_limit[y + cblue];
        outptr0 += RGB_PIXELSIZE;
        y  = GETJSAMPLE(*inptr01++);
        outptr1[RGB_RED] =   range_limit[y + cred];
        outptr1[RGB_GREEN] = range_limit[y + cgreen];
        outptr1[RGB_BLUE] =  range_limit[y + cblue];
        outptr1 += RGB_PIXELSIZE;
        y  = GETJSAMPLE(*inptr01++);
        outptr1[RGB_RED] =   range_limit[y + cred];
        outptr1[RGB_GREEN] = range_limit[y + cgreen];
        outptr1[RGB_BLUE] =  range_limit[y + cblue];
        outptr1 += RGB_PIXELSIZE;
      }
      /* If image width is odd, do the last output column separately */
      if (cinfo->output_width & 1) {
        cb = GETJSAMPLE(*inptr1);
        cr = GETJSAMPLE(*inptr2);
        cred = Crrtab[cr];
        cgreen = (int) RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS);
        cblue = Cbbtab[cb];
        y  = GETJSAMPLE(*inptr00);
        outptr0[RGB_RED] =   range_limit[y + cred];
        outptr0[RGB_GREEN] = range_limit[y + cgreen];
        outptr0[RGB_BLUE] =  range_limit[y + cblue];
        y  = GETJSAMPLE(*inptr01);
        outptr1[RGB_RED] =   range_limit[y + cred];
        outptr1[RGB_GREEN] = range_limit[y + cgreen];
        outptr1[RGB_BLUE] =  range_limit[y + cblue];
      }
        */
}

/**
  | Module initialization routine for
  | merged upsampling/color conversion.
  | 
  | NB: this is called under the conditions
  | determined by use_merged_upsample()
  | in jdmaster.c. That routine MUST correspond
  | to the actual capabilities of this module;
  | no safety checks are made here.
  |
  */
#[cfg(UPSAMPLE_MERGING_SUPPORTED)]
pub fn jinit_merged_upsampler(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            my_upsample_ptr upsample;

      upsample = (my_upsample_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_upsampler));
      cinfo->upsample = (struct jpeg_upsampler *) upsample;
      upsample->pub.start_pass = start_pass_merged_upsample;
      upsample->pub.need_context_rows = FALSE;

      upsample->out_row_width = cinfo->output_width * cinfo->out_color_components;

      if (cinfo->max_v_samp_factor == 2) {
        upsample->pub.upsample = merged_2v_upsample;
        upsample->upmethod = h2v2_merged_upsample;
        /* Allocate a spare row buffer */
        upsample->spare_row = (JSAMPROW)
          (*cinfo->mem->alloc_large) ((JCommonPtr) cinfo, JPOOL_IMAGE,
            (size_t) (upsample->out_row_width * SIZEOF(JSAMPLE)));
      } else {
        upsample->pub.upsample = merged_1v_upsample;
        upsample->upmethod = h2v1_merged_upsample;
        /* No spare row needed */
        upsample->spare_row = NULL;
      }

      build_ycc_rgb_table2(cinfo);
        */
}
