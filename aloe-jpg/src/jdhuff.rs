/*!
  | jdhuff.h
  | 
  | This file contains declarations for
  | Huffman entropy decoding routines
  | that are shared between the sequential
  | decoder (jdhuff.c) and the progressive
  | decoder (jdphuff.c). No other modules
  | need to see these.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdhuff.h]

/**
  | Short forms of external names for systems
  | with brain-damaged linkers.
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_make_d_derived_tbl {
    () => {
        /*
                jMkDDerived
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_fill_bit_buffer {
    () => {
        /*
                jFilBitBuf
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_huff_decode {
    () => {
        /*
                jHufDecode
        */
    }
}

/**
  | Derived data constructed for each Huffman
  | table
  |
  */
pub const HUFF_LOOKAHEAD: usize = 8; //# of bits of lookahead 

pub struct DDerivedTbl {

    /* Basic tables: (element [0] of each array is unused) */

    /**
      | largest code of length k (-1 if none)
      |
      */
    maxcode: [i32; 18],

    /*
      | (maxcode[17] is a sentinel to ensure
      | jpeg_huff_decode terminates)
      |
      */

    /**
      | huffval[] offset for codes of length
      | k
      |
      */
    valoffset: [i32; 17],

    /*
      | valoffset[k] = huffval[] index of 1st
      | symbol of code length k, less the smallest
      | code of length k; so given a code of length
      | k, the corresponding symbol is huffval[code
      | + valoffset[k]]
      |
      */

    /**
      | Link to public Huffman table (needed
      | only in jpeg_huff_decode)
      |
      */
    pub_: *mut JHuffTbl,

    /*
      | Lookahead tables: indexed by the next
      | HUFF_LOOKAHEAD bits of the input data
      | stream. If the next Huffman code is no
      | more than HUFF_LOOKAHEAD bits long,
      | we can obtain its length and the corresponding
      | symbol directly from these tables.
      |
      */

    /**
      | # bits, or 0 if too long
      |
      */
    look_nbits: [i32; 1<<HUFF_LOOKAHEAD],

    /**
      | symbol, or unused
      |
      */
    look_sym:   [u8; 1<<HUFF_LOOKAHEAD],
}

/*
  | Fetching the next N bits from the input
  | stream is a time-critical operation
  | for the Huffman decoders. We implement
  | it with a combination of inline macros
  | and out-of-line subroutines. Note
  | that N (the number of bits demanded at
  | one time) never exceeds 15 for JPEG use.
  | 
  | We read source bytes into get_buffer
  | and dole out bits as needed.
  | 
  | If get_buffer already contains enough
  | bits, they are fetched in-line by the
  | macros CHECK_BIT_BUFFER and GET_BITS.
  | When there aren't enough bits, jpeg_fill_bit_buffer
  | is called; it will attempt to fill get_buffer
  | as full as possible (not just to the number
  | of bits needed; this prefetching reduces
  | the overhead cost of calling jpeg_fill_bit_buffer).
  | 
  | -----------
  | @note
  | 
  | jpeg_fill_bit_buffer may return FALSE
  | to indicate suspension.
  | 
  | On TRUE return, jpeg_fill_bit_buffer
  | guarantees that get_buffer contains
  | at least the requested number of bits
  | --- dummy zeroes are inserted if necessary.
  |
  */

/**
  | type of bit-extraction buffer
  |
  */
pub type BitBufType = i32;

/**
  size of buffer in bits
  */
pub const BIT_BUF_SIZE: usize = 32;

/*
  | If long is > 32 bits on your machine, and
  | shifting/masking longs is reasonably
  | fast, making bit_buf_type be long and
  | setting BIT_BUF_SIZE appropriately
  | should be a win. Unfortunately we can't
  | define the size with something like
  | #define BIT_BUF_SIZE (sizeof(bit_buf_type)*8)
  | because not all machines measure sizeof
  | in 8-bit bytes.
  |
  */

/**
  | Bitreading state saved across MCUs
  |
  */
pub struct BitreadPermState {        

    /**
      | current bit-extraction buffer
      |
      */
    get_buffer: BitBufType,


