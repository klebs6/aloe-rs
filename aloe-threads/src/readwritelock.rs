crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ReadWriteLock.h]

/**
  | A critical section that allows multiple
  | simultaneous readers.
  | 
  | Features of this type of lock are:
  | 
  | - Multiple readers can hold the lock
  | at the same time, but only one writer
  | can hold it at once.
  | 
  | - Writers trying to gain the lock will
  | be blocked until all readers and writers
  | have released it
  | 
  | - Readers trying to gain the lock while
  | a writer is waiting to acquire it will
  | be blocked until the writer has obtained
  | and released it
  | 
  | - If a thread already has a read lock and
  | tries to obtain a write lock, it will
  | succeed if there are no other readers
  | 
  | - If a thread already has the write lock
  | and tries to obtain a read lock, this
  | will succeed.
  | 
  | - Recursive locking is supported.
  | 
  | @see ScopedReadLock, ScopedWriteLock,
  | CriticalSection
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct ReadWriteLock {
    access_lock:         SpinLock,
    read_wait_event:     WaitableEvent,
    write_wait_event:    WaitableEvent,
    num_waiting_writers: RefCell<i32>, // default = 0
    num_writers:         RefCell<i32>, // default = 0
    writer_thread_id:    RefCell<ThreadID>,
    reader_threads:      RefCell<Vec<ThreadRecursionCount>>,
}

pub struct ThreadRecursionCount
{
    threadid: ThreadID,
    count:    i32,
}

impl Drop for ReadWriteLock {

    /**
      | Destructor. If the object is deleted
      | whilst locked, any subsequent behaviour
      | is undefined.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        jassert (readerThreads.size() == 0);
        jassert (numWriters == 0);
     */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ReadWriteLock.cpp]
impl Default for ReadWriteLock {

    /**
      | Creates a ReadWriteLock object.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            readerThreads.ensureStorageAllocated (16);
        */
    }
}
    
impl ReadWriteLock {

    /**
      | Locks this object for reading.
      | 
      | Multiple threads can simultaneously
      | lock the object for reading, but if another
      | thread has it locked for writing, then
      | this will block until it releases the
      | lock.
      | 
      | @see exitRead, ScopedReadLock
      |
      */
    pub fn enter_read(&self)  {
        
        todo!();
        /*
            while (! tryEnterRead())
            readWaitEvent.wait (100);
        */
    }
    
    /**
      | Tries to lock this object for reading.
      | 
      | Multiple threads can simultaneously
      | lock the object for reading, but if another
      | thread has it locked for writing, then
      | this will fail and return false.
      | 
      | -----------
      | @return
      | 
      | true if the lock is successfully gained.
      | @see exitRead, ScopedReadLock
      |
      */
    pub fn try_enter_read(&self) -> bool {
        
        todo!();
        /*
            auto threadId = Thread::getCurrentThreadId();

        const SpinLock::ScopedLockType sl (accessLock);

        for (auto& readerThread : readerThreads)
        {
            if (readerThread.threadID == threadId)
            {
                readerThread.count++;
                return true;
            }
        }

        if (numWriters + numWaitingWriters == 0
             || (threadId == writerThreadId && numWriters > 0))
        {
            readerThreads.add ({ threadId, 1 });
            return true;
        }

        return false;
        */
    }
    
    /**
      | Releases the read-lock.
      | 
      | If the caller thread hasn't got the lock,
      | this can have unpredictable results.
      | 
      | If the enterRead() method has been called
      | multiple times by the thread, each call
      | must be matched by a call to exitRead()
      | before other threads will be allowed
      | to take over the lock.
      | 
      | @see enterRead, ScopedReadLock
      |
      */
    pub fn exit_read(&self)  {
        
        todo!();
        /*
            auto threadId = Thread::getCurrentThreadId();
        const SpinLock::ScopedLockType sl (accessLock);

        for (int i = 0; i < readerThreads.size(); ++i)
        {
            auto& readerThread = readerThreads.getReference (i);

            if (readerThread.threadID == threadId)
            {
                if (--(readerThread.count) == 0)
                {
                    readerThreads.remove (i);

                    readWaitEvent.signal();
                    writeWaitEvent.signal();
                }

                return;
            }
        }

        jassertfalse; // unlocking a lock that wasn't locked..
        */
    }
    
    /**
      | Locks this object for writing.
      | 
      | This will block until any other threads
      | that have it locked for reading or writing
      | have released their lock.
      | 
      | @see exitWrite, ScopedWriteLock
      |
      */
    pub fn enter_write(&self)  {
        
        todo!();
        /*
            auto threadId = Thread::getCurrentThreadId();
        const SpinLock::ScopedLockType sl (accessLock);

        while (! tryEnterWriteInternal (threadId))
        {
            ++numWaitingWriters;
            accessLock.exit();
            writeWaitEvent.wait (100);
            accessLock.enter();
            --numWaitingWriters;
        }
        */
    }
    
    /**
      | Tries to lock this object for writing.
      | 
      | This is like enterWrite(), but doesn't
      | block - it returns true if it manages
      | to obtain the lock.
      | 
      | -----------
      | @return
      | 
      | true if the lock is successfully gained.
      | @see enterWrite
      |
      */
    pub fn try_enter_write(&self) -> bool {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (accessLock);
        return tryEnterWriteInternal (Thread::getCurrentThreadId());
        */
    }
    
    pub fn try_enter_write_internal(&self, thread_id: ThreadID) -> bool {
        
        todo!();
        /*
            if (readerThreads.size() + numWriters == 0
             || threadId == writerThreadId
             || (readerThreads.size() == 1 && readerThreads.getReference (0).threadID == threadId))
        {
            writerThreadId = threadId;
            ++numWriters;
            return true;
        }

        return false;
        */
    }
    
    /**
      | Releases the write-lock.
      | 
      | If the caller thread hasn't got the lock,
      | this can have unpredictable results.
      | 
      | If the enterWrite() method has been
      | called multiple times by the thread,
      | each call must be matched by a call to
      | exit() before other threads will be
      | allowed to take over the lock.
      | 
      | @see enterWrite, ScopedWriteLock
      |
      */
    pub fn exit_write(&self)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (accessLock);

        // check this thread actually had the lock..
        jassert (numWriters > 0 && writerThreadId == Thread::getCurrentThreadId());

        if (--numWriters == 0)
        {
            writerThreadId = {};

            readWaitEvent.signal();
            writeWaitEvent.signal();
        }
        */
    }
}
