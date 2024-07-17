crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_TimeSliceThread.h]

/**
  | Used by the TimeSliceThread class.
  | 
  | To register your class with a TimeSliceThread,
  | derive from this class and use the
  | TimeSliceThread::addTimeSliceClient() method
  | to add it to the list.
  | 
  | Make sure you always call
  | TimeSliceThread::removeTimeSliceClient()
  | before deleting your client!
  | 
  | @see TimeSliceThread
  | 
  | @tags{Core}
  |
  */
pub struct TimeSliceClient {
    next_call_time: Time,
}

pub trait TimeSliceClientInterface {

    /**
      | Called back by a TimeSliceThread.
      | 
      | When you register this class with it,
      | a TimeSliceThread will repeatedly
      | call this method.
      | 
      | The implementation of this method should
      | use its time-slice to do something that's
      | quick - never block for longer than absolutely
      | necessary.
      | 
      | -----------
      | @return
      | 
      | Your method should return the number
      | of milliseconds which it would like
      | to wait before being called again. Returning
      | 0 will make the thread call again as soon
      | as possible (after possibly servicing
      | other busy clients). If you return a
      | value below zero, your client will be
      | removed from the list of clients, and
      | won't be called again. The value you
      | specify isn't a guarantee, and is only
      | used as a hint by the thread - the actual
      | time before the next callback may be
      | more or less than specified. You can
      | force the TimeSliceThread to wake up
      | and poll again immediately by calling
      | its notify() method.
      |
      */
    fn use_time_slice(&mut self) -> i32;
}

/**
  | A thread that keeps a list of clients,
  | and calls each one in turn, giving them
  | all a chance to run some sort of short
  | task.
  | 
  | @see TimeSliceClient, Thread
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TimeSliceThread {
    base:                Thread,
    callback_lock:       CriticalSection,
    list_lock:           CriticalSection,
    clients:             Vec<*mut TimeSliceClient>,
    client_being_called: *mut TimeSliceClient, // default = nullptr
}

impl Drop for TimeSliceThread {

    /**
      | Destructor.
      | 
      | Deleting a Thread object that is running
      | will only give the thread a brief opportunity
      | to stop itself cleanly, so it's recommended
      | that you should always call stopThread()
      | with a decent timeout before deleting,
      | to avoid the thread being forcibly killed
      | (which is a Bad Thing).
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
           stopThread (2000);
           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_TimeSliceThread.cpp]
impl TimeSliceThread {

    /**
      | Creates a TimeSliceThread.
      | 
      | When first created, the thread is not
      | running. Use the startThread() method
      | to start it.
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : thread(name),
        */
    }
    
    /**
      | Adds a client to the list.
      | 
      | The client's callbacks will start after
      | the number of milliseconds specified
      | by millisecondsBeforeStarting (and
      | this may happen before this method has
      | returned).
      |
      */
    pub fn add_time_slice_client(
        &mut self, 
        client:                       *mut TimeSliceClient,
        milliseconds_before_starting: Option<i32>

    )  {

        let milliseconds_before_starting: i32 = milliseconds_before_starting.unwrap_or(0);
        
        todo!();
        /*
            if (client != nullptr)
        {
            const ScopedLock sl (listLock);
            client->nextCallTime = Time::getCurrentTime() + RelativeTime::milliseconds (millisecondsBeforeStarting);
            clients.addIfNotAlreadyThere (client);
            notify();
        }
        */
    }
    
    /**
      | Removes a client from the list.
      | 
      | This method will make sure that all callbacks
      | to the client have completely finished
      | before the method returns.
      |
      */
    pub fn remove_time_slice_client(&mut self, client: *mut TimeSliceClient)  {
        
        todo!();
        /*
            const ScopedLock sl1 (listLock);

        // if there's a chance we're in the middle of calling this client, we need to
        // also lock the outer lock..
        if (clientBeingCalled == client)
        {
            const ScopedUnlock ul (listLock); // unlock first to get the order right..

            const ScopedLock sl2 (callbackLock);
            const ScopedLock sl3 (listLock);

            clients.removeFirstMatchingValue (client);
        }
        else
        {
            clients.removeFirstMatchingValue (client);
        }
        */
    }
    
    /**
      | Removes all the active and pending clients
      | from the list.
      | 
      | This method will make sure that all callbacks
      | to clients have finished before the
      | method returns.
      |
      */
    pub fn remove_all_clients(&mut self)  {
        
        todo!();
        /*
            for (;;)
        {
            if (auto* c = getClient (0))
                removeTimeSliceClient (c);
            else
                break;
        }
        */
    }
    
    /**
      | If the given client is waiting in the
      | queue, it will be moved to the front and
      | given a time-slice as soon as possible.
      | 
      | If the specified client has not been
      | added, nothing will happen.
      |
      */
    pub fn move_to_front_of_queue(&mut self, client: *mut TimeSliceClient)  {
        
        todo!();
        /*
            const ScopedLock sl (listLock);

        if (clients.contains (client))
        {
            client->nextCallTime = Time::getCurrentTime();
            notify();
        }
        */
    }
    
    /**
      | Returns the number of registered clients.
      |
      */
    pub fn get_num_clients(&self) -> i32 {
        
        todo!();
        /*
            return clients.size();
        */
    }
    
    /**
      | Returns one of the registered clients.
      |
      */
    pub fn get_client(&self, i: i32) -> *mut TimeSliceClient {
        
        todo!();
        /*
            const ScopedLock sl (listLock);
        return clients[i];
        */
    }
    
    pub fn get_next_client(&self, index: i32) -> *mut TimeSliceClient {
        
        todo!();
        /*
            Time soonest;
        TimeSliceClient* client = nullptr;

        for (int i = clients.size(); --i >= 0;)
        {
            auto* c = clients.getUnchecked ((i + index) % clients.size());

            if (c != nullptr && (client == nullptr || c->nextCallTime < soonest))
            {
                client = c;
                soonest = c->nextCallTime;
            }
        }

        return client;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            int index = 0;

        while (! threadShouldExit())
        {
            int timeToWait = 500;

            {
                Time nextClientTime;
                int numClients = 0;

                {
                    const ScopedLock sl2 (listLock);

                    numClients = clients.size();
                    index = numClients > 0 ? ((index + 1) % numClients) : 0;

                    if (auto* firstClient = getNextClient (index))
                        nextClientTime = firstClient->nextCallTime;
                }

                if (numClients > 0)
                {
                    auto now = Time::getCurrentTime();

                    if (nextClientTime > now)
                    {
                        timeToWait = (int) jmin ((int64) 500, (nextClientTime - now).inMilliseconds());
                    }
                    else
                    {
                        timeToWait = index == 0 ? 1 : 0;

                        const ScopedLock sl (callbackLock);

                        {
                            const ScopedLock sl2 (listLock);
                            clientBeingCalled = getNextClient (index);
                        }

                        if (clientBeingCalled != nullptr)
                        {
                            const int msUntilNextCall = clientBeingCalled->useTimeSlice();

                            const ScopedLock sl2 (listLock);

                            if (msUntilNextCall >= 0)
                                clientBeingCalled->nextCallTime = now + RelativeTime::milliseconds (msUntilNextCall);
                            else
                                clients.removeFirstMatchingValue (clientBeingCalled);

                            clientBeingCalled = nullptr;
                        }
                    }
                }
            }

            if (timeToWait > 0)
                wait (timeToWait);
        }
        */
    }
}
