crate::ix!();

/**
  | The token values returned by this tokeniser.
  |
  */
pub enum CPlusPlusCodeTokeniserTokenType
{
    tokenType_error = 0,
    tokenType_comment,
    tokenType_keyword,
    tokenType_operator,
    tokenType_identifier,
    tokenType_integer,
    tokenType_float,
    tokenType_string,
    tokenType_bracket,
    tokenType_punctuation,
    tokenType_preprocessor
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CPlusPlusCodeTokeniser.h]

/**
  | A simple lexical analyser for syntax
  | colouring of C++ code.
  | 
  | @see CodeEditorComponent, CodeDocument
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct CPlusPlusCodeTokeniser {

}

impl CodeTokeniser for CPlusPlusCodeTokeniser {

    fn get_default_colour_scheme(&mut self) -> CodeEditorComponentColourScheme {
        
        todo!();
        /*
            struct Type
        {
            const char* name;
            uint32 colour;
        };

        const Type types[] =
        {
            { "Error",              0xffcc0000 },
            { "Comment",            0xff00aa00 },
            { "Keyword",            0xff0000cc },
            { "Operator",           0xff225500 },
            { "Identifier",         0xff000000 },
            { "Integer",            0xff880000 },
            { "Float",              0xff885500 },
            { "String",             0xff990099 },
            { "Bracket",            0xff000055 },
            { "Punctuation",        0xff004400 },
            { "Preprocessor Text",  0xff660000 }
        };

        CodeEditorComponent::ColourScheme cs;

        for (auto& t : types)
            cs.set (t.name, Colour (t.colour));

        return cs;
        */
    }

    fn read_next_token(&mut self, source: &mut CodeDocumentIterator) -> i32 {

        todo!();
            /*
                source.skipWhitespace();
                auto firstChar = source.peekNextChar();

                switch (firstChar)
                {
                case 0:
                    break;

                case '0':   case '1':   case '2':   case '3':   case '4':
                case '5':   case '6':   case '7':   case '8':   case '9':
                case '.':
                {
                    auto result = parseNumber (source);

                    if (result == CPlusPlusCodeTokeniser::tokenType_error)
                    {
                        source.skip();

                        if (firstChar == '.')
                            return CPlusPlusCodeTokeniser::tokenType_punctuation;
                    }

                    return result;
                }

                case ',':
                case ';':
                case ':':
                    source.skip();
                    return CPlusPlusCodeTokeniser::tokenType_punctuation;

                case '(':   case ')':
                case '{':   case '}':
                case '[':   case ']':
                    source.skip();
                    return CPlusPlusCodeTokeniser::tokenType_bracket;

                case '"':
                case '\'':
                    skipQuotedString (source);
                    return CPlusPlusCodeTokeniser::tokenType_string;

                case '+':
                    source.skip();
                    skipIfNextCharMatches (source, '+', '=');
                    return CPlusPlusCodeTokeniser::tokenType_operator;

                case '-':
                {
                    source.skip();
                    auto result = parseNumber (source);

                    if (result == CPlusPlusCodeTokeniser::tokenType_error)
                    {
                        skipIfNextCharMatches (source, '-', '=');
                        return CPlusPlusCodeTokeniser::tokenType_operator;
                    }

                    return result;
                }

                case '*':   case '%':
                case '=':   case '!':
                    source.skip();
                    skipIfNextCharMatches (source, '=');
                    return CPlusPlusCodeTokeniser::tokenType_operator;

                case '/':
                {
                    source.skip();
                    auto nextChar = source.peekNextChar();

                    if (nextChar == '/')
                    {
                        source.skipToEndOfLine();
                        return CPlusPlusCodeTokeniser::tokenType_comment;
                    }

                    if (nextChar == '*')
                    {
                        source.skip();
                        skipComment (source);
                        return CPlusPlusCodeTokeniser::tokenType_comment;
                    }

                    if (nextChar == '=')
                        source.skip();

                    return CPlusPlusCodeTokeniser::tokenType_operator;
                }

                case '?':
                case '~':
                    source.skip();
                    return CPlusPlusCodeTokeniser::tokenType_operator;

                case '<':   case '>':
                case '|':   case '&':   case '^':
                    source.skip();
                    skipIfNextCharMatches (source, firstChar);
                    skipIfNextCharMatches (source, '=');
                    return CPlusPlusCodeTokeniser::tokenType_operator;

                case '#':
                    skipPreprocessorLine (source);
                    return CPlusPlusCodeTokeniser::tokenType_preprocessor;

                default:
                    if (isIdentifierStart (firstChar))
                        return parseIdentifier (source);

                    source.skip();
                    break;
                }

                return CPlusPlusCodeTokeniser::tokenType_error;
            */
    }

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CPlusPlusCodeTokeniser.cpp]
impl CPlusPlusCodeTokeniser {

