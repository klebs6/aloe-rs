crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidMessageQueue {
    base: AndroidRunnable,

    /**
      | the this pointer to this class in Java
      | land
      |
      */
    self_:   GlobalRef,
    queue:   ReferenceCountedArray<Box<dyn MessageBaseInterface>,CriticalSection>,
    handler: AndroidHandler,
}

#[cfg(target_os="android")]
aloe_declare_singleton_singlethreaded!{
    AndroidMessageQueue, true
}

#[cfg(target_os="android")]
aloe_implement_singleton!{
    AndroidMessageQueue
}

#[cfg(target_os="android")]
impl Default for AndroidMessageQueue {
    
    fn default() -> Self {
        todo!();
        /*


            : self (CreateJavaInterface (this, "java/lang/AndroidRunnable")
        */
    }
}

#[cfg(target_os="android")]
impl Drop for AndroidMessageQueue {

    fn drop(&mut self) {
        todo!();
        /* 
            ALOE_ASSERT_MESSAGE_THREAD
            clearSingletonInstance();
         */
    }
}

#[cfg(target_os="android")]
impl AndroidMessageQueue {

    pub fn post(&mut self, message: &dyn MessageBaseInterface) -> bool {
        
        todo!();
        /*
            queue.add (std::move (message));

            // this will call us on the message thread
            return handler.post (self.get());
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            for (;;)
            {
                MessageManager::MessageBase::Ptr message (queue.removeAndReturn (0));

                if (message == nullptr)
                    break;

                message->messageCallback();
            }
        */
    }
}
