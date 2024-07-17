crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CodeTokeniser.h]

/**
  | A base class for tokenising code so that
  | the syntax can be displayed in a code
  | editor.
  | 
  | @see CodeDocument, CodeEditorComponent
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub trait CodeTokeniser {

    /**
      | Reads the next token from the source
      | and returns its token type.
      | 
      | This must leave the source pointing
      | to the first character in the next token.
      |
      */
    fn read_next_token(&mut self, source: &mut CodeDocumentIterator) -> i32;

    /**
      | Returns a suggested syntax highlighting
      | colour scheme.
      |
      */
    fn get_default_colour_scheme(&mut self) -> CodeEditorComponentColourScheme;
}
