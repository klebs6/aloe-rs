crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileChooserDialogBox.h]

/**
  | A file open/save dialog box.
  | 
  | This is a Aloe-based file dialog box;
  | to use a native file chooser, see the
  | 
  | FileChooser class.
  | 
  | -----------
  | @code
  | 
  | {
  |     wildcardFilter = std::make_unique<WildcardFileFilter> ("*.foo", String(), "Foo files");
  | 
  |     browser = std::make_unique<FileBrowserComponent> (FileBrowserComponent::canSelectFiles,
  |                                                       File(),
  |                                                       wildcardFilter.get(),
  |                                                       nullptr);
  | 
  |     dialogBox = std::make_unique<FileChooserDialogBox> ("Open some kind of file",
  |                                                         "Please choose some kind of file that you want to open...",
  |                                                         browser,
  |                                                         false,
  |                                                         Colours::lightgrey);
  | 
  |     auto onFileSelected = [this] (int r)
  |     {
  |         modalStateFinished (r);
  | 
  |         auto selectedFile = browser->getSelectedFile (0);
  | 
  |         ...etc...
  |     };
  | 
  |     dialogBox->centreWithDefaultSize (nullptr);
  |     dialogBox->enterModalState (true,
  |                                 ModalCallbackFunction::create (onFileSelected),
  |                                 true);
  | }
  | 
  | @see FileChooser
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileChooserDialogBox<'a> {
    base:                                  ResizableWindow<'a>,
    content:                               *mut FileChooserDialogBoxContentComponent<'a>,
    warn_about_overwriting_existing_files: bool,
}

impl<'a> FileBrowserListener for FileChooserDialogBox<'a> {

    fn selection_changed(&mut self)  {
        
        todo!();
        /*
            content->okButton.setEnabled (content->chooserComponent.currentFileIsValid());

        content->newFolderButton.setVisible (content->chooserComponent.isSaveMode()
                                              && content->chooserComponent.getRoot().isDirectory());
        */
    }

    fn file_double_clicked(&mut self, _0: &File)  {
        
        todo!();
        /*
            selectionChanged();
        content->okButton.triggerClick();
        */
    }
    
    fn file_clicked(&mut self, 
        _0: &File,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }

    fn browser_root_changed(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for FileChooserDialogBox<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        content->chooserComponent.removeListener (this);
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileChooserDialogBox.cpp]

impl<'a> FileChooserDialogBox<'a> {

    /**
      | Creates a file chooser box.
      | 
      | -----------
      | @param title
      | 
      | the main title to show at the top of the
      | box
      | ----------
      | @param instructions
      | 
      | an optional longer piece of text to show
      | below the title in a smaller font, describing
      | in more detail what's required.
      | ----------
      | @param browserComponent
      | 
      | a FileBrowserComponent that will be
      | shown inside this dialog box. Make sure
      | you delete this after (but not before!)
      | the dialog box has been deleted.
      | ----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true, then the user will be asked to
      | confirm if they try to select a file that
      | already exists. (This flag is only used
      | when saving files)
      | ----------
      | @param backgroundColour
      | 
      | the background colour for the top level
      | window
      | ----------
      | @param parentComponent
      | 
      | an optional component which should
      | be the parent for the file chooser. If
      | this is a nullptr then the dialog box
      | will be a top-level window. AUv3s on
      | iOS must specify this parameter as opening
      | a top-level window in an AUv3 is forbidden
      | due to sandbox restrictions.
      | 
      | @see FileBrowserComponent, FilePreviewComponent
      |
      */
    pub fn new(
        name:              &String,
        instructions:      &String,
        chooser_component: &mut FileBrowserComponent,
        should_warn:       bool,
        background_colour: Colour,
        parent_comp:       *mut Component) -> Self {
    
        todo!();
        /*


            : ResizableWindow (name, backgroundColour, parentComp == nullptr),
          warnAboutOverwritingExistingFiles (shouldWarn)

        content = new FileChooserDialogBoxContentComponent (name, instructions, chooserComponent);
        setContentOwned (content, false);

        setResizable (true, true);
        setResizeLimits (300, 300, 1200, 1000);

        content->okButton.onClick        = [this] { okButtonPressed(); };
        content->cancelButton.onClick    = [this] { closeButtonPressed(); };
        content->newFolderButton.onClick = [this] { createNewFolder(); };

        content->chooserComponent.addListener (this);

        FileChooserDialogBox::selectionChanged();

        if (parentComp != nullptr)
            parentComp->addAndMakeVisible (this);
        else
            setAlwaysOnTop (aloe_areThereAnyAlwaysOnTopWindows());
        */
    }

