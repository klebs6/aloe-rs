crate::ix!();

pub trait ThreadedAnalyticsDestinationInterface {

    /**
      | Override this method to provide the
      | maximum batch size you can handle in
      | your subclass.
      | 
      | Calls to logBatchedEvents will contain
      | no more than this number of events.
      |
      */
    fn get_maximum_batch_size(&mut self) -> i32;

    /**
      | This method will be called periodically
      | on the analytics thread.
      | 
      | If this method returns false then the
      | subsequent call of this function will
      | contain the same events as previous
      | call, plus any new events that have been
      | generated in the period between calls.
      | The order of events will not be changed. 
      | This allows you to retry logging events 
      | until they are logged successfully.
      | 
      | -----------
      | @param events
      | 
      | a list of events to be logged
      | 
      | -----------
      | @return
      | 
      | if the events were successfully logged
      |
      */
    fn log_batched_events(&mut self, 
        events: &[AnalyticsEvent]) -> bool;

    /**
      | You must always call stopAnalyticsThread
      | in the destructor of your subclass (or
      | before then) to give the analytics thread
      | time to shut down.
      |
      | Calling stopAnalyticsThread triggers
      | a call to this method. At this point you
      | are guaranteed that logBatchedEvents has
      | been called for the last time and you
      | should make sure that the current call to
      | logBatchedEvents finishes as quickly as
      | possible. This method and a subsequent
      | call to saveUnloggedEvents must both
      | complete before the timeout supplied to
      | stopAnalyticsThread.
      |
      | In a normal use case stopLoggingEvents
      | will be called on the message thread from
      | the destructor of your
      | ThreadedAnalyticsDestination subclass, and
      | must stop the logBatchedEvents method
      | which is running on the analytics thread.
      |
      | @see stopAnalyticsThread
      */
    fn stop_logging_events(&mut self);

    /**
      | This method will be called when the analytics
      | thread is shut down, giving you the chance
      | to save any analytics events that could
      | not be logged. Once saved these events
      | can be put back into the queue of events
      | when the ThreadedAnalyticsDestination
      | is recreated via restoreUnloggedEvents.
      | This method should return as quickly
      | as possible, as both stopLoggingEvents
      | and this method need to complete inside
      | the timeout set in stopAnalyticsThread.
      | 
      | -----------
      | @param eventsToSave
      | 
      | the events that could not be logged
      | 
      | @see stopAnalyticsThread, stopLoggingEvents,
      | restoreUnloggedEvents
      |
      */
    fn save_unlogged_events(&mut self, 
        events_to_save: &VecDeque<AnalyticsEvent>);

    /**
      | The counterpart to saveUnloggedEvents.
      | Events added to the event queue provided
      | by this method will be the first events
      | supplied to logBatchedEvents calls.
      | Use this method to restore any unlogged
      | events previously stored in a call to
      | saveUnloggedEvents. This method is
      | called on the analytics thread.
      | 
      | -----------
      | @param restoredEventQueue
      | 
      | place restored events into this queue
      | 
      | @see saveUnloggedEvents
      |
      */
    fn restore_unlogged_events(&mut self, 
        restored_event_queue: &mut VecDeque<AnalyticsEvent>);
}
