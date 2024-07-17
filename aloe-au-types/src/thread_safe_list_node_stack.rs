crate::ix!();

pub struct ThreadSafeListNodeStack<T> {
    base: TAtomicStack<ThreadSafeListNode<T>>,
}

impl<T> ThreadSafeListNodeStack<T> {

    pub fn free_all(&mut self)  {
        
        todo!();
        /*
            ThreadSafeListNode *node;
                while ((node = this->pop_NA()) != NULL)
                    free(node);
        */
    }
    
    pub fn phead(&mut self) -> *mut *mut ThreadSafeListNode<T> {
        
        todo!();
        /*
            return &this->mHead;
        */
    }
    
    pub fn head(&self) -> *mut ThreadSafeListNode<T> {
        
        todo!();
        /*
            return this->mHead;
        */
    }
}