    /**
      | # of unused bits in it
      |
      */
    bits_left:  i32,
}

/**
  | Bitreading working state within an
  | MCU
  |
  */
pub struct BitreadWorkingState {        

    /*
       | Current data source location
       | 
       | We need a copy, rather than munging the
       | original, in case of suspension
       |
       */

    /**
      | => next byte to read from source
      |
      */
    next_input_byte: *const JOctet,

    /**
      | # of bytes remaining in source buffer
      |
      */
    bytes_in_buffer: usize,

    /*
       | Bit input buffer --- note these values
       | are kept in register variables, not
       | in this struct, inside the inner loops.
       |
       */

    /**
      | current bit-extraction buffer
      |
      */
    get_buffer: BitBufType,

    /**
      | # of unused bits in it
      |
      */
    bits_left:  i32,

    /* ---- Pointer needed by jpeg_fill_bit_buffer.   ---- */

    /**
      | back link to decompress master record
      |
      */
    cinfo: JDecompressPtr,
}

/**
  | Macros to declare and load/save bitread
  | local variables.
  |
  */
macro_rules! bitread_state_vars {
    () => {
        /*
        
            bit_buf_type get_buffer;  
            int bits_left;  
            bitread_working_state br_state
        */
    }
}

macro_rules! bitread_load_state {
    ($cinfop:ident, $permstate:ident) => {
        /*
        
            br_state.cinfo = cinfop; 
            br_state.next_input_byte = cinfop->src->next_input_byte; 
            br_state.bytes_in_buffer = cinfop->src->bytes_in_buffer; 
            get_buffer = permstate.get_buffer; 
            bits_left = permstate.bits_left;
        */
    }
}

macro_rules! bitread_save_state {
    ($cinfop:ident, $permstate:ident) => {
        /*
        
            cinfop->src->next_input_byte = br_state.next_input_byte; 
            cinfop->src->bytes_in_buffer = br_state.bytes_in_buffer; 
            permstate.get_buffer = get_buffer; 
            permstate.bits_left = bits_left
        */
    }
}

/**
  | These macros provide the in-line portion
  | of bit fetching.
  | 
  | Use CHECK_BIT_BUFFER to ensure there
  | are N bits in get_buffer before using
  | GET_BITS, PEEK_BITS, or DROP_BITS.
  | 
  | The variables get_buffer and bits_left
  | are assumed to be locals, but the state
  | struct might not be (jpeg_huff_decode
  | needs this).
  | 
  | CHECK_BIT_BUFFER(state,n,action);
  | 
  | - Ensure there are N bits in get_buffer;
  | if suspend, take action.
  | 
  | - val = GET_BITS(n);
  | 
  | - Fetch next N bits.
  | 
  | - val = PEEK_BITS(n);
  | 
  | - Fetch next N bits without removing
  | them from the buffer.
  | 
  | DROP_BITS(n);
  | 
  | - Discard next N bits.
  | 
  | The value N should be a simple variable,
  | not an expression, because it is evaluated
  | multiple times.
  |
  */
macro_rules! check_bit_buffer {
    ($state:ident, 
     $nbits:ident, 
     $action:ident) => {
        /*
        
            { if (bits_left < (nbits)) {  
                if (! jpeg_fill_bit_buffer(&(state),get_buffer,bits_left,nbits))  
                  { action; }  
                get_buffer = (state).get_buffer; bits_left = (state).bits_left; } }
        */
    }
}

macro_rules! get_bits {
    ($nbits:ident) => {
        /*
        
            (((int) (get_buffer >> (bits_left -= (nbits)))) & ((1<<(nbits))-1))
        */
    }
}

macro_rules! peek_bits {
    ($nbits:ident) => {
        /*
        
            (((int) (get_buffer >> (bits_left -  (nbits)))) & ((1<<(nbits))-1))
        */
    }
}

macro_rules! drop_bits {
    ($nbits:ident) => {
        /*
        
            (bits_left -= (nbits))
        */
    }
}

