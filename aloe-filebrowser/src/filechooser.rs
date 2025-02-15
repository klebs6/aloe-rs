crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileChooser.h]

/**
  | Creates a dialog box to choose a file
  | or directory to load or save.
  | 
  | -----------
  | @code
  | 
  | std::unique_ptr<FileChooser> myChooser;
  | 
  | void loadMooseFile()
  | {
  |     myChooser = std::make_unique<FileChooser> ("Please select the moose you want to load...",
  |                                                File::getSpecialLocation (File::userHomeDirectory),
  |                                                "*.moose");
  | 
  |     auto folderChooserFlags = FileBrowserComponent::openMode | FileBrowserComponent::canSelectDirectories;
  | 
  |     myChooser->launchAsync (folderChooserFlags, [this] (const FileChooser& chooser)
  |     {
  |         File mooseFile (chooser.getResult());
  | 
  |         loadMoose (mooseFile);
  |     }
  | }
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileChooser<'a> {
    title:                       String,
    filters:                     String,
    starting_file:               File,
    parent:                      *mut Component<'a>,
    results:                     Vec<Url>,
    use_native_dialog_box:       bool,
    treat_file_packages_as_dirs: bool,
    async_callback:              fn(_0: &FileChooser) -> (),
    impl_:                       Arc<dyn FileChooserImpl>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileChooser.cpp]

impl<'a> Drop for FileChooser<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        asyncCallback = nullptr;
 */
    }
}

impl<'a> FileChooser<'a> {

    /**
      | Returns a list of all the files that were
      | chosen during the last call to a browse
      | method.
      | 
      | Use this method if you are using the FileChooser
      | on a mobile platform which may return
      | a Url to a remote document. If a local
      | file is chosen then you can convert this
      | file to a Aloe File class via the Url::getLocalFile
      | method.
      | 
      | This array may be empty if no files were
      | chosen, or can contain multiple entries
      | if multiple files were chosen.
      | 
      | -----------
      | @note
      | 
      | On iOS you must use the returned Url object
      | directly (you are also allowed to copy-
      | or move-construct another Url from
      | the returned Url), rather than just
      | storing the path as a String and then
      | creating a new Url from that
      | 
      | String. This is because the returned
      | Url contains internally a security
      | bookmark that is required to access
      | the files pointed by it. Then, once you
      | stop dealing with the file pointed by
      | the Url, you should dispose that Url
      | object, so that the security bookmark
      | can be released by the system (only a
      | limited number of such URLs is allowed).
      | 
      | @see getResults, Url::getLocalFile
      |
      */
    pub fn get_url_results(&self) -> &[Url] {
        
        todo!();
        /*
            return results;
        */
    }
    
