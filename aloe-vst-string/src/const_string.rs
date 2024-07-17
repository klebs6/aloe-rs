crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fstring.h]

pub union ConstStringU {
    buffer:   *mut c_void,
    buffer8:  *mut u8,
    buffer16: *mut u16,
}

impl ConstStringU {
    
    #[inline] pub fn text8(&self) -> *const u8 {
        
        todo!();
        /*
            return (!isWide && buffer8) ? buffer8: kEmptyString8;
        */
    }
    
    #[inline] pub fn text16(&self) -> *const u16 {
        
        todo!();
        /*
            return (isWide && buffer16) ? buffer16 : kEmptyString16;
        */
    }
    
    #[inline] pub fn get_char8(&self, index: u32) -> u8 {
        
        todo!();
        /*
            if (index < len && buffer8 && !isWide)
            return buffer8[index];
        return 0;
        */
    }
    
    #[inline] pub fn get_char16(&self, index: u32) -> u16 {
        
        todo!();
        /*
            if (index < len && buffer16 && isWide)
            return buffer16[index];
        return 0;
        */
    }
    
    /**
      | Returns character of type tchar at 'index'
      |
      */
    #[inline] pub fn get_char(&self, index: u32) -> u16 {
        
        todo!();
        /*
            #ifdef UNICODE
        return getChar16 (index);
    #else
        return getChar8 (index);
    #endif
        */
    }
    
    /**
      | Returns character of type tchar at 'index',
      | no conversion!
      |
      */
    #[inline] pub fn get_char_at(&self, index: u32) -> u16 {
        
        todo!();
        /*
            #ifdef UNICODE
        if (isWide)
            return getChar16 (index);
    #endif

        return static_cast<tchar> (getChar8 (index));
        */
    }
    
    /**
      | Returns result of scanInt64
      |
      */
    #[inline] pub fn get_number(&self) -> i64 {
        
        todo!();
        /*
            int64 tmp = 0;
        scanInt64 (tmp);
        return tmp;
        */
    }
    
    /**
      | Converts string of type char8 to int32
      | value
      |
      */
    #[inline] pub fn scan_int32_8(
        &mut self, 
        text:        *const u8,
        value:       &mut i32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            int64 tmp;
        if (scanInt64_8 (text, tmp, scanToEnd))
        {
            value = (int32)tmp;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char16 to int32
      | value
      |
      */
    #[inline] pub fn scan_int32_16(
        &mut self, 
        text:        *const u16,
        value:       &mut i32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            int64 tmp;
        if (scanInt64_16 (text, tmp, scanToEnd))
        {
            value = (int32)tmp;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type tchar to int32
      | value
      |
      */
    #[inline] pub fn scan_int32(
        &mut self, 
        text:        *const u16,
        value:       &mut i32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            int64 tmp;
        if (scanInt64 (text, tmp, scanToEnd))
        {
            value = (int32)tmp;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char8 to int32
      | value
      |
      */
    #[inline] pub fn scan_uint32_8(
        &mut self, 
        text:        *const u8,
        value:       &mut u32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            uint64 tmp;
        if (scanUInt64_8 (text, tmp, scanToEnd))
        {
            value = (uint32)tmp;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char16 to int32
      | value
      |
      */
    #[inline] pub fn scan_uint32_16(
        &mut self, 
        text:        *const u16,
        value:       &mut u32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            uint64 tmp;
        if (scanUInt64_16 (text, tmp, scanToEnd))
        {
            value = (uint32)tmp;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type tchar to int32
      | value
      |
      */
    #[inline] pub fn scan_uint32(
        &mut self, 
        text:        *const u16,
        value:       &mut u32,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            uint64 tmp;
        if (scanUInt64 (text, tmp, scanToEnd))
        {
            value = (uint32)tmp;
            return true;
        }
        return false;
        */
    }
}

/**
  | Invariant String. @ingroup adt
  | 
  | A base class which provides methods
  | to work with its member string. Neither
  | of the operations allows modifying
  | the member string and that is why all
  | operation are declared as const.
  | 
  | There are operations for access, comparison,
  | find, numbers and conversion.
  | 
  | Almost all operations exist in three
  | versions for char8, char16 and the polymorphic
  | type tchar. The type tchar can either
  | be char8 or char16 depending on whether
  | UNICODE is activated or not.
  |
  */
pub struct ConstString {
    u: ConstStringU,
    b: ConstStringBitfield,
}

#[bitfield]
struct ConstStringBitfield {
    len:     B30,
    is_wide: B1,
    _zpad:   B1,
}

impl Into<*mut u8> for ConstString {
    
    /**
      | Returns pointer to string of type char8
      | (no modification allowed)
      |
      */
    #[inline] fn into(self) -> *mut u8 {
        todo!();
        /*
            return text8 ();
        */
    }
}

impl Into<*mut u16> for ConstString {
    
    /**
      | Returns pointer to string of type char16(no
      | modification allowed)
      |
      */
    #[inline] fn into(self) -> *mut u16 {
        todo!();
        /*
            return text16 ();
        */
    }
}


impl Index<i16> for ConstString {

    type Output = u16;
    
    /**
      | Returns character at 'idx'
      |
      */
    fn index(&self, idx: i16) -> &Self::Output {
        todo!();
        /*
            return getChar (static_cast<uint32> (idx));
        */
    }
}


impl Index<i64> for ConstString {

    type Output = u16;
    
    fn index(&self, idx: i64) -> &Self::Output {
        todo!();
        /*
            return getChar (static_cast<uint32> (idx));
        */
    }
}

impl Index<i32> for ConstString {

    type Output = u16;
    
    fn index(&self, idx: i32) -> &Self::Output {
        todo!();
        /*
            return getChar (static_cast<uint32> (idx));
        */
    }
}

impl Index<u16> for ConstString {

    type Output = u16;
    
    fn index(&self, idx: u16) -> &Self::Output {
        todo!();
        /*
            return getChar (idx);
        */
    }
}

impl Index<u64> for ConstString {

    type Output = u16;
    
    fn index(&self, idx: u64) -> &Self::Output {
        todo!();
        /*
            return getChar (static_cast<uint32> (idx));
        */
    }
}

impl Index<u32> for ConstString {

    type Output = u16;
    
    fn index(&self, idx: u32) -> &Self::Output {
        todo!();
        /*
            return getChar (idx);
        */
    }
}

impl StringInterface for ConstString {

    #[inline] fn text(&self) -> *const u16 {
        
        todo!();
        /*
            #ifdef UNICODE
        return text16 ();
    #else
        return text8 ();
    #endif
        */
    }
    
    #[inline] fn text8(&self) -> *const u8 {
        
        todo!();
        /*
            if (isWide && !isEmpty ())
            checkToMultiByte (); // this should be avoided, since it can lead to information loss

        return ConstString::text8 ();
        */
    }
    
    #[inline] fn text16(&self) -> *const u16 {
        
        todo!();
        /*
            if (!isWide && !isEmpty ())
        {
            const_cast<String&> (*this).toWideString ();
        }
        return ConstString::text16 ();
        */
    }
    
    #[inline] fn get_char8(&self, index: u32) -> u8 {
        
        todo!();
        /*
            if (isWide && !isEmpty ())
            checkToMultiByte (); // this should be avoided, since it can lead to information loss

        return ConstString::getChar8 (index);
        */
    }
    
    #[inline] fn get_char16(&self, index: u32) -> u16 {
        
        todo!();
        /*
            if (!isWide && !isEmpty ())
        {
            const_cast<String&> (*this).toWideString ();
        }
        return ConstString::getChar16 (index);
        */
    }
}

