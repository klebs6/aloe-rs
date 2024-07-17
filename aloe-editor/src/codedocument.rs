crate::ix!();

pub fn get_character_type(character: wchar_t) -> i32 {
    
    todo!();
        /*
            return (CharacterFunctions::isLetterOrDigit (character) || character == '_')
                    ? 2 : (CharacterFunctions::isWhitespace (character) ? 0 : 1);
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CodeDocument.h]

/**
  | A class for storing and manipulating
  | a source code file.
  | 
  | When using a CodeEditorComponent,
  | it takes one of these as its source object.
  | 
  | The CodeDocument stores its content
  | as an array of lines, which makes it quick
  | to insert and delete.
  | 
  | @see CodeEditorComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct CodeDocument<'a> {
    lines:                 Vec<Box<CodeDocumentLine>>,
    positions_to_maintain: Vec<*mut CodeDocumentPosition<'a>>,
    undo_manager:          UndoManager<'a>,
    current_action_index:  i32, // default = 0
    index_of_saved_state:  i32, // default = -1
    maximum_line_length:   i32, // default = -1
    listeners:             ListenerList<Rc<RefCell<dyn CodeDocumentListener>>>,
    new_line_chars:        String, // default = { "\r\n"  }
}

impl<'a> Default for CodeDocument<'a> {
    
    /**
      | Creates a new, empty document.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CodeDocument.cpp]
impl<'a> CodeDocument<'a> {
    
    /**
      | Returns the number of lines in the document.
      |
      */
    pub fn get_num_lines(&self) -> i32 {
        
        todo!();
        /*
            return lines.size();
        */
    }

    /**
      | Returns the preferred new-line characters
      | for the document.
      | 
      | This will be either "\\n", "\\r\\n",
      | or (rarely) "\\r". @see setNewLineCharacters
      |
      */
    pub fn get_new_line_characters(&self) -> String {
        
        todo!();
        /*
            return newLineChars;
        */
    }

    /**
      | Returns the document's UndoManager
      |
      */
    pub fn get_undo_manager(&mut self) -> &mut UndoManager {
        
        todo!();
        /*
            return undoManager;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
            : undoManager (std::numeric_limits<int>::max(), 10000)
        */
    }
    
    /**
      | Returns the full text of the document.
      |
      */
    pub fn get_all_content(&self) -> String {
        
        todo!();
        /*
            return getTextBetween (CodeDocumentPosition (*this, 0),
                               CodeDocumentPosition (*this, lines.size(), 0));
        */
    }
    
    /**
      | Returns a section of the document's
      | text.
      |
      */
    pub fn get_text_between(&self, 
        start: &CodeDocumentPosition,
        end:   &CodeDocumentPosition) -> String {
        
        todo!();
        /*
            if (end.getPosition() <= start.getPosition())
            return {};

        auto startLine = start.getLineNumber();
        auto endLine = end.getLineNumber();

        if (startLine == endLine)
        {
            if (auto* line = lines [startLine])
                return line->line.substring (start.getIndexInLine(), end.getIndexInLine());

            return {};
        }

        MemoryOutputStream mo;
        mo.preallocate ((size_t) (end.getPosition() - start.getPosition() + 4));

        auto maxLine = jmin (lines.size() - 1, endLine);

        for (int i = jmax (0, startLine); i <= maxLine; ++i)
        {
            auto& line = *lines.getUnchecked(i);
            auto len = line.lineLength;

            if (i == startLine)
            {
                auto index = start.getIndexInLine();
                mo << line.line.substring (index, len);
            }
            else if (i == endLine)
            {
                len = end.getIndexInLine();
                mo << line.line.substring (0, len);
            }
            else
            {
                mo << line.line;
            }
        }

        return mo.toUTF8();
        */
    }
    
    /**
      | Returns the number of characters in
      | the document.
      |
      */
    pub fn get_num_characters(&self) -> i32 {
        
        todo!();
        /*
            if (auto* lastLine = lines.getLast())
            return lastLine->lineStartInFile + lastLine->lineLength;

        return 0;
        */
    }
    
    /**
      | Returns a line from the document.
      |
      */
    pub fn get_line(&self, line_index: i32) -> String {
        
        todo!();
        /*
            if (auto* line = lines[lineIndex])
            return line->line;

        return {};
        */
    }
    
    /**
      | Returns the number of characters in
      | the longest line of the document.
      |
      */
    pub fn get_maximum_line_length(&mut self) -> i32 {
        
        todo!();
        /*
            if (maximumLineLength < 0)
        {
            maximumLineLength = 0;

            for (auto* l : lines)
                maximumLineLength = jmax (maximumLineLength, l->lineLength);
        }

        return maximumLineLength;
        */
    }
    
    /**
      | Deletes a section of the text.
      | 
      | This operation is undoable.
      |
      */
    pub fn delete_section(&mut self, 
        start_position: &CodeDocumentPosition,
        end_position:   &CodeDocumentPosition)  {
        
        todo!();
        /*
            deleteSection (startPosition.getPosition(), endPosition.getPosition());
        */
    }
    
    /**
      | Deletes a section of the text.
      | 
      | This operation is undoable.
      |
      */
    pub fn delete_section_form_start_end(
        &mut self, 
        start: i32,
        end:   i32)  {
        
        todo!();
        /*
            remove (start, end, true);
        */
    }
    
    /**
      | Inserts some text into the document
      | at a given position.
      | 
      | This operation is undoable.
      |
      */
    pub fn insert_text(&mut self, 
        position: &CodeDocumentPosition,
        text:     &String)  {
        
        todo!();
        /*
            insertText (position.getPosition(), text);
        */
    }
    
    /**
      | Inserts some text into the document
      | at a given position.
      | 
      | This operation is undoable.
      |
      */
    pub fn insert_text_at_index(
        &mut self, 
        insert_index: i32,
        text:         &String)  {
        
        todo!();
        /*
            insert (text, insertIndex, true);
        */
    }
    
    /**
      | Replaces a section of the text with a
      | new string.
      | 
      | This operation is undoable.
      |
      */
    pub fn replace_section(&mut self, 
        start:    i32,
        end:      i32,
        new_text: &String)  {
        
        todo!();
        /*
            insertText (end, newText);
        deleteSection (start, end);
        */
    }
    
    /**
      | Analyses the changes between the current
      | content and some new text, and applies
      | those changes.
      |
      */
    pub fn apply_changes(&mut self, new_content: &String)  {
        
        todo!();
        /*
            const String corrected (StringArray::fromLines (newContent)
                                    .joinIntoString (newLineChars));

        TextDiff diff (getAllContent(), corrected);

        for (auto& c : diff.changes)
        {
            if (c.isDeletion())
                remove (c.start, c.start + c.length, true);
            else
                insert (c.insertedText, c.start, true);
        }
        */
    }
    
    /**
      | Clears the document and replaces it
      | with some new text.
      | 
      | This operation is undoable - if you're
      | trying to completely reset the document,
      | you might want to also call clearUndoHistory()
      | and setSavePoint() after using this
      | method.
      |
      */
    pub fn replace_all_content(&mut self, new_content: &String)  {
        
        todo!();
        /*
            remove (0, getNumCharacters(), true);
        insert (newContent, 0, true);
        */
    }
    
    /**
      | Replaces the editor's contents with
      | the contents of a stream.
      | 
      | This will also reset the undo history
      | and save point marker.
      |
      */
    pub fn load_from_stream<R: Read>(&mut self, stream: &mut R) -> bool {
        
        todo!();
        /*
            remove (0, getNumCharacters(), false);
        insert (stream.readEntireStreamAsString(), 0, false);
        setSavePoint();
        clearUndoHistory();
        return true;
        */
    }
    
    /**
      | Writes the editor's current contents
      | to a stream.
      |
      */
    pub fn write_to_stream<W: Write>(&mut self, stream: &mut W) -> bool {
        
        todo!();
        /*
            for (auto* l : lines)
        {
            auto temp = l->line; // use a copy to avoid bloating the memory footprint of the stored string.
            const char* utf8 = temp.toUTF8();

            if (! stream.write (utf8, strlen (utf8)))
                return false;
        }

        return true;
        */
    }
    
    /**
      | Sets the new-line characters that the
      | document should use.
      | 
      | The string must be either "\\n", "\\r\\n",
      | or (rarely) "\\r". @see getNewLineCharacters
      |
      */
    pub fn set_new_line_characters(&mut self, new_chars: &String)  {
        
        todo!();
        /*
            jassert (newChars == "\r\n" || newChars == "\n" || newChars == "\r");
        newLineChars = newChars;
        */
    }
    
    /**
      | Begins a new undo transaction.
      | 
      | The document itself will not call this
      | internally, so relies on whatever is
      | using the document to periodically
      | call this to break up the undo sequence
      | into sensible chunks. @see UndoManager::beginNewTransaction
      |
      */
    pub fn new_transaction(&mut self)  {
        
        todo!();
        /*
            undoManager.beginNewTransaction (String());
        */
    }
    
    /**
      | Undo the last operation. @see UndoManager::undo
      |
      */
    pub fn undo(&mut self)  {
        
        todo!();
        /*
            newTransaction();
        undoManager.undo();
        */
    }
    
    /**
      | Redo the last operation. @see UndoManager::redo
      |
      */
    pub fn redo(&mut self)  {
        
        todo!();
        /*
            undoManager.redo();
        */
    }
    
    /**
      | Clears the undo history. @see UndoManager::clearUndoHistory
      |
      */
    pub fn clear_undo_history(&mut self)  {
        
        todo!();
        /*
            undoManager.clearUndoHistory();
        */
    }
    
    /**
      | Makes a note that the document's current
      | state matches the one that is saved.
      | 
      | After this has been called, hasChangedSinceSavePoint()
      | will return false until the document
      | has been altered, and then it'll start
      | returning true. If the document is altered,
      | but then undone until it gets back to
      | this state, hasChangedSinceSavePoint()
      | will again return false.
      | 
      | @see hasChangedSinceSavePoint
      |
      */
    pub fn set_save_point(&mut self)  {
        
        todo!();
        /*
            indexOfSavedState = currentActionIndex;
        */
    }
    
    /**
      | Returns true if the state of the document
      | differs from the state it was in when
      | setSavePoint() was last called.
      | 
      | @see setSavePoint
      |
      */
    pub fn has_changed_since_save_point(&self) -> bool {
        
        todo!();
        /*
            return currentActionIndex != indexOfSavedState;
        */
    }
    
    /**
      | Searches for a word-break.
      |
      */
    pub fn find_word_break_after(&self, position: &CodeDocumentPosition) -> CodeDocumentPosition {
        
        todo!();
        /*
            auto p = position;
        const int maxDistance = 256;
        int i = 0;

        while (i < maxDistance
                && CharacterFunctions::isWhitespace (p.getCharacter())
                && (i == 0 || (p.getCharacter() != '\n'
                                && p.getCharacter() != '\r')))
        {
            ++i;
            p.moveBy (1);
        }

        if (i == 0)
        {
            auto type = getCharacterType (p.getCharacter());

            while (i < maxDistance && type == getCharacterType (p.getCharacter()))
            {
                ++i;
                p.moveBy (1);
            }

            while (i < maxDistance
                    && CharacterFunctions::isWhitespace (p.getCharacter())
                    && (i == 0 || (p.getCharacter() != '\n'
                                    && p.getCharacter() != '\r')))
            {
                ++i;
                p.moveBy (1);
            }
        }

        return p;
        */
    }
    
    /**
      | Searches for a word-break.
      |
      */
    pub fn find_word_break_before(&self, position: &CodeDocumentPosition) -> CodeDocumentPosition {
        
        todo!();
        /*
            auto p = position;
        const int maxDistance = 256;
        int i = 0;
        bool stoppedAtLineStart = false;

        while (i < maxDistance)
        {
            auto c = p.movedBy (-1).getCharacter();

            if (c == '\r' || c == '\n')
            {
                stoppedAtLineStart = true;

                if (i > 0)
                    break;
            }

            if (! CharacterFunctions::isWhitespace (c))
                break;

            p.moveBy (-1);
            ++i;
        }

        if (i < maxDistance && ! stoppedAtLineStart)
        {
            auto type = getCharacterType (p.movedBy (-1).getCharacter());

            while (i < maxDistance && type == getCharacterType (p.movedBy (-1).getCharacter()))
            {
                p.moveBy (-1);
                ++i;
            }
        }

        return p;
        */
    }
    
    /**
      | Finds the token that contains the given
      | position.
      |
      */
    pub fn find_token_containing(&self, 
        pos:   &CodeDocumentPosition,
        start: &mut CodeDocumentPosition,
        end:   &mut CodeDocumentPosition)  {
        
        todo!();
        /*
            auto isTokenCharacter = [] (aloe_wchar c)  { return CharacterFunctions::isLetterOrDigit (c) || c == '.' || c == '_'; };

        end = pos;
        while (isTokenCharacter (end.getCharacter()))
            end.moveBy (1);

        start = end;
        while (start.getIndexInLine() > 0
                && isTokenCharacter (start.movedBy (-1).getCharacter()))
            start.moveBy (-1);
        */
    }
    
    /**
      | Finds the line that contains the given
      | position.
      |
      */
    pub fn find_line_containing(&self, 
        pos: &CodeDocumentPosition,
        s:   &mut CodeDocumentPosition,
        e:   &mut CodeDocumentPosition)  {
        
        todo!();
        /*
            s.setLineAndIndex (pos.getLineNumber(), 0);
        e.setLineAndIndex (pos.getLineNumber() + 1, 0);
        */
    }
    
    pub fn check_last_line_status(&mut self)  {
        
        todo!();
        /*
            while (lines.size() > 0
                && lines.getLast()->lineLength == 0
                && (lines.size() == 1 || ! lines.getUnchecked (lines.size() - 2)->endsWithLineBreak()))
        {
            // remove any empty lines at the end if the preceding line doesn't end in a newline.
            lines.removeLast();
        }

        const CodeDocumentLine* const lastLine = lines.getLast();

        if (lastLine != nullptr && lastLine->endsWithLineBreak())
        {
            // check that there's an empty line at the end if the preceding one ends in a newline..
            lines.add (new CodeDocumentLine ("", "", 0, 0,
                                             lastLine->lineStartInFile + lastLine->lineLength));
        }
        */
    }
    
    /**
      | Registers a listener object to receive
      | callbacks when the document changes.
      | 
      | If the listener is already registered,
      | this method has no effect. @see removeListener
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn CodeDocumentListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Deregisters a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn CodeDocumentListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn insert(&mut self, 
        text:       &String,
        insert_pos: i32,
        undoable:   bool)  {
        
        todo!();
        /*
            if (text.isNotEmpty())
        {
            if (undoable)
            {
                undoManager.perform (new CodeDocumentInsertAction (*this, text, insertPos));
            }
            else
            {
                CodeDocumentPosition pos (*this, insertPos);
                auto firstAffectedLine = pos.getLineNumber();

                auto* firstLine = lines[firstAffectedLine];
                auto textInsideOriginalLine = text;

                if (firstLine != nullptr)
                {
                    auto index = pos.getIndexInLine();
                    textInsideOriginalLine = firstLine->line.substring (0, index)
                                             + textInsideOriginalLine
                                             + firstLine->line.substring (index);
                }

                maximumLineLength = -1;
                Vec<CodeDocumentLine*> newLines;
                CodeDocumentLine::createLines (newLines, textInsideOriginalLine);
                jassert (newLines.size() > 0);

                auto* newFirstLine = newLines.getUnchecked (0);
                newFirstLine->lineStartInFile = firstLine != nullptr ? firstLine->lineStartInFile : 0;
                lines.set (firstAffectedLine, newFirstLine);

                if (newLines.size() > 1)
                    lines.insertArray (firstAffectedLine + 1, newLines.getRawDataPointer() + 1, newLines.size() - 1);

                int lineStart = newFirstLine->lineStartInFile;

                for (int i = firstAffectedLine; i < lines.size(); ++i)
                {
                    auto& l = *lines.getUnchecked (i);
                    l.lineStartInFile = lineStart;
                    lineStart += l.lineLength;
                }

                checkLastLineStatus();
                auto newTextLength = text.length();

                for (auto* p : positionsToMaintain)
                    if (p->getPosition() >= insertPos)
                        p->setPosition (p->getPosition() + newTextLength);

                listeners.call ([&] (CodeDocumentListener& l) { l.codeDocumentTextInserted (text, insertPos); });
            }
        }
        */
    }
    
    pub fn remove(&mut self, 
        start_pos: i32,
        end_pos:   i32,
        undoable:  bool)  {
        
        todo!();
        /*
            if (endPos <= startPos)
            return;

        if (undoable)
        {
            undoManager.perform (new CodeDocumentDeleteAction (*this, startPos, endPos));
        }
        else
        {
            CodeDocumentPosition startPosition (*this, startPos);
            CodeDocumentPosition endPosition (*this, endPos);

            maximumLineLength = -1;
            auto firstAffectedLine = startPosition.getLineNumber();
            auto endLine = endPosition.getLineNumber();
            auto& firstLine = *lines.getUnchecked (firstAffectedLine);

            if (firstAffectedLine == endLine)
            {
                firstLine.line = firstLine.line.substring (0, startPosition.getIndexInLine())
                               + firstLine.line.substring (endPosition.getIndexInLine());
                firstLine.updateLength();
            }
            else
            {
                auto& lastLine = *lines.getUnchecked (endLine);

                firstLine.line = firstLine.line.substring (0, startPosition.getIndexInLine())
                                + lastLine.line.substring (endPosition.getIndexInLine());
                firstLine.updateLength();

                int numLinesToRemove = endLine - firstAffectedLine;
                lines.removeRange (firstAffectedLine + 1, numLinesToRemove);
            }

            for (int i = firstAffectedLine + 1; i < lines.size(); ++i)
            {
                auto& l = *lines.getUnchecked (i);
                auto& previousLine = *lines.getUnchecked (i - 1);
                l.lineStartInFile = previousLine.lineStartInFile + previousLine.lineLength;
            }

            checkLastLineStatus();
            auto totalChars = getNumCharacters();

            for (auto* p : positionsToMaintain)
            {
                if (p->getPosition() > startPosition.getPosition())
                    p->setPosition (jmax (startPos, p->getPosition() + startPos - endPos));

                if (p->getPosition() > totalChars)
                    p->setPosition (totalChars);
            }

            listeners.call ([=] (CodeDocumentListener& l) { l.codeDocumentTextDeleted (startPos, endPos); });
        }
        */
    }
}
