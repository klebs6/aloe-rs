crate::ix!();

///--------------------------
pub struct TryLockedPtr<Element> {
    ptr:   Box<Element>,
    mutex: SpinLock,
}

impl<Element> TryLockedPtr<Element> {

    pub fn set(&mut self, p: Box<Element>)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType lock (mutex);
            ptr = std::move (p);
        */
    }
    
    pub fn get(&mut self) -> Box<MultichannelEngine> {
        
        todo!();
        /*
            const SpinLock::ScopedTryLockType lock (mutex);
            return lock.isLocked() ? std::move (ptr) : nullptr;
        */
    }
}
