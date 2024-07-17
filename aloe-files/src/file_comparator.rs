crate::ix!();

/**
  | Comparator for files
  |
  */
pub struct NaturalFileComparator {
    folders_first: bool,
}

impl NaturalFileComparator {

    pub fn new(should_put_folders_first: bool) -> Self {
    
        todo!();
        /*
        : folders_first(shouldPutFoldersFirst),
        */
    }
    
    pub fn compare_elements(&self, 
        first_file:  &File,
        second_file: &File) -> i32 {
        
        todo!();
        /*
            if (foldersFirst && (firstFile.isDirectory() != secondFile.isDirectory()))
                    return firstFile.isDirectory() ? -1 : 1;

               #if NAMES_ARE_CASE_SENSITIVE
                return firstFile.getFullPathName().compareNatural (secondFile.getFullPathName(), true);
               #else
                return firstFile.getFullPathName().compareNatural (secondFile.getFullPathName(), false);
               #endif
        */
    }
}
