/*! 
  | @file base/source/fdebug.h
  | Debugging tools.
  |
  | There are 2 levels of debugging messages:
  | - DEVELOPMENT
  |   - During development
  | - RELEASE
  |   - Program is shipping.
  */
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fdebug.h]

crate::ix!();

/**
  | Returns true if a debugger is attached.
  |
  */
pub fn am_ibeing_debugged() -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | If "f" is not true and a debugger is present,
  | send an error string to the debugger
  | for display and cause a breakpoint exception
  | to occur in the current process. SMTG_ASSERT
  | is removed completely in RELEASE configuration.
  | So do not pass methods calls to this macro
  | that are expected to exist in the RELEASE
  | build (for method calls that need to
  | be present in a RELEASE build, use the
  | VERIFY macros instead)
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_assert {
    ($f:ident) => {
        /*
        
            if (!(f))          
                FDebugBreak ("%s(%d) : Assert failed: %s\n", __FILE__, __LINE__, #f);
        */
    }
}

/**
  | Send "comment" string to the debugger
  | for display.
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_warning {
    ($comment:ident) => {
        /*
                FDebugPrint ("%s(%d) : %s\n", __FILE__, __LINE__, comment);
        */
    }
}

/**
  | Send the last error string to the debugger
  | for display.
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_printsyserror {
    () => {
        /*
                FPrintLastError (__FILE__, __LINE__);
        */
    }
}

/**
  | If a debugger is present, send string
  | "s" to the debugger for display and cause
  | a breakpoint exception to occur in the
  | current process.
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_debugstr {
    ($s:ident) => {
        /*
                FDebugBreak (s);
        */
    }
}

/**
  | Use VERIFY for calling methods "f" having
  | a bool result (expecting them to return
  | 'true')
  | 
  | The call of "f" is not removed in RELEASE
  | builds, only the result verification.
  | eg: SMTG_VERIFY (isValid ())
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_verify {
    ($f:ident) => {
        /*
                SMTG_ASSERT (f)
        */
    }
}

/**
  | Use VERIFY_IS for calling methods "f"
  | and expect a certain result "r".
  | 
  | The call of "f" is not removed in RELEASE
  | builds, only the result verification.
  | eg:
  | 
  | SMTG_VERIFY_IS (callMethod (), kResultOK)
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_verify_is {
    ($f:ident, $r:ident) => {
        /*
        
            if ((f) != (r))          
                FDebugBreak ("%s(%d) : Assert failed: %s\n", __FILE__, __LINE__, #f);
        */
    }
}

/**
  | Use VERIFY_NOT for calling methods
  | "f" and expect the result to be anything
  | else but "r".
  | 
  | The call of "f" is not removed in RELEASE
  | builds, only the result verification.
  | eg:
  | 
  | SMTG_VERIFY_NOT (callMethod (), kResultError)
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_verify_not {
    ($f:ident, $r:ident) => {
        /*
        
            if ((f) == (r))           
                FDebugBreak ("%s(%d) : Assert failed: %s\n", __FILE__, __LINE__, #f);
        */
    }
}

/**
  | @name Shortcut macros for sending strings
  | to the debugger for display.
  | 
  | First parameter is always the format
  | string (printf like).
  |
  */
#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt0 {
    ($a:ident) => {
        /*
                FDebugPrint (a);
        */
    }
}

#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt1 {
    ($a:ident, $b:ident) => {
        /*
                FDebugPrint (a, b);
        */
    }
}

#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt2 {
    ($a:ident, $b:ident, $c:ident) => {
        /*
                FDebugPrint (a, b, c);
        */
    }
}

#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt3 {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        /*
                FDebugPrint (a, b, c, d);
        */
    }
}

#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt4 {
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => {
        /*
                FDebugPrint (a, b, c, d, e);
        */
    }
}

#[cfg(DEVELOPMENT)]
macro_rules! smtg_dbprt5 {
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => {
        /*
                FDebugPrint (a, b, c, d, e, f);
        */
    }
}


