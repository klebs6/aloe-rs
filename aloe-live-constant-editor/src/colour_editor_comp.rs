crate::ix!();

pub fn create_colour_editor<'a>(editor: &mut LivePropertyEditorBase) -> *mut Component<'a> {
    
    todo!();
        /*
            return new ColourEditorComp (editor);
        */
}

pub struct ColourEditorComp<'a> {
    base:   Component<'a>,
    editor: &'a mut LivePropertyEditorBase<'a>,
}

impl<'a> ChangeListener for ColourEditorComp<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (auto* cs = dynamic_cast<ColourSelector*> (source))
                editor.applyNewValue (getAsString (cs->getCurrentColour(), true));

            repaint();
        */
    }
}

impl<'a> ColourEditorComp<'a> {

    pub fn new(e: &mut LivePropertyEditorBase) -> Self {
    
        todo!();
        /*
        : editor(e),

            setMouseCursor (MouseCursor::PointingHandCursor);
        */
    }
    
    pub fn get_colour(&self) -> Colour {
        
        todo!();
        /*
            return Colour ((uint32) parseInt (editor.value.getStringValue (false)));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillCheckerBoard (getLocalBounds().toFloat(), 6.0f, 6.0f,
                                Colour (0xffdddddd).overlaidWith (getColour()),
                                Colour (0xffffffff).overlaidWith (getColour()));
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            auto colourSelector = std::make_unique<ColourSelector>();
            colourSelector->setName ("Colour");
            colourSelector->setCurrentColour (getColour());
            colourSelector->addChangeListener (this);
            colourSelector->setColour (ColourSelector::backgroundColourId, Colours::transparentBlack);
            colourSelector->setSize (300, 400);

            CallOutBox::launchAsynchronously (std::move (colourSelector), getScreenBounds(), nullptr);
        */
    }
}
