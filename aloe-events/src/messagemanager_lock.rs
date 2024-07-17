crate::ix!();

/**
  | A lock you can use to lock the message
  | manager. You can use this class with
  | the RAII-based ScopedLock classes.
  |
  */
pub struct MessageManagerLockPimpl {
    blocking_message: RefCell<ReferenceCountedObjectPtr<lock::BlockingMessage>>,
    locked_event:     WaitableEvent,
    abort_wait:       RefCell<AtomicI32>,
    lock_gained:      RefCell<AtomicI32>,
}

pub mod lock {

    use super::*;

    /**
      | Provides the type of scoped lock to use
      | with a CriticalSection.
      |
      */
    pub type ScopedLockType<'a> = GenericScopedLock<'a, MessageManagerLockPimpl>;

    /**
      | Provides the type of scoped unlocker
      | to use with a CriticalSection.
      |
      */
    pub type ScopedUnlockType<'a> = GenericScopedUnlock<'a, MessageManagerLockPimpl>;

    /**
      | Provides the type of scoped try-locker
      | to use with a CriticalSection.
      |
      */
    pub type ScopedTryLockType<'a> = GenericScopedTryLock<'a, MessageManagerLockPimpl>;

    /**
      | The only safe way to lock the message
      | thread while another thread does some
      | work is by posting a special message,
      | whose purpose is to tie up the event loop
      | until the other thread has finished
      | its business.
      | 
      | Any other approach can get horribly
      | deadlocked if the OS uses its own hidden
      | locks which get locked before making
      | an event callback, because if the same
      | OS lock gets indirectly accessed from
      | another thread inside a MM lock, you're
      | screwed. (this is exactly what happens
      | in Cocoa).
      |
      */
    #[no_copy]
    pub struct BlockingMessage {
        owner_critical_section: CriticalSection,
        owner:                  Atomic<*const MessageManagerLockPimpl>,
        release_event:          WaitableEvent,
    }

    impl BlockingMessage {

        pub fn new(parent: *const MessageManagerLockPimpl) -> Self {
        
            todo!();
            /*
            : owner(parent),

            
            */
        }
    }

    impl MessageBaseInterface for BlockingMessage {
        
        fn message_callback(&mut self)  {
            
            todo!();
            /*
                {
                            ScopedLock lock (ownerCriticalSection);

                            if (auto* o = owner.get())
                                o->messageCallback();
                        }

                        releaseEvent.wait();
            */
        }
    }
}

impl Default for MessageManagerLockPimpl {
    
    /**
      | Creates a new critical section to exclusively
      | access methods which can only be called
      | when the message manager is locked.
      | 
      | Unlike CrititcalSection, multiple
      | instances of this lock class provide
      | exclusive access to a single resource
      | - the MessageManager.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Drop for MessageManagerLockPimpl {
    fn drop(&mut self) {
        todo!();
        /*      exit();  */
    }
}

impl MessageManagerLockPimpl {

    /**
      | Acquires the message manager lock.
      | 
      | If the caller thread already has exclusive
      | access to the MessageManager, this
      | method will return immediately. If
      | another thread is currently using the
      | MessageManager, this will wait until
      | that thread releases the lock to the
      | MessageManager.
      | 
      | This call will only exit if the lock was
      | acquired by this thread. Calling abort
      | while a thread is waiting for enter to
      | finish, will have no effect.
      | 
      | @see exit, abort
      |
      */
    pub fn enter(&self)  {
        
        todo!();
        /*
            tryAcquire (true);
        */
    }
    
    /** 
      | Attempts to lock the message manager and
      | exits if abort is called.
      |
      | This method behaves identically to
      | enter, except that it will abort
      | waiting for the lock if the abort
      | method is called.
      |
      | Unlike other Aloe critical sections,
      | this method **will** block waiting for
      | the lock.
      |
      | To ensure predictable behaviour, you
      | should re-check your abort condition
      | if tryEnter returns false.
      |
      | This method can be used if you want to
      | do some work while waiting for the
      | MessageManagerLockPimpl:
      |
      |   void doWorkWhileWaitingForMessageManagerLockPimpl()
      |   {
      |       MessageManager::MessageManagerLockPimpl::ScopedTryLockType mmLock (messageManagerLock);
      |
      |       while (! mmLock.isLocked())
      |       {
      |            while (workQueue.size() > 0)
      |            {
      |                 auto work = workQueue.pop();
      |                 doSomeWork (work);
      |            }
      |
      |            // this will block until we either have the lock or there is work
      |            mmLock.retryLock();
      |       }
      |
      |       // we have the mmlock
      |       // do some message manager stuff like resizing and painting components
      |   }
      |
      |   // called from another thread
      |   void addWorkToDo (Work work)
      |   {
      |        queue.push (work);
      |        messageManagerLock.abort();
      |   }
      |
      |   @returns false if waiting for a lock was aborted, true if the lock was acquired.
      |   @see enter, abort, ScopedTryLock
      */
    pub fn try_enter(&self) -> bool {
        
        todo!();
        /*
            return tryAcquire (false);
        */
    }
    
    pub fn try_acquire(&self, lock_is_mandatory: bool) -> bool {
        
        todo!();
        /*
            auto* mm = MessageManager::instance;

        if (mm == nullptr)
        {
            jassertfalse;
            return false;
        }

        if (! lockIsMandatory && (abortWait.get() != 0))
        {
            abortWait.set (0);
            return false;
        }

        if (mm->currentThreadHasLockedMessageManager())
            return true;

        try
        {
            blockingMessage = *new BlockingMessage (this);
        }
        catch (...)
        {
            jassert (! lockIsMandatory);
            return false;
        }

        if (! blockingMessage->post())
        {
            // post of message failed while trying to get the lock
            jassert (! lockIsMandatory);
            blockingMessage = nullptr;
            return false;
        }

        do
        {
            while (abortWait.get() == 0)
                lockedEvent.wait (-1);

            abortWait.set (0);

            if (lockGained.get() != 0)
            {
                mm->threadWithLock = Thread::getCurrentThreadId();
                return true;
            }

        } while (lockIsMandatory);

        // we didn't get the lock
        blockingMessage->releaseEvent.signal();

        {
            ScopedLock lock (blockingMessage->ownerCriticalSection);

            lockGained.set (0);
            blockingMessage->owner.set (nullptr);
        }

        blockingMessage = nullptr;
        return false;
        */
    }
    
    /**
      | Releases the message manager lock.
      | @see enter, ScopedLock
      |
      */
    pub fn exit(&self)  {
        
        todo!();
        /*
            if (lockGained.compareAndSetBool (false, true))
        {
            auto* mm = MessageManager::instance;

            jassert (mm == nullptr || mm->currentThreadHasLockedMessageManager());
            lockGained.set (0);

            if (mm != nullptr)
                mm->threadWithLock = {};

            if (blockingMessage != nullptr)
            {
                blockingMessage->releaseEvent.signal();
                blockingMessage = nullptr;
            }
        }
        */
    }
    
    pub fn message_callback(&self)  {
        
        todo!();
        /*
            lockGained.set (1);
        abort();
        */
    }
    
    /**
      | Unblocks a thread which is waiting in
      | tryEnter
      | 
      | Call this method if you want to unblock
      | a thread which is waiting for the MessageManager
      | lock in tryEnter.
      | 
      | This method does not have any effect
      | on a thread waiting for a lock in enter.
      | @see tryEnter
      |
      */
    pub fn abort(&self)  {
        
        todo!();
        /*
            abortWait.set (1);
        lockedEvent.signal();
        */
    }
}

