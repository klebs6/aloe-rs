crate::ix!();

/**
  | This window contains a ColourSelector
  | which can be used to change the window's
  | colour.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ColourSelectorWindow<'a> {
    base:     DocumentWindow<'a>,
    selector: ColourSelector<'a>, //{ ColourSelector::showColourAtTop | ColourSelector::showSliders | ColourSelector::showColourspace };
}

impl<'a> ChangeListener for ColourSelectorWindow<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == &selector)
                setBackgroundColour (selector.getCurrentColour());
        */
    }
}

impl<'a> Drop for ColourSelectorWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            selector.removeChangeListener (this);
         */
    }
}

impl<'a> ColourSelectorWindow<'a> {

    pub fn new(
        name:              &String,
        background_colour: Colour,
        buttons_needed:    i32) -> Self {
    
        todo!();
        /*
        : document_window(name, backgroundColour, buttonsNeeded),

            selector.setCurrentColour (backgroundColour);
            selector.setColour (ColourSelector::backgroundColourId, Colours::transparentWhite);
            selector.addChangeListener (this);
            setContentOwned (&selector, false);
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            delete this;
        */
    }
}
