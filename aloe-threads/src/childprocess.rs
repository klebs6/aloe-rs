crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ChildProcess.h]

/**
  | These flags are used by the start() methods.
  |
  */
bitflags!{
    pub struct ChildProcessStreamFlags: u32
    {
        const wantStdOut = 0x01;
        const wantStdErr = 0x10;
    }
}

/**
  | Launches and monitors a child process.
  | 
  | This class lets you launch an executable,
  | and read its output. You can also use
  | it to check whether the child process
  | has finished.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
#[no_copy]
pub struct ChildProcess {
    active_process: Box<ActiveProcess>,
}

impl Default for ChildProcess {
    
    /**
      | Creates a process object.
      | 
      | To actually launch the process, use
      | start().
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Drop for ChildProcess {

    /**
      | Destructor. Note that deleting this
      | object won't terminate the child process.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ChildProcess.cpp]
impl ChildProcess {

    /**
      | Attempts to launch a child process command.
      | 
      | The command should be the name of the
      | executable file, followed by any arguments
      | that are required.
      | 
      | If the process has already been launched,
      | this will launch it again. If a problem
      | occurs, the method will return false.
      | 
      | The streamFlags is a combinations of
      | values to indicate which of the child's
      | output streams should be read and returned
      | by readProcessOutput().
      |
      */
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start_command(
        &mut self, 
        command:      &String,
        stream_flags: Option<ChildProcessStreamFlags>,

    ) -> bool {

        let stream_flags = stream_flags.unwrap_or(
            ChildProcessStreamFlags::wantStdOut | ChildProcessStreamFlags::wantStdErr
        );
        
        todo!();
        /*
            return start (StringArray::fromTokens (command, true), streamFlags);
        */
    }
    
    /**
      | Attempts to launch a child process command.
      | 
      | The first argument should be the name
      | of the executable file, followed by
      | any other arguments that are needed.
      | 
      | If the process has already been launched,
      | this will launch it again. If a problem
      | occurs, the method will return false.
      | 
      | The streamFlags is a combinations of
      | values to indicate which of the child's
      | output streams should be read and returned
      | by readProcessOutput().
      |
      */
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start(
        &mut self, 
        args:         &Vec<String>,
        stream_flags: Option<ChildProcessStreamFlags>,

    ) -> bool {

        let stream_flags 
            = stream_flags.unwrap_or(ChildProcessStreamFlags::wantStdOut | ChildProcessStreamFlags::wantStdErr);
        
        todo!();
        /*
            if (args.size() == 0)
            return false;

        activeProcess.reset (new ActiveProcess (args, streamFlags));

        if (activeProcess->childPID == 0)
            activeProcess.reset();

        return activeProcess != nullptr;
        */
    }

    /**
      | Returns true if the child process is
      | alive.
      |
      */
    pub fn is_running(&self) -> bool {
        
        todo!();
        /*
            return activeProcess != nullptr && activeProcess->isRunning();
        */
    }
    
    /**
      | Attempts to read some output from the
      | child process.
      | 
      | This will attempt to read up to the given
      | number of bytes of data from the process.
      | It returns the number of bytes that were
      | actually read.
      |
      */
    pub fn read_process_output(
        &mut self, 
        dest:      *mut c_void,
        num_bytes: i32

    ) -> i32 {
        
        todo!();
        /*
            return activeProcess != nullptr ? activeProcess->read (dest, numBytes) : 0;
        */
    }

    /**
      | Attempts to kill the child process.
      | 
      | Returns true if it succeeded. Trying
      | to read from the process after calling
      | this may result in undefined behaviour.
      |
      */
    pub fn kill(&mut self) -> bool {
        
        todo!();
        /*
            return activeProcess == nullptr || activeProcess->killProcess();
        */
    }

    /**
      | If the process has finished, this returns
      | its exit code.
      |
      */
    pub fn get_exit_code(&self) -> u32 {
        
        todo!();
        /*
            return activeProcess != nullptr ? activeProcess->getExitCode() : 0;
        */
    }

    /**
      | Blocks until the process is no longer
      | running.
      |
      */
    pub fn wait_for_process_to_finish(&self, timeout_ms: i32) -> bool {
        
        todo!();
        /*
            auto timeoutTime = Time::getMillisecondCounter() + (uint32) timeoutMs;

        do
        {
            if (! isRunning())
                return true;

            Thread::sleep (2);
        }
        while (timeoutMs < 0 || Time::getMillisecondCounter() < timeoutTime);

        return false;
        */
    }

    /**
      | Blocks until the process has finished,
      | and then returns its complete output
      | as a string.
      |
      */
    pub fn read_all_process_output(&mut self) -> String {
        
        todo!();
        /*
            MemoryOutputStream result;

        for (;;)
        {
            char buffer[512];
            auto num = readProcessOutput (buffer, sizeof (buffer));

            if (num <= 0)
                break;

            result.write (buffer, (size_t) num);
        }

        return result.toString();
        */
    }
}


