crate::ix!();

/**
  | A TextButton that pops up a colour chooser
  | to change its colours.
  |
  */
pub struct ColourChangeButton<'a> {
    base: TextButton<'a>,
}

impl<'a> ChangeListener for ColourChangeButton<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (auto* cs = dynamic_cast<ColourSelector*> (source))
                setColour (TextButton::buttonColourId, cs->getCurrentColour());
        */
    }
}

impl<'a> Default for ColourChangeButton<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : TextButton ("Click to change colour...")

            setSize (10, 24);
            changeWidthToFitText()
        */
    }
}

impl<'a> ColourChangeButton<'a> {

    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            auto colourSelector = std::make_unique<ColourSelector> (ColourSelector::showAlphaChannel
                                                                    | ColourSelector::showColourAtTop
                                                                    | ColourSelector::editableColour
                                                                    | ColourSelector::showSliders
                                                                    | ColourSelector::showColourspace);

            colourSelector->setName ("background");
            colourSelector->setCurrentColour (findColour (TextButton::buttonColourId));
            colourSelector->addChangeListener (this);
            colourSelector->setColour (ColourSelector::backgroundColourId, Colours::transparentBlack);
            colourSelector->setSize (300, 400);

            CallOutBox::launchAsynchronously (std::move (colourSelector), getScreenBounds(), nullptr);
        */
    }
}
