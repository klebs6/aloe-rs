crate::ix!();

/**
  | you can use this with Try - if you take the
  | lock in try, pass in the outWasLocked var
  */
pub struct CAMutexTryer<'a> {
    mutex:         &'a mut CAMutex,
    needs_release: bool,
    has_lock:      bool,
}

impl<'a> Drop for CAMutexTryer<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (mNeedsRelease) mMutex.Unlock();
        */
    }
}

impl<'a> CAMutexTryer<'a> {
    
    pub fn new_from_ca_mutex_ref(mutex: &mut CAMutex) -> Self {
    
        todo!();
        /*
        : mutex(mutex),
        : needs_release(false),
        : has_lock(false),

            mHasLock = mMutex.Try (mNeedsRelease);
        */
    }
    
    pub fn has_lock(&self) -> bool {
        
        todo!();
        /*
            return mHasLock;
        */
    }
    
    pub fn new(_0: &CAMutexTryer) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    pub fn assign_from(&mut self, _0: &CAMutexTryer) -> &mut CAMutexTryer {
        
        todo!();
        /*
        
        */
    }
}
