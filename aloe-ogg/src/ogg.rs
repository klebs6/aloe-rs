/*!
 function: toplevel libogg include
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/ogg.h]

pub struct OggIoVec {
    iov_base: *mut c_void,
    iov_len:  usize,
}

pub struct OggPackBuffer {
    endbyte: i64,
    endbit:  i32,
    buffer:  *mut u8,
    ptr:     *mut u8,
    storage: i64,
}

/**
  | ogg_page is used to encapsulate the
  | data in one Ogg bitstream page ****
  |
  */
pub struct OggPage
{
    header:     *mut u8,
    header_len: i64,
    body:       *mut u8,
    body_len:   i64,
}

/**
  | ogg_stream_state contains the current
  | encode/decode state of a logical
  | 
  | Ogg bitstream *********************************************************
  |
  */
pub struct OggStreamState {

    /**
      | bytes from packet bodies
      |
      */
    body_data:       *mut u8,

    /**
      | storage elements allocated
      |
      */
    body_storage:    i64,

    /**
      | elements stored; fill mark
      |
      */
    body_fill:       i64,

    /**
      | elements of fill returned
      |
      */
    body_returned:   i64,

    /**
      | The values that will go to the segment
      | table
      |
      */
    lacing_vals:     *mut i32,

    /**
      | granulepos values for headers. Not
      | compact this way, but it is simple coupled
      | to the lacing fifo
      |
      */
    granule_vals:    *mut i64,

    lacing_storage:  i64,
    lacing_fill:     i64,
    lacing_packet:   i64,
    lacing_returned: i64,

    /**
      | working space for header encode
      |
      */
    header:          [u8; 282],

    header_fill:     i32,

    /**
      | set when we have buffered the last packet
      | in the logical bitstream
      |
      */
    e_o_s:           i32,

    /**
      | set after we've written the initial
      | page of a logical bitstream
      |
      */
    b_o_s:           i32,

    serialno:        i64,
    pageno:          i64,

    /**
      | sequence number for decode; the framing
      | knows where there's a hole in the data,
      | but we need coupling so that the codec
      | (which is in a separate abstraction
      | layer) also knows about the gap
      |
      */
    packetno:        i64,

    granulepos:      i64,
}

/**
  | ogg_packet is used to encapsulate the
  | data and metadata belonging to a single
  | raw Ogg/Vorbis packet 
  |
  */
pub struct OggPacket {
    packet:     *mut u8,
    bytes:      i64,
    b_o_s:      i64,
    e_o_s:      i64,
    granulepos: i64,

    /**
      | sequence number for decode; the framing
      | knows where there's a hole in the data,
      | but we need coupling so that the codec
      | (which is in a separate abstraction
      | layer) also knows about the gap
      |
      */
    packetno:   i64,
}

pub struct OggSyncState {
    data:        *mut u8,
    storage:     i32,
    fill:        i32,
    returned:    i32,
    unsynced:    i32,
    headerbytes: i32,
    bodybytes:   i32,
}

