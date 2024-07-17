crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_MemoryMappedFile.h]

/**
  | Maps a file into virtual memory for easy
  | reading and/or writing.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MemoryMappedFile {

    address:     *mut c_void, // default = nullptr
    range:       Range<i64>,

    #[cfg(target_os="windows")]
    file_handle: *mut c_void, // default = nullptr

    #[cfg(not(target_os="windows"))]
    file_handle: i32, // default = 0
}

pub mod memory_mapped_file {
    use super::*;

    /**
      | The read/write flags used when opening
      | a memory mapped file.
      |
      */
    pub enum AccessMode
    {
        /**
          | Indicates that the memory can only be
          | read.
          |
          */
        readOnly,   
        /**
          | Indicates that the memory can be read
          | and written to - changes that are made
          | will be flushed back to disk at the whim
          | of the OS.
          |
          */
        readWrite   
    }
}

impl MemoryMappedFile {

    /**
      | Opens a file and maps it to an area of virtual
      | memory.
      | 
      | The file should already exist, and should
      | already be the size that you want to work
      | with when you call this. If the file is
      | resized after being opened, the behaviour
      | is undefined.
      | 
      | If the file exists and the operation
      | succeeds, the getData() and getSize()
      | methods will return the location and
      | size of the data that can be read or written.
      | Note that the entire file is not read
      | into memory immediately - the OS simply
      | creates a virtual mapping, which will
      | lazily pull the data into memory when
      | blocks are accessed.
      | 
      | If the file can't be opened for some reason,
      | the getData() method will return a null
      | pointer.
      | 
      | If exclusive is false then other apps
      | can also open the same memory mapped
      | file and use this mapping as an effective
      | way of communicating. If exclusive
      | is true then the mapped file will be opened
      | exclusively - preventing other apps
      | to access the file which may improve
      | the performance of accessing the file.
      |
      */
    pub fn new(
        file:      &File,
        mode:      memory_mapped_file::AccessMode,
        exclusive: Option<bool>) -> Self {

        let exclusive: bool = exclusive.unwrap_or(false);
    
        todo!();
        /*
        : range(0, file.getSize()),

            openInternal (file, mode, exclusive);
        */
    }

    /**
      | Opens a section of a file and maps it to
      | an area of virtual memory.
      | 
      | The file should already exist, and should
      | already be the size that you want to work
      | with when you call this. If the file is
      | resized after being opened, the behaviour
      | is undefined.
      | 
      | If the file exists and the operation
      | succeeds, the getData() and getSize()
      | methods will return the location and
      | size of the data that can be read or written.
      | Note that the entire file is not read
      | into memory immediately - the OS simply
      | creates a virtual mapping, which will
      | lazily pull the data into memory when
      | blocks are accessed.
      | 
      | If the file can't be opened for some reason,
      | the getData() method will return a null
      | pointer.
      | 
      | -----------
      | @note
      | 
      | The start of the actual range used may
      | be rounded-down to a multiple of the
      | OS's page-size, so do not assume that
      | the mapped memory will begin at exactly
      | the position you requested - always
      | use getRange() to check the actual range
      | that is being used.
      |
      */
    pub fn new_with_range(
        file:       &File,
        file_range: &Range<i64>,
        mode:       memory_mapped_file::AccessMode,
        exclusive:  Option<bool>) -> Self {

        let exclusive: bool = exclusive.unwrap_or(false);
    
        todo!();
        /*
        : range (fileRange.getIntersectionWith (Range<int64> (0, file.getSize())))

        openInternal (file, mode, exclusive);
        */
    }

    /**
      | Returns the address at which this file
      | has been mapped, or a null pointer if
      | the file couldn't be successfully mapped.
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            return address;
        */
    }

    /**
      | Returns the number of bytes of data that
      | are available for reading or writing.
      | 
      | This will normally be the size of the
      | file.
      |
      */
    pub fn get_size(&self) -> usize {
        
        todo!();
        /*
            return (size_t) range.getLength();
        */
    }

    /**
      | Returns the section of the file at which
      | the mapped memory represents.
      |
      */
    pub fn get_range(&self) -> Range<i64> {
        
        todo!();
        /*
            return range;
        */
    }
    
    pub fn open_internal(&mut self, 
        _0: &File,
        _1: memory_mapped_file::AccessMode,
        _2: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(ALOE_WASM))]
    pub fn open_internal(&mut self, 
        file:      &File,
        mode:      AccessMode,
        exclusive: bool)  {
        
        todo!();
        /*
            jassert (mode == readOnly || mode == readWrite);

        if (range.getStart() > 0)
        {
            auto pageSize = sysconf (_SC_PAGE_SIZE);
            range.setStart (range.getStart() - (range.getStart() % pageSize));
        }

        auto filename = file.getFullPathName().toUTF8();

        if (mode == readWrite)
            fileHandle = open (filename, O_CREAT | O_RDWR, 00644);
        else
            fileHandle = open (filename, O_RDONLY);

        if (fileHandle != -1)
        {
            auto m = mmap (nullptr, (size_t) range.getLength(),
                           mode == readWrite ? (PROT_READ | PROT_WRITE) : PROT_READ,
                           exclusive ? MAP_PRIVATE : MAP_SHARED, fileHandle,
                           (off_t) range.getStart());

            if (m != MAP_FAILED)
            {
                address = m;
                madvise (m, (size_t) range.getLength(), MADV_SEQUENTIAL);
            }
            else
            {
                range = Range<int64>();
            }

            close (fileHandle);
            fileHandle = 0;
        }
        */
    }
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
impl Drop for MemoryMappedFile {

    fn drop(&mut self) {
        todo!();
        /* 
        if (address != nullptr)
            munmap (address, (size_t) range.getLength());

        if (fileHandle != 0)
            close (fileHandle);
 */
    }
}
