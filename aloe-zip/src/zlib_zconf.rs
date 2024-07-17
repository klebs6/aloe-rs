/*!
  | zconf.h -- configuration of the zlib
  | compression library Copyright (C)
  | 1995-2005 Jean-loup Gailly. For conditions
  | of distribution and use, see copyright
  | notice in zlib.h
  | 
  | @(#) $Id: zconf.h,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/zconf.h]

/**
   Just a few hacks here to make it compile nicely
   with Aloe..
  */
pub const Z_PREFIX: usize = 1;

/**
  | If you *really* need a unique prefix
  | for all types and library functions,
  | compile with -DZ_PREFIX. The "standard"
  | zlib should be compiled without it.
  |
  */
#[cfg(Z_PREFIX)] macro_rules! deflateinit_          { () => { /* z_deflateInit_ */ } }
#[cfg(Z_PREFIX)] macro_rules! deflate               { () => { /* z_deflate */ } }
#[cfg(Z_PREFIX)] macro_rules! deflateend            { () => { /* z_deflateEnd */ } }
#[cfg(Z_PREFIX)] macro_rules! inflateinit_          { () => { /* z_inflateInit_ */ } }
#[cfg(Z_PREFIX)] macro_rules! inflate               { () => { /* z_inflate */ } }
#[cfg(Z_PREFIX)] macro_rules! inflateend            { () => { /* z_inflateEnd */ } }
#[cfg(Z_PREFIX)] macro_rules! inflateprime          { () => { /* z_inflatePrime */ } }
#[cfg(Z_PREFIX)] macro_rules! inflategetheader      { () => { /* z_inflateGetHeader */ } }
#[cfg(Z_PREFIX)] macro_rules! adler32_combine       { () => { /* z_adler32_combine */ } }
#[cfg(Z_PREFIX)] macro_rules! crc32_combine         { () => { /* z_crc32_combine */ } }
#[cfg(Z_PREFIX)] macro_rules! deflateinit2_         { () => { /* z_deflateInit2_ */ } }
#[cfg(Z_PREFIX)] macro_rules! deflatesetdictionary  { () => { /* z_deflateSetDictionary */ } }
#[cfg(Z_PREFIX)] macro_rules! deflatecopy           { () => { /* z_deflateCopy */ } }
#[cfg(Z_PREFIX)] macro_rules! deflatereset          { () => { /* z_deflateReset */ } }
#[cfg(Z_PREFIX)] macro_rules! deflateparams         { () => { /* z_deflateParams */ } }
#[cfg(Z_PREFIX)] macro_rules! deflatebound          { () => { /* z_deflateBound */ } }
#[cfg(Z_PREFIX)] macro_rules! deflateprime          { () => { /* z_deflatePrime */ } }
#[cfg(Z_PREFIX)] macro_rules! inflateinit2_         { () => { /* z_inflateInit2_ */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatesetdictionary  { () => { /* z_inflateSetDictionary */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatesync           { () => { /* z_inflateSync */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatesyncpoint      { () => { /* z_inflateSyncPoint */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatecopy           { () => { /* z_inflateCopy */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatereset          { () => { /* z_inflateReset */ } }
#[cfg(Z_PREFIX)] macro_rules! inflateback           { () => { /* z_inflateBack */ } }
#[cfg(Z_PREFIX)] macro_rules! inflatebackend        { () => { /* z_inflateBackEnd */ } }
#[cfg(Z_PREFIX)] macro_rules! compress              { () => { /* z_compress */ } }
#[cfg(Z_PREFIX)] macro_rules! compress2             { () => { /* z_compress2 */ } }
#[cfg(Z_PREFIX)] macro_rules! compressbound         { () => { /* z_compressBound */ } }
#[cfg(Z_PREFIX)] macro_rules! uncompress            { () => { /* z_uncompress */ } }
#[cfg(Z_PREFIX)] macro_rules! adler32               { () => { /* z_adler32 */ } }
#[cfg(Z_PREFIX)] macro_rules! crc32                 { () => { /* z_crc32 */ } }
#[cfg(Z_PREFIX)] macro_rules! get_crc_table         { () => { /* z_get_crc_table */ } }
#[cfg(Z_PREFIX)] macro_rules! zerror                { () => { /* z_zError */ } }
#[cfg(Z_PREFIX)] macro_rules! alloc_func            { () => { /* z_alloc_func */ } }
#[cfg(Z_PREFIX)] macro_rules! free_func             { () => { /* z_free_func */ } }
#[cfg(Z_PREFIX)] macro_rules! in_func               { () => { /* z_in_func */ } }
#[cfg(Z_PREFIX)] macro_rules! out_func              { () => { /* z_out_func */ } }
#[cfg(Z_PREFIX)] macro_rules! byte                  { () => { /* z_Byte */ } }
#[cfg(Z_PREFIX)] macro_rules! uint                  { () => { /* z_uInt */ } }
#[cfg(Z_PREFIX)] macro_rules! ulong                 { () => { /* z_uLong */ } }
#[cfg(Z_PREFIX)] macro_rules! bytef                 { () => { /* z_Bytef */ } }
#[cfg(Z_PREFIX)] macro_rules! charf                 { () => { /* z_charf */ } }
#[cfg(Z_PREFIX)] macro_rules! intf                  { () => { /* z_intf */ } }
#[cfg(Z_PREFIX)] macro_rules! uintf                 { () => { /* z_uIntf */ } }
#[cfg(Z_PREFIX)] macro_rules! ulongf                { () => { /* z_uLongf */ } }
#[cfg(Z_PREFIX)] macro_rules! voidpf                { () => { /* z_voidpf */ } }
#[cfg(Z_PREFIX)] macro_rules! voidp                 { () => { /* z_voidp */ } }

