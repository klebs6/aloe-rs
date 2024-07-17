/*!
  | jcmarker.c
  | 
  | This file contains routines to write
  | JPEG datastream markers.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jcmarker.c]

/**
  | Private state
  |
  */
pub struct MyMarkerWriter {

    /**
      | public fields
      |
      */
    pub_:                  JpegMarkerWriter,

    /**
      | last DRI value emitted; 0 after SOI
      |
      */
    last_restart_interval: u32,
}

pub type MyMarkerPtr = *mut MyMarkerWriter;

/**
  | Basic output routines.
  | 
  | -----------
  | @note
  | 
  | we do not support suspension while writing
  | a marker.
  | 
  | Therefore, an application using suspension
  | must ensure that there is enough buffer
  | space for the initial markers (typ.
  | 600-700 bytes) before calling jpeg_start_compress,
  | and enough space to write the trailing
  | EOI (a few bytes) before calling jpeg_finish_compress.
  | Multipass compression modes are not
  | supported at all with suspension, so
  | those two are the only points where markers
  | will be written.
  |
  | Emit a byte
  |
  */
pub fn emit_byte(
        cinfo: JCompressPtr,
        val:   i32)  {
    
    todo!();
        /*
            struct jpeg_destination_mgr * dest = cinfo->dest;

      *(dest->next_output_byte)++ = (JOctet) val;
      if (--dest->free_in_buffer == 0) {
        if (! (*dest->empty_output_buffer) (cinfo))
          ERREXIT(cinfo, JERR_CANT_SUSPEND);
      }
        */
}

/**
  | Emit a marker code
  |
  */
pub fn emit_marker(
        cinfo: JCompressPtr,
        mark:  JpegMarker)  {
    
    todo!();
        /*
            emit_byte(cinfo, 0xFF);
      emit_byte(cinfo, (int) mark);
        */
}

/**
  | Emit a 2-byte integer; these are always
  | MSB first in JPEG files
  |
  */
pub fn emit_2bytes(
        cinfo: JCompressPtr,
        value: i32)  {
    
    todo!();
        /*
            emit_byte(cinfo, (value >> 8) & 0xFF);
      emit_byte(cinfo, value & 0xFF);
        */
}

/**
  | Routines to write specific marker types.
  | 
  | Emit a DQT marker
  | 
  | Returns the precision used (0 = 8bits,
  | 1 = 16bits) for baseline checking
  |
  */
pub fn emit_dqt(
        cinfo: JCompressPtr,
        index: i32) -> i32 {
    
    todo!();
        /*
            JQUANT_TBL * qtbl = cinfo->quant_tbl_ptrs[index];
      int prec;
      int i;

      if (qtbl == NULL)
        ERREXIT1(cinfo, JERR_NO_QUANT_TABLE, index);

      prec = 0;
      for (i = 0; i < DCTSIZE2; i++) {
        if (qtbl->quantval[i] > 255)
          prec = 1;
      }

      if (! qtbl->sent_table) {
        emit_marker(cinfo, M_DQT);

        emit_2bytes(cinfo, prec ? DCTSIZE2*2 + 1 + 2 : DCTSIZE2 + 1 + 2);

        emit_byte(cinfo, index + (prec<<4));

        for (i = 0; i < DCTSIZE2; i++) {
          /* The table entries must be emitted in zigzag order. */
          unsigned int qval = qtbl->quantval[jpeg_natural_order[i]];
          if (prec)
        emit_byte(cinfo, (int) (qval >> 8));
          emit_byte(cinfo, (int) (qval & 0xFF));
        }

        qtbl->sent_table = TRUE;
      }

      return prec;
        */
}

/**
  | Emit a DHT marker
  |
  */
