crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_MessageManager.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_MessageManager.cpp]

//---------------------------------
#[derive(Default)]
#[no_copy]
pub struct QuitMessage;

impl MessageBaseInterface for QuitMessage {

    fn message_callback(&mut self)  {

        todo!();
        /*
           if (auto* mm = MessageManager::instance)
           mm->quitMessageReceived = true;
           */
    }
}

/**
  | See MessageManager::callFunctionOnMessageThread()
  | for use of this function type.
  |
  */
pub type MessageCallbackFunction = fn(user_data: *mut c_void) -> *mut c_void;

/**
  | This class is in charge of the application's
  | event-dispatch loop.
  |
  | @see Message, CallbackMessage,
  | MessageManagerLock, ALOEApplication,
  | ALOEApplicationBase
  |
  | @tags{Events}
  */
#[no_copy]
#[leak_detector]
pub struct MessageManager {
    broadcaster:           Box<ActionBroadcaster>,
    quit_message_posted:   AtomicI32, // default = 0 
    quit_message_received: AtomicI32, // default = 0 
    message_thread_id:     ThreadID,
    thread_with_lock:      Atomic<ThreadID>,
}

lazy_static!{
    /*
    static MessageManager* MessageManager::instance = nullptr;
    */
}

impl MessageManager {

    /**
      | Returns true if the stopDispatchLoop()
      | method has been called.
      |
      */
    pub fn has_stop_message_been_sent(&self) -> bool {
        
        todo!();
        /*
            return quitMessagePosted.get() != 0;
        */
    }

    /**
      | Synchronously dispatches messages
      | until a given time has elapsed.
      | 
      | Returns false if a quit message has been
      | posted by a call to stopDispatchLoop(),
      | otherwise returns true.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn run_dispatch_loop_until(&mut self, milliseconds_to_run_for: i32) -> bool {

        #[cfg(not(any(any(target_os="macos",target_os="ios"),target_os="android")))]
        fn _run_dispatch_loop_until(&mut self, milliseconds_to_run_for: i32) -> bool {
            
            todo!();
            /*
                jassert (isThisTheMessageThread()); // must only be called by the message thread

            auto endTime = Time::currentTimeMillis() + millisecondsToRunFor;

            while (quitMessageReceived.get() == 0)
            {
                ALOE_TRY
                {
                    if (! dispatchNextMessageOnSystemQueue (millisecondsToRunFor >= 0))
                        Thread::sleep (1);
                }
                ALOE_CATCH_EXCEPTION

                if (millisecondsToRunFor >= 0 && Time::currentTimeMillis() >= endTime)
                    break;
            }

            return quitMessageReceived.get() == 0;
            */
        }
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the ID of the current message
      | thread, as set by setCurrentThreadAsMessageThread().
      | 
      | (Best to ignore this method unless you
      | really know what you're doing..) @see
      | setCurrentThreadAsMessageThread
      |
      */
    pub fn get_current_message_thread(&self) -> ThreadID {
        
        todo!();
        /*
            return messageThreadId;
        */
    }

    /**
      | Sends a message to all other Aloe applications
      | that are running.
      | 
      | -----------
      | @param messageText
      | 
      | the string that will be passed to the
      | actionListenerCallback() method
      | of the broadcast listeners in the other
      | app. @see registerBroadcastListener,
      | ActionListener
      |
      */
    pub fn broadcast_message(&mut self, _0: &String)  {
        
        todo!();
        /*
            // TODO
        */
    }
}

