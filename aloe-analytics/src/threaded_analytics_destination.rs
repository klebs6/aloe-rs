crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/destinations/aloe_ThreadedAnalyticsDestination.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_analytics/destinations/aloe_ThreadedAnalyticsDestination.cpp]

/**
  | A base class for dispatching analytics events
  | on a dedicated thread.
  |
  | This class is particularly useful for sending
  | analytics events to a web server without
  | blocking the message thread. It can also save
  | (and restore) events that were not dispatched
  | so no information is lost when an internet
  | connection is absent or something else
  | prevents successful logging.
  |
  | Once startAnalyticsThread is called the
  | logBatchedEvents method is periodically
  | invoked on an analytics thread, with the
  | period determined by calls to
  | setBatchPeriod. Here events are grouped
  | together into batches, with the maximum batch
  | size set by the implementation of
  | getMaximumBatchSize.
  |
  | It's important to call stopAnalyticsThread in
  | the destructor of your subclass (or before
  | then) to give the analytics thread time to
  | shut down.  Calling stopAnalyticsThread will,
  | in turn, call stopLoggingEvents, which you
  | should use to terminate the currently running
  | logBatchedEvents call.
  |
  | @see Analytics, AnalyticsDestination,
  | AnalyticsDestination::AnalyticsEvent
  |
  | @tags{Analytics}
  */
#[no_copy]
#[leak_detector]
pub struct ThreadedAnalyticsDestination<'a> {
    base:             AnalyticsDestination,
    destination_name: String,
    dispatcher:       EventDispatcher<'a>,
}

impl<'a> Drop for ThreadedAnalyticsDestination<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        // If you hit this assertion then the analytics thread has not been shut down
        // before this class is destroyed. Call stopAnalyticsThread() in your destructor!
        jassert (! dispatcher.isThreadRunning());
        */
    }
}

impl<'a> ThreadedAnalyticsDestination<'a> {

    /**
      | Creates a ThreadedAnalyticsDestination.
      | 
      | -----------
      | @param threadName
      | 
      | used to identify the analytics
      | thread in debug builds
      |
      */
    pub fn new(thread_name: Option<&str>) -> Self {

        let thread_name =
            thread_name.unwrap_or("Analytics thread");
    
        todo!();
        /*
            : dispatcher (threadName, *this)
        */
    }
    
    /**
      | Call this to set the period between 
      | logBatchedEvents invocations.
      | 
      | This method is thread safe and can be
      | used to implements things like
      | 
      | exponential backoff in logBatchedEvents
      | calls.
      | 
      | -----------
      | @param newSubmissionPeriodMilliseconds
      | 
      | the new submission period to use in 
      | milliseconds
      |
      */
    pub fn set_batch_period(&mut self, new_batch_period_milliseconds: i32)  {
        
        todo!();
        /*
            dispatcher.batchPeriodMilliseconds = newBatchPeriodMilliseconds;
        */
    }
    
    /**
      | Adds an event to the queue, which will
      | ultimately be submitted to logBatchedEvents.
      | 
      | This method is thread safe.
      | 
      | -----------
      | @param event
      | 
      | the analytics event to add to the queue
      |
      */
    pub fn log_event(&mut self, event: &AnalyticsEvent)  {
        
        todo!();
        /*
            dispatcher.addToQueue (event);
        */
    }
    
    /**
      | Starts the analytics thread, with an
      | initial event batching period.
      | 
      | -----------
      | @param initialBatchPeriodMilliseconds
      | 
      | the initial event batching period in
      | milliseconds
      |
      */
    pub fn start_analytics_thread(&mut self, initial_batch_period_milliseconds: i32)  {
        
        todo!();
        /*
            setBatchPeriod (initialBatchPeriodMilliseconds);
        dispatcher.startThread();
        */
    }
    
    /**
      | Triggers the shutdown of the analytics
      | thread. You must call this method in
      | the destructor of your subclass (or
      | before then) to give the analytics thread
      | time to shut down. This method invokes
      | stopLoggingEvents and you should ensure
      | that both the analytics thread and a
      | call to saveUnloggedEvents are able
      | to finish before the supplied timeout.
      | This timeout is important because on
      | platforms like iOS an app is killed if
      | it takes too long to shut down.
      | 
      | -----------
      | @param timeoutMilliseconds
      | 
      | the number of milliseconds before the
      | analytics thread is forcibly terminated
      |
      */
    pub fn stop_analytics_thread(&mut self, timeout: i32)  {
        
        todo!();
        /*
            dispatcher.signalThreadShouldExit();
        stopLoggingEvents();
        dispatcher.stopThread (timeout);

        if (dispatcher.eventQueue.size() > 0)
            saveUnloggedEvents (dispatcher.eventQueue);
        */
    }
}
