/*!
  | jchuff.h
  | 
  | This file contains declarations for
  | Huffman entropy encoding routines
  | that are shared between the sequential
  | encoder (jchuff.c) and the progressive
  | encoder (jcphuff.c). No other modules
  | need to see these.
  |
  -------------------------
  | jchuff.c
  | 
  | This file contains Huffman entropy
  | encoding routines.
  | 
  | Much of the complexity here has to do
  | with supporting output suspension.
  | 
  | If the data destination module demands
  | suspension, we want to be able to back
  | up to the start of the current MCU. To
  | do this, we copy state variables into
  | local working storage, and update them
  | back to the permanent JPEG objects only
  | upon successful completion of an MCU.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jchuff.h]

/**
  | The legal range of a DCT coefficient
  | is
  | 
  | -1024 .. +1023 for 8-bit data;
  | 
  | -16384 .. +16383 for 12-bit data.
  | 
  | Hence the magnitude should always fit
  | in 10 or 14 bits respectively.
  |
  */
#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub const MAX_COEF_BITS: usize = 10;

#[cfg(not(BITS_IN_JSAMPLE_EQ_8))]
pub const MAX_COEF_BITS: usize = 14;

/**
  | Derived data constructed for each Huffman
  | table
  |
  */
pub struct CDerivedTbl {

    /**
      | code for each symbol
      |
      */
    ehufco: [u32; 256],


    /**
      | length of code for each symbol
      |
      */
    ehufsi: [u8; 256],


  /* If no code has been allocated for a symbol S, ehufsi[S] contains 0 */
}

/**
  | Short forms of external names for systems
  | with brain-damaged linkers.
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_make_c_derived_tbl {
    () => {
        /*
                jMkCDerived
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_gen_optimal_table {
    () => {
        /*
                jGenOptTbl
        */
    }
}

/**
  | Expand a Huffman table definition into
  | the derived format
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jpeg_make_c_derived_tbl
        JPP((JCompressPtr cinfo, boolean isDC, int tblno,
             c_derived_tbl ** pdtbl));
    */
}

/**
  | Generate an optimal table definition
  | given the specified counts
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jpeg_gen_optimal_table
        JPP((JCompressPtr cinfo, JHUFF_TBL * htbl, long freq[]));
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jchuff.c]

pub const JPEG_INTERNALS: bool = true;

/**
  | Expanded entropy encoder object for
  | Huffman encoding.
  | 
  | The savable_state subrecord contains
  | fields that change within an MCU, but
  | must not be updated permanently until
  | we complete the MCU.
  |
  */
pub struct SavableState {

    /**
      | current bit-accumulation buffer
      |
      */
    put_buffer:  i32,


    /**
      | # of bits now in it
      |
      */
    put_bits:    i32,


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
        
            ((dest).put_buffer = (src).put_buffer, 
             (dest).put_bits = (src).put_bits, 
             (dest).last_dc_val[0] = (src).last_dc_val[0], 
             (dest).last_dc_val[1] = (src).last_dc_val[1], 
             (dest).last_dc_val[2] = (src).last_dc_val[2], 
             (dest).last_dc_val[3] = (src).last_dc_val[3])
        */
    }
}

///---------------------------
pub struct HuffEntropyEncoder {

    /**
      | public fields
      |
      */
    pub_:  JpegEntropyEncoder,

    /**
      | Bit buffer & DC state at start of MCU
      |
      */
    saved: SavableState,

    /* These fields are NOT loaded into local working state. */

    /**
      | MCUs left in this restart interval
      |
      */
    restarts_to_go:   u32,

    /**
      | next restart number to write (0-7)
      |
      */
    next_restart_num: i32,

    /**
      | Pointers to derived tables (these workspaces
      | have image lifespan)
      |
      */
    dc_derived_tbls: [*mut CDerivedTbl; NUM_HUFF_TBLS],
    ac_derived_tbls: [*mut CDerivedTbl; NUM_HUFF_TBLS],

    /**
      | Statistics tables for optimization
      |
      */
    #[cfg(ENTROPY_OPT_SUPPORTED)]
    dc_count_ptrs: [*mut i64; NUM_HUFF_TBLS],

    #[cfg(ENTROPY_OPT_SUPPORTED)]
    ac_count_ptrs: [*mut i64; NUM_HUFF_TBLS],
}

pub type HuffEntropyPtr = *mut HuffEntropyEncoder;

