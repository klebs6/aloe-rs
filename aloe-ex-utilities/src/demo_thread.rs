crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DemoThread<'a> {
    base:     BouncingBall<'a>,
    base2:    Thread,
    interval: i32, //= Random::getSystemRandom().nextInt (50) + 6;
}

impl<'a> Drop for DemoThread<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            // allow the thread 2 seconds to stop cleanly - should be plenty of time.
            stopThread (2000);
         */
    }
}

impl<'a> DemoThread<'a> {

    pub fn new(container_comp: &mut Component) -> Self {
    
        todo!();
        /*


            : BouncingBall (containerComp),
              Thread ("Aloe Demo Thread")

            // give the threads a random priority, so some will move more
            // smoothly than others..
            startThread (Random::getSystemRandom().nextInt (3) + 3);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            // this is the code that runs this thread - we'll loop continuously,
            // updating the coordinates of our blob.

            // threadShouldExit() returns true when the stopThread() method has been
            // called, so we should check it often, and exit as soon as it gets flagged.
            while (! threadShouldExit())
            {
                // sleep a bit so the threads don't all grind the CPU to a halt..
                wait (interval);

                // because this is a background thread, we mustn't do any UI work without
                // first grabbing a MessageManagerLock..
                const MessageManagerLock mml (Thread::getCurrentThread());

                if (! mml.lockWasGained())  // if something is trying to kill this job, the lock
                    return;                 // will fail, in which case we'd better return..

                // now we've got the UI thread locked, we can mess about with the components
                moveBall();
            }
        */
    }
}
