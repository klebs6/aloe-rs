crate::ix!();

/**
  | Conditional Guard - Locks only if valid
  | lock is passed. @ingroup baseLocks
  |
  */
pub struct FConditionalGuard {

    /**
      | guarded lock
      |
      */
    lock: *mut FLock,
}

impl Drop for FConditionalGuard {

    /**
      | FConditionGuard destructor.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            if (lock)
                lock->unlock ();
        */
    }
}

impl FConditionalGuard {

    /**
      | FConditionGuard constructor.
      | 
      | -----------
      | @param _lock
      | 
      | guard this lock
      |
      */
    pub fn new(lock: *mut FLock) -> Self {
    
        todo!();
        /*
        : lock(_lock),

            if (lock)
                lock->lock ();
        */
    }
}
