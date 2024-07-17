crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ScopedReadLock.h]

/**
  | Automatically locks and unlocks
  | a ReadWriteLock object.
  |
  | Use one of these as a local variable to
  | control access to a ReadWriteLock.
  |
  | e.g. @code
  |
  | ReadWriteLock myLock;
  |
  | for (;;)
  | {
  |     const ScopedReadLock myScopedLock (myLock);
  |     // myLock is now locked
  |
  |     ...do some stuff...
  |
  |     // myLock gets unlocked here.
  | }
  | @endcode
  |
  | @see ReadWriteLock, ScopedWriteLock
  |
  | @tags{Core}
  */
#[no_copy]
pub struct ScopedReadLock<'a> {
    lock: &'a ReadWriteLock,
}

impl<'a> Drop for ScopedReadLock<'a> {

    /**
      | Destructor.
      | 
      | The ReadWriteLock's exitRead() method
      | will be called when the destructor is
      | called.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      lock_.exitRead();  */
    }
}

impl<'a> ScopedReadLock<'a> {

    /**
      | Creates a ScopedReadLock.
      | 
      | As soon as it is created, this will call
      | ReadWriteLock::enterRead(), and
      | when the ScopedReadLock object is deleted,
      | the ReadWriteLock will be unlocked.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      | Best just to use it as a local stack object,
      | rather than creating one with the new()
      | operator.
      |
      */
    pub fn new(lock: &ReadWriteLock) -> Self {
    
        todo!();
        /*
        : lock(lock),

            lock.enterRead();
        */
    }
}
