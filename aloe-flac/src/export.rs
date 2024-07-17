/*!
  | \file include/FLAC/export.h
  | 
  | -----------
  | @brief
  | 
  | This module contains #defines and symbols
  | for exporting function calls, and providing
  | version information and compiled-in
  | features.
  | 
  | See the \link export export \endlink
  | module. \defgroup export FLAC/export.h:
  | export symbols \ingroup flac
  | ----------
  | @brief
  | 
  | This module contains #defines and symbols
  | for exporting function calls, and providing
  | version information and compiled-in
  | features.
  | 
  | If you are compiling with MSVC and will
  | link to the static library (libFLAC.lib)
  | you should define NO_DLL in your
  | project to make sure the symbols are
  | exported properly.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/export.h]

lazy_static!{
    /*
    #if defined(NO_DLL)
    #define FLAC_API

    #elif defined(_MSC_VER)
    #ifdef FLAC_API_EXPORTS
    #define FLAC_API __declspec(dllexport)
    #else
    #define FLAC_API __declspec(dllimport)
    #endif

    #elif defined(USE_VISIBILITY_ATTR)
    #define FLAC_API __attribute__ ((visibility ("default")))

    #else
    #define FLAC_API

    #endif
    */
}

/**
  | These #defines will mirror the libtool-based
  | library version number, see http://www.gnu.org/software/libtool/manual/libtool.html#Libtool-versioning
  |
  */
pub const FLAC_API_VERSION_CURRENT:  usize = 11;
pub const FLAC_API_VERSION_REVISION: usize = 0;
pub const FLAC_API_VERSION_AGE:      usize = 3;

extern "C" {

    /**
      | \c 1 if the library has been compiled
      | with support for Ogg FLAC, else \c 0.
      |
      */
    lazy_static!{
        /*
           extern FLAC_API int FLAC_API_SUPPORTS_OGG_FLAC;
           */
      }
}
