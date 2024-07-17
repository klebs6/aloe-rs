/*!
  | (For an example of how all these routines are
  | used, see the source code for the unit tests in
  | src/test_libFLAC/metadata_*.c, or metaflac in
  | src/metaflac/)
  |
  | \file include/FLAC/metadata.h
  | 
  | -----------
  | @brief
  | 
  | This module provides functions for
  | creating and manipulating FLAC metadata
  | blocks in memory, and three progressively
  | more powerful interfaces for traversing
  | and editing metadata in FLAC files.
  | 
  | See the detailed documentation for
  | each interface in the \link flac_metadata
  | metadata \endlink module. \defgroup
  | flac_metadata FLAC/metadata.h: metadata
  | interfaces \ingroup flac
  | ----------
  | @brief
  | 
  | This module provides functions for
  | creating and manipulating FLAC metadata
  | blocks in memory, and three progressively
  | more powerful interfaces for traversing
  | and editing metadata in native FLAC
  | files.
  | 
  | -----------
  | @note
  | 
  | currently only the Chain interface
  | (level 2) supports Ogg
  | 
  | FLAC files, and it is read-only i.e.
  | no writing back changed metadata to
  | file.
  | 
  | There are three metadata interfaces
  | of increasing complexity:
  | 
  | Level 0:
  | 
  | Read-only access to the STREAMINFO,
  | VORBIS_COMMENT, CUESHEET, and
  | 
  | PICTURE blocks.
  | 
  | Level 1:
  | 
  | Read-write access to all metadata blocks.
  | This level is write- efficient in most
  | cases (more on this below), and uses
  | less memory than level 2.
  | 
  | Level 2:
  | 
  | Read-write access to all metadata blocks.
  | This level is write- efficient in all
  | cases, but uses more memory since all
  | metadata for the whole file is read into
  | memory and manipulated before writing
  | out again.
  | 
  | What do we mean by efficient? Since FLAC
  | metadata appears at the beginning of
  | the file, when writing metadata back
  | to a FLAC file it is possible to grow or
  | shrink the metadata such that the entire
  | file must be rewritten. However, if
  | the size remains the same during changes
  | or PADDING blocks are utilized, only
  | the metadata needs to be overwritten,
  | which is much faster.
  | 
  | Efficient means the whole file is rewritten
  | at most one time, and only when necessary.
  | Level 1 is not efficient only in the case
  | that you cause more than one metadata
  | block to grow or shrink beyond what can
  | be accomodated by padding. In this case
  | you should probably use level 2, which
  | allows you to edit all the metadata for
  | a file in memory and write it out all at
  | once.
  | 
  | All levels know how to skip over and not
  | disturb an ID3v2 tag at the front of the
  | file.
  | 
  | All levels access files via their filenames.
  | In addition, level 2 has additional
  | alternative read and write functions
  | that take an I/O handle and callbacks,
  | for situations where access by filename
  | is not possible.
  | 
  | In addition to the three interfaces,
  | this module defines functions for creating
  | and manipulating various metadata
  | objects in memory. As we see from the
  | Format module, FLAC metadata blocks
  | in memory are very primitive structures
  | for storing information in an efficient
  | way. Reading information from the structures
  | is easy but creating or modifying them
  | directly is more complex. The metadata
  | object routines here facilitate this
  | by taking care of the consistency and
  | memory management drudgery.
  | 
  | Unless you will be using the level 1 or
  | 2 interfaces to modify existing metadata
  | however, you will not probably not need
  | these.
  | 
  | From a dependency standpoint, none
  | of the encoders or decoders require
  | the metadata module. This is so that
  | embedded users can strip out the metadata
  | module from libFLAC to reduce the size
  | and complexity.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/metadata.h]

/**
  | Status type for Metadata_SimpleIterator.
  | 
  | The iterator's current status can be
  | obtained by calling metadata_simple_iterator_status().
  |
  */
pub enum Metadata_SimpleIteratorStatus {

    /**
      | The iterator is in the normal OK state
      |
      */
    Ok = 0,

    /**
      | The data passed into a function violated
      | the function's usage criteria
      |
      */
    IllegalInput,

    /**
      | The iterator could not open the target
      | file
      |
      */
    ErrorOpeningFile,

    /**
      | The iterator could not find the FLAC
      | signature at the start of the file
      |
      */
    NotAFlacFile,

    /**
      | The iterator tried to write to a file
      | that was not writable
      |
      */
    NotWritable,

    /**
      | The iterator encountered input that
      | does not conform to the FLAC metadata
      | specification
      |
      */
    BadMetadata,

    /**
      | The iterator encountered an error while
      | reading the FLAC file
      |
      */
    ReadError,

    /**
      | The iterator encountered an error while
      | seeking in the FLAC file
      |
      */
    SeekError,

    /**
      | The iterator encountered an error while
      | writing the FLAC file
      |
      */
    WriteError,

    /**
      | The iterator encountered an error renaming
      | the FLAC file
      |
      */
    RenameError,

    /**
      | The iterator encountered an error removing
      | the temporary file
      |
      */
    UnlinkError,

    /**
      | Memory allocation failed
      |
      */
    MemoryAllocationError,

    /**
      | The caller violated an assertion or
      | an unexpected error occurred
      |
      */
    InternalError
}

pub enum Metadata_ChainStatus {

    /**
      | The chain is in the normal OK state
      |
      */
    Ok = 0,

    /**
      | The data passed into a function violated
      | the function's usage criteria
      |
      */
    IllegalInput,

    /**
      | The chain could not open the target file
      |
      */
    ErrorOpeningFile,

    /**
      | The chain could not find the FLAC signature
      | at the start of the file
      |
      */
    NotAFlacFile,

    /**
      | The chain tried to write to a file that
      | was not writable
      |
      */
    NotWritable,

    /**
      | The chain encountered input that does
      | not conform to the FLAC metadata specification
      |
      */
    BadMetadata,

    /**
      | The chain encountered an error while
      | reading the FLAC file
      |
      */
    ReadError,

    /**
      | The chain encountered an error while
      | seeking in the FLAC file
      |
      */
    SeekError,

    /**
      | The chain encountered an error while
      | writing the FLAC file
      |
      */
    WriteError,

    /**
      | The chain encountered an error renaming
      | the FLAC file
      |
      */
    RenameError,

    /**
      | The chain encountered an error removing
      | the temporary file
      |
      */
    UnlinkError,

    /**
      | Memory allocation failed
      |
      */
    MemoryAllocationError,

    /**
      | The caller violated an assertion or
      | an unexpected error occurred
      |
      */
    InternalError,

    /**
      | One or more of the required callbacks
      | was NULL
      |
      */
    InvalidCallbacks,

    /**
      | metadata_chain_write() was
      | called on a chain read by
      | 
      | metadata_chain_read_with_callbacks()/metadata_chain_read_ogg_with_callbacks(),
      | or
      | 
      | metadata_chain_write_with_callbacks()/metadata_chain_write_with_callbacks_and_tempfile()
      | was called on a chain read by
      | 
      | metadata_chain_read()/metadata_chain_read_ogg().
      | 
      | Matching read/write methods must always
      | be used.
      |
      */
    ReadWriteMismatch,

    /**
      | metadata_chain_write_with_callbacks()
      | was called when the chain write requires
      | a tempfile; use
      | 
      | metadata_chain_write_with_callbacks_and_tempfile()
      | instead.
      | 
      | Or, metadata_chain_write_with_callbacks_and_tempfile()
      | was called when the chain write does
      | not require a tempfile; use
      | 
      | metadata_chain_write_with_callbacks()
      | instead.
      | 
      | Always check metadata_chain_check_if_tempfile_needed()
      | before writing via callbacks.
      |
      */
    WrongWriteCall
}

/**
  | \defgroup flac_metadata_level2 FLAC/metadata.h:
  | metadata level 2 interface \ingroup
  | flac_metadata
  | 
  | -----------
  | @brief
  | 
  | The level 2 interface provides read-write
  | access to FLAC file metadata; all metadata
  | is read into memory, operated on in memory,
  | and then written to file, which is more
  | efficient than level 1 when editing
  | multiple blocks.
  | 
  | Currently Ogg FLAC is supported for
  | read only, via
  | 
  | metadata_chain_read_ogg()
  | but a subsequent
  | 
  | metadata_chain_write() will
  | fail.
  | 
  | The general usage of this interface
  | is:
  | 
  | - Create a new chain using metadata_chain_new().
  | A chain is a linked list of FLAC metadata
  | blocks.
  | 
  | - Read all metadata into the the chain
  | from a FLAC file using
  | 
  | metadata_chain_read() or metadata_chain_read_ogg()
  | and check the status.
  | 
  | - Optionally, consolidate the padding
  | using
  | 
  | metadata_chain_merge_padding()
  | or
  | 
  | metadata_chain_sort_padding().
  | 
  | - Create a new iterator using metadata_iterator_new()
  | 
  | - Initialize the iterator to point to
  | the first element in the chain using
  | metadata_iterator_init()
  | 
  | - Traverse the chain using metadata_iterator_next
  | and
  | 
  | metadata_iterator_prev().
  | 
  | - Get a block for reading or modification
  | using
  | 
  | metadata_iterator_get_block().
  | The pointer to the object inside the
  | chain is returned, so the block is yours
  | to modify.
  | 
  | Changes will be reflected in the FLAC
  | file when you write the chain. You can
  | also add and delete blocks (see functions
  | below).
  | 
  | - When done, write out the chain using
  | metadata_chain_write().
  | 
  | Make sure to read the whole comment to
  | the function below.
  | 
  | - Delete the chain using metadata_chain_delete().
  | 
  | -----------
  | @note
  | 
  | Even though the FLAC file is not open
  | while the chain is being manipulated,
  | you must not alter the file externally
  | during this time. The chain assumes
  | the FLAC file will not change between
  | the time of metadata_chain_read()/metadata_chain_read_ogg()
  | and metadata_chain_write().
  | ----------
  | @note
  | 
  | Do not modify the is_last, length, or
  | type fields of returned
  | 
  | StreamMetadata objects. These
  | are managed automatically.
  | ----------
  | @note
  | 
  | The metadata objects returned by metadata_iterator_get_block()
  | are owned by the chain; do not metadata_object_delete()
  | them.
  | 
  | In the same way, blocks passed to metadata_iterator_set_block()
  | become owned by the chain and they will
  | be deleted when the chain is deleted.
  |
  */
