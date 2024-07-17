crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_RangedDirectoryIterator.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_RangedDirectoryIterator.cpp]

/**
  | Describes the attributes of a file or
  | folder.
  | 
  | @tags{Core}
  |
  */
pub struct DirectoryEntry {
    iterator:      std::sync::Weak<DirectoryIterator>,
    file:          File,
    mod_time:      Time,
    creation_time: Time,
    file_size:     i64,  // default = 0
    directory:     bool, // default = false
    hidden:        bool, // default = false
    read_only:     bool, // default = false
}

impl<'a> Deref for DirectoryEntry {

    type Target = DirectoryEntry;
    
    /**
      | A convenience operator so that the expression
      | `*it++` works correctly when `it` is
      | an instance of RangedDirectoryIterator.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return e;
        */
    }
}

impl DirectoryEntry {

    /**
      | The path to a file or folder.
      |
      */
    pub fn get_file(&self) -> File {
        
        todo!();
        /*
            return file;
        */
    }

    /**
      | The time at which the item was last modified.
      |
      */
    pub fn get_modification_time(&self) -> Time {
        
        todo!();
        /*
            return modTime;
        */
    }

    /**
      | The time at which the item was created.
      |
      */
    pub fn get_creation_time(&self) -> Time {
        
        todo!();
        /*
            return creationTime;
        */
    }

    /**
      | The size of the item.
      |
      */
    pub fn get_file_size(&self) -> i64 {
        
        todo!();
        /*
            return fileSize;
        */
    }

    /**
      | True if the item is a directory, false
      | otherwise.
      |
      */
    pub fn is_directory(&self) -> bool {
        
        todo!();
        /*
            return directory;
        */
    }

    /**
      | True if the item is hidden, false otherwise.
      |
      */
    pub fn is_hidden(&self) -> bool {
        
        todo!();
        /*
            return hidden;
        */
    }

    /**
      | True if the item is read-only, false
      | otherwise.
      |
      */
    pub fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return readOnly;
        */
    }

    /**
      | The estimated proportion of the range
      | that has been visited by the iterator,
      | from 0.0 to 1.0.
      |
      */
    pub fn get_estimated_progress(&self) -> f32 {
        
        todo!();
        /*
            if (auto it = iterator.lock())
            return it->getEstimatedProgress();

        return 0.0f;
        */
    }
}

/**
  | Allows iterating over files and folders
  | using C++11 range-for syntax.
  | 
  | In the following example, we recursively
  | find all hidden files in a specific directory.
  | 
  | -----------
  | @code
  | 
  | std::vector<File> hiddenFiles;
  | 
  | for (DirectoryEntry entry : RangedDirectoryIterator (File ("/path/to/folder"), isRecursive))
  |     if (entry.isHidden())
  |         hiddenFiles.push_back (entry.getFile());
  | 
  | @tags{Core}
  |
  */
pub struct RangedDirectoryIterator {
    iterator: Arc<DirectoryIterator>,
    entry:    DirectoryEntry,
}

impl InputIteratorTag for RangedDirectoryIterator {}

impl HasValueType for RangedDirectoryIterator {
    type ValueType = DirectoryEntry;
}

impl HasReference for RangedDirectoryIterator {
    type Reference = DirectoryEntry;
}

impl HasDifferenceType for RangedDirectoryIterator {
    type DifferenceType = libc::ptrdiff_t;
}

impl HasPointer for RangedDirectoryIterator {
    type Pointer = c_void;
}

impl Default for RangedDirectoryIterator {
    
    /**
      | The default-constructed iterator
      | acts as the 'end' sentinel.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> Deref for RangedDirectoryIterator {

    type Target = DirectoryEntry;
    
    /**
      | Return an object containing metadata
      | about the file or folder to which the
      | iterator is currently pointing.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return &entry;
        */
    }
}

impl PartialEq<RangedDirectoryIterator> for RangedDirectoryIterator {
    
    /**
      | Returns true if both iterators are in
      | their end/sentinel state, otherwise
      | returns false.
      |
      */
    #[inline] fn eq(&self, other: &RangedDirectoryIterator) -> bool {
        todo!();
        /*
            return iterator == nullptr && other.iterator == nullptr;
        */
    }
}

impl Eq for RangedDirectoryIterator {}

impl RangedDirectoryIterator {

    /**
      | Moves the iterator along to the next
      | file.
      |
      */
    pub fn prefix_increment(&mut self) -> &mut RangedDirectoryIterator {
        
        todo!();
        /*
            increment();
            return *this;
        */
    }

    /**
      | Moves the iterator along to the next
      | file.
      | 
      | -----------
      | @return
      | 
      | an object containing metadata about
      | the file or folder to to which the iterator
      | was previously pointing.
      |
      */
    pub fn postfix_increment(&mut self) -> DirectoryEntry {
        
        todo!();
        /*
            auto result = *(*this);
            ++(*this);
            return result;
        */
    }
    
    /**
      | Creates a RangedDirectoryIterator
      | for a given directory.
      | 
      | The resulting iterator can be used directly
      | in a 'range-for' expression.
      | 
      | -----------
      | @param directory
      | 
      | the directory to search in
      | ----------
      | @param isRecursive
      | 
      | whether all the subdirectories should
      | also be searched
      | ----------
      | @param wildCard
      | 
      | the file pattern to match. This may contain
      | multiple patterns separated by a semi-colon
      | or comma, e.g. "*.jpg;*.png"
      | ----------
      | @param whatToLookFor
      | 
      | a value from the File::TypesOfFileToFind
      | enum, specifying whether to look for
      | files, directories, or both.
      |
      | We implement this in terms of the deprecated
      | DirectoryIterator, but the old DirectoryIterator
      | might go away in the future!
      |
      */
    pub fn new(
        directory:        &File,
        is_recursive:     bool,
        wild_card:        Option<&str>,
        what_to_look_for: Option<i32>) -> Self {

        let wild_card = wild_card.unwrap_or("*");

        todo!();
        /*

        let what_to_look_for: i32 =
            what_to_look_for.unwrap_or(File::findFiles);


            : iterator (new DirectoryIterator (directory,
                                           isRecursive,
                                           wildCard,
                                           whatToLookFor))

        entry.iterator = iterator;
        increment();
        */
    }
    
    pub fn next(&mut self) -> bool {
        
        todo!();
        /*
            const auto result = iterator->next (&entry.directory,
                                            &entry.hidden,
                                            &entry.fileSize,
                                            &entry.modTime,
                                            &entry.creationTime,
                                            &entry.readOnly);
        if (result)
            entry.file = iterator->getFile();
        else
            entry = {};

        return result;
        */
    }
    
    pub fn increment(&mut self)  {
        
        todo!();
        /*
            if (iterator != nullptr && ! next())
            iterator = nullptr;
        */
    }
}

/**
  | Returns the iterator that was passed
  | in.
  | 
  | Provided for range-for compatibility.
  |
  */
#[inline] pub fn begin(it: &RangedDirectoryIterator) -> RangedDirectoryIterator {
    
    todo!();
    /*
        return it;
    */
}

/**
  | Returns a default-constructed sentinel
  | value.
  | 
  | Provided for range-for compatibility.
  |
  */
#[inline] pub fn end(_0: &RangedDirectoryIterator) -> RangedDirectoryIterator {
    
    todo!();
    /*
        return {};
    */
}

