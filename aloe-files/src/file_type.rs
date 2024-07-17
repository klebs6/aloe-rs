crate::ix!();

/**
  | Used in file searching, to specify whether
  | to return files, directories, or both.
  |
  */
pub enum TypesOfFileToFind
{
    /**
      Use this flag to indicate that you want to
      find directories.
      */
    findDirectories             = 1,    

    /**
      Use this flag to indicate that you want to
      find files.
      */
    findFiles                   = 2,    

    /**
      Use this flag to indicate that you want to
      find both files and directories.
      */
    findFilesAndDirectories     = 3,    

    /**
      Add this flag to avoid returning any hidden
      files in the results.
      */
    ignoreHiddenFiles           = 4,     
}
