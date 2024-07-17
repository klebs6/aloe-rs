crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FileChooserNonNative<'a> {
    owner:                &'a mut FileChooser<'a>,
    selects_directories:  bool,
    selects_files:        bool,
    warn_about_overwrite: bool,
    filter:               WildcardFileFilter,
    browser_component:    FileBrowserComponent<'a>,
    dialog_box:           FileChooserDialogBox<'a>,
}

impl<'a> FileChooserPimpl for FileChooserNonNative<'a> {

    fn launch(&mut self)  {
        
        todo!();
        /*
            dialogBox.centreWithDefaultSize (nullptr);
            dialogBox.enterModalState (true, ModalCallbackFunction::create ([this] (int r) { modalStateFinished (r); }), true);
        */
    }
    
    fn run_modally(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MODAL_LOOPS_PERMITTED
            modalStateFinished (dialogBox.show() ? 1 : 0);
           #else
            jassertfalse;
           #endif
        */
    }
}

impl<'a> Drop for FileChooserNonNative<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            dialogBox.exitModalState (0);
         */
    }
}

impl<'a> FileChooserNonNative<'a> {
    
    pub fn new(
        file_chooser: &mut FileChooser,
        flags:        i32,
        preview:      *mut FilePreviewComponent) -> Self {
    
        todo!();
        /*


            : owner (fileChooser),
              selectsDirectories ((flags & FileBrowserComponent::canSelectDirectories)   != 0),
              selectsFiles       ((flags & FileBrowserComponent::canSelectFiles)         != 0),
              warnAboutOverwrite ((flags & FileBrowserComponent::warnAboutOverwriting)   != 0),

              filter (selectsFiles ? owner.filters : String(), selectsDirectories ? "*" : String(), {}),
              browserComponent (flags, owner.startingFile, &filter, preview),
              dialogBox (owner.title, {}, browserComponent, warnAboutOverwrite,
                         browserComponent.findColour (AlertWindow::backgroundColourId), owner.parent)
        */
    }
    
    pub fn modal_state_finished(&mut self, return_value: i32)  {
        
        todo!();
        /*
            Vec<Url> result;

            if (returnValue != 0)
            {
                for (int i = 0; i < browserComponent.getNumSelectedFiles(); ++i)
                    result.add (Url (browserComponent.getSelectedFile (i)));
            }

            owner.finished (result);
        */
    }
}
