#[cfg(feature = "aloe_posix")]
#[cfg(any(any(target_os="macos",target_os="linux"),target_os="bsd"))]
lazy_static!{
    /*
    static MaxNumFileHandlesInitialiser maxNumFileHandlesInitialiser;
    */
}

impl File {
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_separator_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            return '/';
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_separator_string(&mut self) -> &str {
        
        todo!();
        /*
            return "/";
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_current_working_directory(&mut self) -> File {
        
        todo!();
        /*
            HeapBlock<char> heapBuffer;

        char localBuffer[1024];
        auto cwd = getcwd (localBuffer, sizeof (localBuffer) - 1);
        size_t bufferSize = 4096;

        while (cwd == nullptr && errno == ERANGE)
        {
            heapBuffer.malloc (bufferSize);
            cwd = getcwd (heapBuffer, bufferSize - 1);
            bufferSize += 1024;
        }

        return File (CharPointer_UTF8 (cwd));
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_as_current_working_directory(&self) -> bool {
        
        todo!();
        /*
            return chdir (getFullPathName().toUTF8()) == 0;
        */
    }
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
pub fn update_stat_info_for_file(
        path:          &String,
        is_dir:        *mut bool,
        file_size:     *mut i64,
        mod_time:      *mut Time,
        creation_time: *mut Time,
        is_read_only:  *mut bool)  {
    
    todo!();
    /*
        if (isDir != nullptr || fileSize != nullptr || modTime != nullptr || creationTime != nullptr)
        {
            aloe_statStruct info;
            const bool statOk = aloe_stat (path, info);

            if (isDir != nullptr)         *isDir        = statOk && ((info.st_mode & S_IFDIR) != 0);
            if (fileSize != nullptr)      *fileSize     = statOk ? (int64) info.st_size : 0;
            if (modTime != nullptr)       *modTime      = Time (statOk ? (int64) info.st_mtime  * 1000 : 0);
            if (creationTime != nullptr)  *creationTime = Time (statOk ? getCreationTime (info) * 1000 : 0);
        }

        if (isReadOnly != nullptr)
            *isReadOnly = access (path.toUTF8(), W_OK) != 0;
    */
}

#[cfg(feature = "aloe_posix")]
pub fn getfd(handle: *mut c_void) -> i32 {
    
    todo!();
    /*
        return (int) (pointer_sized_int) handle;
    */
}


#[cfg(feature = "aloe_posix")]
pub fn fd_to_void_pointer(fd: i32)  {
    
    todo!();
    /*
        return (void*) (pointer_sized_int) fd;
    */
}

#[cfg(feature = "aloe_posix")]
pub fn has_effective_root_file_permissions() -> bool {
    
    todo!();
    /*
        #if ALOE_LINUX || ALOE_BSD
        return geteuid() == 0;
       #else
        return false;
       #endif
    */
}

#[cfg(feature = "aloe_posix")]
pub fn set_file_mode_flags(
        full_path:  &String,
        flags:      Mode,
        should_set: bool) -> bool {
    
    todo!();
    /*
        aloe_statStruct info;

        if (! aloe_stat (fullPath, info))
            return false;

        info.st_mode &= 0777;

        if (shouldSet)
            info.st_mode |= flags;
        else
            info.st_mode &= ~flags;

        return chmod (fullPath.toUTF8(), (mode_t) info.st_mode) == 0;
    */
}

impl File {
    
