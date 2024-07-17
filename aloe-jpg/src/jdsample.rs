/*!
  | jdsample.c
  | 
  | This file contains upsampling routines.
  | 
  | Upsampling input data is counted in
  | "row groups". A row group is defined
  | to be (v_samp_factor * DCT_scaled_size
  | / min_DCT_scaled_size) sample rows
  | of each component. Upsampling will
  | normally produce max_v_samp_factor
  | pixel rows from each row group (but this
  | could vary if the upsampler is applying
  | a scale factor of its own).
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
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdsample.c]

/**
  | Pointer to routine to upsample a single
  | component
  |
  */
pub type Upsample1Ptr = fn(
        cinfo:           JDecompressPtr,
        compptr:         *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray
) -> ();


/**
  | Private subobject
  |
  */
pub struct MyUpsampler2 {

    /**
      | public fields
      |
      */
    pub_:            JpegUpsampler,


    /**
      | Color conversion buffer. When using
      | separate upsampling and color conversion
      | steps, this buffer holds one upsampled
      | row group until it has been color converted
      | and output.
      | 
      | -----------
      | @note
      | 
      | we do not allocate any storage for component(s)
      | which are full-size, ie do not need rescaling.
      | The corresponding entry of color_buf[]
      | is simply set to point to the input data
      | array, thereby avoiding copying.
      |
      */
    color_buf:       [JSampArray; MAX_COMPONENTS],


    /**
      | Per-component upsampling method pointers
      |
      */
    methods:         [Upsample1Ptr; MAX_COMPONENTS],


    /**
      | counts rows emitted from color_buf
      |
      */
    next_row_out:    i32,


    /**
      | counts rows remaining in image
      |
      */
    rows_to_go:      JDimension,


    /**
      | Height of an input row group for each
      | component.
      |
      */
    rowgroup_height: [i32; MAX_COMPONENTS],


    /**
      | These arrays save pixel expansion factors
      | so that int_expand need not recompute
      | them each time. They are unused for other
      | upsampling methods.
      |
      */
    h_expand:        [u8; MAX_COMPONENTS],

    expand:          [u8; MAX_COMPONENTS],
}

pub type MyUpsamplePtr2 = *mut MyUpsampler2;

/**
  | Initialize for an upsampling pass.
  |
  */
pub fn start_pass_upsample(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            my_upsample_ptr2 upsample = (my_upsample_ptr2) cinfo->upsample;

      /* Mark the conversion buffer empty */
      upsample->next_row_out = cinfo->max_v_samp_factor;
      /* Initialize total-height counter for detecting bottom of image */
      upsample->rows_to_go = cinfo->output_height;
        */
}

/**
  | Control routine to do upsampling (and
  | color conversion).
  | 
  | In this version we upsample each component
  | independently.
  | 
  | We upsample one row group into the conversion
  | buffer, then apply color conversion
  | a row at a time.
  |
  */
pub fn sep_upsample(
        cinfo:            JDecompressPtr,
        input_buf:        JSampImage,
        in_row_group_ctr: *mut JDimension,
        _3:               JDimension,
        output_buf:       JSampArray,
        out_row_ctr:      *mut JDimension,
        out_rows_avail:   JDimension)  {
    
    todo!();
        /*
            my_upsample_ptr2 upsample = (my_upsample_ptr2) cinfo->upsample;
      int ci;
      jpeg_component_info * compptr;
      JDimension num_rows;

      /* Fill the conversion buffer, if it's empty */
      if (upsample->next_row_out >= cinfo->max_v_samp_factor) {
        for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
         ci++, compptr++) {
          /* Invoke per-component upsample method.  Notice we pass a POINTER
           * to color_buf[ci], so that fullsize_upsample can change it.
           */
          (*upsample->methods[ci]) (cinfo, compptr,
        input_buf[ci] + (*in_row_group_ctr * upsample->rowgroup_height[ci]),
        upsample->color_buf + ci);
        }
        upsample->next_row_out = 0;
      }

      /* Color-convert and emit rows */

      /* How many we have in the buffer: */
      num_rows = (JDimension) (cinfo->max_v_samp_factor - upsample->next_row_out);
      /* Not more than the distance to the end of the image.  Need this test
       * in case the image height is not a multiple of max_v_samp_factor:
       */
      if (num_rows > upsample->rows_to_go)
        num_rows = upsample->rows_to_go;
      /* And not more than what the client can accept: */
      out_rows_avail -= *out_row_ctr;
      if (num_rows > out_rows_avail)
        num_rows = out_rows_avail;

      (*cinfo->cconvert->color_convert) (cinfo, upsample->color_buf,
                         (JDimension) upsample->next_row_out,
                         output_buf + *out_row_ctr,
                         (int) num_rows);

      /* Adjust counts */
      *out_row_ctr += num_rows;
      upsample->rows_to_go -= num_rows;
      upsample->next_row_out += num_rows;
      /* When the buffer is emptied, declare this input row group consumed */
      if (upsample->next_row_out >= cinfo->max_v_samp_factor)
        (*in_row_group_ctr)++;
        */
}

