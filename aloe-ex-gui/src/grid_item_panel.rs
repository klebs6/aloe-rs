crate::ix!();

pub struct GridItemPanel<'a> {
    base:   Component<'a>,
    colour: Colour,
    text:   String,
}

impl<'a> GridItemPanel<'a> {

    pub fn new(
        colour_to_use: Colour,
        text_to_use:   &String) -> Self {
    
        todo!();
        /*
        : colour(colourToUse),
        : text(textToUse),

        
        */
    }
}

impl<'a> Paint for GridItemPanel<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (colour.withAlpha (0.5f));

                g.setColour (Colours::black);
                g.drawText (text, getLocalBounds().withSizeKeepingCentre (100, 100), Justification::centred, false);
        */
    }
}
