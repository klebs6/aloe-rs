crate::ix!();

pub enum StringCompareMode
{
    /**
      | Comparison is done with regard to character's
      | case
      |
      */
    CaseSensitive,     

    /**
      | Comparison is done without regard to
      | character's case
      |
      */
    CaseInsensitive    
}

/**
  | UTF-8 EF BB BF
  | 
  | UTF-16 Big Endian FE FF
  | 
  | UTF-16 Little Endian FF FE
  | 
  | UTF-32 Big Endian 00 00 FE FF
  | 
  | UTF-32 Little Endian FF FE 00 00
  |
  */
#[inline] pub fn is_case_sensitive(mode: StringCompareMode) -> bool {
    
    todo!();
        /*
            return mode == ConstString::kCaseSensitive;
        */
}

pub fn tstrnatcmp<T>(
    s1:             *const T,
    s2:             *const T,
    case_sensitive: Option<bool>

) -> i32 {

    let case_sensitive: bool = case_sensitive.unwrap_or(true);
    todo!();

    /*
            if (s1 == nullptr && s2 == nullptr)
            return 0;
        else if (s1 == nullptr)
            return -1;
        else if (s2 == nullptr)
            return 1;

        while (*s1 && *s2)
        {
            if (ConstString::isCharDigit (*s1) && ConstString::isCharDigit (*s2))
            {
                int32 s1LeadingZeros = 0;
                while (*s1 == '0')
                {
                    s1++; // skip leading zeros
                    s1LeadingZeros++;
                }
                int32 s2LeadingZeros = 0;
                while (*s2 == '0')
                {
                    s2++; // skip leading zeros
                    s2LeadingZeros++;
                }

                int32 countS1Digits = 0;
                while (*(s1 + countS1Digits) && ConstString::isCharDigit (*(s1 + countS1Digits)))
                    countS1Digits++;
                int32 countS2Digits = 0;
                while (*(s2 + countS2Digits) && ConstString::isCharDigit (*(s2 + countS2Digits)))
                    countS2Digits++;

                if (countS1Digits != countS2Digits)
                    return countS1Digits - countS2Digits; // one number is longer than the other

                for (int32 i = 0; i < countS1Digits; i++)
                {
                    // countS1Digits == countS2Digits
                    if (*s1 != *s2)
                        return (int32)(*s1 - *s2); // the digits differ
                    s1++;
                    s2++;
                }

                if (s1LeadingZeros != s2LeadingZeros)
                    return s1LeadingZeros - s2LeadingZeros; // differentiate by the number of leading zeros
            }
            else
            {
                if (caseSensitive == false)
                {
                    T srcToUpper = static_cast<T> (toupper (*s1));
                    T dstToUpper = static_cast<T> (toupper (*s2));
                    if (srcToUpper != dstToUpper)
                        return (int32)(srcToUpper - dstToUpper);
                }
                else if (*s1 != *s2)
                    return (int32)(*s1 - *s2);

                s1++;
                s2++;
            }
        }

        if (*s1 == 0 && *s2 == 0)
            return 0;
        else if (*s1 == 0)
            return -1;
        else if (*s2 == 0)
            return 1;
        else
            return (int32)(*s1 - *s2);
        */
}

pub fn strnatcmp8(
        s1:             *const u8,
        s2:             *const u8,
        case_sensitive: Option<bool>) -> i32 {
    let case_sensitive: bool = case_sensitive.unwrap_or(true);

    todo!();
        /*
            return tstrnatcmp (s1, s2, caseSensitive);
        */
}

pub fn strnatcmp16(
        s1:             *const u16,
        s2:             *const u16,
        case_sensitive: Option<bool>) -> i32 {

    let case_sensitive: bool = case_sensitive.unwrap_or(true);

    todo!();
        /*
            return tstrnatcmp (s1, s2, caseSensitive);
        */
}

/**
   The following functions will only work with European Numbers!
   (e.g. Arabic, Tibetan, and Khmer digits are not supported)
  */
lazy_static!{
    /*
    extern int32 strnatcmp8 (const char8* s1, const char8* s2, bool caseSensitive = true);
    extern int32 strnatcmp16 (const char16* s1, const char16* s2, bool caseSensitive = true);
    */
}

