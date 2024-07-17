crate::ix!();

pub struct StandaloneFilterWindowMainContentComponentNotificationArea<'a> {
    base:            Component<'a>,
    notification:    Label<'a>,
    settings_button: TextButton<'a>,
}

impl<'a> StandaloneFilterWindowMainContentComponentNotificationArea<'a> {

    pub fn new(settings_button_listener: *mut dyn ButtonListener) -> Self {
    
        todo!();
        /*


            : notification ("notification", "Audio input is muted to avoid feedback loop"),
                         #if ALOE_IOS || ALOE_ANDROID
                          settingsButton ("Unmute Input")
                         #else
                          settingsButton ("Settings...")
                         #endif
                        setOpaque (true);

                        notification.setColour (Label::textColourId, Colours::black);

                        settingsButton.addListener (settingsButtonListener);

                        addAndMakeVisible (notification);
                        addAndMakeVisible (settingsButton);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

                        g.setColour (Colours::darkgoldenrod);
                        g.fillRect (r.removeFromBottom (1));

                        g.setColour (Colours::lightgoldenrodyellow);
                        g.fillRect (r);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (5);

                        settingsButton.setBounds (r.removeFromRight (70));
                        notification.setBounds (r);
        */
    }
}
