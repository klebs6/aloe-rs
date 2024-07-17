crate::ix!();

/**
  | An input filter for a TextEditor that
  | limits the length of text and/or the
  | characters that it may contain.
  |
  */
#[no_copy]
#[leak_detector]
pub struct TextEditorLengthAndCharacterRestriction {
    allowed_characters: String,
    max_length:         i32,
}

impl TextEditorInputFilter for TextEditorLengthAndCharacterRestriction {

    fn filter_new_text(&mut self, 
        ed:        &mut TextEditor,
        new_input: &String) -> String {
        
        todo!();
        /*
            String t (newInput);

        if (allowedCharacters.isNotEmpty())
            t = t.retainCharacters (allowedCharacters);

        if (maxLength > 0)
            t = t.substring (0, maxLength - (ed.getTotalNumChars() - ed.getHighlightedRegion().getLength()));

        return t;
        */
    }
}

impl TextEditorLengthAndCharacterRestriction {

    /**
      | Creates a filter that limits the length
      | of text, and/or the characters that
      | it can contain.
      | 
      | -----------
      | @param maxNumChars
      | 
      | if this is > 0, it sets a maximum length
      | limit; if <= 0, no limit is set
      | ----------
      | @param allowedCharacters
      | 
      | if this is non-empty, then only characters
      | that occur in this string are allowed
      | to be entered into the editor.
      |
      */
    pub fn new(
        max_len: i32,
        chars:   &String) -> Self {
    
        todo!();
        /*
        : allowed_characters(chars),
        : max_length(maxLen),

        
        */
    }
}
