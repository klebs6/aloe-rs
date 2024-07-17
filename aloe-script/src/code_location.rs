crate::ix!();

pub type Args<'a>  = &'a VarNativeFunctionArgs<'a>;
pub type TokenType = *const char;

///----------------------------------
pub struct CodeLocation {
    program:  String,
    location: CharPointerType,
}

impl CodeLocation {

    pub fn new(code: &String) -> Self {
    
        todo!();
        /*
        : program(code),
        : location(program.getCharPointer()),

        
        */
    }
    
    pub fn new_from_code_location(other: &CodeLocation) -> Self {
    
        todo!();
        /*
        : program(other.program),
        : location(other.location),

        
        */
    }
    
    pub fn throw_error(&self, message: &String)  {
        
        todo!();
        /*
            int col = 1, line = 1;

                        for (auto i = program.getCharPointer(); i < location && ! i.isEmpty(); ++i)
                        {
                            ++col;
                            if (*i == '\n')  { col = 1; ++line; }
                        }

                        throw "Line " + String (line) + ", column " + String (col) + " : " + message;
        */
    }
}
