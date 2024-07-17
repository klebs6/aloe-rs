crate::ix!();

pub trait SetDocumentEditedStatus {

    /**
      | If this type of window is capable of indicating
      | that the document in it has been edited,
      | then this changes its status.
      | 
      | For example in OSX, this changes the
      | appearance of the close button.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the window has a mechanism for
      | showing this, or false if not.
      |
      */
    fn set_document_edited_status(&mut self, edited: bool) -> bool;
}
