crate::ix!();

#[cfg(SMTG_OS_LINUX)] 
pub struct RunLoopTimerCaller {
    base:    Timer,
    handler: *mut ITimerHandler, // default = nullptr
}

#[cfg(SMTG_OS_LINUX)] 
impl Drop for RunLoopTimerCaller {
    fn drop(&mut self) {
        todo!();
        /*
            stopTimer();
        */
    }
}

#[cfg(SMTG_OS_LINUX)] 
impl PartialEq<ITimerHandler> for RunLoopTimerCaller {
    
    #[inline] fn eq(&self, other: &ITimerHandler) -> bool {
        todo!();
        /*
            return handler == other;
        */
    }
}

#[cfg(SMTG_OS_LINUX)] 
impl RunLoopTimerCaller {

    pub fn new(
        h:        *mut ITimerHandler,
        interval: i32

    ) -> Self {
    
        todo!();
        /*
        : handler(h),

            startTimer (interval);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            handler->onTimer();
        */
    }
}