/**
  | Working state while writing an MCU.
  | 
  | This struct contains all the fields
  | that are needed by subroutines.
  |
  */
pub struct WorkingState {

    /**
      | => next byte to write in buffer
      |
      */
    next_output_byte: *mut JOctet,


    /**
      | # of byte spaces remaining in buffer
      |
      */
    free_in_buffer:   usize,


    /**
      | Current bit buffer & DC state
      |
      */
    cur:              SavableState,


    /**
      | dump_buffer needs access to this
      |
      */
    cinfo:            JCompressPtr,
}

/**
  | Initialize for a Huffman-compressed
  | scan.
  | 
  | If gather_statistics is TRUE, we do
  | not output anything during the scan,
  | just count the Huffman symbols used
  | and generate Huffman code tables.
  |
  */
pub fn start_pass_huff(
        cinfo:             JCompressPtr,
        gather_statistics: bool)  {
    
    todo!();
        /*
            huff_entropy_ptr entropy = (huff_entropy_ptr) cinfo->entropy;
      int ci, dctbl, actbl;
      jpeg_component_info * compptr;

      if (gather_statistics) {
    #ifdef ENTROPY_OPT_SUPPORTED
        entropy->pub.encode_mcu = encode_mcu_gather;
        entropy->pub.finish_pass = finish_pass_gather;
    #else
        ERREXIT(cinfo, JERR_NOT_COMPILED);
    #endif
      } else {
        entropy->pub.encode_mcu = encode_mcu_huff;
        entropy->pub.finish_pass = finish_pass_huff;
      }

      for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
        compptr = cinfo->cur_comp_info[ci];
        dctbl = compptr->dc_tbl_no;
        actbl = compptr->ac_tbl_no;
        if (gather_statistics) {
    #ifdef ENTROPY_OPT_SUPPORTED
          /* Check for invalid table indexes */
          /* (make_c_derived_tbl does this in the other path) */
          if (dctbl < 0 || dctbl >= NUM_HUFF_TBLS)
        ERREXIT1(cinfo, JERR_NO_HUFF_TABLE, dctbl);
          if (actbl < 0 || actbl >= NUM_HUFF_TBLS)
        ERREXIT1(cinfo, JERR_NO_HUFF_TABLE, actbl);
          /* Allocate and zero the statistics tables */
          /* Note that jpeg_gen_optimal_table expects 257 entries in each table! */
          if (entropy->dc_count_ptrs[dctbl] == NULL)
        entropy->dc_count_ptrs[dctbl] = (long *)
          (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                          257 * SIZEOF(long));
          MEMZERO(entropy->dc_count_ptrs[dctbl], 257 * SIZEOF(long));
          if (entropy->ac_count_ptrs[actbl] == NULL)
        entropy->ac_count_ptrs[actbl] = (long *)
          (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                          257 * SIZEOF(long));
          MEMZERO(entropy->ac_count_ptrs[actbl], 257 * SIZEOF(long));
    #endif
        } else {
          /* Compute derived values for Huffman tables */
          /* We may do this more than once for a table, but it's not expensive */
          jpeg_make_c_derived_tbl(cinfo, TRUE, dctbl,
                      & entropy->dc_derived_tbls[dctbl]);
          jpeg_make_c_derived_tbl(cinfo, FALSE, actbl,
                      & entropy->ac_derived_tbls[actbl]);
        }
        /* Initialize DC predictions to 0 */
        entropy->saved.last_dc_val[ci] = 0;
      }

      /* Initialize bit buffer to empty */
      entropy->saved.put_buffer = 0;
      entropy->saved.put_bits = 0;

      /* Initialize restart stuff */
      entropy->restarts_to_go = cinfo->restart_interval;
      entropy->next_restart_num = 0;
        */
}

/**
  | Compute the derived values for a Huffman
  | table.
  | 
  | This routine also performs some validation
  | checks on the table.
  | 
  | Note this is also used by jcphuff.c.
  |
  */
