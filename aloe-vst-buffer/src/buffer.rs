crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fbuffer.h]

/**
  | Buffer. @ingroup adt
  | 
  | A Buffer is an object-oriented wrapper
  | for a piece of memory.
  | 
  | It adds several utility functions,
  | e.g. for managing the size of the Buffer,
  | appending or prepending values or strings
  | to it.
  | 
  | Internally it uses the standard memory
  | functions malloc(), free(), etc.
  |
  */
pub struct Buffer {
    buffer:    *mut i8,
    mem_size:  u32,
    fill_size: u32,
    delta:     u32,
}

impl Eq for Buffer {}

impl AddAssign<*const u8> for Buffer {
    
    #[inline]fn add_assign(&mut self, other: *const u8) {
        todo!();
        /*
            return appendString8 (s);
        */
    }
}

impl AddAssign<u8> for Buffer {
    
    #[inline]fn add_assign(&mut self, other: u8) {
        todo!();
        /*
            return appendString8 (c);
        */
    }
}

impl AddAssign<*const u16> for Buffer {
    
    #[inline]fn add_assign(&mut self, other: *const u16) {
        todo!();
        /*
            return appendString16 (s);
        */
    }
}

impl AddAssign<u16> for Buffer {
    
    #[inline]fn add_assign(&mut self, other: u16) {
        todo!();
        /*
            return appendString16 (c);
        */
    }
}

impl Into<*mut c_void> for Buffer {
    
    #[inline] fn into(self) -> *mut c_void {
        todo!();
        /*
            return (void*)buffer;
        */
    }
}

impl Not for Buffer {

    type Output = i32;
    
    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            return buffer == nullptr;
        */
    }
}

pub trait ToWideSring {

    fn to_wide_string(&mut self, source_code_page: i32) -> bool;
}

impl ToWideSring for Buffer {

    /**
      | Converts a Buffer's content to UTF-16
      | from a given multi-byte code page, Buffer
      | must contain char8 of given encoding.
      | 
      | -----------
      | @param[in] sourceCodePage
      | 
      | : the actual code page of the Buffer's
      | content
      | 
      | -----------
      | @return
      | 
      | true, if the conversion was successful,
      | else false
      |
      */
    // Buffer contains char8 of given encoding -> utf16
    fn to_wide_string(&mut self, source_code_page: i32) -> bool {
        
        todo!();
        /*
            }{
        */
    }
}

pub trait ToMultibyteString {

    fn to_multibyte_string(&mut self, dest_code_page: i32) -> bool;
}

impl ToMultibyteString for Buffer {

    /**
      | Converts a Buffer's content from UTF-16
      | to a given multi-byte code page, Buffer
      | must contain UTF-16 encoded characters.
      | 
      | -----------
      | @param[in] destCodePage
      | 
      | : the desired code page to convert the
      | Buffer's content to
      | 
      | -----------
      | @return
      | 
      | true, if the conversion was successful,
      | else false
      |
      */
    // Buffer contains utf16 -> char8 of given encoding
    fn to_multibyte_string(&mut self, dest_code_page: i32) -> bool {
        
        todo!();
        /*
        
        */
    }
}

pub enum BufferSwapSize 
{
    kSwap16 = 2,
    kSwap32 = 4,
    kSwap64 = 8
}

lazy_static!{
    /*
    static const uint32 buffer_default_delta = 0x1000; // 0x1000
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fbuffer.cpp]
impl Drop for Buffer {

    /**
      | Destructor - deallocates the internal
      | memory.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            if (buffer)
            ::free (buffer);
        buffer = nullptr;
        */
    }
}

impl PartialEq<Buffer> for Buffer {
    
    /**
      | Comparison operator - copies contents
      | from a given Buffer and increases the
      | size if necessary.
      | 
      | -----------
      | @param[in] buff
      | 
      | : the Buffer to be compared to
      | 
      | -----------
      | @return
      | 
      | true, if the given Buffer's content
      | is equal to this one, else false
      |
      */
    #[inline] fn eq(&self, other: &Buffer) -> bool {
        todo!();
        /*
            if (&b2 == this)
            return true;
        if (b2.getSize () != getSize ())
            return false;
        return memcmp (this->int8Ptr (), b2.int8Ptr (), getSize ()) == 0 ? true : false;
        */
    }
}

impl Add<u32> for Buffer {

    type Output = i8;
    
