crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileBrowserComponent.h]

/**
  | A component for browsing and selecting
  | a file or directory to open or save.
  | 
  | This contains a FileListComponent
  | and adds various boxes and controls
  | for navigating and selecting a file.
  | It can work in different modes so that
  | it can be used for loading or saving a
  | file, or for choosing a directory.
  | 
  | @see FileChooserDialogBox, FileChooser,
  | FileListComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileBrowserComponent<'a> {
    base:                Component<'a>,
    base3:               FileFilter,
    base4:               Timer,
    file_list:           Box<DirectoryContentsList<'a>>,
    file_filter:         *const FileFilter,
    flags:               i32,
    current_root:        File,
    chosen_files:        Vec<File>,
    listeners:           ListenerList<Rc<RefCell<dyn FileBrowserListener>>>,
    file_list_component: Box<DirectoryContentsDisplayComponent<'a>>,
    preview_comp:        *mut FilePreviewComponent<'a>,
    current_path_box:    ComboBox<'a>,
    filename_box:        TextEditor<'a>,
    file_label:          Label<'a>,
    go_up_button:        Box<Button<'a>>,
    thread:              TimeSliceThread,
    was_process_active:  bool,
}

impl<'a> FileBrowserListener for FileBrowserComponent<'a> {

    fn selection_changed(&mut self)  {
        
        todo!();
        /*
            Vec<String> newFilenames;
        bool resetChosenFiles = true;

        for (int i = 0; i < fileListComponent->getNumSelectedFiles(); ++i)
        {
            const File f (fileListComponent->getSelectedFile (i));

            if (isFileOrDirSuitable (f))
            {
                if (resetChosenFiles)
                {
                    chosenFiles.clear();
                    resetChosenFiles = false;
                }

                chosenFiles.add (f);
                newFilenames.add (f.getRelativePathFrom (getRoot()));
            }
        }

        if (newFilenames.size() > 0)
            filenameBox.setText (newFilenames.joinIntoString (", "), false);

        sendListenerChangeMessage();
        */
    }
    
    fn file_clicked(&mut self, 
        f: &File,
        e: &MouseEvent)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [&] (FileBrowserListener& l) { l.fileClicked (f, e); });
        */
    }
    
    fn file_double_clicked(&mut self, f: &File)  {
        
        todo!();
        /*
            if (f.isDirectory())
        {
            setRoot (f);

            if ((flags & canSelectDirectories) != 0 && (flags & doNotClearFileNameOnRootChange) == 0)
                filenameBox.setText ({});
        }
        else
        {
            Component::BailOutChecker checker (this);
            listeners.callChecked (checker, [&] (FileBrowserListener& l) { l.fileDoubleClicked (f); });
        }
        */
    }
    
    fn browser_root_changed(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for FileBrowserComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        fileListComponent.reset();
        fileList.reset();
        thread.stopThread (10000);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileBrowserComponent.cpp]
impl<'a> FileBrowserComponent<'a> {

