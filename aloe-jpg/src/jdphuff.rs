/*!
  | jdphuff.c
  | 
  | This file contains Huffman entropy
  | decoding routines for progressive
  | JPEG.
  | 
  | Much of the complexity here has to do
  | with supporting input suspension.
  | 
  | If the data source module demands suspension,
  | we want to be able to back up to the start
  | of the current MCU. To do this, we copy
  | state variables into local working
  | storage, and update them back to the
  | permanent storage only upon successful
  | completion of an MCU.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdphuff.c]

#[cfg(D_PROGRESSIVE_SUPPORTED)]
pub mod progressive_supported {
    use super::*;

    /**
      | Expanded entropy decoder object for
      | progressive Huffman decoding.
      | 
      | The savable_state subrecord contains
      | fields that change within an MCU, but
      | must not be updated permanently until
      | we complete the MCU.
      |
      */
    pub struct SavableState3 {

        /**
          | remaining EOBs in EOBRUN
          |
          */
        eobrun:      u32,


        /**
          | last DC coef for each component
          |
          */
        last_dc_val: [i32; MAX_COMPS_IN_SCAN],
    }

    /**
      | This macro is to work around compilers
      | with missing or broken structure assignment.
      | You'll need to fix this code if you have
      | such a compiler and you change MAX_COMPS_IN_SCAN.
      |
      */
    #[cfg(not(NO_STRUCT_ASSIGN))]
    macro_rules! assign_state {
        ($dest:ident, $src:ident) => {
            /*
                    ((dest) = (src))
            */
        }
    }

    #[cfg(NO_STRUCT_ASSIGN)]
    #[cfg(MAX_COMPS_IN_SCAN_EQ_4)]
    macro_rules! assign_state {
        ($dest:ident, $src:ident) => {
            /*
            
                ((dest).EOBRUN = (src).EOBRUN, 
                 (dest).last_dc_val[0] = (src).last_dc_val[0], 
                 (dest).last_dc_val[1] = (src).last_dc_val[1], 
                 (dest).last_dc_val[2] = (src).last_dc_val[2], 
                 (dest).last_dc_val[3] = (src).last_dc_val[3])
            */
        }
    }

    pub struct PHuffEntropyDecoder {

        /**
          | public fields
          |
          */
        pub_: JpegEntropyDecoder,

        /*
          | These fields are loaded into local variables
          | at start of each MCU.
          | 
          | In case of suspension, we exit WITHOUT
          | updating them.
          |
          */

        /**
          | Bit buffer at start of MCU
          |
          */
        bitstate: BitreadPermState,

        /**
          | Other state at start of MCU
          |
          */
        saved:    SavableState3,

        /*
          | These fields are NOT loaded into local
          | working state.
          |
          */

        /**
          | MCUs left in this restart interval
          |
          */
        restarts_to_go: u32,

        /**
          | Pointers to derived tables (these workspaces
          | have image lifespan)
          |
          */
        derived_tbls:   [*mut DDerivedTbl; NUM_HUFF_TBLS],

        /**
          | active table during an AC scan
          |
          */
        ac_derived_tbl: *mut DDerivedTbl,
    }

    pub type PHuffEntropyPtr2 = *mut PHuffEntropyDecoder;

    /**
      | Initialize for a Huffman-compressed
      | scan.
      |
      */
    pub fn start_pass_phuff_decoder(cinfo: JDecompressPtr)  {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          boolean is_DC_band, bad;
          int ci, coefi, tbl;
          int *coef_bit_ptr;
          jpeg_component_info * compptr;

          is_DC_band = (cinfo->Ss == 0);

          /* Validate scan parameters */
          bad = FALSE;
          if (is_DC_band) {
            if (cinfo->Se != 0)
              bad = TRUE;
          } else {
            /* need not check Ss/Se < 0 since they came from unsigned bytes */
            if (cinfo->Ss > cinfo->Se || cinfo->Se >= DCTSIZE2)
              bad = TRUE;
            /* AC scans may have only one component */
            if (cinfo->comps_in_scan != 1)
              bad = TRUE;
          }
          if (cinfo->Ah != 0) {
            /* Successive approximation refinement scan: must have Al = Ah-1. */
            if (cinfo->Al != cinfo->Ah-1)
              bad = TRUE;
          }
          if (cinfo->Al > 13)       /* need not check for < 0 */
            bad = TRUE;
          /* Arguably the maximum Al value should be less than 13 for 8-bit precision,
           * but the spec doesn't say so, and we try to be liberal about what we
           * accept.  Note: large Al values could result in out-of-range DC
           * coefficients during early scans, leading to bizarre displays due to
           * overflows in the IDCT math.  But we won't crash.
           */
          if (bad)
            ERREXIT4(cinfo, JERR_BAD_PROGRESSION,
                 cinfo->Ss, cinfo->Se, cinfo->Ah, cinfo->Al);
          /* Update progression status, and verify that scan order is legal.
           * Note that inter-scan inconsistencies are treated as warnings
           * not fatal errors ... not clear if this is right way to behave.
           */
          for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
            int cindex = cinfo->cur_comp_info[ci]->component_index;
            coef_bit_ptr = & cinfo->coef_bits[cindex][0];
            if (!is_DC_band && coef_bit_ptr[0] < 0) /* AC without prior DC scan */
              WARNMS2(cinfo, JWRN_BOGUS_PROGRESSION, cindex, 0);
            for (coefi = cinfo->Ss; coefi <= cinfo->Se; coefi++) {
              int expected = (coef_bit_ptr[coefi] < 0) ? 0 : coef_bit_ptr[coefi];
              if (cinfo->Ah != expected)
            WARNMS2(cinfo, JWRN_BOGUS_PROGRESSION, cindex, coefi);
              coef_bit_ptr[coefi] = cinfo->Al;
            }
          }

          /* Select MCU decoding routine */
          if (cinfo->Ah == 0) {
            if (is_DC_band)
              entropy->pub.decode_mcu = decode_mcu_DC_first;
            else
              entropy->pub.decode_mcu = decode_mcu_AC_first;
          } else {
            if (is_DC_band)
              entropy->pub.decode_mcu = decode_mcu_DC_refine;
            else
              entropy->pub.decode_mcu = decode_mcu_AC_refine;
          }

          for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
            compptr = cinfo->cur_comp_info[ci];
            /* Make sure requested tables are present, and compute derived tables.
             * We may build same derived table more than once, but it's not expensive.
             */
            if (is_DC_band) {
              if (cinfo->Ah == 0) { /* DC refinement needs no table */
            tbl = compptr->dc_tbl_no;
            jpeg_make_d_derived_tbl(cinfo, TRUE, tbl,
                        & entropy->derived_tbls[tbl]);
              }
            } else {
              tbl = compptr->ac_tbl_no;
              jpeg_make_d_derived_tbl(cinfo, FALSE, tbl,
                          & entropy->derived_tbls[tbl]);
              /* remember the single active table */
              entropy->ac_derived_tbl = entropy->derived_tbls[tbl];
            }
            /* Initialize DC predictions to 0 */
            entropy->saved.last_dc_val[ci] = 0;
          }

          /* Initialize bitread state variables */
          entropy->bitstate.bits_left = 0;
          entropy->bitstate.get_buffer = 0; /* unnecessary, but keeps Purify quiet */
          entropy->pub.insufficient_data = FALSE;

          /* Initialize private state variables */
          entropy->saved.EOBRUN = 0;

          /* Initialize restart counter */
          entropy->restarts_to_go = cinfo->restart_interval;
            */
    }

    /**
      | Check for a restart marker & resynchronize
      | decoder.
      | 
      | Returns FALSE if must suspend.
      |
      */
    pub fn process_restartp(cinfo: JDecompressPtr) -> bool {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          int ci;

          /* Throw away any unused bits remaining in bit buffer; */
          /* include any full bytes in next_marker's count of discarded bytes */
          cinfo->marker->discarded_bytes += entropy->bitstate.bits_left / 8;
          entropy->bitstate.bits_left = 0;

          /* Advance past the RSTn marker */
          if (! (*cinfo->marker->read_restart_marker) (cinfo))
            return FALSE;

          /* Re-initialize DC predictions to 0 */
          for (ci = 0; ci < cinfo->comps_in_scan; ci++)
            entropy->saved.last_dc_val[ci] = 0;
          /* Re-init EOB run count, too */
          entropy->saved.EOBRUN = 0;

          /* Reset restart counter */
          entropy->restarts_to_go = cinfo->restart_interval;

          /* Reset out-of-data flag, unless read_restart_marker left us smack up
           * against a marker.  In that case we will end up treating the next data
           * segment as empty, and we can avoid producing bogus output pixels by
           * leaving the flag set.
           */
          if (cinfo->unread_marker == 0)
            entropy->pub.insufficient_data = FALSE;

          return TRUE;
            */
    }

    /*
      | Huffman MCU decoding.
      | 
      | Each of these routines decodes and returns
      | one MCU's worth of
      | 
      | Huffman-compressed coefficients.
      | 
      | The coefficients are reordered from
      | zigzag order into natural array order,
      | but are not dequantized.
      | 
      | The i'th block of the MCU is stored into
      | the block pointed to by
      | 
      | MCU_data[i]. WE ASSUME THIS AREA IS
      | INITIALLY ZEROED BY THE CALLER.
      | 
      | We return FALSE if data source requested
      | suspension. In that case no changes
      | have been made to permanent state. (Exception:
      | some output coefficients may already
      | have been assigned. This is harmless
      | for spectral selection, since we'll
      | just re-assign them on the next call.
      | 
      | Successive approximation AC refinement
      | has to be more careful, however.)
      |
      */

    /**
      | MCU decoding for DC initial scan (either
      | spectral selection, or first pass of
      | successive approximation).
      |
      */
    pub fn decode_mcu_dc_first(
            cinfo:    JDecompressPtr,
            mcu_data: *mut JBlockRow) -> bool {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          int Al = cinfo->Al;
          int s, r;
          int blkn, ci;
          JBLOCKROW block;
          BITREAD_STATE_VARS;
          savable_state3 state;
          d_derived_tbl * tbl;
          jpeg_component_info * compptr;

          /* Process restart marker if needed; may have to suspend */
          if (cinfo->restart_interval) {
            if (entropy->restarts_to_go == 0)
              if (! process_restartp(cinfo))
            return FALSE;
          }

          /* If we've run out of data, just leave the MCU set to zeroes.
           * This way, we return uniform gray for the remainder of the segment.
           */
          if (! entropy->pub.insufficient_data) {

            /* Load up working state */
            BITREAD_LOAD_STATE(cinfo,entropy->bitstate);
            ASSIGN_STATE(state, entropy->saved);

            /* Outer loop handles each block in the MCU */

            for (blkn = 0; blkn < cinfo->blocks_in_MCU; blkn++) {
              block = MCU_data[blkn];
              ci = cinfo->MCU_membership[blkn];
              compptr = cinfo->cur_comp_info[ci];
              tbl = entropy->derived_tbls[compptr->dc_tbl_no];

              /* Decode a single block's worth of coefficients */

              /* Section F.2.2.1: decode the DC coefficient difference */
              HUFF_DECODE(s, br_state, tbl, return FALSE, label1);
              if (s) {
            CHECK_BIT_BUFFER(br_state, s, return FALSE);
            r = GET_BITS(s);
            s = HUFF_EXTEND(r, s);
              }

              /* Convert DC difference to actual value, update last_dc_val */
              s += state.last_dc_val[ci];
              state.last_dc_val[ci] = s;
              /* Scale and output the coefficient (assumes jpeg_natural_order[0]=0) */
              (*block)[0] = (JCoef) (s << Al);
            }

            /* Completed MCU, so update state */
            BITREAD_SAVE_STATE(cinfo,entropy->bitstate);
            ASSIGN_STATE(entropy->saved, state);
          }

          /* Account for restart interval (no-op if not using restarts) */
          entropy->restarts_to_go--;

          return TRUE;
            */
    }

    /**
      | MCU decoding for AC initial scan (either
      | spectral selection, or first pass of
      | successive approximation).
      |
      */
    pub fn decode_mcu_ac_first(
            cinfo:    JDecompressPtr,
            mcu_data: *mut JBlockRow) -> bool {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          int Se = cinfo->Se;
          int Al = cinfo->Al;
          int s, k, r;
          unsigned int EOBRUN;
          JBLOCKROW block;
          BITREAD_STATE_VARS;
          d_derived_tbl * tbl;

          /* Process restart marker if needed; may have to suspend */
          if (cinfo->restart_interval) {
            if (entropy->restarts_to_go == 0)
              if (! process_restartp(cinfo))
            return FALSE;
          }

          /* If we've run out of data, just leave the MCU set to zeroes.
           * This way, we return uniform gray for the remainder of the segment.
           */
          if (! entropy->pub.insufficient_data) {

            /* Load up working state.
             * We can avoid loading/saving bitread state if in an EOB run.
             */
            EOBRUN = entropy->saved.EOBRUN; /* only part of saved state we need */

            /* There is always only one block per MCU */

            if (EOBRUN > 0)     /* if it's a band of zeroes... */
              EOBRUN--;         /* ...process it now (we do nothing) */
            else {
              BITREAD_LOAD_STATE(cinfo,entropy->bitstate);
              block = MCU_data[0];
              tbl = entropy->ac_derived_tbl;

              for (k = cinfo->Ss; k <= Se; k++) {
            HUFF_DECODE(s, br_state, tbl, return FALSE, label2);
            r = s >> 4;
            s &= 15;
            if (s) {
              k += r;
              CHECK_BIT_BUFFER(br_state, s, return FALSE);
              r = GET_BITS(s);
              s = HUFF_EXTEND(r, s);
              /* Scale and output coefficient in natural (dezigzagged) order */
              (*block)[jpeg_natural_order[k]] = (JCoef) (s << Al);
            } else {
              if (r == 15) {    /* ZRL */
                k += 15;        /* skip 15 zeroes in band */
              } else {      /* EOBr, run length is 2^r + appended bits */
                EOBRUN = 1 << r;
                if (r) {        /* EOBr, r > 0 */
                  CHECK_BIT_BUFFER(br_state, r, return FALSE);
                  r = GET_BITS(r);
                  EOBRUN += r;
                }
                EOBRUN--;       /* this band is processed at this moment */
                break;      /* force end-of-band */
              }
            }
              }

              BITREAD_SAVE_STATE(cinfo,entropy->bitstate);
            }

            /* Completed MCU, so update state */
            entropy->saved.EOBRUN = EOBRUN; /* only part of saved state we need */
          }

          /* Account for restart interval (no-op if not using restarts) */
          entropy->restarts_to_go--;

          return TRUE;
            */
    }

    /**
      | MCU decoding for DC successive approximation
      | refinement scan.
      | 
      | -----------
      | @note
      | 
      | we assume such scans can be multi-component,
      | although the spec is not very clear on
      | the point.
      |
      */
    pub fn decode_mcu_dc_refine(
            cinfo:    JDecompressPtr,
            mcu_data: *mut JBlockRow) -> bool {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          int p1 = 1 << cinfo->Al;  /* 1 in the bit position being coded */
          int blkn;
          JBLOCKROW block;
          BITREAD_STATE_VARS;

          /* Process restart marker if needed; may have to suspend */
          if (cinfo->restart_interval) {
            if (entropy->restarts_to_go == 0)
              if (! process_restartp(cinfo))
            return FALSE;
          }

          /* Not worth the cycles to check insufficient_data here,
           * since we will not change the data anyway if we read zeroes.
           */

          /* Load up working state */
          BITREAD_LOAD_STATE(cinfo,entropy->bitstate);

          /* Outer loop handles each block in the MCU */

          for (blkn = 0; blkn < cinfo->blocks_in_MCU; blkn++) {
            block = MCU_data[blkn];

            /* Encoded data is simply the next bit of the two's-complement DC value */
            CHECK_BIT_BUFFER(br_state, 1, return FALSE);
            if (GET_BITS(1))
              (*block)[0] |= p1;
            /* Note: since we use |=, repeating the assignment later is safe */
          }

          /* Completed MCU, so update state */
          BITREAD_SAVE_STATE(cinfo,entropy->bitstate);

          /* Account for restart interval (no-op if not using restarts) */
          entropy->restarts_to_go--;

          return TRUE;
            */
    }

    /**
      | MCU decoding for AC successive approximation
      | refinement scan.
      |
      */
    pub fn decode_mcu_ac_refine(
            cinfo:    JDecompressPtr,
            mcu_data: *mut JBlockRow) -> bool {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy = (phuff_entropy_ptr2) cinfo->entropy;
          int Se = cinfo->Se;
          int p1 = 1 << cinfo->Al;  /* 1 in the bit position being coded */
          int m1 = (-1) << cinfo->Al;   /* -1 in the bit position being coded */
          int s, k, r;
          unsigned int EOBRUN;
          JBLOCKROW block;
          JCoefPtr thiscoef;
          BITREAD_STATE_VARS;
          d_derived_tbl * tbl;
          int num_newnz;
          int newnz_pos[DCTSIZE2];

          /* Process restart marker if needed; may have to suspend */
          if (cinfo->restart_interval) {
            if (entropy->restarts_to_go == 0)
              if (! process_restartp(cinfo))
            return FALSE;
          }

          /* If we've run out of data, don't modify the MCU.
           */
          if (! entropy->pub.insufficient_data) {

            /* Load up working state */
            BITREAD_LOAD_STATE(cinfo,entropy->bitstate);
            EOBRUN = entropy->saved.EOBRUN; /* only part of saved state we need */

            /* There is always only one block per MCU */
            block = MCU_data[0];
            tbl = entropy->ac_derived_tbl;

            /* If we are forced to suspend, we must undo the assignments to any newly
             * nonzero coefficients in the block, because otherwise we'd get confused
             * next time about which coefficients were already nonzero.
             * But we need not undo addition of bits to already-nonzero coefficients;
             * instead, we can test the current bit to see if we already did it.
             */
            num_newnz = 0;

            /* initialize coefficient loop counter to start of band */
            k = cinfo->Ss;

            if (EOBRUN == 0) {
              for (; k <= Se; k++) {
            HUFF_DECODE(s, br_state, tbl, goto undoit, label3);
            r = s >> 4;
            s &= 15;
            if (s) {
              if (s != 1)       /* size of new coef should always be 1 */
                WARNMS(cinfo, JWRN_HUFF_BAD_CODE);
              CHECK_BIT_BUFFER(br_state, 1, goto undoit);
              if (GET_BITS(1))
                s = p1;     /* newly nonzero coef is positive */
              else
                s = m1;     /* newly nonzero coef is negative */
            } else {
              if (r != 15) {
                EOBRUN = 1 << r;    /* EOBr, run length is 2^r + appended bits */
                if (r) {
                  CHECK_BIT_BUFFER(br_state, r, goto undoit);
                  r = GET_BITS(r);
                  EOBRUN += r;
                }
                break;      /* rest of block is handled by EOB logic */
              }
              /* note s = 0 for processing ZRL */
            }
            /* Advance over already-nonzero coefs and r still-zero coefs,
             * appending correction bits to the nonzeroes.  A correction bit is 1
             * if the absolute value of the coefficient must be increased.
             */
            do {
              thiscoef = *block + jpeg_natural_order[k];
              if (*thiscoef != 0) {
                CHECK_BIT_BUFFER(br_state, 1, goto undoit);
                if (GET_BITS(1)) {
                  if ((*thiscoef & p1) == 0) { /* do nothing if already set it */
                if (*thiscoef >= 0)
                  *thiscoef += p1;
                else
                  *thiscoef += m1;
                  }
                }
              } else {
                if (--r < 0)
                  break;        /* reached target zero coefficient */
              }
              k++;
            } while (k <= Se);
            if (s) {
              int pos = jpeg_natural_order[k];
              /* Output newly nonzero coefficient */
              (*block)[pos] = (JCoef) s;
              /* Remember its position in case we have to suspend */
              newnz_pos[num_newnz++] = pos;
            }
              }
            }

            if (EOBRUN > 0) {
              /* Scan any remaining coefficient positions after the end-of-band
               * (the last newly nonzero coefficient, if any).  Append a correction
               * bit to each already-nonzero coefficient.  A correction bit is 1
               * if the absolute value of the coefficient must be increased.
               */
              for (; k <= Se; k++) {
            thiscoef = *block + jpeg_natural_order[k];
            if (*thiscoef != 0) {
              CHECK_BIT_BUFFER(br_state, 1, goto undoit);
              if (GET_BITS(1)) {
                if ((*thiscoef & p1) == 0) { /* do nothing if already changed it */
                  if (*thiscoef >= 0)
                *thiscoef += p1;
                  else
                *thiscoef += m1;
                }
              }
            }
              }
              /* Count one block completed in EOB run */
              EOBRUN--;
            }

            /* Completed MCU, so update state */
            BITREAD_SAVE_STATE(cinfo,entropy->bitstate);
            entropy->saved.EOBRUN = EOBRUN; /* only part of saved state we need */
          }

          /* Account for restart interval (no-op if not using restarts) */
          entropy->restarts_to_go--;

          return TRUE;

        undoit:
          /* Re-zero any output coefficients that we made newly nonzero */
          while (num_newnz > 0)
            (*block)[newnz_pos[--num_newnz]] = 0;

          return FALSE;
            */
    }

    /**
      | Module initialization routine for
      | progressive Huffman entropy decoding.
      |
      */
    pub fn jinit_phuff_decoder(cinfo: JDecompressPtr)  {
        
        todo!();
            /*
                phuff_entropy_ptr2 entropy;
          int *coef_bit_ptr;
          int ci, i;

          entropy = (phuff_entropy_ptr2)
            (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                        SIZEOF(phuff_entropy_decoder));
          cinfo->entropy = (struct jpeg_entropy_decoder *) entropy;
          entropy->pub.start_pass = start_pass_phuff_decoder;

          /* Mark derived tables unallocated */
          for (i = 0; i < NUM_HUFF_TBLS; i++) {
            entropy->derived_tbls[i] = NULL;
          }

          /* Create progression status table */
          cinfo->coef_bits = (int (*)[DCTSIZE2])
            (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                        cinfo->num_components*DCTSIZE2*SIZEOF(int));
          coef_bit_ptr = & cinfo->coef_bits[0][0];
          for (ci = 0; ci < cinfo->num_components; ci++)
            for (i = 0; i < DCTSIZE2; i++)
              *coef_bit_ptr++ = -1;
            */
    }
}
