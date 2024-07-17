crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_HighResolutionTimer.h]

/**
  | A high-resolution periodic timer.
  | 
  | This provides accurately-timed regular
  | callbacks. Unlike the normal Timer
  | class, this one uses a dedicated thread,
  | not the message thread, so is far more
  | stable and precise.
  | 
  | You should only use this class in situations
  | where you really need accuracy, because
  | unlike the normal Timer class, which
  | is very lightweight and cheap to start/stop,
  | the HighResolutionTimer will use far
  | more resources, and starting/stopping
  | it may involve launching and killing
  | threads.
  | 
  | @see Timer
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct HighResolutionTimer<'a> {
    pimpl: Box<HighResolutionTimerPimpl<'a>>,
}

pub trait HighResolutionTimerInterface {

    /**
      | The user-defined callback routine
      | that actually gets called periodically.
      | 
      | This will be called on a dedicated timer
      | thread, so make sure your implementation
      | is thread-safe!
      | 
      | It's perfectly ok to call startTimer()
      | or stopTimer() from within this callback
      | to change the subsequent intervals.
      |
      */
    fn hi_res_timer_callback(&mut self);
}

impl<'a> Default for HighResolutionTimer<'a> {
    
    /**
      | Creates a HighResolutionTimer.
      | 
      | When created, the timer is stopped,
      | so use startTimer() to get it going.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            : pimpl (new Pimpl (*this)
        */
    }
}

impl<'a> Drop for HighResolutionTimer<'a> {
    fn drop(&mut self) {
        todo!();
        /*      stopTimer();  */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_HighResolutionTimer.cpp]
impl<'a> HighResolutionTimer<'a> {

    /**
      | Starts the timer and sets the length
      | of interval required.
      | 
      | If the timer is already started, this
      | will reset its counter, so the time between
      | calling this method and the next timer
      | callback will not be less than the interval
      | length passed in.
      | 
      | -----------
      | @param intervalInMilliseconds
      | 
      | the interval to use (any values less
      | than 1 will be rounded up to 1)
      |
      */
    pub fn start_timer(&mut self, period_ms: i32)  {
        
        todo!();
        /*
            pimpl->start (jmax (1, periodMs));
        */
    }
    
    /**
      | Stops the timer.
      | 
      | This method may block while it waits
      | for pending callbacks to complete.
      | Once it returns, no more callbacks will
      | be made. If it is called from the timer's
      | own thread, it will cancel the timer
      | after the current callback returns.
      |
      */
    pub fn stop_timer(&mut self)  {
        
        todo!();
        /*
            pimpl->stop();
        */
    }
    
    /**
      | Checks if the timer has been started.
      | 
      | -----------
      | @return
      | 
      | true if the timer is running.
      |
      */
    pub fn is_timer_running(&self) -> bool {
        
        todo!();
        /*
            return pimpl->periodMs != 0;
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
            return pimpl->periodMs;
        */
    }
}

#[no_copy]
struct HighResolutionTimerPimpl<'a> {
    owner:       &'a mut HighResolutionTimer<'a>,
    period_ms:   Atomic<i32>, // default = 0 
    thread:      Thread,
    stop_cond:   std::sync::Condvar,
    timer_mutex: parking_lot::RawMutex,
}

impl<'a> Drop for HighResolutionTimerPimpl<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            jassert (periodMs == 0);
            stop();
         */
    }
}

impl<'a> HighResolutionTimerPimpl<'a> {

    fn new(t: &mut HighResolutionTimer<'a>) -> Self {
    
        todo!();
        /*
        : owner(t),

        
        */
    }
    
    fn start(&mut self, new_period: i32)  {
        
        todo!();
        /*
            if (periodMs == newPeriod)
                return;

            if (thread.get_id() == std::this_thread::get_id())
            {
                periodMs = newPeriod;
                return;
            }

            stop();

            periodMs = newPeriod;

            thread = std::thread ([this, newPeriod]
            {
                setThisThreadToRealtime ((uint64) newPeriod);

                auto lastPeriod = periodMs.load();
                Clock clock (lastPeriod);

                std::unique_lock<std::mutex> unique_lock (timerMutex);

                while (periodMs != 0)
                {
                    clock.next();
                    while (periodMs != 0 && clock.wait (stopCond, unique_lock));

                    if (periodMs == 0)
                        break;

                    owner.hiResTimerCallback();

                    auto nextPeriod = periodMs.load();

                    if (lastPeriod != nextPeriod)
                    {
                        lastPeriod = nextPeriod;
                        clock = Clock (lastPeriod);
                    }
                }

                periodMs = 0;
            });
        */
    }
    
    fn stop(&mut self)  {
        
        todo!();
        /*
            periodMs = 0;

            const auto thread_id = thread.get_id();

            if (thread_id == std::thread::id() || thread_id == std::this_thread::get_id())
                return;

            {
                std::unique_lock<std::mutex> unique_lock (timerMutex);
                stopCond.notify_one();
            }

            thread.join();
        */
    }
    
    fn set_this_thread_to_realtime(period_ms: u64) -> bool {
        
        todo!();
        /*
            const auto thread = pthread_self();

           #if ALOE_MAC || ALOE_IOS
            mach_timebase_info_data_t timebase;
            mach_timebase_info (&timebase);

            const auto ticksPerMs = ((double) timebase.denom * 1000000.0) / (double) timebase.numer;
            const auto periodTicks = (uint32_t) (ticksPerMs * periodMs);

            thread_time_constraint_policy_data_t policy;
            policy.period      = periodTicks;
            policy.computation = jmin ((uint32_t) 50000, policy.period);
            policy.constraint  = policy.period;
            policy.preemptible = true;

            return thread_policy_set (pthread_mach_thread_np (thread),
                                      THREAD_TIME_CONSTRAINT_POLICY,
                                      (thread_policy_t) &policy,
                                      THREAD_TIME_CONSTRAINT_POLICY_COUNT) == KERN_SUCCESS;

           #else
            ignoreUnused (periodMs);
            struct sched_param param;
            param.sched_priority = sched_get_priority_max (SCHED_RR);
            return pthread_setschedparam (thread, SCHED_RR, &param) == 0;
           #endif
        */
    }
}

pub struct HighResolutionTimerClock {
    time: Instant,
    delta: Duration,
}

impl HighResolutionTimerClock {

    pub fn new(millis: u64) -> Self {
        Self {
            time: Instant::now(),
            delta: Duration::from_millis(millis),
        }
    }

    pub fn wait(&mut self, cond: &Condvar, lock: &MutexGuard<RawMutex>) -> bool {

        todo!();

        /*
            return cond.wait_until (lock, time) != std::cv_status::timeout;
            */
    }

    pub fn next(&mut self) {
        self.time += self.delta;
    }
}