pub fn jpeg_make_c_derived_tbl(
        cinfo: JCompressPtr,
        isdc:  bool,
        tblno: i32,
        pdtbl: *mut *mut CDerivedTbl)  {
    
    todo!();
        /*
            JHUFF_TBL *htbl;
      c_derived_tbl *dtbl;
      int p, i, l, lastp, si, maxsymbol;
      char huffsize[257];
      unsigned int huffcode[257];
      unsigned int code;

      /* Note that huffsize[] and huffcode[] are filled in code-length order,
       * paralleling the order of the symbols themselves in htbl->huffval[].
       */

      /* Find the input Huffman table */
      if (tblno < 0 || tblno >= NUM_HUFF_TBLS)
        ERREXIT1(cinfo, JERR_NO_HUFF_TABLE, tblno);
      htbl =
        isDC ? cinfo->dc_huff_tbl_ptrs[tblno] : cinfo->ac_huff_tbl_ptrs[tblno];
      if (htbl == NULL)
        ERREXIT1(cinfo, JERR_NO_HUFF_TABLE, tblno);

      /* Allocate a workspace if we haven't already done so. */
      if (*pdtbl == NULL)
        *pdtbl = (c_derived_tbl *)
          (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                      SIZEOF(c_derived_tbl));
      dtbl = *pdtbl;

      /* Figure C.1: make table of Huffman code length for each symbol */

      p = 0;
      for (l = 1; l <= 16; l++) {
        i = (int) htbl->bits[l];
        if (i < 0 || p + i > 256)   /* protect against table overrun */
          ERREXIT(cinfo, JERR_BAD_HUFF_TABLE);
        while (i--)
          huffsize[p++] = (char) l;
      }
      huffsize[p] = 0;
      lastp = p;

      /* Figure C.2: generate the codes themselves */
      /* We also validate that the counts represent a legal Huffman code tree. */

      code = 0;
      si = huffsize[0];
      p = 0;
      while (huffsize[p]) {
        while (((int) huffsize[p]) == si) {
          huffcode[p++] = code;
          code++;
        }
        /* code is now 1 more than the last code used for codelength si; but
         * it must still fit in si bits, since no code is allowed to be all ones.
         */
        if (((i32) code) >= (((i32) 1) << si))
          ERREXIT(cinfo, JERR_BAD_HUFF_TABLE);
        code <<= 1;
        si++;
      }

      /* Figure C.3: generate encoding tables */
      /* These are code and size indexed by symbol value */

      /* Set all codeless symbols to have code length 0;
       * this lets us detect duplicate VAL entries here, and later
       * allows emit_bits to detect any attempt to emit such symbols.
       */
      MEMZERO(dtbl->ehufsi, SIZEOF(dtbl->ehufsi));

      /* This is also a convenient place to check for out-of-range
       * and duplicated VAL entries.  We allow 0..255 for AC symbols
       * but only 0..15 for DC.  (We could constrain them further
       * based on data depth and mode, but this seems enough.)
       */
      maxsymbol = isDC ? 15 : 255;

      for (p = 0; p < lastp; p++) {
        i = htbl->huffval[p];
        if (i < 0 || i > maxsymbol || dtbl->ehufsi[i])
          ERREXIT(cinfo, JERR_BAD_HUFF_TABLE);
        dtbl->ehufco[i] = huffcode[p];
        dtbl->ehufsi[i] = huffsize[p];
      }
        */
}

/* ---------- Outputting bytes to the file  ---------- */

/**
  | Emit a byte, taking 'action' if must
  | suspend.
  |
  */
macro_rules! emit_byte {
    ($state:ident, 
     $val:ident, 
     $action:ident) => {
        /*
        
            { *(state)->next_output_byte++ = (JOctet) (val);  
              if (--(state)->free_in_buffer == 0)  
                if (! dump_buffer(state))  
                  { action; } }
        */
    }
}

/**
  | Empty the output buffer; return TRUE
  | if successful, FALSE if must suspend
  |
  */
pub fn dump_buffer(state: *mut WorkingState) -> bool {
    
    todo!();
        /*
            struct jpeg_destination_mgr * dest = state->cinfo->dest;

      if (! (*dest->empty_output_buffer) (state->cinfo))
        return FALSE;
      /* After a successful buffer dump, must reset buffer pointers */
      state->next_output_byte = dest->next_output_byte;
      state->free_in_buffer = dest->free_in_buffer;
      return TRUE;
        */
}

/* ----------- Outputting bits to the file  ----------- */

/**
  | Only the right 24 bits of put_buffer
  | are used; the valid bits are left-justified
  | in this part. At most 16 bits can be passed
  | to emit_bits in one call, and we never
  | retain more than 7 bits in put_buffer
  | between calls, so 24 bits are sufficient.
  |
  | Emit some bits; return TRUE if successful,
  | FALSE if must suspend
  |
  */