    /**
      | Displays and runs the dialog box modally.
      | 
      | This will show the box with the specified
      | size, returning true if the user pressed
      | 'ok', or false if they cancelled.
      | 
      | Leave the width or height as 0 to use the
      | default size
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show(&mut self, w: i32, h: i32) -> bool {

        let width:  i32 = width.unwrap_or(0);
        let height: i32 = height.unwrap_or(0);
        
        todo!();
        /*
            return showAt (-1, -1, w, h);
        */
    }
    
    /**
      | Displays and runs the dialog box modally.
      | 
      | This will show the box with the specified
      | size at the specified location, returning
      | true if the user pressed 'ok', or false
      | if they cancelled.
      | 
      | Leave the width or height as 0 to use the
      | default size.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_at(&mut self, 
        x: i32,
        y: i32,
        w: i32,
        h: i32) -> bool {
        
        todo!();
        /*
            if (w <= 0)  w = getDefaultWidth();
        if (h <= 0)  h = 500;

        if (x < 0 || y < 0)
            centreWithSize (w, h);
        else
            setBounds (x, y, w, h);

        const bool ok = (runModalLoop() != 0);
        setVisible (false);
        return ok;
        */
    }
    
    /**
      | Sets the size of this dialog box to its
      | default and positions it either in the
      | centre of the screen, or centred around
      | a component that is provided.
      |
      */
    pub fn centre_with_default_size(&mut self, component_to_centre_around: *mut Component)  {
        
        todo!();
        /*
            centreAroundComponent (componentToCentreAround, getDefaultWidth(), 500);
        */
    }
    
    pub fn get_default_width(&self) -> i32 {
        
        todo!();
        /*
            if (auto* previewComp = content->chooserComponent.getPreviewComponent())
            return 400 + previewComp->getWidth();

        return 600;
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            setVisible (false);
        */
    }
    
    pub fn ok_to_overwrite_file_callback(&mut self, 
        result: i32,
        box_:   *mut FileChooserDialogBox)  {
        
        todo!();
        /*
            if (result != 0 && box != nullptr)
            box->exitModalState (1);
        */
    }
    
    pub fn ok_button_pressed(&mut self)  {
        
        todo!();
        /*
            if (warnAboutOverwritingExistingFiles
             && content->chooserComponent.isSaveMode()
             && content->chooserComponent.getSelectedFile(0).exists())
        {
            AlertWindow::showOkCancelBox (MessageBoxIconType::WarningIcon,
                                          TRANS("File already exists"),
                                          TRANS("There's already a file called: FLNM")
                                             .replace ("FLNM", content->chooserComponent.getSelectedFile(0).getFullPathName())
                                            + "\n\n"
                                            + TRANS("Are you sure you want to overwrite it?"),
                                          TRANS("Overwrite"),
                                          TRANS("Cancel"),
                                          this,
                                          ModalCallbackFunction::forComponent (okToOverwriteFileCallback, this));
        }
        else
        {
            exitModalState (1);
        }
        */
    }
    
    pub fn create_new_folder_callback(&mut self, 
        result: i32,
        box_:   *mut FileChooserDialogBox,
        alert:  ComponentSafePointer<AlertWindow>)  {
        
        todo!();
        /*
            if (result != 0 && alert != nullptr && box != nullptr)
        {
            alert->setVisible (false);
            box->createNewFolderConfirmed (alert->getTextEditorContents ("Folder Name"));
        }
        */
    }
    
    pub fn create_new_folder(&mut self)  {
        
        todo!();
        /*
            auto parent = content->chooserComponent.getRoot();

        if (parent.isDirectory())
        {
            auto* aw = new AlertWindow (TRANS("New Folder"),
                                        TRANS("Please enter the name for the folder"),
                                        MessageBoxIconType::NoIcon, this);

            aw->addTextEditor ("Folder Name", String(), String(), false);
            aw->addButton (TRANS("Create Folder"), 1, KeyPress (KeyPress::returnKey));
            aw->addButton (TRANS("Cancel"),        0, KeyPress (KeyPress::escapeKey));

            aw->enterModalState (true,
                                 ModalCallbackFunction::forComponent (createNewFolderCallback, this,
                                                                      Component::SafePointer<AlertWindow> (aw)),
                                 true);
        }
        */
    }
    
    pub fn create_new_folder_confirmed(&mut self, name_from_dialog: &String)  {
        
        todo!();
        /*
            auto name = File::createLegalFileName (nameFromDialog);

        if (! name.isEmpty())
        {
            auto parent = content->chooserComponent.getRoot();

            if (! parent.getChildFile (name).createDirectory())
                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  TRANS ("New Folder"),
                                                  TRANS ("Couldn't create the folder!"));

            content->chooserComponent.refresh();
        }
        */
    }
}
