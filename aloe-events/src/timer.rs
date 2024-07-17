crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/timers/aloe_Timer.h]

/**
  | Makes repeated callbacks to a virtual
  | method at a specified time interval.
  | 
  | A Timer's timerCallback() method will
  | be repeatedly called at a given interval.
  | When you create a Timer object, it will
  | do nothing until the startTimer() method
  | is called, which will cause the message
  | thread to start making callbacks at
  | the specified interval, until stopTimer()
  | is called or the object is deleted.
  | 
  | The time interval isn't guaranteed
  | to be precise to any more than maybe 10-20ms,
  | and the intervals may end up being much
  | longer than requested if the system
  | is busy. Because the callbacks are made
  | by the main message thread, anything
  | that blocks the message queue for a period
  | of time will also prevent any timers
  | from running until it can carry on.
  | 
  | If you need to have a single callback
  | that is shared by multiple timers with
  | different frequencies, then the MultiTimer
  | class allows you to do that - its structure
  | is very similar to the Timer class, but
  | contains multiple timers internally,
  | each one identified by an ID number.
  | 
  | @see HighResolutionTimer, MultiTimer
  | 
  | @tags{Events}
  |
  */
pub struct Timer {
    position_in_queue: usize, // default = -1 as usize
    timer_period_ms:   i32, // default = 0
}

pub trait TimerInterface {

    /**
      | The user-defined callback routine
      | that actually gets called periodically.
      | 
      | It's perfectly ok to call startTimer()
      | or stopTimer() from within this callback
      | to change the subsequent intervals.
      |
      */
    fn timer_callback(&mut self);
}

#[no_copy]
#[leak_detector]
pub struct TimerThread<'a> {
    base:             Thread,
    base2:            DeletedAtShutdown,
    base3:            AsyncUpdater<'a>,
    timers:           Vec<TimerCountdown>,
    callback_arrived: WaitableEvent,
}

pub mod timer_thread {
    use super::*;

    lazy_static!{
        /*
        static TimerThread* instance;
        static LockType lock;
        Timer::TimerThread* Timer::TimerThread::instance = nullptr;
        Timer::TimerThread::LockType Timer::TimerThread::lock;
        */
    }
}

pub struct TimerCountdown {
    timer:        *mut Timer,
    countdown_ms: i32,
}

#[derive(Default)]
pub struct CallTimersMessage;

impl MessageBaseInterface for CallTimersMessage {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            if (instance != nullptr)
                        instance->callTimers();
        */
    }
}

/**
  | (mysteriously, using a SpinLock here
  | causes problems on some XP machines..)
  |
  */
pub type TimerThreadLockType = CriticalSection;

impl<'a> Default for TimerThread<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : Thread ("Aloe Timer")
                timers.reserve (32);
                triggerAsyncUpdate()
        */
    }
}

impl<'a> Drop for TimerThread<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
                signalThreadShouldExit();
                callbackArrived.signal();
                stopThread (4000);
                jassert (instance == this || instance == nullptr);

                if (instance == this)
                    instance = nullptr;
             */
    }
}

impl<'a> TimerThread<'a> {

    pub fn run(&mut self)  {
        
        todo!();
        /*
            auto lastTime = Time::getMillisecondCounter();
                ReferenceCountedObjectPtr<CallTimersMessage> messageToSend (new CallTimersMessage());

                while (! threadShouldExit())
                {
                    auto now = Time::getMillisecondCounter();
                    auto elapsed = (int) (now >= lastTime ? (now - lastTime)
                                                          : (std::numeric_limits<uint32>::max() - (lastTime - now)));
                    lastTime = now;

                    auto timeUntilFirstTimer = getTimeUntilFirstTimer (elapsed);

                    if (timeUntilFirstTimer <= 0)
                    {
                        if (callbackArrived.wait (0))
                        {
                            // already a message in flight - do nothing..
                        }
                        else
                        {
                            messageToSend->post();

                            if (! callbackArrived.wait (300))
                            {
                                // Sometimes our message can get discarded by the OS (e.g. when running as an RTAS
                                // when the app has a modal loop), so this is how long to wait before assuming the
                                // message has been lost and trying again.
                                messageToSend->post();
                            }

                            continue;
                        }
                    }

                    // don't wait for too long because running this loop also helps keep the
                    // Time::getApproximateMillisecondTimer value stay up-to-date
                    wait (jlimit (1, 100, timeUntilFirstTimer));
                }
        */
    }
    
