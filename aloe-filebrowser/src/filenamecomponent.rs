crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FilenameComponent.h]

/**
  | Shows a filename as an editable text
  | box, with a 'browse' button and a drop-down
  | list for recently selected files.
  | 
  | A handy component for dialogue boxes
  | where you want the user to be able to select
  | a file or directory.
  | 
  | Attach an FilenameComponentListener
  | using the addListener() method, and
  | it will get called each time the user
  | changes the filename, either by browsing
  | for a file and clicking 'ok', or by typing
  | a new filename into the box and pressing
  | return.
  | 
  | @see FileChooser, ComboBox
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FilenameComponent<'a> {
    base:                Component<'a>,
    base2:               SettableTooltipClient,
    base4:               AsyncUpdater<'a>,
    filename_box:        ComboBox<'a>,
    last_filename:       String,
    browse_button:       Box<Button<'a>>,
    max_recent_files:    i32, // default = 30
    is_dir:              bool,
    is_saving:           bool,
    is_file_drag_over:   bool, // default = false
    wildcard:            String,
    enforced_suffix:     String,
    browse_button_text:  String,
    listeners:           ListenerList<Rc<RefCell<dyn FilenameComponentListener>>>,
    default_browse_file: File,
    chooser:             Box<FileChooser<'a>>,
}

impl<'a> FileDragAndDropTarget for FileListComponent<'a> {

}

impl<'a> IsInterestedInDragSource for FileListComponent<'a> {

    fn is_interested_in_drag_source(
        &mut self, 
        _: &DragAndDropTargetSourceDetails<'_>

    ) -> bool { 
        todo!() 
    }
}

impl<'a> ItemDragEnter for FileListComponent<'a> {

}

impl<'a> ItemDragExit for FileListComponent<'a> {

}

impl<'a> ItemDragMove for FileListComponent<'a> {

}

impl<'a> ItemDropped for FileListComponent<'a> {

    fn item_dropped(
        &mut self, 
        _: &DragAndDropTargetSourceDetails<'_>
    ) 
    { 
        todo!() 
    }
}

impl<'a> ShouldDrawDragImageWhenOver for FileListComponent<'a> {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FilenameComponent.cpp]
impl<'a> FilenameComponent<'a> {

    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool { true }

     /**
      | Creates a FilenameComponent.
      | 
      | -----------
      | @param name
      | 
      | the name for this component.
      | ----------
      | @param currentFile
      | 
      | the file to initially show in the box
      | ----------
      | @param canEditFilename
      | 
      | if true, the user can manually edit the
      | filename; if false, they can only change
      | it by browsing for a new file
      | ----------
      | @param isDirectory
      | 
      | if true, the file will be treated as a
      | directory, and an appropriate directory
      | browser used
      | ----------
      | @param isForSaving
      | 
      | if true, the file browser will allow
      | non-existent files to be picked, as
      | the file is assumed to be used for saving
      | rather than loading
      | ----------
      | @param fileBrowserWildcard
      | 
      | a wildcard pattern to use in the file
      | browser - e.g. "*.txt;*.foo".
      | 
      | If an empty string is passed in, then
      | the pattern is assumed to be "*"
      | ----------
      | @param enforcedSuffix
      | 
      | if this is non-empty, it is treated as
      | a suffix that will be added to any filenames
      | that are entered or chosen
      | ----------
      | @param textWhenNothingSelected
      | 
      | the message to display in the box before
      | any filename is entered. (This will
      | only appear if the initial file isn't
      | valid)
      |
      */
    pub fn new(
        name:                       &String,
        current_file:               &File,
        can_edit_filename:          bool,
        is_directory:               bool,
        is_for_saving:              bool,
        file_browser_wildcard:      &String,
        suffix:                     &String,
        text_when_nothing_selected: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : is_dir(isDirectory),
        : is_saving(isForSaving),
        : wildcard(fileBrowserWildcard),
        : enforced_suffix(suffix),

            addAndMakeVisible (filenameBox);
        filenameBox.setEditableText (canEditFilename);
        filenameBox.setTextWhenNothingSelected (textWhenNothingSelected);
        filenameBox.setTextWhenNoChoicesAvailable (TRANS ("(no recently selected files)"));
        filenameBox.onChange = [this] { setCurrentFile (getCurrentFile(), true); };

        setBrowseButtonText ("...");

        setCurrentFile (currentFile, true, dontSendNotification);
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isFileDragOver)
        {
            g.setColour (Colours::red.withAlpha (0.2f));
            g.drawRect (getLocalBounds(), 3);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            getLookAndFeel().layoutFilenameComponent (*this, &filenameBox, browseButton.get());
        */
    }
    
    pub fn create_keyboard_focus_traverser(&mut self) -> Box<dyn ComponentTraverser> {
        
        todo!();
        /*
            // This prevents the sub-components from grabbing focus if the
        // FilenameComponent has been set to refuse focus.
        return getWantsKeyboardFocus() ? Component::createKeyboardFocusTraverser() : nullptr;
        */
    }
    
    /**
      | Changes the text shown on the 'browse'
      | button.
      | 
      | By default this button just says "..."
      | but you can change it. The button itself
      | can be changed using the look-and-feel
      | classes, so it might not actually have
      | any text on it.
      |
      */
    pub fn set_browse_button_text(&mut self, new_browse_button_text: &String)  {
        
        todo!();
        /*
            browseButtonText = newBrowseButtonText;
        lookAndFeelChanged();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            browseButton.reset();
        browseButton.reset (getLookAndFeel().createFilenameComponentBrowseButton (browseButtonText));
        addAndMakeVisible (browseButton.get());
        browseButton->setConnectedEdges (Button::ConnectedOnLeft);
        browseButton->onClick = [this] { showChooser(); };
        resized();
        */
    }
    
    /**
      | Gives the component a tooltip.
      |
      */
    pub fn set_tooltip(&mut self, new_tooltip: &String)  {
        
        todo!();
        /*
            SettableTooltipClient::setTooltip (newTooltip);
        filenameBox.setTooltip (newTooltip);
        */
    }
    
    /**
      | Sets a file or directory to be the default
      | starting point for the browser to show.
      | 
      | This is only used if the current file
      | hasn't been set.
      |
      */
    pub fn set_default_browse_target(&mut self, new_default_directory: &File)  {
        
        todo!();
        /*
            defaultBrowseFile = newDefaultDirectory;
        */
    }
    
    pub fn get_location_to_browse(&mut self) -> File {
        
        todo!();
        /*
            if (lastFilename.isEmpty() && defaultBrowseFile != File())
            return defaultBrowseFile;

        return getCurrentFile();
        */
    }
    
    pub fn show_chooser(&mut self)  {
        
        todo!();
        /*
            chooser = std::make_unique<FileChooser> (isDir ? TRANS ("Choose a new directory")
                                                       : TRANS ("Choose a new file"),
                                                 getLocationToBrowse(),
                                                 wildcard);

        auto chooserFlags = isDir ? FileBrowserComponent::openMode | FileBrowserComponent::canSelectDirectories
                                  : FileBrowserComponent::canSelectFiles | (isSaving ? FileBrowserComponent::saveMode
                                                                                     : FileBrowserComponent::openMode);

        chooser->launchAsync (chooserFlags, [this] (const FileChooser&)
        {
            if (chooser->getResult() == File{})
                return;

            setCurrentFile (chooser->getResult(), true);
        });
        */
    }
    
    pub fn files_dropped(&mut self, 
        filenames: &Vec<String>,
        _1:        i32,
        _2:        i32)  {
        
        todo!();
        /*
            isFileDragOver = false;
        repaint();

        const File f (filenames[0]);

        if (f.exists() && (f.isDirectory() == isDir))
            setCurrentFile (f, true);
        */
    }
    
    pub fn file_drag_enter(&mut self, 
        _0: &Vec<String>,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
            isFileDragOver = true;
        repaint();
        */
    }
    
    pub fn file_drag_exit(&mut self, _0: &Vec<String>)  {
        
        todo!();
        /*
            isFileDragOver = false;
        repaint();
        */
    }
    
    /**
      | Returns the raw text that the user has
      | entered.
      |
      */
    pub fn get_current_file_text(&self) -> String {
        
        todo!();
        /*
            return filenameBox.getText();
        */
    }
    
    /**
      | Returns the currently displayed filename.
      |
      */
    pub fn get_current_file(&self) -> File {
        
        todo!();
        /*
            auto f = File::getCurrentWorkingDirectory().getChildFile (getCurrentFileText());

        if (enforcedSuffix.isNotEmpty())
            f = f.withFileExtension (enforcedSuffix);

        return f;
        */
    }
    
    /**
      | Changes the current filename.
      | 
      | -----------
      | @param newFile
      | 
      | the new filename to use
      | ----------
      | @param addToRecentlyUsedList
      | 
      | if true, the filename will also be added
      | to the drop-down list of recent files.
      | ----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | filename has changed.
      |
      */
    pub fn set_current_file(
        &mut self, 
        new_file:                  File,
        add_to_recently_used_list: bool,
        notification:              Option<NotificationType>

    ) {

        let notification = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            if (enforcedSuffix.isNotEmpty())
            newFile = newFile.withFileExtension (enforcedSuffix);

        if (newFile.getFullPathName() != lastFilename)
        {
            lastFilename = newFile.getFullPathName();

            if (addToRecentlyUsedList)
                addRecentlyUsedFile (newFile);

            filenameBox.setText (lastFilename, dontSendNotification);

            if (notification != dontSendNotification)
            {
                triggerAsyncUpdate();

                if (notification == sendNotificationSync)
                    handleUpdateNowIfNeeded();
            }
        }
        */
    }
    
    /**
      | Changes whether the use can type into
      | the filename box.
      |
      */
    pub fn set_filename_is_editable(&mut self, should_be_editable: bool)  {
        
        todo!();
        /*
            filenameBox.setEditableText (shouldBeEditable);
        */
    }
    
    /**
      | Returns all the entries on the recent
      | files list.
      | 
      | This can be used in conjunction with
      | setRecentlyUsedFilenames() for saving
      | the state of this list.
      | 
      | @see setRecentlyUsedFilenames
      |
      */
    pub fn get_recently_used_filenames(&self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> names;

        for (int i = 0; i < filenameBox.getNumItems(); ++i)
            names.add (filenameBox.getItemText (i));

        return names;
        */
    }
    
    /**
      | Sets all the entries on the recent files
      | list.
      | 
      | This can be used in conjunction with
      | getRecentlyUsedFilenames() for saving
      | the state of this list.
      | 
      | @see getRecentlyUsedFilenames, addRecentlyUsedFile
      |
      */
    pub fn set_recently_used_filenames(&mut self, filenames: &Vec<String>)  {
        
        todo!();
        /*
            if (filenames != getRecentlyUsedFilenames())
        {
            filenameBox.clear();

            for (int i = 0; i < jmin (filenames.size(), maxRecentFiles); ++i)
                filenameBox.addItem (filenames[i], i + 1);
        }
        */
    }
    
    /**
      | Changes the limit for the number of files
      | that will be stored in the recent-file
      | list.
      |
      */
    pub fn set_max_number_of_recent_files(&mut self, new_maximum: i32)  {
        
        todo!();
        /*
            maxRecentFiles = jmax (1, newMaximum);

        setRecentlyUsedFilenames (getRecentlyUsedFilenames());
        */
    }
    
    /**
      | Adds an entry to the recently-used files
      | dropdown list.
      | 
      | If the file is already in the list, it
      | will be moved to the top. A limit is also
      | placed on the number of items that are
      | kept in the list.
      | 
      | @see getRecentlyUsedFilenames, setRecentlyUsedFilenames,
      | setMaxNumberOfRecentFiles
      |
      */
    pub fn add_recently_used_file(&mut self, file: &File)  {
        
        todo!();
        /*
            auto files = getRecentlyUsedFilenames();

        if (file.getFullPathName().isNotEmpty())
        {
            files.removeString (file.getFullPathName(), true);
            files.insert (0, file.getFullPathName());

            setRecentlyUsedFilenames (files);
        }
        */
    }
    
    /**
      | Adds a listener that will be called when
      | the selected file is changed.
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn FilenameComponentListener)  {
        
        todo!();
        /*
            listeners.add (listener);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn FilenameComponentListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [this] (FilenameComponentListener& l) { l.filenameComponentChanged (this); });
        */
    }
}
