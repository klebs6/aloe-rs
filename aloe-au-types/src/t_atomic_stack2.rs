crate::ix!();

/**
  | a more efficient subset of TAtomicStack
  | using OSQueue.
  |
  */
#[cfg(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32)))]
pub struct TAtomicStack2<T> {

    /*
      | caution: do not try to implement
      | pop_all_reversed here. the writer could add
      | new elements while the reader is trying to
      | pop old ones!
      */
    head:            OSQueueHead,
    next_ptr_offset: isize,
}

#[cfg(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32)))]
impl Default for TAtomicStack2 {
    
    fn default() -> Self {
        todo!();
        /*


            /*OSQueueHead h = OS_ATOMIC_QUEUE_INIT; mHead = h;*/
            mHead.opaque1 = 0; mHead.opaque2 = 0;
            mNextPtrOffset = -1;
        */
    }
}

#[cfg(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32)))]
impl TAtomicStack2<T> {

    pub fn push_atomic(&mut self, item: *mut T)  {
        
        todo!();
        /*
            if (mNextPtrOffset < 0) {
                T **pnext = &item->next();  // hack around offsetof not working with C++
                mNextPtrOffset = (Byte *)pnext - (Byte *)item;
            }
            OSAtomicEnqueue(&mHead, item, mNextPtrOffset);
        */
    }
    
    pub fn push_na(&mut self, item: *mut T)  {
        
        todo!();
        /*
            push_atomic(item);
        */
    }
    
    pub fn pop_atomic(&mut self) -> *mut T {
        
        todo!();
        /*
            return (T *)OSAtomicDequeue(&mHead, mNextPtrOffset);
        */
    }
    
    pub fn pop_atomic_single_reader(&mut self) -> *mut T {
        
        todo!();
        /*
            return pop_atomic();
        */
    }
    
    pub fn pop_na(&mut self) -> *mut T {
        
        todo!();
        /*
            return pop_atomic();
        */
    }
}

#[cfg(not(all(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_5,not(TARGET_OS_WIN32))))]
macro_rules! tatomicstack2 {
    () => {
        /*
                TAtomicStack
        */
    }
}
