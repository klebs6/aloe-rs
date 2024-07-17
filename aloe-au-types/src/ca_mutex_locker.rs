crate::ix!();

/**
  | Helper class to manage taking and releasing
  | recursively
  |
  */
pub struct CAMutexLocker {
    mutex:         *mut CAMutex,
    needs_release: bool,
}

impl Drop for CAMutexLocker {

    /**
      | in this case the mutex can be null
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            if(mNeedsRelease) { mMutex->Unlock(); }
        */
    }
}

impl CAMutexLocker {
    
    pub fn new(in_mutex: &mut CAMutex) -> Self {
    
        todo!();
        /*
        : mutex(&inMutex),
        : needs_release(false),

            mNeedsRelease = mMutex->Lock();
        */
    }
    
    pub fn new_from_raw_ca_mutex(in_mutex: *mut CAMutex) -> Self {
    
        todo!();
        /*
        : mutex(inMutex),
        : needs_release(false),

            mNeedsRelease = (mMutex != NULL && mMutex->Lock());
        */
    }
    
    pub fn new_from_ca_mutex_locker(_0: &CAMutexLocker) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    pub fn assign_from(&mut self, _0: &CAMutexLocker) -> &mut CAMutexLocker {
        
        todo!();
        /*
        
        */
    }
}
