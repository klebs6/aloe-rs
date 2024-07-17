crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_Thread.h]

/**
  | A value type used for thread IDs.
  | 
  | @see getCurrentThreadId(), getThreadId()
  |
  */
pub type ThreadID = *mut c_void;

/**
  | Special realtime audio thread priority
  | 
  | This priority will create a high-priority
  | thread which is best suited for realtime
  | audio processing.
  | 
  | Currently, this priority is identical
  | to priority 9, except when building
  | for Android with OpenSL/Oboe support.
  | 
  | In this case, Aloe will ask OpenSL/Oboe
  | to construct a super high priority thread
  | specifically for realtime audio processing.
  | 
  | -----------
  | @note
  | 
  | this priority can only be set **before**
  | the thread has started. Switching to
  | this priority, or from this priority
  | to a different priority, is not supported
  | under Android and will assert.
  | 
  | For best performance this thread should
  | yield at regular intervals and not call
  | any blocking APIs.
  | 
  | @see startThread, setPriority, sleep,
  | WaitableEvent
  |
  */
pub const REALTIME_AUDIO_PRIORITY: isize = -1;

pub trait ThreadInterface {

    /**
      | Must be implemented to perform the thread's
      | actual code.
      | 
      | Remember that the thread must regularly
      | check the threadShouldExit() method
      | whilst running, and if this returns
      | true it should return from the run()
      | method as soon as possible to avoid being
      | forcibly killed.
      | 
      | @see threadShouldExit, startThread
      |
      */
    fn run(&mut self);
}

/**
  | Used to receive callbacks for thread
  | exit calls
  |
  */
pub trait ThreadListener {

    /**
      | Called if Thread::signalThreadShouldExit
      | was called.
      | 
      | @see Thread::threadShouldExit, Thread::addListener,
      | Thread::removeListener
      |
      */
    fn exit_signal_sent(&mut self);
}

/**
  | Encapsulates a thread.
  |
  | Subclasses derive from Thread and implement
  | the run() method, in which they do their
  | business. The thread can then be started with
  | the startThread() method and controlled with
  | various other methods.
  |
  | This class also contains some thread-related
  | static methods, such as sleep(), yield(),
  | getCurrentThreadId() etc.
  |
  | @see CriticalSection, WaitableEvent, Process,
  |      ThreadWithProgressWindow,
  |      MessageManagerLock
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct Thread {
    thread_name:            String,
    thread_handle:          Atomic<*mut c_void>, // default = nullptr 
    thread_id:              Atomic<ThreadID>, // default = {}
    start_stop_lock:        CriticalSection,
    start_suspension_event: WaitableEvent,
    default_event:          WaitableEvent,
    thread_priority:        i32, // default = 5
    thread_stack_size:      usize,
    affinity_mask:          u32, // default = 0
    delete_on_thread_end:   bool, // default = false
    should_exit:            AtomicI32, // default = 0 
    listeners:              ListenerList<Box<dyn ThreadListener>,Array<*mut dyn ThreadListener,CriticalSection>>,

    #[cfg(target_os="android")]
    is_android_realtime_thread: bool, // default = false
}

impl Drop for Thread {

    /**
      | Destructor.
      | 
      | You must never attempt to delete a Thread
      | object while it's still running - always
      | call stopThread() and make sure your
      | thread has stopped before deleting
      | the object. Failing to do so will throw
      | an assertion, and put you firmly into
      | undefined behaviour territory.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        if (deleteOnThreadEnd)
            return;

        /* If your thread class's destructor has been called without first stopping the thread, that
           means that this partially destructed object is still performing some work - and that's
           probably a Bad Thing!

           To avoid this type of nastiness, always make sure you call stopThread() before or during
           your subclass's destructor.
        */
        jassert (! isThreadRunning());

        stopThread (-1);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_Thread.cpp]
impl Thread {
    
    /**
      | Yields the current thread's CPU time-slot
      | and allows a new thread to run.
      | 
      | If there are no other threads of equal
      | or higher priority currently running
      | then this will return immediately and
      | the current thread will continue to
      | run.
      |
      */
    pub fn yield_cpu()  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the name of the thread. This
      | is the name that gets set in the constructor.
      |
      */
    pub fn get_thread_name(&self) -> &String {
        
        todo!();
        /*
            return threadName;
        */
    }

