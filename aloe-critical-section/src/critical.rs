crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_CriticalSection.h]

/**
  | A re-entrant mutex.
  | 
  | A CriticalSection acts as a re-entrant
  | mutex object. The best way to lock and
  | unlock one of these is by using RAII in
  | the form of a local ScopedLock object
  | - have a look through the codebase for
  | many examples of how to do this.
  | 
  | In almost all cases you'll want to declare
  | your CriticalSection as a member variable.
  | 
  | Occasionally you may want to declare
  | one as a static variable, but in that
  | case the usual C++ static object order-of-construction
  | warnings should be heeded.
  | 
  | @see ScopedLock, ScopedTryLock, ScopedUnlock,
  | SpinLock, ReadWriteLock, Thread,
  | InterProcessLock
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct CriticalSection {

    /**
      | To avoid including windows.h in the public
      | Aloe headers, we'll just allocate a block
      | of memory here that's big enough to be used
      | internally as a windows CRITICAL_SECTION
      | structure.
      */
    #[cfg(target_os="windows")]
    #[cfg(ALOE_64BIT)]
    lock: AlignedStorage<44, 8>,

    #[cfg(target_os="windows")]
    #[cfg(not(ALOE_64BIT))]
    lock: AlignedStorage<24, 8>,

    #[cfg(not(target_os="windows"))]
    lock: RefCell<libc::pthread_mutex_t>,
}

pub mod critical_section {

    use super::*;

    /**
      | Provides the type of scoped lock to use
      | with a CriticalSection.
      |
      */
    pub type ScopedLockType<'a> = GenericScopedLock<'a, CriticalSection>;

    /**
      | Provides the type of scoped unlocker
      | to use with a CriticalSection.
      |
      */
    pub type ScopedUnlockType<'a> = GenericScopedUnlock<'a, CriticalSection>;

    /**
      | Provides the type of scoped try-locker
      | to use with a CriticalSection.
      |
      */
    pub type ScopedTryLockType<'a> = GenericScopedTryLock<'a, CriticalSection>;
}

impl Default for CriticalSection {
    
    /**
      | Creates a CriticalSection object.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Drop for CriticalSection {

    /**
      | Destructor.
      | 
      | If the critical section is deleted whilst
      | locked, any subsequent behaviour is
      | unpredictable.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

impl CriticalSection {

    /**
      | Acquires the lock.
      | 
      | If the lock is already held by the caller
      | thread, the method returns immediately.
      | 
      | If the lock is currently held by another
      | thread, this will wait until it becomes
      | free.
      | 
      | It's strongly recommended that you
      | never call this method directly - instead
      | use the ScopedLock class to manage the
      | locking using an RAII pattern instead.
      | 
      | @see exit, tryEnter, ScopedLock
      |
      */
    pub fn enter(&self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Attempts to lock this critical section
      | without blocking.
      | 
      | This method behaves identically to
      | CriticalSection::enter, except that
      | the caller thread does not wait if the
      | lock is currently held by another thread
      | but returns false immediately.
      | 
      | -----------
      | @return
      | 
      | false if the lock is currently held by
      | another thread, true otherwise. @see
      | enter
      |
      */
    pub fn try_enter(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Releases the lock.
      | 
      | If the caller thread hasn't got the lock,
      | this can have unpredictable results.
      | 
      | If the enter() method has been called
      | multiple times by the thread, each call
      | must be matched by a call to exit() before
      | other threads will be allowed to take
      | over the lock.
      | 
      | @see enter, ScopedLock
      |
      */
    pub fn exit(&self)  {
        
        todo!();
        /*
        
        */
    }
}

/**
  | A class that can be used in place of a real
  | CriticalSection object, but which
  | doesn't perform any locking.
  | 
  | This is currently used by some templated
  | classes, and most compilers should
  | manage to optimise it out of existence.
  | 
  | @see CriticalSection, Array, OwnedArray,
  | ReferenceCountedArray
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[derive(Default)]
pub struct DummyCriticalSection {

}

pub mod dummy_critical_section {
    use super::*;

    /**
      | A dummy scoped-lock type to use with
      | a dummy critical section.
      |
      */
    pub struct ScopedLockType { }

    impl ScopedLockType {
        
        pub fn new(_0: &DummyCriticalSection) -> Self {
        
            todo!();
            /*


            
            */
        }
    }

    /**
      | A dummy scoped-unlocker type to use
      | with a dummy critical section.
      |
      */
    pub type ScopedUnlockType = ScopedLockType;
}

impl HasScopedLockType for DummyCriticalSection {

