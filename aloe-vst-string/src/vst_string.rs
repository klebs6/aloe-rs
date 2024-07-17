crate::ix!();

pub enum StringCharGroup {
    kSpace, 
    kNotAlphaNum, 
    kNotAlpha
}

/**
  | String. @ingroup adt
  | 
  | Extends class ConstString by operations
  | which allow modifications.
  | 
  | \see ConstString
  |
  */
pub struct String {
    base: ConstString,
}

impl Drop for String {

    fn drop(&mut self) {
        todo!();
        /*
            if (buffer)
            resize (0, false);
        */
    }
}

impl AddAssign<&String> for String {
    
    #[inline]fn add_assign(&mut self, other: &String) {
        todo!();
        /*
            return append (str);
        */
    }
}

impl AddAssign<&ConstString> for String {
    
    #[inline]fn add_assign(&mut self, other: &ConstString) {
        todo!();
        /*
            return append (str);
        */
    }
}

impl AddAssign<*const u8> for String {
    
    #[inline]fn add_assign(&mut self, other: *const u8) {
        todo!();
        /*
            return append (str);
        */
    }
}

impl AddAssign<*const u16> for String {
    
    #[inline]fn add_assign(&mut self, other: *const u16) {
        todo!();
        /*
            return append (str);
        */
    }
}

impl AddAssign<u8> for String {
    
    #[inline]fn add_assign(&mut self, other: u8) {
        todo!();
        /*
            return append (c);
        */
    }
}

impl AddAssign<u16> for String {
    
    #[inline]fn add_assign(&mut self, other: u16) {
        todo!();
        /*
            return append (c);
        */
    }
}

impl Default for String {

    fn default() -> Self {
    
        todo!();
        /*

            isWide = kWideStringDefault ? 1 : 0;
        */
    }
}

impl From<&FVariant> for String {

    /**
      | assign from FVariant
      |
      */
    fn from(var: &FVariant) -> Self {
    
        todo!();
        /*

            isWide = kWideStringDefault ? 1 : 0;
        fromVariant (var);
        */
    }
}

impl From<*mut dyn IString> for String {
    
    /**
      | assign from IString
      |
      */
    fn from(str_: *mut dyn IString) -> Self {
    
        todo!();
        /*

            isWide = str->isWideString ();
        if (isWide)
            assign (str->getText16 ());
        else
            assign (str->getText8 ());
        */
    }
}

#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
impl From<String> for String {

    fn from(str_: String) -> Self {
    
        todo!();
        /*

            *this = std::move (str);
        */
    }
}

impl String {
    
    /* -------------------- access  -------------------- */

