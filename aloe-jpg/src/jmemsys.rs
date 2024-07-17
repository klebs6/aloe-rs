/*!
  | jmemsys.h
  | 
  | This include file defines the interface
  | between the system-independent and
  | system-dependent portions of the JPEG
  | memory manager. No other modules need
  | include it. (The system-independent
  | portion is jmemmgr.c; there are several
  | different versions of the system-dependent
  | portion.)
  | 
  | This file works as-is for the system-dependent
  | memory managers supplied in the IJG
  | distribution. You may need to modify
  | it if you write a custom memory manager.
  | If system-dependent changes are needed
  | in this file, the best method is to #ifdef
  | them based on a configuration symbol
  | supplied in jconfig.h, as we have done
  | with USE_MSDOS_MEMMGR and USE_MAC_MEMMGR.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jmemsys.h]

/**
  | Short forms of external names for systems
  | with brain-damaged linkers.
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_get_small {
    () => {
        /*
                jGetSmall
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_free_small {
    () => {
        /*
                jFreeSmall
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_get_large {
    () => {
        /*
                jGetLarge
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_free_large {
    () => {
        /*
                jFreeLarge
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_mem_available {
    () => {
        /*
                jMemAvail
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_open_backing_store {
    () => {
        /*
                jOpenBackStore
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_mem_init {
    () => {
        /*
                jMemInit
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_mem_term {
    () => {
        /*
                jMemTerm
        */
    }
}

/**
  | These two functions are used to allocate
  | and release small chunks of memory.
  | (Typically the total amount requested
  | through jpeg_get_small is no more than
  | 20K or so; this will be requested in chunks
  | of a few K each.)
  | 
  | Behavior should be the same as for the
  | standard library functions malloc
  | and free; in particular, jpeg_get_small
  | must return NULL on failure.
  | 
  | On most systems, these ARE malloc and
  | free. jpeg_free_small is passed the
  | size of the object being freed, just
  | in case it's needed.
  | 
  | On an 80x86 machine using small-data
  | memory model, these manage near heap.
  |
  */
lazy_static!{
    /*
    EXTERN(c_void *) jpeg_get_small JPP((JCommonPtr cinfo, size_t sizeofobject));
    EXTERN(c_void) jpeg_free_small JPP((JCommonPtr cinfo, c_void * object,
                      size_t sizeofobject));
    */
}

/**
  | These two functions are used to allocate
  | and release large chunks of memory (up
  | to the total free space designated by
  | jpeg_mem_available).
  | 
  | The interface is the same as above, except
  | that on an 80x86 machine, far pointers
  | are used. On most other machines these
  | are identical to the jpeg_get/free_small
  | routines; but we keep them separate
  | anyway, in case a different allocation
  | strategy is desirable for large chunks.
  |
  */
lazy_static!{
    /*
    EXTERN(c_void FAR *) jpeg_get_large JPP((JCommonPtr cinfo,
                           size_t sizeofobject));
    EXTERN(c_void) jpeg_free_large JPP((JCommonPtr cinfo, c_void FAR * object,
                      size_t sizeofobject));
    */
}

/**
  | The macro MAX_ALLOC_CHUNK designates
  | the maximum number of bytes that may
  | be requested in a single call to jpeg_get_large
  | (and jpeg_get_small for that matter,
  | but that case should never come into
  | play). This macro is needed to model
  | the 64Kb-segment-size limit of far
  | addressing on 80x86 machines.
  | 
  | On those machines, we expect that jconfig.h
  | will provide a proper value.
  | 
  | On machines with 32-bit flat address
  | spaces, any large constant may be used.
  | 
  | NB: jmemmgr.c expects that MAX_ALLOC_CHUNK
  | will be representable as type size_t
  | and will be a multiple of sizeof(align_type).
  |
  | may be overridden in jconfig.h
  |
  */
#[cfg(not(MAX_ALLOC_CHUNK))]
pub const MAX_ALLOC_CHUNK: usize = 1000000000;

/**
  | This routine computes the total space
  | still available for allocation by jpeg_get_large.
  | If more space than this is needed, backing
  | store will be used. NOTE: any memory
  | already allocated must not be counted.
  | 
  | There is a minimum space requirement,
  | corresponding to the minimum feasible
  | buffer sizes; jmemmgr.c will request
  | that much space even if jpeg_mem_available
  | returns zero. The maximum space needed,
  | enough to hold all working storage in
  | memory, is also passed in case it is useful.
  | 
  | Finally, the total space already allocated
  | is passed. If no better method is available,
  | cinfo->mem->max_memory_to_use -
  | already_allocated is often a suitable
  | calculation.
  | 
  | It is OK for jpeg_mem_available to underestimate
  | the space available (that'll just lead
  | to more backing-store access than is
  | really necessary).
  | 
  | However, an overestimate will lead
  | to failure. Hence it's wise to subtract
  | a slop factor from the true available
  | space. 5% should be enough.
  | 
  | On machines with lots of virtual memory,
  | any large constant may be returned.
  | 
  | Conversely, zero may be returned to
  | always use the minimum amount of memory.
  |
  */
