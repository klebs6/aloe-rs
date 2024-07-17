crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TestRunnerThread<'a> {
    base:     Thread,
    base2:    Timer,
    owner:    &'a mut UnitTestsDemo<'a>,
    category: String,
}

impl<'a> TestRunnerThread<'a> {

    pub fn new(
        utd: &mut UnitTestsDemo,
        ctg: &String) -> Self {
    
        todo!();
        /*

            : Thread ("Unit Tests"),
                  owner (utd),
                  category (ctg)
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            CustomTestRunner runner (*this);

                if (category == "All Tests")
                    runner.runAllTests();
                else
                    runner.runTestsInCategory (category);

                startTimer (50); // when finished, start the timer which will
                                 // wait for the thread to end, then tell our component.
        */
    }
    
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            WeakReference<UnitTestsDemo> safeOwner (&owner);

                MessageManager::callAsync ([=]
                {
                    if (auto* o = safeOwner.get())
                        o->logMessage (message);
                });
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (! isThreadRunning())
                    owner.testFinished(); // inform the demo page when done, so it can delete this thread.
        */
    }
}
