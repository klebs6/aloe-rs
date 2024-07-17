crate::ix!();

/**
  | A position in a code document.
  | 
  | Using this class you can find a position
  | in a code document and quickly get its
  | character position, line, and index.
  | By calling setPositionMaintained
  | (true), the position is automatically
  | updated when text is inserted or deleted
  | in the document, so that it maintains
  | its original place in the text.
  |
  */
pub struct CodeDocumentPosition<'a> {
    owner:               *mut CodeDocument<'a>, // default = nullptr
    character_pos:       i32, // default = 0
    line:                i32, // default = 0
    index_in_line:       i32, // default = 0
    position_maintained: bool, // default = false
}

impl<'a> Default for CodeDocumentPosition<'a> {
    
    /**
      | Creates an uninitialised position.
      | 
      | Don't attempt to call any methods on
      | this until you've given it an owner document
      | to refer to!
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<'a> Drop for CodeDocumentPosition<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            setPositionMaintained (false);
        */
    }
}

impl<'a> PartialEq<CodeDocumentPosition<'a>> for CodeDocumentPosition<'a> {
    
    #[inline] fn eq(&self, other: &CodeDocumentPosition) -> bool {
        todo!();
        /*
            jassert ((characterPos == other.characterPos)
                   == (line == other.line && indexInLine == other.indexInLine));

        return characterPos == other.characterPos
                && line == other.line
                && indexInLine == other.indexInLine
                && owner == other.owner;
        */
    }
}

impl<'a> Eq for CodeDocumentPosition<'a> {}

impl<'a> CodeDocumentPosition<'a> {

    /**
      | Returns the position as the number of
      | characters from the start of the document.
      | @see setPosition, getLineNumber,
      | getIndexInLine
      |
      */
    pub fn get_position(&self) -> i32 {
        
        todo!();
        /*
            return characterPos;
        */
    }

    /**
      | Returns the line number of this position.
      | 
      | The first line in the document is numbered
      | zero, not one!
      |
      */
    pub fn get_line_number(&self) -> i32 {
        
        todo!();
        /*
            return line;
        */
    }

    /**
      | Returns the number of characters from
      | the start of the line.
      | 
      | -----------
      | @note
      | 
      | this value is NOT the column at which
      | the position appears in an editor.
      | 
      | If the line contains any tab characters,
      | the relationship of the index to its
      | visual position depends on the number
      | of spaces per tab being used!
      |
      */
    pub fn get_index_in_line(&self) -> i32 {
        
        todo!();
        /*
            return indexInLine;
        */
    }

    /**
      | Creates a position based on a line and
      | index in a document.
      | 
      | -----------
      | @note
      | 
      | this index is NOT the column number,
      | it's the number of characters from the
      | start of the line. The "column" number
      | isn't quite the same, because if the
      | line contains any tab characters, the
      | relationship of the index to its visual
      | column depends on the number of spaces
      | per tab being used!
      | 
      | Lines are numbered from zero, and if
      | the line or index are beyond the bounds
      | of the document, they will be adjusted
      | to keep them within its limits.
      |
      */
    pub fn new_with_line_num_and_index(
        owner_document: &CodeDocument,
        line_num:       i32,
        index:          i32) -> Self {
    
        todo!();
        /*


            : owner (const_cast<CodeDocument*> (&ownerDocument)),
          line (lineNum), indexInLine (index)

        setLineAndIndex (lineNum, index);
        */
    }
    
    /**
      | Creates a position based on a character
      | index in a document.
      | 
      | This position is placed at the specified
      | number of characters from the start
      | of the document. The line and column
      | are auto-calculated.
      | 
      | If the position is beyond the range of
      | the document, it'll be adjusted to keep
      | it inside.
      |
      */
    pub fn new_with_doc_and_pos(
        owner_document: &CodeDocument,
        pos:            i32) -> Self {
    
        todo!();
        /*


            : owner (const_cast<CodeDocument*> (&ownerDocument))

        setPosition (pos);
        */
    }
    
    /**
      | Creates a copy of another position.
      | 
      | This will copy the position, but the
      | new object will not be set to maintain
      | its position, even if the source object
      | was set to do so.
      |
      */
    pub fn new_with_os(other: &CodeDocumentPosition) -> Self {
    
        todo!();
        /*


            : owner (other.owner), characterPos (other.characterPos), line (other.line),
          indexInLine (other.indexInLine)

        jassert (*this == other);
        */
    }
    
    pub fn assign_from(&mut self, other: &CodeDocumentPosition) -> &mut CodeDocumentPosition {
        
        todo!();
        /*
            if (this != &other)
        {
            const bool wasPositionMaintained = positionMaintained;
            if (owner != other.owner)
                setPositionMaintained (false);

            owner = other.owner;
            line = other.line;
            indexInLine = other.indexInLine;
            characterPos = other.characterPos;
            setPositionMaintained (wasPositionMaintained);

            jassert (*this == other);
        }

        return *this;
        */
    }
    