    /**
      | Changes the name of the caller thread.
      | 
      | Different OSes may place different
      | length or content limits on this name.
      |
      */
    pub fn set_current_thread_name(new_thread_name: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Initialises the Aloe subsystem for
      | projects not created by the Proaloer
      | 
      | On Android, Aloe needs to be initialised
      | once before it is used. The Proaloer
      | will automatically generate the necessary
      | java code to do this. However, if you
      | are using Aloe without the Proaloer
      | or are creating a library made with Aloe
      | intended for use in non-Aloe apks, then
      | you must call this method manually once
      | on apk startup.
      | 
      | You can call this method from C++ or directly
      | from java by calling the following java
      | method:
      | 
      | @code com.rmsl.aloe.Java.initialiseALOE
      | (myContext); @endcode
      | 
      | Note that the above java method is only
      | available in Android Studio projects
      | created by the Proaloer. If you need
      | to call this from another type of project
      | then you need to add the following java
      | file to your project:
      | 
      | 
      | @code
      | package com.rmsl.aloe;
      |
      | public class Java
      | {
      |     static { System.loadLibrary ("aloe_jni"); }
      |     public native static void initialiseALOE (Context context);
      | }
      | @endcode
      | 
      | -----------
      | @param jniEnv
      | 
      | this is a pointer to JNI's JNIEnv variable.
      | Any callback from Java into C++ will
      | have this passed in as it's first parameter.
      | ----------
      | @param jContext
      | 
      | this is a jobject referring to your app/service/receiver/
      | provider's Context. Aloe needs this
      | for many of it's internal functions.
      |
      */
    #[cfg(target_os="android")]
    pub fn initialisealoe(
        jni_env:   *mut c_void,
        j_context: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Creates a thread.
      | 
      | When first created, the thread is not
      | running. Use the startThread() method
      | to start it.
      | 
      | -----------
      | @param threadName
      | 
      | The name of the thread which typically
      | appears in debug logs and profiles.
      | ----------
      | @param threadStackSize
      | 
      | The size of the stack of the thread. If
      | this value is zero then the default stack
      | size of the OS will be used.
      |
      */
    pub fn new(
        name:       &String,
        stack_size: Option<usize>) -> Self {

        let stack_size: usize =
            stack_size.unwrap_or(0);
    
        todo!();
        /*
            : threadName (name), threadStackSize (stackSize)
        */
    }
}

/**
  | Use a ref-counted object to hold this shared
  | data, so that it can outlive its static shared
  | pointer when threads are still running during
  | static shutdown.
  */
#[no_copy]
pub struct CurrentThreadHolder {
    base:  ReferenceCountedObject,
    value: ThreadLocalValue<*mut Thread>,
}

impl Default for CurrentThreadHolder {

    fn default() -> Self {
        todo!();
    }
}

type CurrentThreadHolderPtr = ReferenceCountedObjectPtr<CurrentThreadHolder>;

///-----------------------
lazy_static!{
    /*
    static char currentThreadHolderLock [sizeof (SpinLock)]; // (statically initialised to zeros).
    */
}

pub fn cast_to_spin_lock_without_aliasing_warning(s: *mut c_void) -> *mut SpinLock {
    
    todo!();
    /*
        return static_cast<SpinLock*> (s);
    */
}

/**
  | used to wrap the incoming call from the
  | platform-specific code
  |
  */
pub fn aloe_thread_entry_point(user_data: *mut c_void)  {
    
    todo!();
    /*
        static_cast<Thread*> (userData)->threadEntryPoint();
    */
}

pub fn get_current_thread_holder() -> CurrentThreadHolderPtr {
    
    todo!();
    /*
        static CurrentThreadHolder::Ptr currentThreadHolder;
        SpinLock::ScopedLockType lock (*castToSpinLockWithoutAliasingWarning (currentThreadHolderLock));

        if (currentThreadHolder == nullptr)
            currentThreadHolder = new CurrentThreadHolder();

        return currentThreadHolder;
    */
}

impl Thread {
    
    pub fn thread_entry_point(&mut self)  {
        
        todo!();
        /*
            const CurrentThreadHolder::Ptr currentThreadHolder (getCurrentThreadHolder());
        currentThreadHolder->value = this;

        if (threadName.isNotEmpty())
            setCurrentThreadName (threadName);

        if (startSuspensionEvent.wait (10000))
        {
            jassert (getCurrentThreadId() == threadId.get());

            if (affinityMask != 0)
                setCurrentThreadAffinityMask (affinityMask);

            try
            {
                run();
            }
            catch (...)
            {
                jassertfalse; // Your run() method mustn't throw any exceptions!
            }
        }

        currentThreadHolder->value.releaseCurrentThreadStorage();

        // Once closeThreadHandle is called this class may be deleted by a different
        // thread, so we need to store deleteOnThreadEnd in a local variable.
        auto shouldDeleteThis = deleteOnThreadEnd;
        closeThreadHandle();

        if (shouldDeleteThis)
            delete this;
        */
    }
    
    /**
      | Starts the thread running.
      | 
      | This will cause the thread's run() method
      | to be called by a new thread.
      | 
      | If this thread is already running, startThread()
      | won't do anything.
      | 
      | @see stopThread
      |
      */
    pub fn start_thread(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (startStopLock);

        shouldExit = 0;

        if (threadHandle.get() == nullptr)
        {
            launchThread();
            setThreadPriority (threadHandle.get(), threadPriority);
            startSuspensionEvent.signal();
        }
        */
    }
    
    /**
      | Starts the thread with a given priority.
      | 
      | Launches the thread with a given priority,
      | where 0 = lowest, 10 = highest.
      | 
      | If the thread is already running, its
      | priority will be changed.
      | 
      | @see startThread, setPriority, REALTIME_AUDIO_PRIORITY
      |
      */
    pub fn start_thread_with_priority(&mut self, priority: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (startStopLock);

        if (threadHandle.get() == nullptr)
        {
           #if ALOE_ANDROID
            isAndroidRealtimeThread = (priority == REALTIME_AUDIO_PRIORITY);
           #endif

            threadPriority = getAdjustedPriority (priority);
            startThread();
        }
        else
        {
            setPriority (priority);
        }
        */
    }
    
    /**
      | Returns true if the thread is currently
      | active
      |
      */
    pub fn is_thread_running(&self) -> bool {
        
        todo!();
        /*
            return threadHandle.get() != nullptr;
        */
    }

    /**
      | Finds the thread object that is currently
      | running.
      | 
      | -----------
      | @note
      | 
      | the main UI thread (or other non-Aloe
      | threads) don't have a Thread object
      | associated with them, so this will return
      | nullptr.
      |
      */
    pub fn get_current_thread(&mut self) -> *mut Thread {
        
        todo!();
        /*
            return getCurrentThreadHolder()->value.get();
        */
    }
    
    /**
      | Returns the ID of this thread.
      | 
      | That means the ID of this thread object
      | - not of the thread that's calling the
      | method.
      | 
      | This can change when the thread is started
      | and stopped, and will be invalid if the
      | thread's not actually running.
      | 
      | @see getCurrentThreadId
      |
      */
    pub fn get_thread_id(&self) -> ThreadID {
        
        todo!();
        /*
            return threadId.get();
        */
    }
    
    /**
      | Sets a flag to tell the thread it should
      | stop.
      | 
      | Calling this means that the threadShouldExit()
      | method will then return true. The thread
      | should be regularly checking this to
      | see whether it should exit.
      | 
      | If your thread makes use of wait(), you
      | might want to call notify() after calling
      | this method, to interrupt any waits
      | that might be in progress, and allow
      | it to reach a point where it can exit.
      | 
      | @see threadShouldExit, waitForThreadToExit
      |
      */
    pub fn signal_thread_should_exit(&mut self)  {
        
        todo!();
        /*
            shouldExit = 1;
        listeners.call ([] (Listener& l) { l.exitSignalSent(); });
        */
    }
    
    /**
      | Checks whether the thread has been told
      | to stop running.
      | 
      | Threads need to check this regularly,
      | and if it returns true, they should return
      | from their run() method at the first
      | possible opportunity.
      | 
      | @see signalThreadShouldExit, currentThreadShouldExit
      |
      */
    pub fn thread_should_exit(&self) -> bool {
        
        todo!();
        /*
            return shouldExit.get() != 0;
        */
    }
    
    /**
      | Checks whether the current thread has
      | been told to stop running.
      | 
      | On the message thread, this will always
      | return false, otherwise it will return
      | threadShouldExit() called on the current
      | thread.
      | 
      | @see threadShouldExit
      |
      */
    pub fn current_thread_should_exit(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* currentThread = getCurrentThread())
            return currentThread->threadShouldExit();

        return false;
        */
    }
    
    /**
      | Waits for the thread to stop.
      | 
      | This will wait until isThreadRunning()
      | is false or until a timeout expires.
      | 
      | -----------
      | @param timeOutMilliseconds
      | 
      | the time to wait, in milliseconds. If
      | this value is less than zero, it will
      | wait forever.
      | 
      | -----------
      | @return
      | 
      | true if the thread exits, or false if
      | the timeout expires first.
      |
      */
    pub fn wait_for_thread_to_exit(&self, time_out_milliseconds: i32) -> bool {
        
        todo!();
        /*
            // Doh! So how exactly do you expect this thread to wait for itself to stop??
        jassert (getThreadId() != getCurrentThreadId() || getCurrentThreadId() == ThreadID());

        auto timeoutEnd = Time::getMillisecondCounter() + (uint32) timeOutMilliseconds;

        while (isThreadRunning())
        {
            if (timeOutMilliseconds >= 0 && Time::getMillisecondCounter() > timeoutEnd)
                return false;

            sleep (2);
        }

        return true;
        */
    }
    
    /**
      | Attempts to stop the thread running.
      | 
      | This method will cause the threadShouldExit()
      | method to return true and call notify()
      | in case the thread is currently waiting.
      | 
      | Hopefully the thread will then respond
      | to this by exiting cleanly, and the stopThread
      | method will wait for a given time-period
      | for this to happen.
      | 
      | If the thread is stuck and fails to respond
      | after the timeout, it gets forcibly
      | killed, which is a very bad thing to happen,
      | as it could still be holding locks, etc.
      | which are needed by other parts of your
      | program.
      | 
      | -----------
      | @param timeOutMilliseconds
      | 
      | The number of milliseconds to wait for
      | the thread to finish before killing
      | it by force. A negative value in here
      | will wait forever.
      | 
      | -----------
      | @return
      | 
      | true if the thread was cleanly stopped
      | before the timeout, or false if it had
      | to be killed by force. @see signalThreadShouldExit,
      | threadShouldExit, waitForThreadToExit,
      | isThreadRunning
      |
      */
    pub fn stop_thread(&mut self, time_out_milliseconds: i32) -> bool {
        
        todo!();
        /*
            // agh! You can't stop the thread that's calling this method! How on earth
        // would that work??
        jassert (getCurrentThreadId() != getThreadId());

        const ScopedLock sl (startStopLock);

        if (isThreadRunning())
        {
            signalThreadShouldExit();
            notify();

            if (timeOutMilliseconds != 0)
                waitForThreadToExit (timeOutMilliseconds);

            if (isThreadRunning())
            {
                // very bad karma if this point is reached, as there are bound to be
                // locks and events left in silly states when a thread is killed by force..
                jassertfalse;
                Logger::writeToLog ("!! killing thread by force !!");

                killThread();

                threadHandle = nullptr;
                threadId = {};
                return false;
            }
        }

        return true;
        */
    }
    
    /**
      | Add a listener to this thread which will
      | receive a callback when signalThreadShouldExit
      | was called on this thread.
      | 
      | @see signalThreadShouldExit, removeListener
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn ThreadListener)  {
        
        todo!();
        /*
            listeners.add (listener);
        */
    }
    
