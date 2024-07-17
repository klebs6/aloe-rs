crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/destinations/aloe_AnalyticsDestination.h]

pub trait AnalyticsDestinationInterface {

    /**
      | When an AnalyticsDestination is added to
      | an Analytics object this method is
      | called when an analytics event is
      | triggered from the Analytics object.
      |
      | Override this method to log the event
      | information somewhere useful.
      */
    fn log_event(&mut self, event: &AnalyticsEvent);
}

/**
  | An interface for handling analytics events
  | collected by an Analytics object.
  |
  | For basic analytics logging you can implement
  | this interface and add your class to an
  | Analytics object.
  |
  | For more advanced logging you may want to
  | subclass ThreadedAnalyticsDestination instead,
  | which is more suitable for interacting with
  | web servers and other time consuming
  | destinations.
  |
  | @see Analytics, ThreadedAnalyticsDestination
  |
  | @tags{Analytics}
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct AnalyticsDestination {

}