    type ScopedLockType = dummy_critical_section::ScopedLockType;
}

impl HasScopedUnlockType for DummyCriticalSection {

    type ScopedUnlockType = dummy_critical_section::ScopedUnlockType;
}

impl DummyCriticalSection {

    #[inline] pub fn enter(&self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[inline] pub fn try_enter(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    #[inline] pub fn exit(&self)  {
        
        todo!();
        /*
        
        */
    }
}

/**
  | Automatically locks and unlocks
  | a CriticalSection object.
  |
  | You can use a ScopedLock as a local variable
  | to provide RAII-based locking of
  | a CriticalSection.
  |
  | e.g. @code
  |
  | struct MyObject
  | {
  |     CriticalSection objectLock;
  |
  |     // assuming that this example function
  |     // will be called by multiple threads
  |     void foo()
  |     {
  |         const ScopedLock myScopedLock (objectLock);
  |
  |         // objectLock is now locked..
  |
  |         ...do some thread-safe work here...
  |
  |         // ..and objectLock gets unlocked
  |         // here, as myScopedLock goes out of
  |         // scope at the end of the block
  |     }
  | };
  | @endcode
  |
  | @see CriticalSection, ScopedUnlock
  */
pub type ScopedLock<'a> = critical_section::ScopedLockType<'a>;

/**
  | Automatically unlocks and re-locks
  | a CriticalSection object.
  |
  | This is the reverse of a ScopedLock object
  | - instead of locking the critical section for
  | the lifetime of this object, it unlocks it.
  |
  | Make sure you don't try to unlock critical
  | sections that aren't actually locked!
  |
  | e.g. @code
  |
  | struct MyObject
  | {
  |     CriticalSection objectLock;
  |
  |     void foo()
  |     {
  |         {
  |             const ScopedLock myScopedLock (objectLock);
  |
  |             // objectLock is now locked..
  |
  |             {
  |                 ScopedUnlock myUnlocker (objectLock);
  |
  |                 // ..and now unlocked..
  |             }
  |
  |             // ..and now locked again..
  |         }
  |
  |         // ..and finally unlocked.
  |     }
  | };
  | @endcode
  |
  | @see CriticalSection, ScopedLock
  */
pub type ScopedUnlock<'a> = critical_section::ScopedUnlockType<'a>;

/**
  | Automatically tries to lock and unlock
  | a CriticalSection object.
  |
  | Use one of these as a local variable to
  | control access to a CriticalSection.
  |
  | e.g. @code
  |
  | struct MyObject
  | {
  |   CriticalSection objectLock;
  |
  |   void foo()
  |   {
  |       const ScopedTryLock myScopedTryLock (objectLock);
  |
  |       /**
  |         | Unlike using a ScopedLock, this may
  |         | fail to actually get the lock, so
  |         | you must call the isLocked() method
  |         | before making any assumptions..
  |         */
  |       if (myScopedTryLock.isLocked())
  |       {
  |          ...safely do some work...
  |       }
  |       else
  |       {
  |           // If we get here, then our
  |           // attempt at locking failed
  |           // because another thread had
  |           // already locked it..
  |       }
  |   }
  | };
  | @endcode
  |
  | @see CriticalSection::tryEnter, ScopedLock,
  | ScopedUnlock, ScopedReadLock
  */
pub type ScopedTryLock<'a> = critical_section::ScopedTryLockType<'a>;

#[cfg(feature = "aloe_posix")]
impl Drop for CriticalSection {

    fn drop(&mut self) {
        todo!();
        /*      pthread_mutex_destroy (&lock);  */
    }
}

impl CriticalSection {
    
    #[cfg(feature = "aloe_posix")]
    pub fn new() -> Self {
    
        todo!();
        /*


            pthread_mutexattr_t atts;
        pthread_mutexattr_init (&atts);
        pthread_mutexattr_settype (&atts, PTHREAD_MUTEX_RECURSIVE);
       #if ! ALOE_ANDROID
        pthread_mutexattr_setprotocol (&atts, PTHREAD_PRIO_INHERIT);
       #endif
        pthread_mutex_init (&lock, &atts);
        pthread_mutexattr_destroy (&atts);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn enter(&self)  {
        
        todo!();
        /*
            pthread_mutex_lock (&lock);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn try_enter(&self) -> bool {
        
        todo!();
        /*
            return pthread_mutex_trylock (&lock) == 0;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn exit(&self)  {
        
        todo!();
        /*
            pthread_mutex_unlock (&lock);
        */
    }
}
