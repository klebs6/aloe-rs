/*!
  | jcsample.c
  | 
  | This file contains downsampling routines.
  | 
  | Downsampling input data is counted
  | in "row groups". A row group is defined
  | to be max_v_samp_factor pixel rows
  | of each component, from which the downsampler
  | produces v_samp_factor sample rows.
  | 
  | A single row group is processed in each
  | call to the downsampler module.
  | 
  | The downsampler is responsible for
  | edge-expansion of its output data to
  | fill an integral number of DCT blocks
  | horizontally. The source buffer may
  | be modified if it is helpful for this
  | purpose (the source buffer is allocated
  | wide enough to correspond to the desired
  | output width).
  | 
  | The caller (the prep controller) is
  | responsible for vertical padding.
  | 
  | The downsampler may request "context
  | rows" by setting need_context_rows
  | during startup. In this case, the input
  | arrays will contain at least one row
  | group's worth of pixels above and below
  | the passed-in data; the caller will
  | create dummy rows at image top and bottom
  | by replicating the first or last real
  | pixel row.
  | 
  | An excellent reference for image resampling
  | is
  | 
  | Digital Image Warping, George Wolberg,
  | 1990.
  | 
  | Pub. by IEEE Computer Society Press,
  | Los Alamitos, CA. ISBN 0-8186-8944-7.
  | 
  | The downsampling algorithm used here
  | is a simple average of the source pixels
  | covered by the output pixel. The hi-falutin
  | sampling literature refers to this
  | as a "box filter". In general the characteristics
  | of a box filter are not very good, but
  | for the specific cases we normally use
  | (1:1 and 2:1 ratios) the box is equivalent
  | to a "triangle filter" which is not nearly
  | so bad. If you intend to use other sampling
  | ratios, you'd be well advised to improve
  | this code.
  | 
  | A simple input-smoothing capability
  | is provided. This is mainly intended
  | for cleaning up color-dithered GIF
  | input files (if you find it inadequate,
  | we suggest using an external filtering
  | program such as pnmconvol). When enabled,
  | each input pixel P is replaced by a weighted
  | sum of itself and its eight neighbors.
  | P's weight is 1-8*SF and each neighbor's
  | weight is SF, where SF = (smoothing_factor
  | / 1024).
  | 
  | Currently, smoothing is only supported
  | for 2h2v sampling factors.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jcsample.c]

/**
  | Pointer to routine to downsample a single
  | component
  |
  */
pub type Downsample1Ptr = fn(
    cinfo:       JCompressPtr,
    compptr:     *mut JpegComponentInfo,
    input_data:  JSampArray,
    output_data: JSampArray
);

pub struct MyDownsampler {

    /**
      | public fields
      |
      */
    pub_:    JpegDownsampler,


    /**
      | Downsampling method pointers, one
      | per component
      |
      */
    methods: [Downsample1Ptr; MAX_COMPONENTS],
}

pub type MyDownsamplePtr = *mut MyDownsampler;

/**
  | Initialize for a downsampling pass.
  |
  */
pub fn start_pass_downsample(_0: JCompressPtr)  {
    
    todo!();
        /*
            /* no work for now */
        */
}

/**
  | Expand a component horizontally from
  | width input_cols to width output_cols,
  | by duplicating the rightmost samples.
  |
  */
pub fn expand_right_edge(
        image_data:  JSampArray,
        num_rows:    i32,
        input_cols:  JDimension,
        output_cols: JDimension)  {
    
    todo!();
        /*
            JSAMPROW ptr;
      JSAMPLE pixval;
      int count;
      int row;
      int numcols = (int) (output_cols - input_cols);

      if (numcols > 0) {
        for (row = 0; row < num_rows; row++) {
          ptr = image_data[row] + input_cols;
          pixval = ptr[-1];     /* don't need GETJSAMPLE() here */
          for (count = numcols; count > 0; count--)
        *ptr++ = pixval;
        }
      }
        */
}

/**
  | Do downsampling for a whole row group
  | (all components).
  | 
  | In this version we simply downsample
  | each component independently.
  |
  */
