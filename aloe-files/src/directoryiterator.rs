crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_DirectoryIterator.h]

/**
  | This class is now deprecated in favour
  | of RangedDirectoryIterator.
  | 
  | Searches through the files in a directory,
  | returning each file that is found.
  | 
  | A DirectoryIterator will search through
  | a directory and its subdirectories
  | using a wildcard filepattern match.
  | 
  | If you may be scanning a large number
  | of files, it's usually smarter to use
  | this class than File::findChildFiles()
  | because it allows you to stop at any time,
  | rather than having to wait for the entire
  | scan to finish before getting the results.
  | 
  | Please note that the order in which files
  | are returned is completely undefined!
  | They'll arrive in whatever order the
  | underlying OS calls provide them, which
  | will depend on the filesystem and other
  | factors. If you need a sorted list, you'll
  | need to manually sort them using your
  | preferred comparator after collecting
  | the list.
  | 
  | It also provides an estimate of its progress,
  | using a (highly inaccurate!) algorithm.
  | 
  | @tags{Core} @see RangedDirectoryIterator
  |
  */
#[no_copy]
#[leak_detector]
pub struct DirectoryIterator {
    wild_cards:        StringArray,
    file_finder:       directory_iterator::NativeIterator,
    wild_card:         String,
    path:              String,
    index:             i32, // default = -1
    total_num_files:   std::cell::RefCell<i32>, // default = -1
    what_to_look_for:  i32,
    is_recursive:      bool,
    has_been_advanced: bool, // default = false
    sub_iterator:      Box<DirectoryIterator>,
    current_file:      File,
}

pub mod directory_iterator {
    use super::*;

    #[no_copy]
    #[leak_detector]
    pub struct NativeIterator {
        impl_: Box<Impl>,
    }

    struct Impl;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_DirectoryIterator.cpp]
impl DirectoryIterator {

    /** 
      | This class is now deprecated in favour of
      | RangedDirectoryIterator.
      |
      | Creates a DirectoryIterator for a given
      | directory.
      |
      | After creating one of these, call its
      | next() method to get the first file
      | - e.g. @code
      |
      | DirectoryIterator iter (File ("/animals/mooses"), true, "*.moose");
      |
      | while (iter.next())
      | {
      |     File theFileItFound (iter.getFile());
      |
      |     ... etc
      | }
      | @endcode
      |
      | @see RangedDirectoryIterator
      */
    #[deprecated]
    pub fn new(
        directory:        &File,
        recursive:        bool,
        wild_card:        Option<&str>,
        what_to_look_for: Option<i32>) -> Self 
    {
        let wild_card = wild_card.unwrap_or("*");

        todo!();
        /*
        let what_to_look_for: i32 =
            what_to_look_for.unwrap_or(File::findFiles);


            : wildCards (parseWildcards (wild_card)),
        fileFinder (directory, (recursive || wildCards.size() > 1) ? "*" : wild_card),
        wildCard (wild_card),
        path (File::addTrailingSeparator (directory.getFullPathName())),
        whatToLookFor (what_to_look_for),
        isRecursive (recursive)

        // you have to specify the type of files you're looking for!
        jassert ((what_to_look_for & (File::findFiles | File::findDirectories)) != 0);
        jassert (what_to_look_for > 0 && what_to_look_for <= 7);
        */
    }
    
    pub fn parse_wildcards(&mut self, pattern: &String) -> StringArray {
        
        todo!();
        /*
            StringArray s;
        s.addTokens (pattern, ";,", "\"'");
        s.trim();
        s.removeEmptyStrings();
        return s;
        */
    }
    
    pub fn file_matches(&mut self, 
        wildcards: &StringArray,
        filename:  &String) -> bool {
        
        todo!();
        /*
            for (auto& w : wildcards)
            if (filename.matchesWildcard (w, ! File::areFileNamesCaseSensitive()))
                return true;

        return false;
        */
    }
    
    /**
      | Moves the iterator along to the next
      | file.
      | 
      | -----------
      | @return
      | 
      | true if a file was found (you can then
      | use getFile() to see what it was) - or
      | false if there are no more matching files.
      |
      */
    pub fn next_default(&mut self) -> bool {
        
        todo!();
        /*
            return next (nullptr, nullptr, nullptr, nullptr, nullptr, nullptr);
        */
    }
    