    /**
      | Moves the position to a new line and index
      | within the line.
      | 
      | -----------
      | @note
      | 
      | the index is NOT the column at which the
      | position appears in an editor.
      | 
      | If the line contains any tab characters,
      | the relationship of the index to its
      | visual position depends on the number
      | of spaces per tab being used!
      | 
      | Lines are numbered from zero, and if
      | the line or index are beyond the bounds
      | of the document, they will be adjusted
      | to keep them within its limits.
      |
      */
    pub fn set_line_and_index(&mut self, 
        new_line_num:      i32,
        new_index_in_line: i32)  {
        
        todo!();
        /*
            jassert (owner != nullptr);

        if (owner->lines.size() == 0)
        {
            line = 0;
            indexInLine = 0;
            characterPos = 0;
        }
        else
        {
            if (newLineNum >= owner->lines.size())
            {
                line = owner->lines.size() - 1;

                auto& l = *owner->lines.getUnchecked (line);
                indexInLine = l.lineLengthWithoutNewLines;
                characterPos = l.lineStartInFile + indexInLine;
            }
            else
            {
                line = jmax (0, newLineNum);

                auto& l = *owner->lines.getUnchecked (line);

                if (l.lineLengthWithoutNewLines > 0)
                    indexInLine = jlimit (0, l.lineLengthWithoutNewLines, newIndexInLine);
                else
                    indexInLine = 0;

                characterPos = l.lineStartInFile + indexInLine;
            }
        }
        */
    }
    
    /**
      | Points this object at a new position
      | within the document.
      | 
      | If the position is beyond the range of
      | the document, it'll be adjusted to keep
      | it inside. @see getPosition, setLineAndIndex
      |
      */
    pub fn set_position(&mut self, new_position: i32)  {
        
        todo!();
        /*
            jassert (owner != nullptr);

        line = 0;
        indexInLine = 0;
        characterPos = 0;

        if (newPosition > 0)
        {
            int lineStart = 0;
            auto lineEnd = owner->lines.size();

            for (;;)
            {
                if (lineEnd - lineStart < 4)
                {
                    for (int i = lineStart; i < lineEnd; ++i)
                    {
                        auto& l = *owner->lines.getUnchecked (i);
                        auto index = newPosition - l.lineStartInFile;

                        if (index >= 0 && (index < l.lineLength || i == lineEnd - 1))
                        {
                            line = i;
                            indexInLine = jmin (l.lineLengthWithoutNewLines, index);
                            characterPos = l.lineStartInFile + indexInLine;
                        }
                    }

                    break;
                }
                else
                {
                    auto midIndex = (lineStart + lineEnd + 1) / 2;

                    if (newPosition >= owner->lines.getUnchecked (midIndex)->lineStartInFile)
                        lineStart = midIndex;
                    else
                        lineEnd = midIndex;
                }
            }
        }
        */
    }
    
    /**
      | Moves the position forwards or backwards
      | by the specified number of characters.
      | @see movedBy
      |
      */
    pub fn move_by(&mut self, character_delta: i32)  {
        
        todo!();
        /*
            jassert (owner != nullptr);

        if (characterDelta == 1)
        {
            setPosition (getPosition());

            // If moving right, make sure we don't get stuck between the \r and \n characters..
            if (line < owner->lines.size())
            {
                auto& l = *owner->lines.getUnchecked (line);

                if (indexInLine + characterDelta < l.lineLength
                     && indexInLine + characterDelta >= l.lineLengthWithoutNewLines + 1)
                    ++characterDelta;
            }
        }

        setPosition (characterPos + characterDelta);
        */
    }
    
    /**
      | Returns a position which is the same
      | as this one, moved by the specified number
      | of characters. @see moveBy
      |
      */
    pub fn moved_by(&self, character_delta: i32) -> CodeDocumentPosition {
        
        todo!();
        /*
            CodeDocument::CodeDocumentPosition p (*this);
        p.moveBy (characterDelta);
        return p;
        */
    }
    
    /**
      | Returns a position which is the same
      | as this one, moved up or down by the specified
      | number of lines. @see movedBy
      |
      */
    pub fn moved_by_lines(&self, delta_lines: i32) -> CodeDocumentPosition {
        
        todo!();
        /*
            CodeDocument::CodeDocumentPosition p (*this);
        p.setLineAndIndex (getLineNumber() + deltaLines, getIndexInLine());
        return p;
        */
    }
    
    /**
      | Returns the character in the document
      | at this position. @see getLineText
      |
      */
    pub fn get_character(&self) -> wchar_t {
        
        todo!();
        /*
            if (auto* l = owner->lines [line])
            return l->line [getIndexInLine()];

        return 0;
        */
    }
    
    /**
      | Returns the line from the document that
      | this position is within. @see getCharacter,
      | getLineNumber
      |
      */
    pub fn get_line_text(&self) -> String {
        
        todo!();
        /*
            if (auto* l = owner->lines [line])
            return l->line;

        return {};
        */
    }
    
    /**
      | Allows the position to be automatically
      | updated when the document changes.
      | 
      | If this is set to true, the position will
      | register with its document so that when
      | the document has text inserted or deleted,
      | this position will be automatically
      | moved to keep it at the same position
      | in the text.
      |
      */
    pub fn set_position_maintained(&mut self, is_maintained: bool)  {
        
        todo!();
        /*
            if (isMaintained != positionMaintained)
        {
            positionMaintained = isMaintained;

            if (owner != nullptr)
            {
                if (isMaintained)
                {
                    jassert (! owner->positionsToMaintain.contains (this));
                    owner->positionsToMaintain.add (this);
                }
                else
                {
                    // If this happens, you may have deleted the document while there are CodeDocumentPosition objects that are still using it...
                    jassert (owner->positionsToMaintain.contains (this));
                    owner->positionsToMaintain.removeFirstMatchingValue (this);
                }
            }
        }
        */
    }
}
