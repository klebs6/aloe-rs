crate::ix!();

/**
  | Note Expression Text event. Used in
  | VstEvent (union)
  | 
  | A Expression event affects one single
  | playing note. \sa INoteExpressionController
  | 
  | \see NoteExpressionTypeInfo
  |
  */
#[derive(Copy,Clone)]
pub struct NoteExpressionTextEvent
{
    /**
      | see \ref NoteExpressionTypeID (kTextTypeID
      | or kPhoneticTypeID)
      |
      */
    type_id:  NoteExpressionTypeID,

    /**
      | associated note identifier to apply
      | the change
      |
      */
    note_id:  i32,

    /**
      | the number of characters (TChar) between
      | the beginning of text and the terminating
      | null character (without including the
      | terminating null character itself)
      */
    text_len: u32,

    /**
      | UTF-16, null terminated
      |
      */
    text:     *const TChar,
}
