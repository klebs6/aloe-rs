/*!
  | This is the prefered location of all
  | CPP hackery to make $random_compiler
  | work like something approaching a C99
  | (or maybe more accurately GNU99) compiler.
  | 
  | It is assumed that this header will be
  | included after "config.h".
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/compat.h]

lazy_static!{
    /*
    #if defined _MSC_VER || defined __BORLANDC__ || defined __MINGW32__
    #define off_t __int64 /* use this instead of off_t to fix the 2 GB limit */
    #if !defined __MINGW32__
    #define fseeko _fseeki64
    #define ftello _ftelli64
    #else /* MinGW */
    #if !defined(HAVE_FSEEKO)
    #define fseeko fseeko64
    #define ftello ftello64
    #endif
    #endif
    #else
    #define off_t off_t
    #endif

    #if defined(_MSC_VER)
    #define strtoll _strtoi64
    #define strtoull _strtoui64
    #endif

    #if defined(_MSC_VER)
    #define inline __inline
    #endif

    #if defined __INTEL_COMPILER || (defined _MSC_VER && defined _WIN64)
    /* MSVS generates VERY slow 32-bit code with __restrict */
    #define restrict __restrict
    #elif defined __GNUC__
    #define restrict __restrict__
    #else
    #define restrict
    #endif

    #define U64L(x) x##ULL

    #if defined _MSC_VER || defined __BORLANDC__ || defined __MINGW32__
    #define STRCASECMP stricmp
    #define STRNCASECMP strnicmp
    #else
    #define STRCASECMP strcasecmp
    #define STRNCASECMP strncasecmp
    #endif

    #if defined _MSC_VER
    #  if _MSC_VER >= 1600
    /* Visual Studio 2010 has decent C99 support */
    #    define PRIu64 "llu"
    #    define PRId64 "lld"
    #    define PRIx64 "llx"
    #  else
    #    ifndef UINT32_MAX
    #      define UINT32_MAX _UI32_MAX
    #    endif
         typedef unsigned __int64 uint64_t;
         typedef unsigned __int32 uint32_t;
         typedef unsigned __int16 uint16_t;
         typedef unsigned __int8 uint8_t;
         typedef __int64 int64_t;
         typedef __int32 int32_t;
         typedef __int16 int16_t;
         typedef __int8  int8_t;
    #    define PRIu64 "I64u"
    #    define PRId64 "I64d"
    #    define PRIx64 "I64x"
    #  endif
    #endif /* defined _MSC_VER */

    #ifdef _WIN32
    /* All char* strings are in UTF-8 format. Added to support Unicode files on Windows */
    #include "win_utf8_io.h"

    #define printf printf_utf8
    #define fprintf fprintf_utf8
    #define vfprintf vfprintf_utf8
    #define fopen fopen_utf8
    #define chmod chmod_utf8
    #define utime utime_utf8
    #define unlink unlink_utf8
    #define rename rename_utf8
    #define stat _stat64_utf8

    #else

    #define printf printf
    #define fprintf fprintf
    #define vfprintf vfprintf
    #define fopen fopen
    #define chmod chmod
    #define utime utime
    #define unlink unlink
    #define rename rename
    #define stat stat

    #endif

    #ifdef _WIN32
    #define stat_s __stat64 /* stat struct */
    #define fstat _fstat64
    #else
    #define stat_s stat /* stat struct */
    #define fstat fstat
    #endif

    pub const M_LN2: f64 = 0.69314718055994530942;
    pub const M_PI:  f64 = 3.14159265358979323846;

    /* FLAC needs to compile and work correctly on systems with a normal ISO C99
     * snprintf as well as Microsoft Visual Studio which has an non-standards
     * conformant snprint_s function.
     *
     * This function wraps the MS version to behave more like the the ISO version.
     */
    #ifdef __cplusplus
    extern "C" {
    #endif
    int snprintf(char *str, size_t size, const char *fmt, ...);
    int vsnprintf(char *str, size_t size, const char *fmt, va_list va);
    #ifdef __cplusplus
    }
    #endif
    */
}