#[inline] pub fn emit_bits(
        state: *mut WorkingState,
        code:  u32,
        size:  i32) -> bool {
    
    todo!();
        /*

      /* This routine is heavily used, so it's worth coding tightly. */
      i32 put_buffer = (i32) code;
      int put_bits = state->cur.put_bits;

      /* if size is 0, caller used an invalid Huffman table entry */
      if (size == 0)
        ERREXIT(state->cinfo, JERR_HUFF_MISSING_CODE);

      put_buffer &= (((i32) 1)<<size) - 1; /* mask off any extra bits in code */

      put_bits += size;     /* new number of bits in buffer */

      put_buffer <<= 24 - put_bits; /* align incoming bits */

      put_buffer |= state->cur.put_buffer; /* and merge with old buffer contents */

      while (put_bits >= 8) {
        int c = (int) ((put_buffer >> 16) & 0xFF);

        emit_byte(state, c, return FALSE);
        if (c == 0xFF) {        /* need to stuff a zero byte? */
          emit_byte(state, 0, return FALSE);
        }
        put_buffer <<= 8;
        put_bits -= 8;
      }

      state->cur.put_buffer = put_buffer; /* update state variables */
      state->cur.put_bits = put_bits;

      return TRUE;
        */
}

pub fn flush_bits(state: *mut WorkingState) -> bool {
    
    todo!();
        /*
            if (! emit_bits(state, 0x7F, 7)) /* fill any partial byte with ones */
        return FALSE;
      state->cur.put_buffer = 0;    /* and reset bit-buffer to empty */
      state->cur.put_bits = 0;
      return TRUE;
        */
}

/**
  | Encode a single block's worth of coefficients
  |
  */
pub fn encode_one_block(
        state:       *mut WorkingState,
        block:       JCoefPtr,
        last_dc_val: i32,
        dctbl:       *mut CDerivedTbl,
        actbl:       *mut CDerivedTbl) -> bool {
    
    todo!();
        /*
            int temp, temp2;
      int nbits;
      int k, r, i;

      /* Encode the DC coefficient difference per section F.1.2.1 */

      temp = temp2 = block[0] - last_dc_val;

      if (temp < 0) {
        temp = -temp;       /* temp is abs value of input */
        /* For a negative input, want temp2 = bitwise complement of abs(input) */
        /* This code assumes we are on a two's complement machine */
        temp2--;
      }

      /* Find the number of bits needed for the magnitude of the coefficient */
      nbits = 0;
      while (temp) {
        nbits++;
        temp >>= 1;
      }
      /* Check for out-of-range coefficient values.
       * Since we're encoding a difference, the range limit is twice as much.
       */
      if (nbits > MAX_COEF_BITS+1)
        ERREXIT(state->cinfo, JERR_BAD_DCT_COEF);

      /* Emit the Huffman-coded symbol for the number of bits */
      if (! emit_bits(state, dctbl->ehufco[nbits], dctbl->ehufsi[nbits]))
        return FALSE;

      /* Emit that number of bits of the value, if positive, */
      /* or the complement of its magnitude, if negative. */
      if (nbits)            /* emit_bits rejects calls with size 0 */
        if (! emit_bits(state, (unsigned int) temp2, nbits))
          return FALSE;

      /* Encode the AC coefficients per section F.1.2.2 */

      r = 0;            /* r = run length of zeros */

      for (k = 1; k < DCTSIZE2; k++) {
        if ((temp = block[jpeg_natural_order[k]]) == 0) {
          r++;
        } else {
          /* if run length > 15, must emit special run-length-16 codes (0xF0) */
          while (r > 15) {
        if (! emit_bits(state, actbl->ehufco[0xF0], actbl->ehufsi[0xF0]))
          return FALSE;
        r -= 16;
          }

          temp2 = temp;
          if (temp < 0) {
        temp = -temp;       /* temp is abs value of input */
        /* This code assumes we are on a two's complement machine */
        temp2--;
          }

          /* Find the number of bits needed for the magnitude of the coefficient */
          nbits = 1;        /* there must be at least one 1 bit */
          while ((temp >>= 1))
        nbits++;
          /* Check for out-of-range coefficient values */
          if (nbits > MAX_COEF_BITS)
        ERREXIT(state->cinfo, JERR_BAD_DCT_COEF);

          /* Emit Huffman symbol for run length / number of bits */
          i = (r << 4) + nbits;
          if (! emit_bits(state, actbl->ehufco[i], actbl->ehufsi[i]))
        return FALSE;

          /* Emit that number of bits of the value, if positive, */
          /* or the complement of its magnitude, if negative. */
          if (! emit_bits(state, (unsigned int) temp2, nbits))
        return FALSE;

          r = 0;
        }
      }

      /* If the last coef(s) were zero, emit an end-of-block code */
      if (r > 0)
        if (! emit_bits(state, actbl->ehufco[0], actbl->ehufsi[0]))
          return FALSE;

      return TRUE;
        */
}