    #[inline] pub fn set_char(
        &mut self, 
        index: u32,
        c:     u8

    ) -> bool {
        
        todo!();
        /*
            return setChar8 (index, c);
        */
    }
    
    /* ------------------ assignment  ------------------ */

    /**
      | Assign from a string of type char8
      |
      */
    pub fn assign_from_raw_u8_ptr(&mut self, str_: *const u8) -> &mut String {
        
        todo!();
        /*
            return assign (str);
        */
    }
    
    pub fn assign_from_raw_u16_ptr(&mut self, str_: *const u16) -> &mut String {
        
        todo!();
        /*
            return assign (str);
        */
    }
    
    pub fn assign_from_const_string(&mut self, str_: &ConstString) -> &mut String {
        
        todo!();
        /*
            return assign (str);
        */
    }
    
    pub fn assign_from_string(&mut self, str_: &String) -> &mut String {
        
        todo!();
        /*
            return assign (str);
        */
    }
    
    pub fn assign_from_u8(&mut self, c: u8) -> &mut String {
        
        todo!();
        /*
            return assign (c);
        */
    }
    
    pub fn assign_from_u16(&mut self, c: u16) -> &mut String {
        
        todo!();
        /*
            return assign (c);
        */
    }

    /* -------------------- concat  -------------------- */

    pub fn u8_insert_at(&mut self, idx: u32, c: u8) -> &mut String {
        
        todo!();
        /*
            char8 str[] = {c, 0}; return insertAt (idx, str, 1);
        */
    }
    
    pub fn u16_insert_at(&mut self, idx: u32, c: u16) -> &mut String {
        
        todo!();
        /*
            char16 str[] = {c, 0}; return insertAt (idx, str, 1);
        */
    }

    /* -------------------- replace  -------------------- */

    #[inline] pub fn replace_chars8(
        &mut self, 
        to_replace:    u8,
        to_replace_by: u8

    ) -> bool {
        
        todo!();
        /*
            char8 str[] = {toReplace, 0}; return replaceChars8 (str, toReplaceBy);
        */
    }
    
    #[inline] pub fn replace_chars16(
        &mut self, 
        to_replace:    u16,
        to_replace_by: u16

    ) -> bool {
        
        todo!();
        /*
            char16 str[] = {toReplace, 0}; return replaceChars16 (str, toReplaceBy);
        */
    }

    #[inline] pub fn replace_chars_u8(
        &mut self, 
        to_replace:    u8,
        to_replace_by: u8

    ) -> bool {
        
        todo!();
        /*
            return replaceChars8 (toReplace, toReplaceBy);
        */
    }
    
    #[inline] pub fn replace_chars_u16(
        &mut self, 
        to_replace:    u16,
        to_replace_by: u16

    ) -> bool {
        
        todo!();
        /*
            return replaceChars16 (toReplace, toReplaceBy);
        */
    }
    
    #[inline] pub fn replace_chars_with_raw_u8_ptr(
        &mut self, 
        to_replace:    *const u8,
        to_replace_by: u8

    ) -> bool {
        
        todo!();
        /*
            return replaceChars8 (toReplace, toReplaceBy);
        */
    }
    
    #[inline] pub fn replace_chars_with_raw_u16_ptr(
        &mut self, 
        to_replace:    *const u16,
        to_replace_by: u16

    ) -> bool {
        
        todo!();
        /*
            return replaceChars16 (toReplace, toReplaceBy);
        */
    }

    /* -------------------- remove  -------------------- */

    #[inline] pub fn remove_chars8(&mut self, which: u8) -> bool {
        
        todo!();
        /*
            char8 str[] = {which, 0}; return removeChars8 (str);
        */
    }
    
    #[inline] pub fn remove_chars16(&mut self, which: u16) -> bool {
        
        todo!();
        /*
            char16 str[] = {which, 0}; return removeChars16 (str);
        */
    }
    
    #[inline] pub fn remove_chars_with_raw_u8_ptr(&mut self, which: *const u8) -> bool {
        
        todo!();
        /*
            return removeChars8 (which);
        */
    }
    
    #[inline] pub fn remove_chars_with_raw_u16_ptr(&mut self, which: *const u16) -> bool {
        
        todo!();
        /*
            return removeChars16 (which);
        */
    }
    
    #[inline] pub fn remove_chars_by_u8(&mut self, which: u8) -> bool {
        
        todo!();
        /*
            return removeChars8 (which);
        */
    }
    
    #[inline] pub fn remove_chars_by_u16(&mut self, which: u16) -> bool {
        
        todo!();
        /*
            return removeChars16 (which);
        */
    }
    
    /* --------------------- print  --------------------- */
    /* -------------------- numbers  -------------------- */
    /* ------------------ conversion  ------------------ */

    /**
      | assign n characters of str and convert
      | to wide string by using the specified
      | codepage
      |
      */
    pub fn new_with_codepage(
        str_:          *const u8,
        code_page:     MBCodePage,
        n:             Option<i32>,
        is_terminated: Option<bool>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);

        let is_terminated: bool = is_terminated.unwrap_or(true);
    
        todo!();
        /*

            isWide = 0;
        if (str)
        {
            assign (str, n, isTerminated);
            toWideString (codePage);
        }
        */
    }
    
    /**
      | assign n characters of str (-1: all)
      |
      */
    pub fn new_from_raw(
        str_:          *const u8,
        n:             Option<i32>,
        is_terminated: Option<bool>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);

        let is_terminated: bool = is_terminated.unwrap_or(true);
    
        todo!();
        /*

            if (str)
            assign (str, n, isTerminated);
        */
    }
    
    /**
      | assign n characters of str (-1: all)
      |
      */
    pub fn new_from_raw_u16_ptr(
        str_:          *const u16,
        n:             Option<i32>,
        is_terminated: Option<bool>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);

        let is_terminated: bool = is_terminated.unwrap_or(true);
    
        todo!();
        /*

            isWide = 1;
        if (str)
            assign (str, n, isTerminated);
        */
    }
    
    /**
      | assign n characters of str (-1: all)
      |
      */
    pub fn new_from_string(
        str_: &String,
        n:    Option<i32>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
    
        todo!();
        /*

            isWide = str.isWideString ();
        if (!str.isEmpty ())
            assign (str, n);
        */
    }
    
    /**
      | assign n characters of str (-1: all)
      |
      */
    pub fn new_from_const_string(
        str_: &ConstString,
        n:    Option<i32>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
    
        todo!();
        /*

            isWide = str.isWideString ();
        if (!str.isEmpty ())
            assign (str, n);
        */
    }
    
    pub fn assign_from(&mut self, str_: String) -> &mut String {
        
        todo!();
        /*
            SMTG_ASSERT (buffer == nullptr || buffer != str.buffer);
        tryFreeBuffer ();
        
        isWide = str.isWide;
        buffer = str.buffer;
        len = str.len;
        str.buffer = nullptr;
        str.len = 0;
        return *this;
        */
    }
    
    /**
      | Call this when the string is truncated
      | outside (not recommended though)
      |
      */
    pub fn update_length(&mut self)  {
        
        todo!();
        /*
            if (isWide)
            len = strlen16 (text16 ());
        else
            len = strlen8 (text8 ());
        */
    }
    
    /**
      | Converts to wide string according to
      | sourceCodePage
      |
      */
    pub fn to_wide_string(&mut self, source_code_page: Option<MBCodePage>) -> bool {

        let source_code_page = source_code_page.unwrap_or(MBCodePage::CP_Default);
        
        todo!();
        /*
            if (!isWide)
        {
            if (buffer8 && len > 0)
            {
                int32 bytesNeeded = multiByteToWideString (nullptr, buffer8, 0, sourceCodePage) * sizeof (char16);
                if (bytesNeeded)
                {
                    bytesNeeded += sizeof (char16);
                    char16* newStr = (char16*) malloc (bytesNeeded);
                    if (multiByteToWideString (newStr, buffer8, len + 1, sourceCodePage) <= 0)
                    {
                        free (newStr);
                        return false;
                    }
                    free (buffer8);
                    buffer16 = newStr;
                    isWide = true;
                    updateLength ();
                }
                else
                {
                    return false;
                }
            }
            isWide = true;
        }
        return true;
        */
    }
    
    /**
      | to remove debug code from inline - const_cast
      | inside!!!
      |
      */
    pub fn check_to_multi_byte(&self, dest_code_page: Option<MBCodePage>) -> bool {

        let dest_code_page = dest_code_page.unwrap_or(MBCodePage::CP_Default);
        
        todo!();
        /*
            if (!isWide || isEmpty ())
            return true;

    #if DEVELOPMENT && SMTG_STRING_CHECK_CONVERSION
        int debugLen = length ();
        int debugNonASCII = 0;
        for (int32 i = 0; i < length (); i++)
        {
            if (buffer16[i] > 127)
                ++debugNonASCII;
        }
        
        String* backUp = nullptr;
        if (debugNonASCII > 0)
            backUp = NEW String (*this);
    #endif

        // this should be avoided, since it can lead to information loss
        bool result = const_cast <String&> (*this).toMultiByte (destCodePage);

    #if DEVELOPMENT && SMTG_STRING_CHECK_CONVERSION
        if (backUp)
        {
            String temp (*this);
            temp.toWideString (destCodePage);
            
            if (temp != *backUp)
            {
                backUp->toMultiByte (kCP_Utf8);
                SMTG_STRING_CHECK_MSG ("Indirect string conversion information loss !   %d/%d non ASCII chars:   \"%s\"   ->    \"%s\"\n", debugNonASCII, debugLen, backUp->buffer8, buffer8);
            }
            else
                SMTG_STRING_CHECK_MSG ("Indirect string potential conversion information loss !   %d/%d non ASCII chars   result: \"%s\"\n", debugNonASCII, debugLen, buffer8);

            delete backUp;
        }
    #endif

        return result;
        */
    }
    
    pub fn to_multi_byte(&mut self, dest_code_page: Option<MBCodePage>) -> bool {

        let dest_code_page = dest_code_page.unwrap_or(MBCodePage::CP_Default);
        
        todo!();
        /*
            if (isWide)
        {
            if (buffer16 && len > 0)
            {
                int32 numChars = wideStringToMultiByte (nullptr, buffer16, 0, destCodePage) + sizeof (char8);
                char8* newStr = (char8*) malloc (numChars * sizeof (char8));
                if (wideStringToMultiByte (newStr, buffer16, numChars, destCodePage) <= 0)
                {
                    free (newStr);
                    return false;
                }
                free (buffer16);
                buffer8 = newStr;
                isWide = false;
                updateLength ();
            }
            isWide = false;
        }
        else if (destCodePage != kCP_Default)
        {
            if (toWideString () == false)
                return false;
            return toMultiByte (destCodePage);
        }
        return true;
        */
    }
    
    /**
      | Assigns from UTF8 string
      |
      */
    pub fn fromutf8(&mut self, utf_8string: *const u8)  {
        
        todo!();
        /*
            assign (utf8String);
        toWideString (kCP_Utf8);
        */
    }
    
    /**
      | On PC only kUnicodeNormC is working
      |
      */
    pub fn normalize(&mut self, n: Option<UnicodeNormalization>) -> bool {

        let n = n.unwrap_or(UnicodeNormalization::kUnicodeNormC);
        
        todo!();
        /*
            if (isWide == false)
            return false;

        if (buffer16 == nullptr)
            return true;

    #if SMTG_OS_WINDOWS
    #ifdef UNICODE
        if (n != kUnicodeNormC)
            return false;

        uint32 normCharCount = static_cast<uint32> (FoldString (MAP_PRECOMPOSED, buffer16, len, nullptr, 0));
        if (normCharCount == len)
            return true;

        char16* newString = (char16*)malloc ((normCharCount + 1) * sizeof (char16));
        uint32 converterCount = static_cast<uint32> (FoldString (MAP_PRECOMPOSED, buffer16, len, newString, normCharCount + 1));
        if (converterCount != normCharCount)
        {
            free (newString);
            return false;
        }
        newString [converterCount] = 0;
        free (buffer16);
        buffer16 = newString;
        updateLength ();
        return true;
    #else
        return false;
    #endif

    #elif SMTG_OS_MACOS
        CFMutableStringRef origStr = (CFMutableStringRef)toCFStringRef (0xFFFF, true);
        if (origStr)
        {
            CFStringNormalizationForm normForm = kCFStringNormalizationFormD;
            switch (n)
            {
                case kUnicodeNormC: normForm = kCFStringNormalizationFormC; break;
                case kUnicodeNormD: normForm = kCFStringNormalizationFormD; break;
                case kUnicodeNormKC: normForm = kCFStringNormalizationFormKC; break;
                case kUnicodeNormKD: normForm = kCFStringNormalizationFormKD; break;
            }
            CFStringNormalize (origStr, normForm);
            bool result = fromCFStringRef (origStr);
            CFRelease (origStr);
            return result;
        }
        return false;
    #else
        return false;
    #endif
        */
    }
    
    pub fn try_free_buffer(&mut self)  {
        
        todo!();
        /*
            if (buffer)
        {
            free (buffer);
            buffer = nullptr;
        }
        */
    }
    
    pub fn resize(&mut self, 
        new_length: u32,
        wide:       bool,
        fill:       Option<bool>) -> bool {
        
        let fill: bool = fill.unwrap_or(false);

        todo!();
        /*
            if (newLength == 0)
        {
            tryFreeBuffer ();
            len = 0;
            isWide = wide ? 1 : 0;
        }
        else
        {
            size_t newCharSize = wide ? sizeof (char16) : sizeof (char8);
            size_t oldCharSize = (isWide != 0) ? sizeof (char16) : sizeof (char8);

            size_t newBufferSize = (newLength + 1) * newCharSize;
            size_t oldBufferSize = (len + 1) * oldCharSize;

            isWide = wide ? 1 : 0;

            if (buffer)
            {
                if (newBufferSize != oldBufferSize)
                {
                    void* newstr = realloc (buffer, newBufferSize);
                    if (newstr == nullptr)
                        return false;
                    buffer = newstr;
                    if (isWide)
                        buffer16[newLength] = 0;
                    else
                        buffer8[newLength] = 0;
                }
                else if (wide && newCharSize != oldCharSize)
                    buffer16[newLength] = 0;
            }
            else
            {
                void* newstr = malloc (newBufferSize);
                if (newstr == nullptr)
                    return false;
                buffer = newstr;
                if (isWide)
                {
                    buffer16[0] = 0;
                    buffer16[newLength] = 0;
                }
                else
                {
                    buffer8[0] = 0;
                    buffer8[newLength] = 0;
                }
            }

            if (fill && len < newLength && buffer)
            {
                if (isWide)
                {
                    char16 c = ' '; 
                    for (uint32 i = len; i < newLength; i++)
                        buffer16 [i] = c;
                }
                else
                {
                    memset (buffer8 + len, ' ', newLength - len);
                }
            }
        }
        return true;
        */
    }
    
    pub fn set_char8(&mut self, 
        index: u32,
        c:     u8) -> bool {
        
        todo!();
        /*
            if (index == len && c == 0)
            return true;

        if (index >= len)
        {
            if (c == 0)
            {
                if (resize (index, isWide, true) == false)
                    return false;
                len = index;
                return true;
            }
            else
            {
                if (resize (index + 1, isWide, true) == false)
                    return false;
                len = index + 1;
            }
        }
        
        if (index < len && buffer)
        {
            if (isWide)
            {
                if (c == 0)
                    buffer16[index] = 0;
                else
                {
                    char8 src[] = {c, 0};
                    char16 dest[8] = {0};
                    if (multiByteToWideString (dest, src, 2) > 0)
                        buffer16[index] = dest[0];
                }
                SMTG_ASSERT (buffer16[len] == 0)
            }
            else
            {
                buffer8[index] = c;
                SMTG_ASSERT (buffer8[len] == 0)
            }

            if (c == 0)
                updateLength ();

            return true;
        }
        return false;
        */
    }
    
    pub fn set_char16(&mut self, 
        index: u32,
        c:     u16) -> bool {
        
        todo!();
        /*
            if (index == len && c == 0)
            return true;

        if (index >= len)
        {
            if (c == 0)
            {
                if (resize (index, isWide, true) == false)
                    return false;
                len = index;
                return true;
            }
            else
            {
                if (resize (index + 1, isWide, true) == false)
                    return false;
                len = index + 1;
            }
        }

        if (index < len && buffer)
        {
            if (isWide)
            {
                buffer16[index] = c;
                SMTG_ASSERT (buffer16[len] == 0)
            }
            else
            {
                SMTG_ASSERT (buffer8[len] == 0)
                char16 src[] = {c, 0};
                char8 dest[8] = {0};
                if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                    buffer8[index] = dest[0];
                else
                    return false;
            }

            if (c == 0)
                updateLength ();

            return true;
        }
        return false;
        */
    }
    
    /**
      | Append n characters of str to this (n=-1:
      | all)
      |
      */
    pub fn assign_n_from_const_string(
        &mut self, 
        str_: &ConstString,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (str.isWideString ())
            return assign (str.text16 (), n < 0 ? str.length () : n);
        else
            return assign (str.text8 (), n < 0 ? str.length () : n);
        */
    }
    
    pub fn assign_n_from_raw_u8_ptr(
        &mut self, 
        str_:          *const u8,
        n:             i32,
        is_terminated: bool

    ) -> &mut String {
        
        todo!();
        /*
            if (str == buffer8)
            return *this;

        if (isTerminated)
        {
            uint32 stringLength = (uint32)((str) ? strlen (str) : 0);
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);
        }
        else if (n < 0)
            return *this;

        if (resize (n, false))
        {
            if (buffer8 && n > 0 && str)
            {
                memcpy (buffer8, str, n * sizeof (char8));
                SMTG_ASSERT (buffer8[n] == 0)
            }
            isWide = 0;
            len = n;
        }
        return *this;
        */
    }
    
    pub fn assign_n_from_raw_u16_ptr(
        &mut self, 
        str_:          *const u16,
        n:             i32,
        is_terminated: bool

    ) -> &mut String {
        
        todo!();
        /*
            if (str == buffer16)
            return *this;

        if (isTerminated)
        {
            uint32 stringLength = (uint32)((str) ? strlen16 (str) : 0);
            n = n < 0 ? stringLength : Min<uint32> (n, stringLength);
        }
        else if (n < 0)
            return *this;

        if (resize (n, true))
        {
            if (buffer16 && n > 0 && str)
            {
                memcpy (buffer16, str, n * sizeof (char16));
                SMTG_ASSERT (buffer16[n] == 0)
            }
            isWide = 1;
            len = n;
        }
        return *this;
        */
    }
    
    pub fn assign(&mut self, c: u8, n: i32) -> &mut String {
        
        todo!();
        /*
            if (resize (n, false))
        {
            if (buffer8 && n > 0)
            {
                memset (buffer8, c, n * sizeof (char8));
                SMTG_ASSERT (buffer8[n] == 0)
            }
            isWide = 0;
            len = n;
        }
        return *this;
        */
    }
    
    pub fn assign16(&mut self, c: u16, n: i32) -> &mut String {
        
        todo!();
        /*
            if (resize (n, true))
        {
            if (buffer && n > 0)
            {
                for (int32 i = 0; i < n; i++)
                    buffer16[i] = c;
                SMTG_ASSERT (buffer16[n] == 0)
            }
            isWide = 1;
            len = n;
        }
        return *this;
        */
    }
    
    pub fn append_const_string(
        &mut self, 
        str_: &ConstString,
        n:    i32

    ) -> &mut String {
        
        todo!();
        /*
            if (str.isWideString ())
            return append (str.text16 (), n);
        else
            return append (str.text8 (), n);
        */
    }
    
    /**
      | Append n characters of str to this (n=-1:
      | all)
      |
      */
    pub fn append_raw_u8_ptr(
        &mut self, 
        str_: *const u8,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (str == buffer8)
            return *this;

        if (len == 0)
            return assign (str, n);

        if (isWide)
        {
            String tmp (str);
            if (tmp.toWideString () == false)
                return *this;

            return append (tmp.buffer16, n);
        }

        uint32 stringLength = (uint32)((str) ? strlen (str) : 0);
        n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

        if (n > 0)
        {
            int32 newlen = n + len;
            if (!resize (newlen, false))
                return *this;

            if (buffer && str)
            {
                memcpy (buffer8 + len, str, n * sizeof (char8));
                SMTG_ASSERT (buffer8[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Append n characters of str to this (n=-1:
      | all)
      |
      */
    pub fn append_raw_u16_ptr(
        &mut self, 
        str_: *const u16,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (str == buffer16)
            return *this;

        if (len == 0)
            return assign (str, n);

        if (!isWide)
        {
            if (toWideString () == false)
                return *this;
        }

        uint32 stringLength = (uint32)((str) ? strlen16 (str) : 0);
        n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

        if (n > 0)
        {
            int32 newlen = n + len;
            if (!resize (newlen, true))
                return *this;

            if (buffer16 && str)
            {
                memcpy (buffer16 + len, str, n * sizeof (char16));
                SMTG_ASSERT (buffer16[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Append char c n times
      |
      */
    pub fn append(&mut self, c: u8, n: Option<i32>) -> &mut String {

        let n: i32 = n.unwrap_or(1);
        
        todo!();
        /*
            char8 str[] = {c, 0};
        if (n == 1)
        {
            return append (str, 1);
        }
        else if (n > 1)
        {
            if (isWide)
            {
                String tmp (str);
                if (tmp.toWideString () == false)
                    return *this;

                return append (tmp.buffer16[0], n);
            }

            int32 newlen = n + len;
            if (!resize (newlen, false))
                return *this;

            if (buffer)
            {
                memset (buffer8 + len, c, n * sizeof (char8));
                SMTG_ASSERT (buffer8[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Append char c n times
      |
      */
    pub fn append16(&mut self, c: u16, n: Option<i32>) -> &mut String {

        let n: i32 = n.unwrap_or(1);
        
        todo!();
        /*
            if (n == 1)
        {
            char16 str[] = {c, 0};
            return append (str, 1);
        }
        else if (n > 1)
        {
            if (!isWide)
            {
                if (toWideString () == false)
                    return *this;
            }

            int32 newlen = n + len;
            if (!resize (newlen, true))
                return *this;

            if (buffer16)
            {
                for (int32 i = len; i < newlen; i++)
                    buffer16[i] = c;
                SMTG_ASSERT (buffer16[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Insert n characters of str at position
      | idx (n=-1: all)
      |
      */
    pub fn insert_at(
        &mut self, 
        idx:  u32,
        str_: &ConstString,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (str.isWideString ())
            return insertAt (idx, str.text16 (), n);
        else
            return insertAt (idx, str.text8 (), n);
        */
    }
    
    /**
      | Insert n characters of str at position
      | idx (n=-1: all)
      |
      */
    pub fn insert_at_raw(
        &mut self, 
        idx:  u32,
        str_: *const u8,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (idx > len)
            return *this;

        if (isWide)
        {
            String tmp (str);
            if (tmp.toWideString () == false)
                return *this;
            return insertAt (idx, tmp.buffer16, n);
        }

        uint32 stringLength = (uint32)((str) ? strlen (str) : 0);
        n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

        if (n > 0)
        {
            int32 newlen = len + n;
            if (!resize (newlen, false))
                return *this;

            if (buffer && str)
            {
                if (idx < len)
                    memmove (buffer8 + idx + n, buffer8 + idx, (len - idx) * sizeof (char8));
                memcpy (buffer8 + idx, str, n * sizeof (char8));
                SMTG_ASSERT (buffer8[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Insert n characters of str at position
      | idx (n=-1: all)
      |
      */
    pub fn insert_at_raw_u16_ptr(
        &mut self, 
        idx:  u32,
        str_: *const u16,
        n:    Option<i32>

    ) -> &mut String {

        let n: i32 = n.unwrap_or(-1);
        
        todo!();
        /*
            if (idx > len)
            return *this;

        if (!isWide)
        {
            if (toWideString () == false)
                return *this;
        }

        uint32 stringLength = (uint32)((str) ? strlen16 (str) : 0);
        n = n < 0 ? stringLength : Min<uint32> (n, stringLength);

        if (n > 0)
        {
            int32 newlen = len + n;
            if (!resize (newlen, true))
                return *this;

            if (buffer && str)
            {
                if (idx < len)
                    memmove (buffer16 + idx + n, buffer16 + idx, (len - idx) * sizeof (char16));
                memcpy (buffer16 + idx, str, n * sizeof (char16));
                SMTG_ASSERT (buffer16[newlen] == 0)
            }

            len += n;
        }
        return *this;
        */
    }
    
    /**
      | Replace n1 characters of this (starting at
      | idx) with n2 characters of str (n1,n2=-1:
      | until end)
      */
    pub fn replace_n1_characters_of_this_with_n2_characters_of_str(
        &mut self, 
        idx:  u32,
        n1:   i32,
        str_: &ConstString,
        n2:   Option<i32>

    ) -> &mut String {

        let n2: i32 = n2.unwrap_or(-1);
        
        todo!();
        /*
            if (str.isWideString ())
            return replace (idx, n1, str.text16 (), n2);
        else
            return replace (idx, n1, str.text8 (), n2);
        */
    }

    /**
      | "replace" replaces n1 number of characters
      | at the specified index with n2 characters
      | from the specified string.
      |
      | Replace n1 characters of this (starting at
      | idx) with n2 characters of str (n1,n2=-1:
      | until end)
      */
    pub fn replace_n1_characters_at_specified_index_With_n2_characters_from_str(
        &mut self, 
        idx:  u32,
        n1:   i32,
        str_: *const u8,
        n2:   Option<i32>

    ) -> &mut String {

        let n2: i32 = n2.unwrap_or(-1);
        
        todo!();
        /*
            if (idx > len || str == nullptr)
            return *this;

        if (isWide)
        {
            String tmp (str);
            if (tmp.toWideString () == false)
                return *this;
            if (tmp.length () == 0 || n2 == 0)
                return remove (idx, n1);
            return replace (idx, n1, tmp.buffer16, n2);
        }

        if (n1 < 0 || idx + n1 > len)
            n1 = len - idx;
        if (n1 == 0)
            return *this;

        uint32 stringLength = (uint32)((str) ? strlen (str) : 0);
        n2 = n2 < 0 ? stringLength : Min<uint32> (n2, stringLength);

        uint32 newlen = len - n1 + n2;
        if (newlen > len)
            if (!resize (newlen, false))
                return *this;

        if (buffer)
        {
            memmove (buffer8 + idx + n2, buffer8 + idx + n1, (len - (idx + n1)) * sizeof (char8));
            memcpy (buffer8 + idx, str, n2 * sizeof (char8));
            buffer8[newlen] = 0;    // cannot be removed because resize is not called called in all cases (newlen > len)
        }

        len = newlen;

        return *this;
        */
    }
    
    /**
      | Replace n1 characters of this (starting at
      | idx) with n2 characters of str (n1,n2=-1:
      | until end)
      */
    pub fn replace16(
        &mut self, 
        idx:  u32,
        n1:   i32,
        str_: *const u16,
        n2:   Option<i32>

    ) -> &mut String {

        let n2: i32 = n2.unwrap_or(-1);
        
        todo!();
        /*
            if (idx > len || str == nullptr)
            return *this;

        if (!isWide)
        {
            if (toWideString () == false)
                return *this;
        }

        if (n1 < 0 || idx + n1 > len)
            n1 = len - idx;
        if (n1 == 0)
            return *this;

        uint32 stringLength = (uint32)((str) ? strlen16 (str) : 0);
        n2 = n2 < 0 ? stringLength : Min<uint32> (n2, stringLength);

        uint32 newlen = len - n1 + n2;
        if (newlen > len)
            if (!resize (newlen, true))
                return *this;

        if (buffer)
        {
            memmove (buffer16 + idx + n2, buffer16 + idx + n1, (len - (idx + n1)) * sizeof (char16));
            memcpy (buffer16 + idx, str, n2 * sizeof (char16));
            buffer16[newlen] = 0;   // cannot be removed because resize is not called called in all cases (newlen > len)
        }

        len = newlen;

        return *this;
        */
    }
    
    /**
      | Replace find string with replace string
      | - returns number of replacements
      |
      */
    pub fn replace_find_string_with_replace_string(
        &mut self, 
        to_replace:      *const u8,
        to_replace_with: *const u8,
        all:             Option<bool>,
        m:               Option<StringCompareMode>

    ) -> i32 {

        let all: bool = all.unwrap_or(false);
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (toReplace == nullptr || toReplaceWith == nullptr)
            return 0;

        int32 result = 0;

        int32 idx = findFirst (toReplace, -1, m);
        if (idx > -1)
        {
            int32 toReplaceLen = static_cast<int32> (strlen (toReplace));
            int32 toReplaceWithLen = static_cast<int32> (strlen (toReplaceWith));
            while (idx > -1)
            {
                replace (idx, toReplaceLen, toReplaceWith, toReplaceWithLen);
                result++;

                if (all)
                    idx = findNext (idx + toReplaceWithLen , toReplace, -1, m);
                else
                    break;
            }
        }

        return result;
        */
    }
    
    /**
      | Replace find string with replace string
      | - returns number of replacements
      |
      */
    pub fn replace_find_string_with_replace_string16(
        &mut self, 
        to_replace:      *const u16,
        to_replace_with: *const u16,
        all:             Option<bool>,
        m:               Option<StringCompareMode>

    ) -> i32 {

        let all: bool = all.unwrap_or(false);
        let m: StringCompareMode = m.unwrap_or(StringCompareMode::CaseSensitive);
        
        todo!();
        /*
            if (toReplace == nullptr || toReplaceWith == nullptr)
            return 0;

        int32 result = 0;

        int32 idx = findFirst (toReplace, -1, m);
        if (idx > -1)
        {
            int32 toReplaceLen = strlen16 (toReplace);
            int32 toReplaceWithLen = strlen16 (toReplaceWith);
            while (idx > -1)
            {
                replace (idx, toReplaceLen, toReplaceWith, toReplaceWithLen);
                result++;

                if (all)
                    idx = findNext (idx + toReplaceWithLen, toReplace, -1, m);
                else
                    break;
            }
        }
        return result;
        */
    }
    
    /**
      | Returns true when any replacement was
      | done
      |
      */
    pub fn replace_chars8_raw(
        &mut self, 
        to_replace:    *const u8,
        to_replace_by: u8

    ) -> bool {
        
        todo!();
        /*
            if (isEmpty ())
            return false;

        if (isWide)
        {
            String toReplaceW (toReplace);
            if (toReplaceW.toWideString () == false)
                return false;

            char8 src[] = {toReplaceBy, 0};
            char16 dest[2] = {0};
            if (multiByteToWideString (dest, src, 2) > 0)
            {
                return replaceChars16 (toReplaceW.text16 (), dest[0]);
            }
            return false;
        }

        if (toReplaceBy == 0)
            toReplaceBy = ' ';

        return performReplace<char8> (buffer8, toReplace, toReplaceBy);
        */
    }
    
    pub fn replace_chars16_raw(
        &mut self, 
        to_replace:    *const u16,
        to_replace_by: u16

    ) -> bool {
        
        todo!();
        /*
            if (isEmpty ())
            return false;

        if (!isWide)
        {
            String toReplaceA (toReplace);
            if (toReplaceA.toMultiByte () == false)
                return false;

            if (toReplaceA.length () > 1)
            {
                SMTG_WARNING("cannot replace non ASCII chars on non Wide String")
                return false;
            }

            char16 src[] = {toReplaceBy, 0};
            char8 dest[8] = {0};
            if (wideStringToMultiByte (dest, src, 2) > 0 && dest[1] == 0)
                return replaceChars8 (toReplaceA.text8 (), dest[0]);

            return false;
        }

        if (toReplaceBy == 0)
            toReplaceBy = STR16 (' ');

        return performReplace<char16> (buffer16, toReplace, toReplaceBy);
        */
    }

    /**
      | "remove" removes the specified number
      | of characters from the string starting
      | at the specified index.
      |
      | Remove n characters from string starting
      | at index (n=-1: until end)
      |
      */
    pub fn remove(&mut self, idx: Option<u32>, n: Option<i32>) -> &mut String {
        
        let idx: u32 = idx.unwrap_or(0);
        let n:   i32 = n.unwrap_or(-1);

        todo!();
        /*
            if (isEmpty () || idx >= len || n == 0)
            return *this;

        if ((idx + n > len) || n < 0)
            n = len - idx;
        else
        {
            int32 toMove = len - idx - n;
            if (buffer)
            {
                if (isWide)
                    memmove (buffer16 + idx, buffer16 + idx + n, toMove * sizeof (char16));
                else
                    memmove (buffer8 + idx, buffer8 + idx + n, toMove * sizeof (char8));
            }
        }

        resize (len - n, isWide);
        updateLength ();

        return *this;
        */
    }
    
    pub fn remove_sub_string(
        &mut self, 
        sub_string:     &ConstString,
        all_occurences: Option<bool>

    ) -> bool {

        let all_occurences = all_occurences.unwrap_or(true);
        
        todo!();
        /*
            bool removed = false;
        while (!removed || allOccurences)
        {
            int32 idx = findFirst (subString);
            if (idx < 0)
                break;
            remove (idx, subString.length ());
            removed = true;
        }
        return removed;
        */
    }

    /**
      | Trim lead/trail.
      |
      | "trim" trims the leading and trailing
      | unwanted characters from the string.
      |
      */
    pub fn trim(&mut self, group: Option<StringCharGroup>) -> bool {

        let group = group.unwrap_or(StringCharGroup::kSpace);
        
        todo!();
        /*
            if (isEmpty ())
            return false;

        uint32 newLength;

        switch (group)
        {
            case kSpace:
                if (isWide)
                    newLength = performTrim<char16> (buffer16, len, iswspace, true);
                else
                    newLength = performTrim<char8> (buffer8, len, isspace, true);
                break;

            case kNotAlphaNum:
                if (isWide)
                    newLength = performTrim<char16> (buffer16, len, iswalnum, false);
                else
                    newLength = performTrim<char8> (buffer8, len, isalnum, false);
                break;

            case kNotAlpha:
                if (isWide)
                    newLength = performTrim<char16> (buffer16, len, iswalpha, false);
                else
                    newLength = performTrim<char8> (buffer8, len, isalpha, false);
                break;
                
            default: // Undefined enum value
                return false;
        }

        if (newLength != len)
        {
            resize (newLength, isWide);
            len = newLength;
            return true;
        }
        return false;
        */
    }
    
    /**
      | Removes all of group.
      |
      */
    pub fn remove_chars_with_group(&mut self, group: Option<StringCharGroup>)  {

        let group = group.unwrap_or(StringCharGroup::kSpace);
        
        todo!();
        /*
            if (isEmpty ())
            return;

        uint32 newLength;

        switch (group)
        {
            case kSpace:
                if (isWide)
                    newLength = performRemove<char16> (buffer16, len, iswspace, true);
                else
                    newLength = performRemove<char8> (buffer8, len, isspace, true);
                break;

            case kNotAlphaNum:
                if (isWide)
                    newLength = performRemove<char16> (buffer16, len, iswalnum, false);
                else
                    newLength = performRemove<char8> (buffer8, len, isalnum, false);
                break;

            case kNotAlpha:
                if (isWide)
                    newLength = performRemove<char16> (buffer16, len, iswalpha, false);
                else
                    newLength = performRemove<char8> (buffer8, len, isalpha, false);
                break;
                
            default: // Undefined enum value
                return;
        }

        if (newLength != len)
        {
            resize (newLength, isWide);
            len = newLength;
        }
        */
    }
    
    /**
      | Remove all occurrences of each char
      | in 'which'
      |
      */
    pub fn remove_chars8_raw(&mut self, to_remove: *const u8) -> bool {
        
        todo!();
        /*
            if (isEmpty () || toRemove == nullptr)
            return true;

        if (isWide)
        {
            String wStr (toRemove);
            if (wStr.toWideString () == false)
                return false;
            return removeChars16 (wStr.text16 ());
        }

        uint32 newLength = performRemoveChars<char8> (buffer8, len, toRemove);

        if (newLength != len)
        {
            resize (newLength, false);
            len = newLength;
        }
        return true;
        */
    }
    
    /**
      | Remove all occurrences of each char
      | in 'which'
      |
      */
    pub fn remove_chars16_with_ptr(&mut self, to_remove: *const u16) -> bool {
        
        todo!();
        /*
            if (isEmpty () || toRemove == nullptr)
            return true;

        if (!isWide)
        {
            String str8 (toRemove);
            if (str8.toMultiByte () == false)
                return false;
            return removeChars8 (str8.text8 ());
        }

        uint32 newLength = performRemoveChars<char16> (buffer16, len, toRemove);

        if (newLength != len)
        {
            resize (newLength, true);
            len = newLength;
        }
        return true;
        */
    }
    
    /**
      | Print formatted data into string
      |
      */
    pub fn printf(
        &mut self, 
        format: *const u8,
        args:   &[&str]

    ) -> &mut String {
        
        todo!();
        /*
            char8 string[kPrintfBufferSize];

        va_list marker;
        va_start (marker, format);
        
        vsnprintf (string, kPrintfBufferSize-1, format, marker);
        return assign (string);
        */
    }
    
    /**
      | Print formatted data into string
      |
      */
    pub fn printf_with_u16(
        &mut self, 
        format: *const u16,
        args:   &[&str]

    ) -> &mut String {
        
        todo!();
        /*
            char16 string[kPrintfBufferSize];

        va_list marker;
        va_start (marker, format);
        
        vsnwprintf (string, kPrintfBufferSize-1, format, marker);
        return assign (string);
        */
    }
    
    pub fn vprintf(
        &mut self, 
        format: *const u8,
        args:   &[&str]

    ) -> &mut String {
        
        todo!();
        /*
            char8 string[kPrintfBufferSize];

        vsnprintf (string, kPrintfBufferSize-1, format, args);
        return assign (string);
        */
    }
    
    pub fn print_int64(&mut self, value: i64) -> &mut String {
        
        todo!();
        /*
            if (isWide)
        {
        #if SMTG_CPP11
            return String::printf (STR("%") STR(FORMAT_INT64A), value);
        #else
            return String::printf (STR("%" FORMAT_INT64A), value);
        #endif
        }
        else
            return String::printf ("%" FORMAT_INT64A, value);
        */
    }
    
    pub fn print_float(&mut self, value: f64) -> &mut String {
        
        todo!();
        /*
            if (isWide)
        {
            char16 string[kPrintfBufferSize];
            sprintf16 (string, STR16 ("%lf"), value);

            char16* pointPtr = strrchr16 (string, STR ('.'));
            if (pointPtr)
            {
                pointPtr++; // keep 1st digit after point
                int32 index = strlen16 (string) - 1;
                char16 zero = STR16 ('0');
                while (pointPtr < (string + index))
                {
                    if (string[index] == zero)
                    {
                        string[index] = 0;
                        index--;
                    }
                    else
                        break;
                }
            }
            return assign (string);
        }
        else
        {
            char8 string[kPrintfBufferSize];
            sprintf (string, "%lf", value);

            char8* pointPtr = strrchr (string, '.');
            if (pointPtr)
            {
                pointPtr++; // keep 1st digit after point
                int32 index = (int32) (strlen (string) - 1);
                while (pointPtr < (string + index))
                {
                    if (string[index] == '0')
                    {
                        string[index] = 0;
                        index--;
                    }
                    else
                        break;
                }
            }
            return assign (string);
        }
        */
    }
    
    /**
      | Increment the trailing number if present
      | else start with minNumber, width specifies
      | the string width format (width 2 for
      | number 3 is 03), applyOnlyFormat set
      | to true will only format the string to
      | the given width without incrementing
      | the founded trailing number
      |
      */
    pub fn increment_trailing_number(&mut self, 
        width:             Option<u32>,
        separator:         Option<char>,
        min_number:        Option<u32>,
        apply_only_format: Option<bool>) -> bool {

        let width: u32 = width.unwrap_or(2);
        let separator = separator.unwrap_or(' ');
        let min_number: u32 = min_number.unwrap_or(1);
        let apply_only_format: bool = apply_only_format.unwrap_or(false);
        
        todo!();
        /*
            if (width > 32)
            return false;

        int64 number = 1;
        int32 index = getTrailingNumberIndex ();
        if (index >= 0)
        {
            if (scanInt64 (number, index))
                if (!applyOnlyFormat)
                    number++;

            if (separator != 0 && index > 0 && testChar (index - 1, separator) == true)
                index--;

            remove (index);
        }

        if (number < minNumber)
            number = minNumber;

        if (isWide)
        {
            char16 format[64];
            char16 trail[128];
            if (separator && isEmpty () == false)
            {
                sprintf16 (format, STR16 ("%%c%%0%uu"), width);
                sprintf16 (trail, format, separator, (uint32) number);
            }
            else
            {
                sprintf16 (format, STR16 ("%%0%uu"), width);
                sprintf16 (trail, format, (uint32) number);
            }
            append (trail);
        }
        else
        {
            char format[64];
            char trail[128];
            if (separator && isEmpty () == false)
            {
                sprintf (format, "%%c%%0%uu", width);
                sprintf (trail, format, separator, (uint32) number);
            }
            else
            {
                sprintf (format, "%%0%uu", width);
                sprintf (trail, format, (uint32) number);
            }
            append (trail);
        }

        return true;
        */
    }
    
    /**
      | Lower case the character.
      |
      */
    pub fn to_lower_with_index(&mut self, index: u32)  {
        
        todo!();
        /*
            if (buffer && index < len)
        {
            if (isWide)
                buffer16[index] = ConstString::toLower (buffer16[index]);
            else
                buffer8[index] = ConstString::toLower (buffer8[index]);
        }
        */
    }
    
    /**
      | Lower case the string.
      |
      */
    pub fn to_lower(&mut self)  {
        
        todo!();
        /*
            int32 i = len;
        if (buffer && i > 0)
        {
            if (isWide)
            {
    #if SMTG_OS_MACOS
                CFMutableStringRef cfStr = CFStringCreateMutableWithExternalCharactersNoCopy (kCFAllocator, (UniChar*)buffer16, len, len+1, kCFAllocatorNull);
                CFStringLowercase (cfStr, NULL);
                CFRelease (cfStr);
    #else
                char16* c = buffer16;
                while (i--)
                {
                    *c = ConstString::toLower (*c);
                    c++;
                }
    #endif
            }
            else
            {
                char8* c = buffer8;
                while (i--)
                {
                    *c = ConstString::toLower (*c);
                    c++;
                }
            }
        }
        */
    }
    
    /**
      | Upper case the character.
      |
      */
    pub fn to_upper_with_index(&mut self, index: u32)  {
        
        todo!();
        /*
            if (buffer && index < len)
        {
            if (isWide)
                buffer16[index] = ConstString::toUpper (buffer16[index]);
            else
                buffer8[index] = ConstString::toUpper (buffer8[index]);
        }
        */
    }
    
    /**
      | Upper case the string.
      |
      */
    pub fn to_upper(&mut self)  {
        
        todo!();
        /*
            int32 i = len;
        if (buffer && i > 0)
        {
            if (isWide)
            {
    #if SMTG_OS_MACOS
                CFMutableStringRef cfStr = CFStringCreateMutableWithExternalCharactersNoCopy (kCFAllocator, (UniChar*)buffer16, len, len+1, kCFAllocatorNull);
                CFStringUppercase (cfStr, NULL);
                CFRelease (cfStr);
    #else
                char16* c = buffer16;
                while (i--)
                {
                    *c = ConstString::toUpper (*c);
                    c++;
                }
    #endif
            }
            else
            {
                char8* c = buffer8;
                while (i--)
                {
                    *c = ConstString::toUpper (*c);
                    c++;
                }
            }
        }
        */
    }
    
    /**
      | Assigns string from FVariant
      |
      */
    pub fn from_variant(&mut self, var: &FVariant) -> bool {
        
        todo!();
        /*
            switch (var.getType ())
        {
            case FVariant::kString8:
                assign (var.getString8 ());
                return true;

            case FVariant::kString16:
                assign (var.getString16 ());
                return true;

            case FVariant::kFloat:
                printFloat (var.getFloat ());
                return true;

            case FVariant::kInteger:
                printInt64 (var.getInt ());
                return true;

            default:
                remove ();
        }
        return false;
        */
    }
    
    pub fn to_variant(&self, var: &mut FVariant)  {
        
        todo!();
        /*
            if (isWide)
        {
            var.setString16 (text16 ());
        }
        else
        {
            var.setString8 (text8 ());
        }
        */
    }
    
    /**
      | Assigns string from FAttributes
      |
      */
    pub fn from_attributes(
        &mut self, 
        a:      *mut dyn IAttributes,
        attrid: IAttrID

    ) -> bool {
        
        todo!();
        /*
            FVariant variant;
        if (a->get (attrID, variant) == kResultTrue)
            return fromVariant (variant);
        return false;
        */
    }
    
    pub fn to_attributes(
        &mut self, 
        a:      *mut dyn IAttributes,
        attrid: IAttrID

    ) -> bool {
        
        todo!();
        /*
            FVariant variant;
        toVariant (variant);
        if (a->set (attrID, variant) == kResultTrue)
            return true;
        return false;
        */
    }

    /**
      "swapContent" swaps ownership of the strings
      pointed to
      */
    /**
      | Swaps ownership of the strings pointed
      | to
      |
      */
    pub fn swap_content(&mut self, s: &mut String)  {
        
        todo!();
        /*
            void* tmp = s.buffer;
        uint32 tmpLen = s.len;
        bool tmpWide = s.isWide;
        s.buffer = buffer;
        s.len = len;
        s.isWide = isWide;
        buffer = tmp;
        len = tmpLen;
        isWide = tmpWide;
        */
    }
    
    /**
      | Take ownership of the string of 'str'
      |
      */
    pub fn take_other(&mut self, other: &mut String)  {
        
        todo!();
        /*
            resize (0, other.isWide);
        buffer = other.buffer;
        len = other.len;

        other.buffer = nullptr;
        other.len = 0;
        */
    }
    
    /**
      | Take ownership of buffer
      |
      */
    pub fn take(&mut self, 
        b:    *mut c_void,
        wide: bool)  {
        
        todo!();
        /*
            resize (0, wide);
        buffer = b;
        isWide = wide;
        updateLength ();
        */
    }
    
    pub fn pass(&mut self)  {
        
        todo!();
        /*
            void* res = buffer;
        len = 0;
        buffer = nullptr;
        return res;
        */
    }
    
    /**
      | Pass ownership of buffer to Variant
      | - sets
      | Variant ownership
      |
      */
    pub fn pass_to_variant(&mut self, var: &mut FVariant)  {
        
        todo!();
        /*
            void* passed = pass ();

        if (isWide)
        {
            if (passed)
            {
                var.setString16 ((const char16*)passed);
                var.setOwner (true);
            }
            else
                var.setString16 (kEmptyString16);
        }
        else
        {
            if (passed)
            {
                var.setString8 ((const char8*)passed);
                var.setOwner (true);
            }
            else
                var.setString8 (kEmptyString8);
        }
        */
    }
    
    /**
      | Pascal string conversion
      |
      */
    pub fn to_pascal_string(&mut self, buf: *mut u8) -> *mut u8 {
        
        todo!();
        /*
            if (buffer)
        {
            if (isWide)
            {
                String tmp (*this);
                tmp.toMultiByte ();
                return tmp.toPascalString (buf);
            }

            int32 length = len;
            if (length > 255)
                length = 255;
            buf[0] = (uint8)length;
            while (length >= 0)
            {
                buf[length + 1] = buffer8[length];
                length--;
            }
            return buf;
        }
        else
        {
            *buf = 0;
            return buf;
        }
        */
    }
    
    /**
      | Pascal string conversion
      |
      */
    pub fn from_pascal_string(&mut self, buf: *const u8) -> &String {
        
        todo!();
        /*
            resize (0, false);
        isWide = 0;
        int32 length = buf[0];
        resize (length + 1, false);
        buffer8[length] = 0;    // cannot be removed, because we only do the 0-termination for multibyte buffer8
        while (--length >= 0)
            buffer8[length] = buf[length + 1];
        len = buf[0];
        return *this;
        */
    }

    /**
      | CFString conversion
      |
      */
    #[cfg(SMTG_OS_MACOS)]
    pub fn from_cf_string_ref(
        &mut self, 
        cf_str:   *const c_void,
        encoding: u32

    ) -> bool {

        let encoding: u32 = encoding.unwrap_or(0xFFFF);
        
        todo!();
        /*
            if (cfStr == 0)
            return false;

        CFStringRef strRef = (CFStringRef)cfStr;
        if (isWide)
        {
            CFRange range = { 0, CFStringGetLength (strRef)};
            CFIndex usedBytes;
            if (resize (static_cast<int32> (range.length + 1), true))
            {
                if (encoding == 0xFFFF)
                    encoding = kCFStringEncodingUnicode;
                if (CFStringGetBytes (strRef, range, encoding, ' ', false, (UInt8*)buffer16, range.length * 2, &usedBytes) > 0)
                {
                    buffer16[usedBytes/2] = 0;
                    this->len = strlen16 (buffer16);
                    return true;
                }
            }
        }
        else
        {
            if (cfStr == 0)
                return false;
            if (encoding == 0xFFFF)
                encoding = kCFStringEncodingASCII;
            int32 len = static_cast<int32> (CFStringGetLength (strRef) * 2);
            if (resize (++len, false))
            {
                if (CFStringGetCString (strRef, buffer8, len, encoding))
                {
                    this->len = static_cast<int32> (strlen (buffer8));
                    return true;
                }
            }
        }

        return false;
        */
    }
}