/**
  | Code for extracting next Huffman-coded
  | symbol from input bit stream.
  | 
  | Again, this is time-critical and we
  | make the main paths be macros.
  | 
  | We use a lookahead table to process codes
  | of up to HUFF_LOOKAHEAD bits without
  | looping. Usually, more than 95% of the
  | Huffman codes will be 8 or fewer bits
  | long. The few overlength codes are handled
  | with a loop, which need not be inline
  | code.
  | 
  | Notes about the HUFF_DECODE macro:
  | 
  | 1. Near the end of the data segment,
  | we may fail to get enough bits for a lookahead.
  | In that case, we do it the hard way.
  | 
  | 2. If the lookahead table contains
  | no entry, the next code must be more than
  | HUFF_LOOKAHEAD bits long.
  | 
  | 3. jpeg_huff_decode returns -1 if
  | forced to suspend.
  |
  */
macro_rules! huff_decode {
    ($result:ident, 
     $state:ident, 
     $htbl:ident, 
     $failaction:ident, 
     $slowlabel:ident) => {
        /*
        
        { int nb, look; 
          if (bits_left < HUFF_LOOKAHEAD) { 
            if (! jpeg_fill_bit_buffer(&state,get_buffer,bits_left, 0)) {failaction;} 
            get_buffer = state.get_buffer; bits_left = state.bits_left; 
            if (bits_left < HUFF_LOOKAHEAD) { 
              nb = 1; goto slowlabel; 
            } 
          } 
          look = PEEK_BITS(HUFF_LOOKAHEAD); 
          if ((nb = htbl->look_nbits[look]) != 0) { 
            DROP_BITS(nb); 
            result = htbl->look_sym[look]; 
          } else { 
            nb = HUFF_LOOKAHEAD+1; 
        slowlabel: 
            if ((result=jpeg_huff_decode(&state,get_buffer,bits_left,htbl,nb)) < 0) 
            { failaction; } 
            get_buffer = state.get_buffer; bits_left = state.bits_left; 
          } 
        }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdhuff.c]
/**
  | jdhuff.c
  | 
  | This file contains Huffman entropy
  | decoding routines.
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

/**
  | Expanded entropy decoder object for
  | Huffman decoding.
  | 
  | The savable_state subrecord contains
  | fields that change within an MCU, but
  | must not be updated permanently until
  | we complete the MCU.
  |
  */
pub struct SavableState2 {

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
        
            ((dest).last_dc_val[0] = (src).last_dc_val[0], 
             (dest).last_dc_val[1] = (src).last_dc_val[1], 
             (dest).last_dc_val[2] = (src).last_dc_val[2], 
             (dest).last_dc_val[3] = (src).last_dc_val[3])
        */
    }
}

pub struct HuffEntropyDecoder2 {

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
    saved:    SavableState2,

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
    dc_derived_tbls: [*mut DDerivedTbl; NUM_HUFF_TBLS],
    ac_derived_tbls: [*mut DDerivedTbl; NUM_HUFF_TBLS],

    /*
       | Precalculated info set up by start_pass
       | for use in decode_mcu:
       |
       */

    /**
      | Pointers to derived tables to be used
      | for each block within an MCU
      |
      */
    dc_cur_tbls: [*mut DDerivedTbl; D_MAX_BLOCKS_IN_MCU],
    ac_cur_tbls: [*mut DDerivedTbl; D_MAX_BLOCKS_IN_MCU],

    /**
      | Whether we care about the DC and AC coefficient
      | values for each block
      |
      */
    dc_needed: [bool; D_MAX_BLOCKS_IN_MCU],
    ac_needed: [bool; D_MAX_BLOCKS_IN_MCU],
}

pub type HuffEntropyPtr2 = *mut HuffEntropyDecoder2;

/**
  | Initialize for a Huffman-compressed
  | scan.
  |
  */
pub fn start_pass_huff_decoder(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            huff_entropy_ptr2 entropy = (huff_entropy_ptr2) cinfo->entropy;
      int ci, blkn, dctbl, actbl;
      jpeg_component_info * compptr;

      /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
       * This ought to be an error condition, but we make it a warning because
       * there are some baseline files out there with all zeroes in these bytes.
       */
      if (cinfo->Ss != 0 || cinfo->Se != DCTSIZE2-1 ||
          cinfo->Ah != 0 || cinfo->Al != 0)
        WARNMS(cinfo, JWRN_NOT_SEQUENTIAL);

      for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
        compptr = cinfo->cur_comp_info[ci];
        dctbl = compptr->dc_tbl_no;
        actbl = compptr->ac_tbl_no;
        /* Compute derived values for Huffman tables */
        /* We may do this more than once for a table, but it's not expensive */
        jpeg_make_d_derived_tbl(cinfo, TRUE, dctbl,
                    & entropy->dc_derived_tbls[dctbl]);
        jpeg_make_d_derived_tbl(cinfo, FALSE, actbl,
                    & entropy->ac_derived_tbls[actbl]);
        /* Initialize DC predictions to 0 */
        entropy->saved.last_dc_val[ci] = 0;
      }

      /* Precalculate decoding info for each block in an MCU of this scan */
      for (blkn = 0; blkn < cinfo->blocks_in_MCU; blkn++) {
        ci = cinfo->MCU_membership[blkn];
        compptr = cinfo->cur_comp_info[ci];
        /* Precalculate which table to use for each block */
        entropy->dc_cur_tbls[blkn] = entropy->dc_derived_tbls[compptr->dc_tbl_no];
        entropy->ac_cur_tbls[blkn] = entropy->ac_derived_tbls[compptr->ac_tbl_no];
        /* Decide whether we really care about the coefficient values */
        if (compptr->component_needed) {
          entropy->dc_needed[blkn] = TRUE;
          /* we don't need the ACs if producing a 1/8th-size image */
          entropy->ac_needed[blkn] = (compptr->DCT_scaled_size > 1);
        } else {
          entropy->dc_needed[blkn] = entropy->ac_needed[blkn] = FALSE;
        }
      }

      /* Initialize bitread state variables */
      entropy->bitstate.bits_left = 0;
      entropy->bitstate.get_buffer = 0; /* unnecessary, but keeps Purify quiet */
      entropy->pub.insufficient_data = FALSE;

      /* Initialize restart counter */
      entropy->restarts_to_go = cinfo->restart_interval;
        */
}