/**
  | Emit a restart marker & resynchronize
  | predictions.
  |
  */
pub fn emit_restart(
        state:       *mut WorkingState,
        restart_num: i32) -> bool {
    
    todo!();
        /*
            int ci;

      if (! flush_bits(state))
        return FALSE;

      emit_byte(state, 0xFF, return FALSE);
      emit_byte(state, JPEG_RST0 + restart_num, return FALSE);

      /* Re-initialize DC predictions to 0 */
      for (ci = 0; ci < state->cinfo->comps_in_scan; ci++)
        state->cur.last_dc_val[ci] = 0;

      /* The restart counter is not updated until we successfully write the MCU. */

      return TRUE;
        */
}

/**
  | Encode and output one MCU's worth of
  | Huffman-compressed coefficients.
  |
  */
pub fn encode_mcu_huff(
        cinfo:    JCompressPtr,
        mcu_data: *mut JBlockRow) -> bool {
    
    todo!();
        /*
            huff_entropy_ptr entropy = (huff_entropy_ptr) cinfo->entropy;
      working_state state;
      int blkn, ci;
      jpeg_component_info * compptr;

      /* Load up working state */
      state.next_output_byte = cinfo->dest->next_output_byte;
      state.free_in_buffer = cinfo->dest->free_in_buffer;
      ASSIGN_STATE(state.cur, entropy->saved);
      state.cinfo = cinfo;

      /* Emit restart marker if needed */
      if (cinfo->restart_interval) {
        if (entropy->restarts_to_go == 0)
          if (! emit_restart(&state, entropy->next_restart_num))
        return FALSE;
      }

      /* Encode the MCU data blocks */
      for (blkn = 0; blkn < cinfo->blocks_in_MCU; blkn++) {
        ci = cinfo->MCU_membership[blkn];
        compptr = cinfo->cur_comp_info[ci];
        if (! encode_one_block(&state,
                   MCU_data[blkn][0], state.cur.last_dc_val[ci],
                   entropy->dc_derived_tbls[compptr->dc_tbl_no],
                   entropy->ac_derived_tbls[compptr->ac_tbl_no]))
          return FALSE;
        /* Update last_dc_val */
        state.cur.last_dc_val[ci] = MCU_data[blkn][0][0];
      }

      /* Completed MCU, so update state */
      cinfo->dest->next_output_byte = state.next_output_byte;
      cinfo->dest->free_in_buffer = state.free_in_buffer;
      ASSIGN_STATE(entropy->saved, state.cur);

      /* Update restart-interval state too */
      if (cinfo->restart_interval) {
        if (entropy->restarts_to_go == 0) {
          entropy->restarts_to_go = cinfo->restart_interval;
          entropy->next_restart_num++;
          entropy->next_restart_num &= 7;
        }
        entropy->restarts_to_go--;
      }

      return TRUE;
        */
}

/**
  | Finish up at the end of a Huffman-compressed
  | scan.
  |
  */
pub fn finish_pass_huff(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            huff_entropy_ptr entropy = (huff_entropy_ptr) cinfo->entropy;
      working_state state;

      /* Load up working state ... flush_bits needs it */
      state.next_output_byte = cinfo->dest->next_output_byte;
      state.free_in_buffer = cinfo->dest->free_in_buffer;
      ASSIGN_STATE(state.cur, entropy->saved);
      state.cinfo = cinfo;

      /* Flush out the last data */
      if (! flush_bits(&state))
        ERREXIT(cinfo, JERR_CANT_SUSPEND);

      /* Update state */
      cinfo->dest->next_output_byte = state.next_output_byte;
      cinfo->dest->free_in_buffer = state.free_in_buffer;
      ASSIGN_STATE(entropy->saved, state.cur);
        */
}

