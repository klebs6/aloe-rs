crate::ix!();

pub trait GetDocumentTitle {

    /**
      | Overload this to return the title of
      | the document.
      | 
      | This is used in message boxes, filenames
      | and file choosers, so it should be something
      | sensible.
      |
      */
    fn get_document_title(&mut self) -> String;
}

pub trait LoadDocument {

    /**
      | This method should try to load your document
      | from the given file.
      | 
      | -----------
      | @return
      | 
      | a Result object to indicate the whether
      | there was an error.
      |
      */
    fn load_document(&mut self, file: &File) -> Result<(),()>;
}

pub trait LoadDocumentAsync {

    /**
      | This method should try to load your document
      | from the given file, then call the provided
      | callback on the message thread, passing
      | the result of the load.
      | 
      | By default, this will synchronously
      | call through to loadDocument.
      | 
      | For longer-running load operations,
      | you may wish to override this function
      | to run the load on a background thread,
      | and then to call the callback later on
      | the message thread to signal that the
      | load has completed.
      |
      */
    fn load_document_async(
        &mut self, 
        file:     &File,
        callback: fn(_0: Result<(),()>) -> ()
    );

}

pub trait SaveDocument {

    /**
      | This method should try to write your
      | document to the given file.
      | 
      | -----------
      | @return
      | 
      | a Result object to indicate the whether
      | there was an error.
      |
      */
    fn save_document(&mut self, file: &File) -> Result<(),()>;
}

pub trait SaveDocumentAsync {

    /**
      | This method should try to write your
      | document to the given file, then call
      | the provided callback on the message
      | thread, passing the result of the write.
      | 
      | By default, this will synchronously
      | call through to saveDocument.
      | 
      | For longer-running save operations,
      | you may wish to override this function
      | to run the save on a background thread,
      | and then to call the callback later on
      | the message thread to signal that the
      | save has completed.
      |
      */
    fn save_document_async(
        &mut self, 
        file:     &File,
        callback: fn(_0: Result<(),()>) -> ()
    );
}

pub trait GetLastDocumentOpened {

    /**
      | This is used for dialog boxes to make
      | them open at the last folder you were
      | using.
      | 
      | getLastDocumentOpened() and setLastDocumentOpened()
      | are used to store the last document that
      | was used - you might want to store this
      | value in a static variable, or even in
      | your application's properties. It
      | should be a global setting rather than
      | a property of this object.
      | 
      | This method works very well in conjunction
      | with a RecentlyOpenedFilesList object
      | to manage your recent-files list.
      | 
      | As a default value, it's ok to return
      | File(), and the document object will
      | use a sensible one instead.
      | 
      | @see RecentlyOpenedFilesList
      |
      */

    fn get_last_document_opened(&mut self) -> File;

}
pub trait SetLastDocumentOpened {

    /**
      | This is used for dialog boxes to make
      | them open at the last folder you were
      | using.
      | 
      | getLastDocumentOpened() and setLastDocumentOpened()
      | are used to store the last document that
      | was used - you might want to store this
      | value in a static variable, or even in
      | your application's properties. It
      | should be a global setting rather than
      | a property of this object.
      | 
      | This method works very well in conjunction
      | with a RecentlyOpenedFilesList object
      | to manage your recent-files list.
      | 
      | @see RecentlyOpenedFilesList
      |
      */
    fn set_last_document_opened(&mut self, file: &File);
}

pub trait GetSuggestedSaveAsFile {

    /**
      | This is called by saveAsInteractiveAsync()
      | to allow you to optionally customise
      | the filename that the user is presented
      | with in the save dialog.
      | 
      | The defaultFile parameter is an initial
      | suggestion based on what the class knows
      | about the current document - you can
      | return a variation on this file with
      | a different extension, etc, or just
      | return something completely different.
      |
      */
    fn get_suggested_save_as_file(&mut self, default_file: &File) -> File;
}

pub trait Changed {

    /**
      | Called to indicate that the document
      | has changed and needs saving.
      | 
      | This method will also trigger a change
      | message to be sent out using the
      | 
      | ChangeBroadcaster base class.
      | 
      | After calling the method, the hasChangedSinceSaved()
      | method will return true, until it is
      | reset either by saving to a file or using
      | the setChangedFlag() method.
      | 
      | @see hasChangedSinceSaved, setChangedFlag
      |
      */
    fn changed(&mut self);
}