/**
  | Compute the derived values for a Huffman
  | table.
  | 
  | This routine also performs some validation
  | checks on the table.
  | 
  | Note this is also used by jdphuff.c.
  |
  */
pub fn jpeg_make_d_derived_tbl(
        cinfo: JDecompressPtr,
        isdc:  bool,
        tblno: i32,
        pdtbl: *mut *mut DDerivedTbl)  {
    
    todo!();
        /*
            JHUFF_TBL *htbl;
      d_derived_tbl *dtbl;
      int p, i, l, si, numsymbols;
      int lookbits, ctr;
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
        *pdtbl = (d_derived_tbl *)
          (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                      SIZEOF(d_derived_tbl));
      dtbl = *pdtbl;
      dtbl->pub = htbl;     /* fill in back link */

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
      numsymbols = p;

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

      /* Figure F.15: generate decoding tables for bit-sequential decoding */

      p = 0;
      for (l = 1; l <= 16; l++) {
        if (htbl->bits[l]) {
          /* valoffset[l] = huffval[] index of 1st symbol of code length l,
           * minus the minimum code of length l
           */
          dtbl->valoffset[l] = (i32) p - (i32) huffcode[p];
          p += htbl->bits[l];
          dtbl->maxcode[l] = huffcode[p-1]; /* maximum code of length l */
        } else {
          dtbl->maxcode[l] = -1;    /* -1 if no codes of this length */
        }
      }
      dtbl->maxcode[17] = 0xFFFFFL; /* ensures jpeg_huff_decode terminates */

      /* Compute lookahead tables to speed up decoding.
       * First we set all the table entries to 0, indicating "too long";
       * then we iterate through the Huffman codes that are short enough and
       * fill in all the entries that correspond to bit sequences starting
       * with that code.
       */

      MEMZERO(dtbl->look_nbits, SIZEOF(dtbl->look_nbits));

      p = 0;
      for (l = 1; l <= HUFF_LOOKAHEAD; l++) {
        for (i = 1; i <= (int) htbl->bits[l]; i++, p++) {
          /* l = current code's length, p = its index in huffcode[] & huffval[]. */
          /* Generate left-justified code followed by all possible bit sequences */
          lookbits = huffcode[p] << (HUFF_LOOKAHEAD-l);
          for (ctr = 1 << (HUFF_LOOKAHEAD-l); ctr > 0; ctr--) {
        dtbl->look_nbits[lookbits] = l;
        dtbl->look_sym[lookbits] = htbl->huffval[p];
        lookbits++;
          }
        }
      }

      /* Validate symbols as being reasonable.
       * For AC tables, we make no check, but accept all byte values 0..255.
       * For DC tables, we require the symbols to be in range 0..15.
       * (Tighter bounds could be applied depending on the data depth and mode,
       * but this is sufficient to ensure safe decoding.)
       */
      if (isDC) {
        for (i = 0; i < numsymbols; i++) {
          int sym = htbl->huffval[i];
          if (sym < 0 || sym > 15)
        ERREXIT(cinfo, JERR_BAD_HUFF_TABLE);
        }
      }
        */
}