    /**
      | Removes a listener added with addListener.
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn ThreadListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    /**
      | Changes the thread's priority.
      | 
      | May return false if for some reason the
      | priority can't be changed.
      | 
      | -----------
      | @param priority
      | 
      | the new priority, in the range 0 (lowest)
      | to 10 (highest). A priority of 5 is normal.
      | @see REALTIME_AUDIO_PRIORITY
      |
      */
    pub fn set_priority(&mut self, new_priority: i32) -> bool {
        
        todo!();
        /*
            newPriority = getAdjustedPriority (newPriority);

        // NB: deadlock possible if you try to set the thread prio from the thread itself,
        // so using setCurrentThreadPriority instead in that case.
        if (getCurrentThreadId() == getThreadId())
            return setCurrentThreadPriority (newPriority);

        const ScopedLock sl (startStopLock);

       #if ALOE_ANDROID
        bool isRealtime = (newPriority == REALTIME_AUDIO_PRIORITY);

        // you cannot switch from or to an Android realtime thread once the
        // thread is already running!
        jassert (isThreadRunning() && (isRealtime == isAndroidRealtimeThread));

        isAndroidRealtimeThread = isRealtime;
       #endif

        if ((! isThreadRunning()) || setThreadPriority (threadHandle.get(), newPriority))
        {
            threadPriority = newPriority;
            return true;
        }

        return false;
        */
    }
    