    pub fn call_timers(&mut self)  {
        
        todo!();
        /*
            auto timeout = Time::getMillisecondCounter() + 100;

                const LockType::ScopedLockType sl (lock);

                while (! timers.empty())
                {
                    auto& first = timers.front();

                    if (first.countdownMs > 0)
                        break;

                    auto* timer = first.timer;
                    first.countdownMs = timer->timerPeriodMs;
                    shuffleTimerBackInQueue (0);
                    notify();

                    const LockType::ScopedUnlockType ul (lock);

                    ALOE_TRY
                    {
                        timer->timerCallback();
                    }
                    ALOE_CATCH_EXCEPTION

                    // avoid getting stuck in a loop if a timer callback repeatedly takes too long
                    if (Time::getMillisecondCounter() > timeout)
                        break;
                }

                callbackArrived.signal();
        */
    }
    
    pub fn call_timers_synchronously(&mut self)  {
        
        todo!();
        /*
            if (! isThreadRunning())
                {
                    // (This is relied on by some plugins in cases where the MM has
                    // had to restart and the async callback never started)
                    cancelPendingUpdate();
                    triggerAsyncUpdate();
                }

                callTimers();
        */
    }
    
    pub fn add(tim: *mut Timer)  {
        
        todo!();
        /*
            if (instance == nullptr)
                    instance = new TimerThread();

                instance->addTimer (tim);
        */
    }
    
    pub fn remove(tim: *mut Timer)  {
        
        todo!();
        /*
            if (instance != nullptr)
                    instance->removeTimer (tim);
        */
    }
    
    pub fn reset_counter(tim: *mut Timer)  {
        
        todo!();
        /*
            if (instance != nullptr)
                    instance->resetTimerCounter (tim);
        */
    }
    
    pub fn add_timer(&mut self, t: *mut Timer)  {
        
        todo!();
        /*
            // Trying to add a timer that's already here - shouldn't get to this point,
                // so if you get this assertion, let me know!
                jassert (std::find_if (timers.begin(), timers.end(),
                                       [t] (TimerCountdown i) { return i.timer == t; }) == timers.end());

                auto pos = timers.size();

                timers.push_back ({ t, t->timerPeriodMs });
                t->positionInQueue = pos;
                shuffleTimerForwardInQueue (pos);
                notify();
        */
    }
    
    pub fn remove_timer(&mut self, t: *mut Timer)  {
        
        todo!();
        /*
            auto pos = t->positionInQueue;
                auto lastIndex = timers.size() - 1;

                jassert (pos <= lastIndex);
                jassert (timers[pos].timer == t);

                for (auto i = pos; i < lastIndex; ++i)
                {
                    timers[i] = timers[i + 1];
                    timers[i].timer->positionInQueue = i;
                }

                timers.pop_back();
        */
    }
    
    pub fn reset_timer_counter(&mut self, t: *mut Timer)  {
        
        todo!();
        /*
            auto pos = t->positionInQueue;

                jassert (pos < timers.size());
                jassert (timers[pos].timer == t);

                auto lastCountdown = timers[pos].countdownMs;
                auto newCountdown = t->timerPeriodMs;

                if (newCountdown != lastCountdown)
                {
                    timers[pos].countdownMs = newCountdown;

                    if (newCountdown > lastCountdown)
                        shuffleTimerBackInQueue (pos);
                    else
                        shuffleTimerForwardInQueue (pos);

                    notify();
                }
        */
    }
    
    pub fn shuffle_timer_back_in_queue(&mut self, pos: usize)  {
        
        todo!();
        /*
            auto numTimers = timers.size();

                if (pos < numTimers - 1)
                {
                    auto t = timers[pos];

                    for (;;)
                    {
                        auto next = pos + 1;

                        if (next == numTimers || timers[next].countdownMs >= t.countdownMs)
                            break;

                        timers[pos] = timers[next];
                        timers[pos].timer->positionInQueue = pos;

                        ++pos;
                    }

                    timers[pos] = t;
                    t.timer->positionInQueue = pos;
                }
        */
    }
    
    pub fn shuffle_timer_forward_in_queue(&mut self, pos: usize)  {
        
        todo!();
        /*
            if (pos > 0)
                {
                    auto t = timers[pos];

                    while (pos > 0)
                    {
                        auto& prev = timers[(size_t) pos - 1];

                        if (prev.countdownMs <= t.countdownMs)
                            break;

                        timers[pos] = prev;
                        timers[pos].timer->positionInQueue = pos;

                        --pos;
                    }

                    timers[pos] = t;
                    t.timer->positionInQueue = pos;
                }
        */
    }
    
    pub fn get_time_until_first_timer(&mut self, num_millisecs_elapsed: i32) -> i32 {
        
        todo!();
        /*
            const LockType::ScopedLockType sl (lock);

                if (timers.empty())
                    return 1000;

                for (auto& t : timers)
                    t.countdownMs -= numMillisecsElapsed;

                return timers.front().countdownMs;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            startThread (7);
        */
    }
}