    pub fn is_identifier_start(c: wchar_t) -> bool {
        
        todo!();
            /*
                return CharacterFunctions::isLetter (c)
                        || c == '_' || c == '@';
            */
    }

    pub fn is_identifier_body(c: wchar_t) -> bool {
        
        todo!();
            /*
                return CharacterFunctions::isLetterOrDigit (c)
                        || c == '_' || c == '@';
            */
    }

    pub fn is_reserved_keyword(
            token:        CharPointerType,
            token_length: i32) -> bool {
        
        todo!();
            /*
                static const char* const keywords2Char[] =
                    { "do", "if", "or", nullptr };

                static const char* const keywords3Char[] =
                    { "and", "asm", "for", "int", "new", "not", "try", "xor", nullptr };

                static const char* const keywords4Char[] =
                    { "auto", "bool", "case", "char", "else", "enum", "goto",
                      "long", "this", "true", "void", nullptr };

                static const char* const keywords5Char[] =
                    { "bitor", "break", "catch", "class", "compl", "const", "false", "final",
                      "float", "or_eq", "short", "throw", "union", "using", "while", nullptr };

                static const char* const keywords6Char[] =
                    { "and_eq", "bitand", "delete", "double", "export", "extern", "friend",
                      "import", "inline", "module", "not_eq", "public", "return", "signed",
                      "sizeof", "static", "struct", "switch", "typeid", "xor_eq", nullptr };

                static const char* const keywords7Char[] =
                    { "__cdecl", "_Pragma", "alignas", "alignof", "concept", "default",
                      "mutable", "nullptr", "private", "typedef", "uint8_t", "virtual",
                      "wchar_t", nullptr };

                static const char* const keywordsOther[] =
                    { "@class", "@dynamic", "@end", "@implementation", "@interface", "@public",
                      "@private", "@protected", "@property", "@synthesize", "__fastcall", "__stdcall",
                      "atomic_cancel", "atomic_commit", "atomic_", "char16_t", "char32_t",
                      "co_await", "co_return", "co_yield", "const_cast", "constexpr", "continue",
                      "decltype", "dynamic_cast", "explicit", "namespace", "", "operator", "override",
                      "protected", "register", "reinterpret_cast", "requires", "static_assert",
                      "static_cast", "synchronized", "template", "thread_local", "typename", "unsigned",
                      "volatile", nullptr };

                const char* const* k;

                switch (tokenLength)
                {
                    case 2:     k = keywords2Char; break;
                    case 3:     k = keywords3Char; break;
                    case 4:     k = keywords4Char; break;
                    case 5:     k = keywords5Char; break;
                    case 6:     k = keywords6Char; break;
                    case 7:     k = keywords7Char; break;

                    default:
                        if (tokenLength < 2 || tokenLength > 16)
                            return false;

                        k = keywordsOther;
                        break;
                }

                for (int i = 0; k[i] != nullptr; ++i)
                    if (token.compare (CharPointer_ASCII (k[i])) == 0)
                        return true;

                return false;
            */
    }