    /**
      | Changes the priority of the caller thread.
      | 
      | Similar to setPriority(), but this
      | static method acts on the caller thread.
      | 
      | May return false if for some reason the
      | priority can't be changed.
      | 
      | @see setPriority
      |
      */
    pub fn set_current_thread_priority(&mut self, new_priority: i32) -> bool {
        
        todo!();
        /*
            return setThreadPriority ({}, getAdjustedPriority (newPriority));
        */
    }
    
    /**
      | Sets the affinity mask for the thread.
      | 
      | This will only have an effect next time
      | the thread is started - i.e. if the thread
      | is already running when called, it'll
      | have no effect.
      | 
      | @see setCurrentThreadAffinityMask
      |
      */
    pub fn set_affinity_mask(&mut self, new_affinity_mask: u32)  {
        
        todo!();
        /*
            affinityMask = newAffinityMask;
        */
    }
    
    pub fn get_adjusted_priority(&mut self, new_priority: i32) -> i32 {
        
        todo!();
        /*
            return jlimit (0, 10, newPriority == REALTIME_AUDIO_PRIORITY ? 9 : newPriority);
        */
    }
    
    /**
      | Suspends the execution of this thread
      | until either the specified timeout
      | period has elapsed, or another thread
      | calls the notify() method to wake it
      | up.
      | 
      | A negative timeout value means that
      | the method will wait indefinitely.
      | 
      | -----------
      | @return
      | 
      | true if the event has been signalled,
      | false if the timeout expires.
      |
      */
    pub fn wait(&self, time_out_milliseconds: i32) -> bool {
        
        todo!();
        /*
            return defaultEvent.wait (timeOutMilliseconds);
        */
    }
    
