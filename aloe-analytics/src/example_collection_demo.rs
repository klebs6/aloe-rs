crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/AnalyticsCollectionDemo.h]

pub enum DemoAnalyticsEventTypes
{
    event,
    sessionStart,
    sessionEnd,
    screenView,
    exception
}

#[no_copy]
#[leak_detector]
pub struct AnalyticsCollectionDemo<'a> {
    base:                   Component<'a>,
    event_button:           TextButton<'a>, // default = { "Press me!"  }
    crash_button:           TextButton<'a>, // default = { "Simulate crash!"  }
    log_event_button_press: Box<ButtonTracker<'a>>,
}

impl<'a> Default for AnalyticsCollectionDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // Add an analytics identifier for the user. Make sure you don't accidentally
            // collect identifiable information if you haven't asked for permission!
            Analytics::getInstance()->setUserId ("AnonUser1234");

            // Add any other constant user information.
            StringPairArray userData;
            userData.set ("group", "beta");
            Analytics::getInstance()->setUserProperties (userData);

            // Add any analytics destinations we want to use to the Analytics singleton.
            Analytics::getInstance()->addDestination (new GoogleAnalyticsDestination());

            // The event type here should probably be DemoAnalyticsEventTypes::sessionStart
            // in a more advanced app.
            Analytics::getInstance()->logEvent ("startup", {}, DemoAnalyticsEventTypes::event);

            crashButton.onClick = [this] { sendCrash(); };

            addAndMakeVisible (eventButton);
            addAndMakeVisible (crashButton);

            setSize (300, 200);

            StringPairArray logButtonPressParameters;
            logButtonPressParameters.set ("id", "a");
            logEventButtonPress.reset (new ButtonTracker (eventButton, "button_press", logButtonPressParameters))
        */
    }
}

impl<'a> Drop for AnalyticsCollectionDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            // The event type here should probably be DemoAnalyticsEventTypes::sessionEnd
            // in a more advanced app.
            Analytics::getInstance()->logEvent ("shutdown", {}, DemoAnalyticsEventTypes::event);
         */
    }
}

impl<'a> AnalyticsCollectionDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            eventButton.centreWithSize (100, 40);
            eventButton.setBounds (eventButton.getBounds().translated (0, 25));
            crashButton.setBounds (eventButton.getBounds().translated (0, -50));
        */
    }
    
    pub fn send_crash(&mut self)  {
        
        todo!();
        /*
            // In a more advanced application you would probably use a different event
            // type here.
            Analytics::getInstance()->logEvent ("crash", {}, DemoAnalyticsEventTypes::event);
            Analytics::getInstance()->getDestinations().clear();
            ALOEApplication::getInstance()->shutdown();
        */
    }
}
