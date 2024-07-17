crate::ix!();

/**
  | The token values returned by this tokeniser.
  |
  */
pub enum XmlTokeniserTokenType
{
    Error = 0,
    Comment,
    Keyword,
    Operator,
    Identifier,
    String,
    Bracket,
    Punctuation,
    Preprocessor
}

pub fn skip_to_end_of_xmldtd<Iterator>(source: &mut Iterator)  {

    todo!();
        /*
            bool lastWasQuestionMark = false;

        for (;;)
        {
            auto c = source.nextChar();

            if (c == 0 || (c == '>' && lastWasQuestionMark))
                break;

            lastWasQuestionMark = (c == '?');
        }
        */
}

pub fn skip_to_end_of_xml_comment<Iterator>(source: &mut Iterator)  {

    todo!();
        /*
            aloe_wchar last[2] = {};

        for (;;)
        {
            auto c = source.nextChar();

            if (c == 0 || (c == '>' && last[0] == '-' && last[1] == '-'))
                break;

            last[1] = last[0];
            last[0] = c;
        }
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_XMLCodeTokeniser.h]

#[no_copy]
#[leak_detector]
pub struct XmlTokeniser {

}

impl CodeTokeniser for XmlTokeniser {

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
            case 0:  break;

            case '"':
            case '\'':
                CppTokeniserFunctions::skipQuotedString (source);
                return tokenType_string;

            case '<':
            {
                source.skip();
                source.skipWhitespace();
                auto nextChar = source.peekNextChar();

                if (nextChar == '?')
                {
                    source.skip();
                    skipToEndOfXmlDTD (source);
                    return tokenType_preprocessor;
                }

                if (nextChar == '!')
                {
                    source.skip();

                    if (source.peekNextChar() == '-')
                    {
                        source.skip();

                        if (source.peekNextChar() == '-')
                        {
                            skipToEndOfXmlComment (source);
                            return tokenType_comment;
                        }
                    }
                }

                CppTokeniserFunctions::skipIfNextCharMatches (source, '/');
                CppTokeniserFunctions::parseIdentifier (source);
                source.skipWhitespace();
                CppTokeniserFunctions::skipIfNextCharMatches (source, '/');
                source.skipWhitespace();
                CppTokeniserFunctions::skipIfNextCharMatches (source, '>');
                return tokenType_keyword;
            }

            case '>':
                source.skip();
                return tokenType_keyword;

            case '/':
                source.skip();
                source.skipWhitespace();
                CppTokeniserFunctions::skipIfNextCharMatches (source, '>');
                return tokenType_keyword;

            case '=':
            case ':':
                source.skip();
                return tokenType_operator;

            default:
                if (CppTokeniserFunctions::isIdentifierStart (firstChar))
                    CppTokeniserFunctions::parseIdentifier (source);

                source.skip();
                break;
        };

        return tokenType_identifier;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_XMLCodeTokeniser.cpp]
impl XmlTokeniser {
    
}
