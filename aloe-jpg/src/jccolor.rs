/**
  | This file contains input colorspace
  | conversion routines.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jccolor.c]

pub struct MyColorConverter {

    /**
      | public fields
      |
      */
    pub_: JpegColorConverter,


    /**
      | Private state for RGB->YCC conversion
      | 
      | => table for RGB to YCbCr conversion
      |
      */
    rgb_ycc_tab: *mut i32,
}

pub type MyCConvertPtr = *mut MyColorConverter;

/**************** RGB -> YCbCr conversion: most common case **************/

/**
 | YCbCr is defined per CCIR 601-1, except that Cb
 | and Cr are normalized to the range
 | 0..MAXJSAMPLE rather than -0.5 .. 0.5.  The
 | conversion equations to be implemented are
 | therefore
 |
 |  Y  =  
 |  0.29900 * R 
 |  + 0.58700 * G 
 |  + 0.11400 * B
 |
 |  Cb = 
 |  -0.16874 * R 
 |  - 0.33126 * G 
 |  + 0.50000 * B  
 |  + CENTERJSAMPLE
 |
 |  Cr =  
 |  0.50000 * R 
 |  - 0.41869 * G 
 |  - 0.08131 * B  
 |  + CENTERJSAMPLE
 |
 | (These numbers are derived from TIFF 6.0
 | section 21, dated 3-June-92.)
 |
 | Note: older versions of the IJG code used
 | a zero offset of MAXJSAMPLE/2, rather than
 | CENTERJSAMPLE, for Cb and Cr.  This gave equal
 | positive and negative swings for Cb/Cr, but
 | meant that grayscale values (Cb=Cr=0) were not
 | represented exactly.  Now we sacrifice exact
 | representation of maximum red and maximum blue
 | in order to get exact grayscales.
 |
 | To avoid floating-point arithmetic, we
 | represent the fractional constants as integers
 | scaled up by 2^16 (about 4 digits precision);
 | we have to divide the products by 2^16, with
 | appropriate rounding, to get the correct
 | answer.
 |
 | For even more speed, we avoid doing any
 | multiplications in the inner loop by
 | precalculating the constants times R,G,B for
 | all possible values.  For 8-bit JSAMPLEs this
 | is very reasonable (only 256 entries per
 | table); for 12-bit samples it is still
 | acceptable.  It's not very reasonable for
 | 16-bit samples, but if you want lossless
 | storage you shouldn't be changing colorspace
 | anyway.
 |
 | The CENTERJSAMPLE offsets and the rounding
 | fudge-factor of 0.5 are included in the tables
 | to save adding them separately in the inner
 | loop.
 */

macro_rules! scalebits {
    () => {
        /*
                16  /* speediest right-shift on some machines */
        */
    }
}

macro_rules! cbcr_offset {
    () => {
        /*
                ((i32) CENTERJSAMPLE << SCALEBITS)
        */
    }
}

macro_rules! one_half {
    () => {
        /*
                ((i32) 1 << (SCALEBITS-1))
        */
    }
}

macro_rules! fix {
    ($x:ident) => {
        /*
                ((i32) ((x) * (1L<<SCALEBITS) + 0.5))
        */
    }
}

/**
  | We allocate one big table and divide
  | it up into eight parts, instead of doing
  | eight alloc_small requests. This lets
  | us use a single table base address, which
  | can be held in a register in the inner
  | loops on many machines (more than can
  | hold all eight addresses, anyway).
  |
  */

macro_rules! r_y_off {
    () => {
        /*
                0           /* offset to R => Y section */
        */
    }
}

macro_rules! g_y_off {
    () => {
        /*
                (1*(MAXJSAMPLE+1))  /* offset to G => Y section */
        */
    }
}

macro_rules! y_off {
    () => {
        /*
                (2*(MAXJSAMPLE+1))  /* etc. */
        */
    }
}

macro_rules! r_cb_off {
    () => {
        /*
                (3*(MAXJSAMPLE+1))
        */
    }
}