unsafe extern "C" {

    /**
      | Ogg BITSTREAM PRIMITIVES: bitstream
      |
      */
    lazy_static!{
        /*
        extern void  oggpack_writeinit(oggpack_buffer *b);
        extern int   oggpack_writecheck(oggpack_buffer *b);
        extern void  oggpack_writetrunc(oggpack_buffer *b,long bits);
        extern void  oggpack_writealign(oggpack_buffer *b);
        extern void  oggpack_writecopy(oggpack_buffer *b,void *source,long bits);
        extern void  oggpack_reset(oggpack_buffer *b);
        extern void  oggpack_writeclear(oggpack_buffer *b);
        extern void  oggpack_readinit(oggpack_buffer *b,unsigned char *buf,int bytes);
        extern void  oggpack_write(oggpack_buffer *b,unsigned long value,int bits);
        extern long  oggpack_look(oggpack_buffer *b,int bits);
        extern long  oggpack_look1(oggpack_buffer *b);
        extern void  oggpack_adv(oggpack_buffer *b,int bits);
        extern void  oggpack_adv1(oggpack_buffer *b);
        extern long  oggpack_read(oggpack_buffer *b,int bits);
        extern long  oggpack_read1(oggpack_buffer *b);
        extern long  oggpack_bytes(oggpack_buffer *b);
        extern long  oggpack_bits(oggpack_buffer *b);
        extern unsigned char *oggpack_get_buffer(oggpack_buffer *b);

        extern void  oggpackB_writeinit(oggpack_buffer *b);
        extern int   oggpackB_writecheck(oggpack_buffer *b);
        extern void  oggpackB_writetrunc(oggpack_buffer *b,long bits);
        extern void  oggpackB_writealign(oggpack_buffer *b);
        extern void  oggpackB_writecopy(oggpack_buffer *b,void *source,long bits);
        extern void  oggpackB_reset(oggpack_buffer *b);
        extern void  oggpackB_writeclear(oggpack_buffer *b);
        extern void  oggpackB_readinit(oggpack_buffer *b,unsigned char *buf,int bytes);
        extern void  oggpackB_write(oggpack_buffer *b,unsigned long value,int bits);
        extern long  oggpackB_look(oggpack_buffer *b,int bits);
        extern long  oggpackB_look1(oggpack_buffer *b);
        extern void  oggpackB_adv(oggpack_buffer *b,int bits);
        extern void  oggpackB_adv1(oggpack_buffer *b);
        extern long  oggpackB_read(oggpack_buffer *b,int bits);
        extern long  oggpackB_read1(oggpack_buffer *b);
        extern long  oggpackB_bytes(oggpack_buffer *b);
        extern long  oggpackB_bits(oggpack_buffer *b);
        extern unsigned char *oggpackB_get_buffer(oggpack_buffer *b);
        */
    }

    /**
      | Ogg BITSTREAM PRIMITIVES: encoding
      |
      */
    lazy_static!{
        /*
        extern int      ogg_stream_packetin(ogg_stream_state *os, ogg_packet *op);
        extern int      ogg_stream_iovecin(ogg_stream_state *os, ogg_iovec_t *iov,
                                           int count, long e_o_s, ogg_int64_t granulepos);
        extern int      ogg_stream_pageout(ogg_stream_state *os, ogg_page *og);
        extern int      ogg_stream_pageout_fill(ogg_stream_state *os, ogg_page *og, int nfill);
        extern int      ogg_stream_flush(ogg_stream_state *os, ogg_page *og);
        extern int      ogg_stream_flush_fill(ogg_stream_state *os, ogg_page *og, int nfill);
        */
    }

    /**
      | Ogg BITSTREAM PRIMITIVES: decoding
      |
      */
    lazy_static!{
        /*
        extern int      ogg_sync_init(ogg_sync_state *oy);
        extern int      ogg_sync_clear(ogg_sync_state *oy);
        extern int      ogg_sync_reset(ogg_sync_state *oy);
        extern int      ogg_sync_destroy(ogg_sync_state *oy);
        extern int      ogg_sync_check(ogg_sync_state *oy);

        extern char    *ogg_sync_buffer(ogg_sync_state *oy, long size);
        extern int      ogg_sync_wrote(ogg_sync_state *oy, long bytes);
        extern long     ogg_sync_pageseek(ogg_sync_state *oy,ogg_page *og);
        extern int      ogg_sync_pageout(ogg_sync_state *oy, ogg_page *og);
        extern int      ogg_stream_pagein(ogg_stream_state *os, ogg_page *og);
        extern int      ogg_stream_packetout(ogg_stream_state *os,ogg_packet *op);
        extern int      ogg_stream_packetpeek(ogg_stream_state *os,ogg_packet *op);
        */
    }

    /**
      | Ogg BITSTREAM PRIMITIVES: general
      |
      */
    lazy_static!{
        /*
        extern int      ogg_stream_init(ogg_stream_state *os,int serialno);
        extern int      ogg_stream_clear(ogg_stream_state *os);
        extern int      ogg_stream_reset(ogg_stream_state *os);
        extern int      ogg_stream_reset_serialno(ogg_stream_state *os,int serialno);
        extern int      ogg_stream_destroy(ogg_stream_state *os);
        extern int      ogg_stream_check(ogg_stream_state *os);
        extern int      ogg_stream_eos(ogg_stream_state *os);

        extern void     ogg_page_checksum_set(ogg_page *og);

        extern int      ogg_page_version(const ogg_page *og);
        extern int      ogg_page_continued(const ogg_page *og);
        extern int      ogg_page_bos(const ogg_page *og);
        extern int      ogg_page_eos(const ogg_page *og);
        extern ogg_int64_t  ogg_page_granulepos(const ogg_page *og);
        extern int      ogg_page_serialno(const ogg_page *og);
        extern long     ogg_page_pageno(const ogg_page *og);
        extern int      ogg_page_packets(const ogg_page *og);

        extern void     ogg_packet_clear(ogg_packet *op);
        */
    }
}
