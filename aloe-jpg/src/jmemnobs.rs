/*!
  | jmemnobs.c
  | 
  | This file provides a really simple implementation
  | of the system- dependent portion of
  | the JPEG memory manager. This implementation
  | assumes that no backing-store files
  | are needed: all required space can be
  | obtained from malloc().
  | 
  | This is very portable in the sense that
  | it'll compile on almost anything, but
  | you'd better have lots of main memory
  | (or virtual memory) if you want to process
  | big images.
  | 
  | -----------
  | @note
  | 
  | the max_memory_to_use option is ignored
  | by this implementation.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jmemnobs.c]

/**
  | Memory allocation and freeing are controlled
  | by the regular library routines malloc()
  | and free().
  |
  */
pub fn jpeg_get_small(
        _0:           JCommonPtr,
        sizeofobject: usize)  {
    
    todo!();
        /*
            return (c_void *) malloc(sizeofobject);
        */
}

pub fn jpeg_free_small(
        _0:     JCommonPtr,
        object: *mut c_void,
        _2:     usize)  {
    
    todo!();
        /*
            free(object);
        */
}

/**
  | "Large" objects are treated the same
  | as "small" ones.
  | 
  | NB: although we include FAR keywords
  | in the routine declarations, this file
  | won't actually work in 80x86 small/medium
  | model; at least, you probably won't
  | be able to process useful-size images
  | in only 64KB.
  |
  */
pub fn jpeg_get_large(
        _0:           JCommonPtr,
        sizeofobject: usize)  {
    
    todo!();
        /*
            return (c_void FAR *) malloc(sizeofobject);
        */
}

pub fn jpeg_free_large(
        _0:     JCommonPtr,
        object: *mut c_void,
        _2:     usize)  {
    
    todo!();
        /*
            free(object);
        */
}

/**
  | This routine computes the total memory
  | space available for allocation.
  | 
  | Here we always say, "we got all you want
  | bud!"
  |
  */
pub fn jpeg_mem_available(
        _0:               JCommonPtr,
        _1:               i64,
        max_bytes_needed: i64,
        _3:               i64) -> i64 {
    
    todo!();
        /*
            return max_bytes_needed;
        */
}

/**
  | Backing store (temporary file) management.
  | 
  | Since jpeg_mem_available always promised
  | the moon, this should never be called
  | and we can just error out.
  |
  */
pub fn jpeg_open_backing_store(
        cinfo: JCommonPtr,
        _1:    *mut BackingStoreInfo,
        _2:    i64)  {
    
    todo!();
        /*
            ERREXIT(cinfo, JERR_NO_BACKING_STORE);
        */
}

/**
  | These routines take care of any system-dependent
  | initialization and cleanup required.
  | Here, there isn't any.
  |
  */
pub fn jpeg_mem_init(_0: JCommonPtr) -> i64 {
    
    todo!();
        /*
            return 0;         /* just set max_memory_to_use to 0 */
        */
}

pub fn jpeg_mem_term(_0: JCommonPtr)  {
    
    todo!();
        /*
            /* no work */
        */
}
