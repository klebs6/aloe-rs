crate::ix!();

pub struct ThreadSafeListIterator<T> {
    node: *mut ThreadSafeListNode<T>,
}

impl<T> Default for ThreadSafeListIterator<T> {
    fn default() -> Self {
        todo!();
    }
}

impl<T> PartialEq<ThreadSafeListIterator<T>> for ThreadSafeListIterator<T> {
    
    #[inline] fn eq(&self, other: &ThreadSafeListIterator<T>) -> bool {
        todo!();
        /*
            return this->mNode == other.mNode;
        */
    }
}

impl<T> Eq for ThreadSafeListIterator<T> {}

impl<T> Deref for ThreadSafeListIterator<T> {

    type Target = T;

    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return mNode->mObject;
        */
    }
}

impl<T> ThreadSafeListIterator<T> {

    pub fn new(n: *mut ThreadSafeListNode<T>) -> Self {
    
        todo!();
        /*
        : node(n),

        
        */
    }
    
    pub fn prefix_increment(&mut self) -> &mut ThreadSafeListIterator<T> {
        
        todo!();
        /*
            mNode = mNode->next(); return *this;
        */
    }
    
    pub fn postfix_increment(&mut self, _0: i32) -> ThreadSafeListIterator<T> {
        
        todo!();
        /*
            iterator tmp = *this; mNode = mNode->next(); return tmp;
        */
    }
}
