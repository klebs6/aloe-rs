crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BackgroundMessageQueue {
    base:      Thread,
    pop_mutex: CriticalSection,
    queue:     Queue<BackgroundMessageQueueIncomingCommand>,
}

pub type BackgroundMessageQueueIncomingCommand = FixedSizeFunction<400,(),()>;

impl BackgroundMessageQueue {

    pub fn new(entries: i32) -> Self {
    
        todo!();
        /*
            : Thread ("Convolution background loader"), queue (entries)
        */
    }

    /**
      | Push functions here, and they'll be called
      | later on a background thread.
      |
      | This function is wait-free.
      |
      | This function is only safe to call from
      | a single thread at a time.
      */
    pub fn push(&mut self, command: &mut BackgroundMessageQueueIncomingCommand) -> bool {
        
        todo!();
        /*
            return queue.push (command);
        */
    }
    
    pub fn pop_all(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (popMutex);
            queue.popAll ([] (BackgroundMessageQueueIncomingCommand& command) { command(); command = nullptr; });
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
            {
                const auto tryPop = [&]
                {
                    const ScopedLock lock (popMutex);

                    if (! queue.hasPendingMessages())
                        return false;

                    queue.pop ([] (BackgroundMessageQueueIncomingCommand& command) { command(); command = nullptr;});
                    return true;
                };

                if (! tryPop())
                    sleep (10);
            }
        */
    }
}
