crate::ix!();

/**
  | a word or space that can't be broken down
  | any further
  |
  */
#[leak_detector]
pub struct TextAtom {
    atom_text: String,
    width:     f32,
    num_chars: i32,
}

impl TextAtom {

    pub fn is_whitespace(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isWhitespace (atomText[0]);
        */
    }
    
    pub fn is_new_line(&self) -> bool {
        
        todo!();
        /*
            return atomText[0] == '\r' || atomText[0] == '\n';
        */
    }
    
    pub fn get_text(&self, password_character: wchar_t) -> String {
        
        todo!();
        /*
            if (passwordCharacter == 0)
                return atomText;

            return String::repeatedString (String::charToString (passwordCharacter),
                                           atomText.length());
        */
    }
    
    pub fn get_trimmed_text(&self, password_character: wchar_t) -> String {
        
        todo!();
        /*
            if (passwordCharacter == 0)
                return atomText.substring (0, numChars);

            if (isNewLine())
                return {};

            return String::repeatedString (String::charToString (passwordCharacter), numChars);
        */
    }
}