    pub fn parse_identifier<Iterator>(source: &mut Iterator) -> i32 {

        todo!();
            /*
                int tokenLength = 0;
                CharPointerType::CharType possibleIdentifier[100];
                CharPointerType possible (possibleIdentifier);

                while (isIdentifierBody (source.peekNextChar()))
                {
                    auto c = source.nextChar();

                    if (tokenLength < 20)
                        possible.write (c);

                    ++tokenLength;
                }

                if (tokenLength > 1 && tokenLength <= 16)
                {
                    possible.writeNull();

                    if (isReservedKeyword (CharPointerType (possibleIdentifier), tokenLength))
                        return CPlusPlusCodeTokeniser::tokenType_keyword;
                }

                return CPlusPlusCodeTokeniser::tokenType_identifier;
            */
    }

    pub fn skip_number_suffix<Iterator>(source: &mut Iterator) -> bool {

        todo!();
            /*
                auto c = source.peekNextChar();

                if (c == 'l' || c == 'L' || c == 'u' || c == 'U')
                    source.skip();

                if (CharacterFunctions::isLetterOrDigit (source.peekNextChar()))
                    return false;

                return true;
            */
    }


    pub fn is_hex_digit(c: wchar_t) -> bool {
        
        todo!();
            /*
                return (c >= '0' && c <= '9')
                        || (c >= 'a' && c <= 'f')
                        || (c >= 'A' && c <= 'F');
            */
    }

    pub fn parse_hex_literal<Iterator>(source: &mut Iterator) -> bool {

        todo!();
            /*
                if (source.peekNextChar() == '-')
                    source.skip();

                if (source.nextChar() != '0')
                    return false;

                auto c = source.nextChar();

                if (c != 'x' && c != 'X')
                    return false;

                int numDigits = 0;

                while (isHexDigit (source.peekNextChar()))
                {
                    ++numDigits;
                    source.skip();
                }

                if (numDigits == 0)
                    return false;

                return skipNumberSuffix (source);
            */
    }


    pub fn is_octal_digit(c: wchar_t) -> bool {
        
        todo!();
            /*
                return c >= '0' && c <= '7';
            */
    }

    pub fn parse_octal_literal<Iterator>(source: &mut Iterator) -> bool {

        todo!();
            /*
                if (source.peekNextChar() == '-')
                    source.skip();

                if (source.nextChar() != '0')
                    return false;

                if (! isOctalDigit (source.nextChar()))
                    return false;

                while (isOctalDigit (source.peekNextChar()))
                    source.skip();

                return skipNumberSuffix (source);
            */
    }

    pub fn is_decimal_digit(c: wchar_t) -> bool {
        
        todo!();
            /*
                return c >= '0' && c <= '9';
            */
    }

    pub fn parse_decimal_literal<Iterator>(source: &mut Iterator) -> bool {

        todo!();
            /*
                if (source.peekNextChar() == '-')
                    source.skip();

                int numChars = 0;
                while (isDecimalDigit (source.peekNextChar()))
                {
                    ++numChars;
                    source.skip();
                }

                if (numChars == 0)
                    return false;

                return skipNumberSuffix (source);
            */
    }

    pub fn parse_float_literal<Iterator>(source: &mut Iterator) -> bool {

        todo!();
            /*
                if (source.peekNextChar() == '-')
                    source.skip();

                int numDigits = 0;

                while (isDecimalDigit (source.peekNextChar()))
                {
                    source.skip();
                    ++numDigits;
                }

                const bool hasPoint = (source.peekNextChar() == '.');

                if (hasPoint)
                {
                    source.skip();

                    while (isDecimalDigit (source.peekNextChar()))
                    {
                        source.skip();
                        ++numDigits;
                    }
                }

                if (numDigits == 0)
                    return false;

                auto c = source.peekNextChar();
                bool hasExponent = (c == 'e' || c == 'E');

                if (hasExponent)
                {
                    source.skip();
                    c = source.peekNextChar();

                    if (c == '+' || c == '-')
                        source.skip();

                    int numExpDigits = 0;

                    while (isDecimalDigit (source.peekNextChar()))
                    {
                        source.skip();
                        ++numExpDigits;
                    }

                    if (numExpDigits == 0)
                        return false;
                }

                c = source.peekNextChar();

                if (c == 'f' || c == 'F')
                    source.skip();
                else if (! (hasExponent || hasPoint))
                    return false;

                return true;
            */
    }