pub fn sep_downsample(
        cinfo:               JCompressPtr,
        input_buf:           JSampImage,
        in_row_index:        JDimension,
        output_buf:          JSampImage,
        out_row_group_index: JDimension)  {
    
    todo!();
        /*
            my_downsample_ptr downsample = (my_downsample_ptr) cinfo->downsample;
      int ci;
      jpeg_component_info * compptr;
      JSAMPARRAY in_ptr, out_ptr;

      for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
           ci++, compptr++) {
        in_ptr = input_buf[ci] + in_row_index;
        out_ptr = output_buf[ci] + (out_row_group_index * compptr->v_samp_factor);
        (*downsample->methods[ci]) (cinfo, compptr, in_ptr, out_ptr);
      }
        */
}

/**
  | Downsample pixel values of a single
  | component.
  | 
  | One row group is processed per call.
  | 
  | This version handles arbitrary integral
  | sampling ratios, without smoothing.
  | 
  | -----------
  | @note
  | 
  | this version is not actually used for
  | customary sampling ratios.
  |
  */
pub fn int_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            int inrow, outrow, h_expand, v_expand, numpix, numpix2, h, v;
      JDimension outcol, outcol_h;  /* outcol_h == outcol*h_expand */
      JDimension output_cols = compptr->width_in_blocks * DCTSIZE;
      JSAMPROW inptr, outptr;
      i32 outvalue;

      h_expand = cinfo->max_h_samp_factor / compptr->h_samp_factor;
      v_expand = cinfo->max_v_samp_factor / compptr->v_samp_factor;
      numpix = h_expand * v_expand;
      numpix2 = numpix/2;

      /* Expand input data enough to let all the output samples be generated
       * by the standard loop.  Special-casing padded output would be more
       * efficient.
       */
      expand_right_edge(input_data, cinfo->max_v_samp_factor,
                cinfo->image_width, output_cols * h_expand);

      inrow = 0;
      for (outrow = 0; outrow < compptr->v_samp_factor; outrow++) {
        outptr = output_data[outrow];
        for (outcol = 0, outcol_h = 0; outcol < output_cols;
         outcol++, outcol_h += h_expand) {
          outvalue = 0;
          for (v = 0; v < v_expand; v++) {
        inptr = input_data[inrow+v] + outcol_h;
        for (h = 0; h < h_expand; h++) {
          outvalue += (i32) GETJSAMPLE(*inptr++);
        }
          }
          *outptr++ = (JSAMPLE) ((outvalue + numpix2) / numpix);
        }
        inrow += v_expand;
      }
        */
}

/**
  | Downsample pixel values of a single
  | component.
  | 
  | This version handles the special case
  | of a full-size component, without smoothing.
  |
  */
pub fn fullsize_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            /* Copy the data */
      jcopy_sample_rows(input_data, 0, output_data, 0,
                cinfo->max_v_samp_factor, cinfo->image_width);
      /* Edge-expand */
      expand_right_edge(output_data, cinfo->max_v_samp_factor,
                cinfo->image_width, compptr->width_in_blocks * DCTSIZE);
        */
}

/**
  | Downsample pixel values of a single
  | component.
  | 
  | This version handles the common case
  | of 2:1 horizontal and 1:1 vertical,
  | without smoothing.
  | 
  | A note about the "bias" calculations:
  | when rounding fractional values to
  | integer, we do not want to always round
  | 0.5 up to the next integer.
  | 
  | If we did that, we'd introduce a noticeable
  | bias towards larger values.
  | 
  | Instead, this code is arranged so that
  | 0.5 will be rounded up or down at alternate
  | pixel locations (a simple ordered dither
  | pattern).
  |
  */
pub fn h2v1_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            int outrow;
      JDimension outcol;
      JDimension output_cols = compptr->width_in_blocks * DCTSIZE;
      JSAMPROW inptr, outptr;
      int bias;

      /* Expand input data enough to let all the output samples be generated
       * by the standard loop.  Special-casing padded output would be more
       * efficient.
       */
      expand_right_edge(input_data, cinfo->max_v_samp_factor,
                cinfo->image_width, output_cols * 2);

      for (outrow = 0; outrow < compptr->v_samp_factor; outrow++) {
        outptr = output_data[outrow];
        inptr = input_data[outrow];
        bias = 0;           /* bias = 0,1,0,1,... for successive samples */
        for (outcol = 0; outcol < output_cols; outcol++) {
          *outptr++ = (JSAMPLE) ((GETJSAMPLE(*inptr) + GETJSAMPLE(inptr[1])
                      + bias) >> 1);
          bias ^= 1;        /* 0=>1, 1=>0 */
          inptr += 2;
        }
      }
        */
}