/*
  | These are the routines invoked by sep_upsample
  | to upsample pixel values of a single
  | component. One row group is processed
  | per call.
  |
  */

/**
  | For full-size components, we just make
  | color_buf[ci] point at the input buffer,
  | and thus avoid copying any data. Note
  | that this is safe only because sep_upsample
  | doesn't declare the input row group
  | "consumed" until we are done color converting
  | and emitting it.
  |
  */
pub fn fullsize_upsample(
        _0:              JDecompressPtr,
        _1:              *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            *output_data_ptr = input_data;
        */
}

/**
  | This is a no-op version used for "uninteresting"
  | components.
  | 
  | These components will not be referenced
  | by color conversion.
  |
  */
pub fn noop_upsample(
        _0:              JDecompressPtr,
        _1:              *mut JpegComponentInfo,
        _2:              JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            *output_data_ptr = NULL;  /* safety check */
        */
}

/**
  | This version handles any integral sampling
  | ratios.
  | 
  | This is not used for typical JPEG files,
  | so it need not be fast.
  | 
  | Nor, for that matter, is it particularly
  | accurate: the algorithm is simple replication
  | of the input pixel onto the corresponding
  | output pixels. The hi-falutin sampling
  | literature refers to this as a "box filter".
  | A box filter tends to introduce visible
  | artifacts, so if you are actually going
  | to use 3:1 or 4:1 sampling ratios you
  | would be well advised to improve this
  | code.
  |
  */
pub fn int_upsample(
        cinfo:           JDecompressPtr,
        compptr:         *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            my_upsample_ptr2 upsample = (my_upsample_ptr2) cinfo->upsample;
      JSAMPARRAY output_data = *output_data_ptr;
      JSAMPROW inptr, outptr;
      JSAMPLE invalue;
      int h;
      JSAMPROW outend;
      int h_expand, v_expand;
      int inrow, outrow;

      h_expand = upsample->h_expand[compptr->component_index];
      v_expand = upsample->v_expand[compptr->component_index];

      inrow = outrow = 0;
      while (outrow < cinfo->max_v_samp_factor) {
        /* Generate one output row with proper horizontal expansion */
        inptr = input_data[inrow];
        outptr = output_data[outrow];
        outend = outptr + cinfo->output_width;
        while (outptr < outend) {
          invalue = *inptr++;   /* don't need GETJSAMPLE() here */
          for (h = h_expand; h > 0; h--) {
        *outptr++ = invalue;
          }
        }
        /* Generate any additional output rows by duplicating the first one */
        if (v_expand > 1) {
          jcopy_sample_rows(output_data, outrow, output_data, outrow+1,
                v_expand-1, cinfo->output_width);
        }
        inrow++;
        outrow += v_expand;
      }
        */
}

/**
  | Fast processing for the common case
  | of 2:1 horizontal and 1:1 vertical.
  | 
  | It's still a box filter.
  |
  */
pub fn h2v1_upsample(
        cinfo:           JDecompressPtr,
        _1:              *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            JSAMPARRAY output_data = *output_data_ptr;
      JSAMPROW inptr, outptr;
      JSAMPLE invalue;
      JSAMPROW outend;
      int inrow;

      for (inrow = 0; inrow < cinfo->max_v_samp_factor; inrow++) {
        inptr = input_data[inrow];
        outptr = output_data[inrow];
        outend = outptr + cinfo->output_width;
        while (outptr < outend) {
          invalue = *inptr++;   /* don't need GETJSAMPLE() here */
          *outptr++ = invalue;
          *outptr++ = invalue;
        }
      }
        */
}

/**
  | Fast processing for the common case
  | of 2:1 horizontal and 2:1 vertical.
  | 
  | It's still a box filter.
  |
  */