    pub fn parse_number<Iterator>(source: &mut Iterator) -> i32 {

        todo!();
            /*
                const Iterator original (source);

                if (parseFloatLiteral (source))    return CPlusPlusCodeTokeniser::tokenType_float;
                source = original;

                if (parseHexLiteral (source))      return CPlusPlusCodeTokeniser::tokenType_integer;
                source = original;

                if (parseOctalLiteral (source))    return CPlusPlusCodeTokeniser::tokenType_integer;
                source = original;

                if (parseDecimalLiteral (source))  return CPlusPlusCodeTokeniser::tokenType_integer;
                source = original;

                return CPlusPlusCodeTokeniser::tokenType_error;
            */
    }

    pub fn skip_quoted_string<Iterator>(source: &mut Iterator)  {

        todo!();
            /*
                auto quote = source.nextChar();

                for (;;)
                {
                    auto c = source.nextChar();

                    if (c == quote || c == 0)
                        break;

                    if (c == '\\')
                        source.skip();
                }
            */
    }

    pub fn skip_comment<Iterator>(source: &mut Iterator)  {

        todo!();
            /*
                bool lastWasStar = false;

                for (;;)
                {
                    auto c = source.nextChar();

                    if (c == 0 || (c == '/' && lastWasStar))
                        break;

                    lastWasStar = (c == '*');
                }
            */
    }

    pub fn skip_preprocessor_line<Iterator>(source: &mut Iterator)  {

        todo!();
            /*
                bool lastWasBackslash = false;

                for (;;)
                {
                    auto c = source.peekNextChar();

                    if (c == '"')
                    {
                        skipQuotedString (source);
                        continue;
                    }

                    if (c == '/')
                    {
                        Iterator next (source);
                        next.skip();
                        auto c2 = next.peekNextChar();

                        if (c2 == '/' || c2 == '*')
                            return;
                    }

                    if (c == 0)
                        break;

                    if (c == '\n' || c == '\r')
                    {
                        source.skipToEndOfLine();

                        if (lastWasBackslash)
                            skipPreprocessorLine (source);

                        break;
                    }

                    lastWasBackslash = (c == '\\');
                    source.skip();
                }
            */
    }

    pub fn skip_if_next_char_matches<Iterator>(
            source: &mut Iterator,
            c:      wchar_t)  {

        todo!();
            /*
                if (source.peekNextChar() == c)
                    source.skip();
            */
    }

    pub fn skip_if_next_char_matches2<Iterator>(
            source: &mut Iterator,
            c1:     wchar_t,
            c2:     wchar_t)  {

        todo!();
            /*
                auto c = source.peekNextChar();

                if (c == c1 || c == c2)
                    source.skip();
            */
    }


