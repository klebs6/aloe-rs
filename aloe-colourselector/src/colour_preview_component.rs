crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ColourPreviewComp<'a> {
    base:           Component<'a>,
    owner:          &'a mut ColourSelector<'a>,
    current_colour: Colour,
    label_font:     Font, //{ 14.0f, Font::bold };
    label_width:    i32, // default = 0
    colour_label:   Label<'a>,
}

impl<'a> ColourPreviewComp<'a> {

    pub fn new(
        cs:          &mut ColourSelector<'a>,
        is_editable: bool) -> Self {
    
        todo!();
        /*
        : owner(cs),

            colourLabel.setFont (labelFont);
            colourLabel.setJustificationType (Justification::centred);

            if (isEditable)
            {
                colourLabel.setEditable (true);

                colourLabel.onEditorShow = [this]
                {
                    if (auto* ed = colourLabel.getCurrentTextEditor())
                        ed->setInputRestrictions ((owner.flags & showAlphaChannel) ? 8 : 6, "1234567890ABCDEFabcdef");
                };

                colourLabel.onEditorHide = [this]
                {
                    updateColourIfNecessary (colourLabel.getText());
                };
            }

            addAndMakeVisible (colourLabel);
        */
    }
    
    pub fn update_if_needed(&mut self)  {
        
        todo!();
        /*
            auto newColour = owner.getCurrentColour();

            if (currentColour != newColour)
            {
                currentColour = newColour;
                auto textColour = (Colours::white.overlaidWith (currentColour).contrasting());

                colourLabel.setColour (Label::textColourId,            textColour);
                colourLabel.setColour (Label::textWhenEditingColourId, textColour);
                colourLabel.setText (currentColour.toDisplayString ((owner.flags & showAlphaChannel) != 0), dontSendNotification);

                labelWidth = labelFont.getStringWidth (colourLabel.getText());

                repaint();
            }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillCheckerBoard (getLocalBounds().toFloat(), 10.0f, 10.0f,
                                Colour (0xffdddddd).overlaidWith (currentColour),
                                Colour (0xffffffff).overlaidWith (currentColour));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            colourLabel.centreWithSize (labelWidth + 10, (int) labelFont.getHeight() + 10);
        */
    }
    
    pub fn update_colour_if_necessary(&mut self, new_colour_string: &String)  {
        
        todo!();
        /*
            auto newColour = Colour::fromString (newColourString);

            if (newColour != currentColour)
                owner.setCurrentColour (newColour);
        */
    }
}
