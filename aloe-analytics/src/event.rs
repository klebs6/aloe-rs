crate::ix!();

/**
  | Contains information about an event
  | to be logged
  |
  */
pub struct AnalyticsEvent {

    /**
      | The name of the event.
      |
      */
    name: String,

    /** 
      | An optional integer representing the type
      | of the event. You can use this to
      | indicate if the event was a screenview,
      | session start, exception, etc.
      */
    event_type: i32,

    /**
      | The timestamp of the event.
      |
      | Timestamps are automatically applied
      | by an Analytics object and are derived
      | from Time::getMillisecondCounter(). As
      | such these timestamps do not represent
      | absolute times, but relative timings
      | of events for each user in each session
      | will be accurate.
      */
    timestamp: u32,

    /**
      | The parameters of the event.
      |
      */
    parameters: Vec<(String,String)>,

    /**
      | The user ID associated with the event
      |
      */
    userid: String,

    /**
      | Properties associated with the user
      |
      */
    user_properties: Vec<(String,String)>,
}