    /**
      | Takes a UTF8 string and writes it to a
      | stream using standard C++ escape sequences
      | for any non-ascii bytes.
      | 
      | Although not strictly a tokenising
      | function, this is still a function that
      | often comes in handy when working with
      | C++ code!
      | 
      | -----------
      | @note
      | 
      | addEscapeChars() is easier to use than
      | this function if you're working with
      | Strings.
      | 
      | @see addEscapeChars
      |
      */
    pub fn write_escape_chars<W: Write>(
            out:                   &mut W,
            utf8:                  *const u8,
            num_bytes_to_read:     i32,
            max_chars_on_line:     i32,
            break_at_new_lines:    bool,
            replace_single_quotes: bool,
            allow_string_breaks:   bool)  {
        
        todo!();
            /*
                int charsOnLine = 0;
                bool lastWasHexEscapeCode = false;
                bool trigraphDetected = false;

                for (int i = 0; i < numBytesToRead || numBytesToRead < 0; ++i)
                {
                    auto c = (unsigned char) utf8[i];
                    bool startNewLine = false;

                    switch (c)
                    {

                        case '\t':  out << "\\t";  trigraphDetected = false; lastWasHexEscapeCode = false; charsOnLine += 2; break;
                        case '\r':  out << "\\r";  trigraphDetected = false; lastWasHexEscapeCode = false; charsOnLine += 2; break;
                        case '\n':  out << "\\n";  trigraphDetected = false; lastWasHexEscapeCode = false; charsOnLine += 2; startNewLine = breakAtNewLines; break;
                        case '\\':  out << "\\\\"; trigraphDetected = false; lastWasHexEscapeCode = false; charsOnLine += 2; break;
                        case '\"':  out << "\\\""; trigraphDetected = false; lastWasHexEscapeCode = false; charsOnLine += 2; break;

                        case '?':
                            if (trigraphDetected)
                            {
                                out << "\\?";
                                charsOnLine++;
                                trigraphDetected = false;
                            }
                            else
                            {
                                out << "?";
                                trigraphDetected = true;
                            }

                            lastWasHexEscapeCode = false;
                            charsOnLine++;
                            break;

                        case 0:
                            if (numBytesToRead < 0)
                                return;

                            out << "\\0";
                            lastWasHexEscapeCode = true;
                            trigraphDetected = false;
                            charsOnLine += 2;
                            break;

                        case '\'':
                            if (replaceSingleQuotes)
                            {
                                out << "\\\'";
                                lastWasHexEscapeCode = false;
                                trigraphDetected = false;
                                charsOnLine += 2;
                                break;
                            }
                            // deliberate fall-through...
                            ALOE_FALLTHROUGH

                        default:
                            if (c >= 32 && c < 127 && ! (lastWasHexEscapeCode  // (have to avoid following a hex escape sequence with a valid hex digit)
                                                           && CharacterFunctions::getHexDigitValue (c) >= 0))
                            {
                                out << (char) c;
                                lastWasHexEscapeCode = false;
                                trigraphDetected = false;
                                ++charsOnLine;
                            }
                            else if (allowStringBreaks && lastWasHexEscapeCode && c >= 32 && c < 127)
                            {
                                out << "\"\"" << (char) c;
                                lastWasHexEscapeCode = false;
                                trigraphDetected = false;
                                charsOnLine += 3;
                            }
                            else
                            {
                                out << (c < 16 ? "\\x0" : "\\x") << String::toHexString ((int) c);
                                lastWasHexEscapeCode = true;
                                trigraphDetected = false;
                                charsOnLine += 4;
                            }

                            break;
                    }

                    if ((startNewLine || (maxCharsOnLine > 0 && charsOnLine >= maxCharsOnLine))
                         && (numBytesToRead < 0 || i < numBytesToRead - 1))
                    {
                        charsOnLine = 0;
                        out << "\"" << newLine << "\"";
                        lastWasHexEscapeCode = false;
                    }
                }
            */
    }

    /**
      | Takes a string and returns a version
      | of it where standard C++ escape sequences
      | have been used to replace any non-ascii
      | bytes.
      | 
      | Although not strictly a tokenising
      | function, this is still a function that
      | often comes in handy when working with
      | C++ code!
      | 
      | @see writeEscapeChars
      |
      */
    pub fn add_escape_chars(s: &String) -> String {
        
        todo!();
            /*
                MemoryOutputStream mo;
                writeEscapeChars (mo, s.toRawUTF8(), -1, -1, false, true, true);
                return mo.toString();
            */
    }

    pub fn read_next_token_from_iterator(&mut self, source: &mut CodeDocumentIterator) -> i32 {
        
        todo!();
        /*
            return CppTokeniserFunctions::readNextToken (source);
        */
    }
    
    /**
      | This is a handy method for checking whether
      | a string is a c++ reserved keyword.
      |
      */
    pub fn is_token_reserved_keyword(&mut self, token: &String) -> bool {
        
        todo!();
        /*
            return CppTokeniserFunctions::isReservedKeyword (token.getCharPointer(), token.length());
        */
    }
}
