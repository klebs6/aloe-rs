crate::ix!();

#[no_copy]
pub struct LivePropertyEditorBase<'a> {
    base:          Component<'a>,
    value:         &'a mut LiveValueBase,
    name:          Label<'a>,
    value_editor:  TextEditor<'a>,
    reset_button:  TextButton<'a>, // default = { "reset"  }
    document:      &'a mut CodeDocument<'a>,
    tokeniser:     CPlusPlusCodeTokeniser,
    source_editor: CodeEditorComponent<'a>,
    value_start:   CodeDocumentPosition<'a>,
    value_end:     CodeDocumentPosition<'a>,
    custom_comp:   Box<Component<'a>>,
    was_hex:       bool, // default = false
}

impl<'a> LivePropertyEditorBase<'a> {

    pub fn new(
        v: &mut LiveValueBase,
        d: &mut CodeDocument) -> Self {
    
        todo!();
        /*


            : value (v), document (d), sourceEditor (document, &tokeniser)
        setSize (600, 100);

        addAndMakeVisible (name);
        addAndMakeVisible (resetButton);
        addAndMakeVisible (valueEditor);
        addAndMakeVisible (sourceEditor);

        findOriginalValueInCode();
        selectOriginalValue();

        name.setFont (13.0f);
        name.setText (v.name, dontSendNotification);
        valueEditor.setMultiLine (v.isString());
        valueEditor.setReturnKeyStartsNewLine (v.isString());
        valueEditor.setText (v.getStringValue (wasHex), dontSendNotification);
        valueEditor.onTextChange = [this] { applyNewValue (valueEditor.getText()); };
        sourceEditor.setReadOnly (true);
        sourceEditor.setFont (sourceEditor.getFont().withHeight (13.0f));
        resetButton.onClick = [this] { applyNewValue (value.getOriginalStringValue (wasHex)); };
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::white);
        g.fillRect (getLocalBounds().removeFromBottom (1));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (0, 3).withTrimmedBottom (1);

        auto left = r.removeFromLeft (jmax (200, r.getWidth() / 3));

        auto top = left.removeFromTop (25);
        resetButton.setBounds (top.removeFromRight (35).reduced (0, 3));
        name.setBounds (top);

        if (customComp != nullptr)
        {
            valueEditor.setBounds (left.removeFromTop (25));
            left.removeFromTop (2);
            customComp->setBounds (left);
        }
        else
        {
            valueEditor.setBounds (left);
        }

        r.removeFromLeft (4);
        sourceEditor.setBounds (r);
        */
    }
    
    pub fn apply_new_value(&mut self, s: &String)  {
        
        todo!();
        /*
            value.setStringValue (s);

        document.replaceSection (valueStart.getPosition(), valueEnd.getPosition(), value.getCodeValue (wasHex));
        document.clearUndoHistory();
        selectOriginalValue();

        valueEditor.setText (s, dontSendNotification);
        AllComponentRepainter::getInstance()->trigger();
        */
    }
    
    pub fn select_original_value(&mut self)  {
        
        todo!();
        /*
            sourceEditor.selectRegion (valueStart, valueEnd);
        */
    }
    
    pub fn find_original_value_in_code(&mut self)  {
        
        todo!();
        /*
            CodeDocument::Position pos (document, value.sourceLine, 0);
        auto line = pos.getLineText();
        auto p = line.getCharPointer();

        p = CharacterFunctions::find (p, CharPointer_ASCII ("ALOE_LIVE_CONSTANT"));

        if (p.isEmpty())
        {
            // Not sure how this would happen - some kind of mix-up between source code and line numbers..
            jassertfalse;
            return;
        }

        p += (int) (sizeof ("ALOE_LIVE_CONSTANT") - 1);
        p.incrementToEndOfWhitespace();

        if (! CharacterFunctions::find (p, CharPointer_ASCII ("ALOE_LIVE_CONSTANT")).isEmpty())
        {
            // Aargh! You've added two ALOE_LIVE_CONSTANT macros on the same line!
            // They're identified by their line number, so you must make sure each
            // one goes on a separate line!
            jassertfalse;
        }

        if (p.getAndAdvance() == '(')
        {
            auto start = p, end = p;

            int depth = 1;

            while (! end.isEmpty())
            {
                auto c = end.getAndAdvance();

                if (c == '(')  ++depth;
                if (c == ')')  --depth;

                if (depth == 0)
                {
                    --end;
                    break;
                }
            }

            if (end > start)
            {
                valueStart = CodeDocument::Position (document, value.sourceLine, (int) (start - line.getCharPointer()));
                valueEnd   = CodeDocument::Position (document, value.sourceLine, (int) (end   - line.getCharPointer()));

                valueStart.setPositionMaintained (true);
                valueEnd.setPositionMaintained (true);

                wasHex = String (start, end).containsIgnoreCase ("0x");
            }
        }
        */
    }
}
