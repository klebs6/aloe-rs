crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_WaitableEvent.h]

/**
  | Allows threads to wait for events triggered
  | by other threads.
  | 
  | A thread can call WaitableEvent::wait()
  | to suspend the calling thread until
  | another thread wakes it up by calling
  | the WaitableEvent::signal() method.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct WaitableEvent {
    use_manual_reset: bool,
    mutex:            RefCell<parking_lot::RawMutex>,
    condition:        RefCell<std::sync::Condvar>,
    triggered:        RefCell<AtomicBool>, // default = false 
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_WaitableEvent.cpp]
impl WaitableEvent {

    /**
      | Creates a WaitableEvent object.
      | 
      | The object is initially in an unsignalled
      | state.
      | 
      | -----------
      | @param manualReset
      | 
      | If this is false, the event will be reset
      | automatically when the wait() method
      | is called. If manualReset is true, then
      | once the event is signalled, the only
      | way to reset it will be by calling the
      | reset() method.
      |
      */
    pub fn new(manual_reset: Option<bool>) -> Self {
    
        let manual_reset: bool = manual_reset.unwrap_or(false);

        todo!();
        /*
        : use_manual_reset(manualReset),
        */
    }
    
    /**
      | Suspends the calling thread until the
      | event has been signalled.
      | 
      | This will wait until the object's signal()
      | method is called by another thread,
      | or until the timeout expires.
      | 
      | After the event has been signalled,
      | this method will return true and if manualReset
      | was set to false in the WaitableEvent's
      | constructor, then the event will be
      | reset.
      | 
      | -----------
      | @param timeOutMilliseconds
      | 
      | the maximum time to wait, in milliseconds.
      | A negative value will cause it to wait
      | forever.
      | 
      | -----------
      | @return
      | 
      | true if the object has been signalled,
      | false if the timeout expires first.
      | @see signal, reset
      |
      */
    pub fn wait(&self, time_out_milliseconds: Option<i32>) -> bool {

        let time_out_milliseconds: i32 = time_out_milliseconds.unwrap_or(-1);
        
        todo!();
        /*
            std::unique_lock<std::mutex> lock (mutex);

        if (! triggered)
        {
            if (timeOutMilliseconds < 0)
            {
                condition.wait (lock, [this] { return triggered == true; });
            }
            else
            {
                if (! condition.wait_for (lock, std::chrono::milliseconds (timeOutMilliseconds),
                                          [this] { return triggered == true; }))
                {
                    return false;
                }
            }
        }

        if (! useManualReset)
            reset();

        return true;
        */
    }
    
    /**
      | Wakes up any threads that are currently
      | waiting on this object.
      | 
      | If signal() is called when nothing is
      | waiting, the next thread to call wait()
      | will return immediately and reset the
      | signal.
      | 
      | If the WaitableEvent is manual reset,
      | all current and future threads that
      | wait upon this object will be woken,
      | until reset() is explicitly called.
      | 
      | If the WaitableEvent is automatic reset,
      | and one or more threads is waiting upon
      | the object, then one of them will be woken
      | up. If no threads are currently waiting,
      | then the next thread to call wait() will
      | be woken up. As soon as a thread is woken,
      | the signal is automatically reset.
      | 
      | @see wait, reset
      |
      */
    pub fn signal(&self)  {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock (mutex);

        triggered = true;
        condition.notify_all();
        */
    }
    
    /**
      | Resets the event to an unsignalled state.
      | 
      | If it's not already signalled, this
      | does nothing.
      |
      */
    pub fn reset(&self)  {
        
        todo!();
        /*
            triggered = false;
        */
    }
}