/**
  | Out-of-line code for bit fetching (shared
  | with jdphuff.c).
  | 
  | See jdhuff.h for info about usage.
  | 
  | -----------
  | @note
  | 
  | current values of get_buffer and bits_left
  | are passed as parameters, but are returned
  | in the corresponding fields of the state
  | struct.
  | 
  | On most machines MIN_GET_BITS should
  | be 25 to allow the full 32-bit width of
  | get_buffer to be used. (On machines
  | with wider words, an even larger buffer
  | could be used.) However, on some machines
  | 32-bit shifts are quite slow and take
  | time proportional to the number of places
  | shifted. (This is true with most PC compilers,
  | for instance.) In this case it may be
  | a win to set MIN_GET_BITS to the minimum
  | value of 15. This reduces the average
  | shift distance at the cost of more calls
  | to jpeg_fill_bit_buffer.
  |
  */
#[cfg(SLOW_SHIFT_32)]
pub const MIN_GET_BITS: usize = 15; // minimum allowable value 

#[cfg(not(SLOW_SHIFT_32))]
macro_rules! min_get_bits {
    () => {
        /*
                (BIT_BUF_SIZE-7)
        */
    }
}

/**
  | Load up the bit buffer to a depth of at
  | least nbits
  |
  */
pub fn jpeg_fill_bit_buffer(
        state:      *mut BitreadWorkingState,
        get_buffer: BitBufType,
        bits_left:  i32,
        nbits:      i32) -> bool {
    
    todo!();
        /*
            /* Copy heavily used state fields into locals (hopefully registers) */
      const JOctet * next_input_byte = state->next_input_byte;
      size_t bytes_in_buffer = state->bytes_in_buffer;
      JDecompressPtr cinfo = state->cinfo;

      /* Attempt to load at least MIN_GET_BITS bits into get_buffer. */
      /* (It is assumed that no request will be for more than that many bits.) */
      /* We fail to do so only if we hit a marker or are forced to suspend. */

      if (cinfo->unread_marker == 0) {  /* cannot advance past a marker */
        while (bits_left < MIN_GET_BITS) {
          int c;

          /* Attempt to read a byte */
          if (bytes_in_buffer == 0) {
        if (! (*cinfo->src->fill_input_buffer) (cinfo))
          return FALSE;
        next_input_byte = cinfo->src->next_input_byte;
        bytes_in_buffer = cinfo->src->bytes_in_buffer;
          }
          bytes_in_buffer--;
          c = GETJOCTET(*next_input_byte++);

          /* If it's 0xFF, check and discard stuffed zero byte */
          if (c == 0xFF) {
        /* Loop here to discard any padding FF's on terminating marker,
         * so that we can save a valid unread_marker value.  NOTE: we will
         * accept multiple FF's followed by a 0 as meaning a single FF data
         * byte.  This data pattern is not valid according to the standard.
         */
        do {
          if (bytes_in_buffer == 0) {
            if (! (*cinfo->src->fill_input_buffer) (cinfo))
              return FALSE;
            next_input_byte = cinfo->src->next_input_byte;
            bytes_in_buffer = cinfo->src->bytes_in_buffer;
          }
          bytes_in_buffer--;
          c = GETJOCTET(*next_input_byte++);
        } while (c == 0xFF);

        if (c == 0) {
          /* Found FF/00, which represents an FF data byte */
          c = 0xFF;
        } else {
          /* Oops, it's actually a marker indicating end of compressed data.
           * Save the marker code for later use.
           * Fine point: it might appear that we should save the marker into
           * bitread working state, not straight into permanent state.  But
           * once we have hit a marker, we cannot need to suspend within the
           * current MCU, because we will read no more bytes from the data
           * source.  So it is OK to update permanent state right away.
           */
          cinfo->unread_marker = c;
          /* See if we need to insert some fake zero bits. */
          goto no_more_bytes;
        }
          }

          /* OK, load c into get_buffer */
          get_buffer = (get_buffer << 8) | c;
          bits_left += 8;
        } /* end while */
      } else {
      no_more_bytes:
        /* We get here if we've read the marker that terminates the compressed
         * data segment.  There should be enough bits in the buffer register
         * to satisfy the request; if so, no problem.
         */
        if (nbits > bits_left) {
          /* Uh-oh.  Report corrupted data to user and stuff zeroes into
           * the data stream, so that we can produce some kind of image.
           * We use a nonvolatile flag to ensure that only one warning message
           * appears per data segment.
           */
          if (! cinfo->entropy->insufficient_data) {
        WARNMS(cinfo, JWRN_HIT_MARKER);
        cinfo->entropy->insufficient_data = TRUE;
          }
          /* Fill the buffer with zero bits */
          get_buffer <<= MIN_GET_BITS - bits_left;
          bits_left = MIN_GET_BITS;
        }
      }

      /* Unload the local registers */
      state->next_input_byte = next_input_byte;
      state->bytes_in_buffer = bytes_in_buffer;
      state->get_buffer = get_buffer;
      state->bits_left = bits_left;

      return TRUE;
        */
}

