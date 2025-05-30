/*!
  | inftrees.h -- header to use inftrees.c
  | Copyright (C) 1995-2005 Mark Adler
  | For conditions of distribution and
  | use, see copyright notice in zlib.h
  | 
  | inftrees.c -- generate Huffman trees
  | for efficient decoding Copyright (C)
  | 1995-2005 Mark Adler For conditions
  | of distribution and use, see copyright
  | notice in zlib.h
  |
  | -----------
  | @warning
  | 
  | this file should *not* be used by applications.
  | It is part of the implementation of the
  | compression library and is subject
  | to change. Applications should only
  | use zlib.h.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/inftrees.h]

/**
  | Structure for decoding tables. Each
  | entry provides either the information
  | needed to do the operation requested
  | by the code that indexed that table entry,
  | or it provides a pointer to another table
  | that indexes more bits of the code. op
  | indicates whether the entry is a pointer
  | to another table, a literal, a length
  | or distance, an end-of-block, or an
  | invalid code. For a table pointer, the
  | low four bits of op is the number of index
  | bits of that table. For a length or distance,
  | the low four bits of op is the number of
  | extra bits to get after the code. bits
  | is the number of bits in this code or part
  | of the code to drop off of the bit buffer.
  | val is the actual byte to output in the
  | case of a literal, the base length or
  | distance, or the offset from the current
  | table to the next table. Each entry is
  | four bytes.
  |
  */
pub struct Code {

    /**
       operation, extra bits, table bits
      */
    op:   u8,

    /**
       bits in this part of the code
      */
    bits: u8,

    /**
       offset in table or code value
      */
    val:  u16,
}

/*
  | op values as set by inflate_table():
  | 
  | 00000000 - literal
  | 
  | 0000tttt - table link, tttt != 0 is the
  | number of table index bits
  | 
  | 0001eeee - length or distance, eeee
  | is the number of extra bits
  | 
  | 01100000 - end of block
  | 
  | 01000000 - invalid code
  |
  */

/**
  | Maximum size of dynamic tree. The maximum
  | found in a long but non- exhaustive search
  | was 1444 code structures (852 for length/literals
  | and 592 for distances, the latter actually
  | the result of an exhaustive search).
  | The true maximum is not known, but the
  | value below is more than safe.
  |
  */
pub const ENOUGH: usize = 2048;
pub const MAXD:   usize = 592;

/**
  | Type of code to build for inftable()
  |
  */
pub enum CodeType {
    CODES,
    LENS,
    DISTS
}