    /**
      | Moves the iterator along to the next
      | file, and returns various properties
      | of that file.
      | 
      | If you need to find out details about
      | the file, it's more efficient to call
      | this method than to call the normal next()
      | method and then find out the details
      | afterwards.
      | 
      | All the parameters are optional, so
      | pass null pointers for any items that
      | you're not interested in.
      | 
      | -----------
      | @return
      | 
      | true if a file was found (you can then
      | use getFile() to see what it was) - or
      | false if there are no more matching files.
      | If it returns false, then none of the
      | parameters will be filled-in.
      |
      */
    pub fn next_with_properties(&mut self, 
        is_dir_result:    *mut bool,
        is_hidden_result: *mut bool,
        file_size:        *mut i64,
        mod_time:         *mut Time,
        creation_time:    *mut Time,
        is_read_only:     *mut bool) -> bool {
        
        todo!();
        /*
            for (;;)
        {
            hasBeenAdvanced = true;

            if (subIterator != nullptr)
            {
                if (subIterator->next (isDirResult, isHiddenResult, fileSize, modTime, creationTime, isReadOnly))
                    return true;

                subIterator.reset();
            }

            String filename;
            bool isDirectory, isHidden = false, shouldContinue = false;

            while (fileFinder.next (filename, &isDirectory,
                                    (isHiddenResult != nullptr || (whatToLookFor & File::ignoreHiddenFiles) != 0) ? &isHidden : nullptr,
                                    fileSize, modTime, creationTime, isReadOnly))
            {
                ++index;

                if (! filename.containsOnly ("."))
                {
                    bool matches = false;

                    if (isDirectory)
                    {
                        if (isRecursive && ((whatToLookFor & File::ignoreHiddenFiles) == 0 || ! isHidden))
                            subIterator.reset (new DirectoryIterator (File::createFileWithoutCheckingPath (path + filename),
                                                                      true, wildCard, whatToLookFor));

                        matches = (whatToLookFor & File::findDirectories) != 0;
                    }
                    else
                    {
                        matches = (whatToLookFor & File::findFiles) != 0;
                    }

                    // if we're not relying on the OS iterator to do the wildcard match, do it now..
                    if (matches && (isRecursive || wildCards.size() > 1))
                        matches = fileMatches (wildCards, filename);

                    if (matches && (whatToLookFor & File::ignoreHiddenFiles) != 0)
                        matches = ! isHidden;

                    if (matches)
                    {
                        currentFile = File::createFileWithoutCheckingPath (path + filename);
                        if (isHiddenResult != nullptr)     *isHiddenResult = isHidden;
                        if (isDirResult != nullptr)        *isDirResult = isDirectory;

                        return true;
                    }

                    if (subIterator != nullptr)
                    {
                        shouldContinue = true;
                        break;
                    }
                }
            }

            if (! shouldContinue)
                return false;
        }
        */
    }
    
    /**
      | Returns the file that the iterator is
      | currently pointing at.
      | 
      | The result of this call is only valid
      | after a call to next() has returned true.
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            if (subIterator != nullptr && subIterator->hasBeenAdvanced)
            return subIterator->getFile();

        // You need to call DirectoryIterator::next() before asking it for the file that it found!
        jassert (hasBeenAdvanced);

        return currentFile;
        */
    }
    
    /**
      | Returns a guess of how far through the
      | search the iterator has got.
      | 
      | -----------
      | @return
      | 
      | a value 0.0 to 1.0 to show the progress,
      | although this won't be very accurate.
      |
      */
    pub fn get_estimated_progress(&self) -> f32 {
        
        todo!();
        /*
            if (totalNumFiles < 0)
            totalNumFiles = File (path).getNumberOfChildFiles (File::findFilesAndDirectories);

        if (totalNumFiles <= 0)
            return 0.0f;

        auto detailedIndex = (subIterator != nullptr) ? (float) index + subIterator->getEstimatedProgress()
                                                      : (float) index;

        return jlimit (0.0f, 1.0f, detailedIndex / (float) totalNumFiles);
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn new(
        directory:     &File,
        wild_card_str: &String) -> Self {
    
        todo!();
        /*
            : impl (new DirectoryIterator::NativeIterator::Impl (directory, wildCardStr))
        */
    }
    
    fn next(
        &mut self, 
        filename_found: &mut String,
        is_dir:         *mut bool,
        is_hidden:      *mut bool,
        file_size:      *mut i64,
        mod_time:       *mut Time,
        creation_time:  *mut Time,
        is_read_only:   *mut bool) -> bool {
        
        todo!();
        /*
            return impl->next (filenameFound, isDir, isHidden, fileSize, modTime, creationTime, isReadOnly);
        */
    }
}