    /**
      | Creates a FileBrowserComponent.
      | 
      | -----------
      | @param flags
      | 
      | A combination of flags from the FileChooserFlags
      | enumeration, used to specify the component's
      | behaviour. The flags must contain either
      | openMode or saveMode, and canSelectFiles
      | and/or canSelectDirectories.
      | ----------
      | @param initialFileOrDirectory
      | 
      | The file or directory that should be
      | selected when the component begins.
      | 
      | If this is File(), a default directory
      | will be chosen.
      | ----------
      | @param fileFilter
      | 
      | an optional filter to use to determine
      | which files are shown.
      | 
      | If this is nullptr then all files are
      | displayed. Note that a pointer is kept
      | internally to this object, so make sure
      | that it is not deleted before the FileBrowserComponent
      | object is deleted.
      | ----------
      | @param previewComp
      | 
      | an optional preview component that
      | will be used to show previews of files
      | that the user selects
      |
      */
    pub fn new(
        flags:                     i32,
        initial_file_or_directory: &File,
        file_filter:               *const FileFilter,
        preview_comp:              *mut FilePreviewComponent) -> Self {
    
        todo!();
        /*


            : FileFilter ({}),
         fileFilter (fileFilter_),
         flags (flags_),
         previewComp (previewComp_),
         currentPathBox ("path"),
         fileLabel ("f", TRANS ("file:")),
         thread ("Aloe FileBrowser"),
         wasProcessActive (true)

        // You need to specify one or other of the open/save flags..
        jassert ((flags & (saveMode | openMode)) != 0);
        jassert ((flags & (saveMode | openMode)) != (saveMode | openMode));

        // You need to specify at least one of these flags..
        jassert ((flags & (canSelectFiles | canSelectDirectories)) != 0);

        String filename;

        if (initialFileOrDirectory == File())
        {
            currentRoot = File::getCurrentWorkingDirectory();
        }
        else if (initialFileOrDirectory.isDirectory())
        {
            currentRoot = initialFileOrDirectory;
        }
        else
        {
            chosenFiles.add (initialFileOrDirectory);
            currentRoot = initialFileOrDirectory.getParentDirectory();
            filename = initialFileOrDirectory.getFileName();
        }

        fileList.reset (new DirectoryContentsList (this, thread));
        fileList->setDirectory (currentRoot, true, true);

        if ((flags & useTreeView) != 0)
        {
            auto tree = new FileTreeComponent (*fileList);
            fileListComponent.reset (tree);

            if ((flags & canSelectMultipleItems) != 0)
                tree->setMultiSelectEnabled (true);

            addAndMakeVisible (tree);
        }
        else
        {
            auto list = new FileListComponent (*fileList);
            fileListComponent.reset (list);
            list->setOutlineThickness (1);

            if ((flags & canSelectMultipleItems) != 0)
                list->setMultipleSelectionEnabled (true);

            addAndMakeVisible (list);
        }

        fileListComponent->addListener (this);

        addAndMakeVisible (currentPathBox);
        currentPathBox.setEditableText (true);
        resetRecentPaths();
        currentPathBox.onChange = [this] { updateSelectedPath(); };

        addAndMakeVisible (filenameBox);
        filenameBox.setMultiLine (false);
        filenameBox.setSelectAllWhenFocused (true);
        filenameBox.setText (filename, false);
        filenameBox.onTextChange = [this] { sendListenerChangeMessage(); };
        filenameBox.onReturnKey  = [this] { changeFilename(); };
        filenameBox.onFocusLost  = [this]
        {
            if (! isSaveMode())
                selectionChanged();
        };

        filenameBox.setReadOnly ((flags & (filenameBoxIsReadOnly | canSelectMultipleItems)) != 0);

        addAndMakeVisible (fileLabel);
        fileLabel.attachToComponent (&filenameBox, true);

        if (previewComp != nullptr)
            addAndMakeVisible (previewComp);

        lookAndFeelChanged();

        setRoot (currentRoot);

        if (filename.isNotEmpty())
            setFileName (filename);

        thread.startThread (4);

        startTimer (2000);
        */
    }
    
    /**
      | Adds a listener to be told when the user
      | selects and clicks on files. @see removeListener
      |
      */
    pub fn add_listener(&mut self, new_listener: *mut dyn FileBrowserListener)  {
        
        todo!();
        /*
            listeners.add (newListener);
        */
    }
    
    /**
      | Removes a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn FileBrowserListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    /**
      | Returns true if the saveMode flag was
      | set when this component was created.
      |
      */
    pub fn is_save_mode(&self) -> bool {
        
        todo!();
        /*
            return (flags & saveMode) != 0;
        */
    }
    
    /**
      | Returns the number of files that the
      | user has got selected.
      | 
      | If multiple select isn't active, this
      | will only be 0 or 1. To get the complete
      | list of files they've chosen, pass an
      | index to getCurrentFile().
      |
      */
    pub fn get_num_selected_files(&self) -> i32 {
        
        todo!();
        /*
            if (chosenFiles.isEmpty() && currentFileIsValid())
            return 1;

        return chosenFiles.size();
        */
    }
    
    /**
      | Returns one of the files that the user
      | has chosen.
      | 
      | If the box has multi-select enabled,
      | the index lets you specify which of the
      | files to get - see getNumSelectedFiles()
      | to find out how many files were chosen.
      | @see getHighlightedFile
      |
      */
    pub fn get_selected_file(&self, index: i32) -> File {
        
        todo!();
        /*
            if ((flags & canSelectDirectories) != 0 && filenameBox.getText().isEmpty())
            return currentRoot;

        if (! filenameBox.isReadOnly())
            return currentRoot.getChildFile (filenameBox.getText());

        return chosenFiles[index];
        */
    }
    
