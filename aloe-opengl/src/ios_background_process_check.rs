crate::ix!();

/**
  | On iOS, all GL calls will crash when the app is
  | running in the background, so this prevents
  | them from happening (which some messy locking
  | behaviour)
  */
#[cfg(target_os="ios")]
#[no_copy]
#[leak_detector]
pub struct iOSBackgroundProcessCheck {
    base:          AppInactivityCallback,
    is_foreground: Atomic<i32>,
}

#[cfg(target_os="ios")]
impl Default for iOSBackgroundProcessCheck {
    
    fn default() -> Self {
        todo!();
        /*


            isBackgroundProcess(); appBecomingInactiveCallbacks.add (this);
        */
    }
}

#[cfg(target_os="ios")]
impl Drop for iOSBackgroundProcessCheck {

    fn drop(&mut self) {
        todo!();
        /*
            appBecomingInactiveCallbacks.removeAllInstancesOf (this);
        */
    }
}

#[cfg(target_os="ios")]
impl iOSBackgroundProcessCheck {
    
    pub fn is_background_process(&mut self) -> bool {
        
        todo!();
        /*
            const bool b = Process::isForegroundProcess();
            isForeground.set (b ? 1 : 0);
            return ! b;
        */
    }
    
    pub fn app_becoming_inactive(&mut self)  {
        
        todo!();
        /*
            int counter = 2000;

            while (--counter > 0 && isForeground.get() != 0)
                Thread::sleep (1);
        */
    }
}