pub struct Metadata_Chain {}

pub struct Metadata_Iterator {}

/**
  | \defgroup flac_metadata_level1 FLAC/metadata.h:
  | metadata level 1 interface \ingroup
  | flac_metadata
  | 
  | -----------
  | @brief
  | 
  | The level 1 interface provides read-write
  | access to FLAC file metadata and operates
  | directly on the FLAC file.
  | 
  | The general usage of this interface
  | is:
  | 
  | - Create an iterator using
  | metadata_simple_iterator_new()
  | 
  | - Attach it to a file using
  | metadata_simple_iterator_init() and
  | check the exit code. Call
  | metadata_simple_iterator_is_writable()
  | to see if the file is writable, or only read
  | access is allowed.
  | 
  | - Use metadata_simple_iterator_next()
  | and
  | 
  | metadata_simple_iterator_prev()
  | to traverse the blocks.
  | 
  | This is does not read the actual blocks
  | themselves.
  | 
  | metadata_simple_iterator_next()
  | is relatively fast.
  | 
  | metadata_simple_iterator_prev()
  | is slower since it needs to search forward
  | from the front of the file.
  | 
  | - Use metadata_simple_iterator_get_block_type()
  | or
  | 
  | metadata_simple_iterator_get_block()
  | to access the actual data at the current
  | iterator position. The returned object
  | is yours to modify and free.
  | 
  | - Use metadata_simple_iterator_set_block()
  | to write a modified block back. You must
  | have write permission to the original
  | file. Make sure to read the whole comment
  | to metadata_simple_iterator_set_block()
  | below.
  | 
  | - Use metadata_simple_iterator_insert_block_after()
  | to add new blocks.
  | 
  | Use the object creation functions from
  | \link flac_metadata_object here \endlink
  | to generate new objects.
  | 
  | - Use metadata_simple_iterator_delete_block()
  | to remove the block currently referred
  | to by the iterator, or replace it with
  | padding.
  | 
  | - Destroy the iterator with
  | metadata_simple_iterator_delete() when
  | finished.
  | 
  | -----------
  | @note
  | 
  | The FLAC file remains open the whole
  | time between
  | 
  | metadata_simple_iterator_init()
  | and
  | 
  | metadata_simple_iterator_delete(),
  | so make sure you are not altering the
  | file during this time.
  | ----------
  | @note
  | 
  | Do not modify the \a is_last, \a length,
  | or \a type fields of returned
  | 
  | StreamMetadata objects. These
  | are managed automatically.
  | ----------
  | @note
  | 
  | If any of the modification functions
  | (metadata_simple_iterator_set_block(),
  | 
  | metadata_simple_iterator_delete_block(),
  | 
  | metadata_simple_iterator_insert_block_after(),
  | etc.) return \c false, you should delete
  | the iterator as it may no longer be valid.
  |
  */
pub struct Metadata_SimpleIterator {}

/*
   | \defgroup flac_metadata_level0 FLAC/metadata.h:
   | metadata level 0 interface \ingroup
   | flac_metadata
   | 
   | -----------
   | @brief
   | 
   | The level 0 interface consists of individual
   | routines to read the
   | 
   | STREAMINFO, VORBIS_COMMENT, CUESHEET,
   | and PICTURE blocks, requiring only
   | a filename.
   | 
   | They try to skip any ID3v2 tag at the head
   | of the file.
   |
   */

/**
  | Read the STREAMINFO metadata block
  | of the given FLAC file. This function
  | will try to skip any ID3v2 tag at the head
  | of the file.
  | 
  | -----------
  | @param filename
  | 
  | The path to the FLAC file to read.
  | ----------
  | @param streaminfo
  | 
  | A pointer to space for the STREAMINFO
  | block. Since
  | 
  | StreamMetadata is a simple structure
  | with no memory allocation involved,
  | you pass the address of an existing structure.
  | It need not be initialized. \assert
  | 
  | -----------
  | @code
  | 
  | filename != NULL
  | ----------
  | @code
  | 
  | streaminfo != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid STREAMINFO
  | block was read from \a filename. Returns
  | \c false if there was a memory allocation
  | error, a file decoder error, or the file
  | contained no STREAMINFO block. (A memory
  | allocation error is possible because
  | this function must set up a file decoder.)
  |
  */