pub trait StringInterface {

    /**
      | Return length of string
      |
      */
    fn length(&self) -> i32 {
        
        todo!();
        /*
            return static_cast<int32> (len);
        */
    }

    /**
      | Returns pointer to string of type char8
      |
      */
    #[inline] fn text8(&self) -> *const u8;

    /**
      | Returns pointer to string of type char16
      |
      */
    #[inline] fn text16(&self) -> *const u16;

    /**
      | Returns pointer to string of type tchar
      |
      */
    #[inline] fn text(&self) -> *const u16;

    /**
      | Returns pointer to string of type void
      |
      */
    #[inline] fn ptr(&self) {
        
        todo!();
        /*
            return buffer;
        */
    }

    /**
      | Returns character of type char16 at
      | 'index'
      |
      */
    #[inline] fn get_char8(&self, index: u32) -> u8;

    /**
      | Returns character of type char8 at 'index'
      |
      */
    #[inline] fn get_char16(&self, index: u32) -> u16;
}

#[cfg(SMTG_OS_MACOS)]
impl ToCFStringRef for ConstString {

    /**
      | CFString conversion
      |
      */
    fn to_cf_string_ref(&self, 
        encoding:          u32,
        mutable_cf_string: bool)  {

        let encoding: u32 = encoding.unwrap_or(0xFFFF);
        let mutable_cf_string: bool = mutable_cf_string.unwrap_or(false);
        
        todo!();
        /*
            if (mutableCFString)
        {
            CFMutableStringRef str = CFStringCreateMutable (kCFAllocator, 0);
            if (isWide)
            {
                CFStringAppendCharacters (str, (const UniChar *)buffer16, len);
                return str;
            }
            else
            {
                if (encoding == 0xFFFF)
                    encoding = kCFStringEncodingASCII;
                CFStringAppendCString (str, buffer8, encoding);
                return str;
            }
        }
        else
        {
            if (isWide)
            {
                if (encoding == 0xFFFF)
                    encoding = kCFStringEncodingUnicode;
                return (void*)CFStringCreateWithBytes (kCFAllocator, (const unsigned char*)buffer16, len * 2, encoding, false);
            }
            else
            {
                if (encoding == 0xFFFF)
                    encoding = kCFStringEncodingASCII;
                if (buffer8)
                    return (void*)CFStringCreateWithCString (kCFAllocator, buffer8, encoding);
                else
                    return (void*)CFStringCreateWithCString (kCFAllocator, "", encoding);
            }
        }
        return 0;
        */
    }

}

/* ------- String concatenation functions.   ------- */

impl Add<&ConstString> for ConstString {

    type Output = String;
    
    fn add(self, other: &ConstString) -> Self::Output {
        todo!();
        /*
            return String (s1).append (s2);
        */
    }
}

impl Ord for ConstString {
    
    fn cmp(&self, other: &ConstString) -> std::cmp::Ordering {
        todo!();
        /*
            return (s1.compare (s2) < 0) ? true : false;
        */
    }
}

impl PartialOrd<ConstString> for ConstString {
    fn partial_cmp(&self, other: &ConstString) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<ConstString> for ConstString {
    
    fn eq(&self, other: &ConstString) -> bool {
        todo!();
        /*
            return (s1.compare (s2) == 0) ? true : false;
        */
    }
}

impl Eq for ConstString {}

impl From<&FVariant> for ConstString {

    /**
      | Assign a string from FVariant
      |
      */
    fn from(var: &FVariant) -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : len(0),
        : is_wide(0),

            switch (var.getType ())
        {
            case FVariant::kString8:
                buffer8 = (char8*)var.getString8 ();
                len = buffer8 ? strlen8 (buffer8) : 0;
                isWide = false;
                break;

            case FVariant::kString16:
                buffer16 = (char16*)var.getString16 ();
                len = buffer16 ? strlen16 (buffer16) : 0;
                isWide = true;
                break;
        }
        */
    }
}
    
impl Default for ConstString {

    fn default() -> Self {
    
        todo!();
        /*
        : buffer(nullptr),
        : len(0),
        : is_wide(0),
        */
    }
}

impl ConstString {
    

    /* -------------------- access  -------------------- */

