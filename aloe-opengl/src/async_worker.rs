crate::ix!();

///-----------------
pub trait OpenGLContextAsyncWorker {
    fn invoke(&mut self, _0: &mut OpenGLContext);
}

pub type OpenGLContextAsyncWorkerPtr = Rc<RefCell<dyn OpenGLContextAsyncWorker>>;

///-----------------
#[no_copy]
pub struct OpenGLContextAsyncWorkerFunctor<FunctionType> {
    functor: FunctionType,
}

impl<FunctionType> OpenGLContextAsyncWorker for OpenGLContextAsyncWorkerFunctor<FunctionType> {

    fn invoke(&mut self, caller_context: &mut OpenGLContext)  {
        
        todo!();
        /*
            functor (callerContext);
        */
    }
}

impl<FunctionType> OpenGLContextAsyncWorkerFunctor<FunctionType> {

    pub fn new(functor_to_use: FunctionType) -> Self {
    
        todo!();
        /*
        : functor(functorToUse),

        
        */
    }
}