lazy_static!{
    /*
    extern int inflate_table OF((codetype type, unsigned short   *lens,
                                 unsigned codes, code   *   *table,
                                 unsigned   *bits, unsigned short   *work));
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/inftrees.c]

pub const MAXBITS: usize = 15;

/**
  If you use the zlib library in a product, an
  acknowledgment is welcome in the documentation
  of your product. If for some reason you cannot
  include such an acknowledgment, I would
  appreciate that you keep this copyright string
  in the executable of your product.
 */
pub const inflate_copyright: &'static str =
   " inflate 1.2.3 Copyright 1995-2005 Mark Adler ";

/**
  | Build a set of tables to decode the provided
  | canonical Huffman code. The code lengths
  | are lens[0..codes-1]. The result starts
  | at *table, whose indices are 0..2^bits-1.
  | work is a writable array of at least lens
  | shorts, which is used as a work area.
  | type is the type of code to be generated,
  | CODES, LENS, or DISTS. On return, zero
  | is success, -1 is an invalid code, and
  | +1 means that ENOUGH isn't enough. table
  | on return points to the next available
  | entry's address. bits is the requested
  | root table index bits, and on return
  | it is the actual root table index bits.
  | It will differ if the request is greater
  | than the longest code or if it is less
  | than the shortest code.
  |
  */
pub fn inflate_table(
        ty:    CodeType,
        lens:  *mut u16,
        codes: u32,
        table: *mut *mut Code,
        bits:  *mut u32,
        work:  *mut u16) -> i32 {
    
    todo!();
    /*
        unsigned len;               /* a code's length in bits */
        unsigned sym;               /* index of code symbols */
        unsigned min, max;          /* minimum and maximum code lengths */
        unsigned root;              /* number of index bits for root table */
        unsigned curr;              /* number of index bits for current table */
        unsigned drop;              /* code bits to drop for sub-table */
        int left;                   /* number of prefix codes available */
        unsigned used;              /* code entries in table used */
        unsigned huff;              /* Huffman code */
        unsigned incr;              /* for incrementing code, index */
        unsigned fill;              /* index for replicating entries */
        unsigned low;               /* low bits for current root entry */
        unsigned mask;              /* mask for low root bits */
        code thisx;                  /* table entry for duplication */
        code   *next;             /* next available space in table */
        const unsigned short   *base;     /* base value table to use */
        const unsigned short   *extra;    /* extra bits table to use */
        int end;                    /* use base and extra for symbol > end */
        unsigned short count[MAXBITS+1];    /* number of codes of each length */
        unsigned short offs[MAXBITS+1];     /* offsets in table for each length */
        static const unsigned short lbase[31] = { /* Length codes 257..285 base */
            3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
            35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258, 0, 0};
        static const unsigned short lext[31] = { /* Length codes 257..285 extra */
            16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18,
            19, 19, 19, 19, 20, 20, 20, 20, 21, 21, 21, 21, 16, 201, 196};
        static const unsigned short dbase[32] = { /* Distance codes 0..29 base */
            1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
            257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
            8193, 12289, 16385, 24577, 0, 0};
        static const unsigned short dext[32] = { /* Distance codes 0..29 extra */
            16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22,
            23, 23, 24, 24, 25, 25, 26, 26, 27, 27,
            28, 28, 29, 29, 64, 64};

        /*
           Process a set of code lengths to create a canonical Huffman code.  The
           code lengths are lens[0..codes-1].  Each length corresponds to the
           symbols 0..codes-1.  The Huffman code is generated by first sorting the
           symbols by length from short to long, and retaining the symbol order
           for codes with equal lengths.  Then the code starts with all zero bits
           for the first code of the shortest length, and the codes are integer
           increments for the same length, and zeros are appended as the length
           increases.  For the deflate format, these bits are stored backwards
           from their more natural integer increment ordering, and so when the
           decoding tables are built in the large loop below, the integer codes
           are incremented backwards.

           This routine assumes, but does not check, that all of the entries in
           lens[] are in the range 0..MAXBITS.  The caller must assure this.
           1..MAXBITS is interpreted as that code length.  zero means that that
           symbol does not occur in this code.

           The codes are sorted by computing a count of codes for each length,
           creating from that a table of starting indices for each length in the
           sorted table, and then entering the symbols in order in the sorted
           table.  The sorted table is work[], with that space being provided by
           the caller.

           The length counts are used for other purposes as well, i.e. finding
           the minimum and maximum length codes, determining if there are any
           codes at all, checking for a valid set of lengths, and looking ahead
           at length counts to determine sub-table sizes when building the
           decoding tables.
         */

        /* accumulate lengths for codes (assumes lens[] all in 0..MAXBITS) */
        for (len = 0; len <= MAXBITS; len++)
            count[len] = 0;
        for (sym = 0; sym < codes; sym++)
            count[lens[sym]]++;

        /* bound code lengths, force root to be within code lengths */
        root = *bits;
        for (max = MAXBITS; max >= 1; max--)
            if (count[max] != 0) break;
        if (root > max) root = max;
        if (max == 0) {                     /* no symbols to code at all */
            thisx.op = (unsigned char)64;    /* invalid code marker */
            thisx.bits = (unsigned char)1;
            thisx.val = (unsigned short)0;
            *(*table)++ = thisx;             /* make a table to force an error */
            *(*table)++ = thisx;
            *bits = 1;
            return 0;     /* no symbols, but wait for decoding to report error */
        }
        for (min = 1; min <= MAXBITS; min++)
            if (count[min] != 0) break;
        if (root < min) root = min;

        /* check for an over-subscribed or incomplete set of lengths */
        left = 1;
        for (len = 1; len <= MAXBITS; len++) {
            left <<= 1;
            left -= count[len];
            if (left < 0) return -1;        /* over-subscribed */
        }
        if (left > 0 && (type == CODES || max != 1))
            return -1;                      /* incomplete set */

        /* generate offsets into symbol table for each length for sorting */
        offs[1] = 0;
        for (len = 1; len < MAXBITS; len++)
            offs[len + 1] = offs[len] + count[len];

        /* sort symbols by length, by symbol order within each length */
        for (sym = 0; sym < codes; sym++)
            if (lens[sym] != 0) work[offs[lens[sym]]++] = (unsigned short)sym;

        /*
           Create and fill in decoding tables.  In this loop, the table being
           filled is at next and has curr index bits.  The code being used is huff
           with length len.  That code is converted to an index by dropping drop
           bits off of the bottom.  For codes where len is less than drop + curr,
           those top drop + curr - len bits are incremented through all values to
           fill the table with replicated entries.

           root is the number of index bits for the root table.  When len exceeds
           root, sub-tables are created pointed to by the root entry with an index
           of the low root bits of huff.  This is saved in low to check for when a
           new sub-table should be started.  drop is zero when the root table is
           being filled, and drop is root when sub-tables are being filled.

           When a new sub-table is needed, it is necessary to look ahead in the
           code lengths to determine what size sub-table is needed.  The length
           counts are used for this, and so count[] is decremented as codes are
           entered in the tables.

           used keeps track of how many table entries have been allocated from the
           provided *table space.  It is checked when a LENS table is being made
           against the space in *table, ENOUGH, minus the maximum space needed by
           the worst case distance code, MAXD.  This should never happen, but the
           sufficiency of ENOUGH has not been proven exhaustively, hence the check.
           This assumes that when type == LENS, bits == 9.

           sym increments through all symbols, and the loop terminates when
           all codes of length max, i.e. all codes, have been processed.  This
           routine permits incomplete codes, so another loop after this one fills
           in the rest of the decoding tables with invalid code markers.
         */

        /* set up for code type */
        switch (type) {
        case CODES:
            base = extra = work;    /* dummy value--not used */
            end = 19;
            break;
        case LENS:
            base = lbase;
            base -= 257;
            extra = lext;
            extra -= 257;
            end = 256;
            break;
        default:            /* DISTS */
            base = dbase;
            extra = dext;
            end = -1;
        }

        /* initialize state for loop */
        huff = 0;                   /* starting code */
        sym = 0;                    /* starting code symbol */
        len = min;                  /* starting code length */
        next = *table;              /* current table to fill in */
        curr = root;                /* current table index bits */
        drop = 0;                   /* current bits to drop from code for index */
        low = (unsigned)(-1);       /* trigger new sub-table when len > root */
        used = 1U << root;          /* use root table entries */
        mask = used - 1;            /* mask for comparing low */

        /* check available table space */
        if (type == LENS && used >= ENOUGH - MAXD)
            return 1;

        /* process all codes and make table entries */
        for (;;) {
            /* create table entry */
            thisx.bits = (unsigned char)(len - drop);
            if ((int)(work[sym]) < end) {
                thisx.op = (unsigned char)0;
                thisx.val = work[sym];
            }
            else if ((int)(work[sym]) > end) {
                thisx.op = (unsigned char)(extra[work[sym]]);
                thisx.val = base[work[sym]];
            }
            else {
                thisx.op = (unsigned char)(32 + 64);         /* end of block */
                thisx.val = 0;
            }

            /* replicate for those indices with low len bits equal to huff */
            incr = 1U << (len - drop);
            fill = 1U << curr;
            min = fill;                 /* save offset to next table */
            do {
                fill -= incr;
                next[(huff >> drop) + fill] = thisx;
            } while (fill != 0);

            /* backwards increment the len-bit code huff */
            incr = 1U << (len - 1);
            while (huff & incr)
                incr >>= 1;
            if (incr != 0) {
                huff &= incr - 1;
                huff += incr;
            }
            else
                huff = 0;

            /* go to next symbol, update count, len */
            sym++;
            if (--(count[len]) == 0) {
                if (len == max) break;
                len = lens[work[sym]];
            }

            /* create new sub-table if needed */
            if (len > root && (huff & mask) != low) {
                /* if first time, transition to sub-tables */
                if (drop == 0)
                    drop = root;

                /* increment past last table */
                next += min;            /* here min is 1 << curr */

                /* determine length of next table */
                curr = len - drop;
                left = (int)(1 << curr);
                while (curr + drop < max) {
                    left -= count[curr + drop];
                    if (left <= 0) break;
                    curr++;
                    left <<= 1;
                }

                /* check for enough space */
                used += 1U << curr;
                if (type == LENS && used >= ENOUGH - MAXD)
                    return 1;

                /* point entry in root table to sub-table */
                low = huff & mask;
                (*table)[low].op = (unsigned char)curr;
                (*table)[low].bits = (unsigned char)root;
                (*table)[low].val = (unsigned short)(next - *table);
            }
        }

        /*
           Fill in rest of table for incomplete codes.  This loop is similar to the
           loop above in incrementing huff for table indices.  It is assumed that
           len is equal to curr + drop, so there is no loop needed to increment
           through high index bits.  When the current sub-table is filled, the loop
           drops back to the root table to fill in any remaining entries there.
         */
        thisx.op = (unsigned char)64;                /* invalid code marker */
        thisx.bits = (unsigned char)(len - drop);
        thisx.val = (unsigned short)0;
        while (huff != 0) {
            /* when done with sub-table, drop back to root table */
            if (drop != 0 && (huff & mask) != low) {
                drop = 0;
                len = root;
                next = *table;
                thisx.bits = (unsigned char)len;
            }

            /* put invalid code marker in table */
            next[huff >> drop] = thisx;

            /* backwards increment the len-bit code huff */
            incr = 1U << (len - 1);
            while (huff & incr)
                incr >>= 1;
            if (incr != 0) {
                huff &= incr - 1;
                huff += incr;
            }
            else
                huff = 0;
        }

        /* set return parameters */
        *table += used;
        *bits = root;
        return 0;
    */
}
