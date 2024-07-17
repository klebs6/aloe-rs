crate::ix!();

/**
  | Attempts to parse some XML text, returning
  | a new XmlElement if it was valid.
  | 
  | If the parse fails, this will return
  | a nullptr - if you need more information
  | about errors or more parsing options,
  | see the XmlDocument class instead.
  | @see XmlDocument, parseXMLIfTagMatches
  |
  */
pub fn parse_xml_from_string(text_to_parse: &String) -> Box<XmlElement> {
    
    todo!();
    /*
    
    */
}

/**
  | Attempts to parse some XML text, returning
  | a new XmlElement if it was valid.
  | 
  | If the parse fails, this will return
  | a nullptr - if you need more information
  | about errors or more parsing options,
  | see the XmlDocument class instead.
  | @see XmlDocument, parseXMLIfTagMatches
  |
  */
pub fn parse_xml_from_file(file_to_parse: &std::fs::File) -> Box<XmlElement> {
    
    todo!();
    /*
    
    */
}

/**
  | Does an inexpensive check to see whether
  | the top-level element has the given
  | tag name, and if that's true, does a full
  | parse and returns the result.
  | 
  | If the outer tag doesn't match, or the
  | XML has errors, this will return nullptr;
  | @see parseXML
  |
  */
pub fn parse_xml_from_text_if_tag_matches(
        text_to_parse: &String,
        required_tag:  &str) -> Box<XmlElement> {
    
    todo!();
    /*
    
    */
}

/**
  | Does an inexpensive check to see whether
  | the top-level element has the given
  | tag name, and if that's true, does a full
  | parse and returns the result.
  | 
  | If the outer tag doesn't match, or the
  | XML has errors, this will return nullptr;
  | @see parseXML
  |
  */