pub fn emit_dht(
        cinfo: JCompressPtr,
        index: i32,
        is_ac: bool)  {
    
    todo!();
        /*
            JHUFF_TBL * htbl;
      int length, i;

      if (is_ac) {
        htbl = cinfo->ac_huff_tbl_ptrs[index];
        index += 0x10;      /* output index has AC bit set */
      } else {
        htbl = cinfo->dc_huff_tbl_ptrs[index];
      }

      if (htbl == NULL)
        ERREXIT1(cinfo, JERR_NO_HUFF_TABLE, index);

      if (! htbl->sent_table) {
        emit_marker(cinfo, M_DHT);

        length = 0;
        for (i = 1; i <= 16; i++)
          length += htbl->bits[i];

        emit_2bytes(cinfo, length + 2 + 1 + 16);
        emit_byte(cinfo, index);

        for (i = 1; i <= 16; i++)
          emit_byte(cinfo, htbl->bits[i]);

        for (i = 0; i < length; i++)
          emit_byte(cinfo, htbl->huffval[i]);

        htbl->sent_table = TRUE;
      }
        */
}

/**
  | Emit a DAC marker
  | 
  | Since the useful info is so small, we
  | want to emit all the tables in one DAC
  | marker. Therefore this routine does
  | its own scan of the table.
  |
  */
pub fn emit_dac(_0: JCompressPtr)  {
    
    todo!();
        /*
            #ifdef C_ARITH_CODING_SUPPORTED
      char dc_in_use[NUM_ARITH_TBLS];
      char ac_in_use[NUM_ARITH_TBLS];
      int length, i;
      jpeg_component_info *compptr;

      for (i = 0; i < NUM_ARITH_TBLS; i++)
        dc_in_use[i] = ac_in_use[i] = 0;

      for (i = 0; i < cinfo->comps_in_scan; i++) {
        compptr = cinfo->cur_comp_info[i];
        dc_in_use[compptr->dc_tbl_no] = 1;
        ac_in_use[compptr->ac_tbl_no] = 1;
      }

      length = 0;
      for (i = 0; i < NUM_ARITH_TBLS; i++)
        length += dc_in_use[i] + ac_in_use[i];

      emit_marker(cinfo, M_DAC);

      emit_2bytes(cinfo, length*2 + 2);

      for (i = 0; i < NUM_ARITH_TBLS; i++) {
        if (dc_in_use[i]) {
          emit_byte(cinfo, i);
          emit_byte(cinfo, cinfo->arith_dc_L[i] + (cinfo->arith_dc_U[i]<<4));
        }
        if (ac_in_use[i]) {
          emit_byte(cinfo, i + 0x10);
          emit_byte(cinfo, cinfo->arith_ac_K[i]);
        }
      }
    #endif /* C_ARITH_CODING_SUPPORTED */
        */
}

/**
  | Emit a DRI marker
  |
  */
pub fn emit_dri(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            emit_marker(cinfo, M_DRI);

      emit_2bytes(cinfo, 4);    /* fixed length */

      emit_2bytes(cinfo, (int) cinfo->restart_interval);
        */
}

/**
  | Emit a SOF marker
  |
  */
pub fn emit_sof(
        cinfo: JCompressPtr,
        code:  JpegMarker)  {
    
    todo!();
        /*
            int ci;
      jpeg_component_info *compptr;

      emit_marker(cinfo, code);

      emit_2bytes(cinfo, 3 * cinfo->num_components + 2 + 5 + 1); /* length */

      /* Make sure image isn't bigger than SOF field can handle */
      if ((long) cinfo->image_height > 65535L ||
          (long) cinfo->image_width > 65535L)
        ERREXIT1(cinfo, JERR_IMAGE_TOO_BIG, (unsigned int) 65535);

      emit_byte(cinfo, cinfo->data_precision);
      emit_2bytes(cinfo, (int) cinfo->image_height);
      emit_2bytes(cinfo, (int) cinfo->image_width);

      emit_byte(cinfo, cinfo->num_components);

      for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
           ci++, compptr++) {
        emit_byte(cinfo, compptr->component_id);
        emit_byte(cinfo, (compptr->h_samp_factor << 4) + compptr->v_samp_factor);
        emit_byte(cinfo, compptr->quant_tbl_no);
      }
        */
}

/**
  | Emit a SOS marker
  |
  */