/**
  | Downsample pixel values of a single
  | component.
  | 
  | This version handles the standard case
  | of 2:1 horizontal and 2:1 vertical,
  | without smoothing.
  |
  */
pub fn h2v2_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            int inrow, outrow;
      JDimension outcol;
      JDimension output_cols = compptr->width_in_blocks * DCTSIZE;
      JSAMPROW inptr0, inptr1, outptr;
      int bias;

      /* Expand input data enough to let all the output samples be generated
       * by the standard loop.  Special-casing padded output would be more
       * efficient.
       */
      expand_right_edge(input_data, cinfo->max_v_samp_factor,
                cinfo->image_width, output_cols * 2);

      inrow = 0;
      for (outrow = 0; outrow < compptr->v_samp_factor; outrow++) {
        outptr = output_data[outrow];
        inptr0 = input_data[inrow];
        inptr1 = input_data[inrow+1];
        bias = 1;           /* bias = 1,2,1,2,... for successive samples */
        for (outcol = 0; outcol < output_cols; outcol++) {
          *outptr++ = (JSAMPLE) ((GETJSAMPLE(*inptr0) + GETJSAMPLE(inptr0[1]) +
                      GETJSAMPLE(*inptr1) + GETJSAMPLE(inptr1[1])
                      + bias) >> 2);
          bias ^= 3;        /* 1=>2, 2=>1 */
          inptr0 += 2; inptr1 += 2;
        }
        inrow += 2;
      }
        */
}


/**
  | Downsample pixel values of a single
  | component.
  | 
  | This version handles the standard case
  | of 2:1 horizontal and 2:1 vertical,
  | with smoothing. One row of context is
  | required.
  |
  */
