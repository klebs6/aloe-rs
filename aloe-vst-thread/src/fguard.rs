crate::ix!();

/**
  | FGuard - automatic object for locks.
  | @ingroup baseLocks
  |
  */
pub struct FGuard<'a> {

    /**
      | guarded lock
      |
      */
    lock: &'a mut dyn ILock,
}

impl<'a> Drop for FGuard<'a> {

    /**
      | FGuard destructor.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            lock.unlock ();
        */
    }
}

impl<'a> FGuard<'a> {

    /**
      | FGuard constructor.
      | 
      | -----------
      | @param _lock
      | 
      | guard this lock
      |
      */
    pub fn new(lock: &mut dyn ILock) -> Self {
    
        todo!();
        /*
        : lock(_lock),

            lock.lock ();
        */
    }
}