pub fn flac_metadata_get_streaminfo(
        filename:   *const u8,
        streaminfo: *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read the VORBIS_COMMENT metadata block
  | of the given FLAC file. This function
  | will try to skip any ID3v2 tag at the head
  | of the file.
  | 
  | -----------
  | @param filename
  | 
  | The path to the FLAC file to read.
  | ----------
  | @param tags
  | 
  | The address where the returned pointer
  | will be stored. The \a tags object must
  | be deleted by the caller using metadata_object_delete().
  | \assert
  | 
  | -----------
  | @code
  | 
  | filename != NULL
  | ----------
  | @code
  | 
  | tags != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid VORBIS_COMMENT
  | block was read from \a filename, and
  | \a *tags will be set to the address of
  | the metadata structure.
  | 
  | Returns \c false if there was a memory
  | allocation error, a file decoder error,
  | or the file contained no VORBIS_COMMENT
  | block, and \a *tags will be set to \c NULL.
  |
  */
pub fn flac_metadata_get_tags(
        filename: *const u8,
        tags:     *mut *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read the CUESHEET metadata block of
  | the given FLAC file. This function will
  | try to skip any ID3v2 tag at the head of
  | the file.
  | 
  | -----------
  | @param filename
  | 
  | The path to the FLAC file to read.
  | ----------
  | @param cuesheet
  | 
  | The address where the returned pointer
  | will be stored. The \a cuesheet object
  | must be deleted by the caller using metadata_object_delete().
  | \assert
  | 
  | -----------
  | @code
  | 
  | filename != NULL
  | ----------
  | @code
  | 
  | cuesheet != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid CUESHEET
  | block was read from \a filename, and
  | \a *cuesheet will be set to the address
  | of the metadata structure. Returns
  | \c false if there was a memory allocation
  | error, a file decoder error, or the file
  | contained no CUESHEET block, and \a
  | *cuesheet will be set to \c NULL.
  |
  */
pub fn flac_metadata_get_cuesheet(
        filename: *const u8,
        cuesheet: *mut *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read a PICTURE metadata block of the
  | given FLAC file. This function will
  | try to skip any ID3v2 tag at the head of
  | the file.
  | 
  | Since there can be more than one PICTURE
  | block in a file, this function takes
  | a number of parameters that act as constraints
  | to the search. The PICTURE block with
  | the largest area matching all the constraints
  | will be returned, or \a *picture will
  | be set to \c NULL if there was no such block.
  | 
  | -----------
  | @param filename
  | 
  | The path to the FLAC file to read.
  | ----------
  | @param picture
  | 
  | The address where the returned pointer
  | will be stored. The \a picture object
  | must be deleted by the caller using metadata_object_delete().
  | ----------
  | @param type
  | 
  | The desired picture type. Use \c -1 to
  | mean "any type".
  | ----------
  | @param mime_type
  | 
  | The desired MIME type, e.g. "image/jpeg".
  | The string will be matched exactly.
  | Use \c NULL to mean "any MIME type".
  | ----------
  | @param description
  | 
  | The desired description. The string
  | will be matched exactly. Use \c NULL
  | to mean "any description".
  | ----------
  | @param max_width
  | 
  | The maximum width in pixels desired.
  | Use \c (unsigned)(-1) to mean "any width".
  | ----------
  | @param max_height
  | 
  | The maximum height in pixels desired.
  | Use \c (unsigned)(-1) to mean "any height".
  | ----------
  | @param max_depth
  | 
  | The maximum color depth in bits-per-pixel
  | desired.
  | 
  | Use \c (unsigned)(-1) to mean "any depth".
  | ----------
  | @param max_colors
  | 
  | The maximum number of colors desired.
  | Use \c (unsigned)(-1) to mean "any number
  | of colors". \assert
  | 
  | -----------
  | @code
  | 
  | filename != NULL
  | ----------
  | @code
  | 
  | picture != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid PICTURE
  | block was read from \a filename, and
  | \a *picture will be set to the address
  | of the metadata structure. Returns
  | \c false if there was a memory allocation
  | error, a file decoder error, or the file
  | contained no PICTURE block, and \a *picture
  | will be set to \c NULL.
  |
  */
pub fn flac_metadata_get_picture(
        filename:    *const u8,
        picture:     *mut *mut StreamMetadata,
        ty:          StreamMetadata_Picture_Type,
        mime_type:   *const u8,
        description: *const u8,
        max_width:   u32,
        max_height:  u32,
        max_depth:   u32,
        max_colors:  u32) -> bool {
    
    todo!();
        /*
        
        */
}


/**
  | Maps a Metadata_SimpleIteratorStatus
  | to a C string.
  | 
  | Using a Metadata_SimpleIteratorStatus
  | as the index to this array will give the
  | string equivalent. The contents should
  | not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const Metadata_SimpleIteratorStatusString[];
    */
}

/**
  | Create a new iterator instance.
  | 
  | -----------
  | @return
  | 
  | Metadata_SimpleIterator*
  | \c NULL if there was an error allocating
  | memory, else the new instance.
  |
  */
pub fn flac_metadata_simple_iterator_new() -> *mut Metadata_SimpleIterator {
    
    todo!();
        /*
        
        */
}

/**
  | Free an iterator instance. Deletes
  | the object pointed to by \a iterator.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  |
  */
pub fn flac_metadata_simple_iterator_delete(iterator: *mut Metadata_SimpleIterator)  {
    
    todo!();
        /*
        
        */
}

/**
  | Get the current status of the iterator.
  | Call this after a function returns \c
  | false to get the reason for the error.
  | Also resets the status to METADATA_SIMPLE_ITERATOR_STATUS_OK.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | 
  | -----------
  | @return
  | 
  | Metadata_SimpleIteratorStatus
  | 
  | The current status of the iterator.
  |
  */
pub fn flac_metadata_simple_iterator_status(iterator: *mut Metadata_SimpleIterator) -> Metadata_SimpleIteratorStatus {
    
    todo!();
        /*
        
        */
}

/**
  | Initialize the iterator to point to
  | the first metadata block in the given
  | FLAC file.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator.
  | ----------
  | @param filename
  | 
  | The path to the FLAC file.
  | ----------
  | @param read_only
  | 
  | If \c true, the FLAC file will be opened
  | in read-only mode; if \c false, the FLAC
  | file will be opened for edit even if no
  | edits are performed.
  | ----------
  | @param preserve_file_stats
  | 
  | If \c true, the owner and modification
  | time will be preserved even if the FLAC
  | file is written to. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | ----------
  | @code
  | 
  | filename != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if a memory allocation
  | error occurs, the file can't be opened,
  | or another error occurs, else \c true.
  |
  */
pub fn flac_metadata_simple_iterator_init(
        iterator:            *mut Metadata_SimpleIterator,
        filename:            *const u8,
        read_only:           bool,
        preserve_file_stats: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Returns \c true if the FLAC file is writable.
  | If \c false, calls to
  | 
  | metadata_simple_iterator_set_block()
  | and
  | 
  | metadata_simple_iterator_insert_block_after()
  | will fail.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | 
  | -----------
  | @return
  | 
  | bool
  | 
  | See above.
  |
  */
pub fn flac_metadata_simple_iterator_is_writable(iterator: *const Metadata_SimpleIterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Moves the iterator forward one metadata
  | block, returning \c false if already
  | at the end.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if already at the
  | last metadata block of the chain, else
  | \c true.
  |
  */
pub fn flac_metadata_simple_iterator_next(iterator: *mut Metadata_SimpleIterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Moves the iterator backward one metadata
  | block, returning \c false if already
  | at the beginning.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if already at the
  | first metadata block of the chain, else
  | \c true.
  |
  */
pub fn flac_metadata_simple_iterator_prev(iterator: *mut Metadata_SimpleIterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Returns a flag telling if the current
  | metadata block is the last.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c true if the current metadata
  | block is the last in the file, else \c
  | false.
  |
  */
pub fn flac_metadata_simple_iterator_is_last(iterator: *const Metadata_SimpleIterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Get the offset of the metadata block
  | at the current position. This avoids
  | reading the actual block data which
  | can save time for large blocks.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | off_t
  | 
  | The offset of the metadata block at the
  | current iterator position.
  | 
  | This is the byte offset relative to the
  | beginning of the file of the current
  | metadata block's header.
  |
  */
pub fn flac_metadata_simple_iterator_get_block_offset(iterator: *const Metadata_SimpleIterator) -> libc::ptrdiff_t {
    
    todo!();
        /*
        
        */
}

/**
  | Get the type of the metadata block at
  | the current position. This avoids reading
  | the actual block data which can save
  | time for large blocks.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | MetadataType
  | 
  | The type of the metadata block at the
  | current iterator position.
  |
  */
pub fn flac_metadata_simple_iterator_get_block_type(iterator: *const Metadata_SimpleIterator) -> MetadataType {
    
    todo!();
        /*
        
        */
}

/**
  | Get the length of the metadata block
  | at the current position. This avoids
  | reading the actual block data which
  | can save time for large blocks.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | unsigned
  | 
  | The length of the metadata block at the
  | current iterator position.
  | 
  | The is same length as that in the <a href="http://xiph.org/flac/format.html#metadata_block_header">metadata
  | block header</a>, i.e. the length of
  | the metadata body that follows the header.
  |
  */
pub fn flac_metadata_simple_iterator_get_block_length(iterator: *const Metadata_SimpleIterator) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | Get the application ID of the \c APPLICATION
  | block at the current position. This
  | avoids reading the actual block data
  | which can save time for large blocks.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param id
  | 
  | A pointer to a buffer of at least \c 4 bytes
  | where the ID will be stored. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | ----------
  | @code
  | 
  | id != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c true if the ID was successfully
  | read, else \c false, in which case you
  | should check metadata_simple_iterator_status()
  | to find out why. If the status is \c METADATA_SIMPLE_ITERATOR_STATUS_ILLEGAL_INPUT,
  | then the current metadata block is not
  | an \c APPLICATION block. Otherwise
  | if the status is \c METADATA_SIMPLE_ITERATOR_STATUS_READ_ERROR
  | or \c METADATA_SIMPLE_ITERATOR_STATUS_SEEK_ERROR,
  | an I/O error occurred and the iterator
  | can no longer be used.
  |
  */
pub fn flac_metadata_simple_iterator_get_application_id(
        iterator: *mut Metadata_SimpleIterator,
        id:       *mut u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Get the metadata block at the current
  | position. You can modify the block but
  | must use metadata_simple_iterator_set_block()
  | to write it back to the FLAC file.
  | 
  | You must call metadata_object_delete()
  | on the returned object when you are finished
  | with it.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | StreamMetadata*
  | 
  | The current metadata block, or \c NULL
  | if there was a memory allocation error.
  |
  */
pub fn flac_metadata_simple_iterator_get_block(iterator: *mut Metadata_SimpleIterator) -> *mut StreamMetadata {
    
    todo!();
        /*
        
        */
}

/**
  | Write a block back to the FLAC file. This
  | function tries to be as efficient as
  | possible; how the block is actually
  | written is shown by the following:
  | 
  | Existing block is a STREAMINFO block
  | and the new block is a
  | 
  | STREAMINFO block: the new block is written
  | in place. Make sure you know what you're
  | doing when changing the values of a
  | 
  | STREAMINFO block.
  | 
  | Existing block is a STREAMINFO block
  | and the new block is a not a STREAMINFO
  | block: this is an error since the first
  | block must be a STREAMINFO block. Returns
  | \c false without altering the file.
  | 
  | Existing block is not a STREAMINFO block
  | and the new block is a
  | 
  | STREAMINFO block: this is an error since
  | there may be only one
  | 
  | STREAMINFO block. Returns \c false
  | without altering the file.
  | 
  | Existing block and new block are the
  | same length: the existing block will
  | be replaced by the new block, written
  | in place.
  | 
  | Existing block is longer than new block:
  | if use_padding is \c true, the existing
  | block will be overwritten in place with
  | the new block followed by a PADDING block,
  | if possible, to make the total size the
  | same as the existing block. Remember
  | that a padding block requires at least
  | four bytes so if the difference in size
  | between the new block and existing block
  | is less than that, the entire file will
  | have to be rewritten, using the new block's
  | exact size. If use_padding is \c false,
  | the entire file will be rewritten, replacing
  | the existing block by the new block.
  | 
  | Existing block is shorter than new block:
  | if use_padding is \c true, the function
  | will try and expand the new block into
  | the following
  | 
  | PADDING block, if it exists and doing
  | so won't shrink the PADDING block to
  | less than 4 bytes. If there is no following
  | PADDING block, or it will shrink to less
  | than 4 bytes, or use_padding is \c false,
  | the entire file is rewritten, replacing
  | the existing block with the new block.
  | Note that in this case any following
  | PADDING block is preserved as is.
  | 
  | After writing the block, the iterator
  | will remain in the same place, i.e. pointing
  | to the new block.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param block
  | 
  | The block to set.
  | ----------
  | @param use_padding
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | ----------
  | @code
  | 
  | block != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if successful, else
  | \c false.
  |
  */
pub fn flac_metadata_simple_iterator_set_block(
        iterator:    *mut Metadata_SimpleIterator,
        block:       *mut StreamMetadata,
        use_padding: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | This is similar to metadata_simple_iterator_set_block()
  | except that instead of writing over
  | an existing block, it appends a block
  | after the existing block. \a use_padding
  | is again used to tell the function to
  | try an expand into following padding
  | in an attempt to avoid rewriting the
  | entire file.
  | 
  | This function will fail and return \c
  | false if given a STREAMINFO block.
  | 
  | After writing the block, the iterator
  | will be pointing to the new block.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param block
  | 
  | The block to set.
  | ----------
  | @param use_padding
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | ----------
  | @code
  | 
  | block != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if successful, else
  | \c false.
  |
  */
pub fn flac_metadata_simple_iterator_insert_block_after(
        iterator:    *mut Metadata_SimpleIterator,
        block:       *mut StreamMetadata,
        use_padding: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Deletes the block at the current position.
  | This will cause the entire FLAC file
  | to be rewritten, unless \a use_padding
  | is \c true, in which case the block will
  | be replaced by an equal-sized PADDING
  | block. The iterator will be left pointing
  | to the block before the one just deleted.
  | 
  | You may not delete the STREAMINFO block.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param use_padding
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_simple_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c true if successful, else
  | \c false.
  |
  */
pub fn flac_metadata_simple_iterator_delete_block(
        iterator:    *mut Metadata_SimpleIterator,
        use_padding: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Maps a Metadata_ChainStatus
  | to a C string.
  | 
  | Using a Metadata_ChainStatus
  | as the index to this array will give the
  | string equivalent. The contents should
  | not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const Metadata_ChainStatusString[];
    */
}

/*
  | ******* Metadata_Chain **********
  |
  */

/**
  | Create a new chain instance.
  | 
  | 
  | -----------
  | @return
  | 
  | Metadata_Chain* \c NULL if there
  | was an error allocating memory, else
  | the new instance.
  |
  */
pub fn flac_metadata_chain_new() -> *mut Metadata_Chain {
    
    todo!();
        /*
        
        */
}

/**
  | Free a chain instance. Deletes the object
  | pointed to by \a chain.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  |
  */
pub fn flac_metadata_chain_delete(chain: *mut Metadata_Chain)  {
    
    todo!();
        /*
        
        */
}

/**
  | Get the current status of the chain.
  | Call this after a function returns \c
  | false to get the reason for the error.
  | Also resets the status to METADATA_CHAIN_STATUS_OK.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | Metadata_ChainStatus
  | 
  | The current status of the chain.
  |
  */
pub fn flac_metadata_chain_status(chain: *mut Metadata_Chain) -> Metadata_ChainStatus {
    
    todo!();
        /*
        
        */
}

/**
  | Read all metadata from a FLAC file into
  | the chain.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param filename
  | 
  | The path to the FLAC file to read. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | ----------
  | @code
  | 
  | filename != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid list of
  | metadata blocks was read from \a filename,
  | else \c false. On failure, check the
  | status with
  | 
  | metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_read(
        chain:    *mut Metadata_Chain,
        filename: *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read all metadata from an Ogg FLAC file
  | into the chain.
  | 
  | -----------
  | @note
  | 
  | Ogg FLAC metadata data writing is not
  | supported yet and
  | 
  | metadata_chain_write() will
  | fail.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param filename
  | 
  | The path to the Ogg FLAC file to read.
  | \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | ----------
  | @code
  | 
  | filename != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid list of
  | metadata blocks was read from \a filename,
  | else \c false. On failure, check the
  | status with
  | 
  | metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_read_ogg(
        chain:    *mut Metadata_Chain,
        filename: *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read all metadata from a FLAC stream
  | into the chain via I/O callbacks.
  | 
  | The \a handle need only be open for reading,
  | but must be seekable.
  | 
  | The equivalent minimum stdio fopen()
  | file mode is \c "r" (or \c "rb" for Windows).
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param handle
  | 
  | The I/O handle of the FLAC stream to read.
  | The handle will NOT be closed after the
  | metadata is read; that is the duty of
  | the caller.
  | ----------
  | @param callbacks
  | 
  | A set of callbacks to use for I/O. The
  | mandatory callbacks are \a read, \a
  | seek, and \a tell. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid list of
  | metadata blocks was read from \a handle,
  | else \c false. On failure, check the
  | status with
  | 
  | metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_read_with_callbacks(
        chain:     *mut Metadata_Chain,
        handle:    IOHandle,
        callbacks: IOCallbacks) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Read all metadata from an Ogg FLAC stream
  | into the chain via I/O callbacks.
  | 
  | The \a handle need only be open for reading,
  | but must be seekable.
  | 
  | The equivalent minimum stdio fopen()
  | file mode is \c "r" (or \c "rb" for Windows).
  | 
  | -----------
  | @note
  | 
  | Ogg FLAC metadata data writing is not
  | supported yet and
  | 
  | metadata_chain_write() will
  | fail.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param handle
  | 
  | The I/O handle of the Ogg FLAC stream
  | to read. The handle will NOT be closed
  | after the metadata is read; that is the
  | duty of the caller.
  | ----------
  | @param callbacks
  | 
  | A set of callbacks to use for I/O. The
  | mandatory callbacks are \a read, \a
  | seek, and \a tell. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if a valid list of
  | metadata blocks was read from \a handle,
  | else \c false. On failure, check the
  | status with
  | 
  | metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_read_ogg_with_callbacks(
        chain:     *mut Metadata_Chain,
        handle:    IOHandle,
        callbacks: IOCallbacks) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Checks if writing the given chain would
  | require the use of a temporary file,
  | or if it could be written in place.
  | 
  | Under certain conditions, padding
  | can be utilized so that writing edited
  | metadata back to the FLAC file does not
  | require rewriting the entire file.
  | If rewriting is required, then a temporary
  | workfile is required. When writing
  | metadata using callbacks, you must
  | check this function to know whether
  | to call
  | 
  | metadata_chain_write_with_callbacks()
  | or
  | 
  | metadata_chain_write_with_callbacks_and_tempfile().
  | When writing with metadata_chain_write(),
  | the temporary file is handled internally.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param use_padding
  | 
  | Whether or not padding will be allowed
  | to be used during the write. The value
  | of \a use_padding given here must match
  | the value later passed to
  | 
  | metadata_chain_write_with_callbacks()
  | or
  | 
  | metadata_chain_write_with_callbacks_with_tempfile().
  | \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if writing the current
  | chain would require a tempfile, or \c
  | false if metadata can be written in place.
  |
  */
pub fn flac_metadata_chain_check_if_tempfile_needed(
        chain:       *mut Metadata_Chain,
        use_padding: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Write all metadata out to the FLAC file.
  | This function tries to be as efficient
  | as possible; how the metadata is actually
  | written is shown by the following:
  | 
  | If the current chain is the same size
  | as the existing metadata, the new data
  | is written in place.
  | 
  | If the current chain is longer than the
  | existing metadata, and \a use_padding
  | is \c true, and the last block is a PADDING
  | block of sufficient length, the function
  | will truncate the final padding block
  | so that the overall size of the metadata
  | is the same as the existing metadata,
  | and then just rewrite the metadata.
  | Otherwise, if not all of the above conditions
  | are met, the entire FLAC file must be
  | rewritten.
  | 
  | If you want to use padding this way it
  | is a good idea to call
  | 
  | metadata_chain_sort_padding()
  | first so that you have the maximum amount
  | of padding to work with, unless you need
  | to preserve ordering of the PADDING
  | blocks for some reason.
  | 
  | If the current chain is shorter than
  | the existing metadata, and \a use_padding
  | is \c true, and the final block is a PADDING
  | block, the padding is extended to make
  | the overall size the same as the existing
  | data. If \a use_padding is \c true and
  | the last block is not a PADDING block,
  | a new
  | 
  | PADDING block is added to the end of the
  | new data to make it the same size as the
  | existing data (if possible, see the
  | note to
  | 
  | metadata_simple_iterator_set_block()
  | about the four byte limit) and the new
  | data is written in place. If none of the
  | above apply or \a use_padding is \c false,
  | the entire FLAC file is rewritten.
  | 
  | If \a preserve_file_stats is \c true,
  | the owner and modification time will
  | be preserved even if the FLAC file is
  | written.
  | 
  | For this write function to be used, the
  | chain must have been read with
  | 
  | metadata_chain_read()/metadata_chain_read_ogg(),
  | not
  | 
  | metadata_chain_read_with_callbacks()/metadata_chain_read_ogg_with_callbacks().
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param use_padding
  | 
  | See above.
  | ----------
  | @param preserve_file_stats
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if the write succeeded,
  | else \c false. On failure, check the
  | status with metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_write(
        chain:               *mut Metadata_Chain,
        use_padding:         bool,
        preserve_file_stats: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Write all metadata out to a FLAC stream
  | via callbacks. (See metadata_chain_write()
  | for the details on how padding is used
  | to write metadata in place if possible.)
  | 
  | The \a handle must be open for updating
  | and be seekable. The equivalent minimum
  | stdio fopen() file mode is \c "r+" (or
  | \c "r+b" for Windows).
  | 
  | For this write function to be used, the
  | chain must have been read with
  | 
  | metadata_chain_read_with_callbacks()/metadata_chain_read_ogg_with_callbacks(),
  | not metadata_chain_read()/metadata_chain_read_ogg().
  | 
  | Also, metadata_chain_check_if_tempfile_needed()
  | must have returned \c false.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param use_padding
  | 
  | See metadata_chain_write()
  | ----------
  | @param handle
  | 
  | The I/O handle of the FLAC stream to write.
  | The handle will NOT be closed after the
  | metadata is written; that is the duty
  | of the caller.
  | ----------
  | @param callbacks
  | 
  | A set of callbacks to use for I/O. The
  | mandatory callbacks are \a write and
  | \a seek. \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if the write succeeded,
  | else \c false. On failure, check the
  | status with metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_write_with_callbacks(
        chain:       *mut Metadata_Chain,
        use_padding: bool,
        handle:      IOHandle,
        callbacks:   IOCallbacks) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Write all metadata out to a FLAC stream
  | via callbacks. (See metadata_chain_write()
  | for the details on how padding is used
  | to write metadata in place if possible.)
  | 
  | This version of the write-with-callbacks
  | function must be used when
  | 
  | metadata_chain_check_if_tempfile_needed()
  | returns true. In this function, you
  | must supply an I/O handle corresponding
  | to the
  | 
  | FLAC file to edit, and a temporary handle
  | to which the new FLAC file will be written.
  | It is the caller's job to move this temporary
  | 
  | FLAC file on top of the original FLAC
  | file to complete the metadata edit.
  | 
  | The \a handle must be open for reading
  | and be seekable. The equivalent minimum
  | stdio fopen() file mode is \c "r" (or
  | \c "rb" for Windows).
  | 
  | The \a temp_handle must be open for writing.
  | The equivalent minimum stdio fopen()
  | file mode is \c "w" (or \c "wb" for Windows).
  | It should be an empty stream, or at least
  | positioned at the start-of-file (in
  | which case it is the caller's duty to
  | truncate it on return).
  | 
  | For this write function to be used, the
  | chain must have been read with
  | 
  | metadata_chain_read_with_callbacks()/metadata_chain_read_ogg_with_callbacks(),
  | not metadata_chain_read()/metadata_chain_read_ogg().
  | 
  | Also, metadata_chain_check_if_tempfile_needed()
  | must have returned \c true.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain.
  | ----------
  | @param use_padding
  | 
  | See metadata_chain_write()
  | ----------
  | @param handle
  | 
  | The I/O handle of the original FLAC stream
  | to read.
  | 
  | The handle will NOT be closed after the
  | metadata is written; that is the duty
  | of the caller.
  | ----------
  | @param callbacks
  | 
  | A set of callbacks to use for I/O on \a
  | handle.
  | 
  | The mandatory callbacks are \a read,
  | \a seek, and \a eof.
  | ----------
  | @param temp_handle
  | 
  | The I/O handle of the FLAC stream to write.
  | The handle will NOT be closed after the
  | metadata is written; that is the duty
  | of the caller.
  | ----------
  | @param temp_callbacks
  | 
  | A set of callbacks to use for I/O on temp_handle.
  | 
  | The only mandatory callback is \a write.
  | \assert
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if the write succeeded,
  | else \c false. On failure, check the
  | status with metadata_chain_status().
  |
  */
pub fn flac_metadata_chain_write_with_callbacks_and_tempfile(
        chain:          *mut Metadata_Chain,
        use_padding:    bool,
        handle:         IOHandle,
        callbacks:      IOCallbacks,
        temp_handle:    IOHandle,
        temp_callbacks: IOCallbacks) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Merge adjacent PADDING blocks into
  | a single block.
  | 
  | -----------
  | @note
  | 
  | This function does not write to the FLAC
  | file, it only modifies the chain.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain. \assert
  | 
  | -----------
  | @warning
  | 
  | Any iterator on the current chain will
  | become invalid after this call. You
  | should delete the iterator and get a
  | new one.
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  |
  */
pub fn flac_metadata_chain_merge_padding(chain: *mut Metadata_Chain)  {
    
    todo!();
        /*
        
        */
}

/**
  | This function will move all PADDING
  | blocks to the end on the metadata, then
  | merge them into a single block.
  | 
  | -----------
  | @note
  | 
  | This function does not write to the FLAC
  | file, it only modifies the chain.
  | 
  | -----------
  | @param chain
  | 
  | A pointer to an existing chain. \assert
  | 
  | -----------
  | @warning
  | 
  | Any iterator on the current chain will
  | become invalid after this call. You
  | should delete the iterator and get a
  | new one.
  | 
  | -----------
  | @code
  | 
  | chain != NULL
  |
  */
pub fn flac_metadata_chain_sort_padding(chain: *mut Metadata_Chain)  {
    
    todo!();
        /*
        
        */
}

/* ------------ Metadata_Iterator  ------------ */

/**
  | Create a new iterator instance.
  | 
  | 
  | -----------
  | @return
  | 
  | Metadata_Iterator* \c NULL
  | if there was an error allocating memory,
  | else the new instance.
  |
  */
pub fn flac_metadata_iterator_new() -> *mut Metadata_Iterator {
    
    todo!();
        /*
        
        */
}

/**
  | Free an iterator instance. Deletes
  | the object pointed to by \a iterator.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  |
  */
pub fn flac_metadata_iterator_delete(iterator: *mut Metadata_Iterator)  {
    
    todo!();
        /*
        
        */
}

/**
  | Initialize the iterator to point to
  | the first metadata block in the given
  | chain.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing iterator.
  | ----------
  | @param chain
  | 
  | A pointer to an existing and initialized
  | (read) chain. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | ----------
  | @code
  | 
  | chain != NULL
  |
  */
pub fn flac_metadata_iterator_init(
        iterator: *mut Metadata_Iterator,
        chain:    *mut Metadata_Chain)  {
    
    todo!();
        /*
        
        */
}

/**
  | Moves the iterator forward one metadata
  | block, returning \c false if already
  | at the end.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if already at the
  | last metadata block of the chain, else
  | \c true.
  |
  */
pub fn flac_metadata_iterator_next(iterator: *mut Metadata_Iterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Moves the iterator backward one metadata
  | block, returning \c false if already
  | at the beginning.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if already at the
  | first metadata block of the chain, else
  | \c true.
  |
  */
pub fn flac_metadata_iterator_prev(iterator: *mut Metadata_Iterator) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Get the type of the metadata block at
  | the current position.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | MetadataType
  | 
  | The type of the metadata block at the
  | current iterator position.
  |
  */
pub fn flac_metadata_iterator_get_block_type(iterator: *const Metadata_Iterator) -> MetadataType {
    
    todo!();
        /*
        
        */
}

/**
  | Get the metadata block at the current
  | position. You can modify the block in
  | place but must write the chain before
  | the changes are reflected to the FLAC
  | file. You do not need to call
  | 
  | metadata_iterator_set_block()
  | to reflect the changes; the pointer
  | returned by metadata_iterator_get_block()
  | points directly into the chain.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator. \assert
  | 
  | -----------
  | @warning
  | 
  | Do not call metadata_object_delete()
  | on the returned object; to delete a block
  | use metadata_iterator_delete_block().
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | StreamMetadata*
  | 
  | The current metadata block.
  |
  */
pub fn flac_metadata_iterator_get_block(iterator: *mut Metadata_Iterator) -> *mut StreamMetadata {
    
    todo!();
        /*
        
        */
}

/**
  | Set the metadata block at the current
  | position, replacing the existing block.
  | The new block passed in becomes owned
  | by the chain and it will be deleted when
  | the chain is deleted.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param block
  | 
  | A pointer to a metadata block. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | ----------
  | @code
  | 
  | block != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if the conditions
  | in the above description are not met,
  | or a memory allocation error occurs,
  | otherwise \c true.
  |
  */
pub fn flac_metadata_iterator_set_block(
        iterator: *mut Metadata_Iterator,
        block:    *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Removes the current block from the chain.
  | If \a replace_with_padding is \c true,
  | the block will instead be replaced with
  | a padding block of equal size. You can
  | not delete the STREAMINFO block. The
  | iterator will be left pointing to the
  | block before the one just "deleted",
  | even if \a replace_with_padding is
  | \c true.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param replace_with_padding
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if the conditions
  | in the above description are not met,
  | otherwise \c true.
  |
  */
pub fn flac_metadata_iterator_delete_block(
        iterator:             *mut Metadata_Iterator,
        replace_with_padding: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a new block before the current
  | block. You cannot insert a block before
  | the first STREAMINFO block. You cannot
  | insert a STREAMINFO block as there can
  | be only one, the one that already exists
  | at the head when you read in a chain. The
  | chain takes ownership of the new block
  | and it will be deleted when the chain
  | is deleted. The iterator will be left
  | pointing to the new block.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param block
  | 
  | A pointer to a metadata block to insert.
  | \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if the conditions
  | in the above description are not met,
  | or a memory allocation error occurs,
  | otherwise \c true.
  |
  */
pub fn flac_metadata_iterator_insert_block_before(
        iterator: *mut Metadata_Iterator,
        block:    *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a new block after the current
  | block. You cannot insert a STREAMINFO
  | block as there can be only one, the one
  | that already exists at the head when
  | you read in a chain. The chain takes ownership
  | of the new block and it will be deleted
  | when the chain is deleted. The iterator
  | will be left pointing to the new block.
  | 
  | -----------
  | @param iterator
  | 
  | A pointer to an existing initialized
  | iterator.
  | ----------
  | @param block
  | 
  | A pointer to a metadata block to insert.
  | \assert
  | 
  | -----------
  | @code
  | 
  | iterator != NULL
  | \a iterator has been successfully initialized
  | with
  | 
  | metadata_iterator_init()
  | 
  | -----------
  | @return
  | 
  | bool \c false if the conditions
  | in the above description are not met,
  | or a memory allocation error occurs,
  | otherwise \c true.
  |
  */
pub fn flac_metadata_iterator_insert_block_after(
        iterator: *mut Metadata_Iterator,
        block:    *mut StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/*
  | \defgroup flac_metadata_object FLAC/metadata.h:
  | metadata object methods \ingroup flac_metadata
  | 
  | -----------
  | @brief
  | 
  | This module contains methods for manipulating
  | FLAC metadata objects.
  | 
  | Since many are variable length we have
  | to be careful about the memory management.
  | We decree that all pointers to data in
  | the object are owned by the object and
  | memory-managed by the object.
  | 
  | Use the metadata_object_new()
  | and metadata_object_delete()
  | functions to create all instances.
  | When using the
  | 
  | metadata_object_set_*() functions
  | to set pointers to data, set \a copy to
  | \c true to have the function make it's
  | own copy of the data, or to \c false to
  | give the object ownership of your data.
  | In the latter case your pointer must
  | be freeable by free() and will be free()d
  | when the object is metadata_object_delete()d.
  | It is legal to pass a null pointer as the
  | data pointer to a metadata_object_set_*()
  | function as long as the length argument
  | is 0 and the \a copy argument is \c false.
  | 
  | The metadata_object_new()
  | and metadata_object_clone()
  | function will return \c NULL in the case
  | of a memory allocation error, otherwise
  | a new object. The metadata_object_set_*()
  | functions return \c false in the case
  | of a memory allocation error.
  | 
  | We don't have the convenience of C++
  | here, so note that the library relies
  | on you to keep the types straight. In
  | other words, if you pass, for example,
  | a StreamMetadata* that represents
  | a STREAMINFO block to
  | 
  | metadata_object_application_set_data(),
  | you will get an assertion failure.
  | 
  | For convenience the metadata_object_vorbiscomment_*()
  | functions maintain a trailing NUL on
  | each Vorbis comment entry. This is not
  | counted toward the length or stored
  | in the stream, but it can make working
  | with plain comments (those that don't
  | contain embedded-NULs in the value)
  | easier.
  | 
  | Entries passed into these functions
  | have trailing NULs added if missing,
  | and returned entries are guaranteed
  | to have a trailing NUL.
  | 
  | The metadata_object_vorbiscomment_*()
  | functions that take a Vorbis comment
  | entry/name/value will first validate
  | that it complies with the Vorbis comment
  | specification and return false if it
  | does not.
  | 
  | There is no need to recalculate the length
  | field on metadata blocks you have modified.
  | They will be calculated automatically
  | before they are written back to a file.
  |
  */

/**
  | Create a new metadata object instance
  | of the given type.
  | 
  | The object will be "empty"; i.e. values
  | and data pointers will be \c 0, with the
  | exception of METADATA_TYPE_VORBIS_COMMENT,
  | which will have the vendor string set
  | (but zero comments).
  | 
  | Do not pass in a value greater than or
  | equal to \a METADATA_TYPE_UNDEFINED
  | unless you really know what you're doing.
  | 
  | -----------
  | @param type
  | 
  | Type of object to create
  | 
  | -----------
  | @return
  | 
  | StreamMetadata* \c NULL if there
  | was an error allocating memory or the
  | type code is greater than MAX_METADATA_TYPE_CODE,
  | else the new instance.
  |
  */
pub fn flac_metadata_object_new(ty: MetadataType) -> *mut StreamMetadata {
    
    todo!();
        /*
        
        */
}

/**
  | Create a copy of an existing metadata
  | object.
  | 
  | The copy is a "deep" copy, i.e. dynamically
  | allocated data within the object is
  | also copied. The caller takes ownership
  | of the new block and is responsible for
  | freeing it with metadata_object_delete().
  | 
  | -----------
  | @param object
  | 
  | Pointer to object to copy. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | 
  | -----------
  | @return
  | 
  | StreamMetadata* \c NULL if there
  | was an error allocating memory, else
  | the new instance.
  |
  */
pub fn flac_metadata_object_clone(object: *const StreamMetadata) -> *mut StreamMetadata {
    
    todo!();
        /*
        
        */
}

/**
  | Free a metadata object. Deletes the
  | object pointed to by \a object.
  | 
  | The delete is a "deep" delete, i.e. dynamically
  | allocated data within the object is
  | also deleted.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing object. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  |
  */
pub fn flac_metadata_object_delete(object: *mut StreamMetadata)  {
    
    todo!();
        /*
        
        */
}

/**
  | Compares two metadata objects.
  | 
  | The compare is "deep", i.e. dynamically
  | allocated data within the object is
  | also compared.
  | 
  | -----------
  | @param block1
  | 
  | A pointer to an existing object.
  | ----------
  | @param block2
  | 
  | A pointer to an existing object. \assert
  | 
  | -----------
  | @code
  | 
  | block1 != NULL
  | ----------
  | @code
  | 
  | block2 != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c true if objects are identical,
  | else \c false.
  |
  */
pub fn flac_metadata_object_is_equal(
        block1: *const StreamMetadata,
        block2: *const StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets the application data of an APPLICATION
  | block.
  | 
  | If \a copy is \c true, a copy of the data
  | is stored; otherwise, the object takes
  | ownership of the pointer. The existing
  | data will be freed if this function is
  | successful, otherwise the original
  | data will remain if \a copy is \c true
  | and malloc() fails.
  | 
  | -----------
  | @note
  | 
  | It is safe to pass a const pointer to \a
  | data if \a copy is \c true.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing APPLICATION
  | object.
  | ----------
  | @param data
  | 
  | A pointer to the data to set.
  | ----------
  | @param length
  | 
  | The length of \a data in bytes.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_APPLICATION
  | ----------
  | @code
  | 
  | (data != NULL && length > 0) ||
  |  (data == NULL && length == 0 && copy == false)
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_application_set_data(
        object: *mut StreamMetadata,
        data:   *mut u8,
        length: u32,
        copy:   bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Resize the seekpoint array.
  | 
  | If the size shrinks, elements will truncated;
  | if it grows, new placeholder points
  | will be added to the end.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param new_num_points
  | 
  | The desired length of the array; may
  | be \c 0. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | (object->data.seek_table.points == NULL && object->data.seek_table.num_points == 0) ||
  |  (object->data.seek_table.points != NULL && object->data.seek_table.num_points > 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | error, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_resize_points(
        object:         *mut StreamMetadata,
        new_num_points: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Set a seekpoint in a seektable.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param point_num
  | 
  | Index into seekpoint array to set.
  | ----------
  | @param point
  | 
  | The point to set. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | object->data.seek_table.num_points > point_num
  |
  */
pub fn flac_metadata_object_seektable_set_point(
        object:    *mut StreamMetadata,
        point_num: u32,
        point:     StreamMetadata_SeekPoint)  {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a seekpoint into a seektable.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param point_num
  | 
  | Index into seekpoint array to set.
  | ----------
  | @param point
  | 
  | The point to set. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | object->data.seek_table.num_points >= point_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | error, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_insert_point(
        object:    *mut StreamMetadata,
        point_num: u32,
        point:     StreamMetadata_SeekPoint) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Delete a seekpoint from a seektable.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param point_num
  | 
  | Index into seekpoint array to set. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | object->data.seek_table.num_points > point_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | error, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_delete_point(
        object:    *mut StreamMetadata,
        point_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a seektable to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the seektable.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | 
  | -----------
  | @return
  | 
  | bool \c false if seek table is
  | illegal, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_is_legal(object: *const StreamMetadata) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Append a number of placeholder points
  | to the end of a seek table.
  | 
  | -----------
  | @note
  | 
  | As with the other ..._seektable_template_...
  | functions, you should call metadata_object_seektable_template_sort()
  | when finished to make the seek table
  | legal.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param num
  | 
  | The number of placeholder points to
  | append. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_append_placeholders(
        object: *mut StreamMetadata,
        num:    u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Append a specific seek point template
  | to the end of a seek table.
  | 
  | -----------
  | @note
  | 
  | As with the other ..._seektable_template_...
  | functions, you should call metadata_object_seektable_template_sort()
  | when finished to make the seek table
  | legal.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param sample_number
  | 
  | The sample number of the seek point template.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_append_point(
        object:        *mut StreamMetadata,
        sample_number: u64) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Append specific seek point templates
  | to the end of a seek table.
  | 
  | -----------
  | @note
  | 
  | As with the other ..._seektable_template_...
  | functions, you should call metadata_object_seektable_template_sort()
  | when finished to make the seek table
  | legal.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param sample_numbers
  | 
  | An array of sample numbers for the seek
  | points.
  | ----------
  | @param num
  | 
  | The number of seek point templates to
  | append. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_append_points(
        object:         *mut StreamMetadata,
        sample_numbers: &[u64],
        num:            u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Append a set of evenly-spaced seek point
  | templates to the end of a seek table.
  | 
  | -----------
  | @note
  | 
  | As with the other ..._seektable_template_...
  | functions, you should call metadata_object_seektable_template_sort()
  | when finished to make the seek table
  | legal.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param num
  | 
  | The number of placeholder points to
  | append.
  | ----------
  | @param total_samples
  | 
  | The total number of samples to be encoded;
  | the seekpoints will be spaced approximately
  | \a total_samples / \a num samples apart.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | total_samples > 0
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_append_spaced_points(
        object:        *mut StreamMetadata,
        num:           u32,
        total_samples: u64) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Append a set of evenly-spaced seek point
  | templates to the end of a seek table.
  | 
  | -----------
  | @note
  | 
  | As with the other ..._seektable_template_...
  | functions, you should call metadata_object_seektable_template_sort()
  | when finished to make the seek table
  | legal.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing SEEKTABLE object.
  | ----------
  | @param samples
  | 
  | The number of samples apart to space
  | the placeholder points. The first point
  | will be at sample \c 0, the second at sample
  | \a samples, then 2*\a samples, and so
  | on. As long as \a samples and \a total_samples
  | are greater than \c 0, there will always
  | be at least one seekpoint at sample \c
  | 0.
  | ----------
  | @param total_samples
  | 
  | The total number of samples to be encoded;
  | the seekpoints will be spaced \a samples
  | samples apart. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | ----------
  | @code
  | 
  | samples > 0
  | ----------
  | @code
  | 
  | total_samples > 0
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_append_spaced_points_by_samples(
        object:        *mut StreamMetadata,
        samples:       u32,
        total_samples: u64) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sort a seek table's seek points according
  | to the format specification, removing
  | duplicates.
  | 
  | -----------
  | @param object
  | 
  | A pointer to a seek table to be sorted.
  | ----------
  | @param compact
  | 
  | If \c false, behaves like format_seektable_sort().
  | 
  | If \c true, duplicates are deleted and
  | the seek table is shrunk appropriately;
  | the number of placeholder points present
  | in the seek table will be the same after
  | the call as before. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_SEEKTABLE
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_seektable_template_sort(
        object:  *mut StreamMetadata,
        compact: bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets the vendor string in a VORBIS_COMMENT
  | block.
  | 
  | For convenience, a trailing NUL is added
  | to the entry if it doesn't have one already.
  | 
  | If \a copy is \c true, a copy of the entry
  | is stored; otherwise, the object takes
  | ownership of the \c entry.entry pointer.
  | 
  | -----------
  | @note
  | 
  | If this function returns \c false, the
  | caller still owns the pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param entry
  | 
  | The entry to set the vendor string to.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0) ||
  |  (entry.entry == NULL && entry.length == 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_set_vendor_string(
        object: *mut StreamMetadata,
        entry:  StreamMetadata_VorbisComment_Entry,
        copy:   bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Resize the comment array.
  | 
  | If the size shrinks, elements will truncated;
  | if it grows, new empty fields will be
  | added to the end.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param new_num_comments
  | 
  | The desired length of the array; may
  | be \c 0. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | (object->data.vorbis_comment.comments == NULL && object->data.vorbis_comment.num_comments == 0) ||
  |  (object->data.vorbis_comment.comments != NULL && object->data.vorbis_comment.num_comments > 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails, else \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_resize_comments(
        object:           *mut StreamMetadata,
        new_num_comments: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets a comment in a VORBIS_COMMENT block.
  | 
  | For convenience, a trailing NUL is added
  | to the entry if it doesn't have one already.
  | 
  | If \a copy is \c true, a copy of the entry
  | is stored; otherwise, the object takes
  | ownership of the \c entry.entry pointer.
  | 
  | -----------
  | @note
  | 
  | If this function returns \c false, the
  | caller still owns the pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param comment_num
  | 
  | Index into comment array to set.
  | ----------
  | @param entry
  | 
  | The entry to set the comment to.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | comment_num < object->data.vorbis_comment.num_comments
  | ----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0) ||
  |  (entry.entry == NULL && entry.length == 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_set_comment(
        object:      *mut StreamMetadata,
        comment_num: u32,
        entry:       StreamMetadata_VorbisComment_Entry,
        copy:        bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a comment in a VORBIS_COMMENT
  | block at the given index.
  | 
  | For convenience, a trailing NUL is added
  | to the entry if it doesn't have one already.
  | 
  | If \a copy is \c true, a copy of the entry
  | is stored; otherwise, the object takes
  | ownership of the \c entry.entry pointer.
  | 
  | -----------
  | @note
  | 
  | If this function returns \c false, the
  | caller still owns the pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param comment_num
  | 
  | The index at which to insert the comment.
  | The comments at and after \a comment_num
  | move right one position.
  | 
  | To append a comment to the end, set \a
  | comment_num to \c object->data.vorbis_comment.num_comments
  | .
  | ----------
  | @param entry
  | 
  | The comment to insert.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | object->data.vorbis_comment.num_comments >= comment_num
  | ----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0) ||
  |  (entry.entry == NULL && entry.length == 0 && copy == false)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_insert_comment(
        object:      *mut StreamMetadata,
        comment_num: u32,
        entry:       StreamMetadata_VorbisComment_Entry,
        copy:        bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Appends a comment to a VORBIS_COMMENT
  | block.
  | 
  | For convenience, a trailing NUL is added
  | to the entry if it doesn't have one already.
  | 
  | If \a copy is \c true, a copy of the entry
  | is stored; otherwise, the object takes
  | ownership of the \c entry.entry pointer.
  | 
  | -----------
  | @note
  | 
  | If this function returns \c false, the
  | caller still owns the pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param entry
  | 
  | The comment to insert.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0) ||
  |  (entry.entry == NULL && entry.length == 0 && copy == false)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_append_comment(
        object: *mut StreamMetadata,
        entry:  StreamMetadata_VorbisComment_Entry,
        copy:   bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Replaces comments in a VORBIS_COMMENT
  | block with a new one.
  | 
  | For convenience, a trailing NUL is added
  | to the entry if it doesn't have one already.
  | 
  | Depending on the the value of \a all,
  | either all or just the first comment
  | whose field name(s) match the given
  | entry's name will be replaced by the
  | given entry. If no comments match, \a
  | entry will simply be appended.
  | 
  | If \a copy is \c true, a copy of the entry
  | is stored; otherwise, the object takes
  | ownership of the \c entry.entry pointer.
  | 
  | -----------
  | @note
  | 
  | If this function returns \c false, the
  | caller still owns the pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param entry
  | 
  | The comment to insert.
  | ----------
  | @param all
  | 
  | If \c true, all comments whose field
  | name matches \a entry's field name will
  | be removed, and \a entry will be inserted
  | at the position of the first matching
  | comment. If \c false, only the first
  | comment whose field name matches \a
  | entry's field name will be replaced
  | with \a entry.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0) ||
  |  (entry.entry == NULL && entry.length == 0 && copy == false)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_replace_comment(
        object: *mut StreamMetadata,
        entry:  StreamMetadata_VorbisComment_Entry,
        all:    bool,
        copy:   bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Delete a comment in a VORBIS_COMMENT
  | block at the given index.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param comment_num
  | 
  | The index of the comment to delete. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | object->data.vorbis_comment.num_comments > comment_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_delete_comment(
        object:      *mut StreamMetadata,
        comment_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Creates a Vorbis comment entry from
  | NUL-terminated name and value strings.
  | 
  | On return, the filled-in \a entry->entry
  | pointer will point to malloc()ed memory
  | and shall be owned by the caller. For
  | convenience the entry will have a terminating
  | NUL.
  | 
  | -----------
  | @param entry
  | 
  | A pointer to a Vorbis comment entry.
  | The entry's \c entry pointer should
  | not point to allocated memory as it will
  | be overwritten.
  | ----------
  | @param field_name
  | 
  | The field name in ASCII, \c NUL terminated.
  | ----------
  | @param field_value
  | 
  | The field value in UTF-8, \c NUL terminated.
  | \assert
  | 
  | -----------
  | @code
  | 
  | entry != NULL
  | ----------
  | @code
  | 
  | field_name != NULL
  | ----------
  | @code
  | 
  | field_value != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if malloc() fails,
  | or if \a field_name or \a field_value
  | does not comply with the Vorbis comment
  | specification, else \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_entry_from_name_value_pair(
        entry:       *mut StreamMetadata_VorbisComment_Entry,
        field_name:  *const u8,
        field_value: *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Splits a Vorbis comment entry into NUL-terminated
  | name and value strings.
  | 
  | The returned pointers to name and value
  | will be allocated by malloc() and shall
  | be owned by the caller.
  | 
  | -----------
  | @param entry
  | 
  | An existing Vorbis comment entry.
  | ----------
  | @param field_name
  | 
  | The address of where the returned pointer
  | to the field name will be stored.
  | ----------
  | @param field_value
  | 
  | The address of where the returned pointer
  | to the field value will be stored. \assert
  | 
  | -----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0)
  | ----------
  | @code
  | 
  | memchr(entry.entry, '=', entry.length) != NULL
  | ----------
  | @code
  | 
  | field_name != NULL
  | ----------
  | @code
  | 
  | field_value != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | fails or \a entry does not comply with
  | the
  | 
  | Vorbis comment specification, else
  | \c true.
  |
  */
pub fn flac_metadata_object_vorbiscomment_entry_to_name_value_pair(
        entry:       StreamMetadata_VorbisComment_Entry,
        field_name:  *mut *mut u8,
        field_value: *mut *mut u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check if the given Vorbis comment entry's
  | field name matches the given field name.
  | 
  | -----------
  | @param entry
  | 
  | An existing Vorbis comment entry.
  | ----------
  | @param field_name
  | 
  | The field name to check.
  | ----------
  | @param field_name_length
  | 
  | The length of \a field_name, not including
  | the terminating \c NUL. \assert
  | 
  | -----------
  | @code
  | 
  | (entry.entry != NULL && entry.length > 0)
  | 
  | -----------
  | @return
  | 
  | bool \c true if the field names
  | match, else \c false
  |
  */
pub fn flac_metadata_object_vorbiscomment_entry_matches(
        entry:             StreamMetadata_VorbisComment_Entry,
        field_name:        *const u8,
        field_name_length: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Find a Vorbis comment with the given
  | field name.
  | 
  | The search begins at entry number \a
  | offset; use an offset of 0 to search from
  | the beginning of the comment array.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param offset
  | 
  | The offset into the comment array from
  | where to start the search.
  | ----------
  | @param field_name
  | 
  | The field name of the comment to find.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | ----------
  | @code
  | 
  | field_name != NULL
  | 
  | -----------
  | @return
  | 
  | int
  | 
  | The offset in the comment array of the
  | first comment whose field name matches
  | \a field_name, or \c -1 if no match was
  | found.
  |
  */
pub fn flac_metadata_object_vorbiscomment_find_entry_from(
        object:     *const StreamMetadata,
        offset:     u32,
        field_name: *const u8) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Remove first Vorbis comment matching
  | the given field name.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param field_name
  | 
  | The field name of comment to delete.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | 
  | -----------
  | @return
  | 
  | int \c -1 for memory allocation error,
  | \c 0 for no matching entries, \c 1 for
  | one matching entry deleted.
  |
  */
pub fn flac_metadata_object_vorbiscomment_remove_entry_matching(
        object:     *mut StreamMetadata,
        field_name: *const u8) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Remove all Vorbis comments matching
  | the given field name.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing VORBIS_COMMENT
  | object.
  | ----------
  | @param field_name
  | 
  | The field name of comments to delete.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_VORBIS_COMMENT
  | 
  | -----------
  | @return
  | 
  | int \c -1 for memory allocation error,
  | \c 0 for no matching entries, else the
  | number of matching entries deleted.
  |
  */
pub fn flac_metadata_object_vorbiscomment_remove_entries_matching(
        object:     *mut StreamMetadata,
        field_name: *const u8) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Create a new CUESHEET track instance.
  | 
  | The object will be "empty"; i.e. values
  | and data pointers will be \c 0.
  | 
  | 
  | -----------
  | @return
  | 
  | StreamMetadata_CueSheet_Track*
  | \c NULL if there was an error allocating
  | memory, else the new instance.
  |
  */
pub fn flac_metadata_object_cuesheet_track_new() -> *mut StreamMetadata_CueSheet_Track {
    
    todo!();
        /*
        
        */
}

/**
  | Create a copy of an existing CUESHEET
  | track object.
  | 
  | The copy is a "deep" copy, i.e. dynamically
  | allocated data within the object is
  | also copied. The caller takes ownership
  | of the new object and is responsible
  | for freeing it with
  | 
  | metadata_object_cuesheet_track_delete().
  | 
  | -----------
  | @param object
  | 
  | Pointer to object to copy. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | 
  | -----------
  | @return
  | 
  | StreamMetadata_CueSheet_Track*
  | \c NULL if there was an error allocating
  | memory, else the new instance.
  |
  */
pub fn flac_metadata_object_cuesheet_track_clone(object: *const StreamMetadata_CueSheet_Track) -> *mut StreamMetadata_CueSheet_Track {
    
    todo!();
        /*
        
        */
}

/**
  | Delete a CUESHEET track object
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET track
  | object. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  |
  */
pub fn flac_metadata_object_cuesheet_track_delete(object: *mut StreamMetadata_CueSheet_Track)  {
    
    todo!();
        /*
        
        */
}

/**
  | Resize a track's index point array.
  | 
  | If the size shrinks, elements will truncated;
  | if it grows, new blank indices will be
  | added to the end.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index of the track to modify. NOTE:
  | this is not necessarily the same as the
  | track's \a number field.
  | ----------
  | @param new_num_indices
  | 
  | The desired length of the array; may
  | be \c 0. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks > track_num
  | ----------
  | @code
  | 
  | (object->data.cue_sheet.tracks[track_num].indices == NULL && object->data.cue_sheet.tracks[track_num].num_indices == 0) ||
  |  (object->data.cue_sheet.tracks[track_num].indices != NULL && object->data.cue_sheet.tracks[track_num].num_indices > 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | error, else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_track_resize_indices(
        object:          *mut StreamMetadata,
        track_num:       u32,
        new_num_indices: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert an index point in a CUESHEET track
  | at the given index.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index of the track to modify. NOTE:
  | this is not necessarily the same as the
  | track's \a number field.
  | ----------
  | @param index_num
  | 
  | The index into the track's index array
  | at which to insert the index point. NOTE:
  | this is not necessarily the same as the
  | index point's \a number field. The indices
  | at and after \a index_num move right
  | one position. To append an index point
  | to the end, set \a index_num to \c object->data.cue_sheet.tracks[track_num].num_indices
  | .
  | ----------
  | @param index
  | 
  | The index point to insert. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks > track_num
  | ----------
  | @code
  | 
  | object->data.cue_sheet.tracks[track_num].num_indices >= index_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_track_insert_index(
        object:    *mut StreamMetadata,
        track_num: u32,
        index_num: u32,
        index:     StreamMetadata_CueSheet_Index) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a blank index point in a CUESHEET
  | track at the given index.
  | 
  | A blank index point is one in which all
  | field values are zero.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index of the track to modify. NOTE:
  | this is not necessarily the same as the
  | track's \a number field.
  | ----------
  | @param index_num
  | 
  | The index into the track's index array
  | at which to insert the index point. NOTE:
  | this is not necessarily the same as the
  | index point's \a number field. The indices
  | at and after \a index_num move right
  | one position. To append an index point
  | to the end, set \a index_num to \c object->data.cue_sheet.tracks[track_num].num_indices
  | . \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks > track_num
  | ----------
  | @code
  | 
  | object->data.cue_sheet.tracks[track_num].num_indices >= index_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_track_insert_blank_index(
        object:    *mut StreamMetadata,
        track_num: u32,
        index_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Delete an index point in a CUESHEET track
  | at the given index.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index into the track array of the
  | track to modify. NOTE: this is not necessarily
  | the same as the track's \a number field.
  | ----------
  | @param index_num
  | 
  | The index into the track's index array
  | of the index to delete. NOTE: this is
  | not necessarily the same as the index's
  | \a number field. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks > track_num
  | ----------
  | @code
  | 
  | object->data.cue_sheet.tracks[track_num].num_indices > index_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_track_delete_index(
        object:    *mut StreamMetadata,
        track_num: u32,
        index_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Resize the track array.
  | 
  | If the size shrinks, elements will truncated;
  | if it grows, new blank tracks will be
  | added to the end.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param new_num_tracks
  | 
  | The desired length of the array; may
  | be \c 0. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | (object->data.cue_sheet.tracks == NULL && object->data.cue_sheet.num_tracks == 0) ||
  |  (object->data.cue_sheet.tracks != NULL && object->data.cue_sheet.num_tracks > 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if memory allocation
  | error, else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_resize_tracks(
        object:         *mut StreamMetadata,
        new_num_tracks: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets a track in a CUESHEET block.
  | 
  | If \a copy is \c true, a copy of the track
  | is stored; otherwise, the object takes
  | ownership of the \a track pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | Index into track array to set. NOTE:
  | this is not necessarily the same as the
  | track's \a number field.
  | ----------
  | @param track
  | 
  | The track to set the track to. You may
  | safely pass in a const pointer if \a copy
  | is \c true.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | track_num < object->data.cue_sheet.num_tracks
  | ----------
  | @code
  | 
  | (track->indices != NULL && track->num_indices > 0) ||
  |  (track->indices == NULL && track->num_indices == 0)
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_set_track(
        object:    *mut StreamMetadata,
        track_num: u32,
        track:     *mut StreamMetadata_CueSheet_Track,
        copy:      bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a track in a CUESHEET block at
  | the given index.
  | 
  | If \a copy is \c true, a copy of the track
  | is stored; otherwise, the object takes
  | ownership of the \a track pointer.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index at which to insert the track.
  | NOTE: this is not necessarily the same
  | as the track's \a number field. The tracks
  | at and after \a track_num move right
  | one position. To append a track to the
  | end, set \a track_num to \c object->data.cue_sheet.num_tracks
  | .
  | ----------
  | @param track
  | 
  | The track to insert. You may safely pass
  | in a const pointer if \a copy is \c true.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks >= track_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_insert_track(
        object:    *mut StreamMetadata,
        track_num: u32,
        track:     *mut StreamMetadata_CueSheet_Track,
        copy:      bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Insert a blank track in a CUESHEET block
  | at the given index.
  | 
  | A blank track is one in which all field
  | values are zero.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index at which to insert the track.
  | NOTE: this is not necessarily the same
  | as the track's \a number field. The tracks
  | at and after \a track_num move right
  | one position. To append a track to the
  | end, set \a track_num to \c object->data.cue_sheet.num_tracks
  | . \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks >= track_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_insert_blank_track(
        object:    *mut StreamMetadata,
        track_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Delete a track in a CUESHEET block at
  | the given index.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param track_num
  | 
  | The index into the track array of the
  | track to delete. NOTE: this is not necessarily
  | the same as the track's \a number field.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | ----------
  | @code
  | 
  | object->data.cue_sheet.num_tracks > track_num
  | 
  | -----------
  | @return
  | 
  | bool \c false if realloc() fails,
  | else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_delete_track(
        object:    *mut StreamMetadata,
        track_num: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a cue sheet to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the cue sheet.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | ----------
  | @param check_cd_da_subset
  | 
  | If \c true, check CUESHEET against more
  | stringent requirements for a CD-DA
  | (audio) disc.
  | ----------
  | @param violation
  | 
  | Address of a pointer to a string. If there
  | is a violation, a pointer to a string
  | explanation of the violation will be
  | returned here. \a violation may be \c
  | NULL if you don't need the returned string.
  | Do not free the returned string; it will
  | always point to static data. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | 
  | -----------
  | @return
  | 
  | bool \c false if cue sheet is illegal,
  | else \c true.
  |
  */
pub fn flac_metadata_object_cuesheet_is_legal(
        object:             *const StreamMetadata,
        check_cd_da_subset: bool,
        violation:          *const *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Calculate and return the CDDB/freedb
  | ID for a cue sheet. The function assumes
  | the cue sheet corresponds to a CD; the
  | result is undefined if the cuesheet's
  | is_cd bit is not set.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing CUESHEET object.
  | \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_CUESHEET
  | 
  | -----------
  | @return
  | 
  | u32
  | 
  | The unsigned integer representation
  | of the CDDB/freedb ID
  |
  */
pub fn flac_metadata_object_cuesheet_calculate_cddb_id(object: *const StreamMetadata) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | Sets the MIME type of a PICTURE block.
  | 
  | If \a copy is \c true, a copy of the string
  | is stored; otherwise, the object takes
  | ownership of the pointer. The existing
  | string will be freed if this function
  | is successful, otherwise the original
  | string will remain if \a copy is \c true
  | and malloc() fails.
  | 
  | -----------
  | @note
  | 
  | It is safe to pass a const pointer to \a
  | mime_type if \a copy is \c true.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing PICTURE object.
  | ----------
  | @param mime_type
  | 
  | A pointer to the MIME type string. The
  | string must be
  | 
  | ASCII characters 0x20-0x7e, NUL-terminated.
  | No validation is done.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_PICTURE
  | ----------
  | @code
  | 
  | (mime_type != NULL)
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_picture_set_mime_type(
        object:    *mut StreamMetadata,
        mime_type: *mut u8,
        copy:      bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets the description of a PICTURE block.
  | 
  | If \a copy is \c true, a copy of the string
  | is stored; otherwise, the object takes
  | ownership of the pointer. The existing
  | string will be freed if this function
  | is successful, otherwise the original
  | string will remain if \a copy is \c true
  | and malloc() fails.
  | 
  | -----------
  | @note
  | 
  | It is safe to pass a const pointer to \a
  | description if \a copy is \c true.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing PICTURE object.
  | ----------
  | @param description
  | 
  | A pointer to the description string.
  | The string must be valid UTF-8, NUL-terminated.
  | No validation is done.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_PICTURE
  | ----------
  | @code
  | 
  | (description != NULL)
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_picture_set_description(
        object:      *mut StreamMetadata,
        description: *mut u8,
        copy:        bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sets the picture data of a PICTURE block.
  | 
  | If \a copy is \c true, a copy of the data
  | is stored; otherwise, the object takes
  | ownership of the pointer. Also sets
  | the \a data_length field of the metadata
  | object to what is passed in as the \a length
  | parameter. The existing data will be
  | freed if this function is successful,
  | otherwise the original data and data_length
  | will remain if \a copy is \c true and malloc()
  | fails.
  | 
  | -----------
  | @note
  | 
  | It is safe to pass a const pointer to \a
  | data if \a copy is \c true.
  | 
  | -----------
  | @param object
  | 
  | A pointer to an existing PICTURE object.
  | ----------
  | @param data
  | 
  | A pointer to the data to set.
  | ----------
  | @param length
  | 
  | The length of \a data in bytes.
  | ----------
  | @param copy
  | 
  | See above. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_PICTURE
  | ----------
  | @code
  | 
  | (data != NULL && length > 0) ||
  |  (data == NULL && length == 0 && copy == false)
  | 
  | -----------
  | @return
  | 
  | bool \c false if \a copy is \c true
  | and malloc() fails, else \c true.
  |
  */
pub fn flac_metadata_object_picture_set_data(
        object: *mut StreamMetadata,
        data:   *mut u8,
        length: u32,
        copy:   bool) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a PICTURE block to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the
  | 
  | PICTURE block.
  | 
  | -----------
  | @param object
  | 
  | A pointer to existing PICTURE block
  | to be checked.
  | ----------
  | @param violation
  | 
  | Address of a pointer to a string. If there
  | is a violation, a pointer to a string
  | explanation of the violation will be
  | returned here. \a violation may be \c
  | NULL if you don't need the returned string.
  | Do not free the returned string; it will
  | always point to static data. \assert
  | 
  | -----------
  | @code
  | 
  | object != NULL
  | ----------
  | @code
  | 
  | object->type == METADATA_TYPE_PICTURE
  | 
  | -----------
  | @return
  | 
  | bool \c false if PICTURE block
  | is illegal, else \c true.
  |
  */
pub fn flac_metadata_object_picture_is_legal(
        object:    *const StreamMetadata,
        violation: *const *const u8) -> bool {
    
    todo!();
        /*
        
        */
}