pub fn emit_sos(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            int i, td, ta;
      jpeg_component_info *compptr;

      emit_marker(cinfo, M_SOS);

      emit_2bytes(cinfo, 2 * cinfo->comps_in_scan + 2 + 1 + 3); /* length */

      emit_byte(cinfo, cinfo->comps_in_scan);

      for (i = 0; i < cinfo->comps_in_scan; i++) {
        compptr = cinfo->cur_comp_info[i];
        emit_byte(cinfo, compptr->component_id);
        td = compptr->dc_tbl_no;
        ta = compptr->ac_tbl_no;
        if (cinfo->progressive_mode) {
          /* Progressive mode: only DC or only AC tables are used in one scan;
           * furthermore, Huffman coding of DC refinement uses no table at all.
           * We emit 0 for unused field(s); this is recommended by the P&M text
           * but does not seem to be specified in the standard.
           */
          if (cinfo->Ss == 0) {
        ta = 0;         /* DC scan */
        if (cinfo->Ah != 0 && !cinfo->arith_code)
          td = 0;       /* no DC table either */
          } else {
        td = 0;         /* AC scan */
          }
        }
        emit_byte(cinfo, (td << 4) + ta);
      }

      emit_byte(cinfo, cinfo->Ss);
      emit_byte(cinfo, cinfo->Se);
      emit_byte(cinfo, (cinfo->Ah << 4) + cinfo->Al);
        */
}

/**
  | Emit a JFIF-compliant APP0 marker
  |
  */
pub fn emit_jfif_app0(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            /*
       * Length of APP0 block   (2 bytes)
       * Block ID           (4 bytes - ASCII "JFIF")
       * Zero byte          (1 byte to terminate the ID string)
       * Version Major, Minor   (2 bytes - major first)
       * Units          (1 byte - 0x00 = none, 0x01 = inch, 0x02 = cm)
       * Xdpu           (2 bytes - dots per unit horizontal)
       * Ydpu           (2 bytes - dots per unit vertical)
       * Thumbnail X size       (1 byte)
       * Thumbnail Y size       (1 byte)
       */

      emit_marker(cinfo, M_APP0);

      emit_2bytes(cinfo, 2 + 4 + 1 + 2 + 1 + 2 + 2 + 1 + 1); /* length */

      emit_byte(cinfo, 0x4A);   /* Identifier: ASCII "JFIF" */
      emit_byte(cinfo, 0x46);
      emit_byte(cinfo, 0x49);
      emit_byte(cinfo, 0x46);
      emit_byte(cinfo, 0);
      emit_byte(cinfo, cinfo->JFIF_major_version); /* Version fields */
      emit_byte(cinfo, cinfo->JFIF_minor_version);
      emit_byte(cinfo, cinfo->density_unit); /* Pixel size information */
      emit_2bytes(cinfo, (int) cinfo->X_density);
      emit_2bytes(cinfo, (int) cinfo->Y_density);
      emit_byte(cinfo, 0);      /* No thumbnail image */
      emit_byte(cinfo, 0);
        */
}

/**
  | Emit an Adobe APP14 marker
  |
  */
pub fn emit_adobe_app14(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            /*
       * Length of APP14 block  (2 bytes)
       * Block ID           (5 bytes - ASCII "Adobe")
       * Version Number     (2 bytes - currently 100)
       * Flags0         (2 bytes - currently 0)
       * Flags1         (2 bytes - currently 0)
       * Color transform        (1 byte)
       *
       * Although Adobe TN 5116 mentions Version = 101, all the Adobe files
       * now in circulation seem to use Version = 100, so that's what we write.
       *
       * We write the color transform byte as 1 if the JPEG color space is
       * YCbCr, 2 if it's YCCK, 0 otherwise.  Adobe's definition has to do with
       * whether the encoder performed a transformation, which is pretty useless.
       */

      emit_marker(cinfo, M_APP14);

      emit_2bytes(cinfo, 2 + 5 + 2 + 2 + 2 + 1); /* length */

      emit_byte(cinfo, 0x41);   /* Identifier: ASCII "Adobe" */
      emit_byte(cinfo, 0x64);
      emit_byte(cinfo, 0x6F);
      emit_byte(cinfo, 0x62);
      emit_byte(cinfo, 0x65);
      emit_2bytes(cinfo, 100);  /* Version */
      emit_2bytes(cinfo, 0);    /* Flags0 */
      emit_2bytes(cinfo, 0);    /* Flags1 */
      switch (cinfo->jpeg_color_space) {
      case JCS_YCbCr:
        emit_byte(cinfo, 1);    /* Color transform = 1 */
        break;
      case JCS_YCCK:
        emit_byte(cinfo, 2);    /* Color transform = 2 */
        break;
      default:
        emit_byte(cinfo, 0);    /* Color transform = 0 */
        break;
      }
        */
}