/*
  | Huffman coding optimization.
  | 
  | We first scan the supplied data and count
  | the number of uses of each symbol that
  | is to be Huffman-coded. (This process
  | MUST agree with the code above.)
  | 
  | Then we build a Huffman coding tree for
  | the observed counts.
  | 
  | Symbols which are not needed at all for
  | the particular image are not assigned
  | any code, which saves space in the DHT
  | marker as well as in the compressed data.
  |
  */

/**
  | Process a single block's worth of coefficients
  |
  */
#[cfg(ENTROPY_OPT_SUPPORTED)]
pub fn htest_one_block(
        cinfo:       JCompressPtr,
        block:       JCoefPtr,
        last_dc_val: i32,
        dc_counts:   &[i64],
        ac_counts:   &[i64])  {
    
    todo!();
        /*
            int temp;
      int nbits;
      int k, r;

      /* Encode the DC coefficient difference per section F.1.2.1 */

      temp = block[0] - last_dc_val;
      if (temp < 0)
        temp = -temp;

      /* Find the number of bits needed for the magnitude of the coefficient */
      nbits = 0;
      while (temp) {
        nbits++;
        temp >>= 1;
      }
      /* Check for out-of-range coefficient values.
       * Since we're encoding a difference, the range limit is twice as much.
       */
      if (nbits > MAX_COEF_BITS+1)
        ERREXIT(cinfo, JERR_BAD_DCT_COEF);

      /* Count the Huffman symbol for the number of bits */
      dc_counts[nbits]++;

      /* Encode the AC coefficients per section F.1.2.2 */

      r = 0;            /* r = run length of zeros */

      for (k = 1; k < DCTSIZE2; k++) {
        if ((temp = block[jpeg_natural_order[k]]) == 0) {
          r++;
        } else {
          /* if run length > 15, must emit special run-length-16 codes (0xF0) */
          while (r > 15) {
        ac_counts[0xF0]++;
        r -= 16;
          }

          /* Find the number of bits needed for the magnitude of the coefficient */
          if (temp < 0)
        temp = -temp;

          /* Find the number of bits needed for the magnitude of the coefficient */
          nbits = 1;        /* there must be at least one 1 bit */
          while ((temp >>= 1))
        nbits++;
          /* Check for out-of-range coefficient values */
          if (nbits > MAX_COEF_BITS)
        ERREXIT(cinfo, JERR_BAD_DCT_COEF);

          /* Count Huffman symbol for run length / number of bits */
          ac_counts[(r << 4) + nbits]++;

          r = 0;
        }
      }

      /* If the last coef(s) were zero, emit an end-of-block code */
      if (r > 0)
        ac_counts[0]++;
        */
}

/**
  | Trial-encode one MCU's worth of Huffman-compressed
  | coefficients.
  | 
  | No data is actually output, so no suspension
  | return is possible.
  |
  */
#[cfg(ENTROPY_OPT_SUPPORTED)]
pub fn encode_mcu_gather(
        cinfo:    JCompressPtr,
        mcu_data: *mut JBlockRow) -> bool {
    
    todo!();
        /*
            huff_entropy_ptr entropy = (huff_entropy_ptr) cinfo->entropy;
      int blkn, ci;
      jpeg_component_info * compptr;

      /* Take care of restart intervals if needed */
      if (cinfo->restart_interval) {
        if (entropy->restarts_to_go == 0) {
          /* Re-initialize DC predictions to 0 */
          for (ci = 0; ci < cinfo->comps_in_scan; ci++)
        entropy->saved.last_dc_val[ci] = 0;
          /* Update restart state */
          entropy->restarts_to_go = cinfo->restart_interval;
        }
        entropy->restarts_to_go--;
      }

      for (blkn = 0; blkn < cinfo->blocks_in_MCU; blkn++) {
        ci = cinfo->MCU_membership[blkn];
        compptr = cinfo->cur_comp_info[ci];
        htest_one_block(cinfo, MCU_data[blkn][0], entropy->saved.last_dc_val[ci],
                entropy->dc_count_ptrs[compptr->dc_tbl_no],
                entropy->ac_count_ptrs[compptr->ac_tbl_no]);
        entropy->saved.last_dc_val[ci] = MCU_data[blkn][0][0];
      }

      return TRUE;
        */
}