/** 
  | Used to make sure that the calling thread has
  | exclusive access to the message loop.
  |
  | Because it's not thread-safe to call any of
  | the Component or other UI classes from threads
  | other than the message thread, one of these
  | objects can be used to lock the message loop
  | and allow this to be done. The message thread
  | will be suspended for the lifetime of the
  | MessageManagerLock object, so create one on
  | the stack like this: 
  |
  |   @code
  |
  |   void MyThread::run()
  |   {
  |       someData = 1234;
  |
  |       const MessageManagerLock mmLock;
  |       // the event loop will now be locked so it's safe to make a few calls..
  |
  |       myComponent->setBounds (newBounds);
  |       myComponent->repaint();
  |
  |       // ..the event loop will now be unlocked as the MessageManagerLock goes out of scope
  |   }
  |   @endcode
  |
  |   Obviously be careful not to create one of
  |   these and leave it lying around, or your app
  |   will grind to a halt!
  |
  |   MessageManagerLocks are re-entrant, so can be
  |   safely nested if the current thread already
  |   has the lock.
  |
  |   Another caveat is that using this in
  |   conjunction with other CriticalSections can
  |   create lots of interesting ways of producing
  |   a deadlock! In particular, if your message
  |   thread calls stopThread() for a thread that
  |   uses these locks, you'll get an (occasional)
  |   deadlock..
  |
  |   @see MessageManager,
  |   MessageManager::currentThreadHasLockedMessageManager
  |
  |   @tags{Events}
  */
#[no_copy]
pub struct MessageManagerLock {
    base:    Box<dyn ThreadListener>,
    mm_lock: MessageManagerLockPimpl,
    locked:  bool,
}

impl Drop for MessageManagerLock {

    /**
      | Releases the current thread's lock
      | on the message manager.
      | 
      | Make sure this object is created and
      | deleted by the same thread, otherwise
      | there are no guarantees what will happen!
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      mmLock.exit();  */
    }
}


/**
  | This macro is used to catch unsafe use
  | of functions which expect to only be
  | called on the message thread, or when
  | a MessageManagerLock is in place.
  | 
  | It will also fail if you try to use the
  | function before the message manager
  | has been created, which could happen
  | if you accidentally invoke it during
  | a static constructor.
  |
  */
macro_rules! aloe_assert_message_manager_is_locked {
    () => {
        /*
        
            jassert (MessageManager::existsAndIsLockedByCurrentThread());
        */
    }
}

/**
  | This macro is used to catch unsafe use
  | of functions which expect to only be
  | called on the message thread.
  | 
  | It will also fail if you try to use the
  | function before the message manager
  | has been created, which could happen
  | if you accidentally invoke it during
  | a static constructor.
  |
  */
macro_rules! aloe_assert_message_thread {
    () => {
        /*
        
            jassert (MessageManager::existsAndIsCurrentThread());
        */
    }
}

/**
  | This macro is used to catch unsafe use
  | of functions which expect to not be called
  | outside the lifetime of the MessageManager.
  |
  */
macro_rules! aloe_assert_message_manager_exists {
    () => {
        /*
        
            jassert (MessageManager::getInstanceWithoutCreating() != nullptr);
        */
    }
}

/**
   implemented in platform-specific code
   (aloe_linux_Messaging.cpp and
   aloe_win32_Messaging.cpp)
  */
#[cfg(not(any(any(target_os="macos",target_os="ios"),target_os="android")))]
pub fn dispatch_next_message_on_system_queue(return_if_no_pending_messages: bool) -> bool {
    
    todo!();
    /*
    
    */
}

///------------------------------
impl Drop for MessageManager {

    fn drop(&mut self) {
        todo!();
        /* 
        broadcaster.reset();

        doPlatformSpecificShutdown();

        jassert (instance == this);
        instance = nullptr;  // do this last in case this instance is still needed by doPlatformSpecificShutdown()
         */
    }
}

impl Default for MessageManager {

    fn default() -> Self {
    
        todo!();
        /*


            : messageThreadId (Thread::getCurrentThreadId())
        if (ALOEApplicationBase::isStandaloneApp())
            Thread::setCurrentThreadName ("Aloe Message Thread");
        */
    }
}

impl MessageManager {
    
    /**
      | Returns the global instance of the MessageManager.
      |
      */
    pub fn get_instance(&mut self) -> *mut MessageManager {
        
        todo!();
        /*
            if (instance == nullptr)
        {
            instance = new MessageManager();
            doPlatformSpecificInitialisation();
        }

        return instance;
        */
    }
    
    /**
      | Returns the global instance of the MessageManager,
      | or nullptr if it doesn't exist.
      |
      */
    pub fn get_instance_without_creating(&mut self) -> *mut MessageManager {
        
        todo!();
        /*
            return instance;
        */
    }
    