    /**
     | Creates a FileChooser.
     | 
     | After creating one of these, use one
     | of the browseFor... methods to display
     | it.
     | 
     | -----------
     | @note
     | 
     | On iOS when saving a file, a user will
     | not be able to change a file name, so it
     | may be a good idea to include at least
     | a valid file name in initialFileOrDirectory.
     | When no filename is found, "Untitled"
     | will be used.
     | 
     | Also, if you pass an already existing
     | file on iOS, that file will be automatically
     | copied to the destination chosen by
     | user and if it can be previewed, its preview
     | will be presented in the dialog too.
     | You will still be able to write into this
     | file copy, since its Url will be returned
     | by getURLResult(). This can be useful
     | when you want to save e.g. an image, so
     | that you can pass a (temporary) file
     | with low quality preview and after the
     | user picks the destination, you can
     | write a high quality image into the copied
     | file. If you create such a temporary
     | file, you need to delete it yourself,
     | once it is not needed anymore.
     | 
     | -----------
     | @param dialogBoxTitle
     | 
     | a text string to display in the dialog
     | box to tell the user what's going on
     | ----------
     | @param initialFileOrDirectory
     | 
     | the file or directory that should be
     | selected when the dialog box opens.
     | If this parameter is set to File(), a
     | sensible default directory will be
     | used instead. When using native dialogs,
     | not all platforms will actually select
     | the file. For example, on macOS, when
     | initialFileOrDirectory is a file,
     | only the parent directory of initialFileOrDirectory
     | will be used as the initial directory
     | of the native file chooser.
     | ----------
     | @param filePatternsAllowed
     | 
     | a set of file patterns to specify which
     | files can be selected - each pattern
     | should be separated by a comma or semi-colon,
     | e.g. "*" or "*.jpg;*.gif". The native
     | MacOS file browser only supports wildcard
     | that specify extensions, so "*.jpg"
     | is OK but "myfilename*" will not work.
     | An empty string means that all files
     | are allowed
     | ----------
     | @param useOSNativeDialogBox
     | 
     | if true, then a native dialog box will
     | be used if possible; if false, then a
     | Aloe-based browser dialog box will
     | always be used
     | ----------
     | @param treatFilePackagesAsDirectories
     | 
     | if true, then the file chooser will allow
     | the selection of files inside packages
     | when invoked on OS X and when using native
     | dialog boxes.
     | ----------
     | @param parentComponent
     | 
     | An optional component which should
     | be the parent for the file chooser. If
     | this is a nullptr then the
     | 
     | FileChooser will be a top-level window.
     | AUv3s on iOS must specify this parameter
     | as opening a top-level window in an AUv3
     | is forbidden due to sandbox restrictions.
     | 
     | @see browseForFileToOpen, browseForFileToSave,
     | browseForDirectory
     |
     */
    pub fn new(
        chooser_box_title:                  &String,
        current_file_or_directory:          Option<&File>,
        file_filters:                       Option<&String>,
        use_native_box:                     Option<bool>,
        treat_file_packages_as_directories: Option<bool>,
        parent_component_to_use:            *mut Component

    ) -> Self {

        let current_file_or_directory = current_file_or_directory.unwrap_or(&File::default());
        let file_filters              = file_filters.unwrap_or(&String::new());

        let use_native_box:                     bool = use_native_box.unwrap_or(true);
        let treat_file_packages_as_directories: bool = treat_file_packages_as_directories.unwrap_or(false);
    
        todo!();

        /*
          : title (chooserBoxTitle),
          filters (fileFilters),
          startingFile (currentFileOrDirectory),
          parent (parentComponentToUse),
          useNativeDialogBox (useNativeBox && isPlatformDialogAvailable()),
          treatFilePackagesAsDirs (treatFilePackagesAsDirectories)

       #ifndef ALOE_MAC
        ignoreUnused (treatFilePackagesAsDirs);
       #endif

        if (! fileFilters.containsNonWhitespaceChars())
            filters = "*";
        */
    }

