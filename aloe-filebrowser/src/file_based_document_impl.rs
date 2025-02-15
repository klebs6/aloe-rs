crate::ix!();

#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct FileBasedDocumentImpl<'a> {
    document:               &'a mut FileBasedDocument<'a>,
    document_file:          File,
    changed_since_save:     bool, // default = false
    file_extension:         String,
    file_wildcard:          String,
    open_file_dialog_title: String,
    save_file_dialog_title: String,
    async_fc:               Box<FileChooser<'a>>,
}

impl<'a> FileBasedDocumentImpl<'a> {

    pub fn new(
        parent:                 &mut FileBasedDocument,
        file_extension:         &String,
        file_wildcard:          &String,
        open_file_dialog_title: &String,
        save_file_dialog_title: &String) -> Self {
    
        todo!();
        /*
        : document(parent_),
        : file_extension(fileExtension_),
        : file_wildcard(fileWildcard_),
        : open_file_dialog_title(openFileDialogTitle_),
        : save_file_dialog_title(saveFileDialogTitle_),

        
        */
    }
    
    pub fn has_changed_since_saved(&mut self) -> bool {
        
        todo!();
        /*
            return changedSinceSave;
        */
    }
    
    pub fn set_changed_flag(&mut self, has_changed: bool)  {
        
        todo!();
        /*
            if (changedSinceSave != hasChanged)
            {
                changedSinceSave = hasChanged;
                document.sendChangeMessage();
            }
        */
    }
    
    pub fn changed(&mut self)  {
        
        todo!();
        /*
            changedSinceSave = true;
            document.sendChangeMessage();
        */
    }
    