pub fn h2v2_upsample(
        cinfo:           JDecompressPtr,
        _1:              *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            JSAMPARRAY output_data = *output_data_ptr;
      JSAMPROW inptr, outptr;
      JSAMPLE invalue;
      JSAMPROW outend;
      int inrow, outrow;

      inrow = outrow = 0;
      while (outrow < cinfo->max_v_samp_factor) {
        inptr = input_data[inrow];
        outptr = output_data[outrow];
        outend = outptr + cinfo->output_width;
        while (outptr < outend) {
          invalue = *inptr++;   /* don't need GETJSAMPLE() here */
          *outptr++ = invalue;
          *outptr++ = invalue;
        }
        jcopy_sample_rows(output_data, outrow, output_data, outrow+1,
                  1, cinfo->output_width);
        inrow++;
        outrow += 2;
      }
        */
}

/**
  | Fancy processing for the common case
  | of 2:1 horizontal and 1:1 vertical.
  | 
  | The upsampling algorithm is linear
  | interpolation between pixel centers,
  | also known as a "triangle filter". This
  | is a good compromise between speed and
  | visual quality. The centers of the output
  | pixels are 1/4 and 3/4 of the way between
  | input pixel centers.
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
pub fn h2v1_fancy_upsample(
        cinfo:           JDecompressPtr,
        compptr:         *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            JSAMPARRAY output_data = *output_data_ptr;
      JSAMPROW inptr, outptr;
      int invalue;
      JDimension colctr;
      int inrow;

      for (inrow = 0; inrow < cinfo->max_v_samp_factor; inrow++) {
        inptr = input_data[inrow];
        outptr = output_data[inrow];
        /* Special case for first column */
        invalue = GETJSAMPLE(*inptr++);
        *outptr++ = (JSAMPLE) invalue;
        *outptr++ = (JSAMPLE) ((invalue * 3 + GETJSAMPLE(*inptr) + 2) >> 2);

        for (colctr = compptr->downsampled_width - 2; colctr > 0; colctr--) {
          /* General case: 3/4 * nearer pixel + 1/4 * further pixel */
          invalue = GETJSAMPLE(*inptr++) * 3;
          *outptr++ = (JSAMPLE) ((invalue + GETJSAMPLE(inptr[-2]) + 1) >> 2);
          *outptr++ = (JSAMPLE) ((invalue + GETJSAMPLE(*inptr) + 2) >> 2);
        }

        /* Special case for last column */
        invalue = GETJSAMPLE(*inptr);
        *outptr++ = (JSAMPLE) ((invalue * 3 + GETJSAMPLE(inptr[-1]) + 1) >> 2);
        *outptr++ = (JSAMPLE) invalue;
      }
        */
}

/**
  | Fancy processing for the common case
  | of 2:1 horizontal and 2:1 vertical.
  | 
  | Again a triangle filter; see comments
  | for h2v1 case, above.
  | 
  | It is OK for us to reference the adjacent
  | input rows because we demanded context
  | from the main buffer controller (see
  | initialization code).
  |
  */
pub fn h2v2_fancy_upsample(
        cinfo:           JDecompressPtr,
        compptr:         *mut JpegComponentInfo,
        input_data:      JSampArray,
        output_data_ptr: *mut JSampArray)  {
    
    todo!();
        /*
            JSAMPARRAY output_data = *output_data_ptr;
      JSAMPROW inptr0, inptr1, outptr;
    #if BITS_IN_JSAMPLE == 8
      int thiscolsum, lastcolsum, nextcolsum;
    #else
      i32 thiscolsum, lastcolsum, nextcolsum;
    #endif
      JDimension colctr;
      int inrow, outrow, v;

      inrow = outrow = 0;
      while (outrow < cinfo->max_v_samp_factor) {
        for (v = 0; v < 2; v++) {
          /* inptr0 points to nearest input row, inptr1 points to next nearest */
          inptr0 = input_data[inrow];
          if (v == 0)       /* next nearest is row above */
        inptr1 = input_data[inrow-1];
          else          /* next nearest is row below */
        inptr1 = input_data[inrow+1];
          outptr = output_data[outrow++];

          /* Special case for first column */
          thiscolsum = GETJSAMPLE(*inptr0++) * 3 + GETJSAMPLE(*inptr1++);
          nextcolsum = GETJSAMPLE(*inptr0++) * 3 + GETJSAMPLE(*inptr1++);
          *outptr++ = (JSAMPLE) ((thiscolsum * 4 + 8) >> 4);
          *outptr++ = (JSAMPLE) ((thiscolsum * 3 + nextcolsum + 7) >> 4);
          lastcolsum = thiscolsum; thiscolsum = nextcolsum;

          for (colctr = compptr->downsampled_width - 2; colctr > 0; colctr--) {
        /* General case: 3/4 * nearer pixel + 1/4 * further pixel in each */
        /* dimension, thus 9/16, 3/16, 3/16, 1/16 overall */
        nextcolsum = GETJSAMPLE(*inptr0++) * 3 + GETJSAMPLE(*inptr1++);
        *outptr++ = (JSAMPLE) ((thiscolsum * 3 + lastcolsum + 8) >> 4);
        *outptr++ = (JSAMPLE) ((thiscolsum * 3 + nextcolsum + 7) >> 4);
        lastcolsum = thiscolsum; thiscolsum = nextcolsum;
          }

          /* Special case for last column */
          *outptr++ = (JSAMPLE) ((thiscolsum * 3 + lastcolsum + 8) >> 4);
          *outptr++ = (JSAMPLE) ((thiscolsum * 4 + 7) >> 4);
        }
        inrow++;
      }
        */
}

