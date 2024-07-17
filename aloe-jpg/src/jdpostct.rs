/*!
  | jdpostct.c
  | 
  | This file contains the decompression
  | postprocessing controller.
  | 
  | This controller manages the upsampling,
  | color conversion, and color quantization/reduction
  | steps; specifically, it controls the
  | buffering between upsample/color
  | conversion and color quantization/reduction.
  | 
  | If no color quantization/reduction
  | is required, then this module has no
  | work to do, and it just hands off to the
  | upsample/color conversion code.
  | 
  | An integrated upsample/convert/quantize
  | process would replace this module entirely.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdpostct.c]

/**
  | Private buffer controller object
  |
  */
pub struct MyPostController {

    /**
      | public fields
      |
      */
    pub_:         JpegDPostController,

    /*
       | Color quantization source buffer:
       | this holds output data from the upsample/color
       | conversion step to be passed to the quantizer.
       | 
       | For two-pass color quantization, we
       | need a full-image buffer; for one-pass
       | operation, a strip buffer is sufficient.
       |
       */

    /**
      | virtual array, or NULL if one-pass
      |
      */
    whole_image:  JVirtSArrayPtr,


    /**
      | strip buffer, or current strip of virtual
      |
      */
    buffer:       JSampArray,

    /**
      | buffer size in rows
      |
      */
    strip_height: JDimension,

    /*
      | for two-pass mode only:
      |
      */

    /**
      | row # of first row in current strip
      |
      */
    starting_row: JDimension,


    /**
      | index of next row to fill/empty in strip
      |
      */
    next_row:     JDimension,
}

pub type MyPostPtr = *mut MyPostController;


/**
  | Initialize for a processing pass.
  |
  */
pub fn start_pass_dpost(
        cinfo:     JDecompressPtr,
        pass_mode: JBufMode)  {
    
    todo!();
        /*
            my_post_ptr post = (my_post_ptr) cinfo->post;

      switch (pass_mode) {
      case JBUF_PASS_THRU:
        if (cinfo->quantize_colors) {
          /* Single-pass processing with color quantization. */
          post->pub.post_process_data = post_process_1pass;
          /* We could be doing buffered-image output before starting a 2-pass
           * color quantization; in that case, jinit_d_post_controller did not
           * allocate a strip buffer.  Use the virtual-array buffer as workspace.
           */
          if (post->buffer == NULL) {
        post->buffer = (*cinfo->mem->access_virt_sarray)
          ((JCommonPtr) cinfo, post->whole_image,
           (JDimension) 0, post->strip_height, TRUE);
          }
        } else {
          /* For single-pass processing without color quantization,
           * I have no work to do; just call the upsampler directly.
           */
          post->pub.post_process_data = cinfo->upsample->upsample;
        }
        break;
    #ifdef QUANT_2PASS_SUPPORTED
      case JBUF_SAVE_AND_PASS:
        /* First pass of 2-pass quantization */
        if (post->whole_image == NULL)
          ERREXIT(cinfo, JERR_BAD_BUFFER_MODE);
        post->pub.post_process_data = post_process_prepass;
        break;
      case JBUF_CRANK_DEST:
        /* Second pass of 2-pass quantization */
        if (post->whole_image == NULL)
          ERREXIT(cinfo, JERR_BAD_BUFFER_MODE);
        post->pub.post_process_data = post_process_2pass;
        break;
    #endif /* QUANT_2PASS_SUPPORTED */
      default:
        ERREXIT(cinfo, JERR_BAD_BUFFER_MODE);
        break;
      }
      post->starting_row = post->next_row = 0;
        */
}

/**
  | Process some data in the one-pass (strip
  | buffer) case.
  | 
  | This is used for color precision reduction
  | as well as one-pass quantization.
  |
  */
pub fn post_process_1pass(
        cinfo:               JDecompressPtr,
        input_buf:           JSampImage,
        in_row_group_ctr:    *mut JDimension,
        in_row_groups_avail: JDimension,
        output_buf:          JSampArray,
        out_row_ctr:         *mut JDimension,
        out_rows_avail:      JDimension)  {
    
    todo!();
        /*
            my_post_ptr post = (my_post_ptr) cinfo->post;
      JDimension num_rows, max_rows;

      /* Fill the buffer, but not more than what we can dump out in one go. */
      /* Note we rely on the upsampler to detect bottom of image. */
      max_rows = out_rows_avail - *out_row_ctr;
      if (max_rows > post->strip_height)
        max_rows = post->strip_height;
      num_rows = 0;
      (*cinfo->upsample->upsample) (cinfo,
            input_buf, in_row_group_ctr, in_row_groups_avail,
            post->buffer, &num_rows, max_rows);
      /* Quantize and emit data. */
      (*cinfo->cquantize->color_quantize) (cinfo,
            post->buffer, output_buf + *out_row_ctr, (int) num_rows);
      *out_row_ctr += num_rows;
        */
}


/**
  | Process some data in the first pass of
  | 2-pass quantization.
  |
  */
