crate::ix!();

/**
  | An object that receives callbacks from
  | the CodeDocument when its text changes.
  | @see CodeDocument::addListener,
  | CodeDocument::removeListener
  |
  */
pub trait CodeDocumentListener
{
    /**
      | Called by a CodeDocument when text is
      | added.
      |
      */
    fn code_document_text_inserted(
        &mut self, 
        new_text:     &str,
        insert_index: i32
    );

    /**
      | Called by a CodeDocument when text is
      | deleted.
      |
      */
    fn code_document_text_deleted(
        &mut self, 
        start_index: i32,
        end_index:   i32
    );
}

