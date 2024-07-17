crate::ix!();

pub trait ActiveDocumentChanged {

    /**
      | Callback which gets invoked when the
      | currently-active document changes.
      |
      */
    fn active_document_changed(&mut self);
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub trait TryCloseDocument {
    
    /**
      | A subclass must override this to say
      | whether its currently ok for a document
      | to be closed.
      | 
      | This method is called by closeDocument()
      | and closeAllDocuments() to indicate
      | that a document should be saved if possible,
      | ready for it to be closed.
      | 
      | If this method returns true, then it
      | means the document is ok and can be closed.
      | 
      | If it returns false, then it means that
      | the closeDocument() method should
      | stop and not close.
      | 
      | Normally, you'd use this method to ask
      | the user if they want to save any changes,
      | then return true if the save operation
      | went ok. If the user cancelled the save
      | operation you could return false here
      | to abort the close operation.
      | 
      | If your component is based on the FileBasedDocument
      | class, then you'd probably want to call
      | FileBasedDocument::saveIfNeededAndUserAgrees()
      | and return true if this returned
      | 
      | FileBasedDocument::savedOk
      | 
      | @see closeDocument, FileBasedDocument::saveIfNeededAndUserAgrees()
      |
      */
    fn try_to_close_document(&mut self, component: *mut Component) -> bool;
}

pub trait TryCloseDocumentAsync {

    /**
      | A subclass must override this to say
      | whether its currently ok for a document
      | to be closed.
      | 
      | This method is called by closeDocumentAsync()
      | and closeAllDocumentsAsync() to indicate
      | that a document should be saved if possible,
      | ready for it to be closed.
      | 
      | If the callback is called with a true
      | argument, then it means the document
      | is ok and can be closed.
      | 
      | If the callback is called with a false
      | argument, then it means that the closeDocumentAsync()
      | method should stop and not close.
      | 
      | Normally, you'd use this method to ask
      | the user if they want to save any changes,
      | then return true if the save operation
      | went ok. If the user cancelled the save
      | operation you could give a value of false
      | to the callback to abort the close operation.
      | 
      | If your component is based on the FileBasedDocument
      | class, then you'd probably want to call
      | FileBasedDocument::saveIfNeededAndUserAgreesAsync()
      | and call the calback with true if this
      | returned FileBasedDocument::savedOk.
      | 
      | @see closeDocumentAsync, FileBasedDocument::saveIfNeededAndUserAgreesAsync()
      |
      */
    fn try_to_close_document_async(&mut self, 
            component: *mut Component,
            callback:  fn(_0: bool) -> ());
}

pub trait CreateNewDocumentWindow {

    /**
      | Creates a new window to be used for a document.
      | 
      | The default implementation of this
      | just returns a basic MultiDocumentPanelWindow
      | object, but you might want to override
      | it to return a custom component.
      |
      */
    fn create_new_document_window(&mut self) -> *mut MultiDocumentPanelWindow;
}
