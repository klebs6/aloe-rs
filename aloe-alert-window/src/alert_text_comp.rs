crate::ix!();

#[no_copy]
pub struct AlertTextComp<'a> {
    base:       TextEditor<'a>,
    best_width: i32,
}

impl<'a> AlertTextComp<'a> {

    pub fn new(
        owner:   &mut AlertWindow,
        message: &String,
        font:    &Font) -> Self {
    
        todo!();
        /*


            if (owner.isColourSpecified (AlertWindow::textColourId))
                setColour (TextEditor::textColourId, owner.findColour (AlertWindow::textColourId));

            setColour (TextEditor::backgroundColourId, Colours::transparentBlack);
            setColour (TextEditor::outlineColourId, Colours::transparentBlack);
            setColour (TextEditor::shadowColourId, Colours::transparentBlack);

            setReadOnly (true);
            setMultiLine (true, true);
            setCaretVisible (false);
            setScrollbarsShown (true);
            lookAndFeelChanged();
            setWantsKeyboardFocus (false);
            setFont (font);
            setText (message, false);

            bestWidth = 2 * (int) std::sqrt (font.getHeight() * (float) font.getStringWidth (message));
        */
    }
    
    pub fn update_layout(&mut self, width: i32)  {
        
        todo!();
        /*
            AttributedString s;
            s.setJustification (Justification::topLeft);
            s.append (getText(), getFont());

            TextLayout text;
            text.createLayoutWithBalancedLineLengths (s, (float) width - 8.0f);
            setSize (width, jmin (width, (int) (text.getHeight() + getFont().getHeight())));
        */
    }
}