    /**
      | Deletes the global MessageManager
      | instance.
      | 
      | Does nothing if no instance had been
      | created.
      |
      */
    pub fn delete_instance(&mut self)  {
        
        todo!();
        /*
            deleteAndZero (instance);
        */
    }
    
    /**
      | Runs the event dispatch loop until a
      | stop message is posted.
      | 
      | This method is only intended to be run
      | by the application's startup routine,
      | as it blocks, and will only return after
      | the stopDispatchLoop() method has
      | been used.
      | 
      | @see stopDispatchLoop
      |
      */
    #[cfg(not(any(any(target_os="macos",target_os="ios"),target_os="android")))]
    pub fn run_dispatch_loop(&mut self)  {
        
        todo!();
        /*
            jassert (isThisTheMessageThread()); // must only be called by the message thread

        while (quitMessageReceived.get() == 0)
        {
            ALOE_TRY
            {
                if (! dispatchNextMessageOnSystemQueue (false))
                    Thread::sleep (1);
            }
            ALOE_CATCH_EXCEPTION
        }
        */
    }
    
    /**
      | Sends a signal that the dispatch loop
      | should terminate.
      | 
      | After this is called, the runDispatchLoop()
      | or runDispatchLoopUntil() methods
      | will be interrupted and will return.
      | 
      | @see runDispatchLoop
      |
      */
    #[cfg(not(any(any(target_os="macos",target_os="ios"),target_os="android")))]
    pub fn stop_dispatch_loop(&mut self)  {
        
        todo!();
        /*
            (new QuitMessage())->post();
        quitMessagePosted = true;
        */
    }

}

///--------------------------------------------
#[no_copy]
pub struct AsyncFunctionCallback {
    finished:  WaitableEvent,
    result:    Atomic<*mut c_void>, // default = nullptr 
    func:      *const MessageCallbackFunction,
    parameter: *const c_void,
}

impl MessageBaseInterface for AsyncFunctionCallback {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            result = (*func) (parameter);
            finished.signal();
        */
    }
}

impl AsyncFunctionCallback {

    pub fn new(
        f:     *mut MessageCallbackFunction,
        param: *mut c_void) -> Self {
    
        todo!();
        /*
            : func (f), parameter (param)
        */
    }
}

impl MessageManager {

    /**
      | Calls a function using the message-thread.
      | 
      | This can be used by any thread to cause
      | this function to be called-back by the
      | message thread. If it's the message-thread
      | that's calling this method, then the
      | function will just be called; if another
      | thread is calling, a message will be
      | posted to the queue, and this method
      | will block until that message is delivered,
      | the function is called, and the result
      | is returned.
      | 
      | Be careful not to cause any deadlocks
      | with this! It's easy to do - e.g. if the
      | caller thread has a critical section
      | locked, which an unrelated message
      | callback then tries to lock before the
      | message thread gets round to processing
      | this callback.
      | 
      | -----------
      | @param callback
      | 
      | the function to call - its signature
      | must be @code void* myCallbackFunction
      | (void*) @endcode
      | ----------
      | @param userData
      | 
      | a user-defined pointer that will be
      | passed to the function that gets called
      | 
      | -----------
      | @return
      | 
      | the value that the callback function
      | returns. @see MessageManagerLock
      |
      */
    pub fn call_function_on_message_thread(&mut self, 
        func:      *mut MessageCallbackFunction,
        parameter: *mut c_void)  {
        
        todo!();
        /*
            if (isThisTheMessageThread())
            return func (parameter);

        // If this thread has the message manager locked, then this will deadlock!
        jassert (! currentThreadHasLockedMessageManager());

        const ReferenceCountedObjectPtr<AsyncFunctionCallback> message (new AsyncFunctionCallback (func, parameter));

        if (message->post())
        {
            message->finished.wait();
            return message->result.load();
        }

        jassertfalse; // the OS message queue failed to send the message!
        return nullptr;
        */
    }
    