#[no_copy]
#[leak_detector]
pub struct ActiveProcess {
    childpid:    i32, // default = 0
    pipe_handle: i32, // default = 0
    exit_code:   i32, // default = -1
    read_handle: *mut libc::FILE,
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "aloe_posix")]
impl Drop for ActiveProcess {

    fn drop(&mut self) {
        todo!();
        /* 
                if (readHandle != nullptr)
                    fclose (readHandle);

                if (pipeHandle != 0)
                    close (pipeHandle);
             */
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "aloe_posix")]
impl Read for ActiveProcess {

    /**
      | pub fn read(&mut self, dest: *mut c_void, num_bytes: i32) -> i32 {
      |
      */
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        todo!();
        /*
            jassert (dest != nullptr && numBytes > 0);

                #ifdef fdopen
                 #error // some crazy 3rd party headers (e.g. zlib) define this function as NULL!
                #endif

                if (readHandle == nullptr && childPID != 0)
                    readHandle = fdopen (pipeHandle, "r");

                if (readHandle != nullptr)
                {
                    for (;;)
                    {
                        auto numBytesRead = (int) fread (dest, 1, (size_t) numBytes, readHandle);

                        if (numBytesRead > 0 || feof (readHandle))
                            return numBytesRead;

                        // signal occurred during fread() so try again
                        if (ferror (readHandle) && errno == EINTR)
                            continue;

                        break;
                    }
                }

                return 0;
        */
    }
}

impl ActiveProcess {

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "aloe_posix")]
    pub fn new(
        arguments:    &StringArray,
        stream_flags: i32) -> Self {
    
        todo!();
        /*


            auto exe = arguments[0].unquoted();

                // Looks like you're trying to launch a non-existent exe or a folder (perhaps on OSX
                // you're trying to launch the .app folder rather than the actual binary inside it?)
                jassert (File::getCurrentWorkingDirectory().getChildFile (exe).existsAsFile()
                          || ! exe.containsChar (File::getSeparatorChar()));

                int pipeHandles[2] = {};

                if (pipe (pipeHandles) == 0)
                {
                    auto result = fork();

                    if (result < 0)
                    {
                        close (pipeHandles[0]);
                        close (pipeHandles[1]);
                    }
                    else if (result == 0)
                    {
                        // we're the child process..
                        close (pipeHandles[0]);   // close the read handle

                        if ((streamFlags & wantStdOut) != 0)
                            dup2 (pipeHandles[1], STDOUT_FILENO); // turns the pipe into stdout
                        else
                            dup2 (open ("/dev/null", O_WRONLY), STDOUT_FILENO);

                        if ((streamFlags & wantStdErr) != 0)
                            dup2 (pipeHandles[1], STDERR_FILENO);
                        else
                            dup2 (open ("/dev/null", O_WRONLY), STDERR_FILENO);

                        close (pipeHandles[1]);

                        Vec<char*> argv;

                        for (auto& arg : arguments)
                            if (arg.isNotEmpty())
                                argv.add (const_cast<char*> (arg.toRawUTF8()));

                        argv.add (nullptr);

                        execvp (exe.toRawUTF8(), argv.getRawDataPointer());
                        _exit (-1);
                    }
                    else
                    {
                        // we're the parent process..
                        childPID = result;
                        pipeHandle = pipeHandles[0];
                        close (pipeHandles[1]); // close the write handle
                    }
                }
        */
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "aloe_posix")]
    pub fn is_running(&mut self) -> bool {
        
        todo!();
        /*
            if (childPID == 0)
                    return false;

                int childState = 0;
                auto pid = waitpid (childPID, &childState, WNOHANG);

                if (pid == 0)
                    return true;

                if (WIFEXITED (childState))
                {
                    exitCode = WEXITSTATUS (childState);
                    return false;
                }

                return ! WIFSIGNALED (childState);
        */
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "aloe_posix")]
    pub fn kill_process(&self) -> bool {
        
        todo!();
        /*
            return ::kill (childPID, SIGKILL) == 0;
        */
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "aloe_posix")]
    pub fn get_exit_code(&mut self) -> u32 {
        
        todo!();
        /*
            if (exitCode >= 0)
                    return (uint32) exitCode;

                if (childPID != 0)
                {
                    int childState = 0;
                    auto pid = waitpid (childPID, &childState, WNOHANG);

                    if (pid >= 0 && WIFEXITED (childState))
                    {
                        exitCode = WEXITSTATUS (childState);
                        return (uint32) exitCode;
                    }
                }

                return 0;
        */
    }
}

