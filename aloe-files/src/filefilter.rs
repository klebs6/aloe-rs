crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileFilter.h]

/**
  | Interface for deciding which files
  | are suitable for something.
  | 
  | For example, this is used by DirectoryContentsList
  | to select which files go into the list.
  | 
  | @see WildcardFileFilter, DirectoryContentsList,
  | FileListComponent, FileBrowserComponent
  | 
  | @tags{Core}
  |
  */
pub struct FileFilter {
    description: String,
}

pub mod file_filter {
    use super::*;

    pub trait Interface {

        /**
          | Should return true if this file is suitable
          | for inclusion in whatever context the
          | object is being used.
          |
          */

        fn is_file_suitable(&self, file: &File) -> bool;

        /**
          | Should return true if this directory
          | is suitable for inclusion in whatever
          | context the object is being used.
          |
          */
        fn is_directory_suitable(&self, file: &File) -> bool;
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileFilter.cpp]
impl FileFilter {

    /**
      | Creates a filter with the given description.
      | 
      | The description can be returned later
      | with the getDescription() method.
      |
      */
    pub fn new(filter_description: &String) -> Self {
    
        todo!();
        /*
        : description(filterDescription),

        
        */
    }
    
    /**
      | Returns the description that the filter
      | was created with.
      |
      */
    pub fn get_description(&self) -> &String {
        
        todo!();
        /*
            return description;
        */
    }
}
