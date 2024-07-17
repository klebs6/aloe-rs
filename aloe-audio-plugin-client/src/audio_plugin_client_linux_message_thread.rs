crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_LinuxMessageThread.h]

/**
  | Implemented in aloe_linux_Messaging.cpp
  |
  */
#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn dispatch_next_message_on_system_queue(return_if_no_pending_messages: bool) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
#[no_copy]
#[no_move]
#[leak_detector]
pub struct MessageThread {
    thread_initialised: WaitableEvent,
    thread:             std::thread,
    should_exit:        AtomicBool, // default = { false  }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Default for MessageThread {
    
    fn default() -> Self {
        todo!();
        /*


            start();
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Drop for MessageThread {
    fn drop(&mut self) {
        todo!();
        /*
            MessageManager::getInstance()->stopDispatchLoop();
            stop();
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl MessageThread {
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if (isRunning())
                stop();

            shouldExit = false;

            thread = std::thread { [this]
            {
                Thread::setCurrentThreadPriority (7);
                Thread::setCurrentThreadName ("Aloe Plugin Message Thread");

                MessageManager::getInstance()->setCurrentThreadAsMessageThread();
                XWindowSystem::getInstance();

                threadInitialised.signal();

                for (;;)
                {
                    if (! dispatchNextMessageOnSystemQueue (true))
                        Thread::sleep (1);

                    if (shouldExit)
                        break;
                }
            } };

            threadInitialised.wait();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (! isRunning())
                return;

            shouldExit = true;
            thread.join();
        */
    }
    
    pub fn is_running(&self) -> bool {
        
        todo!();
        /*
            return thread.joinable();
        */
    }
}
