crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TextEditor.h]

pub enum TextEditorDragType
{
    notDragging,
    draggingSelectionStart,
    draggingSelectionEnd
}

impl<'a> TextInputTarget for TextEditor<'a> {

    fn is_text_input_active(&self) -> bool {
        
        todo!();
        /*
            return ! isReadOnly();
        */
    }

    /**
      | Returns the range of characters that
      | are selected.
      | 
      | If nothing is selected, this will return
      | an empty range. @see setHighlightedRegion
      |
      */
    fn get_highlighted_region(&self) -> Range<i32> {
        
        todo!();
        /*
            return selection;
        */
    }

    /**
      | Selects a section of the text.
      |
      */
    fn set_highlighted_region(&mut self, new_selection: &Range<i32>)  {
        
        todo!();
        /*
            moveCaretTo (newSelection.getStart(), false);
        moveCaretTo (newSelection.getEnd(), true);
        */
    }

    fn set_temporary_underlining(&mut self, new_underlined_sections: &[Range<i32>])  {
        
        todo!();
        /*
            underlinedSections = newUnderlinedSections;
        repaint();
        */
    }

    /**
      | Returns a section of the contents of
      | the editor.
      |
      */
    fn get_text_in_range(&self, range: &Range<i32>) -> String {
        
        todo!();
        /*
            if (range.isEmpty())
            return {};

        MemoryOutputStream mo;
        mo.preallocate ((size_t) jmin (getTotalNumChars(), range.getLength()));

        int index = 0;

        for (auto* s : sections)
        {
            auto nextIndex = index + s->getTotalLength();

            if (range.getStart() < nextIndex)
            {
                if (range.getEnd() <= index)
                    break;

                s->appendSubstring (mo, range - index);
            }

            index = nextIndex;
        }

        return mo.toUTF8();
        */
    }
    
    /**
      | Inserts some text at the current caret
      | position.
      | 
      | If a section of the text is highlighted,
      | it will be replaced by this string, otherwise
      | it will be inserted.
      | 
      | To delete a section of text, you can use
      | setHighlightedRegion() to highlight
      | it, and call insertTextAtCaret (String()).
      | 
      | @see setCaretPosition, getCaretPosition,
      | setHighlightedRegion
      |
      */
    fn insert_text_at_caret(&mut self, t: &String)  {
        
        todo!();
        /*
            String newText (inputFilter != nullptr ? inputFilter->filterNewText (*this, t) : t);

        if (isMultiLine())
            newText = newText.replace ("\r\n", "\n");
        else
            newText = newText.replaceCharacters ("\r\n", "  ");

        const int insertIndex = selection.getStart();
        const int newCaretPos = insertIndex + newText.length();

        remove (selection, getUndoManager(),
                newText.isNotEmpty() ? newCaretPos - 1 : newCaretPos);

        insert (newText, insertIndex, currentFont, findColour (textColourId),
                getUndoManager(), newCaretPos);

        textChanged();
        */
    }

    /**
      | Get the graphical position of the caret.
      | 
      | The rectangle returned is relative
      | to the component's top-left corner.
      | @see scrollEditorToPositionCaret
      |
      */
    fn get_caret_rectangle(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            return getCaretRectangleFloat().getSmallestIntegerContainer();
        */
    }
}

/**
  | An editable text box.
  | 
  | A TextEditor can either be in single-
  | or multi-line mode, and supports mixed
  | fonts and colours.
  | 
  | @see TextEditor::TextEditorListener, Label
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TextEditor<'a> {
    base:           Component<'a>,
    base3:          SettableTooltipClient,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the text
      | is changed.
      |
      */
    on_text_change: fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the return
      | key is pressed.
      |
      */
    on_return_key:  fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the escape
      | key is pressed.
      |
      */
    on_escape_key:  fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the editor
      | loses key focus.
      |
      */
    on_focus_lost:  fn() -> (),

    viewport:                     Box<Viewport<'a>>,
    text_holder:                  *mut TextHolderComponent<'a>,
    border_size:                  BorderSize<i32>, //{ 1, 1, 1, 3 };
    justification:                Justification, //{ Justification::topLeft };
    read_only:                    bool, // default = false
    caret_visible:                bool, // default = true
    multiline:                    bool, // default = false
    word_wrap:                    bool, // default = false
    return_key_starts_new_line:   bool, // default = false
    popup_menu_enabled:           bool, // default = true
    select_all_text_when_focused: bool, // default = false
    scrollbar_visible:            bool, // default = true
    was_focused:                  bool, // default = false
    keep_caret_on_screen:         bool, // default = true
    tab_key_used:                 bool, // default = false
    menu_active:                  bool, // default = false
    value_text_needs_updating:    bool, // default = false
    consume_esc_and_return_keys:  bool, // default = true
    underline_whitespace:         bool, // default = true
    undo_manager:                 UndoManager<'a>,
    caret:                        Box<CaretComponent<'a>>,
    selection:                    Range<i32>,
    left_indent:                  i32, // default = 4
    top_indent:                   i32, // default = 4
    last_transaction_time:        u32, // default = 0
    current_font:                 Font, // default = { 14.0f  }
    total_num_chars:              RefCell<i32>, // default = 0
    caret_position:               i32, // default = 0
    sections:                     Vec<Box<TextEditorUniformTextSection>>,
    text_to_show_when_empty:      String,
    colour_for_text_when_empty:   Colour,
    password_character:           wchar_t,
    input_filter:                 OptionalScopedPointer<Box<dyn TextEditorInputFilter>>,
    text_value:                   Value<'a>,
    keyboard_type:                VirtualKeyboardType, //= TextInputTarget::textKeyboard;
    line_spacing:                 f32, // default = 1.0f
    drag_type:                    TextEditorDragType, // default = notDragging
    listeners:                    ListenerList<Rc<RefCell<dyn TextEditorListener>>>,
    underlined_sections:          Vec<Range<i32>>,
}

pub const TEXT_EDITOR_TEXT_CHANGE_MESSAGE_ID:      i32 = 0x10003001;
pub const TEXT_EDITOR_RETURN_KEY_MESSAGE_ID:       i32 = 0x10003002;
pub const TEXT_EDITOR_ESCAPE_KEY_MESSAGE_ID:       i32 = 0x10003003;
pub const TEXT_EDITOR_FOCUS_LOSS_MESSAGE_ID:       i32 = 0x10003004;
pub const TEXT_EDITOR_MAX_ACTIONS_PER_TRANSACTION: i32 = 100;

pub fn text_editor_get_character_category(character: wchar_t) -> i32 {
    
    todo!();
        /*
            return CharacterFunctions::isLetterOrDigit (character)
                        ? 2 : (CharacterFunctions::isWhitespace (character) ? 0 : 1);
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TextEditor.cpp]
impl<'a> Drop for TextEditor<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (wasFocused)
            if (auto* peer = getPeer())
                peer->dismissPendingTextInput();

        textValue.removeListener (textHolder);
        textValue.referTo (Value());

        viewport.reset();
        textHolder = nullptr;
        */
    }
}

/**
  | Extra space for the cursor at the right-hand-edge
  |
  */
pub const right_edge_space: i32 = 2;

impl<'a> TextEditor<'a> {

    /**
      | Returns the value set by setReturnKeyStartsNewLine().
      | 
      | See setReturnKeyStartsNewLine()
      | for more info.
      |
      */
    pub fn get_return_key_starts_new_line(&self) -> bool {
        
        todo!();
        /*
            return returnKeyStartsNewLine;
        */
    }

    /**
      | Returns true if the tab key is being used
      | for input. @see setTabKeyUsedAsCharacter
      |
      */
    pub fn is_tab_key_used_as_character(&self) -> bool {
        
        todo!();
        /*
            return tabKeyUsed;
        */
    }

    /**
      | Returns true if the caret is enabled.
      | @see setCaretVisible
      |
      */
    pub fn is_caret_visible(&self) -> bool {
        
        todo!();
        /*
            return caretVisible && ! isReadOnly();
        */
    }
    
    /**
      | Returns true if scrollbars are enabled.
      | @see setScrollbarsShown
      |
      */
    pub fn are_scrollbars_shown(&self) -> bool {
        
        todo!();
        /*
            return scrollbarVisible;
        */
    }

    /**
      | Returns the current password character.
      | @see setPasswordCharacter
      |
      */
    pub fn get_password_character(&self) -> wchar_t {
        
        todo!();
        /*
            return passwordCharacter;
        */
    }
    
    /**
      | Returns true if the right-click menu
      | is enabled. @see setPopupMenuEnabled
      |
      */
    pub fn is_popup_menu_enabled(&self) -> bool {
        
        todo!();
        /*
            return popupMenuEnabled;
        */
    }

