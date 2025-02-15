crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_linux_CommonFile.cpp]

pub mod directory_iterator {
    use super::*;

    #[no_copy]
    #[cfg(target_os="linux")]
    pub struct NativeIteratorImpl {
        parent_dir: String,
        wild_card:  String,
        dir:        *mut DIR,
    }

    #[cfg(target_os="linux")]
    impl Drop for NativeIteratorImpl {
        fn drop(&mut self) {
            todo!();
            /* 
                    if (dir != nullptr)
                        closedir (dir);
                 */
        }
    }

    #[cfg(target_os="linux")]
    impl NativeIteratorImpl {

        pub fn new(
            directory: &File,
            wc:        &String) -> Self {
        
            todo!();
            /*


                : parentDir (File::addTrailingSeparator (directory.getFullPathName())),
                      wildCard (wc), dir (opendir (directory.getFullPathName().toUTF8()))
            */
        }
        
        pub fn next(&mut self, 
            filename_found: &mut String,
            is_dir:         *mut bool,
            is_hidden:      *mut bool,
            file_size:      *mut i64,
            mod_time:       *mut Time,
            creation_time:  *mut Time,
            is_read_only:   *mut bool) -> bool {
            
            todo!();
            /*
                if (dir != nullptr)
                    {
                        const char* wildcardUTF8 = nullptr;

                        for (;;)
                        {
                            struct dirent* const de = readdir (dir);

                            if (de == nullptr)
                                break;

                            if (wildcardUTF8 == nullptr)
                                wildcardUTF8 = wildCard.toUTF8();

                            if (fnmatch (wildcardUTF8, de->d_name, FNM_CASEFOLD) == 0)
                            {
                                filenameFound = CharPointer_UTF8 (de->d_name);

                                updateStatInfoForFile (parentDir + filenameFound, isDir, fileSize, modTime, creationTime, isReadOnly);

                                if (isHidden != nullptr)
                                    *isHidden = filenameFound.startsWithChar ('.');

                                return true;
                            }
                        }
                    }

                    return false;
            */
        }
    }
}
