crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ustring.h]

/**
  | UTF-16 string class without buffer
  | management.
  | 
  | -----------
  | @note
  | 
  | that some characters are encoded in
  | 2 UTF16 code units (surrogate pair),
  | this means that getLength returns the
  | number of code unit, not the count of
  | character!
  |
  */
pub struct UString {

    this_buffer: *mut u16,

    /**
      | size in code unit (not in byte!)
      |
      */
    this_size:   i32,
}

impl Into<*mut u16> for UString {
    
    /**
      | cast to char16*
      |
      */
    #[inline] fn into(self) -> *mut u16 {
        todo!();
        /*
            return thisBuffer;
        */
    }
}

impl UString {

    /**
      | Construct from UTF-16 string, size
      | is in code unit (count of char16)
      |
      */
    pub fn new(
        buffer: *mut u16,
        size:   i32) -> Self {
    
        todo!();
        /*
        : this_buffer(buffer),
        : this_size(size),
        */
    }

    /**
      | returns buffer size
      |
      */
    pub fn get_size(&self) -> i32 {
        
        todo!();
        /*
            return thisSize;
        */
    }

    pub fn assign(
        &mut self, 
        src:      *const u8,
        src_size: Option<i32>

    ) -> &mut UString {

        let src_size: i32 = src_size.unwrap_or(-1);

        todo!();
        /*
            return fromAscii (src, srcSize);
        */
    }
}

/**
  | UTF-16 string with fixed buffer size.
  |
  */
pub struct UStringBuffer<const MAX_SIZE: usize> {
    base: UString,
    data: [u16; MAX_SIZE],
}

impl<const MAX_SIZE: usize> Default for UStringBuffer<MAX_SIZE> {
    
    fn default() -> Self {
        todo!();
        /*
        : string(data, MAX_SIZE),

            data[0] = 0;
        */
    }
}

impl<const MAX_SIZE: usize> UStringBuffer<MAX_SIZE> {

    /**
      | Construct from UTF-16 string.
      |
      */
    pub fn new_from_raw_u16_ptr(
        src:      *const u16,
        src_size: Option<i32>) -> Self {
        let src_size: i32 = src_size.unwrap_or(-1);
        todo!();
        /*
        : string(data, MAX_SIZE),

            data[0] = 0;
            if (src)
                assign (src, srcSize);
        */
    }

    /**
      | Construct from ASCII string.
      |
      */
    pub fn new_from_raw_u8_ptr(
        src:      *const u8,
        src_size: Option<i32>

    ) -> Self {

        let src_size: i32 = src_size.unwrap_or(-1);

        todo!();
        /*
        : string(data, MAX_SIZE),

            data[0] = 0;
            if (src)
                fromAscii (src, srcSize);
        */
    }
}

/**
  | 128 character UTF-16 string
  |
  */
pub type UString128 = UStringBuffer<128>;

/**
  | 256 character UTF-16 string
  |
  */
pub type UString256 = UStringBuffer<256>;

macro_rules! ustring {
    ($asciiString:ident) => {
        /*
           Steinberg::UString256 (asciiString)
           */
    }
}

