crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/analytics/aloe_ButtonTracker.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/analytics/aloe_ButtonTracker.cpp]

/**
  | A class that automatically sends analytics
  | events to the Analytics singleton when
  | a button is clicked.
  |
  | @see Analytics,
  | AnalyticsDestination::AnalyticsEvent
  |
  | @tags{Analytics}
  */
#[no_copy]
#[leak_detector]
pub struct ButtonTracker<'a> {
    button:           &'a mut Button<'a>,
    event_name:       String,
    event_parameters: Vec<(String,String)>,
    event_type:       i32,
}

impl<'a> ButtonListener for ButtonTracker<'a> {

    fn button_clicked(&mut self, b: *mut Button)  {
        
        todo!();
        /*
            if (b == &button)
        {
            auto params = eventParameters;

            if (button.getClickingTogglesState())
                params.set ("ButtonState", button.getToggleState() ? "On" : "Off");

            Analytics::getInstance()->logEvent (eventName, params, eventType);
        }
        */
    }
}

impl<'a> Drop for ButtonTracker<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        button.removeListener (this);
         */
    }
}

impl<'a> ButtonTracker<'a> {

    /**
      | Creating one of these automatically
      | sends analytics events to the Analytics
      | singleton when the corresponding button
      | is clicked. The name and parameters
      | of the analytics event will be populated
      | from the variables supplied here. If
      | clicking changes the button's state
      | then the parameters will have a {"ButtonState",
      | "On"/"Off"} entry added.
      | 
      | -----------
      | @param buttonToTrack
      | 
      | the button to track
      | ----------
      | @param triggeredEventName
      | 
      | the name of the generated event
      | ----------
      | @param triggeredEventParameters
      | 
      | the parameters to add to the generated
      | event
      | ----------
      | @param triggeredEventType
      | 
      | (optional) an integer to indicate the
      | event type, which will be set to 0 if not
      | supplied.
      | 
      | @see Analytics, AnalyticsDestination::AnalyticsEvent
      |
      */
    pub fn new(
        button_to_track:            &mut Button,
        triggered_event_name:       &String,
        triggered_event_parameters: &Vec<(String,String)>,
        triggered_event_type:       Option<i32>

    ) -> Self {

        let triggered_event_type: i32 =
            triggered_event_type.unwrap_or(0);
    
        todo!();
        /*
        : button(buttonToTrack),
        : event_name(triggeredEventName),
        : event_parameters(triggeredEventParameters),
        : event_type(triggeredEventType),

            button.addListener (this);
        */
    }
}
