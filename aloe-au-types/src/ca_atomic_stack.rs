crate::ix!();

#[cfg(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32)))]
pub struct CAAtomicStack {
    head:            OSQueueHead,
    next_ptr_offset: usize,
}

#[cfg(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32)))]
impl CAAtomicStack {

    pub fn new(next_ptr_offset: usize) -> Self {
    
        todo!();
        /*
        : next_ptr_offset(nextPtrOffset),

            /*OSQueueHead h = OS_ATOMIC_QUEUE_INIT; mHead = h;*/
            mHead.opaque1 = 0; mHead.opaque2 = 0;
        */
    }

    /**
       a subset of the above
      */
    pub fn push_atomic(&mut self, p: *mut c_void)  {
        
        todo!();
        /*
            OSAtomicEnqueue(&mHead, p, mNextPtrOffset);
        */
    }
    
    pub fn push_na(&mut self, p: *mut c_void)  {
        
        todo!();
        /*
            push_atomic(p);
        */
    }
    
    pub fn pop_atomic(&mut self)  {
        
        todo!();
        /*
            return OSAtomicDequeue(&mHead, mNextPtrOffset);
        */
    }
    
    pub fn pop_atomic_single_reader(&mut self)  {
        
        todo!();
        /*
            return pop_atomic();
        */
    }
    
    pub fn pop_na(&mut self)  {
        
        todo!();
        /*
            return pop_atomic();
        */
    }
}
