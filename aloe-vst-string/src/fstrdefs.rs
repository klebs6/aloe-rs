/*!
  | string methods defines unicode / ASCII
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/fstrdefs.h]

/*
  | 16 bit string operations
  |
  | if c++11 unicode string literals
  */

#[cfg(SMTG_CPP11)]
macro_rules! smtg_cpp11_cat_private_dont_use {
    ($a:ident, $b:ident) => {
        /*
                a ## b
        */
    }
}

#[cfg(SMTG_CPP11)]
#[cfg(SMTG_OS_WINDOWS)]
macro_rules! str16 {
    ($x:ident) => {
        /*
                SMTG_CPP11_CAT_PRIVATE_DONT_USE(L,x)
        */
    }
}

#[cfg(SMTG_CPP11)]
#[cfg(not(SMTG_OS_WINDOWS))]
macro_rules! str16 {
    ($x:ident) => {
        /*
                SMTG_CPP11_CAT_PRIVATE_DONT_USE(u,x)
        */
    }
}

#[cfg(not(SMTG_CPP11))]
macro_rules! str16 {
    ($x:ident) => {
        /*
                typename Steinberg::ConstStringTable::instance ()->getString (x)
        */
    }
}

///--------------------------
#[cfg(UNICODE)]
macro_rules! str_ {
    ($x:ident) => {
        /*
                STR16(x)
        */
    }
}

#[cfg(UNICODE)]
macro_rules! t_str_buffer_size {
    ($buffer:ident) => {
        /*
                (sizeof(buffer)/sizeof(typename Steinberg::tchar))
        */
    }
}

#[cfg(not(UNICODE))]
macro_rules! str_ {
    ($x:ident) => {
        /*
                x
        */
    }
}

#[cfg(not(UNICODE))]
macro_rules! t_str_buffer_size {
    ($buffer:ident) => {
        /*
                (sizeof(buffer))
        */
    }
}

///--------------------------
macro_rules! str_8buffer_size {
    ($buffer:ident) => {
        /*
                (sizeof(buffer)/sizeof(typename Steinberg::char8))
        */
    }
}

macro_rules! str_16buffer_size {
    ($buffer:ident) => {
        /*
                (sizeof(buffer)/sizeof(typename Steinberg::char16))
        */
    }
}

///--------------------------
#[cfg(SMTG_OS_WINDOWS)]
pub const FORMAT_INT64A:  &'static str = "I64d";

#[cfg(SMTG_OS_WINDOWS)]
pub const FORMAT_UINT64A: &'static str = "I64u";

#[cfg(any(SMTG_OS_MACOS,SMTG_OS_LINUX))]
pub const FORMAT_INT64A:  &'static str =  "lld";

#[cfg(any(SMTG_OS_MACOS,SMTG_OS_LINUX))]
pub const FORMAT_UINT64A: &'static str = "llu";

#[cfg(any(SMTG_OS_MACOS,SMTG_OS_LINUX))]
macro_rules! stricmp { () => { /* strcasecmp */ } }

#[cfg(any(SMTG_OS_MACOS,SMTG_OS_LINUX))]
macro_rules! strnicmp { () => { /* strncasecmp */ } }

///--------------------------
#[cfg(UNICODE)]      macro_rules! format_int64w  { () => { /* STR(FORMAT_INT64A) */ } }
#[cfg(UNICODE)]      macro_rules! format_uint64w { () => { /* STR(FORMAT_UINT64A) */ } }
#[cfg(UNICODE)]      macro_rules! format_int64   { () => { /* FORMAT_INT64W */ } }
#[cfg(UNICODE)]      macro_rules! format_uint64  { () => { /* FORMAT_UINT64W */ } }

#[cfg(not(UNICODE))] macro_rules! format_int64   { () => { /* FORMAT_INT64A */ } }
#[cfg(not(UNICODE))] macro_rules! format_uint64  { () => { /* FORMAT_UINT64A */ } }

