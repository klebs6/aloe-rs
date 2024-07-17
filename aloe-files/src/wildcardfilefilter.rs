crate::ix!();

pub fn parse_wildcard(
        pattern: &String,
        result:  &mut StringArray)  {
    
    todo!();
    /*
        result.addTokens (pattern.toLowerCase(), ";,", "\"'");
        result.trim();
        result.removeEmptyStrings();

        // special case for *.*, because people use it to mean "any file", but it
        // would actually ignore files with no extension.
        for (auto& r : result)
            if (r == "*.*")
                r = "*";
    */
}

pub fn match_wildcard(
        file:      &File,
        wildcards: &StringArray) -> bool {
    
    todo!();
    /*
        auto filename = file.getFileName();

        for (auto& w : wildcards)
            if (filename.matchesWildcard (w, true))
                return true;

        return false;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_WildcardFileFilter.h]

/**
  | A type of FileFilter that works by wildcard
  | pattern matching.
  | 
  | This filter only allows files that match
  | one of the specified patterns, but allows
  | all directories through.
  | 
  | @see FileFilter, DirectoryContentsList,
  | FileListComponent, FileBrowserComponent
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct WildcardFileFilter {
    base:                FileFilter,
    file_wildcards:      StringArray,
    directory_wildcards: StringArray,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_WildcardFileFilter.cpp]
impl WildcardFileFilter {

    /**
      | Creates a wildcard filter for one or
      | more patterns.
      | 
      | The wildcardPatterns parameter is
      | a comma or semicolon-delimited set
      | of patterns, e.g. "*.wav;*.aiff" would
      | look for files ending in either .wav
      | or .aiff.
      | 
      | Passing an empty string as a pattern
      | will fail to match anything, so by leaving
      | either the file or directory pattern
      | parameter empty means you can control
      | whether files or directories are found.
      | 
      | The description is a name to show the
      | user in a list of possible patterns,
      | so for the wav/aiff example, your description
      | might be "audio files".
      |
      */
    pub fn new(
        file_wildcard_patterns:      &String,
        directory_wildcard_patterns: &String,
        desc:                        &String) -> Self {
    
        todo!();
        /*


            : FileFilter (desc.isEmpty() ? fileWildcardPatterns
                                     : (desc + " (" + fileWildcardPatterns + ")"))

        parseWildcard (fileWildcardPatterns, fileWildcards);
        parseWildcard (directoryWildcardPatterns, directoryWildcards);
        */
    }
    
    /**
      | Returns true if the filename matches
      | one of the patterns specified.
      |
      */
    pub fn is_file_suitable(&self, file: &File) -> bool {
        
        todo!();
        /*
            return matchWildcard (file, fileWildcards);
        */
    }
    
    /**
      | This always returns true.
      |
      */
    pub fn is_directory_suitable(&self, file: &File) -> bool {
        
        todo!();
        /*
            return matchWildcard (file, directoryWildcards);
        */
    }
}
