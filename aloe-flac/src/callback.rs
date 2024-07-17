/*!
  | \file include/FLAC/callback.h
  | 
  | -----------
  | @brief
  | 
  | This module defines the structures
  | for describing I/O callbacks to the
  | other FLAC interfaces.
  | 
  | See the detailed documentation for
  | callbacks in the \link callbacks
  | callbacks \endlink module. \defgroup
  | callbacks FLAC/callback.h:
  | I/O callback structures \ingroup flac
  | ----------
  | @brief
  | 
  | This module defines the structures
  | for describing I/O callbacks to the
  | other FLAC interfaces.
  | 
  | The purpose of the I/O callback functions
  | is to create a common way for the metadata
  | interfaces to handle I/O.
  | 
  | Originally the metadata interfaces
  | required filenames as the way of specifying
  | FLAC files to operate on. This is problematic
  | in some environments so there is an additional
  | option to specify a set of callbacks
  | for doing I/O on the FLAC file, instead
  | of the filename.
  | 
  | In addition to the callbacks, a IOHandle
  | type is defined as an opaque structure
  | for a data source.
  | 
  | The callback function prototypes are
  | similar (but not identical) to the stdio
  | functions fread, fwrite, fseek, ftell,
  | feof, and fclose. If you use stdio streams
  | to implement the callbacks, you can
  | pass fread, fwrite, and fclose anywhere
  | a IOCallbackRead, IOCallbackWrite,
  | or
  | 
  | IOCallbackClose is required,
  | and a FILE* anywhere a IOHandle
  | is required. \warning You generally
  | CANNOT directly use fseek or ftell for
  | IOCallbackSeek or IOCallbackTell
  | since on most systems these use 32-bit
  | offsets and FLAC requires 64-bit offsets
  | to deal with large files. You will have
  | to find an equivalent function (e.g.
  | ftello), or write a wrapper. The same
  | is true for feof() since this is usually
  | implemented as a macro, not as a function
  | whose address can be taken. \{
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/callback.h]
/**
  | This is the opaque handle type used by
  | the callbacks. Typically this is a \c
  | FILE* or address of a file descriptor.
  |
  */
pub type IOHandle = *mut c_void;

/**
  | Signature for the read callback.
  | 
  | The signature and semantics match POSIX
  | fread() implementations and can generally
  | be used interchangeably.
  | 
  | -----------
  | @param ptr
  | 
  | The address of the read buffer.
  | ----------
  | @param size
  | 
  | The size of the records to be read.
  | ----------
  | @param nmemb
  | 
  | The number of records to be read.
  | ----------
  | @param handle
  | 
  | The handle to the data source.
  | 
  | -----------
  | @return
  | 
  | size_t
  | 
  | The number of records read.
  |
  */
pub type IOCallbackRead = fn(
    ptr:    *mut c_void,
    size:   usize,
    nmemb:  usize,
    handle: IOHandle
) -> usize;

/**
  | Signature for the write callback.
  | 
  | The signature and semantics match POSIX
  | fwrite() implementations and can generally
  | be used interchangeably.
  | 
  | -----------
  | @param ptr
  | 
  | The address of the write buffer.
  | ----------
  | @param size
  | 
  | The size of the records to be written.
  | ----------
  | @param nmemb
  | 
  | The number of records to be written.
  | ----------
  | @param handle
  | 
  | The handle to the data source.
  | 
  | -----------
  | @return
  | 
  | size_t
  | 
  | The number of records written.
  |
  */
pub type IOCallbackWrite = fn(
    ptr:    *const c_void,
    size:   usize,
    nmemb:  usize,
    handle: IOHandle
) -> usize;

/**
  | Signature for the seek callback.
  | 
  | The signature and semantics mostly
  | match POSIX fseek() WITH ONE IMPORTANT
  | 
  | EXCEPTION: the offset is a 64-bit type
  | whereas fseek() is generally 'long'
  | and 32-bits wide.
  | 
  | -----------
  | @param handle
  | 
  | The handle to the data source.
  | ----------
  | @param offset
  | 
  | The new position, relative to \a whence
  | ----------
  | @param whence
  | 
  | \c SEEK_SET, \c SEEK_CUR, or \c SEEK_END
  | 
  | -----------
  | @return
  | 
  | int \c 0 on success, \c -1 on error.
  |
  */
pub type IOCallbackSeek = fn(
    handle: IOHandle,
    offset: i64,
    whence: i32
) -> i32;


/**
  | Signature for the tell callback.
  | 
  | The signature and semantics mostly
  | match POSIX ftell() WITH ONE IMPORTANT
  | 
  | EXCEPTION: the offset is a 64-bit type
  | whereas ftell() is generally 'long'
  | and 32-bits wide.
  | 
  | -----------
  | @param handle
  | 
  | The handle to the data source.
  | 
  | -----------
  | @return
  | 
  | i64
  | 
  | The current position on success, \c
  | -1 on error.
  |
  */
pub type IOCallbackTell = fn(handle: IOHandle) -> i64;

/**
  | Signature for the EOF callback.
  | 
  | The signature and semantics mostly
  | match POSIX feof() but WATCHOUT: on
  | many systems, feof() is a macro, so in
  | this case a wrapper function must be
  | provided instead.
  | 
  | -----------
  | @param handle
  | 
  | The handle to the data source.
  | 
  | -----------
  | @return
  | 
  | int \c 0 if not at end of file, nonzero
  | if at end of file.
  |
  */
pub type IOCallbackEOF = fn(handle: IOHandle) -> i32;

/**
  | Signature for the close callback.
  | 
  | The signature and semantics match POSIX
  | fclose() implementations and can generally
  | be used interchangeably.
  | 
  | -----------
  | @param handle
  | 
  | The handle to the data source.
  | 
  | -----------
  | @return
  | 
  | int \c 0 on success, \c EOF on error.
  |
  */
pub type IOCallbackClose = fn(handle: IOHandle) -> i32;

/**
  | A structure for holding a set of callbacks.
  | 
  | Each FLAC interface that requires a
  | IOCallbacks structure will
  | describe which of the callbacks are
  | required. The ones that are not required
  | may be set to NULL.
  | 
  | If the seek requirement for an interface
  | is optional, you can signify that a data
  | sorce is not seekable by setting the
  | \a seek field to \c NULL.
  |
  */
pub struct IOCallbacks {
    read:  IOCallbackRead,
    write: IOCallbackWrite,
    seek:  IOCallbackSeek,
    tell:  IOCallbackTell,
    eof:   IOCallbackEOF,
    close: IOCallbackClose,
}
