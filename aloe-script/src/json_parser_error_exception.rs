crate::ix!();

pub struct JSONParserErrorException {
    message: String,
    line:    i32, // default = 1
    column:  i32, // default = 1
}

impl JSONParserErrorException {

    pub fn get_description(&self) -> String {
        
        todo!();
        /*
            return String (line) + ":" + String (column) + ": error: " + message;
        */
    }
    
    pub fn get_result(&self) -> Result<(),()> {
        
        todo!();
        /*
            return Result::fail (getDescription());
        */
    }
}