macro_rules! g_cb_off {
    () => {
        /*
                (4*(MAXJSAMPLE+1))
        */
    }
}

macro_rules! cb_off {
    () => {
        /*
                (5*(MAXJSAMPLE+1))
        */
    }
}

macro_rules! r_cr_off {
    () => {
        /*
                B_CB_OFF        /* B=>Cb, R=>Cr are the same */
        */
    }
}

macro_rules! g_cr_off {
    () => {
        /*
                (6*(MAXJSAMPLE+1))
        */
    }
}

macro_rules! cr_off {
    () => {
        /*
                (7*(MAXJSAMPLE+1))
        */
    }
}

macro_rules! table_size {
    () => {
        /*
                (8*(MAXJSAMPLE+1))
        */
    }
}

/**
  | Initialize for RGB->YCC colorspace
  | conversion.
  |
  */
pub fn rgb_ycc_start(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_cconvert_ptr cconvert = (my_cconvert_ptr) cinfo->cconvert;
      i32 * rgb_ycc_tab;
      i32 i;

      /* Allocate and fill in the conversion tables. */
      cconvert->rgb_ycc_tab = rgb_ycc_tab = (i32 *)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    (TABLE_SIZE * SIZEOF(i32)));

      for (i = 0; i <= MAXJSAMPLE; i++) {
        rgb_ycc_tab[i+R_Y_OFF] = FIX(0.29900) * i;
        rgb_ycc_tab[i+G_Y_OFF] = FIX(0.58700) * i;
        rgb_ycc_tab[i+B_Y_OFF] = FIX(0.11400) * i     + ONE_HALF;
        rgb_ycc_tab[i+R_CB_OFF] = (-FIX(0.16874)) * i;
        rgb_ycc_tab[i+G_CB_OFF] = (-FIX(0.33126)) * i;
        /* We use a rounding fudge-factor of 0.5-epsilon for Cb and Cr.
         * This ensures that the maximum output will round to MAXJSAMPLE
         * not MAXJSAMPLE+1, and thus that we don't have to range-limit.
         */
        rgb_ycc_tab[i+B_CB_OFF] = FIX(0.50000) * i    + CBCR_OFFSET + ONE_HALF-1;
    /*  B=>Cb and R=>Cr tables are the same
        rgb_ycc_tab[i+R_CR_OFF] = FIX(0.50000) * i    + CBCR_OFFSET + ONE_HALF-1;
    */
        rgb_ycc_tab[i+G_CR_OFF] = (-FIX(0.41869)) * i;
        rgb_ycc_tab[i+B_CR_OFF] = (-FIX(0.08131)) * i;
      }
        */
}

/**
  | Convert some rows of samples to the JPEG
  | colorspace.
  | 
  | -----------
  | @note
  | 
  | we change from the application's interleaved-pixel
  | format to our internal noninterleaved,
  | one-plane-per-component format.
  | 
  | The input buffer is therefore three
  | times as wide as the output buffer.
  | 
  | A starting row offset is provided only
  | for the output buffer. The caller can
  | easily adjust the passed input_buf
  | value to accommodate any row offset
  | required on that side.
  |
  */
