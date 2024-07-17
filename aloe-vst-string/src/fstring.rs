crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fstring.cpp]

#[cfg(SMTG_OS_WINDOWS)]
lazy_static!{
    /*
    #define stricmp16 wcsicmp
    #define strnicmp16 wcsnicmp
    #define strrchr16 wcsrchr
    #define sprintf16 swprintf
    #define snprintf16 snwprintf
    #define vsnprintf16 vsnwprintf
    #define vsprintf16 wvsprintf
    #define vfprintf16 vfwprintf
    #define sscanf16 swscanf
    #define toupper16 towupper
    #define tolower16 towlower
    #define isupper16 iswupper
    #define islower16 iswlower
    #define isspace16 iswspace
    #define isalpha16 iswalpha
    #define isdigit16 iswdigit
    #define isalnum16 iswalnum

    #define stricmp _stricmp
    #define strnicmp _strnicmp
    #define snprintf _snprintf
    #define vsnprintf _vsnprintf
    #define snwprintf _snwprintf
    #define vsnwprintf _vsnwprintf

    #define wtoi _wtoi
    #define wtol _wtol
    #define wtof _wtof
    */
}

#[cfg(SMTG_OS_LINUX)]
pub type ConverterFacet = CodecvtUtf8Utf16<u16>;

#[cfg(SMTG_OS_LINUX)]
pub type Converter      = WStringConvert<ConverterFacet,u16>;

#[cfg(SMTG_OS_LINUX)]
pub fn converter_facet() -> &mut ConverterFacet {
    
    todo!();
        /*
            static ConverterFacet gFacet;
        return gFacet;
        */
}

#[cfg(SMTG_OS_LINUX)]
pub fn converter() -> &mut Converter {
    
    todo!();
        /*
            static Converter gConverter;
        return gConverter;
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn sprintf16(
        wcs:    *mut Steinberg::char16,
        format: *const Steinberg::char16,
        args:   &[&str]) -> i32 {
    
    todo!();
        /*
            assert(false && "DEPRECATED No Linux implementation");
        return 0;
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn vsnwprintf(
        wcs:    *mut Steinberg::char16,
        maxlen: usize,
        format: *const Steinberg::char16,
        args:   &[Args]) -> i32 {
    
    todo!();
        /*
             Steinberg::char8 str8[kPrintfBufferSize];
        auto format_utf8 = converter ().to_bytes(format);
        auto len = vsnprintf (str8, kPrintfBufferSize, format_utf8.data (), args);

        auto tmp_str = converter ().from_bytes (str8, str8 + len);
        auto target_len = std::min (tmp_str.size (), maxlen - 1);
        tmp_str.copy (wcs, target_len);
        wcs[target_len] = '\0';

        return tmp_str.size ();
        */
}

#[cfg(SMTG_OS_LINUX)]
#[inline] pub fn strrchr16(
        str_: *const Steinberg::char16,
        c:    Steinberg::char16) -> *mut Steinberg::char16 {
    
    todo!();
        /*
            assert(false && "DEPRECATED No Linux implementation");
        return nullptr;
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn strrchr16(
        str_: *const Steinberg::char16,
        c:    Steinberg::char16) -> *mut Steinberg::char16 {
    
    todo!();
        /*
            typename typename Steinberg::int32 len = typename typename Steinberg::ConstString (str).length ();
        while (len > 0)
        {
            if (str[len] == c)
                return const_cast<typename typename Steinberg::char16*>(str + len);
            len--;
        }
        return 0;
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn vsnwprintf(
        str_:   *mut Steinberg::char16,
        size:   Steinberg::int32,
        format: *const Steinberg::char16,
        ap:     &[Args]) -> Steinberg::int32 {
    
    todo!();
        /*
            // wrapped using CoreFoundation's CFString
        CFMutableStringRef formatString = (CFMutableStringRef)typename typename Steinberg::ConstString (format).toCFStringRef (0xFFFF, true);
        CFStringFindAndReplace (formatString, CFSTR("%s"), CFSTR("%S"), CFRangeMake (0, CFStringGetLength (formatString)), 0);
        CFStringRef resultString = CFStringCreateWithFormatAndArguments (typename typename Steinberg::kCFAllocator, 0, formatString, ap);
        CFRelease (formatString);
        if (resultString)
        {
            typename typename Steinberg::String res;
            res.fromCFStringRef (resultString);
            res.copyTo16 (str, 0, size);
            CFRelease (resultString);
            return 0;
        }
        return 1;
        */
}

#[cfg(SMTG_OS_MACOS)]
#[inline] pub fn sprintf16(
        str_:   *mut Steinberg::char16,
        format: *const Steinberg::char16,
        args:   &[&str]) -> Steinberg::int32 {
    
    todo!();
        /*
            va_list marker;
        va_start (marker, format);
        return vsnwprintf (str, -1, format, marker);
        */
}

#[cfg(SMTG_OS_MACOS)]
lazy_static!{
    /*
    uint32 kDefaultSystemEncoding = kCFStringEncodingMacRoman;
    */
}

pub const SMTG_STRING_CHECK_CONVERSION:          usize = 1;
pub const SMTG_STRING_CHECK_CONVERSION_NO_BREAK: usize = 1;

#[cfg(SMTG_STRING_CHECK_CONVERSION_NO_BREAK)]
macro_rules! smtg_string_check_msg {
    () => {
        /*
                FDebugPrint
        */
    }
}

#[cfg(not(SMTG_STRING_CHECK_CONVERSION_NO_BREAK))]
macro_rules! smtg_string_check_msg {
    () => {
        /*
                FDebugBreak
        */
    }
}
