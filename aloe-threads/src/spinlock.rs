crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_SpinLock.h]

/**
  | A simple spin-lock class that can be
  | used as a simple, low-overhead mutex
  | for uncontended situations.
  | 
  | Note that unlike a CriticalSection,
  | this type of lock is not re-entrant,
  | and may be less efficient when used in
  | a highly contended situation, but it's
  | very small and requires almost no initialisation.
  | 
  | It's most appropriate for simple situations
  | where you're only going to hold the lock
  | for a very brief time.
  | 
  | @see CriticalSection
  | 
  | @tags{Core}
  |
  */
#[derive(Default)]
#[no_copy]
pub struct SpinLock {
    lock: RefCell<Atomic<i32>>,
}

pub mod spin_lock {
    use super::*;

    /**
      | Provides the type of scoped lock to use
      | for locking a SpinLock.
      |
      */
    pub type ScopedLockType<'a> = GenericScopedLock<'a, SpinLock>;

    /**
      | Provides the type of scoped unlocker
      | to use with a SpinLock.
      |
      */
    pub type ScopedUnlockType<'a> = GenericScopedUnlock<'a, SpinLock>;

    /**
      | Provides the type of scoped try-lock
      | to use for locking a SpinLock.
      |
      */
    pub type ScopedTryLockType<'a> = GenericScopedTryLock<'a, SpinLock>;
}

impl SpinLock {

    /**
      | Acquires the lock.
      | 
      | This will block until the lock has been
      | successfully acquired by this thread.
      | 
      | -----------
      | @note
      | 
      | a SpinLock is NOT re-entrant, and is
      | not smart enough to know whether the
      | caller thread already has the lock -
      | so if a thread tries to acquire a lock
      | that it already holds, this method will
      | never return!
      | 
      | It's strongly recommended that you
      | never call this method directly - instead
      | use the ScopedLockType class to manage
      | the locking using an RAII pattern instead.
      |
      */
    pub fn enter(&self)  {
        
        todo!();
        /*
            if (! tryEnter())
        {
            for (int i = 20; --i >= 0;)
                if (tryEnter())
                    return;

            while (! tryEnter())
                Thread::yield();
        }
        */
    }

    /**
      | Attempts to acquire the lock, returning
      | true if this was successful.
      |
      */
    #[inline] pub fn try_enter(&self) -> bool {
        
        todo!();
        /*
            return lock.compareAndSetBool (1, 0);
        */
    }

    /**
      | Releases the lock.
      |
      */
    #[inline] pub fn exit(&self)  {
        
        todo!();
        /*
            jassert (lock.get() == 1); // Agh! Releasing a lock that isn't currently held!
            lock = 0;
        */
    }
}
