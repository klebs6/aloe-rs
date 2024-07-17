/*!
  | compress.c -- compress a memory buffer
  | Copyright (C) 1995-2003 Jean-loup
  | Gailly. For conditions of distribution
  | and use, see copyright notice in zlib.h
  | 
  | @(#) $Id: compress.c,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/compress.c]

/**
  | Compresses the source buffer into the
  | destination buffer. The level parameter
  | has the same meaning as in deflateInit.
  | sourceLen is the byte length of the source
  | buffer. Upon entry, destLen is the total
  | size of the destination buffer, which
  | must be at least 0.1% larger than sourceLen
  | plus 12 bytes. Upon exit, destLen is
  | the actual size of the compressed buffer.
  | 
  | compress2 returns Z_OK if success,
  | Z_MEM_ERROR if there was not enough
  | memory, Z_BUF_ERROR if there was not
  | enough room in the output buffer, Z_STREAM_ERROR
  | if the level parameter is invalid.
  |
  */
pub fn compress2(
        dest:       *mut Bytef,
        dest_len:   *mut u64,
        source:     *const Bytef,
        source_len: u64,
        level:      i32) -> i32 {
    
    todo!();
    /*
        z_stream stream;
        int err;

        stream.next_in = (Bytef*)source;
        stream.avail_in = (uInt)sourceLen;
    #ifdef MAXSEG_64K
        /* Check for source > 64K on 16-bit machine: */
        if ((uLong)stream.avail_in != sourceLen) return Z_BUF_ERROR;
    #endif
        stream.next_out = dest;
        stream.avail_out = (uInt)*destLen;
        if ((uLong)stream.avail_out != *destLen) return Z_BUF_ERROR;

        stream.zalloc = (alloc_func)0;
        stream.zfree = (free_func)0;
        stream.opaque = (voidpf)0;

        err = deflateInit(&stream, level);
        if (err != Z_OK) return err;

        err = deflate(&stream, Z_FINISH);
        if (err != Z_STREAM_END) {
            deflateEnd(&stream);
            return err == Z_OK ? Z_BUF_ERROR : err;
        }
        *destLen = stream.total_out;

        err = deflateEnd(&stream);
        return err;
    */
}

pub fn compress(
        dest:       *mut u8,
        dest_len:   *mut u64,
        source:     *const u8,
        source_len: u64) -> i32 {
    
    todo!();
    /*
        return compress2(dest, destLen, source, sourceLen, Z_DEFAULT_COMPRESSION);
    */
}

/**
  | If the default memLevel or windowBits
  | for deflateInit() is changed, then
  | this function needs to be updated.
  |
  */
pub fn compress_bound(source_len: u64) -> u64 {
    
    todo!();
    /*
        return sourceLen + (sourceLen >> 12) + (sourceLen >> 14) + 11;
    */
}