    /**
      | Returns true if the currently selected
      | file(s) are usable.
      | 
      | This can be used to decide whether the
      | user can press "ok" for the current file.
      | What it does depends on the mode, so for
      | example in an "open" mode, this only
      | returns true if a file has been selected
      | and if it exists.
      | 
      | In a "save" mode, a non-existent file
      | would also be valid.
      |
      */
    pub fn current_file_is_valid(&self) -> bool {
        
        todo!();
        /*
            auto f = getSelectedFile (0);

        if ((flags & canSelectDirectories) == 0 && f.isDirectory())
            return false;

        return isSaveMode() || f.exists();
        */
    }
    
    /**
      | This returns the last item in the view
      | that the user has highlighted.
      | 
      | This may be different from getCurrentFile(),
      | which returns the value that is shown
      | in the filename box, and if there are
      | multiple selections, this will only
      | return one of them. @see getSelectedFile
      |
      */
    pub fn get_highlighted_file(&self) -> File {
        
        todo!();
        /*
            return fileListComponent->getSelectedFile (0);
        */
    }
    
    /**
      | Deselects any files that are currently
      | selected.
      |
      */
    pub fn deselect_all_files(&mut self)  {
        
        todo!();
        /*
            fileListComponent->deselectAllFiles();
        */
    }
    
    pub fn is_file_suitable(&self, file: &File) -> bool {
        
        todo!();
        /*
            return (flags & canSelectFiles) != 0
                 && (fileFilter == nullptr || fileFilter->isFileSuitable (file));
        */
    }
    
    pub fn is_directory_suitable(&self, _0: &File) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_file_or_dir_suitable(&self, f: &File) -> bool {
        
        todo!();
        /*
            if (f.isDirectory())
            return (flags & canSelectDirectories) != 0
                     && (fileFilter == nullptr || fileFilter->isDirectorySuitable (f));

        return (flags & canSelectFiles) != 0 && f.exists()
                 && (fileFilter == nullptr || fileFilter->isFileSuitable (f));
        */
    }
    
    /**
      | Returns the directory whose contents
      | are currently being shown in the listbox.
      |
      */
    pub fn get_root(&self) -> &File {
        
        todo!();
        /*
            return currentRoot;
        */
    }
    
    /**
      | Changes the directory that's being
      | shown in the listbox.
      |
      */
    pub fn set_root(&mut self, new_root_directory: &File)  {
        
        todo!();
        /*
            bool callListeners = false;

        if (currentRoot != newRootDirectory)
        {
            callListeners = true;
            fileListComponent->scrollToTop();

            String path (newRootDirectory.getFullPathName());

            if (path.isEmpty())
                path = File::getSeparatorString();

            Vec<String> rootNames, rootPaths;
            getRoots (rootNames, rootPaths);

            if (! rootPaths.contains (path, true))
            {
                bool alreadyListed = false;

                for (int i = currentPathBox.getNumItems(); --i >= 0;)
                {
                    if (currentPathBox.getItemText (i).equalsIgnoreCase (path))
                    {
                        alreadyListed = true;
                        break;
                    }
                }

                if (! alreadyListed)
                    currentPathBox.addItem (path, currentPathBox.getNumItems() + 2);
            }
        }

        currentRoot = newRootDirectory;
        fileList->setDirectory (currentRoot, true, true);

        if (auto* tree = dynamic_cast<FileTreeComponent*> (fileListComponent.get()))
            tree->refresh();

        auto currentRootName = currentRoot.getFullPathName();

        if (currentRootName.isEmpty())
            currentRootName = File::getSeparatorString();

        currentPathBox.setText (currentRootName, dontSendNotification);

        goUpButton->setEnabled (currentRoot.getParentDirectory().isDirectory()
                                 && currentRoot.getParentDirectory() != currentRoot);

        if (callListeners)
        {
            Component::BailOutChecker checker (this);
            listeners.callChecked (checker, [&] (FileBrowserListener& l) { l.browserRootChanged (currentRoot); });
        }
        */
    }
    
    /**
      | Changes the name that is currently shown
      | in the filename box.
      |
      */
    pub fn set_file_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            filenameBox.setText (newName, true);

