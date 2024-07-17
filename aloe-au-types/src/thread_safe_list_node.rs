crate::ix!();

pub enum ThreadSafeListEventType { 
    Add, 
    Remove, 
    Clear 
}

pub struct ThreadSafeListNode<T> {
    next:       *mut ThreadSafeListNode<T>,
    event_type: ThreadSafeListEventType,
    object:     T,
}

impl<T> ThreadSafeListNode<T> {

    pub fn next(&mut self) -> &mut *mut ThreadSafeListNode<T> {
        
        todo!();
        /*
            return mNext;
        */
    }
}