    /**
      | Asynchronously invokes a function
      | or C++11 lambda on the message thread.
      | 
      | -----------
      | @return
      | 
      | true if the message was successfully
      | posted to the message queue, or false
      | otherwise.
      |
      */
    pub fn call_async(&mut self, fn_: fn() -> ()) -> bool {
        
        todo!();
        /*
            struct AsyncCallInvoker  : public MessageBase
        {
            AsyncCallInvoker (std::function<void()> f) : callback (std::move (f)) {}
            void messageCallback() override  { callback(); }
            std::function<void()> callback;
        };

        return (new AsyncCallInvoker (std::move (fn)))->post();
        */
    }

    pub fn deliver_broadcast_message(&mut self, value: &String)  {
        
        todo!();
        /*
            if (broadcaster != nullptr)
            broadcaster->sendActionMessage (value);
        */
    }

    /**
      | Registers a listener to get told about
      | broadcast messages.
      | 
      | The actionListenerCallback() callback's
      | string parameter is the message passed
      | into broadcastMessage().
      | 
      | @see broadcastMessage
      |
      */
    pub fn register_broadcast_listener(&mut self, listener: *mut dyn ActionListener)  {
        
        todo!();
        /*
            if (broadcaster == nullptr)
            broadcaster.reset (new ActionBroadcaster());

        broadcaster->addActionListener (listener);
        */
    }

    /**
      | Deregisters a broadcast listener.
      |
      */
    pub fn deregister_broadcast_listener(&mut self, listener: *mut dyn ActionListener)  {
        
        todo!();
        /*
            if (broadcaster != nullptr)
            broadcaster->removeActionListener (listener);
        */
    }

    /**
      | Returns true if the caller-thread is
      | the message thread.
      |
      */
    pub fn is_this_the_message_thread(&self) -> bool {
        
        todo!();
        /*
            return Thread::getCurrentThreadId() == messageThreadId;
        */
    }

    /**
      | Called to tell the manager that the current
      | thread is the one that's running the
      | dispatch loop.
      | 
      | (Best to ignore this method unless you
      | really know what you're doing..) @see
      | getCurrentMessageThread
      |
      */
    pub fn set_current_thread_as_message_thread(&mut self)  {
        
        todo!();
        /*
            auto thisThread = Thread::getCurrentThreadId();

        if (messageThreadId != thisThread)
        {
            messageThreadId = thisThread;

           #if ALOE_WINDOWS
            // This is needed on windows to make sure the message window is created by this thread
            doPlatformSpecificShutdown();
            doPlatformSpecificInitialisation();
           #endif
        }
        */
    }

    /**
      | Returns true if the caller thread has
      | currently got the message manager locked.
      | 
      | see the MessageManagerLock class for
      | more info about this.
      | 
      | This will be true if the caller is the
      | message thread, because that automatically
      | gains a lock while a message is being
      | dispatched.
      |
      */
    pub fn current_thread_has_locked_message_manager(&self) -> bool {
        
        todo!();
        /*
            auto thisThread = Thread::getCurrentThreadId();
        return thisThread == messageThreadId || thisThread == threadWithLock.get();
        */
    }

    /**
      | Returns true if there's an instance
      | of the MessageManager, and if the current
      | thread has the lock on it.
      |
      */
    pub fn exists_and_is_locked_by_current_thread(&mut self) -> bool {
        
        todo!();
        /*
            if (auto i = getInstanceWithoutCreating())
            return i->currentThreadHasLockedMessageManager();

        return false;
        */
    }

    /**
      | Returns true if there's an instance
      | of the MessageManager, and if the current
      | thread is running it.
      |
      */
    pub fn exists_and_is_current_thread(&mut self) -> bool {
        
        todo!();
        /*
            if (auto i = getInstanceWithoutCreating())
            return i->isThisTheMessageThread();

        return false;
        */
    }
}

impl MessageManagerLock {

