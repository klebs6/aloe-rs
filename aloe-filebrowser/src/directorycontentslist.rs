crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_DirectoryContentsList.h]

/**
  | A class to asynchronously scan for details
  | about the files in a directory.
  | 
  | This keeps a list of files and some information
  | about them, using a background thread
  | to scan for more files. As files are found,
  | it broadcasts change messages to tell
  | any listeners.
  | 
  | @see FileListComponent, FileBrowserComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DirectoryContentsList<'a> {
    base:             ChangeBroadcaster<'a>,
    base2:            TimeSliceClient,
    root:             File,
    file_filter:      *const FileFilter, // default = nullptr
    thread:           &'a mut TimeSliceThread,
    file_type_flags:  i32, //= File::ignoreHiddenFiles | File::findFiles;
    file_list_lock:   CriticalSection,
    files:            Vec<Box<DirectoryContentsListFileInfo>>,
    file_find_handle: Box<RangedDirectoryIterator>,
    should_stop:      AtomicBool, // default = { true  }
    was_empty:        bool,       // default = true
}

impl<'a> Drop for DirectoryContentsList<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        stopSearching();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_DirectoryContentsList.cpp]
impl<'a> DirectoryContentsList<'a> {
    
    /**
      | Returns the directory that's currently
      | being used.
      |
      */
    pub fn get_directory(&self) -> &File {
        
        todo!();
        /*
            return root;
        */
    }

    /**
      | Returns true if this list contains directories.
      | @see setDirectory
      |
      */
    pub fn is_finding_directories(&self) -> bool {
        
        todo!();
        /*
            return (fileTypeFlags & File::findDirectories) != 0;
        */
    }

    /**
      | Returns true if this list contains files.
      | @see setDirectory
      |
      */
    pub fn is_finding_files(&self) -> bool {
        
        todo!();
        /*
            return (fileTypeFlags & File::findFiles) != 0;
        */
    }

    /**
      | Returns the file filter being used.
      | 
      | The filter is specified in the constructor.
      |
      */
    pub fn get_filter(&self) -> *const FileFilter {
        
        todo!();
        /*
            return fileFilter;
        */
    }

    /**
      | @internal
      |
      */
    pub fn get_time_slice_thread(&self) -> &mut TimeSliceThread {
        
        todo!();
        /*
            return thread;
        */
    }
    
    /**
      | Creates a directory list.
      | 
      | To set the directory it should point
      | to, use setDirectory(), which will
      | also start it scanning for files on the
      | background thread.
      | 
      | When the background thread finds and
      | adds new files to this list, the
      | 
      | ChangeBroadcaster class will send
      | a change message, so you can register
      | listeners and update them when the list
      | changes.
      | 
      | -----------
      | @param fileFilter
      | 
      | an optional filter to select which files
      | are included in the list. If this is nullptr,
      | then all files and directories are included.
      | Make sure that the filter doesn't get
      | deleted during the lifetime of this
      | object
      | ----------
      | @param threadToUse
      | 
      | a thread object that this list can use
      | to scan for files as a background task.
      | Make sure that the thread you give it
      | has been started, or you won't get any
      | files!
      |
      */
    pub fn new(
        f: *const FileFilter,
        t: &mut TimeSliceThread) -> Self {
    
        todo!();
        /*
        : file_filter(f),
        : thread(t),
        */
    }
    
    /**
      | Tells the list whether or not to ignore
      | hidden files.
      | 
      | By default these are ignored.
      |
      */
    pub fn set_ignores_hidden_files(&mut self, should_ignore_hidden_files: bool)  {
        
        todo!();
        /*
            setTypeFlags (shouldIgnoreHiddenFiles ? (fileTypeFlags | File::ignoreHiddenFiles)
                                              : (fileTypeFlags & ~File::ignoreHiddenFiles));
        */
    }
    
    /**
      | Returns true if hidden files are ignored.
      | @see setIgnoresHiddenFiles
      |
      */
    pub fn ignores_hidden_files(&self) -> bool {
        
        todo!();
        /*
            return (fileTypeFlags & File::ignoreHiddenFiles) != 0;
        */
    }
    
    /**
      | Sets the directory to look in for files.
      | 
      | If the directory that's passed in is
      | different to the current one, this will
      | also start the background thread scanning
      | it for files.
      |
      */
    pub fn set_directory(&mut self, 
        directory:           &File,
        include_directories: bool,
        include_files:       bool)  {
        
        todo!();
        /*
            jassert (includeDirectories || includeFiles); // you have to specify at least one of these!

        if (directory != root)
        {
            clear();
            root = directory;
            changed();

            // (this forces a refresh when setTypeFlags() is called, rather than triggering two refreshes)
            fileTypeFlags &= ~(File::findDirectories | File::findFiles);
        }

        auto newFlags = fileTypeFlags;

        if (includeDirectories) newFlags |= File::findDirectories;
        else                    newFlags &= ~File::findDirectories;

        if (includeFiles)       newFlags |= File::findFiles;
        else                    newFlags &= ~File::findFiles;

        setTypeFlags (newFlags);
        */
    }
    
    pub fn set_type_flags(&mut self, new_flags: i32)  {
        
        todo!();
        /*
            if (fileTypeFlags != newFlags)
        {
            fileTypeFlags = newFlags;
            refresh();
        }
        */
    }
    
    pub fn stop_searching(&mut self)  {
        
        todo!();
        /*
            shouldStop = true;
        thread.removeTimeSliceClient (this);
        fileFindHandle = nullptr;
        */
    }
    
    /**
      | Clears the list, and stops the thread
      | scanning for files.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            stopSearching();

        if (! files.isEmpty())
        {
            files.clear();
            changed();
        }
        */
    }
    
    /**
      | Clears the list and restarts scanning
      | the directory for files.
      |
      */
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            stopSearching();
        wasEmpty = files.isEmpty();
        files.clear();

        if (root.isDirectory())
        {
            fileFindHandle = std::make_unique<RangedDirectoryIterator> (root, false, "*", fileTypeFlags);
            shouldStop = false;
            thread.addTimeSliceClient (this);
        }
        */
    }
    
    /**
      | Replaces the current FileFilter.
      | 
      | This can be nullptr to have no filter.
      | The DirectoryContentList does not
      | take ownership of this object - it just
      | keeps a pointer to it, so you must manage
      | its lifetime.
      | 
      | -----------
      | @note
      | 
      | this only replaces the filter, it doesn't
      | refresh the list - you'll probably want
      | to call refresh() after calling this.
      |
      */
    pub fn set_file_filter(&mut self, new_file_filter: *const FileFilter)  {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);
        fileFilter = newFileFilter;
        */
    }
    
    /**
      | Returns the number of files currently
      | available in the list.
      | 
      | The info about one of these files can
      | be retrieved with getFileInfo() or
      | getFile().
      | 
      | Obviously as the background thread
      | runs and scans the directory for files,
      | this number will change.
      | 
      | @see getFileInfo, getFile
      |
      */
    pub fn get_num_files(&self) -> i32 {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);
        return files.size();
        */
    }
    
    /**
      | Returns the cached information about
      | one of the files in the list.
      | 
      | If the index is in-range, this will return
      | true and will copy the file's details
      | to the structure that is passed-in.
      | 
      | If it returns false, then the index wasn't
      | in range, and the structure won't be
      | affected.
      | 
      | @see getNumFiles, getFile
      |
      */
    pub fn get_file_info(&self, 
        index:  i32,
        result: &mut DirectoryContentsListFileInfo) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);

        if (auto* info = files [index])
        {
            result = *info;
            return true;
        }

        return false;
        */
    }
    
    /**
      | Returns one of the files in the list.
      | 
      | -----------
      | @param index
      | 
      | should be less than getNumFiles().
      | If this is out-of-range, the return
      | value will be a default File() object
      | @see getNumFiles, getFileInfo
      |
      */
    pub fn get_file(&self, index: i32) -> File {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);

        if (auto* info = files [index])
            return root.getChildFile (info->filename);

        return {};
        */
    }
    
    /**
      | Returns true if the list contains the
      | specified file.
      |
      */
    pub fn contains(&self, target_file: &File) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);

        for (int i = files.size(); --i >= 0;)
            if (root.getChildFile (files.getUnchecked(i)->filename) == targetFile)
                return true;

        return false;
        */
    }
    
    /**
      | True if the background thread hasn't
      | yet finished scanning for files.
      |
      */
    pub fn is_still_loading(&self) -> bool {
        
        todo!();
        /*
            return fileFindHandle != nullptr;
        */
    }
    
    pub fn changed(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage();
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            auto startTime = Time::getApproximateMillisecondCounter();
        bool hasChanged = false;

        for (int i = 100; --i >= 0;)
        {
            if (! checkNextFile (hasChanged))
            {
                if (hasChanged)
                    changed();

                return 500;
            }

            if (shouldStop || (Time::getApproximateMillisecondCounter() > startTime + 150))
                break;
        }

        if (hasChanged)
            changed();

        return 0;
        */
    }
    
    pub fn check_next_file(&mut self, has_changed: &mut bool) -> bool {
        
        todo!();
        /*
            if (fileFindHandle != nullptr)
        {
            if (*fileFindHandle != RangedDirectoryIterator())
            {
                const auto entry = *(*fileFindHandle)++;

                if (addFile (entry.getFile(),
                             entry.isDirectory(),
                             entry.getFileSize(),
                             entry.getModificationTime(),
                             entry.getCreationTime(),
                             entry.isReadOnly()))
                {
                    hasChanged = true;
                }

                return true;
            }

            fileFindHandle = nullptr;

            if (! wasEmpty && files.isEmpty())
                hasChanged = true;
        }

        return false;
        */
    }
    
    pub fn add_file(
        &mut self, 
        file:          &File,
        is_dir:        bool,
        file_size:     i64,
        mod_time:      Time,
        creation_time: Time,
        is_read_only:  bool

    ) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (fileListLock);

        if (fileFilter == nullptr
             || ((! isDir) && fileFilter->isFileSuitable (file))
             || (isDir && fileFilter->isDirectorySuitable (file)))
        {
            auto info = std::make_unique<DirectoryContentsListFileInfo>();

            info->filename         = file.getFileName();
            info->fileSize         = fileSize;
            info->modificationTime = modTime;
            info->creationTime     = creationTime;
            info->isDirectory      = isDir;
            info->isReadOnly       = isReadOnly;

            for (int i = files.size(); --i >= 0;)
                if (files.getUnchecked(i)->filename == info->filename)
                    return false;

            files.add (std::move (info));

            std::sort (files.begin(), files.end(), [] (const DirectoryContentsListFileInfo* a, const DirectoryContentsListFileInfo* b)
            {
               #if ALOE_WINDOWS
                if (a->isDirectory != b->isDirectory)
                    return a->isDirectory;
               #endif

                return a->filename.compareNatural (b->filename) < 0;
            });

            return true;
        }

        return false;
        */
    }
}
