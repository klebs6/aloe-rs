crate::ix!();

#[no_copy]
#[leak_detector]
pub struct LiveConstantDemo<'a> {
    base:              Component<'a>,
    description_label: Label<'a>,
    start_button:      TextButton<'a>, // default = { "Begin Demo"  }
    demo_comp:         LiveConstantDemoComponent<'a>,
}

impl<'a> Default for LiveConstantDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            descriptionLabel.setMinimumHorizontalScale (1.0f);
            descriptionLabel.setText ("This demonstrates the ALOE_LIVE_CONSTANT macro, which allows you to quickly "
                                      "adjust primitive values at runtime by just wrapping them in a macro.\n\n"
                                      "To understand what's going on in this demo, you should have a look at the "
                                      "LiveConstantDemoComponent class, where you can see the code that's invoking the demo below...",
                                      dontSendNotification);

            addAndMakeVisible (descriptionLabel);
            addAndMakeVisible (startButton);
            addChildComponent (demoComp);
            startButton.onClick = [this] { start(); };

            setSize (500, 500)
        */
    }
}

impl<'a> LiveConstantDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (10);

            descriptionLabel.setBounds (r.removeFromTop (200));
            startButton     .setBounds (r.removeFromTop (22).removeFromLeft (250));
            demoComp        .setBounds (r.withTrimmedTop (10));
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            startButton.setVisible (false);
            demoComp   .setVisible (true);

            descriptionLabel.setText ("Tweak some of the colours and values in the pop-up window to see what "
                                      "the effect of your changes would be on the component below...",
                                      dontSendNotification);
        */
    }
}