    /**
      | Wakes up the thread.
      | 
      | If the thread has called the wait() method,
      | this will wake it up.
      | 
      | @see wait
      |
      */
    pub fn notify(&self)  {
        
        todo!();
        /*
            defaultEvent.signal();
        */
    }
}

///------------------------------
#[no_copy]
#[leak_detector]
pub struct LambdaThread {
    base: Thread,
    fn_:  fn() -> (),
}

impl LambdaThread {

    pub fn new(f: fn() -> ()) -> Self {
    
        todo!();
        /*
            : Thread ("anonymous"), fn (f)
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            fn();
            fn = nullptr; // free any objects that the lambda might contain while the thread is still active
        */
    }
}

///-----------------
impl Thread {
    
    /**
      | Invokes a lambda or function on its own
      | thread.
      | 
      | This will spin up a Thread object which
      | calls the function and then exits. Bear
      | in mind that starting and stopping a
      | thread can be a fairly heavyweight operation,
      | so you might prefer to use a ThreadPool
      | if you're kicking off a lot of short background
      | tasks.
      | 
      | Also note that using an anonymous thread
      | makes it very difficult to interrupt
      | the function when you need to stop it,
      | e.g. when your app quits. So it's up to
      | you to deal with situations where the
      | function may fail to stop in time.
      |
      */
    pub fn launch(&mut self, function_to_run: fn() -> ())  {
        
        todo!();
        /*
            auto anon = new LambdaThread (functionToRun);
        anon->deleteOnThreadEnd = true;
        anon->startThread();
        */
    }
}

