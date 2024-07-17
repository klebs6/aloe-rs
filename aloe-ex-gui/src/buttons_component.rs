crate::ix!();

pub struct ButtonsComponent<'a> {
    base: Component<'a>,
}

impl<'a> Default for ButtonsComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (radioButtons);
                addAndMakeVisible (textButton);

                shapeButton.setShape (getALOELogoPath(), false, true, false);
                shapeButton.onClick = [] { AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon, "Alert", "This is an AlertWindow"); };
                addAndMakeVisible (shapeButton)
        */
    }
}

impl<'a> ButtonsComponent<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                radioButtons.setBounds (bounds.removeFromLeft (bounds.getWidth() / 2).reduced (5));
                textButton.setBounds (bounds.removeFromTop (bounds.getHeight() / 2).reduced (5));
                shapeButton.setBounds (bounds.reduced (5));
        */
    }
}
