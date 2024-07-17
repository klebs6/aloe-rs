crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileSearchPath.h]

/**
  | Represents a set of folders that make
  | up a search path.
  | 
  | @see File
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct FileSearchPath {

    directories: StringArray,
}

impl Default for FileSearchPath {
    
    /**
      | Creates an empty search path.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Index<i32> for FileSearchPath {

    type Output = File;
    
    /**
      | Returns one of the folders in this search
      | path.
      | 
      | The file returned isn't guaranteed
      | to actually be a valid directory. @see
      | getNumPaths
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            return File (directories[index]);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileSearchPath.cpp]
impl FileSearchPath {

    /**
      | Creates a search path from a string of
      | pathnames.
      | 
      | The path can be semicolon- or comma-separated,
      | e.g. "/foo/bar;/foo/moose;/fish/moose"
      | 
      | The separate folders are tokenised
      | and added to the search path.
      |
      */
    pub fn new_from_path(path: &String) -> Self {
    
        todo!();
        /*
            init (path);
        */
    }
    
    /**
      | Creates a copy of another search path.
      |
      */
    pub fn new_from_file_search_path(other: &FileSearchPath) -> Self {
    
        todo!();
        /*
        : directories(other.directories),
        */
    }
    
    /**
      | Copies another search path.
      |
      */
    pub fn assign_from_file_search_path(&mut self, other: &FileSearchPath) -> &mut FileSearchPath {
        
        todo!();
        /*
            directories = other.directories;
        return *this;
        */
    }
    
    /**
      | Uses a string containing a list of pathnames
      | to re-initialise this list.
      | 
      | This search path is cleared and the semicolon-
      | or comma-separated folders in this
      | string are added instead. e.g. "/foo/bar;/foo/moose;/fish/moose"
      |
      */
    pub fn assign_from_path(&mut self, path: &String) -> &mut FileSearchPath {
        
        todo!();
        /*
            init (path);
        return *this;
        */
    }
    
    pub fn init(&mut self, path: &String)  {
        
        todo!();
        /*
            directories.clear();
        directories.addTokens (path, ";", "\"");
        directories.trim();
        directories.removeEmptyStrings();

        for (auto& d : directories)
            d = d.unquoted();
        */
    }
    
    /**
      | Returns the number of folders in this
      | search path. @see operator[]
      |
      */
    pub fn get_num_paths(&self) -> i32 {
        
        todo!();
        /*
            return directories.size();
        */
    }
    
    /**
      | Returns the search path as a semicolon-separated
      | list of directories.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            auto dirs = directories;

        for (auto& d : dirs)
            if (d.containsChar (';'))
                d = d.quoted();

        return dirs.joinIntoString (";");
        */
    }
    
    /**
      | Adds a new directory to the search path.
      | 
      | The new directory is added to the end
      | of the list if the insertIndex parameter
      | is less than zero, otherwise it is inserted
      | at the given index.
      |
      */
    pub fn add(
        &mut self, 
        dir:          &File,
        insert_index: Option<i32>
    )
    {
        let insert_index: i32 = insert_index.unwrap_or(-1);
        
        todo!();
        /*
            directories.insert (insertIndex, dir.getFullPathName());
        */
    }
    
    /**
      | Adds a new directory to the search path
      | if it's not already in there.
      | 
      | -----------
      | @return
      | 
      | true if the directory has been added,
      | false otherwise.
      |
      */
    pub fn add_if_not_already_there(&mut self, dir: &File) -> bool {
        
        todo!();
        /*
            for (auto& d : directories)
            if (File (d) == dir)
                return false;

        add (dir);
        return true;
        */
    }
    
    /**
      | Removes a directory from the search
      | path.
      |
      */
    pub fn remove(&mut self, index: i32)  {
        
        todo!();
        /*
            directories.remove (index);
        */
    }
    
    /**
      | Merges another search path into this
      | one.
      | 
      | This will remove any duplicate directories.
      |
      */
    pub fn add_path(&mut self, other: &FileSearchPath)  {
        
        todo!();
        /*
            for (int i = 0; i < other.getNumPaths(); ++i)
            addIfNotAlreadyThere (other[i]);
        */
    }
    
    /**
      | Removes any directories that are actually
      | subdirectories of one of the other directories
      | in the search path.
      | 
      | If the search is intended to be recursive,
      | there's no point having nested folders
      | in the search path, because they'll
      | just get searched twice and you'll get
      | duplicate results.
      | 
      | e.g. if the path is "c:\abc\de;c:\abc",
      | this method will simplify it to "c:\abc"
      |
      */
    pub fn remove_redundant_paths(&mut self)  {
        
        todo!();
        /*
            for (int i = directories.size(); --i >= 0;)
        {
            const File d1 (directories[i]);

            for (int j = directories.size(); --j >= 0;)
            {
                const File d2 (directories[j]);

                if (i != j && (d1.isAChildOf (d2) || d1 == d2))
                {
                    directories.remove (i);
                    break;
                }
            }
        }
        */
    }
    
    /**
      | Removes any directories that don't
      | actually exist.
      |
      */
    pub fn remove_non_existent_paths(&mut self)  {
        
        todo!();
        /*
            for (int i = directories.size(); --i >= 0;)
            if (! File (directories[i]).isDirectory())
                directories.remove (i);
        */
    }
    
    /**
      | Searches the path for a wildcard.
      | 
      | This will search all the directories
      | in the search path in order and return
      | an array of the files that were found.
      | 
      | -----------
      | @param whatToLookFor
      | 
      | a value from the File::TypesOfFileToFind
      | enum, specifying whether to return
      | files, directories, or both.
      | ----------
      | @param searchRecursively
      | 
      | whether to recursively search the subdirectories
      | too
      | ----------
      | @param wildCardPattern
      | 
      | a pattern to match against the filenames
      | 
      | -----------
      | @return
      | 
      | the number of files added to the array
      | 
      | @see File::findChildFiles
      |
      */
    pub fn find_child_files(
        &self, 
        what_to_look_for: i32,
        recurse:          bool,
        wildcard:         Option<&str>) -> Vec<File> {

        let wildcard = wildcard.unwrap_or("*");
        
        todo!();
        /*
            Vec<File> results;
        findChildFiles (results, whatToLookFor, recurse, wildcard);
        return results;
        */
    }
    
    /**
      | Searches the path for a wildcard.
      | 
      | -----------
      | @note
      | 
      | there's a newer, better version of this
      | method which returns the results array,
      | and in almost all cases, you should use
      | that one instead! This one is kept around
      | mainly for legacy code to use.
      |
      */
    pub fn find_child_files_into(
        &self, 
        results:          &mut Vec<File>,
        what_to_look_for: i32,
        recurse:          bool,
        wildcard:         Option<&str>) -> i32 {

        let wildcard = wildcard.unwrap_or("*");
        
        todo!();
        /*
            int total = 0;

        for (auto& d : directories)
            total += File (d).findChildFiles (results, whatToLookFor, recurse, wildcard);

        return total;
        */
    }
    
    /**
      | Finds out whether a file is inside one
      | of the path's directories.
      | 
      | This will return true if the specified
      | file is a child of one of the directories
      | specified by this path. Note that this
      | doesn't actually do any searching or
      | check that the files exist - it just looks
      | at the pathnames to work out whether
      | the file would be inside a directory.
      | 
      | -----------
      | @param fileToCheck
      | 
      | the file to look for
      | ----------
      | @param checkRecursively
      | 
      | if true, then this will return true if
      | the file is inside a subfolder of one
      | of the path's directories (at any depth).
      | If false it will only return true if the
      | file is actually a direct child of one
      | of the directories.
      | 
      | @see File::isAChildOf
      |
      */
    pub fn is_file_in_path(&self, 
        file_to_check:     &File,
        check_recursively: bool) -> bool {
        
        todo!();
        /*
            for (auto& d : directories)
        {
            if (checkRecursively)
            {
                if (fileToCheck.isAChildOf (File (d)))
                    return true;
            }
            else
            {
                if (fileToCheck.getParentDirectory() == File (d))
                    return true;
            }
        }

        return false;
        */
    }
}