/**
  | These routines allow writing an arbitrary
  | marker with parameters.
  | 
  | The only intended use is to emit COM or
  | APPn markers after calling write_file_header
  | and before calling write_frame_header.
  | 
  | Other uses are not guaranteed to produce
  | desirable results.
  | 
  | Counting the parameter bytes properly
  | is the caller's responsibility.
  |
  | Emit an arbitrary marker header
  |
  */
pub fn write_marker_header(
        cinfo:   JCompressPtr,
        marker:  i32,
        datalen: u32)  {
    
    todo!();
        /*
            if (datalen > (unsigned int) 65533)       /* safety check */
        ERREXIT(cinfo, JERR_BAD_LENGTH);

      emit_marker(cinfo, (JPEG_MARKER) marker);

      emit_2bytes(cinfo, (int) (datalen + 2));  /* total length */
        */
}

/**
  | Emit one byte of marker parameters following
  | write_marker_header
  |
  */
pub fn write_marker_byte(
        cinfo: JCompressPtr,
        val:   i32)  {
    
    todo!();
        /*
            emit_byte(cinfo, val);
        */
}

/**
  | Write datastream header.
  | 
  | This consists of an SOI and optional
  | APPn markers.
  | 
  | We recommend use of the JFIF marker,
  | but not the Adobe marker, when using
  | YCbCr or grayscale data. The JFIF marker
  | should NOT be used for any other JPEG
  | colorspace. The Adobe marker is helpful
  | to distinguish RGB, CMYK, and YCCK colorspaces.
  | 
  | -----------
  | @note
  | 
  | an application can write additional
  | header markers after jpeg_start_compress
  | returns.
  |
  */
pub fn write_file_header(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_marker_ptr marker = (my_marker_ptr) cinfo->marker;

      emit_marker(cinfo, M_SOI);    /* first the SOI */

      /* SOI is defined to reset restart interval to 0 */
      marker->last_restart_interval = 0;

      if (cinfo->write_JFIF_header) /* next an optional JFIF APP0 */
        emit_jfif_app0(cinfo);
      if (cinfo->write_Adobe_marker) /* next an optional Adobe APP14 */
        emit_adobe_app14(cinfo);
        */
}

/**
  | Write frame header.
  | 
  | This consists of DQT and SOFn markers.
  | 
  | -----------
  | @note
  | 
  | we do not emit the SOF until we have emitted
  | the DQT(s).
  | 
  | This avoids compatibility problems
  | with incorrect implementations that
  | try to error-check the quant table numbers
  | as soon as they see the SOF.
  |
  */
pub fn write_frame_header(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            int ci, prec;
      boolean is_baseline;
      jpeg_component_info *compptr;

      /* Emit DQT for each quantization table.
       * Note that emit_dqt() suppresses any duplicate tables.
       */
      prec = 0;
      for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
           ci++, compptr++) {
        prec += emit_dqt(cinfo, compptr->quant_tbl_no);
      }
      /* now prec is nonzero iff there are any 16-bit quant tables. */

      /* Check for a non-baseline specification.
       * Note we assume that Huffman table numbers won't be changed later.
       */
      if (cinfo->arith_code || cinfo->progressive_mode ||
          cinfo->data_precision != 8) {
        is_baseline = FALSE;
      } else {
        is_baseline = TRUE;
        for (ci = 0, compptr = cinfo->comp_info; ci < cinfo->num_components;
         ci++, compptr++) {
          if (compptr->dc_tbl_no > 1 || compptr->ac_tbl_no > 1)
        is_baseline = FALSE;
        }
        if (prec && is_baseline) {
          is_baseline = FALSE;
          /* If it's baseline except for quantizer size, warn the user */
          TRACEMS(cinfo, 0, JTRC_16BIT_TABLES);
        }
      }

      /* Emit the proper SOF marker */
      if (cinfo->arith_code) {
        emit_sof(cinfo, M_SOF9);    /* SOF code for arithmetic coding */
      } else {
        if (cinfo->progressive_mode)
          emit_sof(cinfo, M_SOF2);  /* SOF code for progressive Huffman */
        else if (is_baseline)
          emit_sof(cinfo, M_SOF0);  /* SOF code for baseline implementation */
        else
          emit_sof(cinfo, M_SOF1);  /* SOF code for non-baseline Huffman file */
      }
        */
}

