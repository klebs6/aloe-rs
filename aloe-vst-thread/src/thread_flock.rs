crate::ix!();

/**
  | FLock declaration. @ingroup baseLocks
  |
  */
pub struct FLock {

    /**
      | Mutex object
      |
      */
    #[cfg(SMTG_PTHREADS)]
    mutex: PThreadMutex,

    /**
      | Critical section object
      |
      */
    #[cfg(SMTG_OS_WINDOWS)]
    section: CRITSECT,
}

impl Drop for FLock {

    fn drop(&mut self) {
        todo!();
        /*
            #if SMTG_PTHREADS
        pthread_mutex_destroy (&mutex);

    #elif SMTG_OS_WINDOWS
        DeleteCriticalSection ((LPCRITICAL_SECTION)&section);
            
    #endif
        */
    }
}

impl FLock {

    /**
      | Lock constructor.
      | 
      | -----------
      | @param name
      | 
      | lock name
      |
      */
    pub fn new(name: Option<&str>) -> Self {

        let name = name.unwrap_or("FLock");
    
        todo!();
        /*


            #if SMTG_PTHREADS
        pthread_mutexattr_t mutexAttr;
        pthread_mutexattr_init (&mutexAttr);
        pthread_mutexattr_settype (&mutexAttr, PTHREAD_MUTEX_RECURSIVE);
        if (pthread_mutex_init (&mutex, &mutexAttr) != 0)
            {SMTG_WARNING("mutex_init failed")}
        pthread_mutexattr_destroy (&mutexAttr);

    #elif SMTG_OS_WINDOWS
        INIT_CS (section)
    #else
    #warning implement FLock!
    #endif
        */
    }
}

impl ILock for FLock {

    #[SMTG_OVERRIDE]
    fn lock(&mut self)  {
        
        todo!();
        /*
            #if DEBUG_LOCK
        FDebugPrint ("FLock::lock () %x\n", this);
    #endif

    #if SMTG_PTHREADS
        pthread_mutex_lock (&mutex);

    #elif SMTG_OS_WINDOWS
        EnterCriticalSection ((LPCRITICAL_SECTION)&section);

    #endif
        */
    }
    
    #[SMTG_OVERRIDE]
    fn unlock(&mut self)  {
        
        todo!();
        /*
            #if DEBUG_LOCK
        FDebugPrint ("FLock::unlock () %x\n", this);
    #endif
        
    #if SMTG_PTHREADS
        pthread_mutex_unlock (&mutex);

    #elif SMTG_OS_WINDOWS
        LeaveCriticalSection ((LPCRITICAL_SECTION)&section);

    #endif
        */
    }

    #[SMTG_OVERRIDE]
    fn trylock(&mut self) -> bool {
        
        todo!();
        /*
            #if SMTG_PTHREADS
        return pthread_mutex_trylock (&mutex) == 0;

    #elif SMTG_OS_WINDOWS
        return TryEnterCriticalSection ((LPCRITICAL_SECTION)&section) != 0 ? true : false;

    #else
        return false;
    #endif
        */
    }
}
