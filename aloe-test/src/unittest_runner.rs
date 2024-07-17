crate::ix!();

/**
  | Runs a set of unit tests.
  | 
  | You can instantiate one of these objects
  | and use it to invoke tests on a set of UnitTest
  | objects.
  | 
  | By using a subclass of UnitTestRunner,
  | you can intercept logging messages
  | and perform custom behaviour when each
  | test completes.
  | 
  | @see UnitTest
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct UnitTestRunner {
    current_test:         *mut UnitTest, // default = nullptr
    current_sub_category: String,
    results:              Vec<UnitTestRunnerTestResult>,
    assert_on_failure:    bool, // default = true
    log_passes:           bool, // default = false
    random_for_test:      ThreadRng,
}

impl UnitTestRunner {

    /**
      | Sets a flag to indicate whether an assertion
      | should be triggered if a test fails.
      | 
      | This is true by default.
      |
      */
    pub fn set_assert_on_failure(&mut self, should_assert: bool)  {
        
        todo!();
        /*
            assertOnFailure = shouldAssert;
        */
    }
    
    /**
      | Sets a flag to indicate whether successful
      | tests should be logged.
      | 
      | By default, this is set to false, so that
      | only failures will be displayed in the
      | log.
      |
      */
    pub fn set_passes_are_logged(&mut self, should_display_passes: bool)  {
        
        todo!();
        /*
            logPasses = shouldDisplayPasses;
        */
    }
    
    /**
      | Returns the number of UnitTestRunnerTestResult objects
      | that have been performed.
      | 
      | @see getResult
      |
      */
    pub fn get_num_results(&self) -> i32 {
        
        todo!();
        /*
            return results.size();
        */
    }
    
    /**
      | Returns one of the UnitTestRunnerTestResult objects
      | that describes a test that has been run.
      | 
      | @see getNumResults
      |
      */
    pub fn get_result(&self, index: i32) -> *const UnitTestRunnerTestResult {
        
        todo!();
        /*
            return results [index];
        */
    }
    
    pub fn results_updated(&mut self)  {
        
    }
    
    /**
      | Runs a set of tests.
      | 
      | The tests are performed in order, and
      | the results are logged. To run all the
      | registered UnitTest objects that exist,
      | use runAllTests().
      | 
      | If you want to run the tests with a predetermined
      | seed, you can pass that into the randomSeed
      | argument, or pass 0 to have a randomly-generated
      | seed chosen.
      |
      */
    pub fn run_tests(
        &mut self, 
        tests:       &[*mut UnitTest],
        random_seed: Option<i64>)  {

        let random_seed: i64 = random_seed.unwrap_or(0);
        
        todo!();
        /*
            results.clear();
        resultsUpdated();

        if (randomSeed == 0)
            randomSeed = Random().nextInt (0x7ffffff);

        randomForTest = Random (randomSeed);
        logMessage ("Random seed: 0x" + String::toHexString (randomSeed));

        for (auto* t : tests)
        {
            if (shouldAbortTests())
                break;

           #if ALOE_EXCEPTIONS_DISABLED
            t->performTest (this);
           #else
            try
            {
                t->performTest (this);
            }
            catch (...)
            {
                addFail ("An unhandled exception was thrown!");
            }
           #endif
        }

        endTest();
        */
    }
    
    /**
      | Runs all the UnitTest objects that currently
      | exist.
      | 
      | This calls runTests() for all the objects
      | listed in UnitTest::getAllTests().
      | 
      | If you want to run the tests with a predetermined
      | seed, you can pass that into the randomSeed
      | argument, or pass 0 to have a randomly-generated
      | seed chosen.
      |
      */
    pub fn run_all_tests(&mut self, random_seed: Option<i64>)  {

        let random_seed: i64 = random_seed.unwrap_or(0);
        
        todo!();
        /*
            runTests (UnitTest::getAllTests(), randomSeed);
        */
    }
    
    /**
      | Runs all the UnitTest objects within
      | a specified category.
      | 
      | This calls runTests() for all the objects
      | listed in UnitTest::getTestsInCategory().
      | 
      | If you want to run the tests with a predetermined
      | seed, you can pass that into the randomSeed
      | argument, or pass 0 to have a randomly-generated
      | seed chosen.
      |
      */
    pub fn run_tests_in_category(&mut self, 
        category:    &String,
        random_seed: Option<i64>)  {

        let random_seed: i64 = random_seed.unwrap_or(0);
        
        todo!();
        /*
            runTests (UnitTest::getTestsInCategory (category), randomSeed);
        */
    }
    
    /**
      | Writes a message to the test log.
      | 
      | This can only be called from within your
      | runTest() method.
      |
      */
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            Logger::writeToLog (message);
        */
    }
    
    pub fn should_abort_tests(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn begin_new_test(&mut self, 
        test:         *mut UnitTest,
        sub_category: &String)  {
        
        todo!();
        /*
            endTest();
        currentTest = test;

        auto testName = test->getName();
        results.add (new UnitTestRunnerTestResult (testName, subCategory));

        logMessage ("-----------------------------------------------------------------");
        logMessage ("Starting test: " + testName + " / " + subCategory + "...");

        resultsUpdated();
        */
    }
    
    pub fn end_test(&mut self)  {
        
        todo!();
        /*
            if (auto* r = results.getLast())
        {
            r->endTime = Time::getCurrentTime();

            if (r->failures > 0)
            {
                String m ("FAILED!!  ");
                m << r->failures << (r->failures == 1 ? " test" : " tests")
                  << " failed, out of a total of " << (r->passes + r->failures);

                logMessage (String());
                logMessage (m);
                logMessage (String());
            }
            else
            {
                logMessage ("All tests completed successfully");
            }
        }
        */
    }
    
    pub fn add_pass(&mut self)  {
        
        todo!();
        /*
            {
            const ScopedLock sl (results.getLock());

            auto* r = results.getLast();
            jassert (r != nullptr); // You need to call UnitTest::beginTest() before performing any tests!

            r->passes++;

            if (logPasses)
            {
                String message ("Test ");
                message << (r->failures + r->passes) << " passed";
                logMessage (message);
            }
        }

        resultsUpdated();
        */
    }
    
    pub fn add_fail(&mut self, failure_message: &String)  {
        
        todo!();
        /*
            {
            const ScopedLock sl (results.getLock());

            auto* r = results.getLast();
            jassert (r != nullptr); // You need to call UnitTest::beginTest() before performing any tests!

            r->failures++;

            String message ("!!! Test ");
            message << (r->failures + r->passes) << " failed";

            if (failureMessage.isNotEmpty())
                message << ": " << failureMessage;

            r->messages.add (message);

            logMessage (message);
        }

        resultsUpdated();

        if (assertOnFailure) { jassertfalse; }
        */
    }
}
