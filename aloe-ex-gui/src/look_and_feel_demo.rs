crate::ix!();

#[no_copy]
#[leak_detector]
pub struct LookAndFeelDemo<'a> {
    base:              Component<'a>,
    description_label: Label<'a>,
    laf_box:           ComboBox<'a>,
    random_button:     TextButton<'a>, // default = { "Assign Randomly"  }
    look_and_feels:    Vec<Box<dyn LookAndFeel>>,
    demo_comp:         LookAndFeelDemoComponent<'a>,
}

impl<'a> Default for LookAndFeelDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            descriptionLabel.setMinimumHorizontalScale (1.0f);
            descriptionLabel.setText ("This demonstrates how to create a custom look and feel by overriding only the desired methods.\n\n"
                                      "Components can have their look and feel individually assigned or they will inherit it from their parent. "
                                      "Colours work in a similar way, they can be set for individual components or a look and feel as a whole.",
                                      dontSendNotification);

            addAndMakeVisible (descriptionLabel);
            addAndMakeVisible (lafBox);
            addAndMakeVisible (demoComp);

            addLookAndFeel (new LookAndFeel_V1(), "LookAndFeel_V1");
            addLookAndFeel (new LookAndFeel_V2(), "LookAndFeel_V2");
            addLookAndFeel (new LookAndFeel_V3(), "LookAndFeel_V3");
            addLookAndFeel (new LookAndFeel_V4(), "LookAndFeel_V4 (Dark)");
            addLookAndFeel (new LookAndFeel_V4 (LookAndFeel_V4::getMidnightColourScheme()), "LookAndFeel_V4 (Midnight)");
            addLookAndFeel (new LookAndFeel_V4 (LookAndFeel_V4::getGreyColourScheme()),     "LookAndFeel_V4 (Grey)");
            addLookAndFeel (new LookAndFeel_V4 (LookAndFeel_V4::getLightColourScheme()),    "LookAndFeel_V4 (Light)");

            auto* claf = new CustomLookAndFeel();
            addLookAndFeel (claf, "Custom Look And Feel");
            setupCustomLookAndFeelColours (*claf);

            auto* slaf = new SquareLookAndFeel();
            addLookAndFeel (slaf, "Square Look And Feel");
            setupSquareLookAndFeelColours (*slaf);

            lafBox.onChange = [this] { setAllLookAndFeels (lookAndFeels[lafBox.getSelectedItemIndex()]); };
            lafBox.setSelectedItemIndex (3);

            addAndMakeVisible (randomButton);
            randomButton.onClick = [this] { lafBox.setSelectedItemIndex (Random().nextInt (lafBox.getNumItems())); };

            setSize (500, 500)
        */
    }
}

impl<'a> Paint for LookAndFeelDemo<'a> {
    
    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.4f)));
        */
    }
}
    
impl<'a> Resized for LookAndFeelDemo<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (10);

            descriptionLabel.setBounds (r.removeFromTop (150));
            lafBox          .setBounds (r.removeFromTop (22).removeFromLeft (250));
            randomButton    .setBounds (lafBox.getBounds().withX (lafBox.getRight() + 20).withWidth (140));
            demoComp        .setBounds (r.withTrimmedTop (10));
        */
    }
}
    
impl<'a> LookAndFeelDemo<'a> {

    pub fn add_look_and_feel(
        &mut self, 
        laf:  &mut dyn LookAndFeel,
        name: &str
    ) {
        
        todo!();
        /*
            lookAndFeels.add (laf);
            lafBox.addItem (name, lafBox.getNumItems() + 1);
        */
    }
    
    pub fn setup_custom_look_and_feel_colours(&mut self, laf: &mut dyn LookAndFeel)  {
        
        todo!();
        /*
            laf.setColour (Slider::thumbColourId,               Colour::greyLevel (0.95f));
            laf.setColour (Slider::textBoxOutlineColourId,      Colours::transparentWhite);
            laf.setColour (Slider::rotarySliderFillColourId,    Colour (0xff00b5f6));
            laf.setColour (Slider::rotarySliderOutlineColourId, Colours::white);

            laf.setColour (TextButton::buttonColourId,  Colours::white);
            laf.setColour (TextButton::textColourOffId, Colour (0xff00b5f6));

            laf.setColour (TextButton::buttonOnColourId, laf.findColour (TextButton::textColourOffId));
            laf.setColour (TextButton::textColourOnId,   laf.findColour (TextButton::buttonColourId));
        */
    }
    
    pub fn setup_square_look_and_feel_colours(&mut self, laf: &mut dyn LookAndFeel)  {
        
        todo!();
        /*
            auto baseColour = Colours::red;

            laf.setColour (Slider::thumbColourId,               Colour::greyLevel (0.95f));
            laf.setColour (Slider::textBoxOutlineColourId,      Colours::transparentWhite);
            laf.setColour (Slider::rotarySliderFillColourId,    baseColour);
            laf.setColour (Slider::rotarySliderOutlineColourId, Colours::white);
            laf.setColour (Slider::trackColourId,               Colours::black);

            laf.setColour (TextButton::buttonColourId,  Colours::white);
            laf.setColour (TextButton::textColourOffId, baseColour);

            laf.setColour (TextButton::buttonOnColourId, laf.findColour (TextButton::textColourOffId));
            laf.setColour (TextButton::textColourOnId,   laf.findColour (TextButton::buttonColourId));
        */
    }
    
    pub fn set_all_look_and_feels(&mut self, laf: *mut dyn LookAndFeel)  {
        
        todo!();
        /*
            for (auto* child : demoComp.getChildren())
                child->setLookAndFeel (laf);
        */
    }
}