/**
  | Write scan header.
  | 
  | This consists of DHT or DAC markers,
  | optional DRI, and SOS.
  | 
  | Compressed data will be written following
  | the SOS.
  |
  */
pub fn write_scan_header(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_marker_ptr marker = (my_marker_ptr) cinfo->marker;
      int i;
      jpeg_component_info *compptr;

      if (cinfo->arith_code) {
        /* Emit arith conditioning info.  We may have some duplication
         * if the file has multiple scans, but it's so small it's hardly
         * worth worrying about.
         */
        emit_dac(cinfo);
      } else {
        /* Emit Huffman tables.
         * Note that emit_dht() suppresses any duplicate tables.
         */
        for (i = 0; i < cinfo->comps_in_scan; i++) {
          compptr = cinfo->cur_comp_info[i];
          if (cinfo->progressive_mode) {
        /* Progressive mode: only DC or only AC tables are used in one scan */
        if (cinfo->Ss == 0) {
          if (cinfo->Ah == 0)   /* DC needs no table for refinement scan */
            emit_dht(cinfo, compptr->dc_tbl_no, FALSE);
        } else {
          emit_dht(cinfo, compptr->ac_tbl_no, TRUE);
        }
          } else {
        /* Sequential mode: need both DC and AC tables */
        emit_dht(cinfo, compptr->dc_tbl_no, FALSE);
        emit_dht(cinfo, compptr->ac_tbl_no, TRUE);
          }
        }
      }

      /* Emit DRI if required --- note that DRI value could change for each scan.
       * We avoid wasting space with unnecessary DRIs, however.
       */
      if (cinfo->restart_interval != marker->last_restart_interval) {
        emit_dri(cinfo);
        marker->last_restart_interval = cinfo->restart_interval;
      }

      emit_sos(cinfo);
        */
}

/**
  | Write datastream trailer.
  |
  */
pub fn write_file_trailer(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            emit_marker(cinfo, M_EOI);
        */
}

/**
  | Write an abbreviated table-specification
  | datastream.
  | 
  | This consists of SOI, DQT and DHT tables,
  | and EOI.
  | 
  | Any table that is defined and not marked
  | sent_table = TRUE will be emitted. Note
  | that all tables will be marked sent_table
  | = TRUE at exit.
  |
  */
pub fn write_tables_only(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            int i;

      emit_marker(cinfo, M_SOI);

      for (i = 0; i < NUM_QUANT_TBLS; i++) {
        if (cinfo->quant_tbl_ptrs[i] != NULL)
          (c_void) emit_dqt(cinfo, i);
      }

      if (! cinfo->arith_code) {
        for (i = 0; i < NUM_HUFF_TBLS; i++) {
          if (cinfo->dc_huff_tbl_ptrs[i] != NULL)
        emit_dht(cinfo, i, FALSE);
          if (cinfo->ac_huff_tbl_ptrs[i] != NULL)
        emit_dht(cinfo, i, TRUE);
        }
      }

      emit_marker(cinfo, M_EOI);
        */
}

/**
  | Initialize the marker writer module.
  |
  */
pub fn jinit_marker_writer(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            my_marker_ptr marker;

      /* Create the subobject */
      marker = (my_marker_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(my_marker_writer));
      cinfo->marker = (struct jpeg_marker_writer *) marker;
      /* Initialize method pointers */
      marker->pub.write_file_header = write_file_header;
      marker->pub.write_frame_header = write_frame_header;
      marker->pub.write_scan_header = write_scan_header;
      marker->pub.write_file_trailer = write_file_trailer;
      marker->pub.write_tables_only = write_tables_only;
      marker->pub.write_marker_header = write_marker_header;
      marker->pub.write_marker_byte = write_marker_byte;
      /* Initialize private state */
      marker->last_restart_interval = 0;
        */
}
