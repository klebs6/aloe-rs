crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Demos/IntroScreen.h]

#[no_copy]
#[leak_detector]
pub struct IntroScreen<'a> {
    base:          Component<'a>,
    version_label: Label<'a>,
    link_button:   HyperlinkButton<'a>, // { "www.aloe.com", { "http://www.aloe.com" } };
    logo:          LogoDrawComponent<'a>,
}

impl<'a> Default for IntroScreen<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (versionLabel);
            addAndMakeVisible (linkButton);
            addAndMakeVisible (logo);

            versionLabel.setText (String ("{version}  built on {date}")
                                      .replace ("{version}", SystemStats::getALOEVersion())
                                      .replace ("{date}",    String (__DATE__).replace ("  ", " ")),
                                  dontSendNotification);

            linkButton.setColour (HyperlinkButton::textColourId, Colours::lightblue);

            setTitle ("Home");
            setFocusContainerType (FocusContainerType::focusContainer)
        */
    }
}

impl<'a> IntroScreen<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (10);

            auto bottomSlice = area.removeFromBottom (24);
            linkButton.setBounds (bottomSlice.removeFromRight (getWidth() / 4));
            versionLabel.setBounds (bottomSlice);

            logo.setBounds (area);
        */
    }
}