#[cfg(feature = "aloe_posix")]
pub fn thread_entry_proc(user_data: *mut c_void)  {
    
    todo!();
    /*
        auto* myself = static_cast<Thread*> (userData);

        ALOE_AUTORELEASEPOOL
        {
            aloe_threadEntryPoint (myself);
        }

       #if ALOE_ANDROID
        if (androidJNIJavaVM != nullptr)
        {
            void* env = nullptr;
            androidJNIJavaVM->GetEnv(&env, JNI_VERSION_1_2);

            // only detach if we have actually been attached
            if (env != nullptr)
                androidJNIJavaVM->DetachCurrentThread();
        }
       #endif

        return nullptr;
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(all(all(target_os="android",ALOE_MODULE_AVAILABLE_aloe_audio_devices),any(ALOE_USE_ANDROID_OPENSLES,ALOE_USE_ANDROID_OBOE)))]
pub const ALOE_ANDROID_REALTIME_THREAD_AVAILABLE: usize = 1;

#[cfg(feature = "aloe_posix")]
#[cfg(ALOE_ANDROID_REALTIME_THREAD_AVAILABLE)]
lazy_static!{
    /*
    extern pthread_t aloe_createRealtimeAudioThread (void* (*entry) (void*), void* userPtr);
    */
}

impl Thread {
    
    #[cfg(feature = "aloe_posix")]
    pub fn launch_thread(&mut self)  {
        
        todo!();
        /*
            #if ALOE_ANDROID
        if (isAndroidRealtimeThread)
        {
           #if ALOE_ANDROID_REALTIME_THREAD_AVAILABLE
            threadHandle = (void*) aloe_createRealtimeAudioThread (threadEntryProc, this);
            threadId = (ThreadID) threadHandle.get();

            return;
           #else
            jassertfalse;
           #endif
        }
       #endif

        threadHandle = {};
        pthread_t handle = {};
        pthread_attr_t attr;
        pthread_attr_t* attrPtr = nullptr;

        if (pthread_attr_init (&attr) == 0)
        {
            attrPtr = &attr;
            pthread_attr_setstacksize (attrPtr, threadStackSize);
        }

        if (pthread_create (&handle, attrPtr, threadEntryProc, this) == 0)
        {
            pthread_detach (handle);
            threadHandle = (void*) handle;
            threadId = (ThreadID) threadHandle.get();
        }

        if (attrPtr != nullptr)
            pthread_attr_destroy (attrPtr);
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn close_thread_handle(&mut self)  {
        
        todo!();
        /*
            threadId = {};
        threadHandle = {};
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn kill_thread(&mut self)  {
        
        todo!();
        /*
            if (threadHandle.get() != nullptr)
        {
           #if ALOE_ANDROID
            jassertfalse; // pthread_cancel not available!
           #else
            pthread_cancel ((pthread_t) threadHandle.get());
           #endif
        }
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_current_thread_name(&mut self, name: &String)  {
        
        todo!();
        /*
            #if ALOE_IOS || ALOE_MAC
        ALOE_AUTORELEASEPOOL
        {
            [[NSThread currentThread] setName: aloeStringToNS (name)];
        }
       #elif ALOE_LINUX || ALOE_BSD || ALOE_ANDROID
        #if (ALOE_BSD \
              || (ALOE_LINUX && (__GLIBC__ * 1000 + __GLIBC_MINOR__) >= 2012) \
              || (ALOE_ANDROID && __ANDROID_API__ >= 9))
         pthread_setname_np (pthread_self(), name.toRawUTF8());
        #else
         prctl (PR_SET_NAME, name.toRawUTF8(), 0, 0, 0);
        #endif
       #endif
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn set_thread_priority(&mut self, 
        handle:   *mut c_void,
        priority: i32) -> bool {
        
        todo!();
        /*
            constexpr auto maxInputPriority = 10;
        constexpr auto lowestRealtimePriority = 8;

        struct sched_param param;
        int policy;

        if (handle == nullptr)
            handle = (void*) pthread_self();

        if (pthread_getschedparam ((pthread_t) handle, &policy, &param) != 0)
            return false;

        policy = priority < lowestRealtimePriority ? SCHED_OTHER : SCHED_RR;

        const auto minPriority = sched_get_priority_min (policy);
        const auto maxPriority = sched_get_priority_max (policy);

        param.sched_priority = [&]
        {
            if (policy == SCHED_OTHER)
                return 0;

            return jmap (priority, lowestRealtimePriority, maxInputPriority, minPriority, maxPriority);
        }();

        return pthread_setschedparam ((pthread_t) handle, policy, &param) == 0;
        */
    }
    
