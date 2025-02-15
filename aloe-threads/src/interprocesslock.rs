crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_InterProcessLock.h]

/**
  | Acts as a critical section which processes
  | can use to block each other.
  | 
  | @see CriticalSection
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct InterProcessLock {
    impl_: Box<InterProcessLockImpl>,
    lock:  CriticalSection,
    name:  String,
}

impl Drop for InterProcessLock {

    /**
      | Destructor.
      | 
      | This will also release the lock if it's
      | currently held by this process.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

impl InterProcessLock {

    /**
      | Creates a lock object.
      | 
      | -----------
      | @param name
      | 
      | a name that processes will use to identify
      | this lock object
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    /**
      | Attempts to lock the critical section.
      | 
      | -----------
      | @param timeOutMillisecs
      | 
      | how many milliseconds to wait if the
      | lock is already held by another process
      | - a value of 0 will return immediately,
      | negative values will wait forever
      | 
      | -----------
      | @return
      | 
      | true if the lock could be gained within
      | the timeout period, or false if the timeout
      | expired.
      |
      */
    pub fn enter(&mut self, time_out_millisecs: Option<i32>) -> bool {

        let time_out_millisecs: i32 = time_out_millisecs.unwrap_or(-1);

        todo!();
        /*
        
        */
    }

    /**
      | Releases the lock if it's currently
      | held by this process.
      |
      */
    pub fn exit(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(not(target_os="ios"))]
pub struct InterProcessLockImpl {
    handle:    i32, // default = 0
    ref_count: i32, // default = 1
}

#[cfg(not(target_os="ios"))]
impl Drop for InterProcessLockImpl {
    fn drop(&mut self) {
        todo!();
        /* 
            closeFile();
         */
    }
}


#[cfg(target_os="ios")]
pub struct InterProcessLockImpl {

    /**
       On iOS just fake success..
      */
    handle:    i32, // default = 1
    ref_count: i32, // default = 1
}

#[cfg(feature = "aloe_posix")]
#[cfg(target_os="ios")]
impl InterProcessLockImpl {

    pub fn new(
        _0: &String,
        _1: i32) -> Self {
    
        todo!();
        /*


        
        */
    }
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(target_os="ios"))]
impl InterProcessLockImpl {

    pub fn new(
        lock_name:          &String,
        time_out_millisecs: i32) -> Self {
    
        todo!();
        /*


            #if ALOE_MAC
            if (! createLockFile (File ("~/Library/Caches/com.aloe.locks").getChildFile (lockName), timeOutMillisecs))
                // Fallback if the user's home folder is on a network drive with no ability to lock..
                createLockFile (File ("/tmp/com.aloe.locks").getChildFile (lockName), timeOutMillisecs);

           #else
            File tempFolder ("/var/tmp");

            if (! tempFolder.isDirectory())
                tempFolder = "/tmp";

            createLockFile (tempFolder.getChildFile (lockName), timeOutMillisecs);
           #endif
        */
    }
    
    pub fn create_lock_file(&mut self, 
        file:               &File,
        time_out_millisecs: i32) -> bool {
        
        todo!();
        /*
            file.create();
            handle = open (file.getFullPathName().toUTF8(), O_RDWR);

            if (handle != 0)
            {
                struct flock fl;
                zerostruct (fl);

                fl.l_whence = SEEK_SET;
                fl.l_type = F_WRLCK;

                auto endTime = Time::currentTimeMillis() + timeOutMillisecs;

                for (;;)
                {
                    auto result = fcntl (handle, F_SETLK, &fl);

                    if (result >= 0)
                        return true;

                    auto error = errno;

                    if (error != EINTR)
                    {
                        if (error == EBADF || error == ENOTSUP)
                            return false;

                        if (timeOutMillisecs == 0
                             || (timeOutMillisecs > 0 && Time::currentTimeMillis() >= endTime))
                            break;

                        Thread::sleep (10);
                    }
                }
            }

            closeFile();
            return true; // only false if there's a file system error. Failure to lock still returns true.
        */
    }
    
    pub fn close_file(&mut self)  {
        
        todo!();
        /*
            if (handle != 0)
            {
                struct flock fl;
                zerostruct (fl);

                fl.l_whence = SEEK_SET;
                fl.l_type = F_UNLCK;

                while (! (fcntl (handle, F_SETLKW, &fl) >= 0 || errno != EINTR))
                {}

                close (handle);
                handle = 0;
            }
        */
    }
}

#[cfg(feature = "aloe_posix")]
impl InterProcessLock {
    
    pub fn new(nm: &String) -> Self {
    
        todo!();
        /*
        : name(nm),

        
        */
    }
    
    pub fn enter(&mut self, time_out_millisecs: i32) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (impl == nullptr)
        {
            impl.reset (new Impl (name, timeOutMillisecs));

            if (impl->handle == 0)
                impl.reset();
        }
        else
        {
            impl->refCount++;
        }

        return impl != nullptr;
        */
    }
    
    pub fn exit(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        // Trying to release the lock too many times!
        jassert (impl != nullptr);

        if (impl != nullptr && --(impl->refCount) == 0)
            impl.reset();
        */
    }
}
