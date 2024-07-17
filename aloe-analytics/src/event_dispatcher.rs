crate::ix!();

pub struct EventDispatcher<'a> {
    base:                      Thread,
    parent:                    &'a mut ThreadedAnalyticsDestination<'a>,
    event_queue:               VecDeque<AnalyticsEvent>,
    queue_access:              CriticalSection,
    batch_period_milliseconds: Atomic<i32>, // default = 1000 
    events_to_send:            Vec<AnalyticsEvent>,
}

impl<'a> EventDispatcher<'a> {

    pub fn new(
        dispatcher_thread_name: &String,
        destination:            &mut ThreadedAnalyticsDestination) -> Self {
    
        todo!();
        /*
         : Thread (dispatcherThreadName),
          parent (destination)
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            // We may have inserted some events into the queue (on the message thread)
        // before this thread has started, so make sure the old events are at the
        // front of the queue.
        {
            std::deque<AnalyticsEvent> restoredEventQueue;
            parent.restoreUnloggedEvents (restoredEventQueue);

            const ScopedLock lock (queueAccess);

            for (auto rit = restoredEventQueue.rbegin(); rit != restoredEventQueue.rend(); ++rit)
                eventQueue.push_front (*rit);
        }

        const int maxBatchSize = parent.getMaximumBatchSize();

        while (! threadShouldExit())
        {
            {
                const ScopedLock lock (queueAccess);

                const auto numEventsInBatch = eventsToSend.size();
                const auto freeBatchCapacity = maxBatchSize - numEventsInBatch;

                if (freeBatchCapacity > 0)
                {
                    const auto numNewEvents = (int) eventQueue.size() - numEventsInBatch;

                    if (numNewEvents > 0)
                    {
                        const auto numEventsToAdd = jmin (numNewEvents, freeBatchCapacity);
                        const auto newBatchSize = numEventsInBatch + numEventsToAdd;

                        for (auto i = numEventsInBatch; i < newBatchSize; ++i)
                            eventsToSend.add (eventQueue[(size_t) i]);
                    }
                }
            }

            const auto submissionTime = Time::getMillisecondCounter();

            if (! eventsToSend.isEmpty())
            {
                if (parent.logBatchedEvents (eventsToSend))
                {
                    const ScopedLock lock (queueAccess);

                    for (auto i = 0; i < eventsToSend.size(); ++i)
                        eventQueue.pop_front();

                    eventsToSend.clearQuick();
                }
            }

            while (Time::getMillisecondCounter() - submissionTime < (uint32) batchPeriodMilliseconds.get())
            {
                if (threadShouldExit())
                    return;

                Thread::sleep (100);
            }
        }
        */
    }
    
    pub fn add_to_queue(&mut self, event: &AnalyticsEvent)  {
        
        todo!();
        /*
            const ScopedLock lock (queueAccess);
        eventQueue.push_back (event);
        */
    }
}