    /**
      | Shows a dialog box to choose a file to
      | open.
      | 
      | This will display the dialog box modally,
      | using an "open file" mode, so that it
      | won't allow non-existent files or directories
      | to be chosen.
      | 
      | -----------
      | @param previewComponent
      | 
      | an optional component to display inside
      | the dialog box to show special info about
      | the files that the user is browsing.
      | The component will not be deleted by
      | this object, so the caller must take
      | care of it.
      | 
      | -----------
      | @return
      | 
      | true if the user selected a file, in which
      | case, use the getResult() method to
      | find out what it was. Returns false if
      | they cancelled instead. @see browseForFileToSave,
      | browseForDirectory
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn browse_for_file_to_open(&mut self, preview_comp: *mut FilePreviewComponent) -> bool {
        
        todo!();
        /*
            return showDialog (FileBrowserComponent::openMode
                            | FileBrowserComponent::canSelectFiles,
                           previewComp);
        */
    }
    
    /**
      | Same as browseForFileToOpen, but allows
      | the user to select multiple files.
      | 
      | The files that are returned can be obtained
      | by calling getResults(). See browseForFileToOpen()
      | for more info about the behaviour of
      | this method.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn browse_for_multiple_files_to_open(&mut self, preview_comp: *mut FilePreviewComponent) -> bool {
        
        todo!();
        /*
            return showDialog (FileBrowserComponent::openMode
                            | FileBrowserComponent::canSelectFiles
                            | FileBrowserComponent::canSelectMultipleItems,
                           previewComp);
        */
    }
    
    /**
      | Same as browseForFileToOpen, but allows
      | the user to select multiple files and
      | directories.
      | 
      | The files that are returned can be obtained
      | by calling getResults(). See browseForFileToOpen()
      | for more info about the behaviour of
      | this method.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn browse_for_multiple_files_or_directories(&mut self, preview_comp: *mut FilePreviewComponent) -> bool {
        
        todo!();
        /*
            return showDialog (FileBrowserComponent::openMode
                            | FileBrowserComponent::canSelectFiles
                            | FileBrowserComponent::canSelectDirectories
                            | FileBrowserComponent::canSelectMultipleItems,
                           previewComp);
        */
    }
    
    /**
      | Shows a dialog box to choose a file to
      | save.
      | 
      | This will display the dialog box modally,
      | using an "save file" mode, so it will
      | allow non-existent files to be chosen,
      | but not directories.
      | 
      | -----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true, the dialog box will ask the user
      | if they're sure they want to overwrite
      | a file that already exists
      | 
      | -----------
      | @return
      | 
      | true if the user chose a file and pressed
      | 'ok', in which case, use the getResult()
      | method to find out what the file was.
      | Returns false if they cancelled instead.
      | @see browseForFileToOpen, browseForDirectory
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn browse_for_file_to_save(&mut self, warn_about_overwrite: bool) -> bool {
        
        todo!();
        /*
            return showDialog (FileBrowserComponent::saveMode
                            | FileBrowserComponent::canSelectFiles
                            | (warnAboutOverwrite ? FileBrowserComponent::warnAboutOverwriting : 0),
                           nullptr);
        */
    }
    
    /**
      | Shows a dialog box to choose a directory.
      | 
      | This will display the dialog box modally,
      | using an "open directory" mode, so it
      | will only allow directories to be returned,
      | not files.
      | 
      | -----------
      | @return
      | 
      | true if the user chose a directory and
      | pressed 'ok', in which case, use the
      | getResult() method to find out what
      | they chose. Returns false if they cancelled
      | instead. @see browseForFileToOpen,
      | browseForFileToSave
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn browse_for_directory(&mut self) -> bool {
        
        todo!();
        /*
            return showDialog (FileBrowserComponent::openMode
                            | FileBrowserComponent::canSelectDirectories,
                           nullptr);
        */
    }
    
    /**
      | Runs a dialog box for the given set of
      | option flags.
      | 
      | The flag values used are those in FileBrowserComponent::FileChooserFlags.
      | 
      | -----------
      | @return
      | 
      | true if the user chose a directory and
      | pressed 'ok', in which case, use the
      | getResult() method to find out what
      | they chose. Returns false if they cancelled
      | instead. @see FileBrowserComponent::FileChooserFlags
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_dialog(&mut self, 
        flags:        i32,
        preview_comp: *mut FilePreviewComponent) -> bool {
        
        todo!();
        /*
            FocusRestorer focusRestorer;

        impl = createImpl (flags, previewComp);
        impl->runModally();

        // ensure that the finished function was invoked
        jassert (impl == nullptr);

        return (results.size() > 0);
        */
    }
    
    /**
      | Use this method to launch the file browser
      | window asynchronously.
      | 
      | This will create a file browser dialog
      | based on the settings in this structure
      | and will launch it modally, returning
      | immediately.
      | 
      | You must specify a callback which is
      | called when the file browser is cancelled
      | or a file is selected. To abort the file
      | selection, simply delete the FileChooser
      | object.
      | 
      | You must ensure that the lifetime of
      | the callback object is longer than the
      | lifetime of the file-chooser.
      |
      */
    pub fn launch_async(&mut self, 
        flags:        i32,
        callback:     fn(_0: &FileChooser) -> (),
        preview_comp: *mut FilePreviewComponent)  {
        
        todo!();
        /*
            // You must specify a callback when using launchAsync
        jassert (callback);

        // you cannot run two file chooser dialog boxes at the same time
        jassert (asyncCallback == nullptr);

        asyncCallback = std::move (callback);

        impl = createImpl (flags, previewComp);
        impl->launch();
        */
    }
    
    pub fn create_pimpl(
        &mut self, 
        flags:        i32,
        preview_comp: *mut FilePreviewComponent

    ) -> Arc<dyn FileChooserImpl> {
        
        todo!();
        /*
            results.clear();

        // the preview component needs to be the right size before you pass it in here..
        jassert (previewComp == nullptr || (previewComp->getWidth() > 10
                                             && previewComp->getHeight() > 10));

        if (impl != nullptr)
        {
            // you cannot run two file chooser dialog boxes at the same time
            jassertfalse;
            impl.reset();
        }

        // You've set the flags for both saveMode and openMode!
        jassert (! (((flags & FileBrowserComponent::saveMode) != 0)
                    && ((flags & FileBrowserComponent::openMode) != 0)));

       #if ALOE_WINDOWS
        const bool selectsFiles       = (flags & FileBrowserComponent::canSelectFiles) != 0;
        const bool selectsDirectories = (flags & FileBrowserComponent::canSelectDirectories) != 0;

        if (useNativeDialogBox && ! (selectsFiles && selectsDirectories))
       #else
        if (useNativeDialogBox)
       #endif
        {
            return showPlatformDialog (*this, flags, previewComp);
        }

        return std::make_unique<FileChooserNonNative> (*this, flags, previewComp);
        */
    }
    
    /**
      | Returns a list of all the files that were
      | chosen during the last call to a browse
      | method.
      | 
      | On mobile platforms, the file browser
      | may return a Url instead of a local file.
      | 
      | Therefore, on mobile platforms, you
      | should call getURLResults() instead.
      | 
      | This array may be empty if no files were
      | chosen, or can contain multiple entries
      | if multiple files were chosen.
      | 
      | @see getURLResults, getResult
      |
      */
    pub fn get_results(&self) -> Vec<File> {
        
        todo!();
        /*
            Vec<File> files;

        for (auto url : getURLResults())
            if (url.isLocalFile())
                files.add (url.getLocalFile());

        return files;
        */
    }
    
    /**
      | Returns the last file that was chosen
      | by one of the browseFor methods.
      | 
      | After calling the appropriate browseFor...
      | method, this method lets you find out
      | what file or directory they chose.
      | 
      | -----------
      | @note
      | 
      | the file returned is only valid if the
      | browse method returned true (i.e. if
      | the user pressed 'ok' rather than cancelling).
      | 
      | On mobile platforms, the file browser
      | may return a Url instead of a local file.
      | 
      | Therefore, on mobile platforms, you
      | should call getURLResult() instead.
      | 
      | If you're using a multiple-file select,
      | then use the getResults() method instead,
      | to obtain the list of all files chosen.
      | 
      | @see getURLResult, getResults
      |
      */
    pub fn get_result(&self) -> File {
        
        todo!();
        /*
            auto fileResults = getResults();

        // if you've used a multiple-file select, you should use the getResults() method
        // to retrieve all the files that were chosen.
        jassert (fileResults.size() <= 1);

        return fileResults.getFirst();
        */
    }
    
    /**
      | Returns the last document that was chosen
      | by one of the browseFor methods.
      | 
      | Use this method if you are using the FileChooser
      | on a mobile platform which may return
      | a Url to a remote document. If a local
      | file is chosen then you can convert this
      | file to a Aloe File class via the Url::getLocalFile
      | method.
      | 
      | -----------
      | @note
      | 
      | On iOS you must use the returned Url object
      | directly (you are also allowed to copy-
      | or move-construct another Url from
      | the returned Url), rather than just
      | storing the path as a String and then
      | creating a new Url from that
      | 
      | String. This is because the returned
      | Url contains internally a security
      | bookmark that is required to access
      | the files pointed by it. Then, once you
      | stop dealing with the file pointed by
      | the Url, you should dispose that Url
      | object, so that the security bookmark
      | can be released by the system (only a
      | limited number of such URLs is allowed).
      | 
      | @see getResult, Url::getLocalFile
      |
      */
    pub fn get_url_result(&self) -> Url {
        
        todo!();
        /*
            // if you've used a multiple-file select, you should use the getResults() method
        // to retrieve all the files that were chosen.
        jassert (results.size() <= 1);

        return results.getFirst();
        */
    }
    
    pub fn finished(&mut self, async_results: &[Url])  {
        
        todo!();
        /*
            std::function<void (const FileChooser&)> callback;
         std::swap (callback, asyncCallback);

         results = asyncResults;

         impl.reset();

         if (callback)
             callback (*this);
        */
    }
}
