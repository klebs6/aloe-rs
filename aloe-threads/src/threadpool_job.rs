crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ThreadPool.h]

/**
  | These are the values that can be returned
  | by the runJob() method.
  |
  */
pub enum ThreadPoolJobStatus
{
    /**
      | indicates that the job has finished
      | and can be removed from the pool
      |
      */
    jobHasFinished = 0,     

    /**
      | indicates that the job would like to
      | be called again when a thread is free.
      |
      */
    jobNeedsRunningAgain    
}

pub trait ThreadPoolJobInterface {

    /**
      | Performs the actual work that this job
      | needs to do.
      | 
      | Your subclass must implement this method,
      | in which is does its work.
      | 
      | If the code in this method takes a significant
      | time to run, it must repeatedly check
      | the shouldExit() method to see if something
      | is trying to interrupt the job. If shouldExit()
      | ever returns true, the runJob() method
      | must return immediately.
      | 
      | If this method returns jobHasFinished,
      | then the job will be removed from the
      | pool immediately. If it returns jobNeedsRunningAgain,
      | then the job will be left in the pool and
      | will get a chance to run again as soon
      | as a thread is free.
      | 
      | @see shouldExit()
      |
      */
    fn run_job(&mut self) -> ThreadPoolJobStatus;
}

/**
  | A task that is executed by a ThreadPool
  | object.
  | 
  | A ThreadPool keeps a list of ThreadPoolJob
  | objects which are executed by its threads.
  | 
  | The runJob() method needs to be implemented
  | to do the task, and if the code that does
  | the work takes a significant time to
  | run, it must keep checking the shouldExit()
  | method to see if something is trying
  | to interrupt the job. If shouldExit()
  | returns true, the runJob() method must
  | return immediately.
  | 
  | @see ThreadPool, Thread
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ThreadPoolJob<'a> {
    job_name:          String,
    pool:              *mut ThreadPool<'a>, // default = nullptr
    should_stop:       AtomicBool, // default = false 
    is_active:         AtomicBool, // default = false 
    should_be_deleted: AtomicBool, // default = false 
    listeners:         ListenerList<DynamicThreadListener,Array<*mut dyn ThreadListener,CriticalSection>>,
}

pub type DynamicThreadListener = Box<dyn ThreadListener>;

impl<'a> Drop for ThreadPoolJob<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        // you mustn't delete a job while it's still in a pool! Use ThreadPool::removeJob()
        // to remove it first!
        jassert (pool == nullptr || ! pool->contains (this));
         */
    }
}

impl<'a> ThreadPoolJob<'a> {

    /**
      | Returns true if this job is currently
      | running its runJob() method.
      |
      */
    pub fn is_running(&self) -> bool {
        
        todo!();
        /*
            return isActive;
        */
    }

    /**
      | Returns true if something is trying
      | to interrupt this job and make it stop.
      | 
      | Your runJob() method must call this
      | whenever it gets a chance, and if it ever
      | returns true, the runJob() method must
      | return immediately.
      | 
      | @see signalJobShouldExit()
      |
      */
    pub fn should_exit(&self) -> bool {
        
        todo!();
        /*
            return shouldStop;
        */
    }

    /**
      | Creates a thread pool job object.
      | 
      | After creating your job, add it to a thread
      | pool with ThreadPool::addJob().
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : job_name(name),

        
        */
    }
    
    /**
      | Returns the name of this job. @see setJobName
      |
      */
    pub fn get_job_name(&self) -> String {
        
        todo!();
        /*
            return jobName;
        */
    }
    
    /**
      | Changes the job's name. @see getJobName
      |
      */
    pub fn set_job_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            jobName = newName;
        */
    }
    
    /**
      | Calling this will cause the shouldExit()
      | method to return true, and the job should
      | (if it's been implemented correctly)
      | stop as soon as possible.
      | 
      | @see shouldExit()
      |
      */
    pub fn signal_job_should_exit(&mut self)  {
        
        todo!();
        /*
            shouldStop = true;
        listeners.call ([] (Thread::Listener& l) { l.exitSignalSent(); });
        */
    }
    
    /**
      | Add a listener to this thread job which
      | will receive a callback when signalJobShouldExit
      | was called on this thread job.
      | 
      | @see signalJobShouldExit, removeListener
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
      | If the calling thread is being invoked
      | inside a runJob() method, this will
      | return the ThreadPoolJob that it belongs
      | to.
      |
      */
    pub fn get_current_thread_pool_job(&mut self) -> *mut ThreadPoolJob {
        
        todo!();
        /*
            if (auto* t = dynamic_cast<ThreadPool::ThreadPoolThread*> (Thread::getCurrentThread()))
            return t->currentJob.load();

        return nullptr;
        */
    }
}
