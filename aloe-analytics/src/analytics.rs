crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/analytics/aloe_Analytics.h]

/**
  | A singleton class to manage analytics data.
  |
  | Use an Analytics object to manage sending
  | analytics data to one or more
  | AnalyticsDestinations.
  |
  | @see AnalyticsDestination,
  |      ThreadedAnalyticsDestination,
  |      AnalyticsDestination::AnalyticsEvent
  |
  | @tags{Analytics}
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct Analytics {
    base:            DeletedAtShutdown,
    user_id:         String,
    user_properties: Vec<(String,String)>,
    is_suspended:    bool, // default = false
    destinations:    Vec<Box<AnalyticsDestination>>,
}

aloe_declare_singleton!{
    Analytics, false
}

aloe_implement_singleton!{
    Analytics
}

impl Drop for Analytics {
    fn drop(&mut self) {
        todo!();
        /*      clearSingletonInstance();  */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/analytics/aloe_Analytics.cpp]
impl Analytics {

    /**
      | Adds an AnalyticsDestination to the
      | list of AnalyticsDestinations managed 
      | by this Analytics object.
      | 
      | The Analytics class will take ownership
      | of the AnalyticsDestination
      | 
      | passed to this function.
      | 
      | -----------
      | @param destination
      | 
      | the AnalyticsDestination to manage
      |
      */
    pub fn add_destination(&mut self, destination: *mut AnalyticsDestination)  {
        
        todo!();
        /*
            destinations.add (destination);
        */
    }
    
    /** 
      | Returns the array of AnalyticsDestinations
      | managed by this class.
      |
      | If you have added any subclasses of
      | ThreadedAnalyticsDestination to this class
      | then you can remove them from this list to
      | force them to flush any pending events.
      */
    pub fn get_destinations(&mut self) -> &mut Vec<Box<AnalyticsDestination>> {
        
        todo!();
        /*
            return destinations;
        */
    }
    
    /**
      | Sets a user ID that will be added to all
      | AnalyticsEvents sent to AnalyticsDestinations.
      | 
      | -----------
      | @param newUserId
      | 
      | the userId to add to AnalyticsEvents
      |
      */
    pub fn set_user_id(&mut self, new_user_id: String)  {
        
        todo!();
        /*
            userId = newUserId;
        */
    }
    
    /**
      | Sets some user properties that will
      | be added to all AnalyticsEvents sent
      | to AnalyticsDestinations.
      | 
      | -----------
      | @param properties
      | 
      | the userProperties to add to AnalyticsEvents
      |
      */
    pub fn set_user_properties(&mut self, properties: Vec<(String,String)>)  {
        
        todo!();
        /*
            userProperties = properties;
        */
    }
    
    /**
      | Sends an AnalyticsEvent to all 
      | AnalyticsDestinations.
      |
      | The AnalyticsEvent will be timestamped,
      | and will have the userId and userProperties
      | populated by values previously set
      | by calls to setUserId and setUserProperties.
      | The name, parameters and type will be
      | populated by the arguments supplied
      | to this function.
      | 
      | -----------
      | @param eventName
      | 
      | the event name
      | ----------
      | @param parameters
      | 
      | the event parameters
      | ----------
      | @param eventType
      | 
      | (optional) an integer to indicate the
      | event type, which will be set to 0 if not
      | supplied.
      |
      */
    pub fn log_event(
        &mut self, 
        event_name: &String,
        parameters: &Vec<(String,String)>,
        event_type: Option<i32>

    ) {

        let event_type: i32 = event_type.unwrap_or(0);
        
        todo!();
        /*
            if (! isSuspended)
        {
            AnalyticsDestination::AnalyticsEvent event
            {
                eventName,
                eventType,
                Time::getMillisecondCounter(),
                parameters,
                userId,
                userProperties
            };

            for (auto* destination : destinations)
                destination->logEvent (event);
        }
        */
    }
    
    /**
      | Suspends analytics submissions to
      | AnalyticsDestinations.
      | 
      | -----------
      | @param shouldBeSuspended
      | 
      | if event submission should be suspended
      |
      */
    pub fn set_suspended(&mut self, should_be_suspended: bool)  {
        
        todo!();
        /*
            isSuspended = shouldBeSuspended;
        */
    }
}