/**
  | Out-of-line code for Huffman code decoding.
  | 
  | See jdhuff.h for info about usage.
  |
  */
pub fn jpeg_huff_decode(
        state:      *mut BitreadWorkingState,
        get_buffer: BitBufType,
        bits_left:  i32,
        htbl:       *mut DDerivedTbl,
        min_bits:   i32) -> i32 {
    
    todo!();
        /*
            int l = min_bits;
      i32 code;

      /* HUFF_DECODE has determined that the code is at least min_bits */
      /* bits long, so fetch that many bits in one swoop. */

      CHECK_BIT_BUFFER(*state, l, return -1);
      code = GET_BITS(l);

      /* Collect the rest of the Huffman code one bit at a time. */
      /* This is per Figure F.16 in the JPEG spec. */

      while (code > htbl->maxcode[l]) {
        code <<= 1;
        CHECK_BIT_BUFFER(*state, 1, return -1);
        code |= GET_BITS(1);
        l++;
      }

      /* Unload the local registers */
      state->get_buffer = get_buffer;
      state->bits_left = bits_left;

      /* With garbage input we may reach the sentinel value l = 17. */

      if (l > 16) {
        WARNMS(state->cinfo, JWRN_HUFF_BAD_CODE);
        return 0;           /* fake a zero as the safest result */
      }

      return htbl->pub->huffval[ (int) (code + htbl->valoffset[l]) ];
        */
}

/**
  | Check for a restart marker & resynchronize
  | decoder.
  | 
  | Returns FALSE if must suspend.
  |
  */
