crate::ix!();

pub struct SidePanelHeader<'a> {
    base:            Component<'a>,
    owner:           &'a mut MainComponent<'a>,
    title_label:     Label<'a>,
    home_button:     ShapeButton<'a>, //{ "Home", Colours::transparentBlack, Colours::transparentBlack, Colours::transparentBlack };
    settings_button: ShapeButton<'a>, //{ "Settings", Colours::transparentBlack, Colours::transparentBlack, Colours::transparentBlack };
}

impl<'a> SidePanelHeader<'a> {

    pub fn new(o: &mut MainComponent) -> Self {
    
        todo!();
        /*
        : owner(o),

            setOpaque (true);

            static const unsigned char homeIconPathData[]
                = { 110,109,0,0,64,65,0,0,64,64,98,0,0,64,65,0,0,64,64,74,12,186,64,164,112,5,65,24,217,22,64,70,182,
                    51,65,98,35,219,9,64,240,167,54,65,0,0,0,64,170,241,58,65,0,0,0,64,0,0,64,65,98,0,0,0,64,22,217,
                    72,65,166,155,28,64,0,0,80,65,0,0,64,64,0,0,80,65,108,0,0,160,64,0,0,80,65,108,0,0,160,64,0,0,
                    160,65,98,0,0,160,64,139,108,164,65,211,77,174,64,0,0,168,65,0,0,192,64,0,0,168,65,108,0,0,16,65,
                    0,0,168,65,98,22,217,24,65,0,0,168,65,0,0,32,65,127,106,164,65,0,0,32,65,0,0,160,65,108,0,0,32,65,0,0,128,
                    65,108,0,0,96,65,0,0,128,65,108,0,0,96,65,0,0,160,65,98,0,0,96,65,127,106,164,65,233,38,103,65,0,0,168,
                    65,0,0,112,65,0,0,168,65,108,0,0,144,65,0,0,168,65,98,139,108,148,65,0,0,168,65,0,0,152,65,139,108,164,
                    65,0,0,152,65,0,0,160,65,108,0,0,152,65,0,0,80,65,108,0,0,168,65,0,0,80,65,98,139,108,172,65,0,0,80,65,0,
                    0,176,65,23,217,72,65,0,0,176,65,0,0,64,65,98,0,0,176,65,170,241,58,65,156,196,174,65,240,167,54,65,158,239,
                    172,65,70,182,51,65,98,213,120,145,65,164,112,5,65,0,0,64,65,0,0,64,64,0,0,64,65,0,0,64,64,99,
                    101,0,0 };

            static const unsigned char settingsIconPathData[]
                = { 110,109,202,111,210,64,243,226,61,64,108,0,0,224,64,0,0,0,0,108,0,0,48,65,0,0,0,0,108,27,200,54,65,243,
                    226,61,64,98,91,248,63,65,174,170,76,64,95,130,72,65,231,138,96,64,46,46,80,65,180,163,120,64,108,42,
                    181,124,65,20,38,49,64,108,149,90,142,65,246,108,199,64,108,68,249,118,65,2,85,1,65,98,112,166,119,65,
                    201,31,6,65,0,0,120,65,111,5,11,65,0,0,120,65,0,0,16,65,98,0,0,120,65,145,250,20,65,108,166,119,65,55,
                    224,25,65,72,249,118,65,254,170,30,65,108,151,90,142,65,133,73,60,65,108,46,181,124,65,123,182,115,65,
                    108,50,46,80,65,18,215,97,65,98,99,130,72,65,70,221,103,65,96,248,63,65,83,213,108,65,32,200,54,65,66,
                    135,112,65,108,0,0,48,65,0,0,144,65,108,0,0,224,64,0,0,144,65,108,202,111,210,64,67,135,112,65,98,74,
                    15,192,64,84,213,108,65,65,251,174,64,70,221,103,65,164,163,159,64,19,215,97,65,108,92,43,13,64,123,182,
                    115,65,108,187,181,82,62,133,73,60,65,108,244,26,36,64,254,170,30,65,98,64,102,33,64,55,224,25,5,0,0,32,
                    64,145,250,20,65,0,0,32,64,0,0,16,65,98,0,0,32,64,111,5,11,65,64,102,33,64,201,31,6,65,244,26,36,64,2,85,
                    1,65,108,187,181,82,62,246,108,199,64,108,92,43,13,64,20,38,49,64,108,164,163,159,64,180,163,120,64,98,65,
                    251,174,64,231,138,96,64,74,15,192,64,175,170,76,64,202,111,210,64,243,226,61,64,99,109,0,0,16,65,0,0,64,
                    65,98,121,130,42,65,0,0,64,65,0,0,64,65,121,130,42,65,0,0,64,65,0,0,16,65,98,0,0,64,65,13,251,234,64,121,
                    130,42,65,0,0,192,64,0,0,16,65,0,0,192,64,98,13,251,234,64,0,0,192,64,0,0,192,64,13,251,234,64,0,0,192,64,
                    0,0,16,65,98,0,0,192,64,121,130,42,65,13,251,234,64,0,0,64,65,0,0,16,65,0,0,64,65,99,101,0,0 };

            Path p;
            p.loadPathFromData (homeIconPathData, sizeof (homeIconPathData));
            homeButton.setShape (p, true, true, false);

            p.clear();
            p.loadPathFromData (settingsIconPathData, sizeof (settingsIconPathData));
            settingsButton.setShape (p, true, true, false);

            titleLabel.setText (owner.getSidePanel().getTitleText(), NotificationType::dontSendNotification);
            addAndMakeVisible (titleLabel);

            homeButton.onClick = [this] { owner.homeButtonClicked(); };
            addAndMakeVisible (homeButton);

            addAndMakeVisible (settingsButton);
            settingsButton.onClick = [this] { owner.settingsButtonClicked(); };

            lookAndFeelChanged();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (owner.getSidePanel().findColour (SidePanel::backgroundColour));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
            auto buttonWidth = owner.getSidePanel().getTitleBarHeight();

            bounds.removeFromLeft (10);
            homeButton.setBounds (bounds.removeFromLeft (buttonWidth).reduced (7));
            settingsButton.setBounds (bounds.removeFromLeft (buttonWidth).reduced (7));
            bounds.removeFromLeft (10);

            bounds.removeFromRight (10);
            titleLabel.setBounds (bounds);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto& sidePanel = owner.getSidePanel();
            auto& lf = sidePanel.getLookAndFeel();

            titleLabel.setFont (lf.getSidePanelTitleFont (sidePanel));
            titleLabel.setJustificationType (lf.getSidePanelTitleJustification (sidePanel));
            titleLabel.setColour (Label::textColourId, owner.findColour (SidePanel::titleTextColour));

            auto normal = sidePanel.findColour (SidePanel::dismissButtonNormalColour);
            auto over = sidePanel.findColour (SidePanel::dismissButtonOverColour);
            auto down = sidePanel.findColour (SidePanel::dismissButtonDownColour);

            homeButton.setColours (normal, over, down);
            settingsButton.setColours (normal, over, down);
        */
    }
}
