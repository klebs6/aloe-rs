crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_linux_Messaging.cpp]

pub struct InternalMessageQueue {
    lock:            CriticalSection,
    queue:           ReferenceCountedArray<Box<dyn MessageBaseInterface>>,
    msgpipe:         [i32; 2],
    bytes_in_socket: i32, // default = 0
}

pub mod internal_message_queue {

    use super::*;
    
    aloe_declare_singleton!{
        InternalMessageQueue, false
    }

    lazy_static!{
        /*
        static constexpr int maxBytesInSocketQueue = 128;
        */
    }
}

aloe_implement_singleton!{
    InternalMessageQueue
}

impl Drop for InternalMessageQueue {
    fn drop(&mut self) {
        todo!();
        /* 
            LinuxEventLoop::unregisterFdCallback (getReadHandle());

            close (getReadHandle());
            close (getWriteHandle());

            clearSingletonInstance();
         */
    }
}

impl Default for InternalMessageQueue {
    
    fn default() -> Self {
        todo!();
        /*


            auto err = ::socketpair (AF_LOCAL, SOCK_STREAM, 0, msgpipe);
            jassertquiet (err == 0);

            LinuxEventLoop::registerFdCallback (getReadHandle(),
                                                [this] (int fd)
                                                {
                                                    while (auto msg = popNextMessage (fd))
                                                    {
                                                        ALOE_TRY
                                                        {
                                                            msg->messageCallback();
                                                        }
                                                        ALOE_CATCH_EXCEPTION
                                                    }
                                                })
        */
    }
}

impl InternalMessageQueue {

    pub fn post_message(&mut self, msg: *mut dyn MessageBaseInterface)  {
        
        todo!();
        /*
            ScopedLock sl (lock);
            queue.add (msg);

            if (bytesInSocket < maxBytesInSocketQueue)
            {
                bytesInSocket++;

                ScopedUnlock ul (lock);
                unsigned char x = 0xff;
                auto numBytes = write (getWriteHandle(), &x, 1);
                ignoreUnused (numBytes);
            }
        */
    }
    
    pub fn get_write_handle(&self) -> i32 {
        
        todo!();
        /*
            return msgpipe[0];
        */
    }
    
    pub fn get_read_handle(&self) -> i32 {
        
        todo!();
        /*
            return msgpipe[1];
        */
    }
    
    pub fn pop_next_message(&mut self, fd: i32) -> Box<dyn MessageBaseInterface> {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (bytesInSocket > 0)
            {
                --bytesInSocket;

                ScopedUnlock ul (lock);
                unsigned char x;
                auto numBytes = read (fd, &x, 1);
                ignoreUnused (numBytes);
            }

            return queue.removeAndReturn (0);
        */
    }
}

///----------------------------
pub struct InternalRunLoop {
    lock:                                  CriticalSection,
    fd_read_callbacks:                     Vec<(i32,fn(_0: i32) -> ())>,
    pfds:                                  Vec<libc::pollfd>,
    should_defer_modifying_read_callbacks: bool, // default = false
    deferred_read_callback_modifications:  Vec<fn() -> ()>,
}

aloe_declare_singleton!{
    InternalRunLoop, false
}

aloe_implement_singleton!{
    InternalRunLoop
}

impl Default for InternalRunLoop {
    
    fn default() -> Self {
        todo!();
        /*


            fdReadCallbacks.reserve (16)
        */
    }
}

impl InternalRunLoop {

    pub fn register_fd_callback(&mut self, 
        fd:         i32,
        cb:         fn(_0: i32) -> (),
        event_mask: i16)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (shouldDeferModifyingReadCallbacks)
            {
                deferredReadCallbackModifications.emplace_back ([this, fd, cb, eventMask]() mutable
                                                                {
                                                                    registerFdCallback (fd, std::move (cb), eventMask);
                                                                });
                return;
            }