/**
  | @name Helper functions for the above
  | defined macros.
  | 
  | You shouldn't use them directly (if
  | you do so, don't forget "#if DEVELOPMENT")!
  | 
  | It is recommended to use the macros instead.
  |
  */
#[cfg(DEVELOPMENT)]
pub fn debug_print(
        format: *const u8,
        args:   &[&str])  {
    
    todo!();
        /*
        
        */
}

#[cfg(DEVELOPMENT)]
pub fn debug_break(
        format: *const u8,
        args:   &[&str])  {
    
    todo!();
        /*
        
        */
}

#[cfg(DEVELOPMENT)]
pub fn print_last_error(
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
        
        */
}


/**
  | @name Provide a custom assertion handler
  | and debug print handler, eg so that we
  | can provide an assert with a custom dialog,
  | or redirect the debug output to a file
  | or stream.
  |
  */

#[cfg(DEVELOPMENT)]
pub type assertion_handler  = fn(message: *const u8) -> bool;

#[cfg(DEVELOPMENT)]
pub type debug_print_logger = fn(message: *const u8) -> void;

#[cfg(DEVELOPMENT)]
lazy_static!{
    /*
    extern AssertionHandler gAssertionHandler;
    extern AssertionHandler gPreAssertionHook;
    extern DebugPrintLogger gDebugPrintLogger;
    */
}

/**
  | Definition of memory allocation macros:
  | 
  | Use "NEW" to allocate storage for individual
  | objects.
  | 
  | Use "NEWVEC" to allocate storage for
  | an array of objects.
  |
  */
#[cfg(DEVELOPMENT)]
lazy_static!{
    /*
    #if SMTG_OS_MACOS
    void* operator new (size_t, int, const char*, int);
    void* operator new[] (size_t, int, const char*, int);
    void operator delete (void* p, int, const char* file, int line);
    void operator delete[] (void* p, int, const char* file, int line);
    #ifndef NEW
    #define NEW new (1, __FILE__, __LINE__)
    #define NEWVEC new (1, __FILE__, __LINE__)
    #endif

    #define DEBUG_NEW DEBUG_NEW_LEAKS

    #elif SMTG_OS_WINDOWS && defined(_MSC_VER)
    #ifndef NEW
    void* operator new (size_t, int, const char*, int);
    #define NEW new (1, __FILE__, __LINE__)
    #define NEWVEC new (1, __FILE__, __LINE__)
    #endif

    #else
    #ifndef NEW
    #define NEW new
    #define NEWVEC new
    #endif
    #endif
    */
}

