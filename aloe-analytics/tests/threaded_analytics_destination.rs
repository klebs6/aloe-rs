
#[cfg(ALOE_UNIT_TESTS)]
pub mod tests {

    use super::*;

    pub mod destination_test_helpers {
        use super::*;
            
        pub struct BasicDestination {
            base:                 ThreadedAnalyticsDestination,
            logged_event_queue:   &mut VecDeque<AnalyticsEvent>,
            unlogged_event_store: &mut VecDeque<AnalyticsEvent>,
            logging_is_enabled:   bool, // default = true
            event_queue_changing: CriticalSection,
        }

        impl Drop for BasicDestination {
            fn drop(&mut self) {
                todo!();
                /* 
                        stopAnalyticsThread (1000);
                     */
            }
        }

        impl BasicDestination {

            pub fn new(
                logged_events:   &mut VecDeque<AnalyticsEvent>,
                unlogged_events: &mut VecDeque<AnalyticsEvent>) -> Self {
            
                todo!();
                /*


                    : ThreadedAnalyticsDestination ("ThreadedAnalyticsDestinationTest"),
                          loggedEventQueue (loggedEvents),
                          unloggedEventStore (unloggedEvents)
                        startAnalyticsThread (20);
                */
            }
            
            pub fn get_maximum_batch_size(&mut self) -> i32 {
                
                todo!();
                /*
                    return 5;
                */
            }
            
            pub fn save_unlogged_events(&mut self, events_to_save: &VecDeque<AnalyticsEvent>)  {
                
                todo!();
                /*
                    unloggedEventStore = eventsToSave;
                */
            }
            
            pub fn restore_unlogged_events(&mut self, restored_event_queue: &mut VecDeque<AnalyticsEvent>)  {
                
                todo!();
                /*
                    restoredEventQueue = unloggedEventStore;
                */
            }
            
            pub fn log_batched_events(&mut self, events: &[AnalyticsEvent]) -> bool {
                
                todo!();
                /*
                    jassert (events.size() <= getMaximumBatchSize());

                        if (loggingIsEnabled)
                        {
                            const ScopedLock lock (eventQueueChanging);

                            for (auto& event : events)
                                loggedEventQueue.push_back (event);

                            return true;
                        }

                        return false;
                */
            }
            
            pub fn stop_logging_events(&mut self)  {
                
                todo!();
                /*
                
                */
            }
            
            pub fn set_logging_enabled(&mut self, should_log_events: bool)  {
                
                todo!();
                /*
                    loggingIsEnabled = shouldLogEvents;
                */
            }
        }
    }

    pub struct ThreadedAnalyticsDestinationTests {
        base: UnitTest,
    }

    impl Default for ThreadedAnalyticsDestinationTests {
        
        fn default() -> Self {
            todo!();
            /*


                : UnitTest ("ThreadedAnalyticsDestination", UnitTestCategories::analytics
            */
        }
    }

    impl ThreadedAnalyticsDestinationTests {

        pub fn compare_event_queues(&mut self, 
            a: &VecDeque<AnalyticsDestination::AnalyticsEvent>,
            b: &VecDeque<AnalyticsDestination::AnalyticsEvent>)  {
            
            todo!();
            /*
                const auto numEntries = a.size();
                expectEquals ((int) b.size(), (int) numEntries);

                for (size_t i = 0; i < numEntries; ++i)
                {
                    expectEquals (a[i].name, b[i].name);
                    expect (a[i].timestamp == b[i].timestamp);
                }
            */
        }
        
        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                std::deque<AnalyticsDestination::AnalyticsEvent> testEvents;

                for (int i = 0; i < 7; ++i)
                    testEvents.push_back ({ String (i), 0, Time::getMillisecondCounter(), {}, "TestUser", {} });

                std::deque<AnalyticsDestination::AnalyticsEvent> loggedEvents, unloggedEvents;

                beginTest ("New events");

                {
                    DestinationTestHelpers::BasicDestination destination (loggedEvents, unloggedEvents);

                    for (auto& event : testEvents)
                        destination.logEvent (event);

                    size_t waitTime = 0, numLoggedEvents = 0;

                    while (numLoggedEvents < testEvents.size())
                    {
                        if (waitTime > 4000)
                        {
                            expect (waitTime < 4000);
                            break;
                        }

                        Thread::sleep (40);
                        waitTime += 40;

                        const ScopedLock lock (destination.eventQueueChanging);
                        numLoggedEvents = loggedEvents.size();
                    }
                }

                compareEventQueues (loggedEvents, testEvents);
                expect (unloggedEvents.size() == 0);

                loggedEvents.clear();

                beginTest ("Unlogged events");
                {
                    DestinationTestHelpers::BasicDestination destination (loggedEvents, unloggedEvents);
                    destination.setLoggingEnabled (false);

                    for (auto& event : testEvents)
                        destination.logEvent (event);
                }

                compareEventQueues (unloggedEvents, testEvents);
                expect (loggedEvents.size() == 0);
            */
        }
    }

    lazy_static!{
        /*
        static ThreadedAnalyticsDestinationTests threadedAnalyticsDestinationTests;
        */
    }
}