lazy_static!{
    /*
    EXTERN(long) jpeg_mem_available JPP((JCommonPtr cinfo,
                         long min_bytes_needed,
                         long max_bytes_needed,
                         long already_allocated));
    */
}

/**
  | This structure holds whatever state
  | is needed to access a single backing-store
  | object. The read/write/close method
  | pointers are called by jmemmgr.c to
  | manipulate the backing-store object;
  | all other fields are private to the system-dependent
  | backing store routines.
  |
  | max length of a temporary file's name
  |
  */
pub const TEMP_NAME_LENGTH: usize = 64;

/* --------------- * DOS-specific junk  --------------- */

/**
  | type of extended-memory handles
  |
  */
#[cfg(USE_MSDOS_MEMMGR)]
pub type XMsh = u16;

/**
  | type of expanded-memory handles
  |
  */
#[cfg(USE_MSDOS_MEMMGR)]
pub type EMsh = u16;

#[cfg(USE_MSDOS_MEMMGR)]
pub union HandleUnion {

    /**
      | DOS file handle if it's a temp file
      |
      */
    file_handle: i16,

    /**
      | handle if it's a chunk of XMS
      |
      */
    xms_handle:  XMSH,

    /**
      | handle if it's a chunk of EMS
      |
      */
    ems_handle:  EMSH,
}

/**
  | typedef struct 
  | backing_store_struct * backing_store_ptr;
  |
  */
pub struct BackingStoreInfo {

    /**
      | Methods for reading/writing/closing
      | this backing-store object
      |
      */
    read_backing_store:  fn(
            cinfo:          JCommonPtr,
            info:           *mut BackingStoreInfo,
            buffer_address: *mut c_void,
            file_offset:    i64,
            byte_count:     i64
    ) -> (),

    write_backing_store: fn(
            cinfo:          JCommonPtr,
            info:           *mut BackingStoreInfo,
            buffer_address: *mut c_void,
            file_offset:    i64,
            byte_count:     i64
    ) -> (),

    close_backing_store: fn(cinfo: JCommonPtr, info: *mut BackingStoreInfo) -> (),

    /*
      | Private fields for system-dependent
      | backing-store management
      |
      */

    /* -- For the MS-DOS manager (jmemdos.c), we need:  -- */

    /**
      | reference to backing-store storage
      | object
      |
      */
    #[cfg(USE_MSDOS_MEMMGR)]
    handle:    HandleUnion,

    /**
      | name if it's a file
      |
      */
    #[cfg(USE_MSDOS_MEMMGR)]
    temp_name: [u8; TEMP_NAME_LENGTH],

    /* ---- For the Mac manager (jmemmac.c), we need:  ---- */

    /**
      | file reference number to temp file
      |
      */
    #[cfg(not(USE_MSDOS_MEMMGR))]
    #[cfg(USE_MAC_MEMMGR)]
    temp_file: i16,

    /**
      | the FSSpec for the temp file
      |
      */
    #[cfg(not(USE_MSDOS_MEMMGR))]
    #[cfg(USE_MAC_MEMMGR)]
    temp_spec: FSSpec,

    /**
      | name if it's a file
      |
      */
    #[cfg(not(USE_MSDOS_MEMMGR))]
    #[cfg(USE_MAC_MEMMGR)]
    temp_name: [u8; TEMP_NAME_LENGTH],

    /*
      | For a typical implementation with temp
      | files, we need:
      |
      */

    /**
      | stdio reference to temp file
      |
      */
    #[cfg(not(USE_MSDOS_MEMMGR))]
    #[cfg(not(USE_MAC_MEMMGR))]
    temp_file: *mut libc::FILE,

    /**
      | name of temp file
      |
      */
    #[cfg(not(USE_MSDOS_MEMMGR))]
    #[cfg(not(USE_MAC_MEMMGR))]
    temp_name: [u8; TEMP_NAME_LENGTH],
}

/**
  | Initial opening of a backing-store
  | object. This must fill in the read/write/close
  | pointers in the object. The read/write
  | routines may take an error exit if the
  | specified maximum file size is exceeded.
  | (If jpeg_mem_available always returns
  | a large value, this routine can just
  | take an error exit.)
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jpeg_open_backing_store JPP((JCommonPtr cinfo,
                          struct backing_store_struct *info,
                          long total_bytes_needed));
    */
}

/**
  | These routines take care of any system-dependent
  | initialization and cleanup required.
  | jpeg_mem_init will be called before
  | anything is allocated (and, therefore,
  | nothing in cinfo is of use except the
  | error manager pointer). It should return
  | a suitable default value for max_memory_to_use;
  | this may subsequently be overridden
  | by the surrounding application. (Note
  | that max_memory_to_use is only important
  | if jpeg_mem_available chooses to consult
  | it ... no one else will.) jpeg_mem_term
  | may assume that all requested memory
  | has been freed and that all opened backing-store
  | objects have been closed.
  |
  */
lazy_static!{
    /*
    EXTERN(long) jpeg_mem_init JPP((JCommonPtr cinfo));
    EXTERN(c_void) jpeg_mem_term JPP((JCommonPtr cinfo));
    */
}