            fdReadCallbacks.push_back ({ fd, std::move (cb) });
            pfds.push_back ({ fd, eventMask, 0 });
        */
    }
    
    pub fn unregister_fd_callback(&mut self, fd: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (shouldDeferModifyingReadCallbacks)
            {
                deferredReadCallbackModifications.emplace_back ([this, fd] { unregisterFdCallback (fd); });
                return;
            }

            {
                auto removePredicate = [=] (const std::pair<int, std::function<void (int)>>& cb)  { return cb.first == fd; };

                fdReadCallbacks.erase (std::remove_if (std::begin (fdReadCallbacks), std::end (fdReadCallbacks), removePredicate),
                                       std::end (fdReadCallbacks));
            }

            {
                auto removePredicate = [=] (const pollfd& pfd)  { return pfd.fd == fd; };

                pfds.erase (std::remove_if (std::begin (pfds), std::end (pfds), removePredicate),
                            std::end (pfds));
            }
        */
    }
    
    pub fn dispatch_pending_events(&mut self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (poll (&pfds.front(), static_cast<nfds_t> (pfds.size()), 0) == 0)
                return false;

            bool eventWasSent = false;

            for (auto& pfd : pfds)
            {
                if (pfd.revents == 0)
                    continue;

                pfd.revents = 0;

                auto fd = pfd.fd;

                for (auto& fdAndCallback : fdReadCallbacks)
                {
                    if (fdAndCallback.first == fd)
                    {
                        {
                            ScopedValueSetter<bool> insideFdReadCallback (shouldDeferModifyingReadCallbacks, true);
                            fdAndCallback.second (fd);
                        }

                        if (! deferredReadCallbackModifications.empty())
                        {
                            for (auto& deferredRegisterEvent : deferredReadCallbackModifications)
                                deferredRegisterEvent();

                            deferredReadCallbackModifications.clear();

                            // elements may have been removed from the fdReadCallbacks/pfds array so we really need
                            // to call poll again
                            return true;
                        }

                        eventWasSent = true;
                    }
                }
            }

            return eventWasSent;
        */
    }
    
    pub fn sleep_until_next_event(&mut self, timeout_ms: i32)  {
        
        todo!();
        /*
            poll (&pfds.front(), static_cast<nfds_t> (pfds.size()), timeoutMs);
        */
    }
    
    pub fn get_fd_read_callbacks(&mut self) -> Vec<(i32,fn(_0: i32) -> ())> {
        
        todo!();
        /*
            const ScopedLock sl (lock);
            return fdReadCallbacks;
        */
    }
}

pub mod linux_error_handling {
    use super::*;

    lazy_static!{
        /*
        static bool keyboardBreakOccurred = false;
        */
    }

    pub fn keyboard_break_signal_handler(sig: i32)  {
        
        todo!();
        /*
            if (sig == SIGINT)
                    keyboardBreakOccurred = true;
        */
    }

    pub fn install_keyboard_break_handler()  {
        
        todo!();
        /*
            struct sigaction saction;
                sigset_t maskSet;
                sigemptyset (&maskSet);
                saction.sa_handler = keyboardBreakSignalHandler;
                saction.sa_mask = maskSet;
                saction.sa_flags = 0;
                sigaction (SIGINT, &saction, nullptr);
        */
    }
}

impl MessageManager {
    
    pub fn do_platform_specific_initialisation(&mut self)  {
        
        todo!();
        /*
            if (ALOEApplicationBase::isStandaloneApp())
            LinuxErrorHandling::installKeyboardBreakHandler();

        InternalRunLoop::getInstance();
        InternalMessageQueue::getInstance();
        */
    }
    
    pub fn do_platform_specific_shutdown(&mut self)  {
        
        todo!();
        /*
            InternalMessageQueue::deleteInstance();
        InternalRunLoop::deleteInstance();
        */
    }
    
    pub fn post_message_to_system_queue(&mut self, message: *mut dyn MessageBaseInterface) -> bool {
        
        todo!();
        /*
            if (auto* queue = InternalMessageQueue::getInstanceWithoutCreating())
        {
            queue->postMessage (message);
            return true;
        }

        return false;
        */
    }
}

/**
  | this function expects that it will NEVER
  | be called simultaneously for two concurrent
  | threads
  |
  */
pub fn dispatch_next_message_on_system_queue(return_if_no_pending_messages: bool) -> bool {
    
    todo!();
    /*
        for (;;)
        {
            if (LinuxErrorHandling::keyboardBreakOccurred)
                ALOEApplicationBase::quit();

            if (auto* runLoop = InternalRunLoop::getInstanceWithoutCreating())
            {
                if (runLoop->dispatchPendingEvents())
                    break;

                if (returnIfNoPendingMessages)
                    return false;

                runLoop->sleepUntilNextEvent (2000);
            }
        }

        return true;
    */
}

//TODO
pub struct LinuxEventLoop;

impl LinuxEventLoop {

    pub fn register_fd_callback(&mut self, 
        fd:            i32,
        read_callback: fn(_0: i32) -> (),
        event_mask:    i16)  {
        
        todo!();
        /*
            if (auto* runLoop = InternalRunLoop::getInstanceWithoutCreating())
            runLoop->registerFdCallback (fd, std::move (readCallback), eventMask);
        */
    }
    
    pub fn unregister_fd_callback(&mut self, fd: i32)  {
        
        todo!();
        /*
            if (auto* runLoop = InternalRunLoop::getInstanceWithoutCreating())
            runLoop->unregisterFdCallback (fd);
        */
    }
}

pub fn get_fd_read_callbacks() -> Vec<(i32,fn(_0: i32) -> ())> {
    
    todo!();
    /*
        using namespace aloe;

        if (auto* runLoop = InternalRunLoop::getInstanceWithoutCreating())
            return runLoop->getFdReadCallbacks();

        jassertfalse;
        return {};
    */
}