#[cfg(QUANT_2PASS_SUPPORTED)]
pub fn post_process_prepass(
        cinfo:               JDecompressPtr,
        input_buf:           JSampImage,
        in_row_group_ctr:    *mut JDimension,
        in_row_groups_avail: JDimension,
        _4:                  JSampArray,
        out_row_ctr:         *mut JDimension,
        _6:                  JDimension)  {
    
    todo!();
        /*
            my_post_ptr post = (my_post_ptr) cinfo->post;
      JDimension old_next_row, num_rows;

      /* Reposition virtual buffer if at start of strip. */
      if (post->next_row == 0) {
        post->buffer = (*cinfo->mem->access_virt_sarray)
        ((JCommonPtr) cinfo, post->whole_image,
         post->starting_row, post->strip_height, TRUE);
      }

      /* Upsample some data (up to a strip height's worth). */
      old_next_row = post->next_row;
      (*cinfo->upsample->upsample) (cinfo,
            input_buf, in_row_group_ctr, in_row_groups_avail,
            post->buffer, &post->next_row, post->strip_height);

      /* Allow quantizer to scan new data.  No data is emitted, */
      /* but we advance out_row_ctr so outer loop can tell when we're done. */
      if (post->next_row > old_next_row) {
        num_rows = post->next_row - old_next_row;
        (*cinfo->cquantize->color_quantize) (cinfo, post->buffer + old_next_row,
                         (JSAMPARRAY) NULL, (int) num_rows);
        *out_row_ctr += num_rows;
      }

      /* Advance if we filled the strip. */
      if (post->next_row >= post->strip_height) {
        post->starting_row += post->strip_height;
        post->next_row = 0;
      }
        */
}

/**
  | Process some data in the second pass
  | of 2-pass quantization.
  |
  */
#[cfg(QUANT_2PASS_SUPPORTED)]
pub fn post_process_2pass(
        cinfo:          JDecompressPtr,
        _1:             JSampImage,
        _2:             *mut JDimension,
        _3:             JDimension,
        output_buf:     JSampArray,
        out_row_ctr:    *mut JDimension,
        out_rows_avail: JDimension)  {
    
    todo!();
        /*
            my_post_ptr post = (my_post_ptr) cinfo->post;
      JDimension num_rows, max_rows;

      /* Reposition virtual buffer if at start of strip. */
      if (post->next_row == 0) {
        post->buffer = (*cinfo->mem->access_virt_sarray)
        ((JCommonPtr) cinfo, post->whole_image,
         post->starting_row, post->strip_height, FALSE);
      }

      /* Determine number of rows to emit. */
      num_rows = post->strip_height - post->next_row; /* available in strip */
      max_rows = out_rows_avail - *out_row_ctr; /* available in output area */
      if (num_rows > max_rows)
        num_rows = max_rows;
      /* We have to check bottom of image here, can't depend on upsampler. */
      max_rows = cinfo->output_height - post->starting_row;
      if (num_rows > max_rows)
        num_rows = max_rows;

      /* Quantize and emit data. */
      (*cinfo->cquantize->color_quantize) (cinfo,
            post->buffer + post->next_row, output_buf + *out_row_ctr,
            (int) num_rows);
      *out_row_ctr += num_rows;

      /* Advance if we filled the strip. */
      post->next_row += num_rows;
      if (post->next_row >= post->strip_height) {
        post->starting_row += post->strip_height;
        post->next_row = 0;
      }
        */
}

/**
  | Initialize postprocessing controller.
  |
  */
pub fn jinit_d_post_controller(
        cinfo:            JDecompressPtr,
        need_full_buffer: bool)  {
    
    todo!();
        /*
            my_post_ptr post;

      post = (my_post_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_post_controller));
      cinfo->post = (struct jpeg_d_post_controller *) post;
      post->pub.start_pass = start_pass_dpost;
      post->whole_image = NULL; /* flag for no virtual arrays */
      post->buffer = NULL;      /* flag for no strip buffer */

      /* Create the quantization buffer, if needed */
      if (cinfo->quantize_colors) {
        /* The buffer strip height is max_v_samp_factor, which is typically
         * an efficient number of rows for upsampling to return.
         * (In the presence of output rescaling, we might want to be smarter?)
         */
        post->strip_height = (JDimension) cinfo->max_v_samp_factor;
        if (need_full_buffer) {
          /* Two-pass color quantization: need full-image storage. */
          /* We round up the number of rows to a multiple of the strip height. */
    #ifdef QUANT_2PASS_SUPPORTED
          post->whole_image = (*cinfo->mem->request_virt_sarray)
        ((JCommonPtr) cinfo, JPOOL_IMAGE, FALSE,
         cinfo->output_width * cinfo->out_color_components,
         (JDimension) jround_up((long) cinfo->output_height,
                    (long) post->strip_height),
         post->strip_height);
    #else
          ERREXIT(cinfo, JERR_BAD_BUFFER_MODE);
    #endif /* QUANT_2PASS_SUPPORTED */
        } else {
          /* One-pass color quantization: just make a strip buffer. */
          post->buffer = (*cinfo->mem->alloc_sarray)
        ((JCommonPtr) cinfo, JPOOL_IMAGE,
         cinfo->output_width * cinfo->out_color_components,
         post->strip_height);
        }
      }
        */
}
