crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DemoThreadPoolJob<'a> {
    base:  BouncingBall<'a>,
    base2: ThreadPoolJob<'a>,
}

impl<'a> DemoThreadPoolJob<'a> {

    pub fn new(container_comp: &mut Component) -> Self {
    
        todo!();
        /*


            : BouncingBall (containerComp),
              ThreadPoolJob ("Demo Threadpool Job")
        */
    }
    
    pub fn run_job(&mut self) -> ThreadPoolJobStatus {
        
        todo!();
        /*
            // this is the code that runs this job. It'll be repeatedly called until we return
            // jobHasFinished instead of jobNeedsRunningAgain.
            Thread::sleep (30);

            // because this is a background thread, we mustn't do any UI work without
            // first grabbing a MessageManagerLock..
            const MessageManagerLock mml (this);

            // before moving the ball, we need to check whether the lock was actually gained, because
            // if something is trying to stop this job, it will have failed..
            if (mml.lockWasGained())
                moveBall();

            return jobNeedsRunningAgain;
        */
    }
    
    pub fn removed_from_queue(&mut self)  {
        
        todo!();
        /*
            // This is called to tell us that our job has been removed from the pool.
            // In this case there's no need to do anything here.
        */
    }
}