    /**
      | @return
      | 
      | the internal Buffer's address plus
      | the given offset i, zero if offset is
      | out of range
      |
      */
    #[inline]fn add(self, other: u32) -> Self::Output {
        todo!();
        /*
            if (i < memSize)
            return buffer + i;
        else
        {
            static int8 eof;
            eof = 0;
            return &eof;
        }
        */
    }
}

impl Default for Buffer {

    /**
      | Default constructor, allocates no
      | memory at all.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : mem_size(0),
        : fill_size(0),
        : delta(defaultDelta),

        
        */
    }
}

impl From<&Buffer> for Buffer {

    /**
      | Copy constructor - creates a new Buffer
      | from a given Buffer.
      | 
      | -----------
      | @param[in] buff
      | 
      | : the Buffer from which all memory will
      | be copied to the new one
      |
      */
    fn from(bufferr: &Buffer) -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : mem_size(bufferR.memSize),
        : fill_size(bufferR.fillSize),
        : delta(bufferR.delta),

            if (memSize == 0)
            return;

        buffer = (int8*)::malloc (memSize);
        if (buffer)
            memcpy (buffer, bufferR.buffer, memSize);
        else
            memSize = 0;
        */
    }
}
    
impl From<u32> for Buffer {

    /**
      | Constructor - creates a new Buffer with
      | a given size.
      | 
      | -----------
      | @param[in] size
      | 
      | : the size of the new Buffer to be allocated,
      | in bytes.
      |
      */
    fn from(s: u32) -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : mem_size(s),
        : fill_size(0),
        : delta(defaultDelta),

            if (memSize == 0)
            return;     
        buffer = (int8*)::malloc (memSize);
        if (!buffer)
            memSize = 0;
        */
    }
}

impl Buffer {
    
    /**
      | @return
      | 
      | the actual size of the Buffer's memory,
      | in bytes.
      |
      */
    pub fn get_size(&self) -> u32 {
        
        todo!();
        /*
            return memSize;
        */
    }

    /**
      | see \ref grow()
      |
      */
    pub fn set_max_size(&mut self, size: u32) -> bool {
        
        todo!();
        /*
            return grow (size);
        */
    }

    /**
      | @return
      | 
      | the actual fill size
      |
      */
    pub fn get_fill_size(&self) -> u32 {
        
        todo!();
        /*
            return fillSize;
        */
    }

    /**
      | sets fill size to zero
      |
      */
    #[inline] pub fn flush(&mut self)  {
        