pub fn parse_xml_from_file_if_tag_matches(
        file_to_parse: &std::fs::File,
        required_tag:  &str) -> Box<XmlElement> {
    
    todo!();
    /*
    
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/xml/aloe_XmlDocument.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/xml/aloe_XmlDocument.cpp]

/**
  | Parses a text-based XML document and creates
  | an XmlElement object from it.
  |
  | The parser will parse DTDs to load external
  | entities but won't check the document for
  | validity against the DTD.
  |
  | e.g.
  | @code
  | XmlDocument myDocument (File ("myfile.xml"));
  |
  | if (auto mainElement = myDocument.getDocumentElement())
  | {
  |     ..use the element
  | }
  | else
  | {
  |     String error = myDocument.getLastParseError();
  | }
  | @endcode
  |
  | Or you can use the helper functions for much
  | less verbose parsing..
  |
  | @code
  | if (auto xml = parseXML (myXmlFile))
  | {
  |     if (xml->hasTagName ("foobar"))
  |     {
  |         ...etc
  |     }
  | }
  | @endcode
  |
  | @see XmlElement
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct XmlDocument {
    original_text:              String,
    input:                      CharPointerType, // default = nullptr 
    out_of_data:                bool, // default = false
    error_occurred:             bool, // default = false
    last_error:                 String,
    dtd_text:                   String,
    tokeniseddtd:               Vec<String>,
    need_to_loaddtd:            bool, // default = false
    ignore_empty_text_elements: bool, // default = true
    input_source:               Box<dyn Read>,
}

impl XmlDocument {

    /**
      | Creates an XmlDocument from the xml
      | text.
      | 
      | The text doesn't actually get parsed
      | until the getDocumentElement() method
      | is called.
      |
      */
    pub fn new_from_text(text: &String) -> Self {
    
        todo!();
        /*
        : original_text(text),

        
        */
    }
    
    /**
      | Creates an XmlDocument from a file.
      | 
      | The text doesn't actually get parsed
      | until the getDocumentElement() method
      | is called.
      |
      */
    pub fn new_from_file(file: &std::fs::File) -> Self {
    
        todo!();
        /*


            : inputSource (new FileInputSource (file))
        */
    }
    
    /**
      | A handy static method that parses a file.
      | 
      | This is a shortcut for creating an XmlDocument
      | object and calling getDocumentElement()
      | on it.
      | 
      | -----------
      | @return
      | 
      | a new XmlElement, or nullptr if there
      | was an error.
      |
      */
    pub fn parse_file(&mut self, file: &std::fs::File) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (file).getDocumentElement();
        */
    }
    
    /**
      | A handy static method that parses some
      | XML data.
      | 
      | This is a shortcut for creating an XmlDocument
      | object and calling getDocumentElement()
      | on it.
      | 
      | -----------
      | @return
      | 
      | a new XmlElement, or nullptr if there
      | was an error.
      |
      */
    pub fn parse_string(&mut self, text_to_parse: &String) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (textToParse).getDocumentElement();
        */
    }
    
    pub fn parse_xml_string(&mut self, text_to_parse: &String) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (textToParse).getDocumentElement();
        */
    }
    
    pub fn parse_xml_file(&mut self, file: &std::fs::File) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (file).getDocumentElement();
        */
    }
    
    pub fn parse_xml_from_text_if_tag_matches(
        &mut self, 
        text_to_parse: &String,
        required_tag:  &str) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (textToParse).getDocumentElementIfTagMatches (requiredTag);
        */
    }
    
    pub fn parse_xml_from_file_if_tag_matches(
        &mut self, 
        file:         &std::fs::File,
        required_tag: &str) -> Box<XmlElement> {
        
        todo!();
        /*
            return XmlDocument (file).getDocumentElementIfTagMatches (requiredTag);
        */
    }
    
    /**
      | Sets an input source object to use for
      | parsing documents that reference external
      | entities.
      | 
      | If the document has been created from
      | a file, this probably won't be needed,
      | but if you're parsing some text and there
      | might be a DTD that references external
      | files, you may need to create a custom
      | input source that can retrieve the other
      | files it needs.
      | 
      | The object that is passed-in will be
      | deleted automatically when no longer
      | needed.
      | 
      | @see InputSource
      |
      */
    pub fn set_input_source<R: Read>(&mut self, new_source: *mut R)  {
        
        todo!();
        /*
            inputSource.reset (newSource);
        */
    }
    
    /**
      | Sets a flag to change the treatment of
      | empty text elements.
      | 
      | If this is true (the default state),
      | then any text elements that contain
      | only whitespace characters will be
      | ingored during parsing. If you need
      | to catch whitespace-only text, then
      | you should set this to false before calling
      | the getDocumentElement() method.
      |
      */
    pub fn set_empty_text_elements_ignored(&mut self, should_be_ignored: bool)  {
        
        todo!();
        /*
            ignoreEmptyTextElements = shouldBeIgnored;
        */
    }
}

pub mod xml_identifier_chars {

    use super::*;

    pub fn is_identifier_char_slow(c: wchar_t) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetterOrDigit (c)
                         || c == '_' || c == '-' || c == ':' || c == '.';
        */
    }

    pub fn is_identifier_char(c: wchar_t) -> bool {
        
        todo!();
        /*
            static const uint32 legalChars[] = { 0, 0x7ff6000, 0x87fffffe, 0x7fffffe, 0 };

                return ((int) c < (int) numElementsInArray (legalChars) * 32) ? ((legalChars [c >> 5] & (uint32) (1 << (c & 31))) != 0)
                                                                              : isIdentifierCharSlow (c);
        */
    }

    pub fn find_end_of_token(p: CharPointerType) -> CharPointerType {
        
        todo!();
        /*
            while (isIdentifierChar (*p))
                    ++p;

                return p;
        */
    }
}

impl XmlDocument {
    
