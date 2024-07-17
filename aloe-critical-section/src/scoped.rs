crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ScopedLock.h]

pub trait HasScopedLockType {
    type ScopedLockType;
}

pub trait HasScopedUnlockType {
    type ScopedUnlockType;
}

/**
  | Automatically locks and unlocks a mutex
  | object.
  |
  | Use one of these as a local variable to
  | provide RAII-based locking of a mutex.
  |
  | The templated class could be
  | a CriticalSection, SpinLock, or anything else
  | that provides enter() and exit() methods.
  |
  | e.g. @code
  |
  | CriticalSection myCriticalSection;
  |
  | for (;;)
  | {
  |     const GenericScopedLock<CriticalSection> myScopedLock (myCriticalSection);
  |     // myCriticalSection is now locked
  |
  |     ...do some stuff...
  |
  |     // myCriticalSection gets unlocked here.
  | }
  |
  | @endcode
  |
  | @see GenericScopedUnlock, CriticalSection,
  | SpinLock, ScopedLock, ScopedUnlock
  |
  | @tags{Core}
  */
#[no_copy]
pub struct GenericScopedLock<'a, LockType> {
    lock: &'a LockType,
}

impl<'a, LockType> Drop for GenericScopedLock<'a, LockType> {
    /**
      | Destructor.
      | 
      | The lock will be released when the destructor
      | is called.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      lock_.exit();  */
    }
}

impl<'a, LockType> GenericScopedLock<'a, LockType> {

    /**
      | Creates a GenericScopedLock.
      | 
      | As soon as it is created, this will acquire
      | the lock, and when the GenericScopedLock
      | object is deleted, the lock will be released.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      | Best just to use it as a local stack object,
      | rather than creating one with the new()
      | operator.
      |
      */
    pub fn new(lock: &LockType) -> Self {
    
        todo!();
        /*
        : lock(lock),

            lock.enter();
        */
    }
}

/**
  | Automatically unlocks and re-locks a mutex
  | object.
  |
  | This is the reverse of a GenericScopedLock
  | object - instead of locking the mutex for the
  | lifetime of this object, it unlocks it.
  |
  | Make sure you don't try to unlock mutexes that
  | aren't actually locked!
  |
  | e.g. @code
  |
  | CriticalSection myCriticalSection;
  |
  | for (;;)
  | {
  |     const GenericScopedLock<CriticalSection> myScopedLock (myCriticalSection);
  |
  |     // myCriticalSection is now locked
  |
  |     ... do some stuff with it locked ..
  |
  |     while (xyz)
  |     {
  |         ... do some stuff with it locked ..
  |
  |         const GenericScopedUnlock<CriticalSection> unlocker (myCriticalSection);
  |
  |         // myCriticalSection is now unlocked
  |         // for the remainder of this block,
  |         // and re-locked at the end.
  |
  |         ...do some stuff with it unlocked ...
  |     }
  |
  |     // myCriticalSection gets unlocked here.
  | }
  | @endcode
  |
  | @see GenericScopedLock, CriticalSection,
  | ScopedLock, ScopedUnlock
  |
  | @tags{Core}
  */
#[no_copy]
pub struct GenericScopedUnlock<'a, LockType> {
    lock: &'a LockType,
}

impl<'a, LockType> Drop for GenericScopedUnlock<'a, LockType> {

    /**
      | Destructor.
      | 
      | The CriticalSection will be unlocked
      | when the destructor is called.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      lock_.enter();  */
    }
}

impl<'a, LockType> GenericScopedUnlock<'a, LockType> {

    /**
      | Creates a GenericScopedUnlock.
      | 
      | As soon as it is created, this will unlock
      | the CriticalSection, and when the ScopedLock
      | object is deleted, the CriticalSection
      | will be re-locked.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      | Best just to use it as a local stack object,
      | rather than creating one with the new()
      | operator.
      |
      */
    pub fn new(lock: &LockType) -> Self {
    
        todo!();
        /*
        : lock(lock),

            lock.exit();
        */
    }
}

/**
  | Automatically locks and unlocks a mutex
  | object.
  |
  | Use one of these as a local variable to
  | provide RAII-based locking of a mutex.
  |
  | The templated class could be
  | a CriticalSection, SpinLock, or anything else
  | that provides enter() and exit() methods.
  |
  | e.g. @code
  |
  | CriticalSection myCriticalSection;
  |
  | for (;;)
  | {
  |     const
  |     GenericScopedTryLock<CriticalSection>
  |     myScopedTryLock (myCriticalSection);
  |
  |     // Unlike using a ScopedLock, this may
  |     // fail to actually get the lock, so you
  |     // should test this with the isLocked()
  |     // method before doing your thread-unsafe
  |     // action..
  |     //
  |     if (myScopedTryLock.isLocked())
  |     {
  |        ...do some stuff...
  |     }
  |     else
  |     {
  |         ..our attempt at locking failed
  |         because another thread had already
  |         locked it..
  |
  |     }
  |
  |     // myCriticalSection gets unlocked here
  |     // (if it was locked)
  | }
  | @endcode
  |
  | @see CriticalSection::tryEnter,
  | GenericScopedLock, GenericScopedUnlock
  |
  | @tags{Core}
  */
#[no_copy]
pub struct GenericScopedTryLock<'a, LockType> {
    lock:                &'a LockType,
    lock_was_successful: RefCell<bool>,
}

impl<'a, LockType> Drop for GenericScopedTryLock<'a, LockType> {

    /**
      | Destructor.
      | 
      | The mutex will be unlocked (if it had
      | been successfully locked) when the
      | destructor is called.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      if (lockWasSuccessful) lock_.exit();  */
    }
}

impl<'a, LockType> GenericScopedTryLock<'a, LockType> {

    /**
      | Creates a GenericScopedTryLock.
      | 
      | If acquireLockOnInitialisation is
      | true then as soon as this ScopedTryLock
      | is created, it will attempt to acquire
      | the lock with tryEnter.
      | 
      | You can retry acquiring the lock by calling
      | retryLock.
      | 
      | When GenericScopedTryLock is deleted,
      | the lock will be released (if the lock
      | was successfully acquired).
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      | Best just to use it as a local stack object,
      | rather than creating one with the new()
      | operator.
      | 
      | @see retryLock, isLocked
      |
      */
    pub fn new(
        lock:                           &LockType,
        acquire_lock_on_initialisation: Option<bool>) -> Self {

        let acquire_lock_on_initialisation: bool =
            acquire_lock_on_initialisation.unwrap_or(true);

        todo!();
        /*
            : lock_ (lock), lockWasSuccessful (acquireLockOnInitialisation && lock.tryEnter())
        */
    }

    /**
      | Returns true if the mutex was successfully
      | locked.
      |
      */
    pub fn is_locked(&self) -> bool {
        
        todo!();
        /*
            return lockWasSuccessful;
        */
    }

    /**
      | Retry gaining the lock by calling tryEnter
      | on the underlying lock.
      |
      */
    pub fn retry_lock(&self) -> bool {
        
        todo!();
        /*
            lockWasSuccessful = lock_.tryEnter(); return lockWasSuccessful;
        */
    }
}