/**
  | Generate the best Huffman code table
  | for the given counts, fill htbl.
  | 
  | Note this is also used by jcphuff.c.
  | 
  | The JPEG standard requires that no symbol
  | be assigned a codeword of all one bits
  | (so that padding bits added at the end
  | of a compressed segment can't look like
  | a valid code). Because of the canonical
  | ordering of codewords, this just means
  | that there must be an unused slot in the
  | longest codeword length category.
  | Section K.2 of the JPEG spec suggests
  | reserving such a slot by pretending
  | that symbol 256 is a valid symbol with
  | count 1. In theory that's not optimal;
  | giving it count zero but including it
  | in the symbol set anyway should give
  | a better Huffman code.
  | 
  | But the theoretically better code actually
  | seems to come out worse in practice,
  | because it produces more all-ones bytes
  | (which incur stuffed zero bytes in the
  | final file). In any case the difference
  | is tiny.
  | 
  | The JPEG standard requires Huffman
  | codes to be no more than 16 bits long.
  | 
  | If some symbols have a very small but
  | nonzero probability, the Huffman tree
  | must be adjusted to meet the code length
  | restriction. We currently use the adjustment
  | method suggested in JPEG section K.2.
  | This method is *not* optimal; it may
  | not choose the best possible limited-length
  | code. But typically only very-low-frequency
  | symbols will be given less-than-optimal
  | lengths, so the code is almost optimal.
  | Experimental comparisons against
  | an optimal limited-length-code algorithm
  | indicate that the difference is microscopic
  | --- usually less than a hundredth of
  | a percent of total size.
  | 
  | So the extra complexity of an optimal
  | algorithm doesn't seem worthwhile.
  |
  */
#[cfg(ENTROPY_OPT_SUPPORTED)]
pub fn jpeg_gen_optimal_table(
        cinfo: JCompressPtr,
        htbl:  *mut JHuffTbl,
        freq:  &[i64])  {
    
    todo!();
        /*
            #define MAX_CLEN 32     /* assumed maximum initial code length */
      UINT8 bits[MAX_CLEN+1];   /* bits[k] = # of symbols with code length k */
      int codesize[257];        /* codesize[k] = code length of symbol k */
      int others[257];      /* next symbol in current branch of tree */
      int c1, c2;
      int p, i, j;
      long v;

      /* This algorithm is explained in section K.2 of the JPEG standard */

      MEMZERO(bits, SIZEOF(bits));
      MEMZERO(codesize, SIZEOF(codesize));
      for (i = 0; i < 257; i++)
        others[i] = -1;     /* init links to empty */

      freq[256] = 1;        /* make sure 256 has a nonzero count */
      /* Including the pseudo-symbol 256 in the Huffman procedure guarantees
       * that no real symbol is given code-value of all ones, because 256
       * will be placed last in the largest codeword category.
       */

      /* Huffman's basic algorithm to assign optimal code lengths to symbols */

      for (;;) {
        /* Find the smallest nonzero frequency, set c1 = its symbol */
        /* In case of ties, take the larger symbol number */
        c1 = -1;
        v = 1000000000L;
        for (i = 0; i <= 256; i++) {
          if (freq[i] && freq[i] <= v) {
        v = freq[i];
        c1 = i;
          }
        }

        /* Find the next smallest nonzero frequency, set c2 = its symbol */
        /* In case of ties, take the larger symbol number */
        c2 = -1;
        v = 1000000000L;
        for (i = 0; i <= 256; i++) {
          if (freq[i] && freq[i] <= v && i != c1) {
        v = freq[i];
        c2 = i;
          }
        }

        /* Done if we've merged everything into one frequency */
        if (c2 < 0)
          break;

        /* Else merge the two counts/trees */
        freq[c1] += freq[c2];
        freq[c2] = 0;

        /* Increment the codesize of everything in c1's tree branch */
        codesize[c1]++;
        while (others[c1] >= 0) {
          c1 = others[c1];
          codesize[c1]++;
        }

        others[c1] = c2;        /* chain c2 onto c1's tree branch */

        /* Increment the codesize of everything in c2's tree branch */
        codesize[c2]++;
        while (others[c2] >= 0) {
          c2 = others[c2];
          codesize[c2]++;
        }
      }

      /* Now count the number of symbols of each code length */
      for (i = 0; i <= 256; i++) {
        if (codesize[i]) {
          /* The JPEG standard seems to think that this can't happen, */
          /* but I'm paranoid... */
          if (codesize[i] > MAX_CLEN)
        ERREXIT(cinfo, JERR_HUFF_CLEN_OVERFLOW);

          bits[codesize[i]]++;
        }
      }

      /* JPEG doesn't allow symbols with code lengths over 16 bits, so if the pure
       * Huffman procedure assigned any such lengths, we must adjust the coding.
       * Here is what the JPEG spec says about how this next bit works:
       * Since symbols are paired for the longest Huffman code, the symbols are
       * removed from this length category two at a time.  The prefix for the pair
       * (which is one bit shorter) is allocated to one of the pair; then,
       * skipping the BITS entry for that prefix length, a code word from the next
       * shortest nonzero BITS entry is converted into a prefix for two code words
       * one bit longer.
       */

      for (i = MAX_CLEN; i > 16; i--) {
        while (bits[i] > 0) {
          j = i - 2;        /* find length of new prefix to be used */
          while (bits[j] == 0)
        j--;

          bits[i] -= 2;     /* remove two symbols */
          bits[i-1]++;      /* one goes in this length */
          bits[j+1] += 2;       /* two new symbols in this length */
          bits[j]--;        /* symbol of this length is now a prefix */
        }
      }

      /* Remove the count for the pseudo-symbol 256 from the largest codelength */
      while (bits[i] == 0)      /* find largest codelength still in use */
        i--;
      bits[i]--;

      /* Return final symbol counts (only for lengths 0..16) */
      MEMCOPY(htbl->bits, bits, SIZEOF(htbl->bits));

      /* Return a list of the symbols sorted by code length */
      /* It's not real clear to me why we don't need to consider the codelength
       * changes made above, but the JPEG spec seems to think this works.
       */
      p = 0;
      for (i = 1; i <= MAX_CLEN; i++) {
        for (j = 0; j <= 255; j++) {
          if (codesize[j] == i) {
        htbl->huffval[p] = (UINT8) j;
        p++;
          }
        }
      }

      /* Set sent_table FALSE so updated table will be written to JPEG file. */
      htbl->sent_table = FALSE;
        */
}

