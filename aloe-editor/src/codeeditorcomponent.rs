crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CodeEditorComponent.h]

pub enum CodeEditorComponentDragType
{
    notDragging,
    draggingSelectionStart,
    draggingSelectionEnd
}

/**
  | A text editor component designed specifically
  | for source code.
  | 
  | This is designed to handle syntax highlighting
  | and fast editing of very large files.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct CodeEditorComponent<'a> {
    base:                           Component<'a>,
    base2:                          ApplicationCommandTarget,
    document:                       &'a mut CodeDocument<'a>,
    font:                           Font,
    first_line_on_screen:           i32, // default = 0
    spaces_per_tab:                 i32, // default = 4
    char_width:                     f32, // default = 0
    line_height:                    i32, // default = 0
    lines_on_screen:                i32, // default = 0
    columns_on_screen:              i32, // default = 0
    scrollbar_thickness:            i32, // default = 16
    column_to_try_to_maintain:      i32, // default = -1
    read_only:                      bool, // default = false
    use_spaces_for_tabs:            bool, // default = true
    show_line_numbers:              bool, // default = false
    should_follow_document_changes: bool, // default = false
    x_offset:                       f64, // default = 0
    caret_pos:                      CodeDocumentPosition<'a>,
    selection_start:                CodeDocumentPosition<'a>,
    selection_end:                  CodeDocumentPosition<'a>,
    caret:                          Box<CaretComponent<'a>>,
    vertical_scroll_bar:            ScrollBar<'a>, // default = { true  }
    horizontal_scroll_bar:          ScrollBar<'a>, // default = { false  }
    app_command_manager:            *mut ApplicationCommandManager<'a>, // default = nullptr
    impl_:                          Box<CodeEditorComponentImpl<'a>>,
    gutter:                         Box<CodeEditorComponentGutterComponent<'a>>,
    drag_type:                      CodeEditorComponentDragType, // default = notDragging
    code_tokeniser:                 *mut dyn CodeTokeniser,
    colour_scheme:                  CodeEditorComponentColourScheme,
    lines:                          Vec<Box<CodeEditorLine>>,
    cached_iterators:               Vec<CodeDocumentIterator<'a>>,
}

impl<'a> TextInputTarget for CodeEditorComponent<'a> {

    fn is_text_input_active(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }

    fn get_highlighted_region(&self) -> Range<i32> {
        
        todo!();
        /*
            return { selectionStart.getPosition(),
                 selectionEnd.getPosition() };
        */
    }

    fn set_highlighted_region(&mut self, new_range: &Range<i32>)  {
        
        todo!();
        /*
            selectRegion (CodeDocument::Position (document, newRange.getStart()),
                      CodeDocument::Position (document, newRange.getEnd()));
        */
    }

    fn set_temporary_underlining(&mut self, _0: &[Range<i32>])  {
        
        todo!();
        /*
            jassertfalse; // TODO Windows IME not yet supported for this comp..
        */
    }

    fn get_text_in_range(&self, range: &Range<i32>) -> String {
        
        todo!();
        /*
            return document.getTextBetween (CodeDocument::Position (document, range.getStart()),
                                        CodeDocument::Position (document, range.getEnd()));
        */
    }

    fn insert_text_at_caret(&mut self, new_text: &String)  {
        
        todo!();
        /*
            insertText (newText);
        */
    }

    /**
      | Returns the position of the caret, relative
      | to the editor's origin.
      |
      */
    fn get_caret_rectangle(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (caret != nullptr)
            return getLocalArea (caret.get(), caret->getLocalBounds());

        return {};
        */
    }
}

impl<'a> Drop for CodeEditorComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            document.removeListener (impl.get());
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/code_editor/aloe_CodeEditorComponent.cpp]
impl<'a> CodeEditorComponent<'a> {

    /**
      | Returns the code document that this
      | component is editing.
      |
      */
    pub fn get_document(&self) -> &mut CodeDocument {
        
        todo!();
        /*
            return document;
        */
    }

    /**
      | Returns the standard character width.
      |
      */
    pub fn get_char_width(&self) -> f32 {
        
        todo!();
        /*
            return charWidth;
        */
    }

    /**
      | Returns the height of a line of text,
      | in pixels.
      |
      */
    pub fn get_line_height(&self) -> i32 {
        
        todo!();
        /*
            return lineHeight;
        */
    }

    /**
      | Returns the number of whole lines visible
      | on the screen,
      | 
      | This doesn't include a cut-off line
      | that might be visible at the bottom if
      | the component's height isn't an exact
      | multiple of the line-height.
      |
      */
    pub fn get_num_lines_on_screen(&self) -> i32 {
        
        todo!();
        /*
            return linesOnScreen;
        */
    }

    /**
      | Returns the index of the first line that's
      | visible at the top of the editor.
      |
      */
    pub fn get_first_line_on_screen(&self) -> i32 {
        
        todo!();
        /*
            return firstLineOnScreen;
        */
    }

    /**
      | Returns the number of whole columns
      | visible on the screen.
      | 
      | This doesn't include any cut-off columns
      | at the right-hand edge.
      |
      */
    pub fn get_num_columns_on_screen(&self) -> i32 {
        
        todo!();
        /*
            return columnsOnScreen;
        */
    }

    /**
      | Returns the current caret position.
      |
      */
    pub fn get_caret_pos(&self) -> CodeDocumentPosition {
        
        todo!();
        /*
            return caretPos;
        */
    }

    /**
      | Returns the start of the selection as
      | a position.
      |
      */
    pub fn get_selection_start(&self) -> CodeDocumentPosition {
        
        todo!();
        /*
            return selectionStart;
        */
    }

    /**
      | Returns the end of the selection as a
      | position.
      |
      */
    pub fn get_selection_end(&self) -> CodeDocumentPosition {
        
        todo!();
        /*
            return selectionEnd;
        */
    }

    /**
      | Returns the current number of spaces
      | per tab. @see setTabSize
      |
      */
    pub fn get_tab_size(&self) -> i32 {
        
        todo!();
        /*
            return spacesPerTab;
        */
    }

    /**
      | Returns true if the tab key will insert
      | spaces instead of actual tab characters.
      | @see setTabSize
      |
      */
    pub fn are_spaces_inserted_for_tabs(&self) -> bool {
        
        todo!();
        /*
            return useSpacesForTabs;
        */
    }

