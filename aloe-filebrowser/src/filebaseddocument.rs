crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/documents/aloe_FileBasedDocument.h]
pub trait FileBasedDocumentInterface:
    GetDocumentTitle
    + LoadDocument
    + LoadDocumentAsync
    + SaveDocument
    + SaveDocumentAsync
    + GetLastDocumentOpened
    + SetLastDocumentOpened
    + GetSuggestedSaveAsFile
    + Changed {}

/**
  | A set of possible outcomes of one of the
  | save() methods
  |
  */
pub enum FileBasedDocumentSaveResult
{
    /**
      | indicates that a file was saved successfully.
      |
      */
    savedOk = 0,            

    /**
      | indicates that the user aborted the
      | save operation.
      |
      */
    userCancelledSave,      

    /**
      | indicates that it tried to write to a
      | file but this failed.
      |
      */
    failedToWriteToFile,     
}

/**
  | A class to take care of the logic involved
  | with the loading/saving of some kind
  | of document.
  | 
  | There's quite a lot of tedious logic
  | involved in writing all the load/save/save-as
  | functions you need for documents that
  | get saved to a file, so this class attempts
  | to abstract most of the boring stuff.
  | 
  | Your subclass should just implement
  | all the pure virtual methods, and you
  | can then use the higher-level public
  | methods to do the load/save dialogs,
  | to warn the user about overwriting files,
  | etc.
  | 
  | The document object keeps track of whether
  | it has changed since it was last saved
  | or loaded, so when you change something,
  | call its changed() method. This will
  | set a flag so it knows it needs saving,
  | and will also broadcast a change message
  | using the
  | 
  | ChangeBroadcaster base class.
  | 
  | @see ChangeBroadcaster
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileBasedDocument<'a> {
    base:  ChangeBroadcaster<'a>,
    impl_: Box<FileBasedDocumentImpl<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/documents/aloe_FileBasedDocument.cpp]
impl<'a> FileBasedDocument<'a> {

    /**
      | Creates a FileBasedDocument.
      | 
      | -----------
      | @param fileExtension
      | 
      | the extension to use when loading/saving
      | files, e.g. ".doc"
      | ----------
      | @param fileWildCard
      | 
      | the wildcard to use in file dialogs,
      | e.g. "*.doc"
      | ----------
      | @param openFileDialogTitle
      | 
      | the title to show on an open-file dialog,
      | e.g. "Choose a file to open.."
      | ----------
      | @param saveFileDialogTitle
      | 
      | the title to show on an save-file dialog,
      | e.g. "Choose a file to save as.."
      |
      */
    pub fn new(
        file_extension:         &String,
        file_wildcard:          &String,
        open_file_dialog_title: &String,
        save_file_dialog_title: &String) -> Self {
    
        todo!();
        /*


            : impl (new FileBasedDocumentImpl (*this,
                            fileExtension,
                            fileWildcard,
                            openFileDialogTitle,
                            saveFileDialogTitle))
        */
    }
    
    /**
      | Returns true if the changed() method
      | has been called since the file was last
      | saved or loaded.
      | 
      | @see setChangedFlag, changed
      |
      */
    pub fn has_changed_since_saved(&self) -> bool {
        
        todo!();
        /*
            return impl->hasChangedSinceSaved();
        */
    }
    
    /**
      | Sets the state of the 'changed' flag.
      | 
      | The 'changed' flag is set to true when
      | the changed() method is called - use
      | this method to reset it or to set it without
      | also broadcasting a change message.
      | 
      | @see changed, hasChangedSinceSaved
      |
      */
    pub fn set_changed_flag(&mut self, has_changed: bool)  {
        
        todo!();
        /*
            impl->setChangedFlag (hasChanged);
        */
    }
    
    pub fn changed(&mut self)  {
        
        todo!();
        /*
            impl->changed();
        */
    }
    
    /**
      | Tries to open a file.
      | 
      | If the file opens correctly the document's
      | file (see the getFile() method) is set
      | to this new one; if it fails, the document's
      | file is left unchanged, and optionally
      | a message box is shown telling the user
      | there was an error.
      | 
      | 
      | -----------
      | @return
      | 
      | A result indicating whether the new
      | file loaded successfully, or the error
      | message if it failed. @see loadDocument,
      | loadFromUserSpecifiedFile
      |
      */
    pub fn load_from(
        &mut self, 
        file_to_load_from:       &File,
        show_message_on_failure: bool,
        show_wait_cursor:        Option<bool>

    ) -> Result<(),()> {

        let show_wait_cursor: bool = show_wait_cursor.unwrap_or(true);
        
        todo!();
        /*
            return impl->loadFrom (fileToLoadFrom, showMessageOnFailure, showWaitCursor);
        */
    }
    
    /**
      | Tries to open a file.
      | 
      | The callback is called with the result
      | indicating whether the new file loaded
      | successfully, or the error message
      | if it failed.
      | 
      | If the file opens correctly the document's
      | file (see the getFile() method) is set
      | to this new one; if it fails, the document's
      | file is left unchanged, and optionally
      | a message box is shown telling the user
      | there was an error.
      | 
      | @see loadDocumentAsync, loadFromUserSpecifiedFileAsync
      |
      */
    pub fn load_from_async(
        &mut self, 
        file_to_load_from:       &File,
        show_message_on_failure: bool,
        callback:                fn(_0: Result<(),()>) -> ()

    ) {
        
        todo!();
        /*
            impl->loadFromAsync (fileToLoadFrom, showMessageOnFailure, std::move (callback));
        */
    }

    /**
      | Asks the user for a file and tries to load
      | it.
      | 
      | This will pop up a dialog box using the
      | title, file extension and wildcard
      | specified in the document's constructor,
      | and asks the user for a file. If they pick
      | one, the loadFrom() method is used to
      | try to load it, optionally showing a
      | message if it fails.
      | 
      | 
      | -----------
      | @return
      | 
      | a result indicating success; This will
      | be a failure message if the user cancelled
      | or if they picked a file which failed
      | to load correctly @see loadFrom
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn load_from_user_specified_file(
        &mut self, 
        show_message_on_failure: bool
    ) 
        -> Result<(),()> 
    {
        todo!();
        /*
            return impl->loadFromUserSpecifiedFile (showMessageOnFailure);
        */
    }
    
    /**
      | Asks the user for a file and tries to load
      | it.
      | 
      | This will pop up a dialog box using the
      | title, file extension and wildcard
      | specified in the document's constructor,
      | and asks the user for a file. If they pick
      | one, the loadFrom() method is used to
      | try to load it, optionally showing a
      | message if it fails. The result of the
      | operation is provided in the callback
      | function.
      | 
      | @see loadFrom
      |
      */
    pub fn load_from_user_specified_file_async(
        &mut self, 
        show_message_on_failure: bool,
        callback:                fn(_0: Result<(),()>) -> ()

    ) {
        
        todo!();
        /*
            impl->loadFromUserSpecifiedFileAsync (showMessageOnFailure, std::move (callback));
        */
    }

    /**
      | Tries to save the document to the last
      | file it was saved or loaded from.
      | 
      | This will always try to write to the file,
      | even if the document isn't flagged as
      | having changed.
      | 
      | -----------
      | @param askUserForFileIfNotSpecified
      | 
      | if there's no file currently specified
      | and this is true, it will prompt the user
      | to pick a file, as if saveAsInteractive()
      | was called.
      | ----------
      | @param showMessageOnFailure
      | 
      | if true it will show a warning message
      | when if the save operation fails @see
      | saveIfNeededAndUserAgrees, saveAs,
      | saveAsInteractive
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save(
        &mut self, 
        ask_user_for_file_if_not_specified: bool,
        show_message_on_failure:            bool

    ) -> FileBasedDocumentSaveResult {
        
        todo!();
        /*
            return impl->save (askUserForFileIfNotSpecified, showMessageOnFailure);
        */
    }
    
    /**
      | Tries to save the document to the last
      | file it was saved or loaded from.
      | 
      | This will always try to write to the file,
      | even if the document isn't flagged as
      | having changed.
      | 
      | -----------
      | @param askUserForFileIfNotSpecified
      | 
      | if there's no file currently specified
      | and this is true, it will prompt the user
      | to pick a file, as if saveAsInteractive()
      | was called.
      | ----------
      | @param showMessageOnFailure
      | 
      | if true it will show a warning message
      | when if the save operation fails
      | ----------
      | @param callback
      | 
      | called after the save operation with
      | the result @see saveIfNeededAndUserAgrees,
      | saveAs, saveAsInteractive
      |
      */
    pub fn save_async(
        &mut self, 
        ask_user_for_file_if_not_specified: bool,
        show_message_on_failure:            bool,
        callback:                           fn(_0: FileBasedDocumentSaveResult) -> ()

    ) {
        
        todo!();
        /*
            impl->saveAsync (askUserForFileIfNotSpecified, showMessageOnFailure, std::move (callback));
        */
    }

    /**
      | If the file needs saving, it'll ask the
      | user if that's what they want to do, and
      | save it if they say yes.
      | 
      | If you've got a document open and want
      | to close it (e.g. to quit the app), this
      | is the method to call.
      | 
      | If the document doesn't need saving
      | it'll return the value savedOk so you
      | can go ahead and delete the document.
      | 
      | If it does need saving it'll prompt the
      | user, and if they say "discard changes"
      | it'll return savedOk, so again, you
      | can safely delete the document.
      | 
      | If the user clicks "cancel", it'll return
      | userCancelledSave, so if you can abort
      | the close-document operation.
      | 
      | And if they click "save changes", it'll
      | try to save and either return savedOk,
      | or failedToWriteToFile if there was
      | a problem.
      | 
      | @see save, saveAs, saveAsInteractive
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_if_needed_and_user_agrees(&mut self) 
        -> FileBasedDocumentSaveResult 
    {
        
        todo!();
        /*
            return impl->saveIfNeededAndUserAgrees();
        */
    }
    
    /**
      | If the file needs saving, it'll ask the
      | user if that's what they want to do, and
      | save it if they say yes.
      | 
      | If you've got a document open and want
      | to close it (e.g. to quit the app), this
      | is the method to call.
      | 
      | If the document doesn't need saving
      | the callback will be called with the
      | value savedOk so you can go ahead and
      | delete the document.
      | 
      | If it does need saving it'll prompt the
      | user, and if they say "discard changes"
      | the callback will be called with savedOk,
      | so again, you can safely delete the document.
      | 
      | If the user clicks "cancel", the callback
      | will be aclled with userCancelledSave,
      | so you can abort the close-document
      | operation.
      | 
      | And if they click "save changes", it'll
      | try to save and the callback will be called
      | with either savedOk, or failedToWriteToFile
      | if there was a problem.
      | 
      | @see saveAsync, saveAsAsync, saveAsInteractiveAsync
      |
      */
    pub fn save_if_needed_and_user_agrees_async(
        &mut self, 
        callback: fn(_0: FileBasedDocumentSaveResult) -> ()

    ) {
        
        todo!();
        /*
            impl->saveIfNeededAndUserAgreesAsync (std::move (callback));
        */
    }

    /**
      | Tries to save the document to a specified
      | file.
      | 
      | If this succeeds, it'll also change
      | the document's internal file (as returned
      | by the getFile() method). If it fails,
      | the file will be left unchanged.
      | 
      | -----------
      | @param newFile
      | 
      | the file to try to write to
      | ----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true and the file exists, it'll ask
      | the user first if they want to overwrite
      | it
      | ----------
      | @param askUserForFileIfNotSpecified
      | 
      | if the file is non-existent and this
      | is true, it'll use the saveAsInteractive()
      | method to ask the user for a filename
      | ----------
      | @param showMessageOnFailure
      | 
      | if true and the write operation fails,
      | it'll show a message box to warn the user
      | ----------
      | @param showWaitCursor
      | 
      | if true, the 'wait' mouse cursor will
      | be showin during saving @see saveIfNeededAndUserAgrees,
      | save, saveAsInteractive
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as(
        &mut self, 
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        show_wait_cursor:                      bool

    ) -> FileBasedDocumentSaveResult {

        let show_wait_cursor: bool = show_wait_cursor.unwrap_or(true);
        
        todo!();
        /*
            return impl->saveAs (newFile,
                              warnAboutOverwritingExistingFiles,
                              askUserForFileIfNotSpecified,
                              showMessageOnFailure,
                              showWaitCursor);
        */
    }
    
    /**
      | Tries to save the document to a specified
      | file.
      | 
      | If this succeeds, it'll also change
      | the document's internal file (as returned
      | by the getFile() method). If it fails,
      | the file will be left unchanged.
      | 
      | -----------
      | @param newFile
      | 
      | the file to try to write to
      | ----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true and the file exists, it'll ask
      | the user first if they want to overwrite
      | it
      | ----------
      | @param askUserForFileIfNotSpecified
      | 
      | if the file is non-existent and this
      | is true, it'll use the saveAsInteractive()
      | method to ask the user for a filename
      | ----------
      | @param showMessageOnFailure
      | 
      | if true and the write operation fails,
      | it'll show a message box to warn the user
      | ----------
      | @param callback
      | 
      | called with the result of the save operation
      | 
      | @see saveIfNeededAndUserAgreesAsync,
      | saveAsync, saveAsInteractiveAsync
      |
      */
    pub fn save_as_async(
        &mut self, 
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ()

    ) {
        
        todo!();
        /*
            impl->saveAsAsync (newFile,
                            warnAboutOverwritingExistingFiles,
                            askUserForFileIfNotSpecified,
                            showMessageOnFailure,
                            std::move (callback));
        */
    }

    /**
      | Prompts the user for a filename and tries
      | to save to it.
      | 
      | This will pop up a dialog box using the
      | title, file extension and wildcard
      | specified in the document's constructor,
      | and asks the user for a file. If they pick
      | one, the saveAs() method is used to try
      | to save to this file.
      | 
      | -----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true and the file exists, it'll ask
      | the user first if they want to overwrite
      | it @see saveIfNeededAndUserAgrees,
      | save, saveAs
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as_interactive(
        &mut self, 
        warn_about_overwriting_existing_files: bool

    ) -> FileBasedDocumentSaveResult {
        
        todo!();
        /*
            return impl->saveAsInteractive (warnAboutOverwritingExistingFiles);
        */
    }
    
    /**
      | Prompts the user for a filename and tries
      | to save to it.
      | 
      | This will pop up a dialog box using the
      | title, file extension and wildcard
      | specified in the document's constructor,
      | and asks the user for a file. If they pick
      | one, the saveAs() method is used to try
      | to save to this file.
      | 
      | -----------
      | @param warnAboutOverwritingExistingFiles
      | 
      | if true and the file exists, it'll ask
      | the user first if they want to overwrite
      | it
      | ----------
      | @param callback
      | 
      | called with the result of the save operation
      | @see saveIfNeededAndUserAgreesAsync,
      | saveAsync, saveAsAsync
      |
      */
    pub fn save_as_interactive_async(
        &mut self, 
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ()

    ) {
        
        todo!();
        /*
            impl->saveAsInteractiveAsync (warnAboutOverwritingExistingFiles, std::move (callback));
        */
    }
    
    /**
      | Returns the file that this document
      | was last successfully saved or loaded
      | from.
      | 
      | When the document object is created,
      | this will be set to File().
      | 
      | It is changed when one of the load or save
      | methods is used, or when setFile() is
      | used to explicitly set it.
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return impl->getFile();
        */
    }
    
    /**
      | Sets the file that this document thinks
      | it was loaded from.
      | 
      | This won't actually load anything -
      | it just changes the file stored internally.
      | 
      | @see getFile
      |
      */
    pub fn set_file(&mut self, new_file: &File)  {
        
        todo!();
        /*
            impl->setFile (newFile);
        */
    }
    
    pub fn load_document_async(
        &mut self, 
        file:     &File,
        callback: fn(_0: Result<(),()>) -> ()

    ) {
        
        todo!();
        /*
            const auto result = loadDocument (file);

        if (callback != nullptr)
            callback (result);
        */
    }
    
    pub fn save_document_async(
        &mut self, 
        file:     &File,
        callback: fn(_0: Result<(),()>) -> ()

    ) {
        
        todo!();
        /*
            const auto result = saveDocument (file);

        if (callback != nullptr)
            callback (result);
        */
    }
    
    pub fn get_suggested_save_as_file(&mut self, default_file: &File) -> File {
        
        todo!();
        /*
            return defaultFile.withFileExtension (impl->getFileExtension()).getNonexistentSibling (true);
        */
    }
}