        fileListComponent->setSelectedFile (currentRoot.getChildFile (newName));
        */
    }
    
    /**
      | Updates the items in the dropdown list
      | of recent paths with the values from
      | getRoots().
      |
      */
    pub fn reset_recent_paths(&mut self)  {
        
        todo!();
        /*
            currentPathBox.clear();

        Vec<String> rootNames, rootPaths;
        getRoots (rootNames, rootPaths);

        for (int i = 0; i < rootNames.size(); ++i)
        {
            if (rootNames[i].isEmpty())
                currentPathBox.addSeparator();
            else
                currentPathBox.addItem (rootNames[i], i + 1);
        }

        currentPathBox.addSeparator();
        */
    }
    
    /**
      | Equivalent to pressing the "up" button
      | to browse the parent directory.
      |
      */
    pub fn go_up(&mut self)  {
        
        todo!();
        /*
            setRoot (getRoot().getParentDirectory());
        */
    }
    
    /**
      | Refreshes the directory that's currently
      | being listed.
      |
      */
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            fileList->refresh();
        */
    }
    
    /**
      | Changes the filter that's being used
      | to sift the files.
      |
      */
    pub fn set_file_filter(&mut self, new_file_filter: *const FileFilter)  {
        
        todo!();
        /*
            if (fileFilter != newFileFilter)
        {
            fileFilter = newFileFilter;
            refresh();
        }
        */
    }
    
    pub fn get_action_verb(&self) -> String {
        
        todo!();
        /*
            return isSaveMode() ? ((flags & canSelectDirectories) != 0 ? TRANS("Choose")
                                                                   : TRANS("Save"))
                            : TRANS("Open");
        */
    }
    
    /**
      | Sets the label that will be displayed
      | next to the filename entry box.
      | 
      | By default this is just "file", but you
      | might want to change it to something
      | more appropriate for your app.
      |
      */
    pub fn set_filename_box_label(&mut self, name: &String)  {
        
        todo!();
        /*
            fileLabel.setText (name, dontSendNotification);
        */
    }
    
    pub fn get_preview_component(&self) -> *mut FilePreviewComponent {
        
        todo!();
        /*
            return previewComp;
        */
    }
    
    pub fn get_display_component(&self) -> *mut DirectoryContentsDisplayComponent {
        
        todo!();
        /*
            return fileListComponent.get();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            getLookAndFeel()
            .layoutFileBrowserComponent (*this, fileListComponent.get(), previewComp,
                                         &currentPathBox, &filenameBox, goUpButton.get());
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            goUpButton.reset (getLookAndFeel().createFileBrowserGoUpButton());

        if (auto* buttonPtr = goUpButton.get())
        {
            addAndMakeVisible (*buttonPtr);
            buttonPtr->onClick = [this] { goUp(); };
            buttonPtr->setTooltip (TRANS ("Go up to parent directory"));
        }

        currentPathBox.setColour (ComboBox::backgroundColourId,    findColour (currentPathBoxBackgroundColourId));
        currentPathBox.setColour (ComboBox::textColourId,          findColour (currentPathBoxTextColourId));
        currentPathBox.setColour (ComboBox::arrowColourId,         findColour (currentPathBoxArrowColourId));

        filenameBox.setColour (TextEditor::backgroundColourId,     findColour (filenameBoxBackgroundColourId));
        filenameBox.applyColourToAllText (findColour (filenameBoxTextColourId));

        resized();
        repaint();
        */
    }
    
    pub fn send_listener_change_message(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);

        if (previewComp != nullptr)
            previewComp->selectedFileChanged (getSelectedFile (0));

        // You shouldn't delete the browser when the file gets changed!
        jassert (! checker.shouldBailOut());

        listeners.callChecked (checker, [] (FileBrowserListener& l) { l.selectionChanged(); });
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD || ALOE_WINDOWS
        if (key.getModifiers().isCommandDown()
             && (key.getKeyCode() == 'H' || key.getKeyCode() == 'h'))
        {
            fileList->setIgnoresHiddenFiles (! fileList->ignoresHiddenFiles());
            fileList->refresh();
            return true;
        }
       #endif

        ignoreUnused (key);
        return false;
        */
    }
    
    pub fn change_filename(&mut self)  {
        
        todo!();
        /*
            if (filenameBox.getText().containsChar (File::getSeparatorChar()))
        {
            auto f = currentRoot.getChildFile (filenameBox.getText());

            if (f.isDirectory())
            {
                setRoot (f);
                chosenFiles.clear();

                if ((flags & doNotClearFileNameOnRootChange) == 0)
                    filenameBox.setText ({});
            }
            else
            {
                setRoot (f.getParentDirectory());
                chosenFiles.clear();
                chosenFiles.add (f);
                filenameBox.setText (f.getFileName());
            }
        }
        else
        {
            fileDoubleClicked (getSelectedFile (0));
        }
        */
    }
    
    pub fn update_selected_path(&mut self)  {
        
        todo!();
        /*
            auto newText = currentPathBox.getText().trim().unquoted();

        if (newText.isNotEmpty())
        {
            auto index = currentPathBox.getSelectedId() - 1;

            Vec<String> rootNames, rootPaths;
            getRoots (rootNames, rootPaths);

            if (rootPaths[index].isNotEmpty())
            {
                setRoot (File (rootPaths[index]));
            }
            else
            {
                File f (newText);

                for (;;)
                {
                    if (f.isDirectory())
                    {
                        setRoot (f);
                        break;
                    }

                    if (f.getParentDirectory() == f)
                        break;

                    f = f.getParentDirectory();
                }
            }
        }
        */
    }
    
    /**
      | Returns a platform-specific list of
      | names and paths for some suggested places
      | the user might want to use as root folders.
      | 
      | The list returned contains empty strings
      | to indicate section breaks. @see getRoots()
      |
      */
    pub fn get_default_roots(&mut self, 
        root_names: &mut Vec<String>,
        root_paths: &mut Vec<String>)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
        Vec<File> roots;
        File::findFileSystemRoots (roots);
        rootPaths.clear();

        for (int i = 0; i < roots.size(); ++i)
        {
            const File& drive = roots.getReference(i);

            String name (drive.getFullPathName());
            rootPaths.add (name);

            if (drive.isOnHardDisk())
            {
                String volume (drive.getVolumeLabel());

                if (volume.isEmpty())
                    volume = TRANS("Hard Drive");

                name << " [" << volume << ']';
            }
            else if (drive.isOnCDRomDrive())
            {
                name << " [" << TRANS("CD/DVD drive") << ']';
            }

            rootNames.add (name);
        }

        rootPaths.add ({});
        rootNames.add ({});

        rootPaths.add (File::getSpecialLocation (File::userDocumentsDirectory).getFullPathName());
        rootNames.add (TRANS("Documents"));
        rootPaths.add (File::getSpecialLocation (File::userMusicDirectory).getFullPathName());
        rootNames.add (TRANS("Music"));
        rootPaths.add (File::getSpecialLocation (File::userPicturesDirectory).getFullPathName());
        rootNames.add (TRANS("Pictures"));
        rootPaths.add (File::getSpecialLocation (File::userDesktopDirectory).getFullPathName());
        rootNames.add (TRANS("Desktop"));

       #elif ALOE_MAC
        rootPaths.add (File::getSpecialLocation (File::userHomeDirectory).getFullPathName());
        rootNames.add (TRANS("Home folder"));
        rootPaths.add (File::getSpecialLocation (File::userDocumentsDirectory).getFullPathName());
        rootNames.add (TRANS("Documents"));
        rootPaths.add (File::getSpecialLocation (File::userMusicDirectory).getFullPathName());
        rootNames.add (TRANS("Music"));
        rootPaths.add (File::getSpecialLocation (File::userPicturesDirectory).getFullPathName());
        rootNames.add (TRANS("Pictures"));
        rootPaths.add (File::getSpecialLocation (File::userDesktopDirectory).getFullPathName());
        rootNames.add (TRANS("Desktop"));

        rootPaths.add ({});
        rootNames.add ({});

        for (auto& volume : File ("/Volumes").findChildFiles (File::findDirectories, false))
        {
            if (volume.isDirectory() && ! volume.getFileName().startsWithChar ('.'))
            {
                rootPaths.add (volume.getFullPathName());
                rootNames.add (volume.getFileName());
            }
        }

       #else
        rootPaths.add ("/");
        rootNames.add ("/");
        rootPaths.add (File::getSpecialLocation (File::userHomeDirectory).getFullPathName());
        rootNames.add (TRANS("Home folder"));
        rootPaths.add (File::getSpecialLocation (File::userDesktopDirectory).getFullPathName());
        rootNames.add (TRANS("Desktop"));
       #endif
        */
    }
    
    pub fn get_roots(&mut self, 
        root_names: &mut Vec<String>,
        root_paths: &mut Vec<String>)  {
        
        todo!();
        /*
            getDefaultRoots (rootNames, rootPaths);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            const auto isProcessActive = isForegroundOrEmbeddedProcess (this);

        if (wasProcessActive != isProcessActive)
        {
            wasProcessActive = isProcessActive;

            if (isProcessActive && fileList != nullptr)
                refresh();
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