    /**
      | Returns the font that the editor is using.
      |
      */
    pub fn get_font(&self) -> &Font {
        
        todo!();
        /*
            return font;
        */
    }

    /**
      | Returns true if the editor is set to be
      | read-only.
      |
      */
    pub fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return readOnly;
        */
    }
   
    /**
      | Returns the current syntax highlighting
      | colour scheme.
      |
      */
    pub fn get_colour_scheme(&self) -> &CodeEditorComponentColourScheme {
        
        todo!();
        /*
            return colourScheme;
        */
    }

    /**
      | Returns the thickness of the scrollbars.
      |
      */
    pub fn get_scrollbar_thickness(&self) -> i32 {
        
        todo!();
        /*
            return scrollbarThickness;
        */
    }

    /**
      | Creates an editor for a document.
      | 
      | The tokeniser object is optional - pass
      | nullptr to disable syntax highlighting.
      | 
      | The object that you pass in is not owned
      | or deleted by the editor - you must make
      | sure that it doesn't get deleted while
      | this component is still using it.
      | 
      | @see CodeDocument
      |
      */
    pub fn new(
        doc:       &mut CodeDocument,
        tokeniser: *mut dyn CodeTokeniser

    ) -> Self {
    
        todo!();
        /*
        : document(doc),
        : caret_pos(doc, 0, 0),
        : selection_start(doc, 0, 0),
        : selection_end(doc, 0, 0),
        : code_tokeniser(tokeniser),

            impl.reset (new CodeEditorComponentImpl (*this));

        caretPos.setPositionMaintained (true);
        selectionStart.setPositionMaintained (true);
        selectionEnd.setPositionMaintained (true);

        setOpaque (true);
        setMouseCursor (MouseCursor::IBeamCursor);
        setWantsKeyboardFocus (true);

        addAndMakeVisible (verticalScrollBar);
        verticalScrollBar.setSingleStepSize (1.0);

        addAndMakeVisible (horizontalScrollBar);
        horizontalScrollBar.setSingleStepSize (1.0);

        Font f (12.0f);
        f.setTypefaceName (Font::getDefaultMonospacedFontName());
        setFont (f);

        if (codeTokeniser != nullptr)
            setColourScheme (codeTokeniser->getDefaultColourScheme());

        setLineNumbersShown (true);

        verticalScrollBar.addListener (impl.get());
        horizontalScrollBar.addListener (impl.get());
        document.addListener (impl.get());

        lookAndFeelChanged();
        */
    }
    
    pub fn get_gutter_size(&self) -> i32 {
        
        todo!();
        /*
            return showLineNumbers ? 35 : 5;
        */
    }
    
    /**
      | Loads the given content into the document.
      | 
      | This will completely reset the CodeDocument
      | object, clear its undo history, and
      | fill it with this text.
      |
      */
    pub fn load_content(&mut self, new_content: &String)  {
        
        todo!();
        /*
            clearCachedIterators (0);
        document.replaceAllContent (newContent);
        document.clearUndoHistory();
        document.setSavePoint();
        caretPos.setPosition (0);
        selectionStart.setPosition (0);
        selectionEnd.setPosition (0);
        scrollToLine (0);
        */
    }
    
    /**
      | Enables or disables the line-number
      | display in the gutter.
      |
      */
    pub fn set_line_numbers_shown(&mut self, should_be_shown: bool)  {
        
        todo!();
        /*
            if (showLineNumbers != shouldBeShown)
        {
            showLineNumbers = shouldBeShown;
            gutter.reset();

            if (shouldBeShown)
            {
                gutter.reset (new CodeEditorComponentGutterComponent());
                addAndMakeVisible (gutter.get());
            }

            resized();
        }
        */
    }
    
    /**
      | Makes the editor read-only.
      |
      */
    pub fn set_read_only(&mut self, b: bool)  {
        
        todo!();
        /*
            if (readOnly != b)
        {
            readOnly = b;

            if (b)
                removeChildComponent (caret.get());
            else
                addAndMakeVisible (caret.get());

            invalidateAccessibilityHandler();
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto visibleWidth = getWidth() - scrollbarThickness - getGutterSize();
        linesOnScreen   = jmax (1, (getHeight() - scrollbarThickness) / lineHeight);
        columnsOnScreen = jmax (1, (int) ((float) visibleWidth / charWidth));
        lines.clear();
        rebuildLineTokens();
        updateCaretPosition();

        if (gutter != nullptr)
            gutter->setBounds (0, 0, getGutterSize() - 2, getHeight());

        verticalScrollBar.setBounds (getWidth() - scrollbarThickness, 0,
                                     scrollbarThickness, getHeight() - scrollbarThickness);

        horizontalScrollBar.setBounds (getGutterSize(), getHeight() - scrollbarThickness,
                                       visibleWidth, scrollbarThickness);
        updateScrollBars();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (CodeEditorComponent::backgroundColourId));

        auto gutterSize = getGutterSize();
        auto bottom = horizontalScrollBar.isVisible() ? horizontalScrollBar.getY() : getHeight();
        auto right  = verticalScrollBar.isVisible()   ? verticalScrollBar.getX()   : getWidth();

        g.reduceClipRegion (gutterSize, 0, right - gutterSize, bottom);

        g.setFont (font);

        auto clip = g.getClipBounds();
        auto firstLineToDraw = jmax (0, clip.getY() / lineHeight);
        auto lastLineToDraw  = jmin (lines.size(), clip.getBottom() / lineHeight + 1);
        auto x = (float) (gutterSize - xOffset * charWidth);
        auto rightClip = (float) clip.getRight();

        {
            RectangleList<float> highlightArea;

            for (int i = firstLineToDraw; i < lastLineToDraw; ++i)
                lines.getUnchecked(i)->getHighlightArea (highlightArea, x, lineHeight * i, lineHeight, charWidth);

            g.setColour (findColour (CodeEditorComponent::highlightColourId));
            g.fillRectList (highlightArea);
        }

        for (int i = firstLineToDraw; i < lastLineToDraw; ++i)
            lines.getUnchecked(i)->draw (*this, g, font, rightClip, x, lineHeight * i, lineHeight, charWidth);
        */
    }
    
    /**
      | Changes the size of the scrollbars.
      |
      */
    pub fn set_scrollbar_thickness(&mut self, thickness: i32)  {
        
        todo!();
        /*
            if (scrollbarThickness != thickness)
        {
            scrollbarThickness = thickness;
            resized();
        }
        */
    }
    
    pub fn rebuild_line_tokens_async(&mut self)  {
        
        todo!();
        /*
            impl->triggerAsyncUpdate();
        */
    }
    
    pub fn rebuild_line_tokens(&mut self)  {
        
        todo!();
        /*
            impl->cancelPendingUpdate();

        auto numNeeded = linesOnScreen + 1;
        auto minLineToRepaint = numNeeded;
        int maxLineToRepaint = 0;

        if (numNeeded != lines.size())
        {
            lines.clear();

            for (int i = numNeeded; --i >= 0;)
                lines.add (new CodeEditorLine());

            minLineToRepaint = 0;
            maxLineToRepaint = numNeeded;
        }

        jassert (numNeeded == lines.size());

        CodeDocument::Iterator source (document);
        getIteratorForPosition (CodeDocument::Position (document, firstLineOnScreen, 0).getPosition(), source);

        for (int i = 0; i < numNeeded; ++i)
        {
            if (lines.getUnchecked(i)->update (document, firstLineOnScreen + i, source, codeTokeniser,
                                               spacesPerTab, selectionStart, selectionEnd))
            {
                minLineToRepaint = jmin (minLineToRepaint, i);
                maxLineToRepaint = jmax (maxLineToRepaint, i);
            }
        }

        if (minLineToRepaint <= maxLineToRepaint)
            repaint (0, lineHeight * minLineToRepaint - 1,
                     verticalScrollBar.getX(), lineHeight * (1 + maxLineToRepaint - minLineToRepaint) + 2);

        if (gutter != nullptr)
            gutter->documentChanged (document, firstLineOnScreen);
        */
    }
    
    pub fn code_document_changed(&mut self, 
        start_index: i32,
        end_index:   i32)  {
        
        todo!();
        /*
            const CodeDocument::Position affectedTextStart (document, startIndex);
        const CodeDocument::Position affectedTextEnd (document, endIndex);

        retokenise (startIndex, endIndex);

        updateCaretPosition();
        columnToTryToMaintain = -1;

        if (affectedTextEnd.getPosition() >= selectionStart.getPosition()
             && affectedTextStart.getPosition() <= selectionEnd.getPosition())
            deselectAll();

        if (shouldFollowDocumentChanges)
            if (caretPos.getPosition() > affectedTextEnd.getPosition()
                || caretPos.getPosition() < affectedTextStart.getPosition())
                moveCaretTo (affectedTextStart, false);

        updateScrollBars();
        */
    }
    
    /**
      | Rebuilds the syntax highlighting for
      | a section of text.
      | 
      | This happens automatically any time
      | the CodeDocument is edited, but this
      | method lets you change text colours
      | even when the CodeDocument hasn't changed.
      | 
      | For example, you could use this to highlight
      | tokens as the cursor moves.
      | 
      | To do so you'll need to tell your custom
      | CodeTokeniser where the token you want
      | to highlight is, and make it return a
      | special type of token. Then you should
      | call this method supplying the range
      | of the highlighted text. @see CodeTokeniser
      |
      */
    pub fn retokenise(&mut self, 
        start_index: i32,
        end_index:   i32)  {
        
        todo!();
        /*
            const CodeDocument::Position affectedTextStart (document, startIndex);
        ignoreUnused (endIndex); // Leave room for more efficient impl in future.

        clearCachedIterators (affectedTextStart.getLineNumber());

        rebuildLineTokensAsync();
        */
    }
    
    pub fn update_caret_position(&mut self)  {
        
        todo!();
        /*
            if (caret != nullptr)
        {
            caret->setCaretPosition (getCharacterBounds (getCaretPos()));

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textSelectionChanged);
        }
        */
    }
    
    /**
      | Moves the caret.
      | 
      | If selecting is true, the section of
      | the document between the current caret
      | position and the new one will become
      | selected. If false, any currently selected
      | region will be deselected.
      |
      */
    pub fn move_caret_to(
        &mut self, 
        new_pos:      &CodeDocumentPosition,
        highlighting: bool

    ) {
        
        todo!();
        /*
            caretPos = newPos;
        columnToTryToMaintain = -1;
        bool selectionWasActive = isHighlightActive();

        if (highlighting)
        {
            if (dragType == notDragging)
            {
                auto oldCaretPos = caretPos.getPosition();
                auto isStart = std::abs (oldCaretPos - selectionStart.getPosition())
                                < std::abs (oldCaretPos - selectionEnd.getPosition());

                dragType = isStart ? draggingSelectionStart : draggingSelectionEnd;
            }

            if (dragType == draggingSelectionStart)
            {
                if (selectionEnd.getPosition() < caretPos.getPosition())
                {
                    setSelection (selectionEnd, caretPos);
                    dragType = draggingSelectionEnd;
                }
                else
                {
                    setSelection (caretPos, selectionEnd);
                }
            }
            else
            {
                if (caretPos.getPosition() < selectionStart.getPosition())
                {
                    setSelection (caretPos, selectionStart);
                    dragType = draggingSelectionStart;
                }
                else
                {
                    setSelection (selectionStart, caretPos);
                }
            }

            rebuildLineTokensAsync();
        }
        else
        {
            deselectAll();
        }

        updateCaretPosition();
        scrollToKeepCaretOnScreen();
        updateScrollBars();
        caretPositionMoved();

        if (auto* handler = getAccessibilityHandler())
            handler->notifyAccessibilityEvent (AccessibilityEvent::textChanged);

        if (appCommandManager != nullptr && selectionWasActive != isHighlightActive())
            appCommandManager->commandStatusChanged();
        */
    }
    
    pub fn deselect_all(&mut self)  {
        
        todo!();
        /*
            if (isHighlightActive())
            rebuildLineTokensAsync();

        setSelection (caretPos, caretPos);
        dragType = notDragging;
        */
    }
    
    pub fn update_scroll_bars(&mut self)  {
        
        todo!();
        /*
            verticalScrollBar.setRangeLimits (0, jmax (document.getNumLines(), firstLineOnScreen + linesOnScreen));
        verticalScrollBar.setCurrentRange (firstLineOnScreen, linesOnScreen);

        horizontalScrollBar.setRangeLimits (0, jmax ((double) document.getMaximumLineLength(), xOffset + columnsOnScreen));
        horizontalScrollBar.setCurrentRange (xOffset, columnsOnScreen);
        */
    }
    
    pub fn scroll_to_line_internal(&mut self, new_first_line_on_screen: i32)  {
        
        todo!();
        /*
            newFirstLineOnScreen = jlimit (0, jmax (0, document.getNumLines() - 1),
                                       newFirstLineOnScreen);

        if (newFirstLineOnScreen != firstLineOnScreen)
        {
            firstLineOnScreen = newFirstLineOnScreen;
            updateCaretPosition();

            updateCachedIterators (firstLineOnScreen);
            rebuildLineTokensAsync();
            impl->handleUpdateNowIfNeeded();

            editorViewportPositionChanged();
        }
        */
    }
    
    pub fn scroll_to_column_internal(&mut self, column: f64)  {
        
        todo!();
        /*
            const double newOffset = jlimit (0.0, document.getMaximumLineLength() + 3.0, column);

        if (xOffset != newOffset)
        {
            xOffset = newOffset;
            updateCaretPosition();
            repaint();
        }
        */
    }
    
    pub fn scroll_to_line(&mut self, new_first_line_on_screen: i32)  {
        
        todo!();
        /*
            scrollToLineInternal (newFirstLineOnScreen);
        updateScrollBars();
        */
    }
    
    pub fn scroll_to_column(&mut self, new_first_column_on_screen: i32)  {
        
        todo!();
        /*
            scrollToColumnInternal (newFirstColumnOnScreen);
        updateScrollBars();
        */
    }
    
    pub fn scroll_by(&mut self, delta_lines: i32)  {
        
        todo!();
        /*
            scrollToLine (firstLineOnScreen + deltaLines);
        */
    }
    
    pub fn scroll_to_keep_lines_on_screen(&mut self, range_to_show: Range<i32>)  {
        
        todo!();
        /*
            if (rangeToShow.getStart() < firstLineOnScreen)
            scrollBy (rangeToShow.getStart() - firstLineOnScreen);
        else if (rangeToShow.getEnd() >= firstLineOnScreen + linesOnScreen)
            scrollBy (rangeToShow.getEnd() - (firstLineOnScreen + linesOnScreen - 1));
        */
    }
    
    pub fn scroll_to_keep_caret_on_screen(&mut self)  {
        
        todo!();
        /*
            if (getWidth() > 0 && getHeight() > 0)
        {
            auto caretLine = caretPos.getLineNumber();
            scrollToKeepLinesOnScreen ({ caretLine, caretLine });

            auto column = indexToColumn (caretPos.getLineNumber(), caretPos.getIndexInLine());

            if (column >= xOffset + columnsOnScreen - 1)
                scrollToColumn (column + 1 - columnsOnScreen);
            else if (column < xOffset)
                scrollToColumn (column);
        }
        */
    }
    
    /**
      | Returns the on-screen position of a
      | character in the document.
      | 
      | The rectangle returned is relative
      | to this component's top-left origin.
      |
      */
    pub fn get_character_bounds(&self, pos: &CodeDocumentPosition) -> Rectangle<i32> {
        
        todo!();
        /*
            return { roundToInt ((getGutterSize() - xOffset * charWidth) + (float) indexToColumn (pos.getLineNumber(), pos.getIndexInLine()) * charWidth),
                 (pos.getLineNumber() - firstLineOnScreen) * lineHeight,
                 roundToInt (charWidth),
                 lineHeight };
        */
    }
    
    /**
      | Finds the character at a given on-screen
      | position.
      | 
      | The coordinates are relative to this
      | component's top-left origin.
      |
      */
    pub fn get_position_at(&self, x: i32, y: i32) -> CodeDocumentPosition {
        
        todo!();
        /*
            const int line = y / lineHeight + firstLineOnScreen;
        const int column = roundToInt ((x - (getGutterSize() - xOffset * charWidth)) / charWidth);
        const int index = columnToIndex (line, column);

        return CodeDocument::Position (document, line, index);
        */
    }
    
    pub fn insert_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            if (! readOnly)
        {
            document.deleteSection (selectionStart, selectionEnd);

            if (newText.isNotEmpty())
                document.insertText (caretPos, newText);

            scrollToKeepCaretOnScreen();
            caretPositionMoved();

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textChanged);
        }
        */
    }
    
    pub fn insert_tab_at_caret(&mut self)  {
        
        todo!();
        /*
            if (! readOnly)
        {
            if (CharacterFunctions::isWhitespace (caretPos.getCharacter())
                 && caretPos.getLineNumber() == caretPos.movedBy (1).getLineNumber())
            {
                moveCaretTo (document.findWordBreakAfter (caretPos), false);
            }

            if (useSpacesForTabs)
            {
                auto caretCol = indexToColumn (caretPos.getLineNumber(), caretPos.getIndexInLine());
                auto spacesNeeded = spacesPerTab - (caretCol % spacesPerTab);
                insertTextAtCaret (String::repeatedString (" ", spacesNeeded));
            }
            else
            {
                insertTextAtCaret ("\t");
            }
        }
        */
    }
    
    pub fn delete_whitespace_backwards_to_tab_stop(&mut self) -> bool {
        
        todo!();
        /*
            if (getHighlightedRegion().isEmpty() && ! readOnly)
        {
            for (;;)
            {
                auto currentColumn = indexToColumn (caretPos.getLineNumber(), caretPos.getIndexInLine());

                if (currentColumn <= 0 || (currentColumn % spacesPerTab) == 0)
                    break;

                moveCaretLeft (false, true);
            }

            auto selected = getTextInRange (getHighlightedRegion());

            if (selected.isNotEmpty() && selected.trim().isEmpty())
            {
                cut();
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn indent_selection(&mut self)  {
        
        todo!();
        /*
            indentSelectedLines ( spacesPerTab);
        */
    }
    
    pub fn unindent_selection(&mut self)  {
        
        todo!();
        /*
            indentSelectedLines (-spacesPerTab);
        */
    }
    
    pub fn indent_selected_lines(&mut self, spaces_to_add: i32)  {
        
        todo!();
        /*
            if (! readOnly)
        {
            newTransaction();

            CodeDocument::Position oldSelectionStart (selectionStart), oldSelectionEnd (selectionEnd), oldCaret (caretPos);
            oldSelectionStart.setPositionMaintained (true);
            oldSelectionEnd.setPositionMaintained (true);
            oldCaret.setPositionMaintained (true);

            const int lineStart = selectionStart.getLineNumber();
            int lineEnd = selectionEnd.getLineNumber();

            if (lineEnd > lineStart && selectionEnd.getIndexInLine() == 0)
                --lineEnd;

            for (int line = lineStart; line <= lineEnd; ++line)
            {
                auto lineText = document.getLine (line);
                auto nonWhitespaceStart = CodeEditorHelpers::findFirstNonWhitespaceChar (lineText);

                if (nonWhitespaceStart > 0 || lineText.trimStart().isNotEmpty())
                {
                    const CodeDocument::Position wsStart (document, line, 0);
                    const CodeDocument::Position wsEnd   (document, line, nonWhitespaceStart);

                    const int numLeadingSpaces = indexToColumn (line, wsEnd.getIndexInLine());
                    const int newNumLeadingSpaces = jmax (0, numLeadingSpaces + spacesToAdd);

                    if (newNumLeadingSpaces != numLeadingSpaces)
                    {
                        document.deleteSection (wsStart, wsEnd);
                        document.insertText (wsStart, getTabString (newNumLeadingSpaces));
                    }
                }
            }

            setSelection (oldSelectionStart, oldSelectionEnd);

            if (caretPos != oldCaret)
            {
                caretPos = oldCaret;

                if (auto* handler = getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::textChanged);
            }
        }
        */
    }
    
    pub fn cut(&mut self)  {
        
        todo!();
        /*
            insertText ({});
        */
    }
    
    pub fn copy_to_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        auto selection = document.getTextBetween (selectionStart, selectionEnd);

        if (selection.isNotEmpty())
            SystemClipboard::copyTextToClipboard (selection);

        return true;
        */
    }
    
    pub fn cut_to_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            copyToClipboard();
        cut();
        newTransaction();
        return true;
        */
    }
    
    pub fn paste_from_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        auto clip = SystemClipboard::getTextFromClipboard();

        if (clip.isNotEmpty())
            insertText (clip);

        newTransaction();
        return true;
        */
    }
    
    pub fn move_caret_left(&mut self, 
        move_in_whole_word_steps: bool,
        selecting:                bool) -> bool {
        
        todo!();
        /*
            newTransaction();

        if (selecting && dragType == notDragging)
        {
            selectRegion (CodeDocument::Position (selectionEnd), CodeDocument::Position (selectionStart));
            dragType = draggingSelectionStart;
        }

        if (isHighlightActive() && ! (selecting || moveInWholeWordSteps))
        {
            moveCaretTo (selectionStart, false);
            return true;
        }

        if (moveInWholeWordSteps)
            moveCaretTo (document.findWordBreakBefore (caretPos), selecting);
        else
            moveCaretTo (caretPos.movedBy (-1), selecting);

        return true;
        */
    }
    
    pub fn move_caret_right(&mut self, 
        move_in_whole_word_steps: bool,
        selecting:                bool) -> bool {
        
        todo!();
        /*
            newTransaction();

        if (selecting && dragType == notDragging)
        {
            selectRegion (CodeDocument::Position (selectionStart), CodeDocument::Position (selectionEnd));
            dragType = draggingSelectionEnd;
        }

        if (isHighlightActive() && ! (selecting || moveInWholeWordSteps))
        {
            moveCaretTo (selectionEnd, false);
            return true;
        }

        if (moveInWholeWordSteps)
            moveCaretTo (document.findWordBreakAfter (caretPos), selecting);
        else
            moveCaretTo (caretPos.movedBy (1), selecting);

        return true;
        */
    }
    
    pub fn move_line_delta(&mut self, 
        delta:     i32,
        selecting: bool)  {
        
        todo!();
        /*
            CodeDocument::Position pos (caretPos);
        auto newLineNum = pos.getLineNumber() + delta;

        if (columnToTryToMaintain < 0)
            columnToTryToMaintain = indexToColumn (pos.getLineNumber(), pos.getIndexInLine());

        pos.setLineAndIndex (newLineNum, columnToIndex (newLineNum, columnToTryToMaintain));

        auto colToMaintain = columnToTryToMaintain;
        moveCaretTo (pos, selecting);
        columnToTryToMaintain = colToMaintain;
        */
    }
    
    pub fn move_caret_down(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();

        if (caretPos.getLineNumber() == document.getNumLines() - 1)
            moveCaretTo (CodeDocument::Position (document, std::numeric_limits<int>::max(), std::numeric_limits<int>::max()), selecting);
        else
            moveLineDelta (1, selecting);

        return true;
        */
    }
    
    pub fn move_caret_up(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();

        if (caretPos.getLineNumber() == 0)
            moveCaretTo (CodeDocument::Position (document, 0, 0), selecting);
        else
            moveLineDelta (-1, selecting);

        return true;
        */
    }
    
    pub fn page_down(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        scrollBy (jlimit (0, linesOnScreen, 1 + document.getNumLines() - firstLineOnScreen - linesOnScreen));
        moveLineDelta (linesOnScreen, selecting);
        return true;
        */
    }
    
    pub fn page_up(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        scrollBy (-linesOnScreen);
        moveLineDelta (-linesOnScreen, selecting);
        return true;
        */
    }
    
    pub fn scroll_up(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        scrollBy (1);

        if (caretPos.getLineNumber() < firstLineOnScreen)
            moveLineDelta (1, false);

        return true;
        */
    }
    
    pub fn scroll_down(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        scrollBy (-1);

        if (caretPos.getLineNumber() >= firstLineOnScreen + linesOnScreen)
            moveLineDelta (-1, false);

        return true;
        */
    }
    
    pub fn move_caret_to_top(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        moveCaretTo (CodeDocument::Position (document, 0, 0), selecting);
        return true;
        */
    }
    
    pub fn move_caret_to_start_of_line(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();

        int index = CodeEditorHelpers::findFirstNonWhitespaceChar (caretPos.getLineText());

        if (index >= caretPos.getIndexInLine() && caretPos.getIndexInLine() > 0)
            index = 0;

        moveCaretTo (CodeDocument::Position (document, caretPos.getLineNumber(), index), selecting);
        return true;
        */
    }
    
    pub fn move_caret_to_end(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        moveCaretTo (CodeDocument::Position (document, std::numeric_limits<int>::max(),
                                             std::numeric_limits<int>::max()), selecting);
        return true;
        */
    }
    
    pub fn move_caret_to_end_of_line(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        moveCaretTo (CodeDocument::Position (document, caretPos.getLineNumber(),
                                             std::numeric_limits<int>::max()), selecting);
        return true;
        */
    }
    
    pub fn delete_backwards(&mut self, move_in_whole_word_steps: bool) -> bool {
        
        todo!();
        /*
            if (moveInWholeWordSteps)
        {
            cut(); // in case something is already highlighted
            moveCaretTo (document.findWordBreakBefore (caretPos), true);
        }
        else if (selectionStart == selectionEnd && ! skipBackwardsToPreviousTab())
        {
            selectionStart.moveBy (-1);
        }

        cut();
        return true;
        */
    }
    
    pub fn skip_backwards_to_previous_tab(&mut self) -> bool {
        
        todo!();
        /*
            auto currentLineText = caretPos.getLineText().removeCharacters ("\r\n");
        auto currentIndex = caretPos.getIndexInLine();

        if (currentLineText.isNotEmpty() && currentLineText.length() == currentIndex)
        {
            const int currentLine = caretPos.getLineNumber();
            const int currentColumn = indexToColumn (currentLine, currentIndex);
            const int previousTabColumn = (currentColumn - 1) - ((currentColumn - 1) % spacesPerTab);
            const int previousTabIndex = columnToIndex (currentLine, previousTabColumn);

            if (currentLineText.substring (previousTabIndex, currentIndex).trim().isEmpty())
            {
                selectionStart.moveBy (previousTabIndex - currentIndex);
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn delete_forwards(&mut self, move_in_whole_word_steps: bool) -> bool {
        
        todo!();
        /*
            if (moveInWholeWordSteps)
        {
            cut(); // in case something is already highlighted
            moveCaretTo (document.findWordBreakAfter (caretPos), true);
        }
        else
        {
            if (selectionStart == selectionEnd)
                selectionEnd.moveBy (1);
            else
                newTransaction();
        }

        cut();
        return true;
        */
    }
    
    pub fn select_all(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        selectRegion (CodeDocument::Position (document, std::numeric_limits<int>::max(),
                                              std::numeric_limits<int>::max()),
                      CodeDocument::Position (document, 0, 0));
        return true;
        */
    }
    
    pub fn select_region(
        &mut self, 
        start: &CodeDocumentPosition,
        end:   &CodeDocumentPosition

    ) {
        
        todo!();
        /*
            moveCaretTo (start, false);
        moveCaretTo (end, true);
        */
    }
    
    pub fn undo(&mut self) -> bool {
        
        todo!();
        /*
            if (readOnly)
            return false;

        ScopedValueSetter<bool> svs (shouldFollowDocumentChanges, true, false);
        document.undo();
        scrollToKeepCaretOnScreen();
        return true;
        */
    }
    
    pub fn redo(&mut self) -> bool {
        
        todo!();
        /*
            if (readOnly)
            return false;

        ScopedValueSetter<bool> svs (shouldFollowDocumentChanges, true, false);
        document.redo();
        scrollToKeepCaretOnScreen();
        return true;
        */
    }
    
    pub fn new_transaction(&mut self)  {
        
        todo!();
        /*
            document.newTransaction();
        impl->startTimer (600);
        */
    }
    
    /**
      | Specifies a command-manager which
      | the editor will notify whenever the
      | state of any of its commands changes.
      | 
      | If you're making use of the editor's
      | ApplicationCommandTarget interface,
      | then you should also use this to tell
      | it which command manager it should use.
      | Make sure that the manager does not go
      | out of scope while the editor is using
      | it. You can pass a nullptr here to disable
      | this.
      |
      */
    pub fn set_command_manager(&mut self, new_manager: *mut ApplicationCommandManager)  {
        
        todo!();
        /*
            appCommandManager = newManager;
        */
    }
    
    pub fn is_highlight_active(&self) -> bool {
        
        todo!();
        /*
            return selectionStart != selectionEnd;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (! TextEditorKeyMapper<CodeEditorComponent>::invokeKeyFunction (*this, key))
        {
            if (readOnly)
                return false;

            if (key == KeyPress::tabKey || key.getTextCharacter() == '\t')      handleTabKey();
            else if (key == KeyPress::returnKey)                                handleReturnKey();
            else if (key == KeyPress::escapeKey)                                handleEscapeKey();
            else if (key == KeyPress ('[', ModifierKeys::commandModifier, 0))   unindentSelection();
            else if (key == KeyPress (']', ModifierKeys::commandModifier, 0))   indentSelection();
            else if (key.getTextCharacter() >= ' ')                             insertTextAtCaret (String::charToString (key.getTextCharacter()));
            else                                                                return false;
        }

        impl->handleUpdateNowIfNeeded();
        return true;
        */
    }
    
    pub fn handle_return_key(&mut self)  {
        
        todo!();
        /*
            insertTextAtCaret (document.getNewLineCharacters());
        */
    }
    
    pub fn handle_tab_key(&mut self)  {
        
        todo!();
        /*
            insertTabAtCaret();
        */
    }
    
    pub fn handle_escape_key(&mut self)  {
        
        todo!();
        /*
            newTransaction();
        */
    }
    
    pub fn editor_viewport_position_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn caret_position_moved(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            return findFirstTargetParentComponent();
        */
    }
    
    pub fn get_all_commands(&mut self, commands: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            const CommandID ids[] = { StandardApplicationCommandIDs::cut,
                                  StandardApplicationCommandIDs::copy,
                                  StandardApplicationCommandIDs::paste,
                                  StandardApplicationCommandIDs::del,
                                  StandardApplicationCommandIDs::selectAll,
                                  StandardApplicationCommandIDs::undo,
                                  StandardApplicationCommandIDs::redo };

        commands.addArray (ids, numElementsInArray (ids));
        */
    }
    
    pub fn get_command_info(&mut self, 
        commandid: CommandID,
        result:    &mut ApplicationCommandInfo)  {
        
        todo!();
        /*
            const bool anythingSelected = isHighlightActive();

        switch (commandID)
        {
            case StandardApplicationCommandIDs::cut:
                result.setInfo (TRANS ("Cut"), TRANS ("Copies the currently selected text to the clipboard and deletes it."), "Editing", 0);
                result.setActive (anythingSelected && ! readOnly);
                result.defaultKeypresses.add (KeyPress ('x', ModifierKeys::commandModifier, 0));
                break;

            case StandardApplicationCommandIDs::copy:
                result.setInfo (TRANS ("Copy"), TRANS ("Copies the currently selected text to the clipboard."), "Editing", 0);
                result.setActive (anythingSelected);
                result.defaultKeypresses.add (KeyPress ('c', ModifierKeys::commandModifier, 0));
                break;

            case StandardApplicationCommandIDs::paste:
                result.setInfo (TRANS ("Paste"), TRANS ("Inserts text from the clipboard."), "Editing", 0);
                result.setActive (! readOnly);
                result.defaultKeypresses.add (KeyPress ('v', ModifierKeys::commandModifier, 0));
                break;

            case StandardApplicationCommandIDs::del:
                result.setInfo (TRANS ("Delete"), TRANS ("Deletes any selected text."), "Editing", 0);
                result.setActive (anythingSelected && ! readOnly);
                break;

            case StandardApplicationCommandIDs::selectAll:
                result.setInfo (TRANS ("Select All"), TRANS ("Selects all the text in the editor."), "Editing", 0);
                result.defaultKeypresses.add (KeyPress ('a', ModifierKeys::commandModifier, 0));
                break;

            case StandardApplicationCommandIDs::undo:
                result.setInfo (TRANS ("Undo"), TRANS ("Undo"), "Editing", 0);
                result.defaultKeypresses.add (KeyPress ('z', ModifierKeys::commandModifier, 0));
                result.setActive (document.getUndoManager().canUndo() && ! readOnly);
                break;

            case StandardApplicationCommandIDs::redo:
                result.setInfo (TRANS ("Redo"), TRANS ("Redo"), "Editing", 0);
                result.defaultKeypresses.add (KeyPress ('z', ModifierKeys::commandModifier | ModifierKeys::shiftModifier, 0));
                result.setActive (document.getUndoManager().canRedo() && ! readOnly);
                break;

            default:
                break;
        }
        */
    }
    
    pub fn perform(&mut self, info: &ApplicationCommandTargetInvocationInfo) -> bool {
        
        todo!();
        /*
            return performCommand (info.commandID);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            caret.reset (getLookAndFeel().createCaretComponent (this));
        addAndMakeVisible (caret.get());
        */
    }
    
    pub fn perform_command(&mut self, commandid: CommandID) -> bool {
        
        todo!();
        /*
            switch (commandID)
        {
            case StandardApplicationCommandIDs::cut:        cutToClipboard(); break;
            case StandardApplicationCommandIDs::copy:       copyToClipboard(); break;
            case StandardApplicationCommandIDs::paste:      pasteFromClipboard(); break;
            case StandardApplicationCommandIDs::del:        cut(); break;
            case StandardApplicationCommandIDs::selectAll:  selectAll(); break;
            case StandardApplicationCommandIDs::undo:       undo(); break;
            case StandardApplicationCommandIDs::redo:       redo(); break;
            default:                                        return false;
        }

        return true;
        */
    }
    
    pub fn set_selection(
        &mut self, 
        new_selection_start: CodeDocumentPosition,
        new_selection_end:   CodeDocumentPosition

    ) {
        
        todo!();
        /*
            if (selectionStart != newSelectionStart
            || selectionEnd != newSelectionEnd)
        {
            selectionStart = newSelectionStart;
            selectionEnd = newSelectionEnd;

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textSelectionChanged);
        }
        */
    }
    
    pub fn add_popup_menu_items(&mut self, 
        m:  &mut PopupMenu,
        _1: *const MouseEvent)  {
        
        todo!();
        /*
            m.addItem (StandardApplicationCommandIDs::cut,   TRANS ("Cut"), isHighlightActive() && ! readOnly);
        m.addItem (StandardApplicationCommandIDs::copy,  TRANS ("Copy"), ! getHighlightedRegion().isEmpty());
        m.addItem (StandardApplicationCommandIDs::paste, TRANS ("Paste"), ! readOnly);
        m.addItem (StandardApplicationCommandIDs::del,   TRANS ("Delete"), ! readOnly);
        m.addSeparator();
        m.addItem (StandardApplicationCommandIDs::selectAll, TRANS ("Select All"));
        m.addSeparator();
        m.addItem (StandardApplicationCommandIDs::undo,  TRANS ("Undo"), document.getUndoManager().canUndo());
        m.addItem (StandardApplicationCommandIDs::redo,  TRANS ("Redo"), document.getUndoManager().canRedo());
        */
    }
    
    pub fn perform_popup_menu_action(&mut self, menu_itemid: i32)  {
        
        todo!();
        /*
            performCommand (menuItemID);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            newTransaction();
        dragType = notDragging;

        if (e.mods.isPopupMenu())
        {
            setMouseCursor (MouseCursor::NormalCursor);

            if (getHighlightedRegion().isEmpty())
            {
                CodeDocument::Position start, end;
                document.findTokenContaining (getPositionAt (e.x, e.y), start, end);

                if (start.getPosition() < end.getPosition())
                    selectRegion (start, end);
            }

            PopupMenu m;
            m.setLookAndFeel (&getLookAndFeel());
            addPopupMenuItems (m, &e);

            m.showMenuAsync (typename PopupMenu::Options(),
                             ModalCallbackFunction::forComponent (codeEditorMenuCallback, this));
        }
        else
        {
            beginDragAutoRepeat (100);
            moveCaretTo (getPositionAt (e.x, e.y), e.mods.isShiftDown());
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (! e.mods.isPopupMenu())
            moveCaretTo (getPositionAt (e.x, e.y), true);
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            newTransaction();
        beginDragAutoRepeat (0);
        dragType = notDragging;
        setMouseCursor (MouseCursor::IBeamCursor);
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            CodeDocument::Position tokenStart (getPositionAt (e.x, e.y));
        CodeDocument::Position tokenEnd (tokenStart);

        if (e.getNumberOfClicks() > 2)
            document.findLineContaining (tokenStart, tokenStart, tokenEnd);
        else
            document.findTokenContaining (tokenStart, tokenStart, tokenEnd);

        selectRegion (tokenStart, tokenEnd);
        dragType = notDragging;
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if ((verticalScrollBar.isVisible() && wheel.deltaY != 0.0f)
             || (horizontalScrollBar.isVisible() && wheel.deltaX != 0.0f))
        {
            {
                MouseWheelDetails w (wheel);
                w.deltaX = 0;
                verticalScrollBar.mouseWheelMove (e, w);
            }

            {
                MouseWheelDetails w (wheel);
                w.deltaY = 0;
                horizontalScrollBar.mouseWheelMove (e, w);
            }
        }
        else
        {
            Component::mouseWheelMove (e, wheel);
        }
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            updateCaretPosition();
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            updateCaretPosition();
        */
    }
    
    /**
      | Changes the current tab settings.
      | 
      | This lets you change the tab size and
      | whether pressing the tab key inserts
      | a tab character, or its equivalent number
      | of spaces.
      |
      */
    pub fn set_tab_size(&mut self, 
        num_spaces:    i32,
        insert_spaces: bool)  {
        
        todo!();
        /*
            useSpacesForTabs = insertSpaces;

        if (spacesPerTab != numSpaces)
        {
            spacesPerTab = numSpaces;
            rebuildLineTokensAsync();
        }
        */
    }
    
    /**
      | Returns a string containing spaces
      | or tab characters to generate the given
      | number of spaces.
      |
      */
    pub fn get_tab_string(&self, num_spaces: i32) -> String {
        
        todo!();
        /*
            return String::repeatedString (useSpacesForTabs ? " " : "\t",
                                       useSpacesForTabs ? numSpaces
                                                        : (numSpaces / spacesPerTab));
        */
    }
    
    pub fn index_to_column(&self, 
        line_num: i32,
        index:    i32) -> i32 {
        
        todo!();
        /*
            auto line = document.getLine (lineNum);
        auto t = line.getCharPointer();
        int col = 0;

        for (int i = 0; i < index; ++i)
        {
            if (t.isEmpty())
            {
                jassertfalse;
                break;
            }

            if (t.getAndAdvance() != '\t')
                ++col;
            else
                col += getTabSize() - (col % getTabSize());
        }

        return col;
        */
    }
    
    pub fn column_to_index(&self, 
        line_num: i32,
        column:   i32) -> i32 {
        
        todo!();
        /*
            auto line = document.getLine (lineNum);
        auto t = line.getCharPointer();
        int i = 0, col = 0;

        while (! t.isEmpty())
        {
            if (t.getAndAdvance() != '\t')
                ++col;
            else
                col += getTabSize() - (col % getTabSize());

            if (col > column)
                break;

            ++i;
        }

        return i;
        */
    }
    
    /**
      | Changes the font.
      | 
      | Make sure you only use a fixed-width
      | font, or this component will look pretty
      | nasty!
      |
      */
    pub fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            font = newFont;
        charWidth = font.getStringWidthFloat ("0");
        lineHeight = roundToInt (font.getHeight());
        resized();
        */
    }
    
    /**
      | Changes the syntax highlighting scheme.
      | 
      | The token type values are dependent
      | on the tokeniser being used - use
      | 
      | CodeTokeniser::getTokenTypes()
      | to get a list of the token types. @see
      | getColourForTokenType
      |
      */
    pub fn set_colour_scheme(&mut self, scheme: &CodeEditorComponentColourScheme)  {
        
        todo!();
        /*
            colourScheme = scheme;
        repaint();
        */
    }
    
    /**
      | Returns one the syntax highlighting
      | colour for the given token.
      | 
      | The token type values are dependent
      | on the tokeniser being used. @see setColourScheme
      |
      */
    pub fn get_colour_for_token_type(&self, token_type: i32) -> Colour {
        
        todo!();
        /*
            return isPositiveAndBelow (tokenType, colourScheme.types.size())
                    ? colourScheme.types.getReference (tokenType).colour
                    : findColour (CodeEditorComponent::defaultTextColourId);
        */
    }
    
    pub fn clear_cached_iterators(&mut self, first_line_to_be_invalid: i32)  {
        
        todo!();
        /*
            int i;
        for (i = cachedIterators.size(); --i >= 0;)
            if (cachedIterators.getUnchecked (i).getLine() < firstLineToBeInvalid)
                break;

        cachedIterators.removeRange (jmax (0, i - 1), cachedIterators.size());
        */
    }
    
    pub fn update_cached_iterators(&mut self, max_line_num: i32)  {
        
        todo!();
        /*
            const int maxNumCachedPositions = 5000;
        const int linesBetweenCachedSources = jmax (10, document.getNumLines() / maxNumCachedPositions);

        if (cachedIterators.size() == 0)
            cachedIterators.add (CodeDocument::Iterator (document));

        if (codeTokeniser != nullptr)
        {
            for (;;)
            {
                const auto last = cachedIterators.getLast();

                if (last.getLine() >= maxLineNum)
                    break;

                cachedIterators.add (CodeDocument::Iterator (last));
                auto& t = cachedIterators.getReference (cachedIterators.size() - 1);
                const int targetLine = jmin (maxLineNum, last.getLine() + linesBetweenCachedSources);

                for (;;)
                {
                    codeTokeniser->readNextToken (t);

                    if (t.getLine() >= targetLine)
                        break;

                    if (t.isEOF())
                        return;
                }
            }
        }
        */
    }
    
    pub fn get_iterator_for_position(
        &mut self, 
        position: i32,
        source:   &mut CodeDocumentIterator

    ) {
        
        todo!();
        /*
            if (codeTokeniser != nullptr)
        {
            for (int i = cachedIterators.size(); --i >= 0;)
            {
                auto& t = cachedIterators.getReference (i);

                if (t.getPosition() <= position)
                {
                    source = t;
                    break;
                }
            }

            while (source.getPosition() < position)
            {
                const CodeDocument::Iterator original (source);
                codeTokeniser->readNextToken (source);

                if (source.getPosition() > position || source.isEOF())
                {
                    source = original;
                    break;
                }
            }
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<CodeEditorAccessibilityHandler> (*this);
        */
    }
}