    /**
      | Returns true if a popup-menu is currently
      | being displayed.
      |
      */
    pub fn is_popup_menu_currently_active(&self) -> bool {
        
        todo!();
        /*
            return menuActive;
        */
    }
    
    /**
      | Returns the font that's currently being
      | used for new text.
      | 
      | @see setFont
      |
      */
    pub fn get_font(&self) -> &Font {
        
        todo!();
        /*
            return currentFont;
        */
    }

    /**
      | Sets whether whitespace should be underlined
      | when the editor font is underlined.
      | 
      | @see isWhitespaceUnderlined
      |
      */
    pub fn set_whitespace_underlined(&mut self, should_underline_whitespace: bool)  {
        
        todo!();
        /*
            underlineWhitespace = shouldUnderlineWhitespace;
        */
    }

    /**
      | Returns true if whitespace is underlined
      | for underlined fonts.
      | 
      | @see setWhitespaceIsUnderlined
      |
      */
    pub fn is_whitespace_underlined(&self) -> bool {
        
        todo!();
        /*
            return underlineWhitespace;
        */
    }

    /**
      | Returns the text that will be shown when
      | the text editor is empty.
      | 
      | @see setTextToShowWhenEmpty
      |
      */
    pub fn get_text_to_show_when_empty(&self) -> String {
        
        todo!();
        /*
            return textToShowWhenEmpty;
        */
    }

    /**
      | Returns the gap at the top edge of the
      | editor. @see setIndents
      |
      */
    pub fn get_top_indent(&self) -> i32 {
        
        todo!();
        /*
            return topIndent;
        */
    }

    /**
      | Returns the gap at the left edge of the
      | editor. @see setIndents
      |
      */
    pub fn get_left_indent(&self) -> i32 {
        
        todo!();
        /*
            return leftIndent;
        */
    }

    /**
      | Returns the type of justification,
      | as set in setJustification().
      |
      */
    pub fn get_justification_type(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }

    /**
      | Sets the line spacing of the TextEditor.
      | 
      | The default (and minimum) value is 1.0
      | and values > 1.0 will increase the line
      | spacing as a multiple of the line height
      | e.g. for double-spacing call this method
      | with an argument of 2.0.
      |
      */
    pub fn set_line_spacing(&mut self, new_line_spacing: f32)  {
        
        todo!();
        /*
            lineSpacing = jmax (1.0f, newLineSpacing);
        */
    }

    /**
      | Returns the current line spacing of
      | the TextEditor.
      |
      */
    pub fn get_line_spacing(&self) -> f32 {
        
        todo!();
        /*
            return lineSpacing;
        */
    }

    /**
      | Returns the current TextEditorInputFilter, as
      | set by setInputFilter().
      |
      */
    pub fn get_input_filter(&self) -> *mut dyn TextEditorInputFilter {
        
        todo!();
        /*
            return inputFilter;
        */
    }

    pub fn set_keyboard_type(&mut self, ty: VirtualKeyboardType)  {
        
        todo!();
        /*
            keyboardType = type;
        */
    }
    
    pub fn get_keyboard_type(&mut self) -> VirtualKeyboardType {
        
        todo!();
        /*
            return keyboardType;
        */
    }
    
    /**
      | Creates a new, empty text editor.
      | 
      | -----------
      | @param componentName
      | 
      | the name to pass to the component for
      | it to use as its name
      | ----------
      | @param passwordCharacter
      | 
      | if this is not zero, this character will
      | be used as a replacement for all characters
      | that are drawn on screen - e.g. to create
      | a password-style textbox containing
      | circular blobs instead of text, you
      | could set this value to 0x25cf, which
      | is the unicode character for a black
      | splodge (not all fonts include this,
      | though), or 0x2022, which is a bullet
      | (probably the best choice for linux).
      |
      */
    pub fn new(
        name:          Option<&String>,
        password_char: Option<wchar_t>

    ) -> Self {

        let name = name.unwrap_or(&String::new());

        let password_char = password_char.unwrap_or(0);
    
        todo!();
        /*
        : component(name),
        : password_character(passwordChar),

            setMouseCursor (MouseCursor::IBeamCursor);

        viewport.reset (new TextEditorViewport (*this));
        addAndMakeVisible (viewport.get());
        viewport->setViewedComponent (textHolder = new TextHolderComponent (*this));
        viewport->setWantsKeyboardFocus (false);
        viewport->setScrollBarsShown (false, false);

        setWantsKeyboardFocus (true);
        recreateCaret();
        */
    }
    
    /**
      | Begins a new transaction in the UndoManager.
      |
      */
    pub fn new_transaction(&mut self)  {
        
        todo!();
        /*
            lastTransactionTime = Time::getApproximateMillisecondCounter();
        undoManager.beginNewTransaction();
        */
    }
    
    pub fn undo_or_redo(&mut self, should_undo: bool) -> bool {
        
        todo!();
        /*
            if (! isReadOnly())
        {
            newTransaction();

            if (shouldUndo ? undoManager.undo()
                           : undoManager.redo())
            {
                repaint();
                textChanged();
                scrollToMakeSureCursorIsVisible();

                return true;
            }
        }

        return false;
        */
    }
    
    pub fn undo(&mut self) -> bool {
        
        todo!();
        /*
            return undoOrRedo (true);
        */
    }
    
    pub fn redo(&mut self) -> bool {
        
        todo!();
        /*
            return undoOrRedo (false);
        */
    }
    
    /**
      | Puts the editor into either multi- or
      | single-line mode.
      | 
      | By default, the editor will be in single-line
      | mode, so use this if you need a multi-line
      | editor.
      | 
      | See also the setReturnKeyStartsNewLine()
      | method, which will also need to be turned
      | on if you want a multi-line editor with
      | line-breaks.
      | 
      | -----------
      | @param shouldBeMultiLine
      | 
      | whether the editor should be multi-
      | or single-line.
      | ----------
      | @param shouldWordWrap
      | 
      | sets whether long lines should be broken
      | up in multi-line editors.
      | 
      | If this is false and scrollbars are enabled
      | a horizontal scrollbar will be shown.
      | 
      | @see isMultiLine, setReturnKeyStartsNewLine,
      | setScrollbarsShown
      |
      */
    pub fn set_multi_line(
        &mut self, 
        should_be_multi_line: bool,
        should_word_wrap:     Option<bool>

    ) {

        let should_word_wrap: bool = should_word_wrap.unwrap_or(true);
        
        todo!();
        /*
            if (multiline != shouldBeMultiLine
             || wordWrap != (shouldWordWrap && shouldBeMultiLine))
        {
            multiline = shouldBeMultiLine;
            wordWrap = shouldWordWrap && shouldBeMultiLine;

            checkLayout();

            viewport->setViewPosition (0, 0);
            resized();
            scrollToMakeSureCursorIsVisible();
        }
        */
    }
    
    /**
      | Returns true if the editor is in multi-line
      | mode.
      |
      */
    pub fn is_multi_line(&self) -> bool {
        
        todo!();
        /*
            return multiline;
        */
    }
    
    /**
      | Enables or disables scrollbars (this
      | only applies when in multi-line mode).
      | 
      | When the text gets too long to fit in the
      | component, a scrollbar can appear to
      | allow it to be scrolled. Even when this
      | is enabled, the scrollbar will be hidden
      | unless it's needed.
      | 
      | By default scrollbars are enabled.
      |
      */
    pub fn set_scrollbars_shown(&mut self, shown: bool)  {
        
        todo!();
        /*
            if (scrollbarVisible != shown)
        {
            scrollbarVisible = shown;
            checkLayout();
        }
        */
    }
    
    /**
      | Changes the editor to read-only mode.
      | 
      | By default, the text editor is not read-only.
      | If you're making it read-only, you might
      | also want to call setCaretVisible (false)
      | to get rid of the caret.
      | 
      | The text can still be highlighted and
      | copied when in read-only mode.
      | 
      | @see isReadOnly, setCaretVisible
      |
      */
    pub fn set_read_only(&mut self, should_be_read_only: bool)  {
        
        todo!();
        /*
            if (readOnly != shouldBeReadOnly)
        {
            readOnly = shouldBeReadOnly;
            enablementChanged();
            invalidateAccessibilityHandler();
        }
        */
    }
    