impl Default for Timer {

    /**
      | Creates a Timer. When created, the timer
      | is stopped, so use startTimer() to get
      | it going.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        */
    }
}

impl Drop for Timer {

    fn drop(&mut self) {
        todo!();
        /* 
        // If you're destroying a timer on a background thread, make sure the timer has
        // been stopped before execution reaches this point. A simple way to achieve this
        // is to add a call to `stopTimer()` to the destructor of your class which inherits
        // from Timer.
        jassert (! isTimerRunning()
                 || MessageManager::getInstanceWithoutCreating() == nullptr
                 || MessageManager::getInstanceWithoutCreating()->currentThreadHasLockedMessageManager());

        stopTimer();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/timers/aloe_Timer.cpp]
impl Timer {

    /**
      | Creates a copy of another timer.
      | 
      | Note that this timer won't be started,
      | even if the one you're copying is running.
      |
      */
    pub fn new(_0: &Timer) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the timer is currently
      | running.
      |
      */
    pub fn is_timer_running(&self) -> bool {
        
        todo!();
        /*
            return timerPeriodMs > 0;
        */
    }

    /**
      | Returns the timer's interval.
      | 
      | -----------
      | @return
      | 
      | the timer's interval in milliseconds
      | if it's running, or 0 if it's not.
      |
      */
    pub fn get_timer_interval(&self) -> i32 {
        
        todo!();
        /*
            return timerPeriodMs;
        */
    }
    
    /**
      | Starts the timer and sets the length
      | of interval required.
      | 
      | If the timer is already started, this
      | will reset it, so the time between calling
      | this method and the next timer callback
      | will not be less than the interval length
      | passed in.
      | 
      | -----------
      | @param intervalInMilliseconds
      | 
      | the interval to use (any value less than
      | 1 will be rounded up to 1)
      |
      */
    pub fn start_timer(&mut self, interval: i32)  {
        
        todo!();
        /*
            // If you're calling this before (or after) the MessageManager is
        // running, then you're not going to get any timer callbacks!
        ALOE_ASSERT_MESSAGE_MANAGER_EXISTS

        const TimerThread::LockType::ScopedLockType sl (TimerThread::lock);

        bool wasStopped = (timerPeriodMs == 0);
        timerPeriodMs = jmax (1, interval);

        if (wasStopped)
            TimerThread::add (this);
        else
            TimerThread::resetCounter (this);
        */
    }
    
    /**
      | Starts the timer with an interval specified
      | in Hertz.
      | 
      | This is effectively the same as calling
      | startTimer (1000 / timerFrequencyHz).
      |
      */
    pub fn start_timer_hz(&mut self, timer_frequency_hz: i32)  {
        
        todo!();
        /*
            if (timerFrequencyHz > 0)
            startTimer (1000 / timerFrequencyHz);
        else
            stopTimer();
        */
    }
    
    /**
      | Stops the timer.
      | 
      | No more timer callbacks will be triggered
      | after this method returns.
      | 
      | -----------
      | @note
      | 
      | if you call this from a background thread
      | while the message-thread is already
      | in the middle of your callback, then
      | this method will cancel any future timer
      | callbacks, but it will return without
      | waiting for the current one to finish.
      | The current callback will continue,
      | possibly still running some of your
      | timer code after this method has returned.
      |
      */
    pub fn stop_timer(&mut self)  {
        
        todo!();
        /*
            const TimerThread::LockType::ScopedLockType sl (TimerThread::lock);

        if (timerPeriodMs > 0)
        {
            TimerThread::remove (this);
            timerPeriodMs = 0;
        }
        */
    }
    
    /**
      | For internal use only: invokes any timers
      | that need callbacks.
      | 
      | Don't call this unless you really know
      | what you're doing!
      |
      */
    pub fn call_pending_timers_synchronously(&mut self)  {
        
        todo!();
        /*
            if (TimerThread::instance != nullptr)
            TimerThread::instance->callTimersSynchronously();
        */
    }
    
    /**
      | Invokes a lambda after a given number
      | of milliseconds.
      |
      */
    pub fn call_after_delay(&mut self, 
        milliseconds: i32,
        f:            fn() -> ())  {
        
        todo!();
        /*
            new LambdaInvoker (milliseconds, f);
        */
    }
}

///---------------------------
#[no_copy]
pub struct LambdaInvoker {
    base:     Timer,
    function: fn() -> (),
}

impl LambdaInvoker {
    
    pub fn new(
        milliseconds: i32,
        f:            fn() -> ()) -> Self {
    
        todo!();
        /*
        : function(f),

            startTimer (milliseconds);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto f = function;
            delete this;
            f();
        */
    }
}
