/*!
  | zutil.h -- internal interface and configuration
  | of the compression library Copyright
  | (C) 1995-2005 Jean-loup Gailly. For
  | conditions of distribution and use,
  | see copyright notice in zlib.h
  | 
  |------------
  | zutil.c -- target dependent utility
  | functions for the compression library
  | Copyright (C) 1995-2005 Jean-loup
  | Gailly. For conditions of distribution
  | and use, see copyright notice in zlib.h
  | 
  | @(#) $Id: zutil.c,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  |------------
  | @warning
  | 
  | this file should *not* be used by applications.
  | It is part of the implementation of the
  | compression library and is subject
  | to change. Applications should only
  | use zlib.h.
  | 
  | @(#) $Id: zutil.h,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/zutil.h]

pub const ZLIB_INTERNAL: bool = true;

/**
  | The Microsoft C Run-Time Library for
  | Windows CE doesn't have errno. We define
  | it as a global variable to simplify porting.
  | Its value is always 0 and should not be
  | used. We rename it to avoid conflict
  | with other libraries that use the same
  | workaround.
  |
  */
#[cfg(NO_ERRNO_H)]
#[cfg(_WIN32_WCE)]
macro_rules! errno {
    () => {
        /*
                z_errno
        */
    }
}

#[cfg(NO_ERRNO_H)]
lazy_static!{
    /*
    extern int errno;
    */
}

pub type uch  = u8;
pub type uchf = u8;
pub type ush  = u16;
pub type ushf = u16;
pub type ulg  = u64;

/**
  | indexed by 2-zlib_error (size given
  | to avoid silly warnings with Visual
  | C++)
  |
  */
lazy_static!{
    /*
    extern const char * const z_errmsg[10];
    */
}

macro_rules! err_msg {
    ($err:ident) => {
        /*
                z_errmsg[Z_NEED_DICT-(err)]
        */
    }
}

/**
  | To be used only when the state is known
  | to be valid
  |
  */
macro_rules! err_return {
    ($strm:ident, $err:ident) => {
        /*
        
          return (strm->msg = (char*)ERR_MSG(err), (err))
        */
    }
}


/* ---------------- common constants  ---------------- */

/**
  | default windowBits for decompression.
  | MAX_WBITS is for compression only
  |
  */
#[cfg(not(DEF_WBITS))]
macro_rules! def_wbits {
    () => {
        /*
                MAX_WBITS
        */
    }
}

/**
  | default memLevel
  |
  */
#[cfg(MAX_MEM_LEVEL_GTE_8)]
pub const DEF_MEM_LEVEL: usize = 8;

#[cfg(not(MAX_MEM_LEVEL_GTE_8))]
pub const DEF_MEM_LEVEL: usize = MAX_MEM_LEVEL;

/* ---------- The three kinds of block type  ---------- */
pub const STORED_BLOCK: usize = 0;
pub const STATIC_TREES: usize = 1;
pub const DYN_TREES:    usize = 2;

/* ------ The minimum and maximum match lengths  ------ */
pub const MIN_MATCH:    usize = 3;
pub const MAX_MATCH:    usize = 258;

/* ------ preset dictionary flag in zlib header  ------ */
pub const PRESET_DICT:  usize = 0x20;

/* --------------- target dependencies  --------------- */

#[cfg(any(MSDOS,all(WINDOWS,not(WIN32))))]
pub const OS_CODE: usize = 0x00;

/**
  | Allow compilation with ANSI keywords
  | only enabled
  |
  */
#[cfg(any(MSDOS,all(WINDOWS,not(WIN32))))]
#[cfg(any(__TURBOC__,__BORLANDC__))]
#[cfg(all(__STDC___EQ_1,any(__LARGE__,__COMPACT__)))]
extern "cdecl" {

    pub fn farfree(block: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }


    pub fn farmalloc(nbytes: u64)  {
        
        todo!();
        /*
        
        */
    }
}

///------------------------------
#[cfg(AMIGA)]
pub const OS_CODE: usize = 0x01;

///------------------------------
#[cfg(any(VAXC,VMS))]
pub const OS_CODE: usize = 0x02;

#[cfg(any(VAXC,VMS))]
macro_rules! f_open {
    ($name:ident, 
     $mode:ident) => {
        /*
        
             fopen((name), (mode), "mbc=60", "ctx=stm", "rfm=fix", "mrs=512")
        */
    }
}

#[cfg(any(ATARI,atarist))]
pub const OS_CODE: usize = 0x05;

#[cfg(OS2)]
pub const OS_CODE: usize = 0x06;

#[cfg(any(MACOS,TARGET_OS_MAC))]
pub const OS_CODE: usize = 0x07;

/**
  | No fdopen()
  |
  */
#[cfg(not(all(__MWERKS__,__dest_os_neq___be_os, __dest_os_neq___win32_os)))]
#[cfg(any(MACOS,TARGET_OS_MAC))]
#[cfg(not(fdopen))]
macro_rules! fdopen {
    ($fd:ident, $mode:ident) => {
        /*
                NULL 
        */
    }
}

