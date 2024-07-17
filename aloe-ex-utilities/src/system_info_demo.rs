crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SystemInfoDemo<'a> {
    base:        Component<'a>,
    results_box: TextEditor<'a>,
}

impl<'a> Default for SystemInfoDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (resultsBox);
            resultsBox.setReadOnly (true);
            resultsBox.setMultiLine (true);
            resultsBox.setColour (TextEditor::backgroundColourId, Colours::transparentBlack);
            resultsBox.setFont ({ Font::getDefaultMonospacedFontName(), 12.0f, Font::plain });
            resultsBox.setText (getAllSystemInfo());

            setSize (500, 500)
        */
    }
}

impl<'a> SystemInfoDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.93f)));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            resultsBox.setBounds (getLocalBounds().reduced (8));
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            resultsBox.applyFontToAllText (resultsBox.getFont());
        */
    }
}
