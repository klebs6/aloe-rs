crate::ix!();

pub trait CAMutexInterface {

    /* -------------------- * Actions  -------------------- */
    fn lock(&mut self) -> bool;

    fn unlock(&mut self);

    /**
      | returns true if lock is free, false if
      | not
      |
      */
    fn try_(&mut self, out_was_locked: &mut bool) -> bool;

    fn is_free(&self) -> bool;

    fn is_owned_by_current_thread(&self) -> bool;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAMutex.h]

/**
  | A recursive mutex.
  |
  */
pub struct CAMutex {

    name: *const u8,

    #[cfg(TARGET_OS_MAC)] owner: PThread,
    #[cfg(TARGET_OS_MAC)] mutex: PThreadMutex,

    #[cfg(TARGET_OS_WIN32)] owner: u32,
    #[cfg(TARGET_OS_WIN32)] mutex: HANDLE,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAMutex.cpp]
///-------------------------
impl Drop for CAMutex {

    fn drop(&mut self) {
        todo!();
        /*
            #if TARGET_OS_MAC
        #if Log_Ownership
            DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::~CAMutex: destroying %s, owner: %p\n", pthread_self(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), mName, mOwner);
        #endif
        pthread_mutex_destroy(&mMutex);
    #elif TARGET_OS_WIN32
        #if Log_Ownership
            DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::~CAMutex: destroying %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), mName, mOwner);
        #endif
        if(mMutex != NULL)
        {
            CloseHandle(mMutex);
        }
    #endif
        */
    }
}

impl CAMutex {
    
    pub fn new(in_name: *const u8) -> Self {
    
        todo!();
        /*
        : name(inName),
        : owner(0),

            #if TARGET_OS_MAC
        OSStatus theError = pthread_mutex_init(&mMutex, NULL);
        ThrowIf(theError != 0, CAException(theError), "CAMutex::CAMutex: Could not init the mutex");

        #if Log_Ownership
            DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::CAMutex: creating %s, owner: %p\n", pthread_self(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), mName, mOwner);
        #endif
    #elif TARGET_OS_WIN32
        mMutex = CreateMutex(NULL, false, NULL);
        ThrowIfNULL(mMutex, CAException(GetLastError()), "CAMutex::CAMutex: could not create the mutex.");

        #if Log_Ownership
            DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::CAMutex: creating %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), mName, mOwner);
        #endif
    #endif
        */
    }
    
    pub fn lock(&mut self) -> bool {
        
        todo!();
        /*
            bool theAnswer = false;

    #if TARGET_OS_MAC
        pthread_t theCurrentThread = pthread_self();
        if(!pthread_equal(theCurrentThread, mOwner))
        {
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Lock: thread %p is locking %s, owner: %p\n", theCurrentThread, ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), theCurrentThread, mName, mOwner);
            #endif

            #if Log_LongLatencies
                UInt64 lockTryTime = CAHostTimeBase::GetCurrentTimeInNanos();
            #endif

            OSStatus theError = pthread_mutex_lock(&mMutex);
            ThrowIf(theError != 0, CAException(theError), "CAMutex::Lock: Could not lock the mutex");
            mOwner = theCurrentThread;
            theAnswer = true;

            #if Log_LongLatencies
                UInt64 lockAcquireTime = CAHostTimeBase::GetCurrentTimeInNanos();
                if (lockAcquireTime - lockTryTime >= LongLatencyThresholdNS)
                    DebugPrintfRtn(DebugPrintfFileComma "Thread %p took %.6fs to acquire the lock %s\n", theCurrentThread, (lockAcquireTime - lockTryTime) * 1.0e-9 /* nanos to seconds */, mName);
            #endif

            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Lock: thread %p has locked %s, owner: %p\n", pthread_self(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), pthread_self(), mName, mOwner);
            #endif
        }
    #elif TARGET_OS_WIN32
        if(mOwner != GetCurrentThreadId())
        {
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Lock: thread %lu is locking %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
            #endif

            OSStatus theError = WaitForSingleObject(mMutex, INFINITE);
            ThrowIfError(theError, CAException(theError), "CAMutex::Lock: could not lock the mutex");
            mOwner = GetCurrentThreadId();
            theAnswer = true;

            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Lock: thread %lu has locked %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
            #endif
        }
    #endif

