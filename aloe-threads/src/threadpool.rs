crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ThreadPool.h]

/**
  | A callback class used when you need to
  | select which ThreadPoolJob objects
  | are suitable for some kind of operation.
  | @see ThreadPool::removeAllJobs
  |
  */
pub trait ThreadPoolJobSelector {

    /**
      | Should return true if the specified
      | thread matches your criteria for whatever
      | operation that this object is being
      | used for.
      | 
      | Any implementation of this method must
      | be extremely fast and thread-safe!
      |
      */
    fn is_job_suitable(&mut self, job: *mut ThreadPoolJob) -> bool;

}

///---------------------------------
#[no_copy]
#[leak_detector]
pub struct ThreadPoolThread<'a> {
    base: Thread,
    current_job: Atomic<*mut ThreadPoolJob<'a>>, // default = nullptr 
    pool:        &'a mut ThreadPool<'a>,
}

impl<'a> ThreadPoolThread<'a> {

    pub fn new(
        p:          &mut ThreadPool,
        stack_size: usize) -> Self {
    
        todo!();
        /*


            : Thread ("Pool", stackSize), pool (p)
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
                    if (! pool.runNextJob (*this))
                        wait (500);
        */
    }
}

/**
  | A set of threads that will run a list of
  | jobs.
  | 
  | When a ThreadPoolJob object is added
  | to the ThreadPool's list, its runJob()
  | method will be called by the next pooled
  | thread that becomes free.
  | 
  | @see ThreadPoolJob, Thread
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ThreadPool<'a> {
    jobs:                Vec<*mut ThreadPoolJob<'a>>,
    threads:             Vec<Box<ThreadPoolThread<'a>>>,
    lock:                CriticalSection,
    job_finished_signal: WaitableEvent,
}

impl<'a> Drop for ThreadPool<'a> {

    /**
      | Destructor.
      | 
      | This will attempt to remove all the jobs
      | before deleting, but if you want to specify
      | a timeout, you should call removeAllJobs()
      | explicitly before deleting the pool.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        removeAllJobs (true, 5000);
        stopThreads();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ThreadPool.cpp]
impl<'a> Default for ThreadPool<'a> {

    /**
      | Creates a thread pool with one thread
      | per CPU core.
      | 
      | Once you've created a pool, you can give
      | it some jobs by calling addJob().
      | 
      | If you want to specify the number of threads,
      | use the other constructor; this one
      | creates a pool which has one thread for
      | each CPU core. @see SystemStats::getNumCpus()
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            createThreads (SystemStats::getNumCpus());
        */
    }
}

impl<'a> ThreadPool<'a> {

    /**
      | Creates a thread pool. Once you've created
      | a pool, you can give it some jobs by calling
      | addJob().
      | 
      | -----------
      | @param numberOfThreads
      | 
      | the number of threads to run. These will
      | be started immediately, and will run
      | until the pool is deleted.
      | ----------
      | @param threadStackSize
      | 
      | the size of the stack of each thread.
      | If this value is zero then the default
      | stack size of the OS will be used.
      |
      */
    pub fn new(
        num_threads:       i32,
        thread_stack_size: Option<usize>

    ) -> Self {

        let thread_stack_size: usize =
            thread_stack_size.unwrap_or(0);
    
        todo!();
        /*
           jassert (numThreads > 0); // not much point having a pool without any threads!

           createThreads (numThreads, threadStackSize);
           */
    }
    
    pub fn create_threads(
        &mut self, 
        num_threads:       i32,
        thread_stack_size: Option<usize>

    ) {
        let thread_stack_size: usize = thread_stack_size.unwrap_or(0);
        
        todo!();
        /*
            for (int i = jmax (1, numThreads); --i >= 0;)
            threads.add (new ThreadPoolThread (*this, threadStackSize));

        for (auto* t : threads)
            t->startThread();
        */
    }
    
    pub fn stop_threads(&mut self)  {
        
        todo!();
        /*
            for (auto* t : threads)
            t->signalThreadShouldExit();

        for (auto* t : threads)
            t->stopThread (500);
        */
    }
    