    /**
      | Creates an XmlElement object to represent
      | the main document node.
      | 
      | This method will do the actual parsing
      | of the text, and if there's a parse error,
      | it may returns nullptr (and you can find
      | out the error using the getLastParseError()
      | method).
      | 
      | See also the parse() methods, which
      | provide a shorthand way to quickly parse
      | a file or string.
      | 
      | -----------
      | @param onlyReadOuterDocumentElement
      | 
      | if true, the parser will only read the
      | first section of the file, and will only
      | return the outer document element -
      | this allows quick checking of large
      | files to see if they contain the correct
      | type of tag, without having to parse
      | the entire file
      | 
      | -----------
      | @return
      | 
      | a new XmlElement, or nullptr if there
      | was an error. @see getLastParseError,
      | getDocumentElementIfTagMatches
      |
      */
    pub fn get_document_element(
        &mut self, 
        only_read_outer_document_element: Option<bool>
    ) -> Box<XmlElement> {
        
        let only_read_outer_document_element: bool = only_read_outer_document_element.unwrap_or(false);

        todo!();
        /*
            if (originalText.isEmpty() && inputSource != nullptr)
        {
            std::unique_ptr<InputStream> in (inputSource->createInputStream());

            if (in != nullptr)
            {
                MemoryOutputStream data;
                data.writeFromInputStream (*in, onlyReadOuterDocumentElement ? 8192 : -1);

               #if ALOE_STRING_UTF_TYPE == 8
                if (data.getDataSize() > 2)
                {
                    data.writeByte (0);
                    auto* text = static_cast<const char*> (data.getData());

                    if (CharPointer_UTF16::isByteOrderMarkBigEndian (text)
                          || CharPointer_UTF16::isByteOrderMarkLittleEndian (text))
                    {
                        originalText = data.toString();
                    }
                    else
                    {
                        if (CharPointer_UTF8::isByteOrderMark (text))
                            text += 3;

                        // parse the input buffer directly to avoid copying it all to a string..
                        return parseDocumentElement (CharPointerType (text), onlyReadOuterDocumentElement);
                    }
                }
               #else
                originalText = data.toString();
               #endif
            }
        }

        return parseDocumentElement (originalText.getCharPointer(), onlyReadOuterDocumentElement);
        */
    }
    
    /**
      | Does an inexpensive check to see whether
      | the outer element has the given tag name,
      | and then does a full parse if it matches.
      | 
      | If the tag is different, or the XML parse
      | fails, this will return nullptr.
      |
      */
    pub fn get_document_element_if_tag_matches(&mut self, required_tag: &str) -> Box<XmlElement> {
        
        todo!();
        /*
            if (auto xml = getDocumentElement (true))
            if (xml->hasTagName (requiredTag))
                return getDocumentElement (false);

        return {};
        */
    }
    
    /**
      | Returns the parsing error that occurred
      | the last time getDocumentElement was
      | called.
      | 
      | -----------
      | @return
      | 
      | the error, or an empty string if there
      | was no error.
      |
      */
    pub fn get_last_parse_error(&self) -> &String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn set_last_error(&mut self, 
        desc:     &String,
        carry_on: bool)  {
        
        todo!();
        /*
            lastError = desc;
        errorOccurred = ! carryOn;
        */
    }
    
    pub fn get_file_contents(&self, filename: &String) -> String {
        
        todo!();
        /*
            if (inputSource != nullptr)
        {
            std::unique_ptr<InputStream> in (inputSource->createInputStreamFor (filename.trim().unquoted()));

            if (in != nullptr)
                return in->readEntireStreamAsString();
        }

        return {};
        */
    }
    
    pub fn read_next_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            auto c = input.getAndAdvance();

        if (c == 0)
        {
            outOfData = true;
            --input;
        }