        return theAnswer;
        */
    }
    
    pub fn unlock(&mut self)  {
        
        todo!();
        /*
            #if TARGET_OS_MAC
        if(pthread_equal(pthread_self(), mOwner))
        {
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Unlock: thread %p is unlocking %s, owner: %p\n", pthread_self(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), pthread_self(), mName, mOwner);
            #endif

            mOwner = 0;
            OSStatus theError = pthread_mutex_unlock(&mMutex);
            ThrowIf(theError != 0, CAException(theError), "CAMutex::Unlock: Could not unlock the mutex");

            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Unlock: thread %p has unlocked %s, owner: %p\n", pthread_self(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), pthread_self(), mName, mOwner);
            #endif
        }
        else
        {
            DebugMessage("CAMutex::Unlock: A thread is attempting to unlock a Mutex it doesn't own");
        }
    #elif TARGET_OS_WIN32
        if(mOwner == GetCurrentThreadId())
        {
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Unlock: thread %lu is unlocking %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
            #endif

            mOwner = 0;
            bool wasReleased = ReleaseMutex(mMutex);
            ThrowIf(!wasReleased, CAException(GetLastError()), "CAMutex::Unlock: Could not unlock the mutex");

            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Unlock: thread %lu has unlocked %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
            #endif
        }
        else
        {
            DebugMessage("CAMutex::Unlock: A thread is attempting to unlock a Mutex it doesn't own");
        }
    #endif
        */
    }
    
    pub fn try_(&mut self, out_was_locked: &mut bool) -> bool {
        
        todo!();
        /*
            bool theAnswer = false;
        outWasLocked = false;

    #if TARGET_OS_MAC
        pthread_t theCurrentThread = pthread_self();
        if(!pthread_equal(theCurrentThread, mOwner))
        {
            //  this means the current thread doesn't already own the lock
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Try: thread %p is try-locking %s, owner: %p\n", theCurrentThread, ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), theCurrentThread, mName, mOwner);
            #endif

            //  go ahead and call trylock to see if we can lock it.
            int theError = pthread_mutex_trylock(&mMutex);
            if(theError == 0)
            {
                //  return value of 0 means we successfully locked the lock
                mOwner = theCurrentThread;
                theAnswer = true;
                outWasLocked = true;

                #if Log_Ownership
                    DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Try: thread %p has locked %s, owner: %p\n", theCurrentThread, ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), theCurrentThread, mName, mOwner);
                #endif
            }
            else if(theError == EBUSY)
            {
                //  return value of EBUSY means that the lock was already locked by another thread
                theAnswer = false;
                outWasLocked = false;

                #if Log_Ownership
                    DebugPrintfRtn(DebugPrintfFileComma "%p %.4f: CAMutex::Try: thread %p failed to lock %s, owner: %p\n", theCurrentThread, ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), theCurrentThread, mName, mOwner);
                #endif
            }
            else
            {
                //  any other return value means something really bad happenned
                ThrowIfError(theError, CAException(theError), "CAMutex::Try: call to pthread_mutex_trylock failed");
            }
        }
        else
        {
            //  this means the current thread already owns the lock
            theAnswer = true;
            outWasLocked = false;
        }
    #elif TARGET_OS_WIN32
        if(mOwner != GetCurrentThreadId())
        {
            //  this means the current thread doesn't own the lock
            #if Log_Ownership
                DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Try: thread %lu is try-locking %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
            #endif

            //  try to acquire the mutex
            OSStatus theError = WaitForSingleObject(mMutex, 0);
            if(theError == WAIT_OBJECT_0)
            {
                //  this means we successfully locked the lock
                mOwner = GetCurrentThreadId();
                theAnswer = true;
                outWasLocked = true;

                #if Log_Ownership
                    DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Try: thread %lu has locked %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
                #endif
            }
            else if(theError == WAIT_TIMEOUT)
            {
                //  this means that the lock was already locked by another thread
                theAnswer = false;
                outWasLocked = false;

                #if Log_Ownership
                    DebugPrintfRtn(DebugPrintfFileComma "%lu %.4f: CAMutex::Try: thread %lu failed to lock %s, owner: %lu\n", GetCurrentThreadId(), ((Float64)(CAHostTimeBase::GetCurrentTimeInNanos()) / 1000000.0), GetCurrentThreadId(), mName, mOwner);
                #endif
            }
            else
            {
                //  any other return value means something really bad happenned
                ThrowIfError(theError, CAException(GetLastError()), "CAMutex::Try: call to lock the mutex failed");
            }
        }
        else
        {
            //  this means the current thread already owns the lock
            theAnswer = true;
            outWasLocked = false;
        }
    #endif

        return theAnswer;
        */
    }
    
    pub fn is_free(&self) -> bool {
        
        todo!();
        /*
            return mOwner == 0;
        */
    }
    
    pub fn is_owned_by_current_thread(&self) -> bool {
        
        todo!();
        /*
            bool theAnswer = true;

    #if TARGET_OS_MAC
        theAnswer = pthread_equal(pthread_self(), mOwner);
    #elif TARGET_OS_WIN32
        theAnswer = (mOwner == GetCurrentThreadId());
    #endif

        return theAnswer;
        */
    }
}
