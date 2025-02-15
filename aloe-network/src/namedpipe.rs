crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_NamedPipe.h]

/**
  | A cross-process pipe that can have data
  | written to and read from it.
  | 
  | Two processes can use NamedPipe objects
  | to exchange blocks of data.
  | 
  | @see InterprocessConnection
  | 
  | @tags{Core}
  |
  */
#[cfg(not(ALOE_WASM))]
#[leak_detector]
#[no_copy]
pub struct NamedPipe {
    impl_:             Box<NamedPipeImpl>,
    current_pipe_name: String,
    lock:              ReadWriteLock,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_NamedPipe.cpp]
#[cfg(not(ALOE_WASM))]
#[cfg(target_os_family = "unix")]
impl NamedPipe {
    
    /**
      | Closes the pipe, if it's open.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
            {
            ScopedReadLock sl (lock);

            if (impl != nullptr)
            {
                impl->stopReadOperation = true;

                char buffer[1] = { 0 };
                ssize_t done = ::write (impl->pipeIn, buffer, 1);
                ignoreUnused (done);
            }
        }

        {
            ScopedWriteLock sl (lock);
            impl.reset();
        }
        */
    }
    
    pub fn open_internal(&mut self, 
        pipe_name:      &String,
        create_pipe:    bool,
        must_not_exist: bool) -> bool {
        
        todo!();
        /*
            #if ALOE_IOS
        impl.reset (new Impl (File::getSpecialLocation (File::tempDirectory)
                                 .getChildFile (File::createLegalFileName (pipeName)).getFullPathName(), createPipe));
       #else
        auto file = pipeName;

        if (! File::isAbsolutePath (file))
            file = "/tmp/" + File::createLegalFileName (file);

        impl.reset (new Impl (file, createPipe));
       #endif

        if (createPipe && ! impl->createFifos (mustNotExist))
        {
            impl.reset();
            return false;
        }

        if (! impl->connect (200))
        {
            impl.reset();
            return false;
        }

        return true;
        */
    }
    
    /**
      | Reads data from the pipe.
      | 
      | This will block until another thread
      | has written enough data into the pipe
      | to fill the number of bytes specified,
      | or until another thread calls the cancelPendingReads()
      | method.
      | 
      | If the operation fails, it returns -1,
      | otherwise, it will return the number
      | of bytes read.
      | 
      | If timeOutMilliseconds is less than
      | zero, it will wait indefinitely, otherwise
      | this is a maximum timeout for reading
      | from the pipe.
      |
      */
    pub fn read(&mut self, 
        dest_buffer:           *mut c_void,
        max_bytes_to_read:     i32,
        time_out_milliseconds: i32) -> i32 {
        
        todo!();
        /*
            ScopedReadLock sl (lock);
        return impl != nullptr ? impl->read (static_cast<char*> (destBuffer), maxBytesToRead, timeOutMilliseconds) : -1;
        */
    }
    
    /**
      | Writes some data to the pipe.
      | 
      | -----------
      | @return
      | 
      | the number of bytes written, or -1 on
      | failure.
      |
      */
    pub fn write(&mut self, 
        source_buffer:         *const c_void,
        num_bytes_to_write:    i32,
        time_out_milliseconds: i32) -> i32 {
        
        todo!();
        /*
            ScopedReadLock sl (lock);
        return impl != nullptr ? impl->write (static_cast<const char*> (sourceBuffer), numBytesToWrite, timeOutMilliseconds) : -1;
        */
    }
}

#[cfg(not(ALOE_WASM))]
impl Drop for NamedPipe {

    fn drop(&mut self) {
        todo!();
        /* 
        close();
 */
    }
}

#[cfg(not(ALOE_WASM))]
impl NamedPipe {
    
    /**
      | Tries to open a pipe that already exists.
      | 
      | Returns true if it succeeds.
      |
      */
    pub fn open_existing(&mut self, pipe_name: &String) -> bool {
        
        todo!();
        /*
            close();

        ScopedWriteLock sl (lock);
        currentPipeName = pipeName;
        return openInternal (pipeName, false, false);
        */
    }

    /**
      | True if the pipe is currently open.
      |
      */
    pub fn is_open(&self) -> bool {
        
        todo!();
        /*
            ScopedReadLock sl (lock);
        return impl != nullptr;
        */
    }

    /**
      | Tries to create a new pipe.
      | 
      | Returns true if it succeeds.
      | 
      | If mustNotExist is true then it will
      | fail if a pipe is already open with the
      | same name.
      |
      */
    pub fn create_new_pipe(
        &mut self, 
        pipe_name:      &String,
        must_not_exist: Option<bool>

    ) -> bool {

        let must_not_exist: bool = must_not_exist.unwrap_or(false);
        
        todo!();
        /*
            close();

        ScopedWriteLock sl (lock);
        currentPipeName = pipeName;
        return openInternal (pipeName, true, mustNotExist);
        */
    }

    /**
      | Returns the last name that was used to
      | try to open this pipe.
      |
      */
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            ScopedReadLock sl (lock);
        return currentPipeName;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_posix_NamedPipe.cpp]

#[cfg(not(ALOE_WASM))]
#[no_copy]
#[leak_detector]
pub struct NamedPipeImpl {
    pipe_in_name:        String,
    pipe_out_name:       String,
    pipe_in:             i32, // default = -1
    pipe_out:            i32, // default = -1
    created_fifo_in:     bool, // default = false
    created_fifo_out:    bool, // default = false
    created_pipe:        bool,
    stop_read_operation: AtomicBool, // default = false 
}