    pub fn load_from(
        &mut self, 
        new_file:                &File,
        show_message_on_failure: bool,
        show_wait_cursor:        Option<bool>

    ) -> Result<(),()> {

        let show_wait_cursor: bool = show_wait_cursor.unwrap_or(true);

        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, false };
            auto result = Result::ok();
            loadFromImpl (parent,
                          newFile,
                          showMessageOnFailure,
                          showWaitCursor,
                          [this] (const File& file, const auto& callback) { callback (document.loadDocument (file)); },
                          [&result] (Result r) { result = r; });
            return result;
        */
    }
    
    pub fn load_from_async(
        &mut self, 
        new_file:                &File,
        show_message_on_failure: bool,
        callback:                fn(_0: Result<(),()>) -> ()
    )  {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, true };
            loadFromImpl (parent,
                          newFile,
                          showMessageOnFailure,
                          false,
                          [parent] (const File& file, auto cb)
                          {
                              if (parent != nullptr)
                                  parent->document.loadDocumentAsync (file, std::move (cb));
                          },
                          std::move (callback));
        */
    }

    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn load_from_user_specified_file(&mut self, show_message_on_failure: bool) -> Result {
        
        todo!();
        /*
            FileChooser fc (openFileDialogTitle,
                            document.getLastDocumentOpened(),
                            fileWildcard);

            if (fc.browseForFileToOpen())
                return loadFrom (fc.getResult(), showMessageOnFailure);

            return Result::fail (TRANS ("User cancelled"));
        */
    }
    
    pub fn load_from_user_specified_file_async(
        &mut self, 
        show_message_on_failure: bool,
        callback:                fn(_0: Result<(),()>) -> ()

    ) {
        
        todo!();
        /*
            asyncFc = std::make_unique<FileChooser> (openFileDialogTitle,
                                                     document.getLastDocumentOpened(),
                                                     fileWildcard);

            asyncFc->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles,
                                  [this, showMessageOnFailure, callback = std::move (callback)] (const FileChooser& fc)
            {
                auto chosenFile = fc.getResult();

                if (chosenFile == File{})
                {
                    if (callback != nullptr)
                        callback (Result::fail (TRANS ("User cancelled")));

                    return;
                }

                WeakReference<FileBasedDocumentImpl> parent { this };
                loadFromAsync (chosenFile, showMessageOnFailure, [parent, callback] (Result result)
                {
                    if (parent != nullptr && callback != nullptr)
                        callback (result);
                });

                asyncFc = nullptr;
            });
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save(&mut self, 
        ask_user_for_file_if_not_specified: bool,
        show_message_on_failure:            bool) -> FileBasedDocument::FileBasedDocumentSaveResult {
        
        todo!();
        /*
            return saveAs (documentFile,
                           false,
                           askUserForFileIfNotSpecified,
                           showMessageOnFailure);
        */
    }
    
    pub fn save_async(&mut self, 
        ask_user_for_file_if_not_specified: bool,
        show_message_on_failure:            bool,
        callback:                           fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            saveAsAsync (documentFile,
                         false,
                         askUserForFileIfNotSpecified,
                         showMessageOnFailure,
                         std::move (callback));
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_if_needed_and_user_agrees(&mut self) -> FileBasedDocument::FileBasedDocumentSaveResult {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, false };
            FileBasedDocumentSaveResult result;
            saveIfNeededAndUserAgreesImpl (parent,
                                           [&result] (FileBasedDocumentSaveResult r) { result = r; },
                                           AskToSaveChangesSync { *this },
                                           SaveSync { *this });
            return result;
        */
    }
    
    pub fn save_if_needed_and_user_agrees_async(&mut self, callback: fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, true };

            saveIfNeededAndUserAgreesImpl (parent,
                                           std::move (callback),
                                           [] (FileBasedDocumentSafeParentPointer ptr, auto cb)
                                           {
                                               if (ptr != nullptr)
                                                   ptr->askToSaveChanges (ptr, std::move (cb));
                                           },
                                           [parent] (bool askUserForFileIfNotSpecified,
                                                     bool showMessageOnFailure,
                                                     auto cb)
                                           {
                                               if (parent != nullptr)
                                                   parent->saveAsync (askUserForFileIfNotSpecified,
                                                                      showMessageOnFailure,
                                                                      std::move (cb));
                                           });
        */
    }

    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as(&mut self, 
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        show_wait_cursor:                      bool) -> FileBasedDocument::FileBasedDocumentSaveResult {
        let show_wait_cursor: bool = show_wait_cursor.unwrap_or(true);

        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, false };
            FileBasedDocumentSaveResult result{};
            saveAsSyncImpl (parent,
                            newFile,
                            warnAboutOverwritingExistingFiles,
                            askUserForFileIfNotSpecified,
                            showMessageOnFailure,
                            [&result] (FileBasedDocumentSaveResult r) { result = r; },
                            showWaitCursor);
            return result;
        */
    }
    
    pub fn save_as_async(&mut self, 
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, true };
            saveAsAsyncImpl (parent,
                             newFile,
                             warnAboutOverwritingExistingFiles,
                             askUserForFileIfNotSpecified,
                             showMessageOnFailure,
                             std::move (callback),
                             false);
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as_interactive(&mut self, warn_about_overwriting_existing_files: bool) -> FileBasedDocument::FileBasedDocumentSaveResult {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, false };
            FileBasedDocumentSaveResult result{};
            saveAsInteractiveSyncImpl (parent,
                                       warnAboutOverwritingExistingFiles,
                                       [&result] (FileBasedDocumentSaveResult r) { result = r; });
            return result;
        */
    }
    
    pub fn save_as_interactive_async(&mut self, 
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            FileBasedDocumentSafeParentPointer parent { this, true };
            saveAsInteractiveAsyncImpl (parent,
                                        warnAboutOverwritingExistingFiles,
                                        std::move (callback));
        */
    }
    
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return documentFile;
        */
    }
    
    pub fn set_file(&mut self, new_file: &File)  {
        
        todo!();
        /*
            if (documentFile != newFile)
            {
                documentFile = newFile;
                changed();
            }
        */
    }
    
    pub fn get_file_extension(&self) -> &String {
        
        todo!();
        /*
            return fileExtension;
        */
    }
    
    pub fn load_from_impl<DoLoadDocument>(
        &mut self, 
        parent:                  FileBasedDocumentSafeParentPointer,
        new_file:                &File,
        show_message_on_failure: bool,
        show_wait_cursor:        bool,
        do_load_document:        DoLoadDocument,
        completed:               fn(_0: Result<(),()>) -> ()

    ) {
    
        todo!();
        /*
            if (parent.shouldExitAsyncCallback())
                return;

            if (showWaitCursor)
                MouseCursor::showWaitCursor();

            auto oldFile = documentFile;
            documentFile = newFile;

            auto tidyUp = [parent, newFile, oldFile, showMessageOnFailure, showWaitCursor, completed]
            {
                if (parent.shouldExitAsyncCallback())
                    return;

                parent->documentFile = oldFile;

                if (showWaitCursor)
                    MouseCursor::hideWaitCursor();

                auto result = Result::fail (TRANS ("The file doesn't exist"));

                if (showMessageOnFailure)
                    AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                      TRANS ("Failed to open file..."),
                                                      TRANS ("There was an error while trying to load the file: FLNM")
                                                              .replace ("FLNM", "\n" + newFile.getFullPathName())
                                                          + "\n\n"
                                                          + result.getErrorMessage());

                if (completed != nullptr)
                    completed (result);
            };

            if (newFile.existsAsFile())
            {
                auto afterLoading = [parent,
                                     showWaitCursor,
                                     newFile,
                                     completed = std::move (completed),
                                     tidyUp] (Result result)
                {
                    if (result.wasOk())
                    {
                        parent->setChangedFlag (false);

                        if (showWaitCursor)
                            MouseCursor::hideWaitCursor();

                        parent->document.setLastDocumentOpened (newFile);

                        if (completed != nullptr)
                            completed (result);

                        return;
                    }

                    tidyUp();
                };

                doLoadDocument (newFile, std::move (afterLoading));

                return;
            }

            tidyUp();
        */
    }
    
    pub fn save_if_needed_and_user_agrees_impl<DoAskToSaveChanges, DoSave>(&mut self, 
        parent:                 FileBasedDocumentSafeParentPointer,
        completed:              fn(_0: FileBasedDocumentSaveResult) -> (),
        do_ask_to_save_changes: DoAskToSaveChanges,
        do_save:                DoSave)  {
    
        todo!();
        /*
            if (parent.shouldExitAsyncCallback())
                return;

            if (! hasChangedSinceSaved())
            {
                if (completed != nullptr)
                    completed (savedOk);

                return;
            }

            auto afterAsking = [doSave = std::forward<DoSave> (doSave),
                                completed = std::move (completed)] (FileBasedDocumentSafeParentPointer ptr,
                                                                    int alertResult)
            {
                if (ptr.shouldExitAsyncCallback())
                    return;

                switch (alertResult)
                {
                    case 1:  // save changes
                        doSave (true, true, [ptr, completed] (FileBasedDocumentSaveResult result)
                        {
                            if (ptr.shouldExitAsyncCallback())
                                return;

                            if (completed != nullptr)
                                completed (result);
                        });
                        return;

                    case 2:  // discard changes
                        if (completed != nullptr)
                            completed (savedOk);

                        return;
                }

                if (completed != nullptr)
                    completed (userCancelledSave);
            };

            doAskToSaveChanges (parent, std::move (afterAsking));
        */
    }
    
    pub fn ask_to_save_changes(&mut self, 
        parent:   FileBasedDocumentSafeParentPointer,
        callback: fn(_0: FileBasedDocumentSafeParentPointer, _1: i32) -> ()) -> i32 {
        
        todo!();
        /*
            auto* modalCallback = callback == nullptr
                                      ? nullptr
                                      : ModalCallbackFunction::create ([parent, callback = std::move (callback)] (int alertResult)
                                                                       {
                                                                           if (parent != nullptr)
                                                                               callback (parent, alertResult);
                                                                       });

            return AlertWindow::showYesNoCancelBox (MessageBoxIconType::QuestionIcon,
                                                    TRANS ("Closing document..."),
                                                    TRANS ("Do you want to save the changes to \"DCNM\"?")
                                                        .replace ("DCNM", document.getDocumentTitle()),
                                                    TRANS ("Save"),
                                                    TRANS ("Discard changes"),
                                                    TRANS ("Cancel"),
                                                    nullptr,
                                                    modalCallback);
        */
    }
    
    pub fn save_internal<DoSaveDocument>(&mut self, 
        parent:                  FileBasedDocumentSafeParentPointer,
        new_file:                &File,
        show_message_on_failure: bool,
        show_wait_cursor:        bool,
        after_save:              fn(_0: FileBasedDocumentSaveResult) -> (),
        do_save_document:        DoSaveDocument)  {
    
        todo!();
        /*
            if (showWaitCursor)
                MouseCursor::showWaitCursor();

            auto oldFile = documentFile;
            documentFile = newFile;

            doSaveDocument (newFile, [parent,
                                      showMessageOnFailure,
                                      showWaitCursor,
                                      oldFile,
                                      newFile,
                                      afterSave = std::move (afterSave)] (Result result)
            {
                if (parent.shouldExitAsyncCallback())
                {
                    if (showWaitCursor)
                        MouseCursor::hideWaitCursor();

                    return;
                }

                if (result.wasOk())
                {
                    parent->setChangedFlag (false);

                    if (showWaitCursor)
                        MouseCursor::hideWaitCursor();

                    parent->document.sendChangeMessage(); // because the filename may have changed

                    if (afterSave != nullptr)
                        afterSave (savedOk);

                    return;
                }

                parent->documentFile = oldFile;

                if (showWaitCursor)
                    MouseCursor::hideWaitCursor();

                if (showMessageOnFailure)
                    AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                      TRANS ("Error writing to file..."),
                                                      TRANS ("An error occurred while trying to save \"DCNM\" to the file: FLNM")
                                                              .replace ("DCNM", parent->document.getDocumentTitle())
                                                              .replace ("FLNM", "\n" + newFile.getFullPathName())
                                                          + "\n\n"
                                                          + result.getErrorMessage());

                parent->document.sendChangeMessage(); // because the filename may have changed

                if (afterSave != nullptr)
                    afterSave (failedToWriteToFile);
            });
        */
    }
    
    pub fn save_as_impl<DoSaveAsInteractive, DoAskToOverwriteFile, DoSaveDocument>(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> (),
        show_wait_cursor:                      bool,
        do_save_as_interactive:                DoSaveAsInteractive,
        do_ask_to_overwrite_file:              DoAskToOverwriteFile,
        do_save_document:                      DoSaveDocument)  {
    
        todo!();
        /*
            if (parent.shouldExitAsyncCallback())
                return;

            if (newFile == File())
            {
                if (askUserForFileIfNotSpecified)
                {
                    doSaveAsInteractive (parent, true, std::move (callback));
                    return;
                }

                // can't save to an unspecified file
                jassertfalse;

                if (callback != nullptr)
                    callback (failedToWriteToFile);

                return;
            }

            auto saveInternalHelper = [parent,
                                       callback,
                                       newFile,
                                       showMessageOnFailure,
                                       showWaitCursor,
                                       doSaveDocument = std::forward<DoSaveDocument> (doSaveDocument)]
            {
                if (! parent.shouldExitAsyncCallback())
                    parent->saveInternal (parent,
                                          newFile,
                                          showMessageOnFailure,
                                          showWaitCursor,
                                          callback,
                                          doSaveDocument);
            };

            if (warnAboutOverwritingExistingFiles && newFile.exists())
            {
                auto afterAsking = [callback = std::move (callback),
                                    saveInternalHelper] (FileBasedDocumentSafeParentPointer ptr,
                                                         bool shouldOverwrite)
                {
                    if (ptr.shouldExitAsyncCallback())
                        return;

                    if (shouldOverwrite)
                        saveInternalHelper();
                    else if (callback != nullptr)
                        callback (userCancelledSave);
                };
                doAskToOverwriteFile (parent, newFile, std::move (afterAsking));
                return;
            }

            saveInternalHelper();
        */
    }
    
    pub fn save_as_async_impl(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> (),
        show_wait_cursor:                      bool)  {
        
        todo!();
        /*
            saveAsImpl (parent,
                        newFile,
                        warnAboutOverwritingExistingFiles,
                        askUserForFileIfNotSpecified,
                        showMessageOnFailure,
                        std::move (callback),
                        showWaitCursor,
                        [] (FileBasedDocumentSafeParentPointer ptr, bool warnAboutOverwriting, auto cb)
                        {
                            if (ptr != nullptr)
                                ptr->saveAsInteractiveAsyncImpl (ptr, warnAboutOverwriting, std::move (cb));
                        },
                        [] (FileBasedDocumentSafeParentPointer ptr, const File& destination, std::function<void (FileBasedDocumentSafeParentPointer, bool)> cb)
                        {
                            if (ptr != nullptr)
                                ptr->askToOverwriteFile (ptr, destination, std::move (cb));
                        },
                        [parent] (const File& destination, std::function<void (Result)> cb)
                        {
                            if (parent != nullptr)
                                parent->document.saveDocumentAsync (destination, std::move (cb));
                        });
        */
    }
    
    pub fn save_as_interactive_async_impl(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            if (parent == nullptr)
                return;

            saveAsInteractiveImpl (parent,
                                   warnAboutOverwritingExistingFiles,
                                   std::move (callback),
                                   [] (FileBasedDocumentSafeParentPointer ptr, bool warnAboutOverwriting, auto cb)
                                   {
                                       if (ptr != nullptr)
                                           ptr->getSaveAsFilenameAsync (ptr, warnAboutOverwriting, std::move (cb));
                                   },
                                   [] (FileBasedDocumentSafeParentPointer ptr,
                                       const File& newFile,
                                       bool warnAboutOverwriting,
                                       bool askUserForFileIfNotSpecified,
                                       bool showMessageOnFailure,
                                       auto cb,
                                       bool showWaitCursor)
                                   {
                                       if (ptr != nullptr)
                                           ptr->saveAsAsyncImpl (ptr,
                                                                 newFile,
                                                                 warnAboutOverwriting,
                                                                 askUserForFileIfNotSpecified,
                                                                 showMessageOnFailure,
                                                                 std::move (cb),
                                                                 showWaitCursor);
                                   },
                                   [] (FileBasedDocumentSafeParentPointer ptr, const File& destination, auto cb)
                                   {
                                       if (ptr != nullptr)
                                           ptr->askToOverwriteFile (ptr, destination, std::move (cb));
                                   });
        */
    }
    
    pub fn ask_to_overwrite_file(&mut self, 
        parent:   FileBasedDocumentSafeParentPointer,
        new_file: &File,
        callback: fn(_0: FileBasedDocumentSafeParentPointer, _1: bool) -> ()) -> bool {
        
        todo!();
        /*
            if (parent == nullptr)
                return false;

            auto* modalCallback = callback == nullptr
                                      ? nullptr
                                      : ModalCallbackFunction::create ([parent, callback = std::move (callback)] (int r)
                                                                       {
                                                                           if (parent != nullptr)
                                                                               callback (parent, r == 1);
                                                                       });

            return AlertWindow::showOkCancelBox (MessageBoxIconType::WarningIcon,
                                                 TRANS ("File already exists"),
                                                 TRANS ("There's already a file called: FLNM")
                                                         .replace ("FLNM", newFile.getFullPathName())
                                                     + "\n\n"
                                                     + TRANS ("Are you sure you want to overwrite it?"),
                                                 TRANS ("Overwrite"),
                                                 TRANS ("Cancel"),
                                                 nullptr,
                                                 modalCallback);
        */
    }
    
    pub fn get_save_as_filename_async(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSafeParentPointer, _1: &File) -> ())  {
        
        todo!();
        /*
            asyncFc = getInteractiveFileChooser();

            auto flags = FileBrowserComponent::saveMode | FileBrowserComponent::canSelectFiles;

            if (warnAboutOverwritingExistingFiles)
                flags |= FileBrowserComponent::warnAboutOverwriting;

            asyncFc->launchAsync (flags, [parent, callback = std::move (callback)] (const FileChooser& fc)
            {
                callback (parent, fc.getResult());
            });
        */
    }
    
    pub fn save_as_interactive_impl<DoSelectFilename, DoSaveAs, DoAskToOverwriteFile>(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> (),
        do_select_filename:                    DoSelectFilename,
        do_save_as:                            DoSaveAs,
        do_ask_to_overwrite_file:              DoAskToOverwriteFile)  {
    
        todo!();
        /*
            doSelectFilename (parent,
                              warnAboutOverwritingExistingFiles,
                              [doSaveAs = std::forward<DoSaveAs> (doSaveAs),
                               doAskToOverwriteFile = std::forward<DoAskToOverwriteFile> (doAskToOverwriteFile),
                               callback = std::move (callback)] (FileBasedDocumentSafeParentPointer parentPtr, File chosen)
            {
                if (parentPtr.shouldExitAsyncCallback())
                    return;

                if (chosen == File{})
                {
                    if (callback != nullptr)
                        callback (userCancelledSave);

                    return;
                }

                auto updateAndSaveAs = [parentPtr, doSaveAs, callback] (const File& chosenFile)
                {
                    if (parentPtr.shouldExitAsyncCallback())
                        return;

                    parentPtr->document.setLastDocumentOpened (chosenFile);
                    doSaveAs (parentPtr, chosenFile, false, false, true, callback, false);
                };

                if (chosen.getFileExtension().isEmpty())
                {
                    chosen = chosen.withFileExtension (parentPtr->fileExtension);

                    if (chosen.exists())
                    {
                        auto afterAsking = [chosen, updateAndSaveAs, callback] (FileBasedDocumentSafeParentPointer overwritePtr,
                                                                                bool overwrite)
                        {
                            if (overwritePtr.shouldExitAsyncCallback())
                                return;

                            if (overwrite)
                                updateAndSaveAs (chosen);
                            else if (callback != nullptr)
                                callback (userCancelledSave);
                        };

                        doAskToOverwriteFile (parentPtr, chosen, std::move (afterAsking));
                        return;
                    }
                }

                updateAndSaveAs (chosen);
            });
        */
    }
    
    pub fn get_interactive_file_chooser(&mut self) -> Box<FileChooser> {
        
        todo!();
        /*
            auto f = documentFile.existsAsFile() ? documentFile : document.getLastDocumentOpened();

            auto legalFilename = File::createLegalFileName (document.getDocumentTitle());

            if (legalFilename.isEmpty())
                legalFilename = "unnamed";

            f = (f.existsAsFile() || f.getParentDirectory().isDirectory())
                ? f.getSiblingFile (legalFilename)
                : File::getSpecialLocation (File::userDocumentsDirectory).getChildFile (legalFilename);

            f = document.getSuggestedSaveAsFile (f);

            return std::make_unique<FileChooser> (saveFileDialogTitle,
                                                  f,
                                                  fileWildcard);
        */
    }

    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as_sync_impl(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        new_file:                              &File,
        warn_about_overwriting_existing_files: bool,
        ask_user_for_file_if_not_specified:    bool,
        show_message_on_failure:               bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> (),
        show_wait_cursor:                      bool)  {
        
        todo!();
        /*
            saveAsImpl (parent,
                        newFile,
                        warnAboutOverwritingExistingFiles,
                        askUserForFileIfNotSpecified,
                        showMessageOnFailure,
                        std::move (callback),
                        showWaitCursor,
                        SaveAsInteractiveSyncImpl { *this },
                        AskToOverwriteFileSync { *this },
                        [this] (const File& file, const auto& cb) { cb (document.saveDocument (file)); });
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn ask_to_save_changes_sync<Callback>(&mut self, 
        parent:   FileBasedDocumentSafeParentPointer,
        callback: Callback)  {
    
        todo!();
        /*
            callback (parent, askToSaveChanges (parent, nullptr));
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_as_interactive_sync_impl(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        warn_about_overwriting_existing_files: bool,
        callback:                              fn(_0: FileBasedDocumentSaveResult) -> ())  {
        
        todo!();
        /*
            saveAsInteractiveImpl (parent,
                                   warnAboutOverwritingExistingFiles,
                                   std::move (callback),
                                   GetSaveAsFilenameSync { *this },
                                   SaveAsSyncImpl { *this },
                                   AskToOverwriteFileSync { *this });
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn ask_to_overwrite_file_sync<Callback>(&mut self, 
        parent:   FileBasedDocumentSafeParentPointer,
        new_file: &File,
        callback: Callback)  {
    
        todo!();
        /*
            callback (parent, askToOverwriteFile (parent, newFile, nullptr));
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn save_sync<Callback>(&mut self, 
        ask_user_for_file_if_not_specified: bool,
        show_message_on_failure:            bool,
        callback:                           Callback)  {
    
        todo!();
        /*
            callback (save (askUserForFileIfNotSpecified, showMessageOnFailure));
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn get_save_as_filename_sync<Callback>(&mut self, 
        parent:                                FileBasedDocumentSafeParentPointer,
        warn_about_overwriting_existing_files: bool,
        callback:                              Callback)  {
    
        todo!();
        /*
            auto fc = getInteractiveFileChooser();

            if (fc->browseForFileToSave (warnAboutOverwritingExistingFiles))
            {
                callback (parent, fc->getResult());
                return;
            }

            callback (parent, {});
        */
    }
}