    /** 
     | Tries to acquire a lock on the message
     | manager.
     |
     | The constructor attempts to gain a lock on
     | the message loop, and the lock will be
     | kept for the lifetime of this object.
     |
     | Optionally, you can pass a thread object
     | here, and while waiting to obtain the
     | lock, this method will keep checking
     | whether the thread has been given the
     | Thread::signalThreadShouldExit()
     | signal. If this happens, then it will
     | return without gaining the lock. If you
     | pass a thread, you must check whether the
     | lock was successful by calling
     | lockWasGained(). If this is false, your
     | thread is being told to die, so you should
     | take evasive action.
     |
     | If you pass nullptr for the thread object,
     | it will wait indefinitely for the lock
     | - be careful when doing this, because it's
     | very easy to deadlock if your message
     | thread attempts to call stopThread() on
     | a thread just as that thread attempts to
     | get the message lock.
     |
     | If the calling thread already has the
     | lock, nothing will be done, so it's safe
     | and quick to use these locks recursively.
     |
     |    E.g.
     |    @code
     |    void run()
     |    {
     |        ...
     |
     |        while (! threadShouldExit())
     |        {
     |            MessageManagerLock mml (Thread::getCurrentThread());
     |
     |            if (! mml.lockWasGained())
     |                return; // another thread is trying to kill us!
     |
     |            ..do some locked stuff here..
     |        }
     |
     |        ..and now the MM is now unlocked..
     |    }
     |    @endcode
     |
    */
    pub fn new_with_thread(thread_to_check: Option<*mut Thread>) -> Self {
    
        todo!();
        /*
        : locked(attemptLock (threadToCheck, nullptr)),

        
        */
    }
    
    /**
      | This has the same behaviour as the other
      | constructor, but takes a ThreadPoolJob
      | instead of a thread.
      | 
      | See the MessageManagerLock (Thread*)
      | constructor for details on how this
      | works.
      |
      */
    pub fn new_with_job(job_to_check: *mut ThreadPoolJob) -> Self {
    
        todo!();
        /*
        : locked(attemptLock (nullptr, jobToCheck)),
        */
    }

    /**
      | Returns true if the lock was successfully
      | acquired. (See the constructor that
      | takes a Thread for more info).
      |
      */
    pub fn lock_was_gained(&self) -> bool {
        
        todo!();
        /*
            return locked;
        */
    }
    
    pub fn attempt_lock(&mut self, 
        thread_to_check: *mut Thread,
        job_to_check:    *mut ThreadPoolJob) -> bool {
        
        todo!();
        /*
            jassert (threadToCheck == nullptr || jobToCheck == nullptr);

        if (threadToCheck != nullptr)
            threadToCheck->addListener (this);

        if (jobToCheck != nullptr)
            jobToCheck->addListener (this);

        // tryEnter may have a spurious abort (return false) so keep checking the condition
        while ((threadToCheck == nullptr || ! threadToCheck->threadShouldExit())
                 && (jobToCheck == nullptr || ! jobToCheck->shouldExit()))
        {
            if (mmLock.tryEnter())
                break;
        }

        if (threadToCheck != nullptr)
        {
            threadToCheck->removeListener (this);

            if (threadToCheck->threadShouldExit())
                return false;
        }

        if (jobToCheck != nullptr)
        {
            jobToCheck->removeListener (this);

            if (jobToCheck->shouldExit())
                return false;
        }

        return true;
        */
    }
    
    pub fn exit_signal_sent(&mut self)  {
        
        todo!();
        /*
            mmLock.abort();
        */
    }
}

pub fn initialise_aloe_gui()  {
    
    todo!();
    /*
        ALOE_AUTORELEASEPOOL
        {
            MessageManager::getInstance();
        }
    */
}

pub fn shutdown_aloe_gui()  {
    
    todo!();
    /*
        ALOE_AUTORELEASEPOOL
        {
            DeletedAtShutdown::deleteAll();
            MessageManager::deleteInstance();
        }
    */
}

lazy_static!{
    /*
    static int numScopedInitInstances = 0;
    */
}

impl Default for ScopedAloeInitialiser_GUI {
    
    /**
      | The constructor simply calls initialiseAloe_GUI().
      |
      */
    fn default() -> Self {
        todo!();
        /*


            if (numScopedInitInstances++ == 0) initialiseAloe_GUI()
        */
    }
}
