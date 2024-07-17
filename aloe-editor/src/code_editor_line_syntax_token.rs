crate::ix!();

pub struct CodeEditorLineSyntaxToken {
    text:       String,
    length:     i32,
    token_type: i32,
}

impl PartialEq<CodeEditorLineSyntaxToken> for CodeEditorLineSyntaxToken {
    
    #[inline] fn eq(&self, other: &CodeEditorLineSyntaxToken) -> bool {
        todo!();
        /*
            return tokenType == other.tokenType
                        && length == other.length
                        && text == other.text;
        */
    }
}

impl Eq for CodeEditorLineSyntaxToken {}

impl CodeEditorLineSyntaxToken {

    pub fn new(
        t:   &String,
        len: i32,
        ty:  i32) -> Self {
    
        todo!();
        /*
        : text(t),
        : length(len),
        : token_type(type),

        
        */
    }
}
