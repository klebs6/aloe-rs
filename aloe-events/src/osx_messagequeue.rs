crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_osx_MessageQueue.h]

/**
  | An internal message pump class used
  | in OSX and iOS.
  |
  */
pub struct MessageQueue {
    messages:        ReferenceCountedArray<Box<dyn MessageBaseInterface>,CriticalSection>,
    run_loop:        CFRunLoopRef,
    run_loop_source: Box<CFRunLoopSourceRef>,//CFUniquePtr
}

impl Default for MessageQueue {
    
    fn default() -> Self {
        todo!();
        /*


            #if ALOE_IOS
            runLoop = CFRunLoopGetCurrent();
           #else
            runLoop = CFRunLoopGetMain();
           #endif

            CFRunLoopSourceContext sourceContext;
            zerostruct (sourceContext); // (can't use "= { 0 }" on this object because it's typedef'ed as a C struct)
            sourceContext.info = this;
            sourceContext.perform = runLoopSourceCallback;
            runLoopSource.reset (CFRunLoopSourceCreate (kCFAllocatorDefault, 1, &sourceContext));
            CFRunLoopAddSource (runLoop, runLoopSource.get(), kCFRunLoopCommonModes)
        */
    }
}

impl Drop for MessageQueue {

    fn drop(&mut self) {
        todo!();
        /* 
            CFRunLoopRemoveSource (runLoop, runLoopSource.get(), kCFRunLoopCommonModes);
            CFRunLoopSourceInvalidate (runLoopSource.get());
         */
    }
}

impl MessageQueue {
    
    pub fn post(&mut self, message: *mut dyn MessageBaseInterface)  {
        
        todo!();
        /*
            messages.add (message);
            wakeUp();
        */
    }
    
    pub fn wake_up(&mut self)  {
        
        todo!();
        /*
            CFRunLoopSourceSignal (runLoopSource.get());
            CFRunLoopWakeUp (runLoop);
        */
    }
    
    pub fn deliver_next_message(&mut self) -> bool {
        
        todo!();
        /*
            const MessageBaseInterfacePtr nextMessage (messages.removeAndReturn (0));

            if (nextMessage == nullptr)
                return false;

            ALOE_AUTORELEASEPOOL
            {
                ALOE_TRY
                {
                    nextMessage->messageCallback();
                }
                ALOE_CATCH_EXCEPTION
            }

            return true;
        */
    }
    
    pub fn run_loop_callback(&mut self)  {
        
        todo!();
        /*
            for (int i = 4; --i >= 0;)
                if (! deliverNextMessage())
                    return;

            wakeUp();
        */
    }
    
    pub fn run_loop_source_callback(info: *mut c_void)  {
        
        todo!();
        /*
            static_cast<MessageQueue*> (info)->runLoopCallback();
        */
    }
}