#[cfg(TOPS20)]
pub const OS_CODE: usize = 0x0a;

/**
  | Cygwin is Unix, not Win32
  |
  */
#[cfg(WIN32)]
#[cfg(not(__CYGWIN__))]
pub const OS_CODE: usize = 0x0b;

/**
  | Prime/PRIMOS
  |
  */
#[cfg(__50SERIES)]
pub const OS_CODE: usize = 0x0f;

/**
  | No fdopen()
  |
  */
#[cfg(any(_BEOS_,RISCOS))]
macro_rules! fdopen {
    ($fd:ident, 
     $mode:ident) => {
        /*
                NULL 
        */
    }
}

/**
  | No fdopen()
  |
  */
#[cfg(all(_MSC_VER,_MSC_VER_GT_600))]
#[cfg(_WIN32_WCE)]
macro_rules! fdopen {
    ($fd:ident, $mode:ident) => {
        /*
                NULL 
        */
    }
}

#[cfg(all(_MSC_VER,_MSC_VER_GT_600))]
#[cfg(_WIN32_WCE)]
#[cfg(not(_PTRDIFF_T_DEFINED))]
pub type ptrdiff_t = i32;

#[cfg(all(_MSC_VER,_MSC_VER_GT_600))]
#[cfg(_WIN32_WCE)]
pub const _PTRDIFF_T_DEFINED: bool = true;

#[cfg(all(_MSC_VER,_MSC_VER_GT_600))]
#[cfg(not(_WIN32_WCE))]
macro_rules! fdopen {
    ($fd:ident, $type:ident) => {
        /*
                _fdopen(fd,type)
        */
    }
}

/* ----------------- common defaults  ----------------- */

/**
  | assume Unix
  |
  */
#[cfg(not(OS_CODE))]
pub const OS_CODE: usize = 0x03;

#[cfg(not(F_OPEN))]
macro_rules! f_open {
    ($name:ident, $mode:ident) => {
        /*
                fopen((name), (mode))
        */
    }
}

/* -------------------- functions  -------------------- */

/** Diagnostic functions */
macro_rules! assert  { ($cond:ident, $msg:ident) => { } }
macro_rules! trace   { ($x:ident) => { } }
macro_rules! tracev  { ($x:ident) => { } }
macro_rules! tracevv { ($x:ident) => { } }
macro_rules! tracec  { ($c:ident, $x:ident) => { } }
macro_rules! tracecv { ($c:ident, $x:ident) => { } }
macro_rules! z_error { ($x:ident) => { } }

macro_rules! z_verbose {
    () => {
        /*
                0
        */
    }
}

macro_rules! zalloc {
    ($strm:ident, 
     $items:ident, 
     $size:ident) => {
        /*
        
                   (*((strm)->zalloc))((strm)->opaque, (items), (size))
        */
    }
}

macro_rules! zfree {
    ($strm:ident, 
     $addr:ident) => {
        /*
                (*((strm)->zfree))((strm)->opaque, (voidpf)(addr))
        */
    }
}

macro_rules! try_free {
    ($s:ident, 
     $p:ident) => {
        /*
                {if (p) ZFREE(s, p);}
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/zutil.c]

/**
  | for buggy compilers
  |
  */
#[cfg(not(NO_DUMMY_DECL))]
pub struct InternalState {
    dummy: i32,
}

pub const z_errmsg: [&'static str; 10] = [

    /**
      | Z_NEED_DICT 2
      |
      */
    "need dictionary",     

    /**
      | Z_STREAM_END 1
      |
      */
    "stream end",          

    /**
      | Z_OK 0
      |
      */
    "",                    

    /**
      | Z_ERRNO (-1)
      |
      */
    "file error",          

    /**
      | Z_STREAM_ERROR (-2)
      |
      */
    "stream error",        

    /**
      | Z_DATA_ERROR (-3)
      |
      */
    "data error",          

    /**
      | Z_MEM_ERROR (-4)
      |
      */
    "insufficient memory", 

    /**
      | Z_BUF_ERROR (-5)
      |
      */
    "buffer error",        

    /**
      | Z_VERSION_ERROR (-6)
      |
      */
    "incompatible version",

    ""
];

/**
  | exported to allow conversion of error
  | code to string for compress() and uncompress()
  |
  */
pub fn z_error(err: i32) -> *const u8 {
    
    todo!();
    /*
        return ERR_MSG(err);
    */
}

/**
  | The Microsoft C Run-Time Library for
  | Windows CE doesn't have errno. We define
  | it as a global variable to simplify porting.
  | Its value is always 0 and should not be
  | used.
  |
  */
#[cfg(_WIN32_WCE)]
lazy_static!{
    /*
    int errno = 0;
    */
}

#[cfg(not(HAVE_MEMCPY))]
pub fn zmemcpy(
        dest:   *mut u8,
        source: *const u8,
        len:    u32)  {
    
    todo!();
    /*
        if (len == 0) return;
        do {
            *dest++ = *source++; /* ??? to be unrolled */
        } while (--len != 0);
    */
}

#[cfg(not(HAVE_MEMCPY))]
pub fn zmemcmp(
        s1:  *const u8,
        s2:  *const u8,
        len: u32) -> i32 {
    
    todo!();
    /*
        uInt j;

        for (j = 0; j < len; j++) {
            if (s1[j] != s2[j]) return 2*(s1[j] > s2[j])-1;
        }
        return 0;
    */
}

#[cfg(not(HAVE_MEMCPY))]
pub fn zmemzero(
        dest: *mut u8,
        len:  u32)  {
    
    todo!();
    /*
        if (len == 0) return;
        do {
            *dest++ = 0;  /* ??? to be unrolled */
        } while (--len != 0);
    */
}