#[inline] pub fn strnatcmp(
        s1:             *const u16,
        s2:             *const u16,
        case_sensitive: Option<bool>) -> i32 {
    let case_sensitive: bool = case_sensitive.unwrap_or(true);

    todo!();
        /*
            #ifdef UNICODE
        return strnatcmp16 (s1, s2, caseSensitive);
    #else
        return strnatcmp8 (s1, s2, caseSensitive);
    #endif
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn stricasecmp(
        s1: *const Steinberg::char8,
        s2: *const Steinberg::char8) -> i32 {
    
    todo!();
        /*
            return ::strcasecmp (s1, s2);
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn strnicasecmp(
        s1: *const Steinberg::char8,
        s2: *const Steinberg::char8,
        n:  usize) -> i32 {
    
    todo!();
        /*
            return ::strncasecmp (s1, s2, n);
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn stricmp16(
        s1: *const Steinberg::char16,
        s2: *const Steinberg::char16) -> i32 {
    
    todo!();
        /*
            auto str1 = converter ().to_bytes (s1);
        auto str2 = converter ().to_bytes (s2);
        return stricasecmp (str1.data (), str2.data ());
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn strnicmp16(
        s1: *const Steinberg::char16,
        s2: *const Steinberg::char16,
        n:  i32) -> i32 {
    
    todo!();
        /*
            auto str1 = converter ().to_bytes (s1);
        auto str2 = converter ().to_bytes (s2);
        return strnicasecmp (str1.data (), str2.data (), n);
        */
}

#[cfg(SMTG_OS_MACOS)]
lazy_static!{
    /*
    #define tstrtoi64 strtoll
    #define stricmp strcasecmp
    #define strnicmp strncasecmp
    */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn strnicmp16(
        str1: *const Steinberg::char16,
        str2: *const Steinberg::char16,
        size: usize) -> Steinberg::int32 {
    
    todo!();
        /*
            if (size == 0)
            return 0;

        CFIndex str1Len = Steinberg::strlen16 (str1);
        CFIndex str2Len = Steinberg::strlen16 (str2);
        if (static_cast<CFIndex> (size) < str2Len) // range is not applied to second string
            str2Len = size;
        CFStringRef cfStr1 = CFStringCreateWithCharactersNoCopy (Steinberg::kCFAllocator, (UniChar*)str1, str1Len, kCFAllocatorNull);
        CFStringRef cfStr2 = CFStringCreateWithCharactersNoCopy (Steinberg::kCFAllocator, (UniChar*)str2, str2Len, kCFAllocatorNull);
        CFComparisonResult result = CFStringCompareWithOptions (cfStr1, cfStr2, CFRangeMake (0, size), kCFCompareCaseInsensitive);
        CFRelease (cfStr1);
        CFRelease (cfStr2);
        switch (result)
        {
            case kCFCompareEqualTo: return 0;
            case kCFCompareLessThan: return -1;
            case kCFCompareGreaterThan: 
            default: return 1;
        };
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn stricmp16(
        str1:     *const u16,
        str_1len: CFIndex,
        str2:     *const u16,
        str_2len: CFIndex) -> i32 {
    
    todo!();
        /*
            CFStringRef cfStr1 = CFStringCreateWithCharactersNoCopy (Steinberg::kCFAllocator, (UniChar*)str1, str1Len, kCFAllocatorNull);
        CFStringRef cfStr2 = CFStringCreateWithCharactersNoCopy (Steinberg::kCFAllocator, (UniChar*)str2, str2Len, kCFAllocatorNull);
        CFComparisonResult result = CFStringCompare (cfStr1, cfStr2, kCFCompareCaseInsensitive);
        CFRelease (cfStr1);
        CFRelease (cfStr2);
        switch (result)
        {
            case kCFCompareEqualTo: return 0;
            case kCFCompareLessThan: return -1;
            case kCFCompareGreaterThan: 
            default: return 1;
        };
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn stricmp16(
        str1: &Steinberg::ConstString,
        str2: &Steinberg::ConstString) -> Steinberg::int32 {
    
    todo!();
        /*
            return stricmp16 (str1.text16 (), str1.length (), str2.text16 (), str2.length ());
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn stricmp16(
        str1: *const Steinberg::char16,
        str2: *const Steinberg::char16) -> Steinberg::int32 {
    
    todo!();
        /*
            CFIndex str1Len = typename typename Steinberg::strlen16 (str1);
        CFIndex str2Len = typename typename Steinberg::strlen16 (str2);
        return stricmp16 (str1, str1Len, str2, str2Len);
        */
}