/*
  | if DEVELOPMENT is not set, these macros
  | will do nothing.
  |
  */

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_assert        { ($f:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_warning       { ($s:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_printsyserror { () => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_debugstr      { ($s:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_verify {
    ($f:ident) => {
        /*
                f;
        */
    }
}

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_verify_is {
    ($f:ident, $r:ident) => {
        /*
                f;
        */
    }
}

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_verify_not {
    ($f:ident, $r:ident) => {
        /*
                f;
        */
    }
}

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt0 { ($a:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt1 { ($a:ident, $b:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt2 { ($a:ident, $b:ident, $c:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt3 { ($a:ident, $b:ident, $c:ident, $d:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt4 { ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => { } }

#[cfg(not(DEVELOPMENT))]
macro_rules! smtg_dbprt5 { ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => { } }

#[cfg(not(DEVELOPMENT))]
lazy_static!{
    /*
    #ifndef NEW
    #define NEW new
    #define NEWVEC new
    #endif
    */
}

///---------------------------------------
// replace #if SMTG_CPPUNIT_TESTING

#[cfg(not(SMTG_RENAME_ASSERT))]
lazy_static!{
    /*
    #define ASSERT              SMTG_ASSERT
    #define WARNING             SMTG_WARNING
    #define DEBUGSTR            SMTG_DEBUGSTR
    #define VERIFY              SMTG_VERIFY
    #define VERIFY_IS           SMTG_VERIFY_IS
    #define VERIFY_NOT          SMTG_VERIFY_NOT
    #define PRINTSYSERROR       SMTG_PRINTSYSERROR

    #define DBPRT0              SMTG_DBPRT0
    #define DBPRT1              SMTG_DBPRT1
    #define DBPRT2              SMTG_DBPRT2
    #define DBPRT3              SMTG_DBPRT3
    #define DBPRT4              SMTG_DBPRT4
    #define DBPRT5              SMTG_DBPRT5
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fdebug.cpp]

#[cfg(SMTG_OS_WINDOWS)]
pub fn am_ibeing_debugged() -> bool {
    
    todo!();
        /*
            return IsDebuggerPresent ();
        */
}

#[cfg(SMTG_OS_LINUX)]
pub fn am_ibeing_debugged() -> bool {
    
    todo!();
        /*
            // TODO: check if GDB or LLDB is attached
        return true;
        */
}

/**
  | Returns true if the current process is being
  | debugged (either running under the debugger or
  | has a debugger attached post facto).
  |
  | from Technical Q&A QA1361
  | (http://developer.apple.com/qa/qa2004/qa1361.html)
  */
#[cfg(SMTG_OS_MACOS)]
pub fn am_ibeing_debugged() -> bool {
    
    todo!();
        /*
            int mib[4];
        struct kinfo_proc info;
        size_t size;

        // Initialize the flags so that, if sysctl fails for some bizarre
        // reason, we get a predictable result.

        info.kp_proc.p_flag = 0;

        // Initialize mib, which tells sysctl the info we want, in this case
        // we're looking for information about a specific process ID.

        mib[0] = CTL_KERN;
        mib[1] = KERN_PROC;
        mib[2] = KERN_PROC_PID;
        mib[3] = getpid ();

        // Call sysctl.

        size = sizeof (info);
        sysctl (mib, sizeof (mib) / sizeof (*mib), &info, &size, NULL, 0);

        // We're being debugged if the P_TRACED flag is set.
        return ((info.kp_proc.p_flag & P_TRACED) != 0);
        */
}

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_WINDOWS)]
#[cfg(not(_WIN32_WINNT))]
pub const _WIN32_WINNT: usize = 0x0400;

/**
   check allocations on specific threads
  */
#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
pub const THREAD_ALLOC_WATCH: usize = 0;

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
#[cfg(THREAD_ALLOC_WATCH)]
lazy_static!{
    /*
    mach_port_t watchThreadID = 0;
    */
}

#[cfg(DEVELOPMENT)]
lazy_static!{
    /*
    AssertionHandler gAssertionHandler = nullptr;
    AssertionHandler gPreAssertionHook = nullptr;
    DebugPrintLogger gDebugPrintLogger = nullptr;
    */
}

//--------------------------------------------------------------------------
#[cfg(DEVELOPMENT)]
pub const k_debug_printf_buffer_size: i32 = 10000;

/**
   so I can switch it off in the debugger...
  */
#[cfg(DEVELOPMENT)]
lazy_static!{
    /*
    static bool neverDebugger = false;
    */
}

#[cfg(DEVELOPMENT)]
pub fn print_debug_string(string: *const u8)  {
    
    todo!();
        /*
            if (!string)
            return;

        if (gDebugPrintLogger)
        {
            gDebugPrintLogger (string);
        }
        else
        {
    #if SMTG_OS_MACOS || defined(__MINGW32__)
            fprintf (stderr, "%s", string);
    #elif SMTG_OS_WINDOWS
            OutputDebugStringA (string);
    #endif
        }
        */
}


/* --------- printf style debugging output  --------- */

#[cfg(DEVELOPMENT)]
pub fn debug_print(
        format: *const u8,
        args:   &[&str])  {
    
    todo!();
        /*
            char string[kDebugPrintfBufferSize];
        va_list marker;
        va_start (marker, format);
        vsnprintf (string, kDebugPrintfBufferSize, format, marker);

        printDebugString (string);
        */
}

#[cfg(DEVELOPMENT)]
pub fn debug_break(
        format: *const u8,
        args:   &[&str])  {
    
    todo!();
        /*
            char string[kDebugPrintfBufferSize];
        va_list marker;
        va_start (marker, format);
        vsnprintf (string, kDebugPrintfBufferSize, format, marker);

        printDebugString (string);

        // The Pre-assertion hook is always called, even if we're not running in the debugger,
        // so that we can log asserts without displaying them
        if (gPreAssertionHook)
        {
            gPreAssertionHook (string);
        }

        if (neverDebugger)
            return;
        if (AmIBeingDebugged ())
        {
            // do not crash if no debugger present
            // If there  is an assertion handler defined then let this override the UI
            // and tell us whether we want to break into the debugger
            bool breakIntoDebugger = true;
            if (gAssertionHandler && gAssertionHandler (string) == false)
            {
                breakIntoDebugger = false;
            }

            if (breakIntoDebugger)
            {
    #if SMTG_OS_WINDOWS && _MSC_VER
                __debugbreak (); // intrinsic version of DebugBreak()
    #elif SMTG_OS_MACOS && __arm64__
                raise (SIGSTOP);

    #elif __ppc64__ || __ppc__ || __arm__
                kill (getpid (), SIGINT);
    #elif __i386__ || __x86_64__
                {
                    __asm__ volatile ("int3");
                }
    #endif
            }
        }
        */
}

#[cfg(DEVELOPMENT)]
pub fn print_last_error(
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            #if SMTG_OS_WINDOWS
        LPVOID lpMessageBuffer;
        FormatMessageA (FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM, nullptr,
                        GetLastError (), MAKELANGID (LANG_NEUTRAL, SUBLANG_DEFAULT),
                        (LPSTR)&lpMessageBuffer, 0, nullptr);
        FDebugPrint ("%s(%d) : %s\n", file, line, lpMessageBuffer);
        LocalFree (lpMessageBuffer);
    #endif

    #if SMTG_OS_MACOS
    #if !__MACH__
        extern int errno;
    #endif
        FDebugPrint ("%s(%d) : Errno %d\n", file, line, errno);
    #endif
        */
}

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
pub fn operator_new(
        size: usize,
        _1:   i32,
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            #if THREAD_ALLOC_WATCH
        mach_port_t threadID = mach_thread_self ();
        if (watchThreadID == threadID)
        {
            FDebugPrint ("Watched Thread Allocation : %s (Line:%d)\n", file ? file : "Unknown", line);
        }
    #endif
        try
        {
            return ::operator new (size);
        }
        catch (std::bad_alloc exception)
        {
            FDebugPrint ("bad_alloc exception : %s (Line:%d)", file ? file : "Unknown", line);
        }
        return (void*)-1;
        */
}

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
pub fn operator_new_arr(
        size: usize,
        _1:   i32,
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            #if THREAD_ALLOC_WATCH
        mach_port_t threadID = mach_thread_self ();
        if (watchThreadID == threadID)
        {
            FDebugPrint ("Watched Thread Allocation : %s (Line:%d)\n", file ? file : "Unknown", line);
        }
    #endif
        try
        {
            return ::operator new[] (size);
        }
        catch (std::bad_alloc exception)
        {
            FDebugPrint ("bad_alloc exception : %s (Line:%d)", file ? file : "Unknown", line);
        }
        return (void*)-1;
        */
}

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
pub fn operator_delete(
        p:    *mut c_void,
        _1:   i32,
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            ::operator delete (p);
        */
}

#[cfg(DEVELOPMENT)]
#[cfg(SMTG_OS_MACOS)]
pub fn operator_delete_arr(
        p:    *mut c_void,
        _1:   i32,
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            ::operator delete[] (p);
        */
}

/**
   ugly hack to unit testing ...
  */
lazy_static!{
    /*
    static bool smtg_unit_testing_active = false;
    */
}

pub fn is_smtg_unit_testing() -> bool {
    
    todo!();
        /*
            return smtg_unit_testing_active;
        */
}

pub fn set_smtg_unit_testing()  {
    
    todo!();
        /*
            smtg_unit_testing_active = true;
        */
}
