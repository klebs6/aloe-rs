crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/aloe_core.h]

pub const ALOE_CORE_H_INCLUDED: bool = true;

/**
  | Config: ALOE_FORCE_DEBUG
  | 
  | Normally, ALOE_DEBUG is set to 1 or 0
  | based on compiler and project settings,
  | but if you define this value, you can
  | override this to force it to be true or
  | false.
  |
  */
#[cfg(not(ALOE_FORCE_DEBUG))]
pub const ALOE_FORCE_DEBUG: bool = false;

/**
  | Config: ALOE_LOG_ASSERTIONS
  | 
  | If this flag is enabled, the jassert
  | and jassertfalse macros will always
  | use Logger::writeToLog() to write
  | a message when an assertion happens.
  | 
  | Enabling it will also leave this turned
  | on in release builds. When it's disabled,
  | however, the jassert and jassertfalse
  | macros will not be compiled in a release
  | build.
  | 
  | @see jassert, jassertfalse, Logger
  |
  */
#[cfg(not(ALOE_LOG_ASSERTIONS))]
#[cfg(target_os="android")]
pub const ALOE_LOG_ASSERTIONS: usize = 1;

#[cfg(not(ALOE_LOG_ASSERTIONS))]
#[cfg(not(target_os="android"))]
pub const ALOE_LOG_ASSERTIONS: usize = 0;

/**
  | Config: ALOE_CHECK_MEMORY_LEAKS
  | 
  | Enables a memory-leak check for certain
  | objects when the app terminates. See
  | the LeakedObjectDetector class and
  | the ALOE_LEAK_DETECTOR macro for more
  | details about enabling leak checking
  | for specific classes.
  |
  */
#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_MEMORY_LEAKS)))]
pub const ALOE_CHECK_MEMORY_LEAKS: usize = 1;

/**
  | Config: ALOE_DONT_AUTOLINK_TO_WIN32_LIBRARIES
  | 
  | In a Windows build, this can be used to
  | stop the required system libs being
  | automatically added to the link stage.
  |
  */
#[cfg(not(ALOE_DONT_AUTOLINK_TO_WIN32_LIBRARIES))]
pub const ALOE_DONT_AUTOLINK_TO_WIN32_LIBRARIES: usize = 0;

/**
  | Config: ALOE_INCLUDE_ZLIB_CODE This
  | can be used to disable Aloe's embedded
  | 3rd-party zlib code. You might need
  | to tweak this if you're linking to an
  | external zlib library in your app, but
  | for normal apps, this option should
  | be left alone.
  | 
  | If you disable this, you might also want
  | to set a value for ALOE_ZLIB_INCLUDE_PATH,
  | to specify the path where your zlib headers
  | live.
  |
  */
#[cfg(not(ALOE_INCLUDE_ZLIB_CODE))]
pub const ALOE_INCLUDE_ZLIB_CODE: usize = 1;

/**
  | Config: ALOE_USE_CURL
  | 
  | Enables http/https support via libcurl
  | (Linux only). Enabling this will add
  | an additional run-time dynamic dependency
  | to libcurl.
  | 
  | If you disable this then https/ssl support
  | will not be available on Linux.
  |
  */
#[cfg(not(ALOE_USE_CURL))]
pub const ALOE_USE_CURL: usize = 1;

/**
  | Config: ALOE_LOAD_CURL_SYMBOLS_LAZILY
  | 
  | If enabled, Aloe will load libcurl lazily
  | when required (for example, when WebInputStream
  | is used). Enabling this flag may also
  | help with library dependency errors
  | as linking libcurl at compile-time
  | may instruct the linker to hard depend
  | on a specific version of libcurl. It's
  | also useful if you want to limit the amount
  | of Aloe dependencies and you are not
  | using WebInputStream or the Url classes.
  |
  */
#[cfg(not(ALOE_LOAD_CURL_SYMBOLS_LAZILY))]
pub const ALOE_LOAD_CURL_SYMBOLS_LAZILY: usize = 0;

/**
  | Config: ALOE_CATCH_UNHANDLED_EXCEPTIONS
  | 
  | If enabled, this will add some exception-catching
  | code to forward unhandled exceptions
  | to your ALOEApplicationBase::unhandledException()
  | callback.
  |
  */
#[cfg(not(ALOE_CATCH_UNHANDLED_EXCEPTIONS))]
pub const ALOE_CATCH_UNHANDLED_EXCEPTIONS: usize = 0;

/**
  | Config: ALOE_ALLOW_STATIC_NULL_VARIABLES
  | 
  | If disabled, this will turn off dangerous
  | static globals like String::empty,
  | var::null, etc which can cause nasty
  | order-of-initialisation problems
  | if they are referenced during static
  | constructor code.
  |
  */
#[cfg(not(ALOE_ALLOW_STATIC_NULL_VARIABLES))]
pub const ALOE_ALLOW_STATIC_NULL_VARIABLES: usize = 0;

/**
  | Config: ALOE_STRICT_REFCOUNTEDPOINTER
  | 
  | If enabled, this will make the ReferenceCountedObjectPtr
  | class stricter about allowing itself
  | to be cast directly to a raw pointer.
  | By default this is disabled, for compatibility
  | with old code, but if possible, you should
  | always enable it to improve code safety!
  |
  */
#[cfg(not(ALOE_STRICT_REFCOUNTEDPOINTER))]
pub const ALOE_STRICT_REFCOUNTEDPOINTER: usize = 0;

/**
  | Config: ALOE_ENABLE_ALLOCATION_HOOKS
  | 
  | If enabled, this will add global allocation
  | functions with built-in assertions,
  | which may help when debugging allocations
  | in unit tests.
  |
  */
#[cfg(not(ALOE_ENABLE_ALLOCATION_HOOKS))]
pub const ALOE_ENABLE_ALLOCATION_HOOKS: usize = 0;

#[cfg(not(ALOE_STRING_UTF_TYPE))]
pub const ALOE_STRING_UTF_TYPE: usize = 8;

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/aloe_core.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:   usize = 1;
pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:  usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS: usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:    usize = 1;

#[cfg(not(ALOE_STANDALONE_APPLICATION))]
pub const ALOE_STANDALONE_APPLICATION: bool = false; 
