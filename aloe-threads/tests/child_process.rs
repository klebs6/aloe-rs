crate::ix!();

pub struct ChildProcessTests {
    base: UnitTest,
}

impl Default for ChildProcessTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("ChildProcess", UnitTestCategories::threads
        */
    }
}

impl ChildProcessTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Child Processes");

          #if ALOE_WINDOWS || ALOE_MAC || ALOE_LINUX || ALOE_BSD
            ChildProcess p;

           #if ALOE_WINDOWS
            expect (p.start ("tasklist"));
           #else
            expect (p.start ("ls /"));
           #endif

            auto output = p.readAllProcessOutput();
            expect (output.isNotEmpty());
          #endif
        */
    }
}

lazy_static!{
    /*
    static ChildProcessTests childProcessUnitTests;
    */
}