        return c;
        */
    }
    
    pub fn parse_document_element(&mut self, 
        text_to_parse:                    CharPointerType,
        only_read_outer_document_element: bool) -> Box<XmlElement> {
        
        todo!();
        /*
            input = textToParse;
        errorOccurred = false;
        outOfData = false;
        needToLoadDTD = true;

        if (textToParse.isEmpty())
        {
            lastError = "not enough input";
        }
        else if (! parseHeader())
        {
            lastError = "malformed header";
        }
        else if (! parseDTD())
        {
            lastError = "malformed DTD";
        }
        else
        {
            lastError.clear();
            std::unique_ptr<XmlElement> result (readNextElement (! onlyReadOuterDocumentElement));

            if (! errorOccurred)
                return result;
        }

        return {};
        */
    }
    
    pub fn parse_header(&mut self) -> bool {
        
        todo!();
        /*
            skipNextWhiteSpace();

        if (CharacterFunctions::compareUpTo (input, CharPointer_ASCII ("<?xml"), 5) == 0)
        {
            auto headerEnd = CharacterFunctions::find (input, CharPointer_ASCII ("?>"));

            if (headerEnd.isEmpty())
                return false;

           #if ALOE_DEBUG
            auto encoding = String (input, headerEnd)
                              .fromFirstOccurrenceOf ("encoding", false, true)
                              .fromFirstOccurrenceOf ("=", false, false)
                              .fromFirstOccurrenceOf ("\"", false, false)
                              .upToFirstOccurrenceOf ("\"", false, false)
                              .trim();

            /* If you load an XML document with a non-UTF encoding type, it may have been
               loaded wrongly.. Since all the files are read via the normal aloe file streams,
               they're treated as UTF-8, so by the time it gets to the parser, the encoding will
               have been lost. Best plan is to stick to utf-8 or if you have specific files to
               read, use your own code to convert them to a unicode String, and pass that to the
               XML parser.
            */
            jassert (encoding.isEmpty() || encoding.startsWithIgnoreCase ("utf-"));
           #endif

            input = headerEnd + 2;
            skipNextWhiteSpace();
        }

        return true;
        */
    }
    
    pub fn parsedtd(&mut self) -> bool {
        
        todo!();
        /*
            if (CharacterFunctions::compareUpTo (input, CharPointer_ASCII ("<!DOCTYPE"), 9) == 0)
        {
            input += 9;
            auto dtdStart = input;

            for (int n = 1; n > 0;)
            {
                auto c = readNextChar();

                if (outOfData)
                    return false;

                if (c == '<')
                    ++n;
                else if (c == '>')
                    --n;
            }

            dtdText = String (dtdStart, input - 1).trim();
        }

        return true;
        */
    }
    
    pub fn skip_next_white_space(&mut self)  {
        
        todo!();
        /*
            for (;;)
        {
            input.incrementToEndOfWhitespace();

            if (input.isEmpty())
            {
                outOfData = true;
                break;
            }

            if (*input == '<')
            {
                if (input[1] == '!'
                     && input[2] == '-'
                     && input[3] == '-')
                {
                    input += 4;
                    auto closeComment = input.indexOf (CharPointer_ASCII ("-->"));

                    if (closeComment < 0)
                    {
                        outOfData = true;
                        break;
                    }

                    input += closeComment + 3;
                    continue;
                }

                if (input[1] == '?')
                {
                    input += 2;
                    auto closeBracket = input.indexOf (CharPointer_ASCII ("?>"));

                    if (closeBracket < 0)
                    {
                        outOfData = true;
                        break;
                    }

                    input += closeBracket + 2;
                    continue;
                }
            }

            break;
        }
        */
    }
    
    pub fn read_quoted_string(&mut self, result: &mut String)  {
        
        todo!();
        /*
            auto quote = readNextChar();

        while (! outOfData)
        {
            auto c = readNextChar();

            if (c == quote)
                break;

            --input;

            if (c == '&')
            {
                readEntity (result);
            }
            else
            {
                auto start = input;

                for (;;)
                {
                    auto character = *input;

                    if (character == quote)
                    {
                        result.appendCharPointer (start, input);
                        ++input;
                        return;
                    }

                    if (character == '&')
                    {
                        result.appendCharPointer (start, input);
                        break;
                    }

                    if (character == 0)
                    {
                        setLastError ("unmatched quotes", false);
                        outOfData = true;
                        break;
                    }

                    ++input;
                }
            }
        }
        */
    }
    
    pub fn read_next_element(&mut self, also_parse_sub_elements: bool) -> *mut XmlElement {
        
        todo!();
        /*
            XmlElement* node = nullptr;
        skipNextWhiteSpace();

        if (outOfData)
            return nullptr;

        if (*input == '<')
        {
            ++input;
            auto endOfToken = XmlIdentifierChars::findEndOfToken (input);

            if (endOfToken == input)
            {
                // no tag name - but allow for a gap after the '<' before giving an error
                skipNextWhiteSpace();
                endOfToken = XmlIdentifierChars::findEndOfToken (input);

                if (endOfToken == input)
                {
                    setLastError ("tag name missing", false);
                    return node;
                }
            }

            node = new XmlElement (input, endOfToken);
            input = endOfToken;
            LinkedListPointer<XmlElement::XmlAttributeNode>::Appender attributeAppender (node->attributes);

            // look for attributes
            for (;;)
            {
                skipNextWhiteSpace();
                auto c = *input;

                // empty tag..
                if (c == '/' && input[1] == '>')
                {
                    input += 2;
                    break;
                }

                // parse the guts of the element..
                if (c == '>')
                {
                    ++input;

                    if (alsoParseSubElements)
                        readChildElements (*node);

                    break;
                }

                // get an attribute..
                if (XmlIdentifierChars::isIdentifierChar (c))
                {
                    auto attNameEnd = XmlIdentifierChars::findEndOfToken (input);

                    if (attNameEnd != input)
                    {
                        auto attNameStart = input;
                        input = attNameEnd;
                        skipNextWhiteSpace();

                        if (readNextChar() == '=')
                        {
                            skipNextWhiteSpace();
                            auto nextChar = *input;

                            if (nextChar == '"' || nextChar == '\'')
                            {
                                auto* newAtt = new XmlElement::XmlAttributeNode (attNameStart, attNameEnd);
                                readQuotedString (newAtt->value);
                                attributeAppender.append (newAtt);
                                continue;
                            }
                        }
                        else
                        {
                            setLastError ("expected '=' after attribute '"
                                            + String (attNameStart, attNameEnd) + "'", false);
                            return node;
                        }
                    }
                }
                else
                {
                    if (! outOfData)
                        setLastError ("illegal character found in " + node->getTagName() + ": '" + c + "'", false);
                }

                break;
            }
        }

        return node;
        */
    }
    
    pub fn read_child_elements(&mut self, parent: &mut XmlElement)  {
        
        todo!();
        /*
            LinkedListPointer<XmlElement>::Appender childAppender (parent.firstChildElement);

        for (;;)
        {
            auto preWhitespaceInput = input;
            skipNextWhiteSpace();

            if (outOfData)
            {
                setLastError ("unmatched tags", false);
                break;
            }

            if (*input == '<')
            {
                auto c1 = input[1];

                if (c1 == '/')
                {
                    // our close tag..
                    auto closeTag = input.indexOf ((aloe_wchar) '>');

                    if (closeTag >= 0)
                        input += closeTag + 1;

                    break;
                }

                if (c1 == '!' && CharacterFunctions::compareUpTo (input + 2, CharPointer_ASCII ("[CDATA["), 7) == 0)
                {
                    input += 9;
                    auto inputStart = input;

                    for (;;)
                    {
                        auto c0 = *input;

                        if (c0 == 0)
                        {
                            setLastError ("unterminated CDATA section", false);
                            outOfData = true;
                            break;
                        }

                        if (c0 == ']' && input[1] == ']' && input[2] == '>')
                        {
                            childAppender.append (XmlElement::createTextElement (String (inputStart, input)));
                            input += 3;
                            break;
                        }

                        ++input;
                    }
                }
                else
                {
                    // this is some other element, so parse and add it..
                    if (auto* n = readNextElement (true))
                        childAppender.append (n);
                    else
                        break;
                }
            }
            else  // must be a character block
            {
                input = preWhitespaceInput; // roll back to include the leading whitespace
                MemoryOutputStream textElementContent;
                bool contentShouldBeUsed = ! ignoreEmptyTextElements;

                for (;;)
                {
                    auto c = *input;

                    if (c == '<')
                    {
                        if (input[1] == '!' && input[2] == '-' && input[3] == '-')
                        {
                            input += 4;
                            auto closeComment = input.indexOf (CharPointer_ASCII ("-->"));

                            if (closeComment < 0)
                            {
                                setLastError ("unterminated comment", false);
                                outOfData = true;
                                return;
                            }

                            input += closeComment + 3;
                            continue;
                        }

                        break;
                    }

                    if (c == 0)
                    {
                        setLastError ("unmatched tags", false);
                        outOfData = true;
                        return;
                    }

                    if (c == '&')
                    {
                        String entity;
                        readEntity (entity);

                        if (entity.startsWithChar ('<') && entity [1] != 0)
                        {
                            auto oldInput = input;
                            auto oldOutOfData = outOfData;

                            input = entity.getCharPointer();
                            outOfData = false;

                            while (auto* n = readNextElement (true))
                                childAppender.append (n);

                            input = oldInput;
                            outOfData = oldOutOfData;
                        }
                        else
                        {
                            textElementContent << entity;
                            contentShouldBeUsed = contentShouldBeUsed || entity.containsNonWhitespaceChars();
                        }
                    }
                    else
                    {
                        for (;; ++input)
                        {
                            auto nextChar = *input;

                            if (nextChar == '\r')
                            {
                                nextChar = '\n';

                                if (input[1] == '\n')
                                    continue;
                            }

                            if (nextChar == '<' || nextChar == '&')
                                break;

                            if (nextChar == 0)
                            {
                                setLastError ("unmatched tags", false);
                                outOfData = true;
                                return;
                            }

                            textElementContent.appendUTF8Char (nextChar);
                            contentShouldBeUsed = contentShouldBeUsed || ! CharacterFunctions::isWhitespace (nextChar);
                        }
                    }
                }

                if (contentShouldBeUsed)
                    childAppender.append (XmlElement::createTextElement (textElementContent.toUTF8()));
            }
        }
        */
    }
    
    pub fn read_entity(&mut self, result: &mut String)  {
        
        todo!();
        /*
            // skip over the ampersand
        ++input;

        if (input.compareIgnoreCaseUpTo (CharPointer_ASCII ("amp;"), 4) == 0)
        {
            input += 4;
            result += '&';
        }
        else if (input.compareIgnoreCaseUpTo (CharPointer_ASCII ("quot;"), 5) == 0)
        {
            input += 5;
            result += '"';
        }
        else if (input.compareIgnoreCaseUpTo (CharPointer_ASCII ("apos;"), 5) == 0)
        {
            input += 5;
            result += '\'';
        }
        else if (input.compareIgnoreCaseUpTo (CharPointer_ASCII ("lt;"), 3) == 0)
        {
            input += 3;
            result += '<';
        }
        else if (input.compareIgnoreCaseUpTo (CharPointer_ASCII ("gt;"), 3) == 0)
        {
            input += 3;
            result += '>';
        }
        else if (*input == '#')
        {
            int64_t charCode = 0;
            ++input;

            if (*input == 'x' || *input == 'X')
            {
                ++input;
                int numChars = 0;

                while (input[0] != ';')
                {
                    auto hexValue = CharacterFunctions::getHexDigitValue (input[0]);

                    if (hexValue < 0 || ++numChars > 8)
                    {
                        setLastError ("illegal escape sequence", true);
                        break;
                    }

                    charCode = (charCode << 4) | hexValue;
                    ++input;
                }

                ++input;
            }
            else if (input[0] >= '0' && input[0] <= '9')
            {
                int numChars = 0;

                for (;;)
                {
                    const auto firstChar = input[0];

                    if (firstChar == 0)
                    {
                        setLastError ("unexpected end of input", true);
                        return;
                    }

                    if (firstChar == ';')
                        break;

                    if (++numChars > 12)
                    {
                        setLastError ("illegal escape sequence", true);
                        break;
                    }

                    charCode = charCode * 10 + ((int) firstChar - '0');
                    ++input;
                }

                ++input;
            }
            else
            {
                setLastError ("illegal escape sequence", true);
                result += '&';
                return;
            }

            result << (aloe_wchar) charCode;
        }
        else
        {
            auto entityNameStart = input;
            auto closingSemiColon = input.indexOf ((aloe_wchar) ';');

            if (closingSemiColon < 0)
            {
                outOfData = true;
                result += '&';
            }
            else
            {
                input += closingSemiColon + 1;
                result += expandExternalEntity (String (entityNameStart, (size_t) closingSemiColon));
            }
        }
        */
    }
    
    pub fn expand_entity(&mut self, ent: &String) -> String {
        
        todo!();
        /*
            if (ent.equalsIgnoreCase ("amp"))   return String::charToString ('&');
        if (ent.equalsIgnoreCase ("quot"))  return String::charToString ('"');
        if (ent.equalsIgnoreCase ("apos"))  return String::charToString ('\'');
        if (ent.equalsIgnoreCase ("lt"))    return String::charToString ('<');
        if (ent.equalsIgnoreCase ("gt"))    return String::charToString ('>');

        if (ent[0] == '#')
        {
            auto char1 = ent[1];

            if (char1 == 'x' || char1 == 'X')
                return String::charToString (static_cast<aloe_wchar> (ent.substring (2).getHexValue32()));

            if (char1 >= '0' && char1 <= '9')
                return String::charToString (static_cast<aloe_wchar> (ent.substring (1).getIntValue()));

            setLastError ("illegal escape sequence", false);
            return String::charToString ('&');
        }

        return expandExternalEntity (ent);
        */
    }
    
    pub fn expand_external_entity(&mut self, entity: &String) -> String {
        
        todo!();
        /*
            if (needToLoadDTD)
        {
            if (dtdText.isNotEmpty())
            {
                dtdText = dtdText.trimCharactersAtEnd (">");
                tokenisedDTD.addTokens (dtdText, true);

                if (tokenisedDTD[tokenisedDTD.size() - 2].equalsIgnoreCase ("system")
                     && tokenisedDTD[tokenisedDTD.size() - 1].isQuotedString())
                {
                    auto fn = tokenisedDTD[tokenisedDTD.size() - 1];

                    tokenisedDTD.clear();
                    tokenisedDTD.addTokens (getFileContents (fn), true);
                }
                else
                {
                    tokenisedDTD.clear();
                    auto openBracket = dtdText.indexOfChar ('[');

                    if (openBracket > 0)
                    {
                        auto closeBracket = dtdText.lastIndexOfChar (']');

                        if (closeBracket > openBracket)
                            tokenisedDTD.addTokens (dtdText.substring (openBracket + 1,
                                                                       closeBracket), true);
                    }
                }

                for (int i = tokenisedDTD.size(); --i >= 0;)
                {
                    if (tokenisedDTD[i].startsWithChar ('%')
                         && tokenisedDTD[i].endsWithChar (';'))
                    {
                        auto parsed = getParameterEntity (tokenisedDTD[i].substring (1, tokenisedDTD[i].length() - 1));
                        StringArray newToks;
                        newToks.addTokens (parsed, true);

                        tokenisedDTD.remove (i);

                        for (int j = newToks.size(); --j >= 0;)
                            tokenisedDTD.insert (i, newToks[j]);
                    }
                }
            }

            needToLoadDTD = false;
        }

        for (int i = 0; i < tokenisedDTD.size(); ++i)
        {
            if (tokenisedDTD[i] == entity)
            {
                if (tokenisedDTD[i - 1].equalsIgnoreCase ("<!entity"))
                {
                    auto ent = tokenisedDTD [i + 1].trimCharactersAtEnd (">").trim().unquoted();

                    // check for sub-entities..
                    auto ampersand = ent.indexOfChar ('&');

                    while (ampersand >= 0)
                    {
                        auto semiColon = ent.indexOf (i + 1, ";");

                        if (semiColon < 0)
                        {
                            setLastError ("entity without terminating semi-colon", false);
                            break;
                        }

                        auto resolved = expandEntity (ent.substring (i + 1, semiColon));

                        ent = ent.substring (0, ampersand)
                               + resolved
                               + ent.substring (semiColon + 1);

                        ampersand = ent.indexOfChar (semiColon + 1, '&');
                    }

                    return ent;
                }
            }
        }

        setLastError ("unknown entity", true);
        return entity;
        */
    }
    
    pub fn get_parameter_entity(&mut self, entity: &String) -> String {
        
        todo!();
        /*
            for (int i = 0; i < tokenisedDTD.size(); ++i)
        {
            if (tokenisedDTD[i] == entity
                 && tokenisedDTD [i - 1] == "%"
                 && tokenisedDTD [i - 2].equalsIgnoreCase ("<!entity"))
            {
                auto ent = tokenisedDTD [i + 1].trimCharactersAtEnd (">");

                if (ent.equalsIgnoreCase ("system"))
                    return getFileContents (tokenisedDTD [i + 2].trimCharactersAtEnd (">"));

                return ent.trim().unquoted();
            }
        }

        return entity;
        */
    }
}
