crate::ix!();

pub trait IsTextMetaEvent {

    /**
      | Returns true if this is a 'text' meta-event.
      | @see getTextFromTextMetaEvent
      |
      */
    fn is_text_meta_event(&self) -> bool;
}

pub trait GetTextFromTextMetaEvent {

    /**
      | Returns the text from a text meta-event.
      | @see isTextMetaEvent
      |
      */
    fn get_text_from_text_meta_event(&self) -> String;
}

pub trait TextMetaEvent {

    /**
      | Creates a text meta-event.
      |
      */
    fn text_meta_event(
        &mut self, 
        ty:   i32,
        text: &str
    ) -> dyn MidiMessageInterface;
}
