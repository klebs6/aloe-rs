/*!
  | This is the header file for the MD5 message-digest
  | algorithm.
  | 
  | The algorithm is due to Ron Rivest. This
  | code was written by Colin Plumb in 1993,
  | no copyright is claimed.
  | 
  | This code is in the public domain; do
  | with it what you wish.
  | 
  | Equivalent code is available from RSA
  | Data Security, Inc.
  | 
  | This code has been tested against that,
  | and is equivalent, except that you don't
  | need to include two pages of legalese
  | with every copy.
  | 
  | To compute the message digest of a chunk
  | of bytes, declare an
  | 
  | MD5Context structure, pass it to MD5Init,
  | call MD5Update as needed on buffers
  | full of bytes, and then call MD5Final,
  | which will fill a supplied 16-byte array
  | with the digest.
  | 
  | Changed so as no longer to depend on Colin
  | Plumb's `usual.h' header definitions;
  | now uses stuff from dpkg's config.h
  | 
  | - Ian Jackson <ijackson@nyx.cs.du.edu>.
  | 
  | Still in the public domain.
  | 
  | Josh Coalson: made some changes to integrate
  | with libFLAC.
  | 
  | Still in the public domain, with no warranty.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/md5.h]

pub union multibyte {
    p8:  u8,
    p16: i16,
    p32: i32,
}

pub struct MD5Context {
    in_:          [u32; 16],
    buf:          [u32; 4],
    bytes:        [u32; 2],
    internal_buf: multibyte,
    capacity:     usize,
}

pub fn flac_md5init(context: *mut MD5Context)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_md5final(
        digest:  [u8; 16],
        context: *mut MD5Context)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_md5accumulate(
        ctx:              *mut MD5Context,
        signal:           *const &[i32],
        channels:         u32,
        samples:          u32,
        bytes_per_sample: u32) -> bool {
    
    todo!();
        /*
        
        */
}
