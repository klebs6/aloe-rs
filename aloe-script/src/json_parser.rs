crate::ix!();

pub struct JSONParser {
    start_location:   CharPointerType,
    current_location: CharPointerType,
}

impl JSONParser {

    pub fn new(text: CharPointerType) -> Self {
    
        todo!();
        /*
        : start_location(text),
        : current_location(text),

        
        */
    }
    
    pub fn throw_error(&mut self, 
        message:  String,
        location: CharPointerType) -> ! {
        
        todo!();
        /*
            JSONParserErrorException e;
            e.message = std::move (message);

            for (auto i = startLocation; i < location && ! i.isEmpty(); ++i)
            {
                ++e.column;
                if (*i == '\n')  { e.column = 1; e.line++; }
            }

            throw e;
        */
    }
    
    pub fn skip_whitespace(&mut self)  {
        
        todo!();
        /*
            currentLocation = currentLocation.findEndOfWhitespace();
        */
    }
    
    pub fn read_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            return currentLocation.getAndAdvance();
        */
    }
    
    pub fn peek_char(&self) -> wchar_t {
        
        todo!();
        /*
            return *currentLocation;
        */
    }
    
    pub fn match_if(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            if (peekChar() == (aloe_wchar) c) { ++currentLocation; return true; } return false;
        */
    }
    
    pub fn iseof(&self) -> bool {
        
        todo!();
        /*
            return peekChar() == 0;
        */
    }
    
    pub fn match_string(&mut self, t: *const u8) -> bool {
        
        todo!();
        /*
            while (*t != 0)
                if (! matchIf (*t++))
                    return false;

            return true;
        */
    }
    
    pub fn parse_object_or_array(&mut self) -> Var {
        
        todo!();
        /*
            skipWhitespace();

            if (matchIf ('{')) return parseObject();
            if (matchIf ('[')) return parseArray();

            if (! isEOF())
                throwError ("Expected '{' or '['", currentLocation);

            return {};
        */
    }
    
    pub fn parse_string(&mut self, quote_char: wchar_t) -> String {
        
        todo!();
        /*
            MemoryOutputStream buffer (256);

            for (;;)
            {
                auto c = readChar();

                if (c == quoteChar)
                    break;

                if (c == '\\')
                {
                    auto errorLocation = currentLocation;
                    c = readChar();

                    switch (c)
                    {
                        case '"':
                        case '\'':
                        case '\\':
                        case '/':  break;

                        case 'a':  c = '\a'; break;
                        case 'b':  c = '\b'; break;
                        case 'f':  c = '\f'; break;
                        case 'n':  c = '\n'; break;
                        case 'r':  c = '\r'; break;
                        case 't':  c = '\t'; break;

                        case 'u':
                        {
                            c = 0;

                            for (int i = 4; --i >= 0;)
                            {
                                auto digitValue = CharacterFunctions::getHexDigitValue (readChar());

                                if (digitValue < 0)
                                    throwError ("Syntax error in unicode escape sequence", errorLocation);

                                c = (aloe_wchar) ((c << 4) + static_cast<aloe_wchar> (digitValue));
                            }

                            break;
                        }

                        default:  break;
                    }
                }

                if (c == 0)
                    throwError ("Unexpected EOF in string constant", currentLocation);

                buffer.appendUTF8Char (c);
            }

            return buffer.toUTF8();
        */
    }
    
    pub fn parse_any(&mut self) -> Var {
        
        todo!();
        /*
            skipWhitespace();
            auto originalLocation = currentLocation;

            switch (readChar())
            {
                case '{':    return parseObject();
                case '[':    return parseArray();
                case '"':    return parseString ('"');
                case '\'':   return parseString ('\'');

                case '-':
                    skipWhitespace();
                    return parseNumber (true);

                case '0': case '1': case '2': case '3': case '4':
                case '5': case '6': case '7': case '8': case '9':
                    currentLocation = originalLocation;
                    return parseNumber (false);

                case 't':   // "true"
                    if (matchString ("rue"))
                        return Var (true);

                    break;

                case 'f':   // "false"
                    if (matchString ("alse"))
                        return Var (false);

                    break;

                case 'n':   // "null"
                    if (matchString ("ull"))
                        return {};

                    break;

                default:
                    break;
            }

            throwError ("Syntax error", originalLocation);
        */
    }
    
    pub fn parse_number(&mut self, is_negative: bool) -> Var {
        
        todo!();
        /*
            auto originalPos = currentLocation;

            int64 intValue = readChar() - '0';
            jassert (intValue >= 0 && intValue < 10);

            for (;;)
            {
                auto lastPos = currentLocation;
                auto c = readChar();
                auto digit = ((int) c) - '0';

                if (isPositiveAndBelow (digit, 10))
                {
                    intValue = intValue * 10 + digit;
                    continue;
                }

                if (c == 'e' || c == 'E' || c == '.')
                {
                    currentLocation = originalPos;
                    auto asDouble = CharacterFunctions::readDoubleValue (currentLocation);
                    return Var (isNegative ? -asDouble : asDouble);
                }

                if (CharacterFunctions::isWhitespace (c)
                     || c == ',' || c == '}' || c == ']' || c == 0)
                {
                    currentLocation = lastPos;
                    break;
                }

                throwError ("Syntax error in number", lastPos);
            }

            auto correctedValue = isNegative ? -intValue : intValue;

            return (intValue >> 31) != 0 ? Var (correctedValue)
                                         : Var ((int) correctedValue);
        */
    }
    
    pub fn parse_object(&mut self) -> Var {
        
        todo!();
        /*
            auto resultObject = new DynamicObject();
            Var result (resultObject);
            auto& resultProperties = resultObject->getProperties();
            auto startOfObjectDecl = currentLocation;

            for (;;)
            {
                skipWhitespace();
                auto errorLocation = currentLocation;
                auto c = readChar();

                if (c == '}')
                    break;

                if (c == 0)
                    throwError ("Unexpected EOF in object declaration", startOfObjectDecl);

                if (c != '"')
                    throwError ("Expected a property name in double-quotes", errorLocation);

                errorLocation = currentLocation;
                Identifier propertyName (parseString ('"'));

                if (! propertyName.isValid())
                    throwError ("Invalid property name", errorLocation);

                skipWhitespace();
                errorLocation = currentLocation;

                if (readChar() != ':')
                    throwError ("Expected ':'", errorLocation);

                resultProperties.set (propertyName, parseAny());

                skipWhitespace();
                if (matchIf (',')) continue;
                if (matchIf ('}')) break;

                throwError ("Expected ',' or '}'", currentLocation);
            }

            return result;
        */
    }
    
    pub fn parse_array(&mut self) -> Var {
        
        todo!();
        /*
            auto result = Var (Vec<Var>());
            auto destArray = result.getArray();
            auto startOfArrayDecl = currentLocation;

            for (;;)
            {
                skipWhitespace();

                if (matchIf (']'))
                    break;

                if (isEOF())
                    throwError ("Unexpected EOF in array declaration", startOfArrayDecl);

                destArray->add (parseAny());
                skipWhitespace();

                if (matchIf (',')) continue;
                if (matchIf (']')) break;

                throwError ("Expected ',' or ']'", currentLocation);
            }

            return result;
        */
    }
}