    /**
      | Returns an id that identifies the caller
      | thread.
      | 
      | To find the ID of a particular thread
      | object, use getThreadId().
      | 
      | -----------
      | @return
      | 
      | a unique identifier that identifies
      | the calling thread. @see getThreadId
      |
      */
    #[cfg(feature = "aloe_posix")]
    pub fn get_current_thread_id(&mut self) -> thread::ThreadID {
        
        todo!();
        /*
            return (ThreadID) pthread_self();
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    pub fn yield_(&mut self)  {
        
        todo!();
        /*
            sched_yield();
        */
    }
}

/**
  | Remove this macro if you're having problems
  | compiling the cpu affinity calls (the
  | API for these has changed about quite
  | a bit in various Linux versions, and
  | a lot of distros seem to ship with obsolete
  | versions)
  |
  */
#[cfg(feature = "aloe_posix")]
#[cfg(all(CPU_ISSET,not(SUPPORT_AFFINITIES)))]
pub const SUPPORT_AFFINITIES: usize = 1;

impl Thread {
    
    /**
      | Changes the affinity mask for the caller
      | thread.
      | 
      | This will change the affinity mask for
      | the thread that calls this static method.
      | 
      | @see setAffinityMask
      |
      */
    #[cfg(feature = "aloe_posix")]
    pub fn set_current_thread_affinity_mask(&mut self, affinity_mask: u32)  {
        
        todo!();
        /*
            #if SUPPORT_AFFINITIES
        cpu_set_t affinity;
        CPU_ZERO (&affinity);

        for (int i = 0; i < 32; ++i)
            if ((affinityMask & (uint32) (1 << i)) != 0)
                CPU_SET ((size_t) i, &affinity);

       #if (! ALOE_ANDROID) && ((! (ALOE_LINUX || ALOE_BSD)) || ((__GLIBC__ * 1000 + __GLIBC_MINOR__) >= 2004))
        pthread_setaffinity_np (pthread_self(), sizeof (cpu_set_t), &affinity);
       #elif ALOE_ANDROID
        sched_setaffinity (gettid(), sizeof (cpu_set_t), &affinity);
       #else
        // NB: this call isn't really correct because it sets the affinity of the process,
        // (getpid) not the thread (not gettid). But it's included here as a fallback for
        // people who are using ridiculously old versions of glibc
        sched_setaffinity (getpid(), sizeof (cpu_set_t), &affinity);
       #endif

        sched_yield();

       #else
        // affinities aren't supported because either the appropriate header files weren't found,
        // or the SUPPORT_AFFINITIES macro was turned off
        jassertfalse;
        ignoreUnused (affinityMask);
       #endif
        */
    }
    
    /**
      | Suspends the execution of the current
      | thread until the specified timeout
      | period has elapsed (note that this may
      | not be exact).
      | 
      | The timeout period must not be negative
      | and whilst sleeping the thread cannot
      | be woken up so it should only be used for
      | short periods of time and when other
      | methods such as using a WaitableEvent
      | or CriticalSection are not possible.
      |
      */
    #[cfg(feature = "aloe_posix")]
    pub fn sleep(&mut self, millisecs: i32)  {
        
        todo!();
        /*
            struct timespec time;
        time.tv_sec = millisecs / 1000;
        time.tv_nsec = (millisecs % 1000) * 1000000;
        nanosleep (&time, nullptr);
        */
    }
}