    /**
      | Returns true if the editor is in read-only
      | mode.
      |
      */
    pub fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return readOnly || ! isEnabled();
        */
    }
    
    /**
      | Changes the behaviour of the return
      | key.
      | 
      | If set to true, the return key will insert
      | a new-line into the text; if false it
      | will trigger a call to the TextEditor::TextEditorListener::textEditorReturnKeyPressed()
      | method. By default this is set to false,
      | and when true it will only insert new-lines
      | when in multi-line mode (see setMultiLine()).
      |
      */
    pub fn set_return_key_starts_new_line(&mut self, should_start_new_line: bool)  {
        
        todo!();
        /*
            returnKeyStartsNewLine = shouldStartNewLine;
        */
    }
    
    /**
      | Indicates whether the tab key should
      | be accepted and used to input a tab character,
      | or whether it gets ignored.
      | 
      | By default the tab key is ignored, so
      | that it can be used to switch keyboard
      | focus between components.
      |
      */
    pub fn set_tab_key_used_as_character(&mut self, should_tab_key_be_used: bool)  {
        
        todo!();
        /*
            tabKeyUsed = shouldTabKeyBeUsed;
        */
    }
    
    /**
      | Allows a right-click menu to appear
      | for the editor.
      | 
      | (This defaults to being enabled).
      | 
      | If enabled, right-clicking (or command-clicking
      | on the Mac) will pop up a menu of options
      | such as cut/copy/paste, undo/redo,
      | etc.
      |
      */
    pub fn set_popup_menu_enabled(&mut self, b: bool)  {
        
        todo!();
        /*
            popupMenuEnabled = b;
        */
    }
    
    /**
      | If set to true, focusing on the editor
      | will highlight all its text.
      | 
      | (Set to false by default).
      | 
      | This is useful for boxes where you expect
      | the user to re-enter all the text when
      | they focus on the component, rather
      | than editing what's already there.
      |
      */
    pub fn set_select_all_when_focused(&mut self, b: bool)  {
        
        todo!();
        /*
            selectAllTextWhenFocused = b;
        */
    }
    
    /**
      | Modifies the justification of the text
      | within the editor window.
      |
      */
    pub fn set_justification(&mut self, j: Justification)  {
        
        todo!();
        /*
            if (justification != j)
        {
            justification = j;

            resized();
            repaint();
        }
        */
    }
    
    /**
      | Sets the font to use for newly added text.
      | 
      | This will change the font that will be
      | used next time any text is added or entered
      | into the editor. It won't change the
      | font of any existing text - to do that,
      | use applyFontToAllText() instead.
      | 
      | @see applyFontToAllText
      |
      */
    pub fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            currentFont = newFont;
        scrollToMakeSureCursorIsVisible();
        */
    }
    
    /**
      | Applies a font to all the text in the editor.
      | 
      | If the changeCurrentFont argument
      | is true then this will also set the new
      | font as the font to be used for any new
      | text that's added.
      | 
      | @see setFont
      |
      */
    pub fn apply_font_to_all_text(
        &mut self, 
        new_font:            &Font,
        change_current_font: Option<bool>

    ) {

        let change_current_font: bool = change_current_font.unwrap_or(true);

        
        todo!();
        /*
            if (changeCurrentFont)
            currentFont = newFont;

        auto overallColour = findColour (textColourId);

        for (auto* uts : sections)
        {
            uts->setFont (newFont, passwordCharacter);
            uts->colour = overallColour;
        }

        coalesceSimilarSections();
        checkLayout();
        scrollToMakeSureCursorIsVisible();
        repaint();
        */
    }
    
    /**
      | Applies a colour to all the text in the
      | editor.
      | 
      | If the changeCurrentTextColour argument
      | is true then this will also set the new
      | colour as the colour to be used for any
      | new text that's added.
      |
      */
    pub fn apply_colour_to_all_text(
        &mut self, 
        new_colour:                 &Colour,
        change_current_text_colour: Option<bool>

    ) {

        let change_current_text_colour: bool = change_current_text_colour.unwrap_or(true);
        
        todo!();
        /*
            for (auto* uts : sections)
            uts->colour = newColour;

        if (changeCurrentTextColour)
            setColour (TextEditor::textColourId, newColour);
        else
            repaint();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            caret.reset();
        recreateCaret();
        repaint();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            recreateCaret();
        repaint();
        */
    }
    
    /**
      | Makes the caret visible or invisible.
      | 
      | By default the caret is visible. @see
      | setCaretColour, setCaretPosition
      |
      */
    pub fn set_caret_visible(&mut self, should_caret_be_visible: bool)  {
        
        todo!();
        /*
            if (caretVisible != shouldCaretBeVisible)
        {
            caretVisible = shouldCaretBeVisible;
            recreateCaret();
        }
        */
    }
    
    pub fn recreate_caret(&mut self)  {
        
        todo!();
        /*
            if (isCaretVisible())
        {
            if (caret == nullptr)
            {
                caret.reset (getLookAndFeel().createCaretComponent (this));
                textHolder->addChildComponent (caret.get());
                updateCaretPosition();
            }
        }
        else
        {
            caret.reset();
        }
        */
    }
    
    pub fn update_caret_position(&mut self)  {
        
        todo!();
        /*
            if (caret != nullptr
            && getWidth() > 0 && getHeight() > 0)
        {
            TextEditorIterator i (*this);
            caret->setCaretPosition (getCaretRectangle().translated (leftIndent,
                                                                     topIndent + roundToInt (i.getYOffset())));

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textSelectionChanged);
        }
        */
    }
    
    /**
      | Sets an input filter that should be applied
      | to this editor.
      | 
      | The filter can be nullptr, to remove
      | any existing filters.
      | 
      | If takeOwnership is true, then the filter
      | will be owned and deleted by the editor
      | when no longer needed.
      |
      */
    pub fn set_input_filter(
        &mut self, 
        new_filter:     *mut dyn TextEditorInputFilter,
        take_ownership: bool

    ) {
        
        todo!();
        /*
            inputFilter.set (newFilter, takeOwnership);
        */
    }
    
    /**
      | Sets limits on the characters that can
      | be entered.
      | 
      | This is just a shortcut that passes an
      | instance of the TextEditorLengthAndCharacterRestriction
      | class to setInputFilter().
      | 
      | -----------
      | @param maxTextLength
      | 
      | if this is > 0, it sets a maximum length
      | limit; if 0, no limit is set
      | ----------
      | @param allowedCharacters
      | 
      | if this is non-empty, then only characters
      | that occur in this string are allowed
      | to be entered into the editor.
      |
      */
    pub fn set_input_restrictions(
        &mut self, 
        max_len: i32,
        chars:   Option<&String>

    ) {

        let chars: &String = chars.unwrap_or(&String::new());
        
        todo!();
        /*
            setInputFilter (new TextEditorLengthAndCharacterRestriction (maxLen, chars), true);
        */
    }
    
    /**
      | When the text editor is empty, it can
      | be set to display a message.
      | 
      | This is handy for things like telling
      | the user what to type in the box - the string
      | is only displayed, it's not taken to
      | actually be the contents of the editor.
      |
      */
    pub fn set_text_to_show_when_empty(&mut self, 
        text:          &String,
        colour_to_use: Colour)  {
        
        todo!();
        /*
            textToShowWhenEmpty = text;
        colourForTextWhenEmpty = colourToUse;
        */
    }
    
    /**
      | Changes the password character used
      | to disguise the text.
      | 
      | -----------
      | @param passwordCharacter
      | 
      | if this is not zero, this character will
      | be used as a replacement for all characters
      | that are drawn on screen - e.g. to create
      | a password-style textbox containing
      | circular blobs instead of text, you
      | could set this value to 0x25cf, which
      | is the unicode character for a black
      | splodge (not all fonts include this,
      | though), or 0x2022, which is a bullet
      | (probably the best choice for linux).
      |
      */
    pub fn set_password_character(&mut self, new_password_character: wchar_t)  {
        
        todo!();
        /*
            if (passwordCharacter != newPasswordCharacter)
        {
            passwordCharacter = newPasswordCharacter;
            applyFontToAllText (currentFont);
        }
        */
    }
    
    /**
      | Changes the size of the scrollbars that
      | are used.
      | 
      | Handy if you need smaller scrollbars
      | for a small text box.
      |
      */
    pub fn set_scroll_bar_thickness(&mut self, new_thickness_pixels: i32)  {
        
        todo!();
        /*
            viewport->setScrollBarThickness (newThicknessPixels);
        */
    }
    
    /**
      | Deletes all the text from the editor.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            clearInternal (nullptr);
        checkLayout();
        undoManager.clearUndoHistory();
        */
    }
    
    /**
      | Sets the entire content of the editor.
      | 
      | This will clear the editor and insert
      | the given text (using the current text
      | colour and font). You can set the current
      | text colour using
      | 
      | -----------
      | @param newText
      | 
      | the text to add
      | ----------
      | @param sendTextChangeMessage
      | 
      | if true, this will cause a change message
      | to be sent to all the listeners. @see
      | insertTextAtCaret
      | 
      | -----------
      | @code
      | 
      | setColour (TextEditor::textColourId, ...);
      |
      */
    pub fn set_text(
        &mut self, 
        new_text:                 &String,
        send_text_change_message: Option<bool>

    ) {

        let send_text_change_message: bool = send_text_change_message.unwrap_or(true);
        
        todo!();
        /*
            auto newLength = newText.length();

        if (newLength != getTotalNumChars() || getText() != newText)
        {
            if (! sendTextChangeMessage)
                textValue.removeListener (textHolder);

            textValue = newText;

            auto oldCursorPos = caretPosition;
            bool cursorWasAtEnd = oldCursorPos >= getTotalNumChars();

            clearInternal (nullptr);
            insert (newText, 0, currentFont, findColour (textColourId), nullptr, caretPosition);

            // if you're adding text with line-feeds to a single-line text editor, it
            // ain't gonna look right!
            jassert (multiline || ! newText.containsAnyOf ("\r\n"));

            if (cursorWasAtEnd && ! isMultiLine())
                oldCursorPos = getTotalNumChars();

            moveCaretTo (oldCursorPos, false);

            if (sendTextChangeMessage)
                textChanged();
            else
                textValue.addListener (textHolder);

            checkLayout();
            scrollToMakeSureCursorIsVisible();
            undoManager.clearUndoHistory();

            repaint();
        }
        */
    }
    
    pub fn update_value_from_text(&mut self)  {
        
        todo!();
        /*
            if (valueTextNeedsUpdating)
        {
            valueTextNeedsUpdating = false;
            textValue = getText();
        }
        */
    }
    
    /**
      | Returns a Value object that can be used
      | to get or set the text.
      | 
      | Bear in mind that this operate quite
      | slowly if your text box contains large
      | amounts of text, as it needs to dynamically
      | build the string that's involved.
      | 
      | It's best used for small text boxes.
      |
      */
    pub fn get_text_value(&mut self) -> &mut Value {
        
        todo!();
        /*
            updateValueFromText();
        return textValue;
        */
    }
    
    pub fn text_was_changed_by_value(&mut self)  {
        
        todo!();
        /*
            if (textValue.getValueSource().getReferenceCount() > 1)
            setText (textValue.getValue());
        */
    }
    
    /**
      | Used internally to dispatch a text-change
      | message.
      |
      */
    pub fn text_changed(&mut self)  {
        
        todo!();
        /*
            checkLayout();

        if (listeners.size() != 0 || onTextChange != nullptr)
            postCommandMessage (TextEditorDefs::textChangeMessageId);

        if (textValue.getValueSource().getReferenceCount() > 1)
        {
            valueTextNeedsUpdating = false;
            textValue = getText();
        }

        if (auto* handler = getAccessibilityHandler())
            handler->notifyAccessibilityEvent (AccessibilityEvent::textChanged);
        */
    }
    
    pub fn set_selection(&mut self, new_selection: Range<i32>)  {
        
        todo!();
        /*
            if (newSelection != selection)
        {
            selection = newSelection;

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textSelectionChanged);
        }
        */
    }
    
    pub fn return_pressed(&mut self)  {
        
        todo!();
        /*
            postCommandMessage (TextEditorDefs::returnKeyMessageId);
        */
    }
    
    pub fn escape_pressed(&mut self)  {
        
        todo!();
        /*
            postCommandMessage (TextEditorDefs::escapeKeyMessageId);
        */
    }
    
    /**
      | Registers a listener to be told when
      | things happen to the text. @see removeListener
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn TextEditorListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Deregisters a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn TextEditorListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn timer_callback_int(&mut self)  {
        
        todo!();
        /*
            checkFocus();

        auto now = Time::getApproximateMillisecondCounter();

        if (now > lastTransactionTime + 200)
            newTransaction();
        */
    }
    
    pub fn check_focus(&mut self)  {
        
        todo!();
        /*
            if (! wasFocused && hasKeyboardFocus (false) && ! isCurrentlyBlockedByAnotherModalComponent())
        {
            wasFocused = true;

            if (auto* peer = getPeer())
                if (! isReadOnly())
                    peer->textInputRequired (peer->globalToLocal (getScreenPosition()), *this);
        }
        */
    }
    
    pub fn repaint_text(&mut self, range: Range<i32>)  {
        
        todo!();
        /*
            if (! range.isEmpty())
        {
            if (range.getEnd() >= getTotalNumChars())
            {
                textHolder->repaint();
                return;
            }

            TextEditorIterator i (*this);

            Point<float> anchor;
            auto lh = currentFont.getHeight();
            i.getCharPosition (range.getStart(), anchor, lh);

            auto y1 = std::trunc (anchor.y);
            int y2 = 0;

            if (range.getEnd() >= getTotalNumChars())
            {
                y2 = textHolder->getHeight();
            }
            else
            {
                i.getCharPosition (range.getEnd(), anchor, lh);
                y2 = (int) (anchor.y + lh * 2.0f);
            }

            auto offset = i.getYOffset();
            textHolder->repaint (0, roundToInt (y1 + offset), textHolder->getWidth(), roundToInt ((float) y2 - y1 + offset));
        }
        */
    }
    
    pub fn move_caret(&mut self, new_caret_pos: i32)  {
        
        todo!();
        /*
            if (newCaretPos < 0)
            newCaretPos = 0;
        else
            newCaretPos = jmin (newCaretPos, getTotalNumChars());

        if (newCaretPos != getCaretPosition())
        {
            caretPosition = newCaretPos;

            if (hasKeyboardFocus (false))
                textHolder->restartTimer();

            scrollToMakeSureCursorIsVisible();
            updateCaretPosition();

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::textChanged);
        }
        */
    }
    
    /**
      | Returns the current index of the caret.
      | @see setCaretPosition
      |
      */
    pub fn get_caret_position(&self) -> i32 {
        
        todo!();
        /*
            return caretPosition;
        */
    }
    
    /**
      | Moves the caret to be in front of a given
      | character. @see getCaretPosition,
      | moveCaretToEnd
      |
      */
    pub fn set_caret_position(&mut self, new_index: i32)  {
        
        todo!();
        /*
            moveCaretTo (newIndex, false);
        */
    }
    
    pub fn move_caret_to_end(&mut self)  {
        
        todo!();
        /*
            setCaretPosition (std::numeric_limits<int>::max());
        */
    }
    
    /**
      | Attempts to scroll the text editor so
      | that the caret ends up at a specified
      | position.
      | 
      | This won't affect the caret's position
      | within the text, it tries to scroll the
      | entire editor vertically and horizontally
      | so that the caret is sitting at the given
      | position (relative to the top-left
      | of this component).
      | 
      | Depending on the amount of text available,
      | it might not be possible to scroll far
      | enough for the caret to reach this exact
      | position, but it will go as far as it can
      | in that direction.
      |
      */
    pub fn scroll_editor_to_position_caret(&mut self, 
        desired_caretx: i32,
        desired_carety: i32)  {
        
        todo!();
        /*
            updateCaretPosition();
        auto caretRect = getCaretRectangle().translated (leftIndent, topIndent);

        auto vx = caretRect.getX() - desiredCaretX;
        auto vy = caretRect.getY() - desiredCaretY;

        if (desiredCaretX < jmax (1, proportionOfWidth (0.05f)))
            vx += desiredCaretX - proportionOfWidth (0.2f);
        else if (desiredCaretX > jmax (0, viewport->getMaximumVisibleWidth() - (wordWrap ? 2 : 10)))
            vx += desiredCaretX + (isMultiLine() ? proportionOfWidth (0.2f) : 10) - viewport->getMaximumVisibleWidth();

        vx = jlimit (0, jmax (0, textHolder->getWidth() + 8 - viewport->getMaximumVisibleWidth()), vx);

        if (! isMultiLine())
        {
            vy = viewport->getViewPositionY();
        }
        else
        {
            vy = jlimit (0, jmax (0, textHolder->getHeight() - viewport->getMaximumVisibleHeight()), vy);

            if (desiredCaretY < 0)
                vy = jmax (0, desiredCaretY + vy);
            else if (desiredCaretY > jmax (0, viewport->getMaximumVisibleHeight() - caretRect.getHeight()))
                vy += desiredCaretY + 2 + caretRect.getHeight() - viewport->getMaximumVisibleHeight();
        }

        viewport->setViewPosition (vx, vy);
        */
    }
    
    pub fn get_caret_rectangle_float(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            Point<float> anchor;
        auto cursorHeight = currentFont.getHeight(); // (in case the text is empty and the call below doesn't set this value)
        getCharPosition (caretPosition, anchor, cursorHeight);

        return { anchor.x, anchor.y, 2.0f, cursorHeight };
        */
    }
    
    pub fn get_text_offset(&self) -> Point<i32> {
        
        todo!();
        /*
            TextEditorIterator i (*this);
        auto yOffset = i.getYOffset();

        return { getLeftIndent() + borderSize.getLeft() - viewport->getViewPositionX(),
                 roundToInt ((float) getTopIndent() + (float) borderSize.getTop() + yOffset) - viewport->getViewPositionY() };
        */
    }
    
    /**
      | Returns the bounding box for a range
      | of text in the editor. As the range may
      | span multiple lines, this method returns
      | a RectangleList.
      | 
      | The bounds are relative to the component's
      | top-left and may extend beyond the bounds
      | of the component if the text is long and
      | word wrapping is disabled.
      |
      */
    pub fn get_text_bounds(&mut self, text_range: Range<i32>) -> RectangleList<i32> {
        
        todo!();
        /*
            RectangleList<int> boundingBox;
        TextEditorIterator i (*this);

        while (i.next())
        {
            if (textRange.intersects ({ i.indexInText,
                                        i.indexInText + i.atom->numChars }))
            {
                boundingBox.add (i.getTextBounds (textRange));
            }
        }

        boundingBox.offsetAll (getTextOffset());
        return boundingBox;
        */
    }
    
    pub fn get_word_wrap_width(&self) -> i32 {
        
        todo!();
        /*
            return wordWrap ? getMaximumTextWidth()
                        : std::numeric_limits<int>::max();
        */
    }
    
    pub fn get_maximum_text_width(&self) -> i32 {
        
        todo!();
        /*
            return jmax (1, viewport->getMaximumVisibleWidth() - leftIndent - rightEdgeSpace);
        */
    }
    
    pub fn get_maximum_text_height(&self) -> i32 {
        
        todo!();
        /*
            return jmax (1, viewport->getMaximumVisibleHeight() - topIndent);
        */
    }
    
    pub fn check_layout(&mut self)  {
        
        todo!();
        /*
            if (getWordWrapWidth() > 0)
        {
            const auto textBottom = TextEditorIterator (*this).getTotalTextHeight() + topIndent;
            const auto textRight = jmax (viewport->getMaximumVisibleWidth(),
                                         TextEditorIterator (*this).getTextRight() + leftIndent + rightEdgeSpace);

            textHolder->setSize (textRight, textBottom);
            viewport->setScrollBarsShown (scrollbarVisible && multiline && textBottom > viewport->getMaximumVisibleHeight(),
                                          scrollbarVisible && multiline && ! wordWrap && textRight > viewport->getMaximumVisibleWidth());
        }
        */
    }
    
    /**
      | Returns the total width of the text,
      | as it is currently laid-out.
      | 
      | This may be larger than the size of the
      | TextEditor, and can change when the
      | TextEditor is resized or the text changes.
      |
      */
    pub fn get_text_width(&self) -> i32 {
        
        todo!();
        /*
            return textHolder->getWidth();
        */
    }
    
    /**
      | Returns the maximum height of the text,
      | as it is currently laid-out.
      | 
      | This may be larger than the size of the
      | TextEditor, and can change when the
      | TextEditor is resized or the text changes.
      |
      */
    pub fn get_text_height(&self) -> i32 {
        
        todo!();
        /*
            return textHolder->getHeight();
        */
    }
    
    /**
      | Changes the size of the gap at the top
      | and left-edge of the editor.
      | 
      | By default there's a gap of 4 pixels.
      |
      */
    pub fn set_indents(&mut self, 
        new_left_indent: i32,
        new_top_indent:  i32)  {
        
        todo!();
        /*
            if (leftIndent != newLeftIndent || topIndent != newTopIndent)
        {
            leftIndent = newLeftIndent;
            topIndent  = newTopIndent;

            resized();
            repaint();
        }
        */
    }
    
    /**
      | Changes the size of border left around
      | the edge of the component. @see getBorder
      |
      */
    pub fn set_border(&mut self, border: BorderSize<i32>)  {
        
        todo!();
        /*
            borderSize = border;
        resized();
        */
    }
    
    /**
      | Returns the size of border around the
      | edge of the component. @see setBorder
      |
      */
    pub fn get_border(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            return borderSize;
        */
    }
    
    /**
      | Used to disable the auto-scrolling
      | which keeps the caret visible.
      | 
      | If true (the default), the editor will
      | scroll when the caret moves offscreen.
      | If set to false, it won't.
      |
      */
    pub fn set_scroll_to_show_cursor(&mut self, should_scroll_to_show_cursor: bool)  {
        
        todo!();
        /*
            keepCaretOnScreen = shouldScrollToShowCursor;
        */
    }
    
    /**
      | Scrolls the minimum distance needed
      | to get the caret into view.
      |
      */
    pub fn scroll_to_make_sure_cursor_is_visible(&mut self)  {
        
        todo!();
        /*
            updateCaretPosition();

        if (keepCaretOnScreen)
        {
            auto viewPos = viewport->getViewPosition();
            auto caretRect = getCaretRectangle().translated (leftIndent, topIndent);
            auto relativeCursor = caretRect.getPosition() - viewPos;

            if (relativeCursor.x < jmax (1, proportionOfWidth (0.05f)))
            {
                viewPos.x += relativeCursor.x - proportionOfWidth (0.2f);
            }
            else if (relativeCursor.x > jmax (0, viewport->getMaximumVisibleWidth() - (wordWrap ? 2 : 10)))
            {
                viewPos.x += relativeCursor.x + (isMultiLine() ? proportionOfWidth (0.2f) : 10) - viewport->getMaximumVisibleWidth();
            }

            viewPos.x = jlimit (0, jmax (0, textHolder->getWidth() + 8 - viewport->getMaximumVisibleWidth()), viewPos.x);

            if (! isMultiLine())
            {
                viewPos.y = (getHeight() - textHolder->getHeight() - topIndent) / -2;
            }
            else if (relativeCursor.y < 0)
            {
                viewPos.y = jmax (0, relativeCursor.y + viewPos.y);
            }
            else if (relativeCursor.y > jmax (0, viewport->getMaximumVisibleHeight() - caretRect.getHeight()))
            {
                viewPos.y += relativeCursor.y + 2 + caretRect.getHeight() - viewport->getMaximumVisibleHeight();
            }

            viewport->setViewPosition (viewPos);
        }
        */
    }
    
    pub fn move_caret_to(&mut self, 
        new_position: i32,
        is_selecting: bool)  {
        
        todo!();
        /*
            if (isSelecting)
        {
            moveCaret (newPosition);

            auto oldSelection = selection;

            if (dragType == notDragging)
            {
                if (std::abs (getCaretPosition() - selection.getStart()) < std::abs (getCaretPosition() - selection.getEnd()))
                    dragType = draggingSelectionStart;
                else
                    dragType = draggingSelectionEnd;
            }

            if (dragType == draggingSelectionStart)
            {
                if (getCaretPosition() >= selection.getEnd())
                    dragType = draggingSelectionEnd;

                setSelection (Range<int>::between (getCaretPosition(), selection.getEnd()));
            }
            else
            {
                if (getCaretPosition() < selection.getStart())
                    dragType = draggingSelectionStart;

                setSelection (Range<int>::between (getCaretPosition(), selection.getStart()));
            }

            repaintText (selection.getUnionWith (oldSelection));
        }
        else
        {
            dragType = notDragging;

            repaintText (selection);

            moveCaret (newPosition);
            setSelection (Range<int>::emptyRange (getCaretPosition()));
        }
        */
    }
    
    /**
      | Finds the index of the character at a
      | given position.
      | 
      | The coordinates are relative to the
      | component's top-left.
      |
      */
    pub fn get_text_index_at(&self, x: i32, y: i32) -> i32 {
        
        todo!();
        /*
            const auto offset = getTextOffset();

        return indexAtPosition ((float) (x - offset.x),
                                (float) (y - offset.y));
        */
    }
    
    /**
      | Copies the currently selected region
      | to the clipboard. @see cut, paste, SystemClipboard
      |
      */
    pub fn copy(&mut self)  {
        
        todo!();
        /*
            if (passwordCharacter == 0)
        {
            auto selectedText = getHighlightedText();

            if (selectedText.isNotEmpty())
                SystemClipboard::copyTextToClipboard (selectedText);
        }
        */
    }
    
    /**
      | Pastes the contents of the clipboard
      | into the editor at the caret position.
      | @see cut, copy, SystemClipboard
      |
      */
    pub fn paste(&mut self)  {
        
        todo!();
        /*
            if (! isReadOnly())
        {
            auto clip = SystemClipboard::getTextFromClipboard();

            if (clip.isNotEmpty())
                insertTextAtCaret (clip);
        }
        */
    }
    
    /**
      | Deletes the currently selected region.
      | 
      | This doesn't copy the deleted section
      | to the clipboard - if you need to do that,
      | call copy() first. @see copy, paste,
      | SystemClipboard
      |
      */
    pub fn cut(&mut self)  {
        
        todo!();
        /*
            if (! isReadOnly())
        {
            moveCaret (selection.getEnd());
            insertTextAtCaret (String());
        }
        */
    }
    
    pub fn draw_content(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (getWordWrapWidth() > 0)
        {
            g.setOrigin (leftIndent, topIndent);
            auto clip = g.getClipBounds();

            auto yOffset = TextEditorIterator (*this).getYOffset();

            AffineTransform transform;

            if (yOffset > 0)
            {
                transform = AffineTransform::translation (0.0f, yOffset);
                clip.setY (roundToInt ((float) clip.getY() - yOffset));
            }

            TextEditorIterator i (*this);
            Colour selectedTextColour;

            if (! selection.isEmpty())
            {
                selectedTextColour = findColour (highlightedTextColourId);

                g.setColour (findColour (highlightColourId).withMultipliedAlpha (hasKeyboardFocus (true) ? 1.0f : 0.5f));

                auto boundingBox = getTextBounds (selection);
                boundingBox.offsetAll (-getTextOffset());

                g.fillPath (boundingBox.toPath(), transform);
            }

            const TextEditorUniformTextSection* lastSection = nullptr;

            while (i.next() && i.lineY < (float) clip.getBottom())
            {
                if (i.lineY + i.lineHeight >= (float) clip.getY())
                {
                    if (selection.intersects ({ i.indexInText, i.indexInText + i.atom->numChars }))
                    {
                        i.drawSelectedText (g, selection, selectedTextColour, transform);
                        lastSection = nullptr;
                    }
                    else
                    {
                        i.draw (g, lastSection, transform);
                    }
                }
            }

            for (auto& underlinedSection : underlinedSections)
            {
                TextEditorIterator i2 (*this);

                while (i2.next() && i2.lineY < (float) clip.getBottom())
                {
                    if (i2.lineY + i2.lineHeight >= (float) clip.getY()
                          && underlinedSection.intersects ({ i2.indexInText, i2.indexInText + i2.atom->numChars }))
                    {
                        i2.drawUnderline (g, underlinedSection, findColour (textColourId), transform);
                    }
                }
            }
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().fillTextEditorBackground (g, getWidth(), getHeight(), *this);
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (textToShowWhenEmpty.isNotEmpty()
             && (! hasKeyboardFocus (false))
             && getTotalNumChars() == 0)
        {
            g.setColour (colourForTextWhenEmpty);
            g.setFont (getFont());

            Rectangle<int> textBounds (leftIndent,
                                       topIndent,
                                       viewport->getWidth() - leftIndent,
                                       getHeight() - topIndent);

            if (! textBounds.isEmpty())
                g.drawText (textToShowWhenEmpty, textBounds, justification, true);
        }

        getLookAndFeel().drawTextEditorOutline (g, getWidth(), getHeight(), *this);
        */
    }
    
    pub fn add_popup_menu_items(&mut self, 
        m:  &mut PopupMenu,
        _1: *const MouseEvent)  {
        
        todo!();
        /*
            const bool writable = ! isReadOnly();

        if (passwordCharacter == 0)
        {
            m.addItem (StandardApplicationCommandIDs::cut,   TRANS("Cut"), writable);
            m.addItem (StandardApplicationCommandIDs::copy,  TRANS("Copy"), ! selection.isEmpty());
        }

        m.addItem (StandardApplicationCommandIDs::paste,     TRANS("Paste"), writable);
        m.addItem (StandardApplicationCommandIDs::del,       TRANS("Delete"), writable);
        m.addSeparator();
        m.addItem (StandardApplicationCommandIDs::selectAll, TRANS("Select All"));
        m.addSeparator();

        if (getUndoManager() != nullptr)
        {
            m.addItem (StandardApplicationCommandIDs::undo, TRANS("Undo"), undoManager.canUndo());
            m.addItem (StandardApplicationCommandIDs::redo, TRANS("Redo"), undoManager.canRedo());
        }
        */
    }
    
    pub fn perform_popup_menu_action(&mut self, menu_itemid: i32)  {
        
        todo!();
        /*
            switch (menuItemID)
        {
            case StandardApplicationCommandIDs::cut:        cutToClipboard(); break;
            case StandardApplicationCommandIDs::copy:       copyToClipboard(); break;
            case StandardApplicationCommandIDs::paste:      pasteFromClipboard(); break;
            case StandardApplicationCommandIDs::del:        cut(); break;
            case StandardApplicationCommandIDs::selectAll:  selectAll(); break;
            case StandardApplicationCommandIDs::undo:       undo(); break;
            case StandardApplicationCommandIDs::redo:       redo(); break;
            default: break;
        }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            beginDragAutoRepeat (100);
        newTransaction();

        if (wasFocused || ! selectAllTextWhenFocused)
        {
            if (! (popupMenuEnabled && e.mods.isPopupMenu()))
            {
                moveCaretTo (getTextIndexAt (e.x, e.y),
                             e.mods.isShiftDown());
            }
            else
            {
                PopupMenu m;
                m.setLookAndFeel (&getLookAndFeel());
                addPopupMenuItems (m, &e);

                menuActive = true;

                m.showMenuAsync (typename PopupMenu::Options(),
                                 [safeThis = SafePointer<TextEditor> { this }] (int menuResult)
                                 {
                                     if (auto* editor = safeThis.getComponent())
                                     {
                                         editor->menuActive = false;

                                         if (menuResult != 0)
                                             editor->performPopupMenuAction (menuResult);
                                     }
                                 });
            }
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (wasFocused || ! selectAllTextWhenFocused)
            if (! (popupMenuEnabled && e.mods.isPopupMenu()))
                moveCaretTo (getTextIndexAt (e.x, e.y), true);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            newTransaction();
        textHolder->restartTimer();

        if (wasFocused || ! selectAllTextWhenFocused)
            if (e.mouseWasClicked() && ! (popupMenuEnabled && e.mods.isPopupMenu()))
                moveCaret (getTextIndexAt (e.x, e.y));

        wasFocused = true;
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            int tokenEnd = getTextIndexAt (e.x, e.y);
        int tokenStart = 0;

        if (e.getNumberOfClicks() > 3)
        {
            tokenEnd = getTotalNumChars();
        }
        else
        {
            auto t = getText();
            auto totalLength = getTotalNumChars();

            while (tokenEnd < totalLength)
            {
                auto c = t[tokenEnd];

                // (note the slight bodge here - it's because iswalnum only checks for alphabetic chars in the current locale)
                if (CharacterFunctions::isLetterOrDigit (c) || c > 128)
                    ++tokenEnd;
                else
                    break;
            }

            tokenStart = tokenEnd;

            while (tokenStart > 0)
            {
                auto c = t[tokenStart - 1];

                // (note the slight bodge here - it's because iswalnum only checks for alphabetic chars in the current locale)
                if (CharacterFunctions::isLetterOrDigit (c) || c > 128)
                    --tokenStart;
                else
                    break;
            }

            if (e.getNumberOfClicks() > 2)
            {
                while (tokenEnd < totalLength)
                {
                    auto c = t[tokenEnd];

                    if (c != '\r' && c != '\n')
                        ++tokenEnd;
                    else
                        break;
                }

                while (tokenStart > 0)
                {
                    auto c = t[tokenStart - 1];

                    if (c != '\r' && c != '\n')
                        --tokenStart;
                    else
                        break;
                }
            }
        }

        moveCaretTo (tokenEnd, false);
        moveCaretTo (tokenStart, true);
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if (! viewport->useMouseWheelMoveIfNeeded (e, wheel))
            Component::mouseWheelMove (e, wheel);
        */
    }
    
    pub fn move_caret_with_transaction(&mut self, 
        new_pos:   i32,
        selecting: bool) -> bool {
        
        todo!();
        /*
            newTransaction();
        moveCaretTo (newPos, selecting);
        return true;
        */
    }
    
    pub fn move_caret_left(&mut self, 
        move_in_whole_word_steps: bool,
        selecting:                bool) -> bool {
        
        todo!();
        /*
            auto pos = getCaretPosition();

        if (moveInWholeWordSteps)
            pos = findWordBreakBefore (pos);
        else
            --pos;

        return moveCaretWithTransaction (pos, selecting);
        */
    }
    
    pub fn move_caret_right(&mut self, 
        move_in_whole_word_steps: bool,
        selecting:                bool) -> bool {
        
        todo!();
        /*
            auto pos = getCaretPosition();

        if (moveInWholeWordSteps)
            pos = findWordBreakAfter (pos);
        else
            ++pos;

        return moveCaretWithTransaction (pos, selecting);
        */
    }
    
    pub fn move_caret_up(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            if (! isMultiLine())
            return moveCaretToStartOfLine (selecting);

        auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition (caretPos.getX(), caretPos.getY() - 1.0f), selecting);
        */
    }
    
    pub fn move_caret_down(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            if (! isMultiLine())
            return moveCaretToEndOfLine (selecting);

        auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition (caretPos.getX(), caretPos.getBottom() + 1.0f), selecting);
        */
    }
    
    pub fn page_up(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            if (! isMultiLine())
            return moveCaretToStartOfLine (selecting);

        auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition (caretPos.getX(), caretPos.getY() - (float) viewport->getViewHeight()), selecting);
        */
    }
    
    pub fn page_down(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            if (! isMultiLine())
            return moveCaretToEndOfLine (selecting);

        auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition (caretPos.getX(), caretPos.getBottom() + (float) viewport->getViewHeight()), selecting);
        */
    }
    
    pub fn scroll_by_lines(&mut self, delta_lines: i32)  {
        
        todo!();
        /*
            viewport->getVerticalScrollBar().moveScrollbarInSteps (deltaLines);
        */
    }
    
    pub fn scroll_down(&mut self) -> bool {
        
        todo!();
        /*
            scrollByLines (-1);
        return true;
        */
    }
    
    pub fn scroll_up(&mut self) -> bool {
        
        todo!();
        /*
            scrollByLines (1);
        return true;
        */
    }
    
    pub fn move_caret_to_top(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            return moveCaretWithTransaction (0, selecting);
        */
    }
    
    pub fn move_caret_to_start_of_line(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition (0.0f, caretPos.getY()), selecting);
        */
    }
    
    pub fn move_caret_to_end_with_selecting_gate(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            return moveCaretWithTransaction (getTotalNumChars(), selecting);
        */
    }
    
    pub fn move_caret_to_end_of_line(&mut self, selecting: bool) -> bool {
        
        todo!();
        /*
            auto caretPos = getCaretRectangleFloat();
        return moveCaretWithTransaction (indexAtPosition ((float) textHolder->getWidth(), caretPos.getY()), selecting);
        */
    }
    
    pub fn delete_backwards(&mut self, move_in_whole_word_steps: bool) -> bool {
        
        todo!();
        /*
            if (moveInWholeWordSteps)
            moveCaretTo (findWordBreakBefore (getCaretPosition()), true);
        else if (selection.isEmpty() && selection.getStart() > 0)
            setSelection ({ selection.getEnd() - 1, selection.getEnd() });

        cut();
        return true;
        */
    }
    
    pub fn delete_forwards(&mut self, move_in_whole_word_steps: bool) -> bool {
        
        todo!();
        /*
            if (selection.isEmpty() && selection.getStart() < getTotalNumChars())
            setSelection ({ selection.getStart(), selection.getStart() + 1 });

        cut();
        return true;
        */
    }
    
    pub fn copy_to_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        copy();
        return true;
        */
    }
    
    pub fn cut_to_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        copy();
        cut();
        return true;
        */
    }
    
    pub fn paste_from_clipboard(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        paste();
        return true;
        */
    }
    
    pub fn select_all(&mut self) -> bool {
        
        todo!();
        /*
            newTransaction();
        moveCaretTo (getTotalNumChars(), false);
        moveCaretTo (0, true);
        return true;
        */
    }
    
    /**
      | This can be used to change whether escape
      | and return keypress events are propagated
      | up to the parent component.
      | 
      | The default here is true, meaning that
      | these events are not allowed to reach
      | the parent, but you may want to allow
      | them through so that they can trigger
      | other actions, e.g. closing a dialog
      | box, etc.
      |
      */
    pub fn set_escape_and_return_keys_consumed(&mut self, should_be_consumed: bool)  {
        
        todo!();
        /*
            consumeEscAndReturnKeys = shouldBeConsumed;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (isReadOnly() && key != KeyPress ('c', ModifierKeys::commandModifier, 0)
                         && key != KeyPress ('a', ModifierKeys::commandModifier, 0))
            return false;

        if (! TextEditorKeyMapper<TextEditor>::invokeKeyFunction (*this, key))
        {
            if (key == KeyPress::returnKey)
            {
                newTransaction();

                if (returnKeyStartsNewLine)
                {
                    insertTextAtCaret ("\n");
                }
                else
                {
                    returnPressed();
                    return consumeEscAndReturnKeys;
                }
            }
            else if (key.isKeyCode (KeyPress::escapeKey))
            {
                newTransaction();
                moveCaretTo (getCaretPosition(), false);
                escapePressed();
                return consumeEscAndReturnKeys;
            }
            else if (key.getTextCharacter() >= ' '
                      || (tabKeyUsed && (key.getTextCharacter() == '\t')))
            {
                insertTextAtCaret (String::charToString (key.getTextCharacter()));

                lastTransactionTime = Time::getApproximateMillisecondCounter();
            }
            else
            {
                return false;
            }
        }

        return true;
        */
    }
    
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            if (! isKeyDown)
            return false;

       #if ALOE_WINDOWS
        if (KeyPress (KeyPress::F4Key, ModifierKeys::altModifier, 0).isCurrentlyDown())
            return false;  // We need to explicitly allow alt-F4 to pass through on Windows
       #endif

        if ((! consumeEscAndReturnKeys)
             && (KeyPress (KeyPress::escapeKey).isCurrentlyDown()
              || KeyPress (KeyPress::returnKey).isCurrentlyDown()))
            return false;

        // (overridden to avoid forwarding key events to the parent)
        return ! ModifierKeys::currentModifiers.isCommandDown();
        */
    }
    
    pub fn focus_gained(&mut self, cause: FocusChangeType)  {
        
        todo!();
        /*
            newTransaction();

        if (selectAllTextWhenFocused)
        {
            moveCaretTo (0, false);
            moveCaretTo (getTotalNumChars(), true);
        }

        checkFocus();

        if (cause == FocusChangeType::focusChangedByMouseClick && selectAllTextWhenFocused)
            wasFocused = false;

        repaint();
        updateCaretPosition();
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            newTransaction();

        wasFocused = false;
        textHolder->stopTimer();

        underlinedSections.clear();

        if (auto* peer = getPeer())
            peer->dismissPendingTextInput();

        updateCaretPosition();

        postCommandMessage (TextEditorDefs::focusLossMessageId);
        repaint();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            viewport->setBoundsInset (borderSize);
        viewport->setSingleStepSizes (16, roundToInt (currentFont.getHeight()));

        checkLayout();

        if (isMultiLine())
            updateCaretPosition();
        else
            scrollToMakeSureCursorIsVisible();
        */
    }
    
    pub fn handle_command_message(&mut self, command_id: i32)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);

        switch (commandId)
        {
        case TextEditorDefs::textChangeMessageId:
            listeners.callChecked (checker, [this] (TextEditorListener& l) { l.textEditorTextChanged (*this); });

            if (! checker.shouldBailOut() && onTextChange != nullptr)
                onTextChange();

            break;

        case TextEditorDefs::returnKeyMessageId:
            listeners.callChecked (checker, [this] (TextEditorListener& l) { l.textEditorReturnKeyPressed (*this); });

            if (! checker.shouldBailOut() && onReturnKey != nullptr)
                onReturnKey();

            break;

        case TextEditorDefs::escapeKeyMessageId:
            listeners.callChecked (checker, [this] (TextEditorListener& l) { l.textEditorEscapeKeyPressed (*this); });

            if (! checker.shouldBailOut() && onEscapeKey != nullptr)
                onEscapeKey();

            break;

        case TextEditorDefs::focusLossMessageId:
            updateValueFromText();
            listeners.callChecked (checker, [this] (TextEditorListener& l) { l.textEditorFocusLost (*this); });

            if (! checker.shouldBailOut() && onFocusLost != nullptr)
                onFocusLost();

            break;

        default:
            jassertfalse;
            break;
        }
        */
    }
    
    pub fn get_undo_manager(&mut self) -> *mut UndoManager {
        
        todo!();
        /*
            return readOnly ? nullptr : &undoManager;
        */
    }
    
    pub fn clear_internal(&mut self, um: *mut UndoManager)  {
        
        todo!();
        /*
            remove ({ 0, getTotalNumChars() }, um, caretPosition);
        */
    }
    
    pub fn insert(&mut self, 
        text:                      &String,
        insert_index:              i32,
        font:                      &Font,
        colour:                    Colour,
        um:                        *mut UndoManager,
        caret_position_to_move_to: i32)  {
        
        todo!();
        /*
            if (text.isNotEmpty())
        {
            if (um != nullptr)
            {
                if (um->getNumActionsInCurrentTransaction() > TextEditorDefs::maxActionsPerTransaction)
                    newTransaction();

                um->perform (new TextEditorInsertAction (*this, text, insertIndex, font, colour,
                                               caretPosition, caretPositionToMoveTo));
            }
            else
            {
                repaintText ({ insertIndex, getTotalNumChars() }); // must do this before and after changing the data, in case
                                                                   // a line gets moved due to word wrap

                int index = 0;
                int nextIndex = 0;

                for (int i = 0; i < sections.size(); ++i)
                {
                    nextIndex = index + sections.getUnchecked (i)->getTotalLength();

                    if (insertIndex == index)
                    {
                        sections.insert (i, new TextEditorUniformTextSection (text, font, colour, passwordCharacter));
                        break;
                    }

                    if (insertIndex > index && insertIndex < nextIndex)
                    {
                        splitSection (i, insertIndex - index);
                        sections.insert (i + 1, new TextEditorUniformTextSection (text, font, colour, passwordCharacter));
                        break;
                    }

                    index = nextIndex;
                }

                if (nextIndex == insertIndex)
                    sections.add (new TextEditorUniformTextSection (text, font, colour, passwordCharacter));

                coalesceSimilarSections();
                totalNumChars = -1;
                valueTextNeedsUpdating = true;

                checkLayout();
                moveCaretTo (caretPositionToMoveTo, false);

                repaintText ({ insertIndex, getTotalNumChars() });
            }
        }
        */
    }
    
    pub fn reinsert(&mut self, 
        insert_index:       i32,
        sections_to_insert: &Vec<Box<TextEditorUniformTextSection>>)  {
        
        todo!();
        /*
            int index = 0;
        int nextIndex = 0;

        for (int i = 0; i < sections.size(); ++i)
        {
            nextIndex = index + sections.getUnchecked (i)->getTotalLength();

            if (insertIndex == index)
            {
                for (int j = sectionsToInsert.size(); --j >= 0;)
                    sections.insert (i, new TextEditorUniformTextSection (*sectionsToInsert.getUnchecked(j)));

                break;
            }

            if (insertIndex > index && insertIndex < nextIndex)
            {
                splitSection (i, insertIndex - index);

                for (int j = sectionsToInsert.size(); --j >= 0;)
                    sections.insert (i + 1, new TextEditorUniformTextSection (*sectionsToInsert.getUnchecked(j)));

                break;
            }

            index = nextIndex;
        }

        if (nextIndex == insertIndex)
            for (auto* s : sectionsToInsert)
                sections.add (new TextEditorUniformTextSection (*s));

        coalesceSimilarSections();
        totalNumChars = -1;
        valueTextNeedsUpdating = true;
        */
    }
    
    pub fn remove(&mut self, 
        range:                     Range<i32>,
        um:                        *mut UndoManager,
        caret_position_to_move_to: i32)  {
        
        todo!();
        /*
            if (! range.isEmpty())
        {
            int index = 0;

            for (int i = 0; i < sections.size(); ++i)
            {
                auto nextIndex = index + sections.getUnchecked(i)->getTotalLength();

                if (range.getStart() > index && range.getStart() < nextIndex)
                {
                    splitSection (i, range.getStart() - index);
                    --i;
                }
                else if (range.getEnd() > index && range.getEnd() < nextIndex)
                {
                    splitSection (i, range.getEnd() - index);
                    --i;
                }
                else
                {
                    index = nextIndex;

                    if (index > range.getEnd())
                        break;
                }
            }

            index = 0;

            if (um != nullptr)
            {
                Vec<TextEditorUniformTextSection*> removedSections;

                for (auto* section : sections)
                {
                    if (range.getEnd() <= range.getStart())
                        break;

                    auto nextIndex = index + section->getTotalLength();

                    if (range.getStart() <= index && range.getEnd() >= nextIndex)
                        removedSections.add (new TextEditorUniformTextSection (*section));

                    index = nextIndex;
                }

                if (um->getNumActionsInCurrentTransaction() > TextEditorDefs::maxActionsPerTransaction)
                    newTransaction();

                um->perform (new TextEditorRemoveAction (*this, range, caretPosition,
                                               caretPositionToMoveTo, removedSections));
            }
            else
            {
                auto remainingRange = range;

                for (int i = 0; i < sections.size(); ++i)
                {
                    auto* section = sections.getUnchecked (i);
                    auto nextIndex = index + section->getTotalLength();

                    if (remainingRange.getStart() <= index && remainingRange.getEnd() >= nextIndex)
                    {
                        sections.remove (i);
                        remainingRange.setEnd (remainingRange.getEnd() - (nextIndex - index));

                        if (remainingRange.isEmpty())
                            break;

                        --i;
                    }
                    else
                    {
                        index = nextIndex;
                    }
                }

                coalesceSimilarSections();
                totalNumChars = -1;
                valueTextNeedsUpdating = true;

                checkLayout();
                moveCaretTo (caretPositionToMoveTo, false);

                repaintText ({ range.getStart(), getTotalNumChars() });
            }
        }
        */
    }
    
    /**
      | Returns the entire contents of the editor.
      |
      */
    pub fn get_text(&self) -> String {
        
        todo!();
        /*
            MemoryOutputStream mo;
        mo.preallocate ((size_t) getTotalNumChars());

        for (auto* s : sections)
            s->appendAllText (mo);

        return mo.toUTF8();
        */
    }
    
    /**
      | Returns the section of text that is currently
      | selected.
      |
      */
    pub fn get_highlighted_text(&self) -> String {
        
        todo!();
        /*
            return getTextInRange (selection);
        */
    }
    
    /**
      | Counts the number of characters in the
      | text.
      | 
      | This is quicker than getting the text
      | as a string if you just need to know the
      | length.
      |
      */
    pub fn get_total_num_chars(&self) -> i32 {
        
        todo!();
        /*
            if (totalNumChars < 0)
        {
            totalNumChars = 0;

            for (auto* s : sections)
                totalNumChars += s->getTotalLength();
        }

        return totalNumChars;
        */
    }
    
    /**
      | Returns true if there are no characters
      | in the editor.
      | 
      | This is far more efficient than calling
      | getText().isEmpty().
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return getTotalNumChars() == 0;
        */
    }
    
    pub fn get_char_position(&self, 
        index:       i32,
        anchor:      &mut Point<f32>,
        line_height: &mut f32)  {
        
        todo!();
        /*
            if (getWordWrapWidth() <= 0)
        {
            anchor = {};
            lineHeight = currentFont.getHeight();
        }
        else
        {
            TextEditorIterator i (*this);

            if (sections.isEmpty())
            {
                anchor = { i.getJustificationOffsetX (0), 0 };
                lineHeight = currentFont.getHeight();
            }
            else
            {
                i.getCharPosition (index, anchor, lineHeight);
            }
        }
        */
    }
    
    pub fn index_at_position(&self, x: f32, y: f32) -> i32 {
        
        todo!();
        /*
            if (getWordWrapWidth() > 0)
        {
            for (TextEditorIterator i (*this); i.next();)
            {
                if (y < i.lineY + i.lineHeight)
                {
                    if (y < i.lineY)
                        return jmax (0, i.indexInText - 1);

                    if (x <= i.atomX || i.atom->isNewLine())
                        return i.indexInText;

                    if (x < i.atomRight)
                        return i.xToIndex (x);
                }
            }
        }

        return getTotalNumChars();
        */
    }
    
    pub fn find_word_break_after(&self, position: i32) -> i32 {
        
        todo!();
        /*
            auto t = getTextInRange ({ position, position + 512 });
        auto totalLength = t.length();
        int i = 0;

        while (i < totalLength && CharacterFunctions::isWhitespace (t[i]))
            ++i;

        auto type = TextEditorDefs::getCharacterCategory (t[i]);

        while (i < totalLength && type == TextEditorDefs::getCharacterCategory (t[i]))
            ++i;

        while (i < totalLength && CharacterFunctions::isWhitespace (t[i]))
            ++i;

        return position + i;
        */
    }
    
    pub fn find_word_break_before(&self, position: i32) -> i32 {
        
        todo!();
        /*
            if (position <= 0)
            return 0;

        auto startOfBuffer = jmax (0, position - 512);
        auto t = getTextInRange ({ startOfBuffer, position });

        int i = position - startOfBuffer;

        while (i > 0 && CharacterFunctions::isWhitespace (t [i - 1]))
            --i;

        if (i > 0)
        {
            auto type = TextEditorDefs::getCharacterCategory (t [i - 1]);

            while (i > 0 && type == TextEditorDefs::getCharacterCategory (t [i - 1]))
                --i;
        }

        jassert (startOfBuffer + i >= 0);
        return startOfBuffer + i;
        */
    }
    
    pub fn split_section(&mut self, 
        section_index:    i32,
        char_to_split_at: i32)  {
        
        todo!();
        /*
            jassert (sections[sectionIndex] != nullptr);

        sections.insert (sectionIndex + 1,
                         sections.getUnchecked (sectionIndex)->split (charToSplitAt));
        */
    }
    
    pub fn coalesce_similar_sections(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < sections.size() - 1; ++i)
        {
            auto* s1 = sections.getUnchecked (i);
            auto* s2 = sections.getUnchecked (i + 1);

            if (s1->font == s2->font
                 && s1->colour == s2->colour)
            {
                s1->append (*s2);
                sections.remove (i + 1);
                --i;
            }
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<TextEditorAccessibilityHandler> (*this);
        */
    }
}
