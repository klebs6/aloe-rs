crate::ix!();

/**
  | This subclass of UnitTestRunner is used
  | to redirect the test output to our
  | TextBox, and to interrupt the running
  | tests when our thread is asked to
  | stop..
  */
#[no_copy]
#[leak_detector]
pub struct CustomTestRunner<'a> {
    base: UnitTestRunner,
    owner: &'a mut TestRunnerThread<'a>,
}

impl<'a> CustomTestRunner<'a> {

    pub fn new(trt: &mut TestRunnerThread) -> Self {
    
        todo!();
        /*
        : owner(trt),

        
        */
    }
    
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            owner.logMessage (message);
        */
    }
    
    pub fn should_abort_tests(&mut self) -> bool {
        
        todo!();
        /*
            return owner.threadShouldExit();
        */
    }
}