    /**
      | Adds a job to the queue.
      | 
      | Once a job has been added, then the next
      | time a thread is free, it will run the
      | job's ThreadPoolJob::runJob() method.
      | Depending on the return value of the
      | runJob() method, the pool will either
      | remove the job from the pool or add it
      | to the back of the queue to be run again.
      | 
      | If deleteJobWhenFinished is true,
      | then the job object will be owned and
      | deleted by the pool when not needed -
      | if you do this, make sure that your object's
      | destructor is thread-safe.
      | 
      | If deleteJobWhenFinished is false,
      | the pointer will be used but not deleted,
      | and the caller is responsible for making
      | sure the object is not deleted before
      | it has been removed from the pool.
      |
      */
    pub fn add_job(
        &mut self, 
        job:                      *mut ThreadPoolJob,
        delete_job_when_finished: bool)  {
        
        todo!();
        /*
            jassert (job != nullptr);
        jassert (job->pool == nullptr);

        if (job->pool == nullptr)
        {
            job->pool = this;
            job->shouldStop = false;
            job->isActive = false;
            job->shouldBeDeleted = deleteJobWhenFinished;

            {
                const ScopedLock sl (lock);
                jobs.add (job);
            }

            for (auto* t : threads)
                t->notify();
        }
        */
    }
    
    /**
      | Adds a lambda function to be called as
      | a job.
      | 
      | This will create an internal ThreadPoolJob
      | object to encapsulate and call the lambda.
      |
      */
    pub fn add_job_by_lambda(&mut self, job_to_run: fn() -> ThreadPoolJobStatus)  {
        
        todo!();
        /*
            struct LambdaJobWrapper  : public ThreadPoolJob
        {
            LambdaJobWrapper (std::function<ThreadPoolJob::JobStatus()> j) : ThreadPoolJob ("lambda"), job (j) {}
            JobStatus runJob() override      { return job(); }

            std::function<ThreadPoolJob::JobStatus()> job;
        };

        addJob (new LambdaJobWrapper (jobToRun), true);
        */
    }
    
    /**
      | Adds a lambda function to be called as
      | a job.
      | 
      | This will create an internal ThreadPoolJob
      | object to encapsulate and call the lambda.
      |
      */
    pub fn add_job_by_basic_lambda(&mut self, job_to_run: fn() -> ())  {
        
        todo!();
        /*
            struct LambdaJobWrapper  : public ThreadPoolJob
        {
            LambdaJobWrapper (std::function<void()> j) : ThreadPoolJob ("lambda"), job (j) {}
            JobStatus runJob() override      { job(); return ThreadPoolJob::jobHasFinished; }

            std::function<void()> job;
        };

        addJob (new LambdaJobWrapper (jobToRun), true);
        */
    }
    
    /**
      | Returns the number of jobs currently
      | running or queued.
      |
      */
    pub fn get_num_jobs(&self) -> i32 {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return jobs.size();
        */
    }
    
    /**
      | Returns the number of threads assigned
      | to this thread pool.
      |
      */
    pub fn get_num_threads(&self) -> i32 {
        
        todo!();
        /*
            return threads.size();
        */
    }
    
    /**
      | Returns one of the jobs in the queue.
      | 
      | -----------
      | @note
      | 
      | this can be a very volatile list as jobs
      | might be continuously getting shifted
      | around in the list, and this method may
      | return nullptr if the index is currently
      | out-of-range.
      |
      */
    pub fn get_job(&self, index: i32) -> *mut ThreadPoolJob {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return jobs [index];
        */
    }
    
