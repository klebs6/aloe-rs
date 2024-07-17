crate::ix!();

pub struct TokenIterator {
    location:      CodeLocation,
    current_type:  TokenType,
    current_value: Var,
    p:             CharPointerType,
}

impl TokenIterator {
    
    pub fn new(code: &String) -> Self {
    
        todo!();
        /*
        : location(code),
        : p(code.getCharPointer()),

            skip();
        */
    }
    
    pub fn skip(&mut self)  {
        
        todo!();
        /*
            skipWhitespaceAndComments();
            location.location = p;
            currentType = matchNextToken();
        */
    }
    
    pub fn match_(&mut self, expected: TokenType)  {
        
        todo!();
        /*
            if (currentType != expected)
                location.throwError ("Found " + getTokenName (currentType) + " when expecting " + getTokenName (expected));

            skip();
        */
    }
    
    pub fn match_if(&mut self, expected: TokenType) -> bool {
        
        todo!();
        /*
            if (currentType == expected)  { skip(); return true; } return false;
        */
    }
    
    pub fn matches_any2(
        &self, 
        t1: TokenType,
        t2: TokenType
    ) -> bool {
        
        todo!();
        /*
            return currentType == t1 || currentType == t2;
        */
    }
    
    pub fn matches_any3(
        &self, 
        t1: TokenType,
        t2: TokenType,
        t3: TokenType

    ) -> bool {
        
        todo!();
        /*
            return matchesAny (t1, t2) || currentType == t3;
        */
    }
    
    pub fn is_identifier_start(c: wchar_t) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetter (c)        || c == '_';
        */
    }
    
    pub fn is_identifier_body(c: wchar_t) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetterOrDigit (c) || c == '_';
        */
    }
    
    pub fn match_next_token(&mut self) -> TokenType {
        
        todo!();
        /*
            if (isIdentifierStart (*p))
            {
                auto end = p;
                while (isIdentifierBody (*++end)) {}

                auto len = (size_t) (end - p);
                #define ALOE_JS_COMPARE_KEYWORD(name, str) if (len == sizeof (str) - 1 && matchToken (TokenTypes::name, len)) return TokenTypes::name;
                ALOE_JS_KEYWORDS (ALOE_JS_COMPARE_KEYWORD)

                    currentValue = String (p, end); p = end;
                return TokenTypes::identifier;
            }

            if (p.isDigit())
            {
                if (parseHexLiteral() || parseFloatLiteral() || parseOctalLiteral() || parseDecimalLiteral())
                    return TokenTypes::literal;

                location.throwError ("Syntax error in numeric constant");
            }

            if (parseStringLiteral (*p) || (*p == '.' && parseFloatLiteral()))
                return TokenTypes::literal;

            #define ALOE_JS_COMPARE_OPERATOR(name, str) if (matchToken (TokenTypes::name, sizeof (str) - 1)) return TokenTypes::name;
            ALOE_JS_OPERATORS (ALOE_JS_COMPARE_OPERATOR)

                if (! p.isEmpty())
                    location.throwError ("Unexpected character '" + String::charToString (*p) + "' in source");

            return TokenTypes::eof;
        */
    }
    
    pub fn match_token(&mut self, 
        name: TokenType,
        len:  usize) -> bool {
        
        todo!();
        /*
            if (p.compareUpTo (CharPointer_ASCII (name), (int) len) != 0) return false;
            p += (int) len;  return true;
        */
    }
    
    pub fn skip_whitespace_and_comments(&mut self)  {
        
        loop {

            self.p.increment_to_end_of_whitespace();

            if self.p[0] == '/'
            {
                let mut c2 = self.p[1];

                if c2 == '/'  { 

                    self.p = CharacterFunctions::find_char(&self.p, '\n'); 

                    continue; 
                }

                if c2 == '*'
                {
                    todo!("all should be uncommented, the line comments are used because of the stars and slashes");

                    // self.location = self.p;

                    // self.p = CharacterFunctions::find_char( self.p + 2, CharPointer_ASCII::new_from_str("*/"));

                    todo!();
                    //if self.p.is_empty() { self.location.throw_error("Unterminated '/*' comment"); }

                    todo!();
                    /*
                    self.p += 2; 

                    continue;
                    */
                }
            }

            break;
        }
    }
    
    pub fn parse_string_literal(&mut self, quote_type: wchar_t) -> bool {
        
        todo!();
        /*
            if (quoteType != '"' && quoteType != '\'')
                return false;

            auto r = JSON::parseQuotedString (p, currentValue);
            if (r.failed()) location.throwError (r.getErrorMessage());
            return true;
        */
    }
    
    pub fn parse_hex_literal(&mut self) -> bool {
        
        todo!();
        /*
            if (*p != '0' || (p[1] != 'x' && p[1] != 'X')) return false;

            auto t = ++p;
            int64 v = CharacterFunctions::getHexDigitValue (*++t);
            if (v < 0) return false;

            for (;;)
            {
                auto digit = CharacterFunctions::getHexDigitValue (*++t);
                if (digit < 0) break;
                v = v * 16 + digit;
            }

            currentValue = v; p = t;
            return true;
        */
    }
    
    pub fn parse_float_literal(&mut self) -> bool {
        
        todo!();
        /*
            int numDigits = 0;
            auto t = p;
            while (t.isDigit())  { ++t; ++numDigits; }

            const bool hasPoint = (*t == '.');

            if (hasPoint)
                while ((++t).isDigit())  ++numDigits;

            if (numDigits == 0)
                return false;

            auto c = *t;
            const bool hasExponent = (c == 'e' || c == 'E');

            if (hasExponent)
            {
                c = *++t;
                if (c == '+' || c == '-')  ++t;
                if (! t.isDigit()) return false;
                while ((++t).isDigit()) {}
            }

            if (! (hasExponent || hasPoint)) return false;

            currentValue = CharacterFunctions::getDoubleValue (p);  p = t;
            return true;
        */
    }
    
    pub fn parse_octal_literal(&mut self) -> bool {
        
        todo!();
        /*
            auto t = p;
            int64 v = *t - '0';
            if (v != 0) return false;  // first digit of octal must be 0

            for (;;)
            {
                auto digit = (int) (*++t - '0');
                if (isPositiveAndBelow (digit, 8))        v = v * 8 + digit;
                else if (isPositiveAndBelow (digit, 10))  location.throwError ("Decimal digit in octal constant");
                else break;
            }

            currentValue = v;  p = t;
            return true;
        */
    }
    
    pub fn parse_decimal_literal(&mut self) -> bool {
        
        todo!();
        /*
            int64 v = 0;

            for (;; ++p)
            {
                auto digit = (int) (*p - '0');
                if (isPositiveAndBelow (digit, 10))  v = v * 10 + digit;
                else break;
            }

            currentValue = v;
            return true;
        */
    }
}