pub fn rgb_ycc_convert(
        cinfo:      JCompressPtr,
        input_buf:  JSampArray,
        output_buf: JSampImage,
        output_row: JDimension,
        num_rows:   i32)  {
    
    todo!();
        /*
            my_cconvert_ptr cconvert = (my_cconvert_ptr) cinfo->cconvert;
      int r, g, b;
      i32 * ctab = cconvert->rgb_ycc_tab;
      JSAMPROW inptr;
      JSAMPROW outptr0, outptr1, outptr2;
      JDimension col;
      JDimension num_cols = cinfo->image_width;

      while (--num_rows >= 0) {
        inptr = *input_buf++;
        outptr0 = output_buf[0][output_row];
        outptr1 = output_buf[1][output_row];
        outptr2 = output_buf[2][output_row];
        output_row++;
        for (col = 0; col < num_cols; col++) {
          r = GETJSAMPLE(inptr[RGB_RED]);
          g = GETJSAMPLE(inptr[RGB_GREEN]);
          b = GETJSAMPLE(inptr[RGB_BLUE]);
          inptr += RGB_PIXELSIZE;
          /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
           * must be too; we do not need an explicit range-limiting operation.
           * Hence the value being shifted is never negative, and we don't
           * need the general RIGHT_SHIFT macro.
           */
          /* Y */
          outptr0[col] = (JSAMPLE)
            ((ctab[r+R_Y_OFF] + ctab[g+G_Y_OFF] + ctab[b+B_Y_OFF])
             >> SCALEBITS);
          /* Cb */
          outptr1[col] = (JSAMPLE)
            ((ctab[r+R_CB_OFF] + ctab[g+G_CB_OFF] + ctab[b+B_CB_OFF])
             >> SCALEBITS);
          /* Cr */
          outptr2[col] = (JSAMPLE)
            ((ctab[r+R_CR_OFF] + ctab[g+G_CR_OFF] + ctab[b+B_CR_OFF])
             >> SCALEBITS);
        }
      }
        */
}

/**************** Cases other than RGB -> YCbCr **************/

/**
  | Convert some rows of samples to the JPEG
  | colorspace.
  | 
  | This version handles RGB->grayscale
  | conversion, which is the same as the
  | RGB->Y portion of RGB->YCbCr.
  | 
  | We assume rgb_ycc_start has been called
  | (we only use the Y tables).
  |
  */
pub fn rgb_gray_convert(
        cinfo:      JCompressPtr,
        input_buf:  JSampArray,
        output_buf: JSampImage,
        output_row: JDimension,
        num_rows:   i32)  {
    
    todo!();
        /*
            my_cconvert_ptr cconvert = (my_cconvert_ptr) cinfo->cconvert;
      int r, g, b;
      i32 * ctab = cconvert->rgb_ycc_tab;
      JSAMPROW inptr;
      JSAMPROW outptr;
      JDimension col;
      JDimension num_cols = cinfo->image_width;

      while (--num_rows >= 0) {
        inptr = *input_buf++;
        outptr = output_buf[0][output_row];
        output_row++;
        for (col = 0; col < num_cols; col++) {
          r = GETJSAMPLE(inptr[RGB_RED]);
          g = GETJSAMPLE(inptr[RGB_GREEN]);
          b = GETJSAMPLE(inptr[RGB_BLUE]);
          inptr += RGB_PIXELSIZE;
          /* Y */
          outptr[col] = (JSAMPLE)
            ((ctab[r+R_Y_OFF] + ctab[g+G_Y_OFF] + ctab[b+B_Y_OFF])
             >> SCALEBITS);
        }
      }
        */
}

/**
  | Convert some rows of samples to the JPEG
  | colorspace.
  | 
  | This version handles Adobe-style CMYK->YCCK
  | conversion, where we convert R=1-C,
  | G=1-M, and B=1-Y to YCbCr using the same
  | conversion as above, while passing
  | K (black) unchanged.
  | 
  | We assume rgb_ycc_start has been called.
  |
  */
