crate::ix!();

/**
  | Iterates the text in a CodeDocument.
  | 
  | This class lets you read characters
  | from a CodeDocument. It's designed
  | to be used by a CodeTokeniser object.
  | 
  | @see CodeDocument
  |
  */
pub struct CodeDocumentIterator<'a> {
    document:     *const CodeDocument<'a>,
    char_pointer: RefCell<CharPointerType>, // default = { nullptr  }
    line:         i32, // default = 0
    position:     i32, // default = 0
}

impl<'a> Default for CodeDocumentIterator<'a> {

    /**
      | Creates an uninitialised iterator.
      | 
      | Don't attempt to call any methods on
      | this until you've given it an owner document
      | to refer to!
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : document(nullptr),
        */
    }
}


impl<'a> From<&CodeDocument<'a>> for CodeDocumentIterator<'a> {

    fn from(doc: &CodeDocument<'a>) -> Self {
    
        todo!();
        /*
        : document(&doc),
        */
    }
}

impl<'a> From<CodeDocumentPosition<'a>> for CodeDocumentIterator<'a> {

    fn from(p: CodeDocumentPosition<'a>) -> Self {
    
        todo!();
        /*

            : document (p.owner),
          line (p.getLineNumber()),
          position (p.getPosition())

        reinitialiseCharPtr();

        for (int i = 0; i < p.getIndexInLine(); ++i)
        {
            charPointer.getAndAdvance();

            if (charPointer.isEmpty())
            {
                position -= (p.getIndexInLine() - i);
                break;
            }
        }
        */
    }
}

impl<'a> CodeDocumentIterator<'a> {

    /**
      | Returns the position as the number of
      | characters from the start of the document.
      |
      */
    pub fn get_position(&self) -> i32 {
        
        todo!();
        /*
            return position;
        */
    }

    /**
      | Returns the line number of the next character.
      |
      */
    pub fn get_line(&self) -> i32 {
        
        todo!();
        /*
            return line;
        */
    }

    pub fn reinitialise_char_ptr(&self) -> bool {
        
        todo!();
        /*
            /** You're trying to use a default constructed iterator. Bad idea! */
        jassert (document != nullptr);

        if (charPointer.getAddress() == nullptr)
        {
            if (auto* l = document->lines[line])
                charPointer = l->line.getCharPointer();
            else
                return false;
        }

        return true;
        */
    }
    
    /**
      | Reads the next character and returns
      | it. Returns 0 if you try to read past the
      | document's end. @see peekNextChar,
      | previousChar
      |
      */
    pub fn next_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            for (;;)
        {
            if (! reinitialiseCharPtr())
                return 0;

            if (auto result = charPointer.getAndAdvance())
            {
                if (charPointer.isEmpty())
                {
                    ++line;
                    charPointer = nullptr;
                }

                ++position;
                return result;
            }

            ++line;
            charPointer = nullptr;
        }
        */
    }
    
    /**
      | Advances the position by one character.
      |
      */
    pub fn skip(&mut self)  {
        
        todo!();
        /*
            nextChar();
        */
    }
    
    /**
      | Skips forward until the next character
      | will be the first character on the next
      | line
      |
      */
    pub fn skip_to_end_of_line(&mut self)  {
        
        todo!();
        /*
            if (! reinitialiseCharPtr())
            return;

        position += (int) charPointer.length();
        ++line;
        charPointer = nullptr;
        */
    }
    
    /**
      | Skips backward until the next character
      | will be the first character on this line
      |
      */
    pub fn skip_to_start_of_line(&mut self)  {
        
        todo!();
        /*
            if (! reinitialiseCharPtr())
            return;

        if (auto* l = document->lines [line])
        {
            auto startPtr = l->line.getCharPointer();
            position -= (int) startPtr.lengthUpTo (charPointer);
            charPointer = startPtr;
        }
        */
    }
    
    /**
      | Reads the next character without moving
      | the current position.
      |
      */
    pub fn peek_next_char(&self) -> wchar_t {
        
        todo!();
        /*
            if (! reinitialiseCharPtr())
            return 0;

        if (auto c = *charPointer)
            return c;

        if (auto* l = document->lines [line + 1])
            return l->line[0];

        return 0;
        */
    }
    
    /**
      | Reads the previous character and returns
      | it. Returns 0 if you try to read past the
      | document's start. @see isSOF, peekPreviousChar,
      | nextChar
      |
      */
    pub fn previous_char(&mut self) -> wchar_t {
        
        todo!();
        /*
            if (! reinitialiseCharPtr())
            return 0;

        for (;;)
        {
            if (auto* l = document->lines[line])
            {
                if (charPointer != l->line.getCharPointer())
                {
                    --position;
                    --charPointer;
                    break;
                }
            }

            if (line == 0)
                return 0;

            --line;

            if (auto* prev = document->lines[line])
                charPointer = prev->line.getCharPointer().findTerminatingNull();
        }

        return *charPointer;
        */
    }
    
    /**
      | Reads the next character without moving
      | the current position.
      |
      */
    pub fn peek_previous_char(&self) -> wchar_t {
        
        todo!();
        /*
            if (! reinitialiseCharPtr())
            return 0;

        if (auto* l = document->lines[line])
        {
            if (charPointer != l->line.getCharPointer())
                return *(charPointer - 1);

            if (auto* prev = document->lines[line - 1])
                return *(prev->line.getCharPointer().findTerminatingNull() - 1);
        }

        return 0;
        */
    }
    
    /**
      | Skips over any whitespace characters
      | until the next character is non-whitespace.
      |
      */
    pub fn skip_whitespace(&mut self)  {
        
        todo!();
        /*
            while (CharacterFunctions::isWhitespace (peekNextChar()))
            skip();
        */
    }
    
    /**
      | Returns true if the iterator has reached
      | the end of the document.
      |
      */
    pub fn iseof(&self) -> bool {
        
        todo!();
        /*
            return charPointer.getAddress() == nullptr && line >= document->lines.size();
        */
    }
    
    /**
      | Returns true if the iterator is at the
      | start of the document.
      |
      */
    pub fn issof(&self) -> bool {
        
        todo!();
        /*
            return position == 0;
        */
    }
    
    /**
      | Convert this iterator to a CodeDocument::CodeDocumentPosition.
      |
      */
    pub fn to_position(&self) -> CodeDocumentPosition {
        
        todo!();
        /*
            if (auto* l = document->lines[line])
        {
            reinitialiseCharPtr();
            int indexInLine = 0;
            auto linePtr = l->line.getCharPointer();

            while (linePtr != charPointer && ! linePtr.isEmpty())
            {
                ++indexInLine;
                ++linePtr;
            }

            return CodeDocument::CodeDocumentPosition (*document, line, indexInLine);
        }

        if (isEOF())
        {
            if (auto* last = document->lines.getLast())
            {
                auto lineIndex = document->lines.size() - 1;
                return CodeDocument::CodeDocumentPosition (*document, lineIndex, last->lineLength);
            }
        }

        return CodeDocument::CodeDocumentPosition (*document, 0, 0);
        */
    }
}
