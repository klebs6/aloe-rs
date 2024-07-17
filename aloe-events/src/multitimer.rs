crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/timers/aloe_MultiTimer.h]

/**
  | A type of timer class that can run multiple
  | timers with different frequencies,
  | all of which share a single callback.
  | 
  | This class is very similar to the Timer
  | class, but allows you run multiple separate
  | timers, where each one has a unique ID
  | number. The methods in this class are
  | exactly equivalent to those in Timer,
  | but with the addition of this ID number.
  | 
  | To use it, you need to create a subclass
  | of MultiTimer, implementing the timerCallback()
  | method. Then you can start timers with
  | startTimer(), and each time the callback
  | is triggered, it passes in the ID of the
  | timer that caused it.
  | 
  | @see Timer
  | 
  | @tags{Events}
  |
  */
pub struct MultiTimer {
    timer_list_lock: SpinLock,
    timers:          Vec<Box<Timer>>,
}

pub trait MultiTimerInterface {

    /**
      | The user-defined callback routine
      | that actually gets called by each of
      | the timers that are running.
      | 
      | It's perfectly ok to call startTimer()
      | or stopTimer() from within this callback
      | to change the subsequent intervals.
      |
      */
    fn timer_callback(&mut self, timerid: i32);
}

impl Default for MultiTimer {
    
    /**
      | Creates a MultiTimer.
      | 
      | When created, no timers are running,
      | so use startTimer() to start things
      | off.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/timers/aloe_MultiTimer.cpp]
impl Drop for MultiTimer {

    fn drop(&mut self) {
        todo!();
        /* 
        const SpinLock::ScopedLockType sl (timerListLock);
        timers.clear();
 */
    }
}

impl MultiTimer {

    /**
      | Creates a copy of another timer.
      | 
      | Note that this timer will not contain
      | any running timers, even if the one you're
      | copying from was running.
      |
      */
    pub fn new(_0: &MultiTimer) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    pub fn get_callback(&self, timerid: i32) -> *mut Timer {
        
        todo!();
        /*
            for (int i = timers.size(); --i >= 0;)
        {
            MultiTimerCallback* const t = static_cast<MultiTimerCallback*> (timers.getUnchecked(i));

            if (t->timerID == timerID)
                return t;
        }

        return nullptr;
        */
    }
    
    /**
      | Starts a timer and sets the length of
      | interval required.
      | 
      | If the timer is already started, this
      | will reset it, so the time between calling
      | this method and the next timer callback
      | will not be less than the interval length
      | passed in.
      | 
      | -----------
      | @param timerID
      | 
      | a unique Id number that identifies the
      | timer to start. This is the id that will
      | be passed back to the timerCallback()
      | method when this timer is triggered
      | ----------
      | @param intervalInMilliseconds
      | 
      | the interval to use (any values less
      | than 1 will be rounded up to 1)
      |
      */
    pub fn start_timer(&mut self, 
        timerid:                  i32,
        interval_in_milliseconds: i32)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (timerListLock);

        Timer* timer = getCallback (timerID);

        if (timer == nullptr)
            timers.add (timer = new MultiTimerCallback (timerID, *this));

        timer->startTimer (intervalInMilliseconds);
        */
    }
    
    /**
      | Stops a timer.
      | 
      | If a timer has been started with the given
      | ID number, it will be cancelled. No more
      | callbacks will be made for the specified
      | timer after this method returns.
      | 
      | If this is called from a different thread,
      | any callbacks that may be currently
      | executing may be allowed to finish before
      | the method returns.
      |
      */
    pub fn stop_timer(&mut self, timerid: i32)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (timerListLock);

        if (Timer* const t = getCallback (timerID))
            t->stopTimer();
        */
    }
    
    /**
      | Checks whether a timer has been started
      | for a specified ID.
      | 
      | -----------
      | @return
      | 
      | true if a timer with the given ID is running.
      |
      */
    pub fn is_timer_running(&self, timerid: i32) -> bool {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (timerListLock);

        if (Timer* const t = getCallback (timerID))
            return t->isTimerRunning();

        return false;
        */
    }
    
    /**
      | Returns the interval for a specified
      | timer ID.
      | 
      | -----------
      | @return
      | 
      | the timer's interval in milliseconds
      | if it's running, or 0 if no timer was running
      | for the ID number specified.
      |
      */
    pub fn get_timer_interval(&self, timerid: i32) -> i32 {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (timerListLock);

        if (Timer* const t = getCallback (timerID))
            return t->getTimerInterval();

        return 0;
        */
    }
}

///---------------------------
#[no_copy]
#[leak_detector]
pub struct MultiTimerCallback<'a> {
    base:    Timer,
    owner:   &'a mut MultiTimer,
    timerid: i32,
}

impl<'a> MultiTimerCallback<'a> {

    pub fn new(
        tid: i32,
        mt:  &mut MultiTimer) -> Self {
    
        todo!();
        /*
        : owner(mt),
        : timerid(tid),
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            owner.timerCallback (timerID);
        */
    }
}