#[cfg(INPUT_SMOOTHING_SUPPORTED)]
pub fn h2v2_smooth_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            int inrow, outrow;
      JDimension colctr;
      JDimension output_cols = compptr->width_in_blocks * DCTSIZE;
      JSAMPROW inptr0, inptr1, above_ptr, below_ptr, outptr;
      i32 membersum, neighsum, memberscale, neighscale;

      /* Expand input data enough to let all the output samples be generated
       * by the standard loop.  Special-casing padded output would be more
       * efficient.
       */
      expand_right_edge(input_data - 1, cinfo->max_v_samp_factor + 2,
                cinfo->image_width, output_cols * 2);

      /* We don't bother to form the individual "smoothed" input pixel values;
       * we can directly compute the output which is the average of the four
       * smoothed values.  Each of the four member pixels contributes a fraction
       * (1-8*SF) to its own smoothed image and a fraction SF to each of the three
       * other smoothed pixels, therefore a total fraction (1-5*SF)/4 to the final
       * output.  The four corner-adjacent neighbor pixels contribute a fraction
       * SF to just one smoothed pixel, or SF/4 to the final output; while the
       * eight edge-adjacent neighbors contribute SF to each of two smoothed
       * pixels, or SF/2 overall.  In order to use integer arithmetic, these
       * factors are scaled by 2^16 = 65536.
       * Also recall that SF = smoothing_factor / 1024.
       */

      memberscale = 16384 - cinfo->smoothing_factor * 80; /* scaled (1-5*SF)/4 */
      neighscale = cinfo->smoothing_factor * 16; /* scaled SF/4 */

      inrow = 0;
      for (outrow = 0; outrow < compptr->v_samp_factor; outrow++) {
        outptr = output_data[outrow];
        inptr0 = input_data[inrow];
        inptr1 = input_data[inrow+1];
        above_ptr = input_data[inrow-1];
        below_ptr = input_data[inrow+2];

        /* Special case for first column: pretend column -1 is same as column 0 */
        membersum = GETJSAMPLE(*inptr0) + GETJSAMPLE(inptr0[1]) +
            GETJSAMPLE(*inptr1) + GETJSAMPLE(inptr1[1]);
        neighsum = GETJSAMPLE(*above_ptr) + GETJSAMPLE(above_ptr[1]) +
               GETJSAMPLE(*below_ptr) + GETJSAMPLE(below_ptr[1]) +
               GETJSAMPLE(*inptr0) + GETJSAMPLE(inptr0[2]) +
               GETJSAMPLE(*inptr1) + GETJSAMPLE(inptr1[2]);
        neighsum += neighsum;
        neighsum += GETJSAMPLE(*above_ptr) + GETJSAMPLE(above_ptr[2]) +
            GETJSAMPLE(*below_ptr) + GETJSAMPLE(below_ptr[2]);
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr++ = (JSAMPLE) ((membersum + 32768) >> 16);
        inptr0 += 2; inptr1 += 2; above_ptr += 2; below_ptr += 2;

        for (colctr = output_cols - 2; colctr > 0; colctr--) {
          /* sum of pixels directly mapped to this output element */
          membersum = GETJSAMPLE(*inptr0) + GETJSAMPLE(inptr0[1]) +
              GETJSAMPLE(*inptr1) + GETJSAMPLE(inptr1[1]);
          /* sum of edge-neighbor pixels */
          neighsum = GETJSAMPLE(*above_ptr) + GETJSAMPLE(above_ptr[1]) +
             GETJSAMPLE(*below_ptr) + GETJSAMPLE(below_ptr[1]) +
             GETJSAMPLE(inptr0[-1]) + GETJSAMPLE(inptr0[2]) +
             GETJSAMPLE(inptr1[-1]) + GETJSAMPLE(inptr1[2]);
          /* The edge-neighbors count twice as much as corner-neighbors */
          neighsum += neighsum;
          /* Add in the corner-neighbors */
          neighsum += GETJSAMPLE(above_ptr[-1]) + GETJSAMPLE(above_ptr[2]) +
              GETJSAMPLE(below_ptr[-1]) + GETJSAMPLE(below_ptr[2]);
          /* form final output scaled up by 2^16 */
          membersum = membersum * memberscale + neighsum * neighscale;
          /* round, descale and output it */
          *outptr++ = (JSAMPLE) ((membersum + 32768) >> 16);
          inptr0 += 2; inptr1 += 2; above_ptr += 2; below_ptr += 2;
        }

        /* Special case for last column */
        membersum = GETJSAMPLE(*inptr0) + GETJSAMPLE(inptr0[1]) +
            GETJSAMPLE(*inptr1) + GETJSAMPLE(inptr1[1]);
        neighsum = GETJSAMPLE(*above_ptr) + GETJSAMPLE(above_ptr[1]) +
               GETJSAMPLE(*below_ptr) + GETJSAMPLE(below_ptr[1]) +
               GETJSAMPLE(inptr0[-1]) + GETJSAMPLE(inptr0[1]) +
               GETJSAMPLE(inptr1[-1]) + GETJSAMPLE(inptr1[1]);
        neighsum += neighsum;
        neighsum += GETJSAMPLE(above_ptr[-1]) + GETJSAMPLE(above_ptr[1]) +
            GETJSAMPLE(below_ptr[-1]) + GETJSAMPLE(below_ptr[1]);
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (JSAMPLE) ((membersum + 32768) >> 16);

        inrow += 2;
      }
        */
}

/**
  | Downsample pixel values of a single
  | component.
  | 
  | This version handles the special case
  | of a full-size component, with smoothing.
  | One row of context is required.
  |
  */