#[cfg(not(ALOE_WASM))]
#[cfg(target_os_family = "unix")]
impl Drop for NamedPipeImpl {
    fn drop(&mut self) {
        todo!();
        /* 
                if (pipeIn  != -1)  ::close (pipeIn);
                if (pipeOut != -1)  ::close (pipeOut);

                if (createdPipe)
                {
                    if (createdFifoIn)  unlink (pipeInName.toUTF8());
                    if (createdFifoOut) unlink (pipeOutName.toUTF8());
                }
             */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(target_os_family = "unix")]
impl NamedPipeImpl {

    pub fn new(
        pipe_path:   &String,
        create_pipe: bool) -> Self {
    
        todo!();
        /*


            : pipeInName  (pipePath + "_in"),
                 pipeOutName (pipePath + "_out"),
                 createdPipe (createPipe)

                signal (SIGPIPE, signalHandler);
                aloe_siginterrupt (SIGPIPE, 1);
        */
    }
    
    pub fn connect(&mut self, time_out_milliseconds: i32) -> bool {
        
        todo!();
        /*
            return openPipe (true, getTimeoutEnd (timeOutMilliseconds));
        */
    }
    
    pub fn read(&mut self, 
        dest_buffer:           *mut u8,
        max_bytes_to_read:     i32,
        time_out_milliseconds: i32) -> i32 {
        
        todo!();
        /*
            auto timeoutEnd = getTimeoutEnd (timeOutMilliseconds);
                int bytesRead = 0;

                while (bytesRead < maxBytesToRead)
                {
                    auto bytesThisTime = maxBytesToRead - bytesRead;
                    auto numRead = (int) ::read (pipeIn, destBuffer, (size_t) bytesThisTime);

                    if (numRead <= 0)
                    {
                        if (errno != EWOULDBLOCK || stopReadOperation.load() || hasExpired (timeoutEnd))
                            return -1;

                        const int maxWaitingTime = 30;
                        waitForInput (pipeIn, timeoutEnd == 0 ? maxWaitingTime
                                                              : jmin (maxWaitingTime,
                                                                      (int) (timeoutEnd - Time::getMillisecondCounter())));
                        continue;
                    }

                    bytesRead += numRead;
                    destBuffer += numRead;
                }

                return bytesRead;
        */
    }
    
    pub fn write(&mut self, 
        source_buffer:         *const u8,
        num_bytes_to_write:    i32,
        time_out_milliseconds: i32) -> i32 {
        
        todo!();
        /*
            auto timeoutEnd = getTimeoutEnd (timeOutMilliseconds);

                if (! openPipe (false, timeoutEnd))
                    return -1;

                int bytesWritten = 0;

                while (bytesWritten < numBytesToWrite && ! hasExpired (timeoutEnd))
                {
                    auto bytesThisTime = numBytesToWrite - bytesWritten;
                    auto numWritten = (int) ::write (pipeOut, sourceBuffer, (size_t) bytesThisTime);

                    if (numWritten <= 0)
                        return -1;

                    bytesWritten += numWritten;
                    sourceBuffer += numWritten;
                }

                return bytesWritten;
        */
    }
    
    pub fn create_fifo(
        name:           &String,
        must_not_exist: bool) -> bool {
        
        todo!();
        /*
            return mkfifo (name.toUTF8(), 0666) == 0 || ((! mustNotExist) && errno == EEXIST);
        */
    }
    
    pub fn create_fifos(&mut self, must_not_exist: bool) -> bool {
        
        todo!();
        /*
            createdFifoIn  = createFifo (pipeInName, mustNotExist);
                createdFifoOut = createFifo (pipeOutName, mustNotExist);

                return createdFifoIn && createdFifoOut;
        */
    }
    
    pub fn signal_handler(_0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_timeout_end(time_out_milliseconds: i32) -> u32 {
        
        todo!();
        /*
            return timeOutMilliseconds >= 0 ? Time::getMillisecondCounter() + (uint32) timeOutMilliseconds : 0;
        */
    }
    
    pub fn has_expired(timeout_end: u32) -> bool {
        
        todo!();
        /*
            return timeoutEnd != 0 && Time::getMillisecondCounter() >= timeoutEnd;
        */
    }
    
    pub fn open_pipe(&mut self, 
        name:        &String,
        flags:       i32,
        timeout_end: u32) -> i32 {
        
        todo!();
        /*
            for (;;)
                {
                    auto p = ::open (name.toUTF8(), flags);

                    if (p != -1 || hasExpired (timeoutEnd) || stopReadOperation.load())
                        return p;

                    Thread::sleep (2);
                }
        */
    }
    
    pub fn open_pipe(&mut self, 
        is_input:    bool,
        timeout_end: u32) -> bool {
        
        todo!();
        /*
            auto& pipe = isInput ? pipeIn : pipeOut;
                int flags = (isInput ? O_RDWR : O_WRONLY) | O_NONBLOCK;

                const String& pipeName = isInput ? (createdPipe ? pipeInName : pipeOutName)
                                                 : (createdPipe ? pipeOutName : pipeInName);

                if (pipe == -1)
                {
                    pipe = openPipe (pipeName, flags, timeoutEnd);

                    if (pipe == -1)
                        return false;
                }

                return true;
        */
    }
    
    pub fn wait_for_input(
        handle:        i32,
        timeout_msecs: i32)  {
        
        todo!();
        /*
            pollfd pfd { handle, POLLIN, 0 };
                poll (&pfd, 1, timeoutMsecs);
        */
    }
}

// other methods for this class are implemented in
// the platform-specific files