/**
  | Module initialization routine for
  | upsampling.
  |
  */
pub fn jinit_upsampler(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            my_upsample_ptr2 upsample;
      int ci;
      jpeg_component_info * compptr;
      boolean need_buffer, do_fancy;
      int h_in_group, v_in_group, h_out_group, v_out_group;

      upsample = (my_upsample_ptr2)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_upsampler2));
      cinfo->upsample = (struct jpeg_upsampler *) upsample;
      upsample->pub.start_pass = start_pass_upsample;
      upsample->pub.upsample = sep_upsample;
      upsample->pub.need_context_rows = FALSE; /* until we find out differently */

      if (cinfo->CCIR601_sampling)  /* this isn't supported */
        ERREXIT(cinfo, JERR_CCIR601_NOTIMPL);

      /* jdmainct.c doesn't support context rows when min_DCT_scaled_size = 1,
       * so don't ask for it.
       */
      do_fancy = cinfo->do_fancy_upsampling && cinfo->min_DCT_scaled_size > 1;

      /* Verify we can handle the sampling factors, select per-component methods,
       * and create storage as needed.
       */
      for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
           ci++, compptr++) {
        /* Compute size of an "input group" after IDCT scaling.  This many samples
         * are to be converted to max_h_samp_factor * max_v_samp_factor pixels.
         */
        h_in_group = (compptr->h_samp_factor * compptr->DCT_scaled_size) /
             cinfo->min_DCT_scaled_size;
        v_in_group = (compptr->v_samp_factor * compptr->DCT_scaled_size) /
             cinfo->min_DCT_scaled_size;
        h_out_group = cinfo->max_h_samp_factor;
        v_out_group = cinfo->max_v_samp_factor;
        upsample->rowgroup_height[ci] = v_in_group; /* save for use later */
        need_buffer = TRUE;
        if (! compptr->component_needed) {
          /* Don't bother to upsample an uninteresting component. */
          upsample->methods[ci] = noop_upsample;
          need_buffer = FALSE;
        } else if (h_in_group == h_out_group && v_in_group == v_out_group) {
          /* Fullsize components can be processed without any work. */
          upsample->methods[ci] = fullsize_upsample;
          need_buffer = FALSE;
        } else if (h_in_group * 2 == h_out_group &&
               v_in_group == v_out_group) {
          /* Special cases for 2h1v upsampling */
          if (do_fancy && compptr->downsampled_width > 2)
        upsample->methods[ci] = h2v1_fancy_upsample;
          else
        upsample->methods[ci] = h2v1_upsample;
        } else if (h_in_group * 2 == h_out_group &&
               v_in_group * 2 == v_out_group) {
          /* Special cases for 2h2v upsampling */
          if (do_fancy && compptr->downsampled_width > 2) {
        upsample->methods[ci] = h2v2_fancy_upsample;
        upsample->pub.need_context_rows = TRUE;
          } else
        upsample->methods[ci] = h2v2_upsample;
        } else if ((h_out_group % h_in_group) == 0 &&
               (v_out_group % v_in_group) == 0) {
          /* Generic integral-factors upsampling method */
          upsample->methods[ci] = int_upsample;
          upsample->h_expand[ci] = (UINT8) (h_out_group / h_in_group);
          upsample->v_expand[ci] = (UINT8) (v_out_group / v_in_group);
        } else
          ERREXIT(cinfo, JERR_FRACT_SAMPLE_NOTIMPL);
        if (need_buffer) {
          upsample->color_buf[ci] = (*cinfo->mem->alloc_sarray)
        ((JCommonPtr) cinfo, JPOOL_IMAGE,
         (JDimension) jround_up((long) cinfo->output_width,
                    (long) cinfo->max_h_samp_factor),
         (JDimension) cinfo->max_v_samp_factor);
        }
      }
        */
}