    #[cfg(feature = "aloe_posix")]
    pub fn is_directory(&self) -> bool {
        
        todo!();
        /*
            aloe_statStruct info;

        return fullPath.isNotEmpty()
                 && (aloe_stat (fullPath, info) && ((info.st_mode & S_IFDIR) != 0));
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn exists(&self) -> bool {
        
        todo!();
        /*
            return fullPath.isNotEmpty()
                 && access (fullPath.toUTF8(), F_OK) == 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn exists_as_file(&self) -> bool {
        
        todo!();
        /*
            return exists() && ! isDirectory();
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_size(&self) -> i64 {
        
        todo!();
        /*
            aloe_statStruct info;
        return aloe_stat (fullPath, info) ? info.st_size : 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_file_identifier(&self) -> u64 {
        
        todo!();
        /*
            aloe_statStruct info;
        return aloe_stat (fullPath, info) ? (uint64) info.st_ino : 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn has_write_access(&self) -> bool {
        
        todo!();
        /*
            if (exists())
            return (hasEffectiveRootFilePermissions()
                 || access (fullPath.toUTF8(), W_OK) == 0);

        if ((! isDirectory()) && fullPath.containsChar (getSeparatorChar()))
            return getParentDirectory().hasWriteAccess();

        return false;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_file_read_only_internal(&self, should_be_read_only: bool) -> bool {
        
        todo!();
        /*
            // Hmm.. should we give global write permission or just the current user?
        return setFileModeFlags (fullPath, S_IWUSR | S_IWGRP | S_IWOTH, ! shouldBeReadOnly);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_file_executable_internal(&self, should_be_executable: bool) -> bool {
        
        todo!();
        /*
            return setFileModeFlags (fullPath, S_IXUSR | S_IXGRP | S_IXOTH, shouldBeExecutable);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_file_times_internal(&self, 
        modification_time: &mut i64,
        access_time:       &mut i64,
        creation_time:     &mut i64)  {
        
        todo!();
        /*
            modificationTime = 0;
        accessTime = 0;
        creationTime = 0;

        aloe_statStruct info;

        if (aloe_stat (fullPath, info))
        {
          #if ALOE_MAC || (ALOE_IOS && __DARWIN_ONLY_64_BIT_INO_T)
            modificationTime  = (int64) info.st_mtimespec.tv_sec * 1000 + info.st_mtimespec.tv_nsec / 1000000;
            accessTime        = (int64) info.st_atimespec.tv_sec * 1000 + info.st_atimespec.tv_nsec / 1000000;
            creationTime      = (int64) info.st_birthtimespec.tv_sec * 1000 + info.st_birthtimespec.tv_nsec / 1000000;
          #else
            modificationTime  = (int64) info.st_mtime * 1000;
            accessTime        = (int64) info.st_atime * 1000;
           #if ALOE_IOS
            creationTime      = (int64) info.st_birthtime * 1000;
           #else
            creationTime      = (int64) info.st_ctime * 1000;
           #endif
          #endif
        }
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_file_times_internal(&self, 
        modification_time: i64,
        access_time:       i64,
        creation_time:     i64) -> bool {
        
        todo!();
        /*
            #if ! ALOE_WASM
        aloe_statStruct info;

        if ((modificationTime != 0 || accessTime != 0) && aloe_stat (fullPath, info))
        {
           #if ALOE_MAC || (ALOE_IOS && __DARWIN_ONLY_64_BIT_INO_T)
            struct timeval times[2];

            bool setModificationTime = (modificationTime != 0);
            bool setAccessTime       = (accessTime != 0);

            times[0].tv_sec  = setAccessTime ? static_cast<__darwin_time_t> (accessTime / 1000)
                                             : info.st_atimespec.tv_sec;

            times[0].tv_usec = setAccessTime ? static_cast<__darwin_suseconds_t> ((accessTime % 1000) * 1000)
                                             : static_cast<__darwin_suseconds_t> (info.st_atimespec.tv_nsec / 1000);

            times[1].tv_sec  = setModificationTime ? static_cast<__darwin_time_t> (modificationTime / 1000)
                                                   : info.st_mtimespec.tv_sec;

            times[1].tv_usec = setModificationTime ? static_cast<__darwin_suseconds_t> ((modificationTime % 1000) * 1000)
                                                   : static_cast<__darwin_suseconds_t> (info.st_mtimespec.tv_nsec / 1000);

            return utimes (fullPath.toUTF8(), times) == 0;
           #else
            struct utimbuf times;
            times.actime  = accessTime != 0       ? static_cast<time_t> (accessTime / 1000)       : static_cast<time_t> (info.st_atime);
            times.modtime = modificationTime != 0 ? static_cast<time_t> (modificationTime / 1000) : static_cast<time_t> (info.st_mtime);

            return utime (fullPath.toUTF8(), &times) == 0;
           #endif
        }
       #endif

        return false;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn delete_file(&self) -> bool {
        
        todo!();
        /*
            if (! isSymbolicLink())
        {
            if (! exists())
                return true;

            if (isDirectory())
                return rmdir (fullPath.toUTF8()) == 0;
        }

        return remove (fullPath.toUTF8()) == 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn move_internal(&self, dest: &File) -> bool {
        
        todo!();
        /*
            if (rename (fullPath.toUTF8(), dest.getFullPathName().toUTF8()) == 0)
            return true;

        if (hasWriteAccess() && copyInternal (dest))
        {
            if (deleteFile())
                return true;

            dest.deleteFile();
        }

        return false;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn replace_internal(&self, dest: &File) -> bool {
        
        todo!();
        /*
            return moveInternal (dest);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn create_directory_internal(&self, file_name: &String) -> Result {
        
        todo!();
        /*
            return getResultForReturnValue (mkdir (fileName.toUTF8(), 0777));
        */
    }
}

#[cfg(feature = "aloe_posix")]
impl Drop for FileInputStream {
    fn drop(&mut self) {
        todo!();
        /* 
        if (fileHandle != nullptr)
            close (getFD (fileHandle));
 */
    }
}

impl FileInputStream {
    