pub fn process_restart(cinfo: JDecompressPtr) -> bool {
    
    todo!();
        /*
            huff_entropy_ptr2 entropy = (huff_entropy_ptr2) cinfo->entropy;
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

/**
  | Decode and return one MCU's worth of
  | Huffman-compressed coefficients.
  | 
  | The coefficients are reordered from
  | zigzag order into natural array order,
  | but are not dequantized.
  | 
  | The i'th block of the MCU is stored into
  | the block pointed to by
  | 
  | MCU_data[i]. WE ASSUME THIS AREA HAS
  | BEEN ZEROED BY THE CALLER. (Wholesale
  | zeroing is usually a little faster than
  | retail...)
  | 
  | Returns FALSE if data source requested
  | suspension. In that case no changes
  | have been made to permanent state. (Exception:
  | some output coefficients may already
  | have been assigned. This is harmless
  | for this module, since we'll just re-assign
  | them on the next call.)
  |
  */
pub fn decode_mcu(
        cinfo:    JDecompressPtr,
        mcu_data: *mut JBlockRow) -> bool {
    
    todo!();
        /*
            huff_entropy_ptr2 entropy = (huff_entropy_ptr2) cinfo->entropy;
      int blkn;
      BITREAD_STATE_VARS;
      savable_state2 state;

      /* Process restart marker if needed; may have to suspend */
      if (cinfo->restart_interval) {
        if (entropy->restarts_to_go == 0)
          if (! process_restart(cinfo))
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
          JBLOCKROW block = MCU_data[blkn];
          d_derived_tbl * dctbl = entropy->dc_cur_tbls[blkn];
          d_derived_tbl * actbl = entropy->ac_cur_tbls[blkn];
          int s, k, r;

          /* Decode a single block's worth of coefficients */

          /* Section F.2.2.1: decode the DC coefficient difference */
          HUFF_DECODE(s, br_state, dctbl, return FALSE, label1);
          if (s) {
        CHECK_BIT_BUFFER(br_state, s, return FALSE);
        r = GET_BITS(s);
        s = HUFF_EXTEND(r, s);
          }

          if (entropy->dc_needed[blkn]) {
        /* Convert DC difference to actual value, update last_dc_val */
        int ci = cinfo->MCU_membership[blkn];
        s += state.last_dc_val[ci];
        state.last_dc_val[ci] = s;
        /* Output the DC coefficient (assumes jpeg_natural_order[0] = 0) */
        (*block)[0] = (JCoef) s;
          }

          if (entropy->ac_needed[blkn]) {

        /* Section F.2.2.2: decode the AC coefficients */
        /* Since zeroes are skipped, output area must be cleared beforehand */
        for (k = 1; k < DCTSIZE2; k++) {
          HUFF_DECODE(s, br_state, actbl, return FALSE, label2);

          r = s >> 4;
          s &= 15;

          if (s) {
            k += r;
            CHECK_BIT_BUFFER(br_state, s, return FALSE);
            r = GET_BITS(s);
            s = HUFF_EXTEND(r, s);
            /* Output coefficient in natural (dezigzagged) order.
             * Note: the extra entries in jpeg_natural_order[] will save us
             * if k >= DCTSIZE2, which could happen if the data is corrupted.
             */
            (*block)[jpeg_natural_order[k]] = (JCoef) s;
          } else {
            if (r != 15)
              break;
            k += 15;
          }
        }

          } else {

        /* Section F.2.2.2: decode the AC coefficients */
        /* In this path we just discard the values */
        for (k = 1; k < DCTSIZE2; k++) {
          HUFF_DECODE(s, br_state, actbl, return FALSE, label3);

          r = s >> 4;
          s &= 15;

          if (s) {
            k += r;
            CHECK_BIT_BUFFER(br_state, s, return FALSE);
            DROP_BITS(s);
          } else {
            if (r != 15)
              break;
            k += 15;
          }
        }

          }
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
  | Module initialization routine for
  | Huffman entropy decoding.
  |
  */
pub fn jinit_huff_decoder(cinfo: JDecompressPtr)  {
    
    todo!();
        /*
            huff_entropy_ptr2 entropy;
      int i;

      entropy = (huff_entropy_ptr2)
        (*cinfo->mem->alloc_small) ((JCommonPtr) cinfo, JPOOL_IMAGE,
                    SIZEOF(huff_entropy_decoder2));
      cinfo->entropy = (struct jpeg_entropy_decoder *) entropy;
      entropy->pub.start_pass = start_pass_huff_decoder;
      entropy->pub.decode_mcu = decode_mcu;

      /* Mark tables unallocated */
      for (i = 0; i < NUM_HUFF_TBLS; i++) {
        entropy->dc_derived_tbls[i] = entropy->ac_derived_tbls[i] = NULL;
      }
        */
}