    /**
      | Returns true if the given job is currently
      | queued or running.
      | 
      | @see isJobRunning()
      |
      */
    pub fn contains(&self, job: *const ThreadPoolJob) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return jobs.contains (const_cast<ThreadPoolJob*> (job));
        */
    }
    
    /**
      | Returns true if the given job is currently
      | being run by a thread.
      |
      */
    pub fn is_job_running(&self, job: *const ThreadPoolJob) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return jobs.contains (const_cast<ThreadPoolJob*> (job)) && job->isActive;
        */
    }
    
    /**
      | If the given job is in the queue, this
      | will move it to the front so that it is
      | the next one to be executed.
      |
      */
    pub fn move_job_to_front(&mut self, job: *const ThreadPoolJob)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        auto index = jobs.indexOf (const_cast<ThreadPoolJob*> (job));

        if (index > 0 && ! job->isActive)
            jobs.move (index, 0);
        */
    }
    
    /**
      | Waits until a job has finished running
      | and has been removed from the pool.
      | 
      | This will wait until the job is no longer
      | in the pool - i.e. until its runJob()
      | method returns ThreadPoolJob::jobHasFinished.
      | 
      | If the timeout period expires before
      | the job finishes, this will return false;
      | it returns true if the job has finished
      | successfully.
      |
      */
    pub fn wait_for_job_to_finish(&self, 
        job:         *const ThreadPoolJob,
        time_out_ms: i32) -> bool {
        
        todo!();
        /*
            if (job != nullptr)
        {
            auto start = Time::getMillisecondCounter();

            while (contains (job))
            {
                if (timeOutMs >= 0 && Time::getMillisecondCounter() >= start + (uint32) timeOutMs)
                    return false;

                jobFinishedSignal.wait (2);
            }
        }

        return true;
        */
    }
    
    /**
      | Tries to remove a job from the pool.
      | 
      | If the job isn't yet running, this will
      | simply remove it. If it is running, it
      | will wait for it to finish.
      | 
      | If the timeout period expires before
      | the job finishes running, then the job
      | will be left in the pool and this will
      | return false. It returns true if the
      | job is successfully stopped and removed.
      | 
      | -----------
      | @param job
      | 
      | the job to remove
      | ----------
      | @param interruptIfRunning
      | 
      | if true, then if the job is currently
      | busy, its ThreadPoolJob::signalJobShouldExit()
      | method will be called to try to interrupt
      | it. If false, then if the job will be allowed
      | to run until it stops normally (or the
      | timeout expires)
      | ----------
      | @param timeOutMilliseconds
      | 
      | the length of time this method should
      | wait for the job to finish before giving
      | up and returning false
      |
      */
    pub fn remove_job(&mut self, 
        job:                  *mut ThreadPoolJob,
        interrupt_if_running: bool,
        time_out_ms:          i32) -> bool {
        
        todo!();
        /*
            bool dontWait = true;
        Vec<Box<ThreadPoolJob>> deletionList;

        if (job != nullptr)
        {
            const ScopedLock sl (lock);

            if (jobs.contains (job))
            {
                if (job->isActive)
                {
                    if (interruptIfRunning)
                        job->signalJobShouldExit();

                    dontWait = false;
                }
                else
                {
                    jobs.removeFirstMatchingValue (job);
                    addToDeleteList (deletionList, job);
                }
            }
        }

        return dontWait || waitForJobToFinish (job, timeOutMs);
        */
    }
    
    /**
      | Tries to remove all jobs from the pool.
      | 
      | -----------
      | @param interruptRunningJobs
      | 
      | if true, then all running jobs will have
      | their ThreadPoolJob::signalJobShouldExit()
      | methods called to try to interrupt them
      | ----------
      | @param timeOutMilliseconds
      | 
      | the length of time this method should
      | wait for all the jobs to finish before
      | giving up and returning false
      | ----------
      | @param selectedJobsToRemove
      | 
      | if this is not a nullptr, the JobSelector
      | object is asked to decide which jobs
      | should be removed. If it is a nullptr,
      | all jobs are removed
      | 
      | -----------
      | @return
      | 
      | true if all jobs are successfully stopped
      | and removed; false if the timeout period
      | expires while waiting for one or more
      | jobs to stop
      |
      */
    pub fn remove_all_jobs(
        &mut self, 
        interrupt_running_jobs:  bool,
        time_out_ms:             i32,
        selected_jobs_to_remove: *mut dyn ThreadPoolJobSelector

    ) -> bool {
        
        todo!();
        /*
            Vec<ThreadPoolJob*> jobsToWaitFor;

        {
            Vec<Box<ThreadPoolJob>> deletionList;

            {
                const ScopedLock sl (lock);

                for (int i = jobs.size(); --i >= 0;)
                {
                    auto* job = jobs.getUnchecked(i);

                    if (selectedJobsToRemove == nullptr || selectedJobsToRemove->isJobSuitable (job))
                    {
                        if (job->isActive)
                        {
                            jobsToWaitFor.add (job);

                            if (interruptRunningJobs)
                                job->signalJobShouldExit();
                        }
                        else
                        {
                            jobs.remove (i);
                            addToDeleteList (deletionList, job);
                        }
                    }
                }
            }
        }

        auto start = Time::getMillisecondCounter();

        for (;;)
        {
            for (int i = jobsToWaitFor.size(); --i >= 0;)
            {
                auto* job = jobsToWaitFor.getUnchecked (i);

                if (! isJobRunning (job))
                    jobsToWaitFor.remove (i);
            }

            if (jobsToWaitFor.size() == 0)
                break;

            if (timeOutMs >= 0 && Time::getMillisecondCounter() >= start + (uint32) timeOutMs)
                return false;

            jobFinishedSignal.wait (20);
        }

        return true;
        */
    }
    
    /**
      | Returns a list of the names of all the
      | jobs currently running or queued.
      | 
      | If onlyReturnActiveJobs is true, only
      | the ones currently running are returned.
      |
      */
    pub fn get_names_of_all_jobs(&self, only_return_active_jobs: bool) -> Vec<String> {
        
        todo!();
        /*
            StringArray s;
        const ScopedLock sl (lock);

        for (auto* job : jobs)
            if (job->isActive || ! onlyReturnActiveJobs)
                s.add (job->getJobName());

        return s;
        */
    }
    
    /**
      | Changes the priority of all the threads.
      | 
      | This will call Thread::setPriority()
      | for each thread in the pool.
      | 
      | May return false if for some reason the
      | priority can't be changed.
      |
      */
    pub fn set_thread_priorities(&mut self, new_priority: i32) -> bool {
        
        todo!();
        /*
            bool ok = true;

        for (auto* t : threads)
            if (! t->setPriority (newPriority))
                ok = false;

        return ok;
        */
    }
    
    pub fn pick_next_job_to_run(&mut self) -> *mut ThreadPoolJob {
        
        todo!();
        /*
            Vec<Box<ThreadPoolJob>> deletionList;

        {
            const ScopedLock sl (lock);

            for (int i = 0; i < jobs.size(); ++i)
            {
                if (auto* job = jobs[i])
                {
                    if (! job->isActive)
                    {
                        if (job->shouldStop)
                        {
                            jobs.remove (i);
                            addToDeleteList (deletionList, job);
                            --i;
                            continue;
                        }

                        job->isActive = true;
                        return job;
                    }
                }
            }
        }

        return nullptr;
        */
    }
    
    pub fn run_next_job(&mut self, thread: &mut ThreadPoolThread) -> bool {
        
        todo!();
        /*
            if (auto* job = pickNextJobToRun())
        {
            auto result = ThreadPoolJob::jobHasFinished;
            thread.currentJob = job;

            try
            {
                result = job->runJob();
            }
            catch (...)
            {
                jassertfalse; // Your runJob() method mustn't throw any exceptions!
            }

            thread.currentJob = nullptr;

            Vec<Box<ThreadPoolJob>> deletionList;

            {
                const ScopedLock sl (lock);

                if (jobs.contains (job))
                {
                    job->isActive = false;

                    if (result != ThreadPoolJob::jobNeedsRunningAgain || job->shouldStop)
                    {
                        jobs.removeFirstMatchingValue (job);
                        addToDeleteList (deletionList, job);

                        jobFinishedSignal.signal();
                    }
                    else
                    {
                        // move the job to the end of the queue if it wants another go
                        jobs.move (jobs.indexOf (job), -1);
                    }
                }
            }

            return true;
        }

        return false;
        */
    }
    
    pub fn add_to_delete_list(&self, 
        deletion_list: &mut Vec<Box<ThreadPoolJob>>,
        job:           *mut ThreadPoolJob)  {
        
        todo!();
        /*
            job->shouldStop = true;
        job->pool = nullptr;

        if (job->shouldBeDeleted)
            deletionList.add (job);
        */
    }
}
