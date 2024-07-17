crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/TimersAndEventsDemo.h]

pub const TIMERS_AND_EVENTS_DEMO_NUM_FLASHING_COMPONENTS: usize = 9;

#[no_copy]
#[leak_detector]
pub struct TimersAndEventsDemo<'a> {
    base:                 Component<'a>,
    flashing_components:  Vec<Box<FlashingComponent<'a>>>,
    random_colour_button: TextButton<'a>, // default = { "Set Random Colour"  }
    stop_button:          TextButton<'a>, // default = { "Stop"  }
    random:               Random,
}

impl<'a> ChangeListener for TimersAndEventsDemo<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            for (int i = 0; i < flashingComponents.size(); ++i)
                if (source == flashingComponents.getUnchecked (i))
                    flashingComponents.getUnchecked ((i + 1) % flashingComponents.size())->startFlashing();
        */
    }
    
}

impl<'a> Default for TimersAndEventsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            // Create and add our FlashingComponents with some random colours and sizes
            for (int i = 0; i < numFlashingComponents; ++i)
            {
                auto* newFlasher = new FlashingComponent();
                flashingComponents.add (newFlasher);

                newFlasher->setFlashColour (getRandomBrightColour());
                newFlasher->addChangeListener (this);

                auto diameter = 25 + random.nextInt (75);
                newFlasher->setSize (diameter, diameter);

                addAndMakeVisible (newFlasher);
            }

            addAndMakeVisible (stopButton);
            stopButton.onClick = [this] { stopButtonClicked(); };

            addAndMakeVisible (randomColourButton);
            randomColourButton.onClick = [this] { randomColourButtonClicked(); };

            // lay out our components in a pseudo random grid
            Rectangle<int> area (0, 100, 150, 150);

            for (auto* comp : flashingComponents)
            {
                auto buttonArea = area.withSize (comp->getWidth(), comp->getHeight());
                buttonArea.translate (random.nextInt (area.getWidth()  - comp->getWidth()),
                                      random.nextInt (area.getHeight() - comp->getHeight()));
                comp->setBounds (buttonArea);

                area.translate (area.getWidth(), 0);

                // if we go off the right start a new row
                if (area.getRight() > (800 - area.getWidth()))
                {
                    area.translate (0, area.getWidth());
                    area.setX (0);
                }
            }

            setSize (600, 600)
        */
    }
}

impl<'a> Drop for TimersAndEventsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            for (auto* fc : flashingComponents)
                fc->removeChangeListener (this);
         */
    }
}

impl<'a> TimersAndEventsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colours::darkgrey));
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto explanationArea = getLocalBounds().removeFromTop (100);

            AttributedString s;
            s.append ("Click on a circle to make it flash. When it has finished flashing it will send a message which causes the next circle to flash");
            s.append (newLine);
            s.append ("Click the \"Set Random Colour\" button to change the colour of one of the circles.");
            s.append (newLine);
            s.setFont (16.0f);
            s.setColour (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::defaultText, Colours::lightgrey));
            s.draw (g, explanationArea.reduced (10).toFloat());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().removeFromBottom (40);
            randomColourButton.setBounds (area.removeFromLeft (166) .reduced (8));
            stopButton        .setBounds (area.removeFromRight (166).reduced (8));
        */
    }
    
    pub fn random_colour_button_clicked(&mut self)  {
        
        todo!();
        /*
            // Here we post a new ColourMessage with a random colour to a random flashing component.
            // This will send a message to the component asynchronously and trigger its handleMessage callback
            flashingComponents.getUnchecked (random.nextInt (flashingComponents.size()))->postMessage (new ColourMessage (getRandomBrightColour()));
        */
    }
    
    pub fn stop_button_clicked(&mut self)  {
        
        todo!();
        /*
            for (auto* fc : flashingComponents)
                fc->stopFlashing();
        */
    }
}
