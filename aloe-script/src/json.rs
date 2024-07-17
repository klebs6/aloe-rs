crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/javascript/aloe_JSON.h]

/**
  | Contains static methods for converting
  | JSON-formatted text to and from Var
  | objects.
  | 
  | The Var class is structurally compatible
  | with JSON-formatted data, so these
  | functions allow you to parse JSON into
  | a Var object, and to convert a Var object
  | to JSON-formatted text.
  | 
  | @see Var
  | 
  | @tags{Core}
  |
  */
pub struct JSON {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/javascript/aloe_JSON.cpp]
impl JSON {

    /**
      | Attempts to parse some JSON-formatted
      | text, and returns the result as a Var
      | object.
      | 
      | If the parsing fails, this simply returns
      | Var() - if you need to find out more detail
      | about the parse error, use the alternative
      | parse() method which returns a Result.
      | 
      | -----------
      | @note
      | 
      | this will only parse valid JSON, which
      | means that the item given must be either
      | an object or an array definition. If
      | you want to also be able to parse any kind
      | of primitive JSON object, use the fromString()
      | method.
      |
      */
    pub fn parse_text(&mut self, text: &String) -> Var {
        
        todo!();
        /*
            Var result;

        if (parse (text, result))
            return result;

        return {};
        */
    }
    
    /**
      | Parses a string that was created with
      | the toString() method.
      | 
      | This is slightly different to the parse()
      | methods because they will reject primitive
      | values and only accept array or object
      | definitions, whereas this method will
      | handle either.
      |
      */
    pub fn from_string(&mut self, text: &str) -> Var {
        
        todo!();
        /*
            try
        {
            return JSONParser (text.text).parseAny();
        }
        catch (const JSONParser::ErrorException&) {}

        return {};
        */
    }
    
    /**
      | Attempts to parse some JSON-formatted
      | text from a stream, and returns the result
      | as a Var object.
      | 
      | -----------
      | @note
      | 
      | this is just a short-cut for reading
      | the entire stream into a string and parsing
      | the result.
      | 
      | If the parsing fails, this simply returns
      | Var() - if you need to find out more detail
      | about the parse error, use the alternative
      | parse() method which returns a Result.
      |
      */
    pub fn parse_input(&mut self, input: &mut dyn Read) -> Var {
        
        todo!();
        /*
            return parse (input.readEntireStreamAsString());
        */
    }
    
    /**
      | Attempts to parse some JSON-formatted
      | text from a file, and returns the result
      | as a Var object.
      | 
      | -----------
      | @note
      | 
      | this is just a short-cut for reading
      | the entire file into a string and parsing
      | the result.
      | 
      | If the parsing fails, this simply returns
      | Var() - if you need to find out more detail
      | about the parse error, use the alternative
      | parse() method which returns a Result.
      |
      */
    pub fn parse_file(&mut self, file: &File) -> Var {
        
        todo!();
        /*
            return parse (file.loadFileAsString());
        */
    }
    
    /**
      | Parses a string of JSON-formatted text,
      | and returns a result code containing
      | any parse errors.
      | 
      | This will return the parsed structure
      | in the parsedResult parameter, and
      | will return a Result object to indicate
      | whether parsing was successful, and
      | if not, it will contain an error message.
      | 
      | If you're not interested in the error
      | message, you can use one of the other
      | shortcut parse methods, which simply
      | return a Var() if the parsing fails.
      | 
      | Note that this will only parse valid
      | JSON, which means that the item given
      | must be either an object or an array definition.
      | If you want to also be able to parse any
      | kind of primitive JSON object, use the
      | fromString() method.
      |
      */
    pub fn parse_text_into_result(
        &mut self, 
        text:   &String,
        result: &mut Var

    ) -> Result<(),()> {
        
        todo!();
        /*
            try
        {
            result = JSONParser (text.getCharPointer()).parseObjectOrArray();
        }
        catch (const JSONParser::ErrorException& error)
        {
            return error.getResult();
        }

        return Result::ok();
        */
    }
    
    /**
      | Returns a string which contains a JSON-formatted
      | representation of the Var object.
      | 
      | If allOnOneLine is true, the result
      | will be compacted into a single line
      | of text with no carriage-returns. If
      | false, it will be laid-out in a more human-readable
      | format.
      | 
      | The maximumDecimalPlaces parameter
      | determines the precision of floating
      | point numbers in scientific notation.
      | @see writeToStream
      |
      */
    pub fn to_string(
        &mut self, 
        data:                   &Var,
        all_on_one_line:        Option<bool>,
        maximum_decimal_places: Option<i32>

    ) -> String {

        let all_on_one_line:        bool = all_on_one_line.unwrap_or(false);
        let maximum_decimal_places:  i32 = maximum_decimal_places.unwrap_or(15);
        
        todo!();
        /*
            MemoryOutputStream mo (1024);
        JSONFormatter::write (mo, data, 0, allOnOneLine, maximumDecimalPlaces);
        return mo.toUTF8();
        */
    }
    
    /**
      | Writes a JSON-formatted representation
      | of the Var object to the given stream.
      | 
      | If allOnOneLine is true, the result
      | will be compacted into a single line
      | of text with no carriage-returns. If
      | false, it will be laid-out in a more human-readable
      | format.
      | 
      | The maximumDecimalPlaces parameter
      | determines the precision of floating
      | point numbers in scientific notation.
      | @see toString
      |
      */
    pub fn write_to_stream(
        &mut self, 
        output:                 &mut dyn Write,
        data:                   &Var,
        all_on_one_line:        Option<bool>,
        maximum_decimal_places: Option<i32>

    ) {

        let all_on_one_line:        bool = all_on_one_line.unwrap_or(false);
        let maximum_decimal_places:  i32 = maximum_decimal_places.unwrap_or(15);
        
        todo!();
        /*
            JSONFormatter::write (output, data, 0, allOnOneLine, maximumDecimalPlaces);
        */
    }
    
    /**
      | Returns a version of a string with any
      | extended characters escaped.
      |
      */
    pub fn escape_string(&mut self, s: &str) -> String {
        
        todo!();
        /*
            MemoryOutputStream mo;
        JSONFormatter::writeString (mo, s.text);
        return mo.toString();
        */
    }
    
    /**
      | Parses a quoted string-literal in JSON
      | format, returning the un-escaped result
      | in the result parameter, and an error
      | message in case the content was illegal.
      | 
      | This advances the text parameter, leaving
      | it positioned after the closing quote.
      |
      */
    pub fn parse_quoted_string(
        &mut self, 
        t:      &mut CharPointerType,
        result: &mut Var

    ) -> Result<(),()> {
        
        todo!();
        /*
            try
        {
            JSONParser parser (t);
            auto quote = parser.readChar();

            if (quote != '"' && quote != '\'')
                return Result::fail ("Not a quoted string!");

            result = parser.parseString (quote);
            t = parser.currentLocation;
        }
        catch (const JSONParser::ErrorException& error)
        {
            return error.getResult();
        }

        return Result::ok();
        */
    }
}
