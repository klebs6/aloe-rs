crate::ix!();

/* ------------------- CAMutexUnlocker  ------------------- */
pub struct CAMutexUnlocker<'a> {
    mutex:      &'a mut CAMutex,
    needs_lock: bool,
}

impl<'a> Drop for CAMutexUnlocker<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if(mNeedsLock)
        {
            mMutex.Lock();
        }
        */
    }
}

impl<'a> CAMutexUnlocker<'a> {

    pub fn new(in_mutex: &mut CAMutex) -> Self {
    
        todo!();
        /*


            :   mMutex(inMutex),
        mNeedsLock(false)
        Assert(mMutex.IsOwnedByCurrentThread(), "Major problem: CAMutexUnlocker attempted to unlock a mutex not owned by the current thread!");

        mMutex.Unlock();
        mNeedsLock = true;
        */
    }
}
