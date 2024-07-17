crate::ix!();

pub struct ComponentInitLocker {

    #[cfg(TARGET_OS_WIN32)]
    needs_unlocking: bool,

    previous_new_instance_type: ComponentBaseEInstanceType,
}

impl ComponentInitLocker {

    #[cfg(target_os="macos")]
    pub fn init_component_init_locker(&mut self)  {
        
        todo!();
        /*
            // have to do this because OS X lacks PTHREAD_MUTEX_RECURSIVE_INITIALIZER_NP
        pthread_mutexattr_t attr;
        pthread_mutexattr_init(&attr);
        pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);
        pthread_mutex_init(&sComponentOpenMutex, &attr);
        pthread_mutexattr_destroy(&attr);
        */
    }
}

impl Default for ComponentInitLocker {

    #[cfg(target_os="macos")]
    fn default() -> Self {
        todo!();
        /*


            pthread_once(&sOnce, InitComponentInitLocker);
            pthread_mutex_lock(&sComponentOpenMutex);
            mPreviousNewInstanceType = ComponentBase::sNewInstanceType;
        */
    }
    
    #[cfg(TARGET_OS_WIN32)]
    fn default() -> Self {
        todo!();
        /*


            sNeedsUnlocking = sComponentOpenGuard.Lock();
        */
    }
}

impl Drop for ComponentInitLocker {

    #[cfg(target_os="macos")]
    fn drop(&mut self) {
        todo!();
        /*
            ComponentBase::sNewInstanceType = mPreviousNewInstanceType;
            pthread_mutex_unlock(&sComponentOpenMutex);
        */
    }

    #[cfg(TARGET_OS_WIN32)]
    fn drop(&mut self) {
        todo!();
        /*
            if(sNeedsUnlocking) { sComponentOpenGuard.Unlock(); }
        */
    }
}

pub mod component_init_locker_ {
    use super::*;

    #[cfg(target_os="macos")]
    lazy_static!{
        /*
        pthread_mutex_t ComponentInitLocker::sComponentOpenMutex = PTHREAD_MUTEX_INITIALIZER;
        pthread_once_t ComponentInitLocker::sOnce = PTHREAD_ONCE_INIT;
        */
    }

    #[cfg(TARGET_OS_WIN32)]
    lazy_static!{
        /*
        CAGuard ComponentInitLocker::sComponentOpenGuard("sComponentOpenGuard");
        */
    }

    #[cfg(target_os="macos")]
    lazy_static!{
        /*
        static pthread_mutex_t sComponentOpenMutex;
            static pthread_once_t sOnce;
        */
    }

    ///-----------------------
    #[cfg(TARGET_OS_WIN32)]
    lazy_static!{
        /*
        static CAGuard  sComponentOpenGuard;
        */
    }
}
