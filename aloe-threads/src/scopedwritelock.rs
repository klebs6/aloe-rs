crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ScopedWriteLock.h]

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
  |     const ScopedWriteLock myScopedLock (myLock);
  |     // myLock is now locked
  |
  |     ...do some stuff...
  |
  |     // myLock gets unlocked here.
  | }
  | @endcode
  |
  | @see ReadWriteLock, ScopedReadLock
  |
  | @tags{Core}
  */
#[no_copy]
pub struct ScopedWriteLock<'a> {
    lock: &'a ReadWriteLock,
}

impl<'a> Drop for ScopedWriteLock<'a> {

    /**
      | Destructor.
      | 
      | The ReadWriteLock's exitWrite() method
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
        /*      lock_.exitWrite();  */
    }
}

impl<'a> ScopedWriteLock<'a> {

    /**
      | Creates a ScopedWriteLock.
      | 
      | As soon as it is created, this will call
      | ReadWriteLock::enterWrite(), and
      | when the ScopedWriteLock object is
      | deleted, the ReadWriteLock will be
      | unlocked.
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

            lock.enterWrite();
        */
    }
}