///---------------------------
#[cfg(SMTG_OS_WINDOWS)] pub const ENDLINE_A: &'static str = "\r\n";
#[cfg(SMTG_OS_WINDOWS)] pub const ENDLINE_W: &'static str = "\r\n";

#[cfg(SMTG_OS_MACOS)]   pub const ENDLINE_A: &'static str = "\r";
#[cfg(SMTG_OS_MACOS)]   pub const ENDLINE_W: &'static str = "\r";

#[cfg(SMTG_OS_LINUX)]   pub const ENDLINE_A: &'static str = "\n";
#[cfg(SMTG_OS_LINUX)]   pub const ENDLINE_W: &'static str = "\n";

///---------------------------
#[cfg(UNICODE)]      macro_rules! endline { () => { /* ENDLINE_W */ } }
#[cfg(not(UNICODE))] macro_rules! endline { () => { /* ENDLINE_A */ } }

///---------------------------
#[cfg(all(all(SMTG_OS_WINDOWS,not(__GNUC__)),all(_MSC_VER,_MSC_VER_LT_1900)))]
macro_rules! stricmp  { () => { /* _stricmp */ } }

#[cfg(all(all(SMTG_OS_WINDOWS,not(__GNUC__)),all(_MSC_VER,_MSC_VER_LT_1900)))]
macro_rules! strnicmp { () => { /* _strnicmp */ } }

#[cfg(all(all(SMTG_OS_WINDOWS,not(__GNUC__)),all(_MSC_VER,_MSC_VER_LT_1900)))]
macro_rules! snprintf { () => { /* _snprintf */ } }

///---------------------------

pub const empty_string:   &[u16] = &[ 0 ];
pub const empty_string8:  &[u8]  = &[ 0 ];
pub const empty_string16: &[u16] = &[ 0 ];

#[cfg(UNICODE)]      pub const infinite_symbol: &[u16] = &[ 0x221E, 0 ];
#[cfg(not(UNICODE))] pub const infinite_symbol: &'static str = "oo";

#[inline] pub fn tstrlen<T>(wcs: *const T) -> i32 {

    todo!();
        /*
            const T* eos = wcs;

        while (*eos++)
            ;

        return (int32) (eos - wcs - 1);
        */
}

#[inline] pub fn strlen8(str_: *const u8) -> i32 {
    
    todo!();
        /*
            return _tstrlen (str);
        */
}

#[inline] pub fn strlen16(str_: *const u16) -> i32 {
    
    todo!();
        /*
            return _tstrlen (str);
        */
}

#[inline] pub fn tstrcmp<T>(
        src: *const T,
        dst: *const T) -> i32 {

    todo!();
        /*
            while (*src == *dst && *dst)
        {
            src++;
            dst++;
        }

        if (*src == 0 && *dst == 0)
            return 0;
        else if (*src == 0)
            return -1;
        else if (*dst == 0)
            return 1;
        else
            return (int32) (*src - *dst);
        */
}

#[inline] pub fn strcmp8(
        src: *const u8,
        dst: *const u8) -> i32 {
    
    todo!();
        /*
            return _tstrcmp (src, dst);
        */
}

#[inline] pub fn strcmp16(
        src: *const u16,
        dst: *const u16) -> i32 {
    
    todo!();
        /*
            return _tstrcmp (src, dst);
        */
}

lazy_static!{
    /*
    template <typename T>
    inline int32 strcmpT (const T* first, const T* last);

    template <>
    inline int32 strcmpT<char8> (const char8* first, const char8* last) { return _tstrcmp (first, last); }

    template <>
    inline int32 strcmpT<char16> (const char16* first, const char16* last) { return _tstrcmp (first, last); }
    */
}

///------------------------
#[inline] pub fn tstrncmp<T>(
        first: *const T,
        last:  *const T,
        count: u32) -> i32 {

    todo!();
        /*
            if (count == 0)
            return 0;

        while (--count && *first && *first == *last)
        {
            first++;
            last++;
        }

        if (*first == 0 && *last == 0)
            return 0;
        else if (*first == 0)
            return -1;
        else if (*last == 0)
            return 1;
        else
            return (int32) (*first - *last);
        */
}