    #[cfg(feature = "aloe_posix")]
    pub fn open_handle(&mut self)  {
        
        todo!();
        /*
            auto f = open (file.getFullPathName().toUTF8(), O_RDONLY);

        if (f != -1)
            fileHandle = fdToVoidPointer (f);
        else
            status = getResultForErrno();
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn read_internal(&mut self, 
        buffer:    *mut c_void,
        num_bytes: usize) -> usize {
        
        todo!();
        /*
            ssize_t result = 0;

        if (fileHandle != nullptr)
        {
            result = ::read (getFD (fileHandle), buffer, numBytes);

            if (result < 0)
            {
                status = getResultForErrno();
                result = 0;
            }
        }

        return (size_t) result;
        */
    }
}

impl FileOutputStream {
    
    #[cfg(feature = "aloe_posix")]
    pub fn open_handle(&mut self)  {
        
        todo!();
        /*
            if (file.exists())
        {
            auto f = open (file.getFullPathName().toUTF8(), O_RDWR);

            if (f != -1)
            {
                currentPosition = lseek (f, 0, SEEK_END);

                if (currentPosition >= 0)
                {
                    fileHandle = fdToVoidPointer (f);
                }
                else
                {
                    status = getResultForErrno();
                    close (f);
                }
            }
            else
            {
                status = getResultForErrno();
            }
        }
        else
        {
            auto f = open (file.getFullPathName().toUTF8(), O_RDWR | O_CREAT, 00644);

            if (f != -1)
                fileHandle = fdToVoidPointer (f);
            else
                status = getResultForErrno();
        }
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn close_handle(&mut self)  {
        
        todo!();
        /*
            if (fileHandle != nullptr)
        {
            close (getFD (fileHandle));
            fileHandle = nullptr;
        }
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn write_internal(&mut self, 
        data:      *const c_void,
        num_bytes: usize) -> isize {
        
        todo!();
        /*
            if (fileHandle == nullptr)
            return 0;

        auto result = ::write (getFD (fileHandle), data, numBytes);

        if (result == -1)
            status = getResultForErrno();

        return (ssize_t) result;
        */
    }

    #[cfg(feature = "aloe_posix")]
    #[cfg(not(target_os="android"))]
    pub fn flush_internal(&mut self)  {
        
        todo!();
        /*
            if (fileHandle != nullptr && fsync (getFD (fileHandle)) == -1)
            status = getResultForErrno();
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn truncate(&mut self) -> Result {
        
        todo!();
        /*
            if (fileHandle == nullptr)
            return status;

        flush();
        return getResultForReturnValue (ftruncate (getFD (fileHandle), (off_t) currentPosition));
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(ALOE_WASM))]
    pub fn get_bytes_free_on_volume(&self) -> i64 {
        
        todo!();
        /*
            struct statfs buf;

        if (aloe_doStatFS (*this, buf))
            return (int64) buf.f_bsize * (int64) buf.f_bavail; // Note: this returns space available to non-super user

        return 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(ALOE_WASM))]
    pub fn get_volume_total_size(&self) -> i64 {
        
        todo!();
        /*
            struct statfs buf;

        if (aloe_doStatFS (*this, buf))
            return (int64) buf.f_bsize * (int64) buf.f_blocks;

        return 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(ALOE_WASM))]
    pub fn get_volume_label(&self) -> String {
        
        todo!();
        /*
            #if ALOE_MAC
        struct VolAttrBuf
        {
            u_int32_t       length;
            attrreference_t mountPointRef;
            char            mountPointSpace[MAXPATHLEN];
        } attrBuf;

        struct attrlist attrList;
        zerostruct (attrList); // (can't use "= {}" on this object because it's a C struct)
        attrList.bitmapcount = ATTR_BIT_MAP_COUNT;
        attrList.volattr = ATTR_VOL_INFO | ATTR_VOL_NAME;

        File f (*this);

        for (;;)
        {
            if (getattrlist (f.getFullPathName().toUTF8(), &attrList, &attrBuf, sizeof (attrBuf), 0) == 0)
                return String::fromUTF8 (((const char*) &attrBuf.mountPointRef) + attrBuf.mountPointRef.attr_dataoffset,
                                         (int) attrBuf.mountPointRef.attr_length);

            auto parent = f.getParentDirectory();

            if (f == parent)
                break;

            f = parent;
        }
       #endif

        return {};
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(ALOE_WASM))]
    pub fn get_volume_serial_number(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
}
