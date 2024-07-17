crate::ix!();

/**
  | Automatically locks and unlocks an
  | InterProcessLock object.
  | 
  | This works like a ScopedLock, but using
  | an InterprocessLock rather than a CriticalSection.
  | 
  | @see ScopedLock
  |
  */
#[no_copy]
pub struct ScopedLockType<'a> {
    ip_lock:             &'a mut InterProcessLock,
    lock_was_successful: bool,
}

impl<'a> Drop for ScopedLockType<'a> {

    /**
      | Destructor.
      | 
      | The InterProcessLock will be unlocked
      | when the destructor is called.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    #[inline] fn drop(&mut self) {
        todo!();
        /*      ipLock.exit();  */
    }
}

impl<'a> ScopedLockType<'a> {

    /**
      | Creates a scoped lock.
      | 
      | As soon as it is created, this will lock
      | the InterProcessLock, and when the
      | ScopedLockType object is deleted,
      | the InterProcessLock will be unlocked.
      | 
      | -----------
      | @note
      | 
      | since an InterprocessLock can fail
      | due to errors, you should check isLocked()
      | to make sure that the lock was successful
      | before using it.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      | Best just to use it as a local stack object,
      | rather than creating one with the new()
      | operator.
      |
      */
    pub fn new(l: &mut InterProcessLock) -> Self {
    
        todo!();
        /*


            : ipLock (l) 
        lockWasSuccessful = l.enter();
        */
    }

    /**
      | Returns true if the InterProcessLock
      | was successfully locked.
      |
      */
    pub fn is_locked(&self) -> bool {
        
        todo!();
        /*
            return lockWasSuccessful;
        */
    }
}