#[inline] pub fn strncmp8(
        first: *const u8,
        last:  *const u8,
        count: u32) -> i32 {
    
    todo!();
        /*
            return _tstrncmp (first, last, count);
        */
}

#[inline] pub fn strncmp16(
        first: *const u16,
        last:  *const u16,
        count: u32) -> i32 {
    
    todo!();
        /*
            return _tstrncmp (first, last, count);
        */
}

lazy_static!{
    /*
    template <typename T>
    inline int32 strncmpT (const T* first, const T* last, uint32 count);

    template <>
    inline int32 strncmpT<char8> (const char8* first, const char8* last, uint32 count) { return _tstrncmp (first, last, count); }

    template <>
    inline int32 strncmpT<char16> (const char16* first, const char16* last, uint32 count) {return _tstrncmp (first, last, count); }
    */
}

#[inline] pub fn tstrcpy<T>(
        dst: *mut T,
        src: *const T) -> *mut T {

    todo!();
        /*
            T* cp = dst;
        while ((*cp++ = *src++) != 0) // copy string
            ;
        return dst;
        */
}

#[inline] pub fn strcpy8(
        dst: *mut u8,
        src: *const u8) -> *mut u8 {
    
    todo!();
        /*
            return _tstrcpy (dst, src);
        */
}

#[inline] pub fn strcpy16(
        dst: *mut u16,
        src: *const u16) -> *mut u16 {
    
    todo!();
        /*
            return _tstrcpy (dst, src);
        */
}

#[inline] pub fn tstrncpy<T>(
        dest:   *mut T,
        source: *const T,
        count:  u32) -> *mut T {

    todo!();
        /*
            T* start = dest;
        while (count && (*dest++ = *source++) != 0) // copy string
            count--;

        if (count) // pad out with zeros
        {
            while (--count)
                *dest++ = 0;
        }
        return start;
        */
}

#[inline] pub fn strncpy8(
        dest:   *mut u8,
        source: *const u8,
        count:  u32) -> *mut u8 {
    
    todo!();
        /*
            return _tstrncpy (dest, source, count);
        */
}

#[inline] pub fn strncpy16(
        dest:   *mut u16,
        source: *const u16,
        count:  u32) -> *mut u16 {
    
    todo!();
        /*
            return _tstrncpy (dest, source, count);
        */
}

#[inline] pub fn tstrcat<T>(
        dst: *mut T,
        src: *const T) -> *mut T {

    todo!();
        /*
            T* cp = dst;

        while (*cp)
            cp++; // find end of dst

        while ((*cp++ = *src++) != 0) // Copy src to end of dst
            ;

        return dst;
        */
}

#[inline] pub fn strcat8(
        dst: *mut u8,
        src: *const u8) -> *mut u8 {
    
    todo!();
        /*
            return _tstrcat (dst, src);
        */
}

#[inline] pub fn strcat16(
        dst: *mut u16,
        src: *const u16) -> *mut u16 {
    
    todo!();
        /*
            return _tstrcat (dst, src);
        */
}

#[inline] pub fn str_8to_str16(
        dst: *mut u16,
        src: *const u8,
        n:   Option<i32>)  {
    let n: i32 = n.unwrap_or(-1);

    todo!();
        /*
            int32 i = 0;
        for (;;)
        {
            if (i == n)
            {
                dst[i] = 0;
                return;
            }

    #if BYTEORDER == kBigEndian
            char8* pChr = (char8*)&dst[i];
            pChr[0] = 0;
            pChr[1] = src[i];
    #else
            dst[i] = static_cast<char16> (src[i]);
    #endif
            if (src[i] == 0)
                break;
            i++;
        }

        while (n > i)
        {
            dst[i] = 0;
            i++;
        }
        */
}

#[inline] pub fn fid_strings_equal(
        id1: FIDString,
        id2: FIDString) -> bool {
    
    todo!();
        /*
            return (id1 && id2) ? (strcmp8 (id1, id2) == 0) : false;
        */
}

pub const printf_buffer_size: u32 = 4096;