lazy_static!{
    /*
    #if defined(__MSDOS__) && !defined(MSDOS)
    #  define MSDOS
    #endif

    #if (defined(OS_2) || defined(__OS2__)) && !defined(OS2)
    #  define OS2
    #endif

    #if defined(_WINDOWS) && !defined(WINDOWS)
    #  define WINDOWS
    #endif

    #if defined(_WIN32) || defined(_WIN32_WCE) || defined(__WIN32__)
    #  ifndef WIN32
    #    define WIN32
    #  endif
    #endif

    #if (defined(MSDOS) || defined(OS2) || defined(WINDOWS)) && !defined(WIN32)
    #  if !defined(__GNUC__) && !defined(__FLAT__) && !defined(__386__)
    #    ifndef SYS16BIT
    #      define SYS16BIT
    #    endif
    #  endif
    #endif
    */
}

/**
  | Compile with -DMAXSEG_64K if the alloc
  | function cannot allocate more than
  | 64k bytes at a time (needed on systems
  | with 16-bit int).
  |
  */
#[cfg(SYS16BIT)]
pub const MAXSEG_64K: bool = true;

#[cfg(MSDOS)]
pub const UNALIGNED_OK: bool = true;

/**
  | Some Mac compilers merge all .h files
  | incorrectly:
  |
  */
#[cfg(any(any(__MWERKS__,applec),any(THINK_C,__SC__)))]
pub const NO_DUMMY_DECL: bool = true;

/**
  | Maximum value for memLevel in deflateInit2
  |
  */
#[cfg(not(MAX_MEM_LEVEL))]
#[cfg(MAXSEG_64K)]
pub const MAX_MEM_LEVEL: usize = 8;

#[cfg(not(MAX_MEM_LEVEL))]
#[cfg(not(MAXSEG_64K))]
pub const MAX_MEM_LEVEL: usize = 9;

/**
  | Maximum value for windowBits in deflateInit2
  | and inflateInit2.
  | 
  | -----------
  | @warning
  | 
  | reducing MAX_WBITS makes minigzip
  | unable to extract .gz files created
  | by gzip. (Files created by minigzip
  | can still be extracted by gzip.)
  |
  |---------------
  | 32K LZ77 window
  |
  */
#[cfg(not(MAX_WBITS))]
pub const MAX_WBITS: usize = 15;

/**
  | The memory requirements for deflate
  | are (in bytes): (1 << (windowBits+2))
  | + (1 << (memLevel+9))
  | 
  | that is: 128K for windowBits=15 + 128K
  | for memLevel = 8 (default values) plus
  | a few kilobytes for small objects. For
  | example, if you want to reduce the default
  | memory requirements from 256K to 128K,
  | compile with make CFLAGS="-O -DMAX_WBITS=14
  | -DMAX_MEM_LEVEL=7"
  | 
  | Of course this will generally degrade
  | compression (there's no free lunch).
  | 
  | The memory requirements for inflate
  | are (in bytes) 1 << windowBits that is,
  | 32K for windowBits=15 (default value)
  | plus a few kilobytes for small objects.
  |
  */
#[cfg(not(__MACTYPES__))]
pub type Byte = u8; //8 bits

/**
  | 16 bits or more
  |
  */
pub type uInt = u32;

/**
  | 32 bits or more
  |
  */
pub type uLong = u64;

/**
  | Borland C/C++ and some old MSC versions
  | ignore FAR inside typedef
  |
  */
#[cfg(SMALL_MEDIUM)]
macro_rules! bytef {
    () => {
        /*
                Byte FAR
        */
    }
}

#[cfg(not(SMALL_MEDIUM))]
pub type Bytef = u8;

///---------------------
pub type charf  = u8;
pub type intf   = i32;
pub type uIntf  = u32;
pub type uLongf = u32;

#[cfg(STDC)]      pub type voidpc = *const c_void;
#[cfg(STDC)]      pub type voidpf = *mut c_void;
#[cfg(STDC)]      pub type voidp  = *mut c_void;

#[cfg(not(STDC))] pub type voidpc = *const u8;
#[cfg(not(STDC))] pub type voidpf = *mut u8;
#[cfg(not(STDC))] pub type voidp  = *mut u8;

/**
  | Seek from beginning of file.
  |
  */
#[cfg(not(SEEK_SET))]
pub const SEEK_SET: usize = 0;

/**
  | Seek from current position.
  |
  */
#[cfg(not(SEEK_SET))]
pub const SEEK_CUR: usize = 1;

/**
  | Set file pointer to EOF plus "offset"
  |
  */
#[cfg(not(SEEK_SET))]
pub const SEEK_END: usize = 2;

#[cfg(not(z_off_t))]
macro_rules! z_off_t {
    () => {
        /*
                long
        */
    }
}