macro_rules! ustringsize {
    ($var:ident) => {
        /*
           (sizeof (var) / sizeof (Steinberg::char16))
           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ustring.cpp]

#[cfg(SMTG_OS_LINUX)]
pub type Converter = WStringConvert<CodecvtUtf8Utf16<u16>,u16>;

#[cfg(SMTG_OS_LINUX)]
pub fn converter() -> &mut Converter {
    
    todo!();
        /*
            static Converter instance;
        return instance;
        */
}

/**
  | Copy strings of different character
  | width.
  |
  */
pub fn string_copy<TDstChar, TSrcChar>(
        dst:      *mut TDstChar,
        dst_size: i32,
        src:      *const TSrcChar,
        src_size: Option<i32>)  {

    let src_size: i32 = src_size.unwrap_or(-1);

    todo!();
    /*
            int32 count = dstSize;
        if (srcSize >= 0 && srcSize < dstSize)
            count = srcSize;
        for (int32 i = 0; i < count; i++)
        {
            dst[i] = (TDstChar)src[i];
            if (src[i] == 0)
                break;
        }
        dst[dstSize - 1] = 0;
        */
}

/**
  | Find length of null-terminated string,
  | i.e. StringLength (L"ABC\0") => 3
  |
  */
pub fn string_length<TSrcChar>(
        src:      *const TSrcChar,
        src_size: Option<i32>) -> i32 {
    let src_size: i32 = src_size.unwrap_or(-1);
    todo!();
        /*
            if (srcSize == 0)
            return 0;
        int32 length = 0;
        while (src[length])
        {
            length++;
            if (srcSize > 0 && length >= srcSize)
                break;
        }
        return length;
        */
}


impl UString {
    
    /**
      | Returns length of string (in code unit).
      | Note this is not the count of character!
      |
      */
    pub fn get_length(&self) -> i32 {
        
        todo!();
        /*
            return StringLength<char16> (thisBuffer, thisSize);
        */
    }
    
    /**
      | Copy from UTF-16 buffer (srcSize is
      | in code unit (count of char16)).
      |
      */
    pub fn assign_u16(&mut self, 
        src:      *const u16,
        src_size: Option<i32>) -> &mut UString {

        let src_size: i32 = src_size.unwrap_or(-1);
        
        todo!();
        /*
            StringCopy<char16, char16> (thisBuffer, thisSize, src, srcSize);
        return *this;
        */
    }
    
    /**
      | Append UTF-16 buffer (srcSize is in
      | code unit (count of char16)).
      |
      */
    pub fn append(&mut self, 
        src:      *const u16,
        src_size: Option<i32>) -> &mut UString {

        let src_size: i32 = src_size.unwrap_or(-1);
        
        todo!();
        /*
            int32 length = getLength ();
        StringCopy<char16, char16> (thisBuffer + length, thisSize - length, src, srcSize);
        return *this;
        */
    }
    
    /**
      | Copy to UTF-16 buffer (dstSize is in
      | code unit (count of char16)).
      |
      */
    pub fn copy_to(&self, 
        dst:      *mut u16,
        dst_size: i32) -> &UString {
        
        todo!();
        /*
            StringCopy<char16, char16> (dst, dstSize, thisBuffer, thisSize);
        return *this;
        */
    }
    
    /**
      | Copy from ASCII string (srcSize is in
      | code unit (count of char16)).
      |
      */
    pub fn from_ascii(&mut self, 
        src:      *const u8,
        src_size: Option<i32>) -> &mut UString {
        
        let src_size: i32 = src_size.unwrap_or(-1);

        todo!();
        /*
            StringCopy<char16, char> (thisBuffer, thisSize, src, srcSize);
        return *this;
        */
    }
    
    /**
      | Copy to ASCII string.
      |
      */
    pub fn to_ascii(&self, 
        dst:      *mut u8,
        dst_size: i32) -> &UString {
        
        todo!();
        /*
            StringCopy<char, char16> (dst, dstSize, thisBuffer, thisSize);
        return *this;
        */
    }
    
    /**
      | Scan float from string.
      |
      */
    pub fn scan_float(&self, value: &mut f64) -> bool {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
        return swscanf ((const wchar_t*)thisBuffer, L"%lf", &value) != -1;

    #elif TARGET_API_MAC_CARBON
        CFStringRef cfStr = CFStringCreateWithBytes (0, (const UInt8 *)thisBuffer, getLength () * 2, kCFStringEncodingUTF16, false);
        if (cfStr)
        {
            value = CFStringGetDoubleValue (cfStr);
            CFRelease (cfStr);
            return true;
        }
        return false;

    #elif SMTG_OS_LINUX
        auto str = converter ().to_bytes (thisBuffer);
        return sscanf (str.data (), "%lf", &value) == 1;

    #else
    #warning Implement me
        // implement me!
        return false;
    #endif
        */
    }
    
    /**
      | Print float to string.
      |
      */
    pub fn print_float(
        &mut self, 
        value:     f64,
        precision: Option<i32>

    ) -> bool {

        let precision: i32 = precision.unwrap_or(4);
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
        return swprintf ((wchar_t*)thisBuffer, L"%.*lf", precision, value) != -1;
    #elif SMTG_OS_MACOS
        bool result = false;
        CFStringRef cfStr = CFStringCreateWithFormat (0, 0, CFSTR("%.*lf"), precision, value);
        if (cfStr)
        {
            memset (thisBuffer, 0, thisSize);
            CFRange range = {0, CFStringGetLength (cfStr)};
            CFStringGetBytes (cfStr, range, kCFStringEncodingUTF16, 0, false, (UInt8*)thisBuffer, thisSize, 0);
            CFRelease (cfStr);
            return true;
        }
        return result;
    #elif SMTG_OS_LINUX
        auto utf8Buffer = reinterpret_cast<char*> (thisBuffer);
        auto len = snprintf (utf8Buffer, thisSize, "%.*lf", precision, value);
        if (len > 0)
        {
            auto utf16Buffer = reinterpret_cast<char16*> (thisBuffer);
            utf16Buffer[len] = 0;
            while (--len >= 0)
            {
                utf16Buffer[len] = utf8Buffer[len];
            }
            return true;
        }
        return false;
    #else
    #warning Implement me
        // implement me!
        return false;
    #endif
        */
    }
    
    /**
      | Scan integer from string.
      |
      */
    pub fn scan_int(&self, value: &mut i64) -> bool {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
        return swscanf ((const wchar_t*)thisBuffer, L"%I64d", &value) != -1;

    #elif SMTG_OS_MACOS
        CFStringRef cfStr = CFStringCreateWithBytes (0, (const UInt8 *)thisBuffer, getLength () * 2, kCFStringEncodingUTF16, false);
        if (cfStr)
        {
            value = CFStringGetIntValue (cfStr);
            CFRelease (cfStr);
            return true;
        }
        return false;

    #elif SMTG_OS_LINUX
        auto str = converter ().to_bytes (thisBuffer);
        return sscanf (str.data (), "%lld", &value) == 1;

    #else
    #warning Implement me
        // implement me!
        return false;
    #endif
        */
    }
    
    /**
      | Print integer to string.
      |
      */
    pub fn print_int(&mut self, value: i64) -> bool {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
        return swprintf ((wchar_t*)thisBuffer, L"%I64d", value) != -1;

    #elif SMTG_OS_MACOS
        CFStringRef cfStr = CFStringCreateWithFormat (0, 0, CFSTR("%lld"), value);
        if (cfStr)
        {
            memset (thisBuffer, 0, thisSize);
            CFRange range = {0, CFStringGetLength (cfStr)};
            CFStringGetBytes (cfStr, range, kCFStringEncodingUTF16, 0, false, (UInt8*)thisBuffer, thisSize, 0);
            CFRelease (cfStr);
            return true;
        }
        return false;
    #elif SMTG_OS_LINUX
        auto utf8Buffer = reinterpret_cast<char*> (thisBuffer);
        auto len = snprintf (utf8Buffer, thisSize, "%lld", value);
        if (len > 0)
        {
            auto utf16Buffer = reinterpret_cast<char16*> (thisBuffer);
            utf16Buffer[len] = 0;
            while (--len >= 0)
            {
                utf16Buffer[len] = utf8Buffer[len];
            }
            return true;
        }
        return false;

    #else
    #warning Implement me
        // implement me!
        return false;
    #endif
        */
    }
}
