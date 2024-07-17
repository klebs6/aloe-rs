crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_NewLine.h]

/**
  | This class is used for represent a new-line
  | character sequence.
  | 
  | To write a new-line to a stream, you can
  | use the predefined 'newLine' variable,
  | e.g.
  | 
  | @code
  | 
  | myOutputStream << "Hello World" <<
  | newLine << newLine;
  | 
  | @endcode
  | 
  | The exact character sequence that will
  | be used for the new-line can be set and
  | retrieved with OutputStream::setNewLineString()
  | and OutputStream::getNewLineString().
  | 
  | @tags{Core}
  |
  */
pub struct NewLine {

}

impl Into<String> for NewLine {
    
    /**
      | Returns the default new-line sequence
      | that the library uses.
      | 
      | @see getDefault()
      |
      */
    #[inline] fn into(self) -> String {
        todo!();
        /*
            return getDefault();
        */
    }
}

impl<'a> Into<&'a str> for NewLine {
    
    /**
      | Returns the default new-line sequence
      | that the library uses.
      | 
      | @see OutputStream::setNewLineString()
      |
      */
    #[inline] fn into(self) -> &'a str {
        todo!();
        /*
            return getDefault();
        */
    }
}

impl NewLine {

    /**
      | Returns the default new-line sequence
      | that the library uses.
      | 
      | @see OutputStream::setNewLineString()
      |
      */
    pub fn get_default() -> *const u8 {
        
        todo!();
        /*
            return "\r\n";
        */
    }
}

/**
  | A predefined object representing a
  | new-line, which can be written to a string
  | or stream.
  | 
  | To write a new-line to a stream, you can
  | use the predefined 'newLine' variable
  | like this:
  | 
  | @code
  | 
  | myOutputStream << "Hello World" <<
  | newLine << newLine;
  | 
  | @endcode
  |
  */
lazy_static!{
    /*
    extern NewLine newLine;
    */
}

impl Add<&NewLine> for String {

    type Output = Self;
    
    /**
      | Writes a new-line sequence to a string.
      | 
      | You can use the predefined object 'newLine'
      | to invoke this, e.g.
      | 
      | @code
      | 
      | myString << "Hello World" << newLine
      | << newLine;
      | 
      | @endcode
      |
      */
    fn add(self, other: &NewLine) -> Self::Output {
        todo!();
        /*
            return String (NewLine::getDefault()) + s2;
        */
    }
}