        todo!();
        /*
            setFillSize (0);
        */
    }

    /**
      | @return
      | 
      | true, if all memory is filled up, else
      | false
      |
      */
    pub fn is_full(&self) -> bool {
        
        todo!();
        /*
            return (fillSize == memSize);
        */
    }

    /**
      | @return
      | 
      | remaining memory
      |
      */
    pub fn get_free(&self) -> u32 {
        
        todo!();
        /*
            return (memSize - fillSize);
        */
    }

    /**
      | moves all memory by given amount, grows
      | the
      | 
      | Buffer if necessary
      |
      */
    #[inline] pub fn shift_start(&mut self, amount: i32)  {
        
        todo!();
        /*
            return shiftAt (0, amount);
        */
    }

    /**
      | define the block size by which the Buffer
      | grows, see \ref grow()
      |
      */
    pub fn set_delta(&mut self, d: u32)  {
        
        todo!();
        /*
            delta = d;
        */
    }

    /* ------------------- strings   ------------------- */
    
    pub fn append_string_from_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            return put (c);
        */
    }
    
    pub fn append_string8_from_mut_u8_ptr(&mut self, s: *mut u8) -> bool {
        
        todo!();
        /*
            return appendString8 ((const char8*)s);
        */
    }
    
    pub fn append_string8_from_u8_ptr(&mut self, s: *const u8) -> bool {
        
        todo!();
        /*
            return appendString8 ((const char8*)s);
        */
    }
    
    pub fn append_string8_from_u8(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            return put ((uint8)c);
        */
    }
    
    pub fn append_string16_from_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            return put (c);
        */
    }
    
    pub fn append_string16_from_u16_ptr_mut(&mut self, s: *mut u16) -> bool {
        
        todo!();
        /*
            return appendString16 ((const char16*)s);
        */
    }
    
    pub fn prepend_string8_from_u8_ptr_mut(&mut self, s: *mut u8) -> bool {
        
        todo!();
        /*
            return prependString8 ((const char8*)s);
        */
    }
    
    pub fn prepend_string8_From_u8_ptr_mut(&mut self, s: *mut u8) -> bool {
        
        todo!();
        /*
            return prependString8((const char8*)s);
        */
    }
    
    pub fn prepend_string16_from_u16_ptr_mut(&mut self, s: *mut u16) -> bool {
        
        todo!();
        /*
            return prependString16 ((const char16*)s);
        */
    }
    
    pub fn assign_from_u8_ptr(&mut self, s: *const u8) -> bool {
        
        todo!();
        /*
            flush (); return appendString8 (s);
        */
    }
    
    pub fn assign_from_u16_ptr(&mut self, s: *const u16) -> bool {
        
        todo!();
        /*
            flush (); return appendString16 (s);
        */
    }
    
    pub fn assign_from_u8(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            flush (); return appendString8 (c);
        */
    }
    
    pub fn assign_from_from_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            flush (); return appendString16 (c);
        */
    }
    
    pub fn end_string(&mut self)  {
        
        todo!();
        /*
            put (tchar (0));
        */
    }
    
    pub fn end_string8(&mut self)  {
        
        todo!();
        /*
            put (char8 (0));
        */
    }
    
    pub fn end_string16(&mut self)  {
        
        todo!();
        /*
            put (char16 (0));
        */
    }
    
    #[inline] pub fn str_(&self) -> *mut u16 {
        
        todo!();
        /*
            return (tchar*)buffer;
        */
    }
    
    #[inline] pub fn str8(&self) -> *mut u8 {
        
        todo!();
        /*
            return (char8*)buffer;
        */
    }
    
    #[inline] pub fn str16(&self) -> *mut u16 {
        
        todo!();
        /*
            return (char16*)buffer;
        */
    }
    
    #[inline] pub fn int_8ptr(&self) -> *mut i8 {
        
        todo!();
        /*
            return (int8*)buffer;
        */
    }
    
    #[inline] pub fn uint_8ptr(&self) -> *mut u8 {
        
        todo!();
        /*
            return (uint8*)buffer;
        */
    }
    
    #[inline] pub fn int_16ptr(&self) -> *mut i16 {
        
        todo!();
        /*
            return (int16*)buffer;
        */
    }
    
    #[inline] pub fn uint_16ptr(&self) -> *mut u16 {
        
        todo!();
        /*
            return (uint16*)buffer;
        */
    }
    
    #[inline] pub fn int_32ptr(&self) -> *mut i32 {
        
        todo!();
        /*
            return (int32*)buffer;
        */
    }
    
    #[inline] pub fn uint_32ptr(&self) -> *mut u32 {
        
        todo!();
        /*
            return (uint32*)buffer;
        */
    }
    
    #[inline] pub fn float_ptr(&self) -> *mut f32 {
        
        todo!();
        /*
            return (float*)buffer;
        */
    }
    
    #[inline] pub fn double_ptr(&self) -> *mut f64 {
        
        todo!();
        /*
            return (double*)buffer;
        */
    }
    
    #[inline] pub fn wchar_ptr(&self) -> *mut u16 {
        
        todo!();
        /*
            return (char16*)buffer;
        */
    }

    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    #[inline] pub fn put_from_raw(
        &mut self, 
        p:     *mut c_void,
        count: u32

    ) -> bool {
        
        todo!();
        /*
            return put ((const void*)p , count );
        */
    }
    
    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    #[inline] pub fn put_from_raw_u8_ptr_mut(
        &mut self, 
        p:     *mut u8,
        count: u32

    ) -> bool {
        
        todo!();
        /*
            return put ((const void*)p , count );
        */
    }
    
    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    #[inline] pub fn put_from_raw_u8_mut(
        &mut self, 
        p:     *mut u8,
        count: u32

    ) -> bool {
        
        todo!();
        /*
            return put ((const void*)p , count );
        */
    }
    
    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    #[inline] pub fn put_from_raw_u8_ptr(
        &mut self, 
        p:     *const u8,
        count: u32

    ) -> bool {
        
        todo!();
        /*
            return put ((const void*)p , count );
        */
    }
    
    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    #[inline] pub fn put_from_raw_u8(
        &mut self, 
        p:     *const u8,
        count: u32

    ) -> bool {
        
        todo!();
        /*
            return put ((const void*)p , count );
        */
    }
    
    #[inline] pub fn append_string_from_u16_ptr(&mut self, s: *const u16) -> bool {
        
        todo!();
        /*
            #ifdef UNICODE
        return appendString16 (s);
    #else
        return appendString8 (s);
    #endif
        */
    }
    
    #[inline] pub fn append_string_from_u16_ptr_mut(&mut self, s: *mut u16) -> bool {
        
        todo!();
        /*
            #ifdef UNICODE
        return appendString16 (s);
    #else
        return appendString8 (s);
    #endif
        */
    }
    
    #[inline] pub fn prepend_string_from_u16_ptr(&mut self, s: *const u16) -> bool {
        
        todo!();
        /*
            #ifdef UNICODE
        return prependString16 (s);
    #else
        return prependString8 (s);
    #endif
        */
    }
    
    #[inline] pub fn prepend_string_from_u16_ptr_mut(&mut self, s: *mut u16) -> bool {
        
        todo!();
        /*
            #ifdef UNICODE
        return prependString16 (s);
    #else
        return prependString8 (s);
    #endif
        */
    }
    
    #[inline] pub fn prepend_string_from_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            #ifdef UNICODE
        return prependString16 (c);
    #else
        return prependString8 (c);
    #endif
        */
    }
    
    /**
      | Constructor - creates a new Buffer with
      | a given size and fills it all with a given
      | value.
      | 
      | -----------
      | @param[in] size
      | 
      | : the size of the new Buffer to be allocated,
      | in bytes.
      | ----------
      | @param[in] initVal
      | 
      | : the initial value the Buffer will be
      | completely filled with
      |
      */
    pub fn new(
        s:        u32,
        init_val: u8) -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : mem_size(s),
        : fill_size(0),
        : delta(defaultDelta),

            if (memSize == 0)
            return;     
        buffer = (int8*)::malloc (memSize);
        if (buffer)
            memset (buffer, initVal, memSize);
        else
            memSize = 0;
        */
    }
    
    /**
      | Constructor - creates a new Buffer with
      | a given size and copies contents from
      | optional memory pointer.
      | 
      | -----------
      | @param[in] b
      | 
      | : optional memory pointer with the size
      | of at least the given size
      | ----------
      | @param[in] size
      | 
      | : the size of the new Buffer to be allocated,
      | in bytes.
      |
      */
    pub fn new_from_raw(
        b: *const c_void,
        s: u32) -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : mem_size(s),
        : fill_size(s),
        : delta(defaultDelta),

            if (memSize == 0)
            return;
        buffer = (int8*)::malloc (memSize);
        if (buffer)
            memcpy (buffer, b, memSize);
        else
        {
            memSize = 0;
            fillSize = 0;
        }
        */
    }
    
    /**
      | Assignment operator - copies contents
      | from a given Buffer and increases the
      | size if necessary.
      | 
      | -----------
      | @param[in] buff
      | 
      | : the Buffer from which all memory will
      | be copied
      |
      */
    pub fn assign_from_buffer_ref(&mut self, b2: &Buffer)  {
        
        todo!();
        /*
            if (&b2 != this) 
        {
            setSize (b2.memSize);       
            if (b2.memSize > 0 && buffer)
                memcpy (buffer, b2.buffer, b2.memSize);
            fillSize = b2.fillSize;
            delta = b2.delta;
        }
        */
    }
    
    /**
      | copy to buffer from fillSize, and shift
      | fillSize
      |
      */
    pub fn get(&mut self, 
        b:    *mut c_void,
        size: u32) -> u32 {
        
        todo!();
        /*
            uint32 maxGet = memSize - fillSize;
        if (size > maxGet)
            size = maxGet;
        if (size > 0)
            memcpy (b, buffer + fillSize, size);
        fillSize += size;
        return size;
        */
    }
    
    /**
      | append value at end, grows Buffer if
      | necessary
      |
      */
    pub fn put_with_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            return put ((const void*)&c, sizeof (c));
        */
    }
    
    /**
      | append value at end, grows Buffer if
      | necessary
      |
      */
    pub fn put_with_u8(&mut self, byte: u8) -> bool {
        
        todo!();
        /*
            if (grow (fillSize + 1) == false)
            return false;
        
        buffer [fillSize++] = byte;
        return true;
        */
    }
    
    /**
      | append bytes from a given buffer, grows
      | Buffer if necessary
      |
      */
    pub fn put_from_raw_u32(
        &mut self, 
        to_put: *const c_void,
        s:      u32

    ) -> bool {
        
        todo!();
        /*
            if (!toPut)
            return false;

        if (grow (fillSize + s) == false)
            return false;

        memcpy (buffer + fillSize, toPut, s);
        fillSize += s;
        return true;
        */
    }
    
    /**
      | append String at end, grows Buffer if
      | necessary
      |
      */
    pub fn put_with_string(&mut self, str_: &String) -> bool {
        
        todo!();
        /*
            return put ((const void*)str.text () , (str.length () + 1) * sizeof (tchar));
        */
    }
    
    pub fn append_string8(&mut self, s: *const u8) -> bool {
        
        todo!();
        /*
            if (!s)
            return false;

        uint32 len = (uint32) strlen (s);
        return put (s, len);
        */
    }
    
    pub fn append_string16_from_u16_ptr(&mut self, s: *const u16) -> bool {
        
        todo!();
        /*
            if (!s)
            return false;
        ConstString str (s);
        uint32 len = (uint32) str.length () * sizeof (char16);
        return put (s, len);
        */
    }
    
    pub fn prepend_string8_from_u8_ptr(&mut self, s: *const u8) -> bool {
        
        todo!();
        /*
            if (!s)
            return false;

        uint32 len = (uint32) strlen (s);
        if (len > 0)
        {
            shiftStart (len);
            memcpy (buffer, s, len);
            return true;
        }
        return false;
        */
    }
    
    pub fn prepend_string16_from_u16_ptr(&mut self, s: *const u16) -> bool {
        
        todo!();
        /*
            if (!s)
            return false;

        ConstString str (s);
        uint32 len = (uint32) str.length () * sizeof (char16);
        
        if (len > 0)
        {
            shiftStart (len);
            memcpy (buffer, s, len);
            return true;
        }
        return false;
        */
    }
    
    pub fn prepend_string8_from_u8(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            shiftStart (sizeof (char));
        char* b = (char*)buffer;
        b [0] = c;
        return true;
        */
    }
    
    pub fn prepend_string16_from_u16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            shiftStart (sizeof (char16));
        char16* b = (char16*)buffer;
        b [0] = c;
        return true;
        */
    }
    
    /**
      | copies a number of bytes from one position
      | to another, the size may be adapted
      |
      */
    pub fn copy(&mut self, 
        from:  u32,
        to:    u32,
        bytes: u32) -> bool {
        
        todo!();
        /*
            if (from + bytes > memSize || bytes == 0)
            return false;
        
        if (to + bytes > memSize)
            setSize (to + bytes);
        
        if (from + bytes > to && from < to)
        {              // overlap
            Buffer tmp (buffer + from, bytes);
            memcpy (buffer + to, tmp, bytes);
        }
        else    
            memcpy (buffer + to, buffer + from, bytes);
        return true;
        */
    }
    
    pub fn make_hex_string(&mut self, result: &mut String) -> bool {
        
        todo!();
        /*
            unsigned char* data = uint8Ptr ();
        uint32 bytes = getSize ();

        if (data == nullptr || bytes == 0)
            return false;

        char8* stringBuffer = (char8*)malloc ((bytes * 2) + 1);
        if (!stringBuffer)
            return false;

        int32 count = 0;
        while (bytes > 0)
        {
            unsigned char t1 = ((*data) >> 4) & 0x0F;
            unsigned char t2 = (*data) & 0x0F;
            if (t1 < 10)
                t1 += '0';
            else
                t1 = t1 - 10 + 'A';
            if (t2 < 10)
                t2 += '0';
            else
                t2 = t2 - 10 + 'A';
            
            stringBuffer [count++] = t1;
            stringBuffer [count++] = t2;
            data++;
            bytes--;
        }
        stringBuffer [count] = 0;

        result.take ((void*)stringBuffer, false);
        return true;
        */
    }
    
    pub fn from_hex_string(&mut self, string: *const u8) -> bool {
        
        todo!();
        /*
            flush ();
        if (string == nullptr)
            return false;

        int32 len = strlen8 (string);
        if (len == 0 || ((len & 1) == 1)/*odd number*/ )
            return false;

        setSize (len / 2);
        unsigned char* data = uint8Ptr ();

        bool upper = true;
        int32 count = 0;
        while (count < len)
        {
            char c = string [count];

            unsigned char d = 0;
            if (c >= '0' && c <= '9')       d += c - '0';
            else if (c >= 'A' && c <= 'F')  d += c - 'A' + 10;
            else if (c >= 'a' && c <= 'f')  d += c - 'a' + 10;
            else return false; // no hex string

            if (upper)
                data [count >> 1] = static_cast<unsigned char> (d << 4);
            else
                data [count >> 1] += d;

            upper = !upper;
            count++;
        }
        setFillSize (len / 2);
        return true;
        */
    }
    
    /**
      | fills complete Buffer with given value
      |
      */
    pub fn set(&mut self, value: u8)  {
        
        todo!();
        /*
            if (buffer)
            memset (buffer, value, memSize);
        */
    }
    
    /**
      | sets a new fill size, does not change
      | any memory
      |
      */
    pub fn set_fill_size(&mut self, c: u32) -> bool {
        
        todo!();
        /*
            if (c <= memSize)
        {
            fillSize = c;
            return true;    
        }
        return false;
        */
    }
    
    /**
      | @return
      | 
      | always true, truncates the size of the
      | Buffer to the actual fill size
      |
      */
    pub fn truncate_to_fill_size(&mut self) -> bool {
        
        todo!();
        /*
            if (fillSize < memSize)
            setSize (fillSize);
        
        return true;
        */
    }
    
    /**
      | Increases the Buffer to the next block,
      | block size given by delta.
      | 
      | -----------
      | @param[in] memSize
      | 
      | : the new minimum size of the Buffer,
      | newSize maybe zero
      | 
      | -----------
      | @return
      | 
      | true, if the Buffer could be grown successfully,
      | else false
      |
      */
    pub fn grow(&mut self, new_size: u32) -> bool {
        
        todo!();
        /*
            if (newSize > memSize)
        {
            if (delta == 0)
                delta = defaultDelta;
            uint32 s = ((newSize + delta - 1) / delta) * delta;
            return setSize (s);
        }
        return true;
        */
    }
    
    /**
      | moves memory starting at the given position
      |
      */
    pub fn shift_at(&mut self, 
        position: u32,
        amount:   i32)  {
        
        todo!();
        /*
            if (amount > 0)
        {
            if (grow (fillSize + amount))
            {
                if (position < fillSize)
                    memmove (buffer + amount + position, buffer + position, fillSize - position);
                
                fillSize += amount;
            }
        }   
        else if (amount < 0 && fillSize > 0)
        {
            uint32 toRemove = -amount;
        
            if (toRemove < fillSize)
            {
                if (position < fillSize)
                    memmove (buffer + position, buffer + toRemove + position, fillSize - position - toRemove);
                fillSize -= toRemove;
            }
        }
        */
    }
    
    /**
      | shifts memory at start without growing
      | the buffer, so data is lost and initialized
      | with init val
      |
      */
    pub fn move_(
        &mut self, 
        amount:   i32,
        init_val: Option<u8>
    ) {

        let init_val: u8 = init_val.unwrap_or(0);
        
        todo!();
        /*
            if (memSize == 0)
            return;

        if (amount > 0)
        {
            if ((uint32)amount < memSize)
            {
                memmove (buffer + amount, buffer, memSize - amount);
                memset (buffer, initVal, amount);
            }
            else
                memset (buffer, initVal, memSize);
        }
        else
        {   
            uint32 toRemove = -amount;
            if (toRemove < memSize)
            {
                memmove (buffer, buffer + toRemove, memSize - toRemove);
                memset (buffer + memSize - toRemove, initVal, toRemove);    
            }
            else
                memset (buffer, initVal, memSize);  
        }
        */
    }
    
    /**
      | Sets a new size for this Buffer, keeping
      | as much content as possible.
      | 
      | -----------
      | @param[in] newSize
      | 
      | : the new size for the Buffer, in bytes,
      | newSize maybe zero
      | 
      | -----------
      | @return
      | 
      | true, if the new size could be adapted,
      | else false
      |
      */
    pub fn set_size(&mut self, new_size: u32) -> bool {
        
        todo!();
        /*
            if (memSize != newSize)
        {
            if (buffer)
            {
                if (newSize > 0)
                {
                    int8* newBuffer = (int8*) ::realloc (buffer, newSize);
                    if (newBuffer == nullptr)
                    {
                        newBuffer = (int8*)::malloc (newSize);
                        if (newBuffer)
                        {
                            uint32 tmp = newSize;
                            if (tmp > memSize)
                                tmp = memSize;
                            memcpy (newBuffer, buffer, tmp);
                            ::free (buffer);
                            buffer = newBuffer;
                        }
                        else
                        {
                            ::free (buffer);
                            buffer = nullptr;
                        }
                    }
                    else
                        buffer = newBuffer;
                }
                else
                {
                    ::free (buffer);
                    buffer = nullptr;
                }
            }
            else
                buffer = (int8*)::malloc (newSize);

            if (newSize > 0 && !buffer)
                memSize = 0;
            else
                memSize = newSize;
            if (fillSize > memSize)
                fillSize = memSize;
        }

        return (newSize > 0) == (buffer != nullptr);
        */
    }
    
    /**
      | set from fillSize to end
      |
      */
    pub fn fillup(&mut self, value: Option<u8>)  {

        let value: u8 = value.unwrap_or(0);
        
        todo!();
        /*
            if (getFree () > 0)
            memset (buffer + fillSize, value, getFree ());
        */
    }
    
    /**
      | swap all bytes of this Buffer by the given
      | swapSize
      |
      */
    pub fn swap(&mut self, swap_size: i16) -> bool {
        
        todo!();
        /*
            return swap (buffer, memSize, swapSize);
        */
    }
    
    /**
      | utility, swap given number of bytes
      | in given buffer by the given swapSize
      |
      */
    pub fn swap_utility(&mut self, 
        buffer:      *mut c_void,
        buffer_size: u32,
        swap_size:   i16) -> bool {
        
        todo!();
        /*
            if (swapSize != kSwap16 && swapSize != kSwap32 && swapSize != kSwap64)
            return false;
        
        if (swapSize == kSwap16)
        {
            for (uint32 count = 0 ; count < bufferSize ; count += 2)
            {
                SWAP_16 ( * (((int16*)buffer) + count) );
            }
        }
        else if (swapSize == kSwap32)
        {
            for (uint32 count = 0 ; count < bufferSize ; count += 4) 
            {
                SWAP_32 ( * (((int32*)buffer) + count) );
            }
        }
        else if (swapSize == kSwap64)
        {
            for (uint32 count = 0 ; count < bufferSize ; count += 8) 
            {
                SWAP_64 ( * (((int64*)buffer) + count) );
            }
        }

        return true;
        */
    }
    
    /**
      | takes another Buffer's memory, frees
      | the current Buffer's memory
      |
      */
    pub fn take(&mut self, from: &mut Buffer)  {
        
        todo!();
        /*
            setSize (0);
        memSize = from.memSize;
        fillSize = from.fillSize;   
        buffer = from.buffer;
        from.buffer = nullptr;
        from.memSize = 0;
        from.fillSize = 0;
        */
    }
    
    /**
      | pass the current Buffer's memory
      |
      */
    pub fn pass(&mut self) -> *mut i8 {
        
        todo!();
        /*
            int8* res = buffer;
        buffer = nullptr;
        memSize = 0;
        fillSize = 0;
        return res;
        */
    }
    
    pub fn to_wide_string(&mut self, source_code_page: i32) -> bool {
        
        todo!();
        /*
            if (getFillSize () > 0)
        {
            if (str8 () [getFillSize () - 1] != 0) // multiByteToWideString only works with 0-terminated strings
                endString8 ();

            Buffer dest (getFillSize () * sizeof (char16));
            int32 result = String::multiByteToWideString (dest.str16 (), buffer, dest.getFree () / sizeof (char16), sourceCodePage);
            if (result > 0)
            {
                dest.setFillSize ((result - 1) * sizeof (char16));
                take (dest);
                return true;
            }
            return false;
        }
        return true;
        */
    }
    
    pub fn to_multibyte_string(&mut self, dest_code_page: i32) -> bool {
        
        todo!();
        /*
            if (getFillSize () > 0)
        {
            int32 textLength = getFillSize () / sizeof (char16); // wideStringToMultiByte only works with 0-terminated strings
            if (str16 () [textLength - 1] != 0)
                endString16 ();

            Buffer dest (getFillSize ());
            int32 result = String::wideStringToMultiByte (dest.str8 (), str16 (), dest.getFree (), destCodePage);
            if (result > 0)
            {
                dest.setFillSize (result - 1);
                take (dest);
                return true;
            }
            return false;
        }
        return true;
        */
    }
}