/* ------------- Turbo C in 16-bit mode  ------------- */

#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
pub const MY_ZCALLOC: bool = true;

/*
  | Turbo C malloc() does not allow dynamic
  | allocation of 64K bytes and farmalloc(64K)
  | returns a pointer with an offset of 8,
  | so we must fix the pointer. Warning:
  | the pointer must be put back to its original
  | form in order to free it, use zcfree().
  |
  */

/**
  | 10*64K = 640K
  |
  */
#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
pub const MAX_PTR: usize = 10;

#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
lazy_static!{
    /*
    static int next_ptr = 0;
    */
}

#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
pub struct PtrTable {
    org_ptr: *mut c_void,
    new_ptr: *mut c_void,
}

#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
lazy_static!{
    /*
    static ptr_table table[MAX_PTR];
    */
}

/**
  | This table is used to remember the original
  | form of pointers to large buffers (64K).
  | Such pointers are normalized with a
  | zero offset.
  | 
  | Since MSDOS is not a preemptive multitasking
  | OS, this table is not protected from
  | concurrent access. This hack doesn't
  | work anyway on a protected system like
  | OS/2. Use Microsoft C instead.
  |
  */
#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
pub fn zcalloc(
        opaque: *mut c_void,
        items:  u32,
        size:   u32) -> *mut c_void {
    
    todo!();
    /*
        voidpf buf = opaque; /* just to make some compilers happy */
        ulg bsize = (ulg)items*size;

        /* If we allocate less than 65520 bytes, we assume that farmalloc
         * will return a usable pointer which doesn't have to be normalized.
         */
        if (bsize < 65520L) {
            buf = farmalloc(bsize);
            if (*(ush*)&buf != 0) return buf;
        } else {
            buf = farmalloc(bsize + 16L);
        }
        if (buf == NULL || next_ptr >= MAX_PTR) return NULL;
        table[next_ptr].org_ptr = buf;

        /* Normalize the pointer to seg:0 */
        *((ush*)&buf+1) += ((ush)((uch*)buf-0) + 15) >> 4;
        *(ush*)&buf = 0;
        table[next_ptr++].new_ptr = buf;
        return buf;
    */
}

#[cfg(SYS16BIT)]
#[cfg(__TURBOC__)]
pub fn zcfree(
        opaque: *mut c_void,
        ptr:    *mut c_void)  {
    
    todo!();
    /*
        int n;
        if (*(ush*)&ptr != 0) { /* object < 64K */
            farfree(ptr);
            return;
        }
        /* Find the original pointer */
        for (n = 0; n < next_ptr; n++) {
            if (ptr != table[n].new_ptr) continue;

            farfree(table[n].org_ptr);
            while (++n < next_ptr) {
                table[n-1] = table[n];
            }
            next_ptr--;
            return;
        }
        ptr = opaque; /* just to make some compilers happy */
        Assert(0, "zcfree: ptr not found");
    */
}

/* ----------- Microsoft C in 16-bit mode  ----------- */

#[cfg(SYS16BIT)]
#[cfg(M_I86)]
pub const MY_ZCALLOC: bool = true;

#[cfg(SYS16BIT)]
#[cfg(M_I86)]
pub fn zcalloc(
        opaque: *mut c_void,
        items:  u32,
        size:   u32) -> *mut c_void {
    
    todo!();
    /*
        if (opaque) opaque = 0; /* to make compiler happy */
        return _halloc((long)items, size);
    */
}

#[cfg(SYS16BIT)]
#[cfg(M_I86)]
pub fn zcfree(
        opaque: *mut c_void,
        ptr:    *mut c_void)  {
    
    todo!();
    /*
        if (opaque) opaque = 0; /* to make compiler happy */
        _hfree(ptr);
    */
}

/**
  | Any system without a special alloc function
  |
  */
#[cfg(not(MY_ZCALLOC))]
pub fn zcalloc(
        opaque: *mut c_void,
        items:  u32,
        size:   u32) -> *mut c_void {
    
    todo!();
    /*
        if (opaque) items += size - size; /* make compiler happy */
        return sizeof(uInt) > 2 ? (voidpf)malloc(items * size) :
                                  (voidpf)calloc(items, size);
    */
}

#[cfg(not(MY_ZCALLOC))]
pub fn zcfree(
        opaque: *mut c_void,
        ptr:    *mut c_void)  {
    
    todo!();
    /*
        free(ptr);
        if (opaque) return; /* make compiler happy */
    */
}
