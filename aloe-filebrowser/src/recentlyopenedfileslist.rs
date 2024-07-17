crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_RecentlyOpenedFilesList.h]

/**
  | Manages a set of files for use as a list
  | of recently-opened documents.
  | 
  | This is a handy class for holding your
  | list of recently-opened documents,
  | with helpful methods for things like
  | purging any non-existent files, automatically
  | adding them to a menu, and making persistence
  | easy.
  | 
  | @see File, FileBasedDocument
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct RecentlyOpenedFilesList {
    files:               Vec<String>,
    max_number_of_items: i32,
}

impl Default for RecentlyOpenedFilesList {
    
    /**
      | Creates an empty list.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_RecentlyOpenedFilesList.cpp]
impl RecentlyOpenedFilesList {

    /**
      | Returns the number of items that this
      | list will store. @see setMaxNumberOfItems
      |
      */
    pub fn get_max_number_of_items(&self) -> i32 {
        
        todo!();
        /*
            return maxNumberOfItems;
        */
    }

    /**
      | Returns an array of all the absolute
      | pathnames in the list.
      |
      */
    pub fn get_all_filenames(&self) -> &Vec<String> {
        
        todo!();
        /*
            return files;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : max_number_of_items(10),

        
        */
    }
    
    /**
      | Sets a limit for the number of files that
      | will be stored in the list.
      | 
      | When addFile() is called, then if there
      | is no more space in the list, the least-recently
      | added file will be dropped.
      | 
      | @see getMaxNumberOfItems
      |
      */
    pub fn set_max_number_of_items(&mut self, new_max_number: i32)  {
        
        todo!();
        /*
            maxNumberOfItems = jmax (1, newMaxNumber);

        files.removeRange (maxNumberOfItems, getNumFiles());
        */
    }
    
    /**
      | Returns the number of files in the list.
      | 
      | The most recently added file is always
      | at index 0.
      |
      */
    pub fn get_num_files(&self) -> i32 {
        
        todo!();
        /*
            return files.size();
        */
    }
    
    /**
      | Returns one of the files in the list.
      | 
      | The most recently added file is always
      | at index 0.
      |
      */
    pub fn get_file(&self, index: i32) -> File {
        
        todo!();
        /*
            return File (files [index]);
        */
    }
    
    /**
      | Clears all the files from the list.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            files.clear();
        */
    }
    
    /**
      | Adds a file to the list.
      | 
      | The file will be added at index 0. If this
      | file is already in the list, it will be
      | moved up to index 0, but a file can only
      | appear once in the list.
      | 
      | If the list already contains the maximum
      | number of items that is permitted, the
      | least-recently added file will be dropped
      | from the end.
      |
      */
    pub fn add_file(&mut self, file: &File)  {
        
        todo!();
        /*
            removeFile (file);
        files.insert (0, file.getFullPathName());

        setMaxNumberOfItems (maxNumberOfItems);
        */
    }
    
    /**
      | Removes a file from the list.
      |
      */
    pub fn remove_file(&mut self, file: &File)  {
        
        todo!();
        /*
            files.removeString (file.getFullPathName());
        */
    }
    
    /**
      | Checks each of the files in the list,
      | removing any that don't exist.
      | 
      | You might want to call this after reloading
      | a list of files, or before putting them
      | on a menu.
      |
      */
    pub fn remove_non_existent_files(&mut self)  {
        
        todo!();
        /*
            for (int i = getNumFiles(); --i >= 0;)
            if (! getFile(i).exists())
                files.remove (i);
        */
    }
    
    /**
      | Adds entries to a menu, representing
      | each of the files in the list.
      | 
      | This is handy for creating an "open recent
      | file..." menu in your app. The menu items
      | are numbered consecutively starting
      | with the baseItemId value, and can either
      | be added as complete pathnames, or just
      | the last part of the filename.
      | 
      | If dontAddNonExistentFiles is true,
      | then each file will be checked and only
      | those that exist will be added.
      | 
      | If filesToAvoid is not a nullptr, then
      | it is considered to be a zero-terminated
      | array of pointers to file objects. Any
      | files that appear in this list will not
      | be added to the menu - the reason for this
      | is that you might have a number of files
      | already open, so might not want these
      | to be shown in the menu.
      | 
      | It returns the number of items that were
      | added.
      |
      */
    pub fn create_popup_menu_items(&mut self, 
        menu_to_add_to:              &mut PopupMenu,
        base_item_id:                i32,
        show_full_paths:             bool,
        dont_add_non_existent_files: bool,
        files_to_avoid:              *const *const File) -> i32 {
        
        todo!();
        /*
            int num = 0;

        for (int i = 0; i < getNumFiles(); ++i)
        {
            const File f (getFile(i));

            if ((! dontAddNonExistentFiles) || f.exists())
            {
                bool needsAvoiding = false;

                if (filesToAvoid != nullptr)
                {
                    for (const File** avoid = filesToAvoid; *avoid != nullptr; ++avoid)
                    {
                        if (f == **avoid)
                        {
                            needsAvoiding = true;
                            break;
                        }
                    }
                }

                if (! needsAvoiding)
                {
                    menuToAddTo.addItem (baseItemId + i,
                                         showFullPaths ? f.getFullPathName()
                                                       : f.getFileName());
                    ++num;
                }
            }
        }

        return num;
        */
    }
    
    /**
      | Returns a string that encapsulates
      | all the files in the list.
      | 
      | The string that is returned can later
      | be passed into restoreFromString()
      | in order to recreate the list. This is
      | handy for persisting your list, e.g.
      | in a PropertiesFile object.
      | 
      | @see restoreFromString
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return files.joinIntoString ("\n");
        */
    }
    
    /**
      | Restores the list from a previously
      | stringified version of the list.
      | 
      | Pass in a stringified version created
      | with toString() in order to persist/restore
      | your list.
      | 
      | @see toString
      |
      */
    pub fn restore_from_string(&mut self, stringified_version: &String)  {
        
        todo!();
        /*
            clear();
        files.addLines (stringifiedVersion);

        setMaxNumberOfItems (maxNumberOfItems);
        */
    }
    
    /**
      | Tells the OS to add a file to the OS-managed
      | list of recent documents for this app.
      | 
      | Not all OSes maintain a list of recent
      | files for an application, so this function
      | will have no effect on some OSes. Currently
      | it's just implemented for OSX.
      |
      */
    pub fn register_recent_file_natively(&mut self, file: &File)  {
        
        todo!();
        /*
            #if ALOE_MAC
        ALOE_AUTORELEASEPOOL
        {
            [[NSDocumentController sharedDocumentController] noteNewRecentDocumentURL: createNSURLFromFile (file)];
        }
       #else
        ignoreUnused (file);
       #endif
        */
    }
    
    /**
      | Tells the OS to remove a file from the
      | OS-managed list of recent documents
      | for this app.
      | 
      | Not all OSes maintain a list of recent
      | files for an application, so this function
      | will have no effect on some OSes. Currently
      | it's just implemented for OSX.
      |
      */
    pub fn forget_recent_file_natively(&mut self, file: &File)  {
        
        todo!();
        /*
            #if ALOE_MAC
        ALOE_AUTORELEASEPOOL
        {
            // for some reason, OSX doesn't provide a method to just remove a single file
            // from the recent list, so we clear them all and add them back excluding
            // the specified file

            auto sharedDocController = [NSDocumentController sharedDocumentController];
            auto recentDocumentURLs  = [sharedDocController recentDocumentURLs];

            [sharedDocController clearRecentDocuments: nil];

            auto* nsFile = createNSURLFromFile (file);

            auto reverseEnumerator = [recentDocumentURLs reverseObjectEnumerator];

            for (NSURL* url : reverseEnumerator)
                if (! [url isEqual:nsFile])
                    [sharedDocController noteNewRecentDocumentURL:url];
        }
       #else
        ignoreUnused (file);
       #endif
        */
    }
    
    /**
      | Tells the OS to clear the OS-managed
      | list of recent documents for this app.
      | 
      | Not all OSes maintain a list of recent
      | files for an application, so this function
      | will have no effect on some OSes. Currently
      | it's just implemented for OSX.
      |
      */
    pub fn clear_recent_files_natively(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MAC
        ALOE_AUTORELEASEPOOL
        {
            [[NSDocumentController sharedDocumentController] clearRecentDocuments: nil];
        }
       #endif
        */
    }
}