pub fn cmyk_ycck_convert(
        cinfo:      JCompressPtr,
        input_buf:  JSampArray,
        output_buf: JSampImage,
        output_row: JDimension,
        num_rows:   i32)  {
    
    todo!();
        /*
            my_cconvert_ptr cconvert = (my_cconvert_ptr) cinfo->cconvert;
      int r, g, b;
      i32 * ctab = cconvert->rgb_ycc_tab;
      JSAMPROW inptr;
      JSAMPROW outptr0, outptr1, outptr2, outptr3;
      JDimension col;
      JDimension num_cols = cinfo->image_width;

      while (--num_rows >= 0) {
        inptr = *input_buf++;
        outptr0 = output_buf[0][output_row];
        outptr1 = output_buf[1][output_row];
        outptr2 = output_buf[2][output_row];
        outptr3 = output_buf[3][output_row];
        output_row++;
        for (col = 0; col < num_cols; col++) {
          r = MAXJSAMPLE - GETJSAMPLE(inptr[0]);
          g = MAXJSAMPLE - GETJSAMPLE(inptr[1]);
          b = MAXJSAMPLE - GETJSAMPLE(inptr[2]);
          /* K passes through as-is */
          outptr3[col] = inptr[3];  /* don't need GETJSAMPLE here */
          inptr += 4;
          /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
           * must be too; we do not need an explicit range-limiting operation.
           * Hence the value being shifted is never negative, and we don't
           * need the general RIGHT_SHIFT macro.
           */
          /* Y */
          outptr0[col] = (JSAMPLE)
            ((ctab[r+R_Y_OFF] + ctab[g+G_Y_OFF] + ctab[b+B_Y_OFF])
             >> SCALEBITS);
          /* Cb */
          outptr1[col] = (JSAMPLE)
            ((ctab[r+R_CB_OFF] + ctab[g+G_CB_OFF] + ctab[b+B_CB_OFF])
             >> SCALEBITS);
          /* Cr */
          outptr2[col] = (JSAMPLE)
            ((ctab[r+R_CR_OFF] + ctab[g+G_CR_OFF] + ctab[b+B_CR_OFF])
             >> SCALEBITS);
        }
      }
        */
}


/**
  | Convert some rows of samples to the JPEG
  | colorspace.
  | 
  | This version handles grayscale output
  | with no conversion.
  | 
  | The source can be either plain grayscale
  | or YCbCr (since Y == gray).
  |
  */
pub fn grayscale_convert(
        cinfo:      JCompressPtr,
        input_buf:  JSampArray,
        output_buf: JSampImage,
        output_row: JDimension,
        num_rows:   i32)  {
    
    todo!();
        /*
            JSAMPROW inptr;
      JSAMPROW outptr;
      JDimension col;
      JDimension num_cols = cinfo->image_width;
      int instride = cinfo->input_components;

      while (--num_rows >= 0) {
        inptr = *input_buf++;
        outptr = output_buf[0][output_row];
        output_row++;
        for (col = 0; col < num_cols; col++) {
          outptr[col] = inptr[0];   /* don't need GETJSAMPLE() here */
          inptr += instride;
        }
      }
        */
}

/**
  | Convert some rows of samples to the JPEG
  | colorspace.
  | 
  | This version handles multi-component
  | colorspaces without conversion.
  | 
  | We assume input_components == num_components.
  |
  */
pub fn null_convert(
        cinfo:      JCompressPtr,
        input_buf:  JSampArray,
        output_buf: JSampImage,
        output_row: JDimension,
        num_rows:   i32)  {
    
    todo!();
        /*
            JSAMPROW inptr;
      JSAMPROW outptr;
      JDimension col;
      int ci;
      int nc = cinfo->num_components;
      JDimension num_cols = cinfo->image_width;

      while (--num_rows >= 0) {
        /* It seems fastest to make a separate pass for each component. */
        for (ci = 0; ci < nc; ci++) {
          inptr = *input_buf;
          outptr = output_buf[ci][output_row];
          for (col = 0; col < num_cols; col++) {
        outptr[col] = inptr[ci]; /* don't need GETJSAMPLE() here */
        inptr += nc;
          }
        }
        input_buf++;
        output_row++;
      }
        */
}

/**
  | Empty method for start_pass.
  |
  */
pub fn null_method(_0: JCompressPtr)  {
    
    todo!();
        /*
            /* no work needed */
        */
}

/**
  | Module initialization routine for
  | input colorspace conversion.
  |
  */
