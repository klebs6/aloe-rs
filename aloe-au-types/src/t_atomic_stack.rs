crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAtomicStack.h]

/**
  | linked list LIFO or FIFO (pop_all_reversed)
  | stack, elements are pushed and popped
  | atomically class T must implement T
  | *& next().
  |
  */
pub struct TAtomicStack<T> {
    head: *mut T,
}

impl<T> Default for TAtomicStack<T> {
    
    fn default() -> Self {
        todo!();
        /*
        : head(NULL),

        
        */
    }
}

impl<T> TAtomicStack<T> {

    /**
      | non-atomic routines, for use when
      | initializing/deinitializing, operate
      | NON-atomically
      */
    pub fn push_na(&mut self, item: *mut T)  {
        
        todo!();
        /*
            item->next() = mHead;
            mHead = item;
        */
    }
    
    pub fn pop_na(&mut self) -> *mut T {
        
        todo!();
        /*
            T *result = mHead;
            if (result)
                mHead = result->next();
            return result;
        */
    }
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return mHead == NULL;
        */
    }
    
    pub fn head(&mut self) -> *mut T {
        
        todo!();
        /*
            return mHead;
        */
    }

    /**
       atomic routines
      */
    pub fn push_atomic(&mut self, item: *mut T)  {
        
        todo!();
        /*
            T *head_;
            do {
                head_ = mHead;
                item->next() = head_;
            } while (!compare_and_swap(head_, item, &mHead));
        */
    }

    /**
       pushes entire linked list headed by item
      */
    pub fn push_multiple_atomic(&mut self, item: *mut T)  {
        
        todo!();
        /*
            T *head_, *p = item, *tail;
            // find the last one -- when done, it will be linked to head
            do {
                tail = p;
                p = p->next();
            } while (p);
            do {
                head_ = mHead;
                tail->next() = head_;
            } while (!compare_and_swap(head_, item, &mHead));
        */
    }

    /**
      | this may only be used when only one
      | thread may potentially pop from the
      | stack.  if multiple threads may pop,
      | this suffers from the ABA problem.
      | <rdar://problem/4606346> TAtomicStack
      | suffers from the ABA problem
      */
    pub fn pop_atomic_single_reader(&mut self) -> *mut T {
        
        todo!();
        /*
            T *result;
            do {
                if ((result = mHead) == NULL)
                    break;
            } while (!compare_and_swap(result, result->next(), &mHead));
            return result;
        */
    }

    /**
      | This is inefficient for large linked
      | lists.  prefer pop_all() to a series of
      | calls to pop_atomic.
      | push_multiple_atomic has to traverse
      | the entire list.
      */
    pub fn pop_atomic(&mut self) -> *mut T {
        
        todo!();
        /*
            T *result = pop_all();
            if (result) {
                T *next = result->next();
                if (next)
                    // push all the remaining items back onto the stack
                    push_multiple_atomic(next);
            }
            return result;
        */
    }
    
    pub fn pop_all(&mut self) -> *mut T {
        
        todo!();
        /*
            T *result;
            do {
                if ((result = mHead) == NULL)
                    break;
            } while (!compare_and_swap(result, NULL, &mHead));
            return result;
        */
    }
    
    pub fn pop_all_reversed(&mut self) -> *mut T {
        
        todo!();
        /*
            TAtomicStack<T> reversed;
            T *p = pop_all(), *next;
            while (p != NULL) {
                next = p->next();
                reversed.push_NA(p);
                p = next;
            }
            return reversed.mHead;
        */
    }
    
    pub fn compare_and_swap(
        oldvalue: *mut T,
        newvalue: *mut T,
        pvalue:   *mut *mut T) -> bool {
        
        todo!();
        /*
            #if TARGET_OS_MAC
        #if __LP64__
                return ::OSAtomicCompareAndSwap64Barrier(int64_t(oldvalue), int64_t(newvalue), (int64_t *)pvalue);
        #elif MAC_OS_X_VERSION_MAX_ALLOWED >= MAC_OS_X_VERSION_10_4
                return ::OSAtomicCompareAndSwap32Barrier(int32_t(oldvalue), int32_t(newvalue), (int32_t *)pvalue);
        #else
                return ::CompareAndSwap(UInt32(oldvalue), UInt32(newvalue), (UInt32 *)pvalue);
        #endif
    #else
                //return ::CompareAndSwap(UInt32(oldvalue), UInt32(newvalue), (UInt32 *)pvalue);
                return CAAtomicCompareAndSwap32Barrier(SInt32(oldvalue), SInt32(newvalue), (SInt32*)pvalue);
    #endif
        */
    }
}