#[cfg(INPUT_SMOOTHING_SUPPORTED)]
pub fn fullsize_smooth_downsample(
        cinfo:       JCompressPtr,
        compptr:     *mut JpegComponentInfo,
        input_data:  JSampArray,
        output_data: JSampArray)  {
    
    todo!();
        /*
            int outrow;
      JDimension colctr;
      JDimension output_cols = compptr->width_in_blocks * DCTSIZE;
      JSAMPROW inptr, above_ptr, below_ptr, outptr;
      i32 membersum, neighsum, memberscale, neighscale;
      int colsum, lastcolsum, nextcolsum;

      /* Expand input data enough to let all the output samples be generated
       * by the standard loop.  Special-casing padded output would be more
       * efficient.
       */
      expand_right_edge(input_data - 1, cinfo->max_v_samp_factor + 2,
                cinfo->image_width, output_cols);

      /* Each of the eight neighbor pixels contributes a fraction SF to the
       * smoothed pixel, while the main pixel contributes (1-8*SF).  In order
       * to use integer arithmetic, these factors are multiplied by 2^16 = 65536.
       * Also recall that SF = smoothing_factor / 1024.
       */

      memberscale = 65536L - cinfo->smoothing_factor * 512L; /* scaled 1-8*SF */
      neighscale = cinfo->smoothing_factor * 64; /* scaled SF */

      for (outrow = 0; outrow < compptr->v_samp_factor; outrow++) {
        outptr = output_data[outrow];
        inptr = input_data[outrow];
        above_ptr = input_data[outrow-1];
        below_ptr = input_data[outrow+1];

        /* Special case for first column */
        colsum = GETJSAMPLE(*above_ptr++) + GETJSAMPLE(*below_ptr++) +
             GETJSAMPLE(*inptr);
        membersum = GETJSAMPLE(*inptr++);
        nextcolsum = GETJSAMPLE(*above_ptr) + GETJSAMPLE(*below_ptr) +
             GETJSAMPLE(*inptr);
        neighsum = colsum + (colsum - membersum) + nextcolsum;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr++ = (JSAMPLE) ((membersum + 32768) >> 16);
        lastcolsum = colsum; colsum = nextcolsum;

        for (colctr = output_cols - 2; colctr > 0; colctr--) {
          membersum = GETJSAMPLE(*inptr++);
          above_ptr++; below_ptr++;
          nextcolsum = GETJSAMPLE(*above_ptr) + GETJSAMPLE(*below_ptr) +
               GETJSAMPLE(*inptr);
          neighsum = lastcolsum + (colsum - membersum) + nextcolsum;
          membersum = membersum * memberscale + neighsum * neighscale;
          *outptr++ = (JSAMPLE) ((membersum + 32768) >> 16);
          lastcolsum = colsum; colsum = nextcolsum;
        }

        /* Special case for last column */
        membersum = GETJSAMPLE(*inptr);
        neighsum = lastcolsum + (colsum - membersum) + colsum;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (JSAMPLE) ((membersum + 32768) >> 16);

      }
        */
}

/**
  | Module initialization routine for
  | downsampling.
  | 
  | -----------
  | @note
  | 
  | we must select a routine for each component.
  |
  */
pub fn jinit_downsampler(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_downsample_ptr downsample;
      int ci;
      jpeg_component_info * compptr;
      boolean smoothok = TRUE;

      downsample = (my_downsample_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_downsampler));
      cinfo->downsample = (struct jpeg_downsampler *) downsample;
      downsample->pub.start_pass = start_pass_downsample;
      downsample->pub.downsample = sep_downsample;
      downsample->pub.need_context_rows = FALSE;

      if (cinfo->CCIR601_sampling)
        ERREXIT(cinfo, JERR_CCIR601_NOTIMPL);

      /* Verify we can handle the sampling factors, and set up method pointers */
      for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
           ci++, compptr++) {
        if (compptr->h_samp_factor == cinfo->max_h_samp_factor &&
        compptr->v_samp_factor == cinfo->max_v_samp_factor) {
    #ifdef INPUT_SMOOTHING_SUPPORTED
          if (cinfo->smoothing_factor) {
        downsample->methods[ci] = fullsize_smooth_downsample;
        downsample->pub.need_context_rows = TRUE;
          } else
    #endif
        downsample->methods[ci] = fullsize_downsample;
        } else if (compptr->h_samp_factor * 2 == cinfo->max_h_samp_factor &&
               compptr->v_samp_factor == cinfo->max_v_samp_factor) {
          smoothok = FALSE;
          downsample->methods[ci] = h2v1_downsample;
        } else if (compptr->h_samp_factor * 2 == cinfo->max_h_samp_factor &&
               compptr->v_samp_factor * 2 == cinfo->max_v_samp_factor) {
    #ifdef INPUT_SMOOTHING_SUPPORTED
          if (cinfo->smoothing_factor) {
        downsample->methods[ci] = h2v2_smooth_downsample;
        downsample->pub.need_context_rows = TRUE;
          } else
    #endif
        downsample->methods[ci] = h2v2_downsample;
        } else if ((cinfo->max_h_samp_factor % compptr->h_samp_factor) == 0 &&
               (cinfo->max_v_samp_factor % compptr->v_samp_factor) == 0) {
          smoothok = FALSE;
          downsample->methods[ci] = int_downsample;
        } else
          ERREXIT(cinfo, JERR_FRACT_SAMPLE_NOTIMPL);
      }

    #ifdef INPUT_SMOOTHING_SUPPORTED
      if (cinfo->smoothing_factor && !smoothok)
        TRACEMS(cinfo, 0, JTRC_SMOOTH_NOTIMPL);
    #endif
        */
}