    /**
      | Return true if string is empty
      |
      */
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return buffer == nullptr || len == 0;
        */
    }

    #[inline] pub fn test_char(
        &self, 
        index: u32,
        c:     u8

    ) -> bool {
        
        todo!();
        /*
            return testChar8 (index, c);
        */
    }
    
    #[inline] pub fn test_char_u16(
        &self, 
        index: u32,
        c:     u16

    ) -> bool {
        
        todo!();
        /*
            return testChar16 (index, c);
        */
    }
    
    #[inline] pub fn hash(&self, tsize: u32) -> u32 {
        
        todo!();
        /*
            return isWide ? hashString16 (buffer16, tsize) : hashString8 (buffer8, tsize) ;
        */
    }

    /* -------------------- compare  -------------------- */

    /**
      | @name Find first occurrence of n characters
      | of str in this (n=-1: all) ending at endIndex
      | (endIndex = -1: all)
      |
      */
    #[inline] pub fn find_first(
        &self, 
        str_:      &ConstString,
        n:         Option<i32>,
        m:         Option<StringCompareMode>,
        end_index: Option<i32>

    ) -> i32 {

        let n: i32 = n.unwrap_or(-1);
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);

        todo!();
        /*
            return findNext (0, str, n, m, endIndex);
        */
    }
    
    #[inline] pub fn find_first_u8(
        &self, 
        c:         u8,
        m:         Option<StringCompareMode>,
        end_index: Option<i32>

    ) -> i32 {

        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);

        todo!();
        /*
            return findNext (0, c, m, endIndex);
        */
    }
    
    #[inline] pub fn find_first_u16(
        &self, 
        c:         u16,
        m:         Option<StringCompareMode>,
        end_index: Option<i32>

    ) -> i32 {

        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);

        todo!();
        /*
            return findNext (0, c, m, endIndex);
        */
    }

    /**
      | Find last occurrence of n characters
      | of str in this (n=-1: all)
      |
      */
    #[inline] pub fn find_last(
        &self, 
        str_: &ConstString,
        n:    Option<i32>,
        m:    Option<StringCompareMode>

    ) -> i32 {

        let n: i32 = n.unwrap_or(-1);
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            return findPrev (-1, str, n, m);
        */
    }
    
    #[inline] pub fn find_last_u8(
        &self, 
        c: u8,
        m: Option<StringCompareMode>

    ) -> i32 {
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            return findPrev (-1, c, m);
        */
    }
    
    #[inline] pub fn find_last_u16(
        &self, 
        c: u16,
        m: Option<StringCompareMode>

    ) -> i32 {
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            return findPrev (-1, c, m);
        */
    }

    /* -------------------- numbers  -------------------- */

    /* ---------------- static methods  ---------------- */

    /* ------------------ conversion  ------------------ */
    
    /**
      | Returns true if string is wide
      |
      */
    pub fn is_wide_string(&self) -> bool {
        
        todo!();
        /*
            return isWide != 0;
        */
    }

    /**
      | Assign from string of type char8 (length=-1:
      | all)
      |
      */
    pub fn new_with_raw_u8_ptr(
        str_:   *const u8,
        length: Option<i32>

    ) -> Self {

        let length: i32 = length.unwrap_or(-1);
    
        todo!();
        /*

            : buffer8 ((char8*)str)
    , len (length < 0 ? (str ? static_cast<uint32> (strlen (str)) : 0) : length)
    , isWide (0)
        */
    }
    
    /**
      | Assign from string of type char16 (length=-1:
      | all)
      |
      */
    pub fn new_from_raw_u16(
        str_:   *const u16,
        length: Option<i32>

    ) -> Self {

        let length: i32 = length.unwrap_or(-1);
    
        todo!();
        /*

            : buffer16 ((char16*)str)
    , len (length < 0 ? (str ? strlen16 (str) : 0) : length)
    , isWide (1)
        */
    }
    
    /**
      | Copy constructor (length=-1: all).
      |
      */
    pub fn new_copy(
        str_:   &ConstString,
        offset: Option<i32>,
        length: Option<i32>
    ) -> Self {

        let offset: i32 = offset.unwrap_or(0);
        let length: i32 = length.unwrap_or(-1);
    
        todo!();
        /*

            : buffer (str.buffer)
    , len (length < 0 ? (str.len - (offset > 0 ? offset : 0)) : length)
    , isWide (str.isWide)

        if (offset > 0)
        {
            if (isWide)
                buffer16 += offset;
            else
                buffer8 += offset;
        }
        */
    }
    
    /**
      | Returns true if character is equal at
      | position 'index'
      |
      */
    pub fn test_char8(&self, 
        index: u32,
        c:     u8) -> bool {
        
        todo!();
        /*
            if (index >= len)
            return c == 0;
        if (isWide)
        {
            // make c wide
            char8 src[] = {c, 0};
            char16 dest[2] = {0};
            if (multiByteToWideString (dest, src, 2) > 0)
                return buffer16[index] == dest[0];
            return false;
        }
        return buffer8[index] == c;
        */
    }
    
    pub fn test_char16(&self, 
        index: u32,
        c:     u16) -> bool {
        
        todo!();
        /*
            if (index >= len)
            return c == 0;
        if (!isWide)
        {
            // make c ansi
            char16 src[] = {c, 0};
            char8 dest[8] = {0};
            if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                return buffer8[index] == dest[0];
            return false;
        }
        return buffer16[index] == c;
        */
    }
    
    /**
      | Get n characters long substring starting
      | at index (n=-1: until end)
      |
      */
    pub fn extract(
        &self, 
        result: &mut String,
        idx:    u32,
        n:      Option<i32>
    ) -> bool {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (len == 0|| idx >= len)
            return false;

        if ((idx + n > len) || n < 0)
            n = len - idx;

        if (isWide)
            result.assign (buffer16 + idx, n);
        else
            result.assign (buffer8 + idx, n);

        return true;
        */
    }
    
    pub fn copy_to8(
        &self, 
        str_: *mut u8,
        idx:  Option<u32>,
        n:    Option<i32>

    ) -> i32 {

        let idx: u32 = idx.unwrap_or(0);
        let n:   i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (!str)
            return 0;

        if (isWide)
        {
            String tmp (text16 ());
            if (tmp.toMultiByte () == false)
                return 0;
            return tmp.copyTo8 (str, idx, n);
        }

        if (isEmpty () || idx >= len || !buffer8)
        {
            str[0] = 0;
            return 0;
        }

        if ((idx + n > len) || n < 0)
            n = len - idx;

        memcpy (str, &(buffer8[idx]), n * sizeof (char8));
        str[n] = 0;
        return n;
        */
    }
    
    pub fn copy_to16(
        &self, 
        str_: *mut u16,
        idx:  Option<u32>,
        n:    Option<i32>
    ) -> i32 {

        let idx: u32 = idx.unwrap_or(0);
        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (!str)
            return 0;

        if (!isWide)
        {
            String tmp (text8 ());
            if (tmp.toWideString () == false)
                return 0;
            return tmp.copyTo16 (str, idx, n);
        }
        
        if (isEmpty () || idx >= len || !buffer16)
        {
            str[0] = 0;
            return 0;
        }

        if ((idx + n > len) || n < 0)
            n = len - idx;
        
        memcpy (str, &(buffer16[idx]), n * sizeof (char16));
        str[n] = 0;
        return n;
        */
    }
    
    pub fn copy_to_raw(
        &self, 
        str_: *mut u16,
        idx:  Option<u32>,
        n:    Option<i32>

    ) -> i32 {

        let idx: u32 = idx.unwrap_or(0);
        let n:   i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            #ifdef UNICODE
        return copyTo16 (str, idx, n);
    #else
        return copyTo8 (str, idx, n);
    #endif
        */
    }
    
    /**
      | Copies whole member string
      |
      */
    pub fn copy_to_result(&self, result: *mut dyn IStringResult)  {
        
        todo!();
        /*
            if (isWideString () == false)
        {
            result->setText (text8 ()); 
        }
        else
        {
            FUnknownPtr<IString> iStr (result);
            if (iStr)
            {
                iStr->setText16 (text16 ());
            }
            else
            {
                String tmp (*this);
                tmp.toMultiByte ();
                result->setText (tmp.text8 ());
            }
        }
        */
    }
    
    /**
      | Copies whole member string
      |
      */
    pub fn copy_to(&self, string: &mut dyn IString)  {
        
        todo!();
        /*
            if (isWideString ())
            string.setText16 (text16 ());
        else
            string.setText8 (text8 ());
        */
    }
    
    /**
      | Compare n characters of str with n characters
      | of this (return: see above)
      |
      */
    pub fn compare_n(
        &self, 
        str_: &ConstString,
        n:    i32,
        mode: Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (n == 0)
            return 0;

        if (str.isEmpty ())
        {
            if (isEmpty ())
                return 0;
            return 1;
        }
        else if (isEmpty ())
            return -1;

        if (!isWide && !str.isWide)
        {
            if (n < 0)
            {
                if (isCaseSensitive (mode))
                    return strcmp (*this, str);
                else
                    return stricmp (*this, str);
            }
            else
            {
                if (isCaseSensitive (mode))
                    return strncmp (*this, str, n);
                else
                    return strnicmp (*this, str, n);
            }
        }
        else if (isWide && str.isWide)
        {
            if (n < 0)
            {
                if (isCaseSensitive (mode))
                    return strcmp16 (*this, str);
                else
                    return stricmp16 (*this, str);
            }
            else
            {
                if (isCaseSensitive (mode))
                    return strncmp16 (*this, str, n);
                else
                    return strnicmp16 (*this, str, n);
            }
        }
        return compareAt (0, str, n, mode);
        */
    }
    
    /**
      | Compare all characters of str with this
      | (return: see above)
      |
      */
    pub fn compare(
        &self, 
        str_: &ConstString,
        mode: Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            return compare (str, -1, mode);
        */
    }
    
    /**
      | Compare n characters of str with n characters
      | of this starting at index (return: see
      | above)
      |
      */
    pub fn compare_at(
        &self, 
        index: u32,
        str_:  &ConstString,
        n:     Option<i32>,
        mode:  Option<StringCompareMode>
    ) -> i32 {

        let n: i32 = n.unwrap_or(-1);
        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (n == 0)
            return 0;

        if (str.isEmpty ())
        {
            if (isEmpty ())
                return 0;
            return 1;
        }
        else if (isEmpty ())
            return -1;

        if (!isWide && !str.isWide)
        {
            char8* toCompare = buffer8;
            if (index > 0)
            {
                if (index >= len)
                {
                    if (str.isEmpty ())
                        return 0;
                    return -1;
                }
                toCompare += index;
            }

            if (n < 0)
            {
                if (isCaseSensitive (mode))
                    return strcmp (toCompare, str);
                else
                    return stricmp (toCompare, str);
            }
            else
            {
                if (isCaseSensitive (mode))
                    return strncmp (toCompare, str, n);
                else
                    return strnicmp (toCompare, str, n);
            }
        }
        else if (isWide && str.isWide)
        {
            char16* toCompare = buffer16;
            if (index > 0)
            {
                if (index >= len)
                {
                    if (str.isEmpty ())
                        return 0;
                    return -1;
                }
                toCompare += index;
            }

            if (n < 0)
            {
                if (isCaseSensitive (mode))
                    return strcmp16 (toCompare, str.text16 ());
                else
                    return stricmp16 (toCompare, str.text16 ());
            }
            else
            {
                if (isCaseSensitive (mode))
                    return strncmp16 (toCompare, str.text16 (), n);
                else
                    return strnicmp16 (toCompare, str.text16 (), n);
            }
        }
        else
        {
            if (isWide)
            {
                String tmp (str.text8 ());
                if (tmp.toWideString () == false)
                    return -1;
                return compareAt (index, tmp, n, mode);
            }
            else
            {
                String tmp (text8 ());
                if (tmp.toWideString () == false)
                    return 1;
                return tmp.compareAt (index, str, n, mode);
            }
        }
        */
    }

    pub fn natural_compare(
        &self, 
        str_: &ConstString,
        mode: Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            if (str.isEmpty ())
        {
            if (isEmpty ())
                return 0;
            return 1;
        }
        else if (isEmpty ())
            return -1;

        if (!isWide && !str.isWide)
            return strnatcmp8 (buffer8, str.text8 (), isCaseSensitive (mode));
        else if (isWide && str.isWide)
            return strnatcmp16 (buffer16, str.text16 (), isCaseSensitive (mode));
        else
        {
            if (isWide)
            {
                String tmp (str.text8 ());
                tmp.toWideString ();
                return strnatcmp16 (buffer16, tmp.text16 (), isCaseSensitive (mode));
            }
            else
            {
                String tmp (text8 ());
                tmp.toWideString ();
                return strnatcmp16 (tmp.text16 (), str.text16 (), isCaseSensitive (mode));
            }
        }
        */
    }
    
    /**
      | Check if this starts with str
      |
      */
    pub fn starts_with(
        &self, 
        str_: &ConstString,
        mode: Option<StringCompareMode>

    ) -> bool {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            if (str.isEmpty ())
        {
            return isEmpty ();
        }
        else if (isEmpty ())
        {
            return false;
        }
        if (length () < str.length ())
        {
            return false;
        }
        if (!isWide && !str.isWide)
        {
            if (isCaseSensitive (mode))
                return strncmp (buffer8, str.buffer8, str.length ()) == 0;
            return strnicmp (buffer8, str.buffer8, str.length ()) == 0;
        }
        else if (isWide && str.isWide)
        {
            if (isCaseSensitive (mode))
                return strncmp16 (buffer16, str.buffer16, str.length ()) == 0;
            return strnicmp16 (buffer16, str.buffer16, str.length ()) == 0;
        }
        else if (isWide)
        {
            String tmp (str.text8 ());
            tmp.toWideString ();
            if (tmp.length () > length ())
                return false;
            if (isCaseSensitive (mode))
                return strncmp16 (buffer16, tmp.buffer16, tmp.length ()) == 0;
            return strnicmp16 (buffer16, tmp.buffer16, tmp.length ()) == 0;
        }
        else
        {
            String tmp (text8 ());
            tmp.toWideString ();
            if (str.length () > tmp.length ())
                return false;
            if (isCaseSensitive (mode))
                return strncmp16 (tmp.buffer16, str.buffer16, str.length ()) == 0;
            return strnicmp16 (tmp.buffer16, str.buffer16, str.length ()) == 0;
        }
        */
    }
    
    /**
      | Check if this ends with str
      |
      */
    pub fn ends_with(
        &self, 
        str_: &ConstString,
        mode: Option<StringCompareMode>

    ) -> bool {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            if (str.isEmpty ())
        {
            return isEmpty ();
        }
        else if (isEmpty ())
        {
            return false;
        }
        if (length () < str.length ())
        {
            return false;
        }
        if (!isWide && !str.isWide)
        {
            if (isCaseSensitive (mode))
                return strncmp (buffer8 + (length () - str.length ()), str.buffer8, str.length ()) == 0;
            return strnicmp (buffer8 + (length () - str.length ()), str.buffer8, str.length ()) == 0;
        }
        else if (isWide && str.isWide)
        {
            if (isCaseSensitive (mode))
                return strncmp16 (buffer16 + (length () - str.length ()), str.buffer16, str.length ()) == 0;
            return strnicmp16 (buffer16 + (length () - str.length ()), str.buffer16, str.length ()) == 0;
        }
        else if (isWide)
        {
            String tmp (str.text8 ());
            tmp.toWideString ();
            if (tmp.length () > length ())
                return false;
            if (isCaseSensitive (mode))
                return strncmp16 (buffer16 + (length () - tmp.length ()), tmp.buffer16, tmp.length ()) == 0;
            return strnicmp16 (buffer16 + (length () - tmp.length ()), tmp.buffer16, tmp.length ()) == 0;
        }
        else
        {
            String tmp (text8 ());
            tmp.toWideString ();
            if (str.length () > tmp.length ())
                return false;
            if (isCaseSensitive (mode))
                return strncmp16 (tmp.buffer16 + (tmp.length () - str.length ()), str.buffer16, str.length ()) == 0;
            return strnicmp16 (tmp.buffer16 + (tmp.length () - str.length ()), str.buffer16, str.length ()) == 0;
        }
        */
    }
    
    /**
      | Check if this contains str
      |
      */
    pub fn contains(
        &self, 
        str_: &ConstString,
        m:    Option<StringCompareMode>
    ) -> bool {

        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            return findFirst (str, -1, m) != -1;
        */
    }
    
    /**
      | @name Find next occurrence of n characters
      | of str starting at startIndex in this
      | (n=-1: all) ending at endIndex (endIndex
      | = -1: all)
      |
      */
    pub fn find_next_occurrence_of_n_characters_of_str(
        &self, 
        start_index: i32,
        str_:        &ConstString,
        n:           Option<i32>,
        mode:        Option<StringCompareMode>,
        end_index:   Option<i32>

    ) -> i32 {

        let n: i32 = n.unwrap_or(-1);
        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);
        
        todo!();
        /*
            uint32 endLength = len;
        if (endIndex > -1 && (uint32)endIndex < len)
            endLength = endIndex + 1;

        if (isWide && str.isWide)
        {
            if (startIndex < 0)
                startIndex = 0;

            uint32 stringLength = str.length ();
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

            if (n > 0)
            {
                uint32 i = 0;

                if (isCaseSensitive (mode))
                {
                    for (i = startIndex; i < endLength; i++)
                        if (strncmp16 (buffer16 + i, str, n) == 0)
                            return i;
                }
                else
                {
                    for (i = startIndex; i < endLength; i++)
                        if (strnicmp16 (buffer16 + i, str, n) == 0)
                            return i;
                }
            }
            return -1;
        }
        else if (!isWide && !str.isWide)
        {
            uint32 stringLength = str.length ();
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

            if (startIndex < 0)
                startIndex = 0;

            if (n > 0)
            {
                uint32 i = 0;

                if (isCaseSensitive (mode))
                {
                    for (i = startIndex; i < endLength; i++)
                        if (strncmp (buffer8 + i, str, n) == 0)
                            return i;
                }
                else
                {
                    for (i = startIndex; i < endLength; i++)
                        if (strnicmp (buffer8 + i, str, n) == 0)
                            return i;
                }
            }
            return -1;
        }
        String tmp;
        if (isWide)
        {
            tmp = str.text8 ();
            tmp.toWideString ();
            return findNext (startIndex, tmp, n , mode, endIndex);
        }
        tmp = text8 ();
        tmp.toWideString ();
        return tmp.findNext (startIndex, str, n, mode, endIndex);
        */
    }
    
    pub fn find_next_u8(
        &self, 
        start_index: i32,
        c:           u8,
        mode:        Option<StringCompareMode>,
        end_index:   Option<i32>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);
        
        todo!();
        /*
            uint32 endLength = len;
        if (endIndex > -1 && (uint32)endIndex < len)
            endLength = endIndex + 1;

        if (isWide)
        {
            char8 src[] = {c, 0};
            char16 dest[8] = {0};
            if (multiByteToWideString (dest, src, 2) > 0)
                return findNext (startIndex, dest[0], mode, endIndex);
            return -1;
        }

        if (startIndex < 0)
            startIndex = 0;
        uint32 i;

        if (isCaseSensitive (mode))
        {
            for (i = startIndex; i < endLength; i++)
            {
                if (buffer8[i] == c)
                    return i;
            }
        }
        else
        {
            c = toLower (c);
            for (i = startIndex; i < endLength; i++)
            {
                if (toLower (buffer8[i]) == c)
                    return i;
            }
        }
        return -1;
        */
    }
    
    pub fn find_next_u16(
        &self, 
        start_index: i32,
        c:           u16,
        mode:        Option<StringCompareMode>,
        end_index:   Option<i32>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        let end_index: i32 = end_index.unwrap_or(-1);
        
        todo!();
        /*
            uint32 endLength = len;
        if (endIndex > -1 && (uint32)endIndex < len)
            endLength = endIndex + 1;

        if (!isWide)
        {
            char16 src[] = {c, 0};
            char8 dest[8] = {0};
            if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                return findNext (startIndex, dest[0], mode, endIndex);

            return -1;
        }

        uint32 i;
        if (startIndex < 0)
            startIndex = 0;

        if (isCaseSensitive (mode))
        {
            for (i = startIndex; i < endLength; i++)
            {
                if (buffer16[i] == c)
                    return i;
            }
        }
        else
        {
            c = toLower (c);
            for (i = startIndex; i < endLength; i++)
            {
                if (toLower (buffer16[i]) == c)
                    return i;
            }
        }
        return -1;
        */
    }
    
    /**
      | @name Find previous occurrence of n
      | characters of str starting at startIndex
      | in this (n=-1: all)
      |
      */
    pub fn find_prev_u8(
        &self, 
        start_index: Option<i32>,
        c:           u8,
        mode:        Option<StringCompareMode>

    ) -> i32 {

        let start_index: i32 = start_index.unwrap_or(-1);
        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (len == 0)
            return -1;

        if (isWide)
        {
            char8 src[] = {c, 0};
            char16 dest[8] = {0};
            if (multiByteToWideString (dest, src, 2) > 0)
                return findPrev (startIndex, dest[0], mode);
            return -1;
        }

        if (startIndex < 0 || startIndex > (int32)len)
            startIndex = len;

        int32 i;

        if (isCaseSensitive (mode))
        {
            for (i = startIndex; i >= 0; i--)
            {
                if (buffer8[i] == c)
                    return i;
            }
        }
        else
        {
            c = toLower (c);
            for (i = startIndex; i >= 0; i--)
            {
                if (toLower (buffer8[i]) == c)
                    return i;
            }
        }
        return -1;
        */
    }
    
    pub fn find_prev_u16(
        &self, 
        start_index: i32,
        c:           u16,
        mode:        Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (len == 0)
            return -1;

        if (!isWide)
        {
            char16 src[] = {c, 0};
            char8 dest[8] = {0};
            if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                return findPrev (startIndex, dest[0], mode);

            return -1;
        }

        if (startIndex < 0 || startIndex > (int32)len)
            startIndex = len;

        int32 i;

        if (isCaseSensitive (mode))
        {
            for (i = startIndex; i >= 0; i--)
            {
                if (buffer16[i] == c)
                    return i;
            }
        }
        else
        {
            c = toLower (c);
            for (i = startIndex; i >= 0; i--)
            {
                if (toLower (buffer16[i]) == c)
                    return i;
            }
        }
        return -1;
        */
    }
    
    pub fn find_prev_str(
        &self, 
        start_index: i32,
        str_:        &ConstString,
        n:           i32,
        mode:        Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (isWide && str.isWide)
        {
            uint32 stringLength = str.length ();
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

            if (startIndex < 0 || startIndex >= (int32)len)
                startIndex = len - 1;

            if (n > 0)
            {
                int32 i = 0;

                if (isCaseSensitive (mode))
                {
                    for (i = startIndex; i >= 0; i--)
                        if (strncmp16 (buffer16 + i, str, n) == 0)
                            return i;
                }
                else
                {
                    for (i = startIndex; i >= 0; i--)
                        if (strnicmp16 (buffer16 + i, str, n) == 0)
                            return i;
                }
            }
            return -1;
        }
        else if (!isWide && !str.isWide)
        {
            uint32 stringLength = str.length ();
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

            if (startIndex < 0 || startIndex >= (int32)len)
                startIndex = len - 1;

            if (n > 0)
            {
                int32 i = 0;

                if (isCaseSensitive (mode))
                {
                    for (i = startIndex; i >= 0; i--)
                        if (strncmp (buffer8 + i, str, n) == 0)
                            return i;
                }
                else
                {
                    for (i = startIndex; i >= 0; i--)
                        if (strnicmp (buffer8 + i, str, n) == 0)
                            return i;
                }
            }
            return -1;
        }
        if (isWide)
        {
            String tmp (str.text8 ());
            tmp.toWideString ();
            return findPrev (startIndex, tmp, n, mode);
        }
        String tmp (text8 ());
        tmp.toWideString ();
        return tmp.findPrev (startIndex, str, n, mode);
        */
    }
    
    /**
      | Counts occurences of c within this starting
      | at index
      |
      */
    pub fn count_occurences_of_u8(
        &self, 
        c:           u8,
        start_index: u32,
        mode:        Option<StringCompareMode>

    ) -> i32 {
        
        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);

        todo!();
        /*
            if (isWide)
        {
            char8 src[] = {c, 0};
            char16 dest[8] = {0};
            if (multiByteToWideString (dest, src, 2) > 0)
                return countOccurences (dest[0], startIndex, mode);
            return -1;
        }

        int32 result = 0;
        int32 next = startIndex;
        while (true)
        {
            next = findNext (next, c, mode);
            if (next >= 0)
            {
                next++;
                result++;
            }
            else
                break;
        }
        return result;
        */
    }
    
    pub fn count_occurences_of_u16(
        &self, 
        c:           u16,
        start_index: u32,
        mode:        Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (!isWide)
        {
            char16 src[] = {c, 0};
            char8 dest[8] = {0};
            if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                return countOccurences (dest[0], startIndex, mode);

            return -1;
        }
        int32 result = 0;
        int32 next = startIndex;
        while (true)
        {
            next = findNext (next, c, mode);
            if (next >= 0)
            {
                next++;
                result++;
            }
            else
                break;
        }
        return result;
        */
    }
    
    /**
      | Returns position of first different
      | character
      |
      */
    pub fn get_first_different(
        &self, 
        str_: &ConstString,
        mode: Option<StringCompareMode>

    ) -> i32 {

        let mode: StringCompareMode = mode.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (str.isWide != isWide)
        {
            if (isWide)
            {
                String tmp (str.text8 ());
                if (tmp.toWideString () == false)
                    return -1;
                return getFirstDifferent (tmp, mode);
            }
            else
            {
                String tmp (text8 ());
                if (tmp.toWideString () == false)
                    return -1;
                return tmp.getFirstDifferent (str, mode);
            }
        }

        uint32 len1 = len;
        uint32 len2 = str.len;
        uint32 i;

        if (isWide)
        {
            if (isCaseSensitive (mode))
            {
                for (i = 0; i <= len1 && i <= len2; i++)
                {
                    if (buffer16[i] != str.buffer16[i])
                        return i;
                }
            }
            else
            {
                for (i = 0; i <= len1 && i <= len2; i++)
                {
                    if (toLower (buffer16[i]) != toLower (str.buffer16[i]))
                        return i;
                }
            }
        }
        else
        {
            if (isCaseSensitive (mode))
            {
                for (i = 0; i <= len1 && i <= len2; i++)
                {
                    if (buffer8[i] != str.buffer8[i])
                        return i;
                }
            }
            else
            {
                for (i = 0; i <= len1 && i <= len2; i++)
                {
                    if (toLower (buffer8[i]) != toLower (str.buffer8[i]))
                        return i;
                }
            }
        }
        return -1;
        */
    }
    
    /**
      | Converts string to int64 value starting
      | at offset
      |
      */
    pub fn scan_int64(
        &self, 
        value:       &mut i64,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        if (isWide)
            return scanInt64_16 (buffer16 + offset, value, scanToEnd);
        else
            return scanInt64_8 (buffer8 + offset, value, scanToEnd);
        */
    }
    
    /**
      | Converts string to uint64 value starting
      | at offset
      |
      */
    pub fn scan_uint64(
        &self, 
        value:       &mut u64,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        if (isWide)
            return scanUInt64_16 (buffer16 + offset, value, scanToEnd);
        else
            return scanUInt64_8 (buffer8 + offset, value, scanToEnd);
        */
    }
    
    /**
      | Converts string to hex/uint8 value
      | starting at offset
      |
      */
    pub fn scan_hex(
        &self, 
        value:       &mut u8,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        if (isWide)
            return scanHex_16 (buffer16 + offset, value, scanToEnd);
        else
            return scanHex_8 (buffer8 + offset, value, scanToEnd);
        */
    }
    
    /**
      | Converts string to int32 value starting
      | at offset
      |
      */
    pub fn scan_int32(
        &self, 
        value:       &mut i32,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        if (isWide)
            return scanInt32_16 (buffer16 + offset, value, scanToEnd);
        else
            return scanInt32_8 (buffer8 + offset, value, scanToEnd);
        */
    }
    
    /**
      | Converts string to uint32 value starting
      | at offset
      |
      */
    pub fn scan_uint32(
        &self, 
        value:       &mut u32,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        if (isWide)
            return scanUInt32_16 (buffer16 + offset, value, scanToEnd);
        else
            return scanUInt32_8 (buffer8 + offset, value, scanToEnd);
        */
    }
    
    /**
      | Converts string of type char8 to int64
      | value
      |
      */
    pub fn scan_int64_8(
        &mut self, 
        text:        *const u8,
        value:       &mut i64,
        scan_to_end: Option<bool>
    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            while (text && text[0])
        {
            if (sscanf (text, "%" FORMAT_INT64A, &value) == 1)
                return true;
            else if (scanToEnd == false)
                return false;
            text++;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char16 to int64
      | value
      |
      */
    pub fn scan_int64_16(
        &mut self, 
        text:        *const u16,
        value:       &mut i64,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (text && text[0])
        {
            String str (text);
            str.toMultiByte (kCP_Default);
            return scanInt64_8 (str, value, scanToEnd);
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char8 to uint64
      | value
      |
      */
    pub fn scan_uint64_8(
        &mut self, 
        text:        *const u8,
        value:       &mut u64,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            while (text && text[0])
        {
            if (sscanf (text, "%" FORMAT_UINT64A, &value) == 1)
                return true;
            else if (scanToEnd == false)
                return false;
            text++;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char16 to uint64
      | value
      |
      */
    pub fn scan_uint64_16(
        &mut self, 
        text:        *const u16,
        value:       &mut u64,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (text && text[0])
        {
            String str (text);
            str.toMultiByte (kCP_Default);
            return scanUInt64_8 (str, value, scanToEnd);
        }
        return false;
        */
    }
    
    /**
      | Converts string of type tchar to int64
      | value
      |
      */
    pub fn scan_int64_from_tchar(
        &mut self, 
        text:        *const u16,
        value:       &mut i64,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            #ifdef UNICODE
        return scanInt64_16 (text, value,scanToEnd);
    #else
        return scanInt64_8 (text, value, scanToEnd);
    #endif
        */
    }
    
    /**
      | Converts string of type tchar to uint64
      | value
      |
      */
    pub fn scan_uint64_from_tchar(
        &mut self, 
        text:        *const u16,
        value:       &mut u64,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            #ifdef UNICODE
        return scanUInt64_16 (text, value, scanToEnd);
    #else
        return scanUInt64_8 (text, value, scanToEnd);
    #endif
        */
    }
    
    /**
      | Converts string of type char8 to hex/unit8
      | value
      |
      */
    pub fn scan_hex_8(
        &mut self, 
        text:        *const u8,
        value:       &mut u8,
        scan_to_end: Option<bool>
    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            while (text && text[0])
        {
            unsigned int v; // scanf expects an unsigned int for %x
            if (sscanf (text, "%x", &v) == 1)
            {
                value = (uint8)v;
                return true;
            }
            else if (scanToEnd == false)
                return false;
            text++;
        }
        return false;
        */
    }
    
    /**
      | Converts string of type char16 to hex/unit8
      | value
      |
      */
    pub fn scan_hex_16(
        &mut self, 
        text:        *const u16,
        value:       &mut u8,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (text && text[0])
        {
            String str (text);
            str.toMultiByte (kCP_Default); // scanf uses default codepage
            return scanHex_8 (str, value, scanToEnd);
        }
        return false;
        */
    }
    
    /**
      | Converts string of type tchar to hex/unit8
      | value
      |
      */
    pub fn scan_hex_tchar(
        &mut self, 
        text:        *const u16,
        value:       &mut u8,
        scan_to_end: Option<bool>

    ) -> bool {

        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            #ifdef UNICODE
        return scanHex_16 (text, value, scanToEnd);
    #else
        return scanHex_8 (text, value, scanToEnd);
    #endif
        */
    }
    
    /**
      | Converts string to double value starting
      | at offset
      |
      */
    pub fn scan_float(
        &self, 
        value:       &mut f64,
        offset:      Option<u32>,
        scan_to_end: Option<bool>

    ) -> bool {

        let offset: u32 = offset.unwrap_or(0);
        let scan_to_end: bool = scan_to_end.unwrap_or(true);
        
        todo!();
        /*
            if (isEmpty () || offset >= len)
            return false;

        String str (*this);
        int32 pos = -1;
        if (isWide)
        {
            if ((pos = str.findNext (offset, STR(','))) >= 0 && ((uint32)pos) >= offset)
                str.setChar (pos, STR('.'));

            str.toMultiByte (kCP_Default); // scanf uses default codepage
        }
        else
        {
            if ((pos = str.findNext (offset, ',')) >= 0 && ((uint32)pos) >= offset)
                str.setChar (pos, '.');
        }

        const char8* txt = str.text8 () + offset;
        while (txt && txt[0])
        {
            if (sscanf (txt, "%lf", &value) == 1)
                return true;
            else if (scanToEnd == false)
                return false;
            txt++;
        }
        return false;
        */
    }
    
    pub fn to_lower_u16(&mut self, c: u16) -> u16 {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
            WCHAR temp[2] = {c, 0};
            ::CharLowerW (temp);
            return temp[0];
        #elif SMTG_OS_MACOS
            // only convert characters which in lowercase are also single characters
            UniChar characters [2] = {0};
            characters[0] = c;
            CFMutableStringRef str = CFStringCreateMutableWithExternalCharactersNoCopy (kCFAllocator, characters, 1, 2, kCFAllocatorNull);
            if (str)
            {
                CFStringLowercase (str, NULL);
                CFRelease (str);
                if (characters[1] == 0)
                    return characters[0];
            }
            return c;
        #elif SMTG_OS_LINUX
        assert(false && "DEPRECATED No Linux implementation");
            return c;
        #else
            return towlower (c);
        #endif
        */
    }
    
    pub fn to_upper_u16(&mut self, c: u16) -> u16 {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
            WCHAR temp[2] = {c, 0};
            ::CharUpperW (temp);
            return temp[0];
        #elif SMTG_OS_MACOS
            // only convert characters which in uppercase are also single characters (don't translate a sharp-s which would result in SS)
            UniChar characters [2] = {0};
            characters[0] = c;
            CFMutableStringRef str = CFStringCreateMutableWithExternalCharactersNoCopy (kCFAllocator, characters, 1, 2, kCFAllocatorNull);
            if (str)
            {
                CFStringUppercase (str, NULL);
                CFRelease (str);
                if (characters[1] == 0)
                    return characters[0];
            }
            return c;
        #elif SMTG_OS_LINUX
        assert(false && "DEPRECATED No Linux implementation");
            return c;
        #else
            return towupper (c);
        #endif
        */
    }
    
    /**
      | Converts to lower case
      |
      */
    pub fn to_lower_u8(&mut self, c: u8) -> u8 {
        
        todo!();
        /*
            if ((c >= 'A') && (c <= 'Z'))
            return c + ('a' - 'A');
        #if SMTG_OS_WINDOWS
            CHAR temp[2] = {c, 0};
            ::CharLowerA (temp);
            return temp[0];
        #else
            return static_cast<char8> (tolower (c));
        #endif
        */
    }
    
    /**
      | Converts to upper case
      |
      */
    pub fn to_upper_u8(&mut self, c: u8) -> u8 {
        
        todo!();
        /*
            if ((c >= 'a') && (c <= 'z'))
            return c - ('a' - 'A');
        #if SMTG_OS_WINDOWS
            CHAR temp[2] = {c, 0};
            ::CharUpperA (temp);
            return temp[0];
        #else
            return static_cast<char8> (toupper (c));
        #endif
        */
    }
    
    /**
      | Returns true if character is a space
      |
      */
    pub fn is_u8_char_space(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return isspace (character) != 0;
        */
    }
    
    /**
      | @copydoc isCharSpace(const char8)
      |
      */
    pub fn is_u16_char_space(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            switch (character)
        {
            case 0x0020:
            case 0x00A0:
            case 0x2002:
            case 0x2003:
            case 0x2004:
            case 0x2005:
            case 0x2006:
            case 0x2007:
            case 0x2008:
            case 0x2009:
            case 0x200A:
            case 0x200B:
            case 0x202F:
            case 0x205F:
            case 0x3000:
                return true;
        }
        return false;
        */
    }
    
    /**
      | Returns true if character is an alphabetic
      | character
      |
      */
    pub fn is_u8_char_alpha(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return isalpha (character) != 0;
        */
    }
    
    /**
      | @copydoc isCharAlpha(const char8)
      |
      */
    pub fn is_u16_char_alpha(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return iswalpha (character) != 0;
        */
    }
    
    /**
      | Returns true if character is an alphanumeric
      | character
      |
      */
    pub fn is_u8_char_alpha_num(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return isalnum (character) != 0;
        */
    }
    
    /**
      | @copydoc isCharAlphaNum(const char8)
      |
      */
    pub fn is_u16_char_alpha_num(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return iswalnum (character) != 0; // this may not work on macOSX when another locale is set inside the c-lib
        */
    }
    
    /**
      | Returns true if character is a number
      |
      */
    pub fn is_u8_char_digit(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return isdigit (character) != 0;
        */
    }
    
    /**
      | @copydoc isCharDigit(const char8)
      |
      */
    pub fn is_u16_char_digit(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return iswdigit (character) != 0;   // this may not work on macOSX when another locale is set inside the c-lib
        */
    }
    
    /**
      | Returns true if character is in ASCII
      | range
      |
      */
    pub fn is_u8_char_ascii(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return character >= 0;
        */
    }
    
    /**
      | Returns true if character is in ASCII
      | range
      |
      */
    pub fn is_u16_char_ascii(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return character < 128;
        */
    }
    
    pub fn is_u8_char_upper(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return toUpper (character) == character;
        */
    }
    
    pub fn is_u16_char_upper(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return toUpper (character) == character;
        */
    }
    
    pub fn is_u8_char_lower(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return toLower (character) == character;
        */
    }
    
    pub fn is_u16_char_lower(&mut self, character: u16) -> bool {
        
        todo!();
        /*
            return toLower (character) == character;
        */
    }
    
    /**
      | Returns true if character at position
      | is a digit
      |
      */
    pub fn is_digit(&self, index: u32) -> bool {
        
        todo!();
        /*
            if (isEmpty () || index >= len)
            return false;

        if (isWide)
            return ConstString::isCharDigit (buffer16[index]);
        else
            return ConstString::isCharDigit (buffer8[index]);
        */
    }
    
    /**
      | Returns start index of trailing number
      |
      */
    pub fn get_trailing_number_index(&self, width: Option<u32>) -> i32 {

        let width: u32 = width.unwrap_or(0);
        
        todo!();
        /*
            if (isEmpty ())
            return -1;

        int32 endIndex = len - 1;
        int32 i = endIndex;
        while (isDigit ((uint32) i) && i >= 0)
            i--;

        // now either all are digits or i is on the first non digit
        if (i < endIndex)
        {
            if (width > 0 && (endIndex - i != static_cast<int32> (width)))
                return -1;

            return i + 1;
        }

        return -1;
        */
    }
    
    /**
      | Returns result of scanInt64 or the fallback
      |
      */
    pub fn get_trailing_number(&self, fallback: Option<i64>) -> i64 {

        let fallback: i64 = fallback.unwrap_or(0);
        
        todo!();
        /*
            int32 index = getTrailingNumberIndex ();

        int64 number = 0;

        if (index >= 0)
            if (scanInt64 (number, index))
                return number;

        return fallback;
        */
    }
    
    pub fn to_variant(&self, var: &mut FVariant)  {
        
        todo!();
        /*
            if (isWide)
        {
            var.setString16 (buffer16);
        }
        else
        {
            var.setString8 (buffer8);
        }
        */
    }
    
    /**
      | Checks if all characters in string are
      | in ascii range
      |
      */
    pub fn is_ascii_string(&self) -> bool {
        
        todo!();
        /*
            uint32 i;
        if (isWide)
        {
            for (i = 0; i < len; i++)
                if (ConstString::isCharAscii (buffer16 [i]) == false)
                    return false;
        }
        else
        {
            for (i = 0; i < len; i++)
                if (ConstString::isCharAscii (buffer8 [i]) == false)
                    return false;
        }
        return true;
        */
    }
    
    /**
      | If dest is zero, this returns the maximum
      | number of bytes needed to convert source
      |
      */
    pub fn multi_byte_to_wide_string(
        &mut self, 
        dest:             *mut u16,
        source:           *const u8,
        char_count:       i32,
        source_code_page: Option<MBCodePage>

    ) -> i32 {

        let source_code_page = source_code_page.unwrap_or(MBCodePage::CP_Default);
        
        todo!();
        /*
            if (source == nullptr || source[0] == 0)
        {
            if (dest && charCount > 0)
            {
                dest[0] = 0;
            }
            return 0;
        }
        int32 result = 0;
    #if SMTG_OS_WINDOWS
        result = MultiByteToWideChar (sourceCodePage, MB_ERR_INVALID_CHARS, source, -1, dest, charCount);
    #endif

    #if SMTG_OS_MACOS
        CFStringRef cfStr =
            (CFStringRef)::toCFStringRef (source, MBCodePageToCFStringEncoding (sourceCodePage));
        if (cfStr)
        {
            CFRange range = {0, CFStringGetLength (cfStr)};
            CFIndex usedBytes;
            if (CFStringGetBytes (cfStr, range, kCFStringEncodingUnicode, ' ', false, (UInt8*)dest,
                                  charCount * 2, &usedBytes) > 0)
            {
                result = static_cast<int32> (usedBytes / 2 + 1);
                if (dest)
                    dest[usedBytes / 2] = 0;
            }

            CFRelease (cfStr);
        }
    #endif

    #if SMTG_OS_LINUX
        if (sourceCodePage == kCP_ANSI || sourceCodePage == kCP_US_ASCII || sourceCodePage == kCP_Utf8)
        {
            if (dest == nullptr)
            {
                auto state = std::mbstate_t ();
                auto maxChars = charCount ? charCount : std::numeric_limits<int32>::max () - 1;
                result = converterFacet ().length (state, source, source + strlen (source), maxChars);
            }
            else
            {
                auto utf16Str = converter ().from_bytes (source);
                if (!utf16Str.empty ())
                {
                    result = std::min<int32> (charCount, utf16Str.size ());
                    memcpy (dest, utf16Str.data (), result * sizeof (char16));
                    dest[result] = 0;
                }
            }
        }
        else
        {
            assert(false && "DEPRECATED No Linux implementation");
        }

    #endif

        SMTG_ASSERT (result > 0)
        return result;
        */
    }
    
    /**
      | If dest is zero, this returns the maximum
      | number of bytes needed to convert source
      |
      */
    pub fn wide_string_to_multi_byte(
        &mut self, 
        dest:           *mut u8,
        wide_string:    *const u16,
        char_count:     i32,
        dest_code_page: Option<MBCodePage>

    ) -> i32 {

        let dest_code_page = dest_code_page.unwrap_or(MBCodePage::CP_Default);
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
        return WideCharToMultiByte (destCodePage, 0, wideString, -1, dest, charCount, nullptr, nullptr);

    #elif SMTG_OS_MACOS
        int32 result = 0;
        if (wideString != 0)
        {
            if (dest)
            {
                CFStringRef cfStr = CFStringCreateWithCharactersNoCopy (kCFAllocator, (const UniChar*)wideString, strlen16 (wideString), kCFAllocatorNull);
                if (cfStr)
                {
                    if (fromCFStringRef (dest, charCount, cfStr, MBCodePageToCFStringEncoding (destCodePage)))
                        result = static_cast<int32> (strlen (dest) + 1);
                    CFRelease (cfStr);
                }
            }
            else
            {
                return static_cast<int32> (CFStringGetMaximumSizeForEncoding (strlen16 (wideString), MBCodePageToCFStringEncoding (destCodePage)));
            }
        }
        return result;

    #elif SMTG_OS_LINUX
        int32 result = 0;
        if (destCodePage == kCP_Utf8)
        {
            if (dest == nullptr)
            {
                auto maxChars = charCount ? charCount : tstrlen (wideString);
                result = converterFacet ().max_length () * maxChars;
            }
            else
            {
                auto utf8Str = converter ().to_bytes (wideString);
                if (!utf8Str.empty ())
                {
                    result = std::min<int32> (charCount, utf8Str.size ());
                    memcpy (dest, utf8Str.data (), result * sizeof (char8));
                    dest[result] = 0;
                }
            }
        }
        else if (destCodePage == kCP_ANSI || destCodePage == kCP_US_ASCII)
        {
            if (dest == nullptr)
            {
                result = strlen16 (wideString) + 1;
            }
            else
            {
                int32 i = 0;
                for (; i < charCount; ++i)
                {
                    if (wideString[i] == 0)
                        break;
                    if (wideString[i] <= 0x007F)
                        dest[i] = wideString[i];
                    else
                        dest[i] = '_';
                }
                dest[i] = 0;
                result = i;
            }
        }
        else
        {
            assert(false && "DEPRECATED No Linux implementation");
        }
        return result;

    #else
    #warning DEPRECATED No Linux implementation
        assert(false && "DEPRECATED No Linux implementation");
        return 0;
    #endif
        */
    }
    
    /**
      | On PC only kUnicodeNormC is working
      |
      */
    pub fn is_normalized(&mut self, n: Option<UnicodeNormalization>) -> bool {

        let n: UnicodeNormalization = n.unwrap_or(UnicodeNormalization::kUnicodeNormC);
        
        todo!();
        /*
            if (isWide == false)
            return false;

    #if SMTG_OS_WINDOWS
    #ifdef UNICODE
        if (n != kUnicodeNormC)
            return false;
        uint32 normCharCount = static_cast<uint32> (FoldString (MAP_PRECOMPOSED, buffer16, len, nullptr, 0));
        return (normCharCount == len);
    #else
        return false; 
    #endif

    #elif SMTG_OS_MACOS
        if (n != kUnicodeNormC)
            return false;

        CFStringRef cfStr = (CFStringRef)toCFStringRef ();
        CFIndex charCount = CFStringGetLength (cfStr);
        CFRelease (cfStr);
        return (charCount == len);
    #else
        return false;
    #endif
        */
    }
}
