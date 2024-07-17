crate::ix!();

pub fn draw_text_layout(
        g:           &mut Graphics,
        owner:       &mut Component,
        text:        &str,
        text_bounds: &Rectangle<i32>,
        enabled:     bool)  {
    
    todo!();
    /*
        const auto textColour = owner.findColour (ListBox::textColourId, true).withMultipliedAlpha (enabled ? 1.0f : 0.6f);

        AttributedString attributedString { text };
        attributedString.setColour (textColour);
        attributedString.setFont ((float) textBounds.getHeight() * 0.6f);
        attributedString.setJustification (Justification::centredLeft);
        attributedString.setWordWrap (AttributedString::WordWrap::none);

        TextLayout textLayout;
        textLayout.createLayout (attributedString,
                                 (float) textBounds.getWidth(),
                                 (float) textBounds.getHeight());
        textLayout.draw (g, textBounds.toFloat());
    */
}