pub fn jinit_color_converter(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_cconvert_ptr cconvert;

      cconvert = (my_cconvert_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_color_converter));
      cinfo->cconvert = (struct jpeg_color_converter *) cconvert;
      /* set start_pass to null method until we find out differently */
      cconvert->pub.start_pass = null_method;

      /* Make sure input_components agrees with in_color_space */
      switch (cinfo->in_color_space) {
      case JCS_GRAYSCALE:
        if (cinfo->input_components != 1)
          ERREXIT(cinfo, JERR_BAD_IN_COLORSPACE);
        break;

      case JCS_RGB:
    #if RGB_PIXELSIZE != 3
        if (cinfo->input_components != RGB_PIXELSIZE)
          ERREXIT(cinfo, JERR_BAD_IN_COLORSPACE);
        break;
    #endif /* else share code with YCbCr */

      case JCS_YCbCr:
        if (cinfo->input_components != 3)
          ERREXIT(cinfo, JERR_BAD_IN_COLORSPACE);
        break;

      case JCS_CMYK:
      case JCS_YCCK:
        if (cinfo->input_components != 4)
          ERREXIT(cinfo, JERR_BAD_IN_COLORSPACE);
        break;

      default:          /* JCS_UNKNOWN can be anything */
        if (cinfo->input_components < 1)
          ERREXIT(cinfo, JERR_BAD_IN_COLORSPACE);
        break;
      }

      /* Check num_components, set conversion method based on requested space */
      switch (cinfo->jpeg_color_space) {
      case JCS_GRAYSCALE:
        if (cinfo->num_components != 1)
          ERREXIT(cinfo, JERR_BAD_J_COLORSPACE);
        if (cinfo->in_color_space == JCS_GRAYSCALE)
          cconvert->pub.color_convert = grayscale_convert;
        else if (cinfo->in_color_space == JCS_RGB) {
          cconvert->pub.start_pass = rgb_ycc_start;
          cconvert->pub.color_convert = rgb_gray_convert;
        } else if (cinfo->in_color_space == JCS_YCbCr)
          cconvert->pub.color_convert = grayscale_convert;
        else
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        break;

      case JCS_RGB:
        if (cinfo->num_components != 3)
          ERREXIT(cinfo, JERR_BAD_J_COLORSPACE);
        if (cinfo->in_color_space == JCS_RGB && RGB_PIXELSIZE == 3)
          cconvert->pub.color_convert = null_convert;
        else
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        break;

      case JCS_YCbCr:
        if (cinfo->num_components != 3)
          ERREXIT(cinfo, JERR_BAD_J_COLORSPACE);
        if (cinfo->in_color_space == JCS_RGB) {
          cconvert->pub.start_pass = rgb_ycc_start;
          cconvert->pub.color_convert = rgb_ycc_convert;
        } else if (cinfo->in_color_space == JCS_YCbCr)
          cconvert->pub.color_convert = null_convert;
        else
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        break;

      case JCS_CMYK:
        if (cinfo->num_components != 4)
          ERREXIT(cinfo, JERR_BAD_J_COLORSPACE);
        if (cinfo->in_color_space == JCS_CMYK)
          cconvert->pub.color_convert = null_convert;
        else
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        break;

      case JCS_YCCK:
        if (cinfo->num_components != 4)
          ERREXIT(cinfo, JERR_BAD_J_COLORSPACE);
        if (cinfo->in_color_space == JCS_CMYK) {
          cconvert->pub.start_pass = rgb_ycc_start;
          cconvert->pub.color_convert = cmyk_ycck_convert;
        } else if (cinfo->in_color_space == JCS_YCCK)
          cconvert->pub.color_convert = null_convert;
        else
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        break;

      default:          /* allow null conversion of JCS_UNKNOWN */
        if (cinfo->jpeg_color_space != cinfo->in_color_space ||
        cinfo->num_components != cinfo->input_components)
          ERREXIT(cinfo, JERR_CONVERSION_NOTIMPL);
        cconvert->pub.color_convert = null_convert;
        break;
      }
        */
}
