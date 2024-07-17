crate::ix!();

pub struct OpenGLContextCachedImageBlockingWorker {
    original_worker: OpenGLContextAsyncWorkerPtr,
    finished_signal: WaitableEvent,
}

impl OpenGLContextAsyncWorker for OpenGLContextCachedImageBlockingWorker {

    fn invoke(&mut self, callee_context: &mut OpenGLContext)  {
        
        todo!();
        /*
            if (originalWorker != nullptr)
                    (*originalWorker) (calleeContext);

                finishedSignal.signal();
        */
    }
}

impl OpenGLContextCachedImageBlockingWorker {

    pub fn new(worker_to_use: OpenGLContextAsyncWorkerPtr) -> Self {
    
        todo!();
        /*
            : originalWorker (std::move (workerToUse))
        */
    }
    
    pub fn block(&mut self)  {
        
        todo!();
        /*
            finishedSignal.wait();
        */
    }
}
