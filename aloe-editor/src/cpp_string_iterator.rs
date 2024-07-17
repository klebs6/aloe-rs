crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CPlusPlusCodeTokeniserFunctions.h]

/**
  | A class that can be passed to the
  | CppTokeniserFunctions functions in order
  | to parse a String.
  |
  */
pub struct CppTokeniserStringIterator {
    t:         CharPointerType,
    num_chars: i32, // default = 0
}

/**
  | Class containing some basic functions
  | for simple tokenising of C++ code.
  | 
  | @tags{GUI}
  |
  */
impl CppTokeniserStringIterator {

    pub fn new_from_string(s: &String) -> Self {
    
        todo!();
        /*
        : t(s.getCharPointer()),

        
        */
    }
    
    pub fn new(s: CharPointerType) -> Self {
    
        todo!();
        /*
        : t(s),

        
        */
    }
    
    pub fn next_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            if (isEOF()) return 0; ++numChars; return t.getAndAdvance();
        */
    }
    
    pub fn peek_next_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            return *t;
        */
    }
    
    pub fn skip(&mut self)  {
        
        todo!();
        /*
            if (! isEOF()) { ++t; ++numChars; }
        */
    }
    
    pub fn skip_whitespace(&mut self)  {
        
        todo!();
        /*
            while (t.isWhitespace()) skip();
        */
    }
    
    pub fn skip_to_end_of_line(&mut self)  {
        
        todo!();
        /*
            while (*t != '\r' && *t != '\n' && *t != 0) skip();
        */
    }
    
    pub fn iseof(&self) -> bool {
        
        todo!();
        /*
            return t.isEmpty();
        */
    }
}