/**
  | Finish up a statistics-gathering pass
  | and create the new Huffman tables.
  |
  */
#[cfg(ENTROPY_OPT_SUPPORTED)]
pub fn finish_pass_gather(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            huff_entropy_ptr entropy = (huff_entropy_ptr) cinfo->entropy;
      int ci, dctbl, actbl;
      jpeg_component_info * compptr;
      JHUFF_TBL **htblptr;
      boolean did_dc[NUM_HUFF_TBLS];
      boolean did_ac[NUM_HUFF_TBLS];

      /* It's important not to apply jpeg_gen_optimal_table more than once
       * per table, because it clobbers the input frequency counts!
       */
      MEMZERO(did_dc, SIZEOF(did_dc));
      MEMZERO(did_ac, SIZEOF(did_ac));

      for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
        compptr = cinfo->cur_comp_info[ci];
        dctbl = compptr->dc_tbl_no;
        actbl = compptr->ac_tbl_no;
        if (! did_dc[dctbl]) {
          htblptr = & cinfo->dc_huff_tbl_ptrs[dctbl];
          if (*htblptr == NULL)
        *htblptr = jpeg_alloc_huff_table((JCommonPtr) cinfo);
          jpeg_gen_optimal_table(cinfo, *htblptr, entropy->dc_count_ptrs[dctbl]);
          did_dc[dctbl] = TRUE;
        }
        if (! did_ac[actbl]) {
          htblptr = & cinfo->ac_huff_tbl_ptrs[actbl];
          if (*htblptr == NULL)
        *htblptr = jpeg_alloc_huff_table((JCommonPtr) cinfo);
          jpeg_gen_optimal_table(cinfo, *htblptr, entropy->ac_count_ptrs[actbl]);
          did_ac[actbl] = TRUE;
        }
      }
        */
}

/**
  | Module initialization routine for
  | Huffman entropy encoding.
  |
  */
pub fn jinit_huff_encoder(cinfo: JCompressPtr)  {
    
    todo!();
        /*
            huff_entropy_ptr entropy;
      int i;

      entropy = (huff_entropy_ptr)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(huff_entropy_encoder));
      cinfo->entropy = (struct jpeg_entropy_encoder *) entropy;
      entropy->pub.start_pass = start_pass_huff;

      /* Mark tables unallocated */
      for (i = 0; i < NUM_HUFF_TBLS; i++) {
        entropy->dc_derived_tbls[i] = entropy->ac_derived_tbls[i] = NULL;
    #ifdef ENTROPY_OPT_SUPPORTED
        entropy->dc_count_ptrs[i] = entropy->ac_count_ptrs[i] = NULL;
    #endif
      }
        */
}
