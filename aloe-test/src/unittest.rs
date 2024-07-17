crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/unit_tests/aloe_UnitTest.h]

/**
  | This is a base class for classes that perform
  | a unit test.
  |
  | To write a test using this class, your code
  | should look something like this:
  |
  | @code
  | class MyTest  : public UnitTest
  | {
  |
  |     MyTest()  : UnitTest ("Foobar testing") {}
  |
  |     void runTest() override
  |     {
  |         beginTest ("Part 1");
  |
  |         expect (myFoobar.doesSomething());
  |         expect (myFoobar.doesSomethingElse());
  |
  |         beginTest ("Part 2");
  |
  |         expect (myOtherFoobar.doesSomething());
  |         expect (myOtherFoobar.doesSomethingElse());
  |
  |         ...etc..
  |     }
  | };
  |
  | // Creating a static instance will
  | // automatically add the instance to the array
  | // returned by UnitTest::getAllTests(), so the
  | // test will be included when you call
  | // UnitTestRunner::runAllTests()
  | static MyTest test;
  | @endcode
  |
  | To run a test, use the UnitTestRunner class.
  |
  | @see UnitTestRunner
  |
  | @tags{Core}
  */
#[no_copy]
pub struct UnitTest {
    name:     String,
    category: String,
    runner:   *mut UnitTestRunner, // default = nullptr
}

impl Drop for UnitTest {

    fn drop(&mut self) {
        todo!();
        /* 
        getAllTests().removeFirstMatchingValue (this);
 */
    }
}

impl UnitTest {

    /**
      | Returns the name of the test.
      |
      */
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Returns the category of the test.
      |
      */
    pub fn get_category(&self) -> &String {
        
        todo!();
        /*
            return category;
        */
    }

    /**
      | Compares a value to an expected value.
      | 
      | If they are not equal, prints out a message
      | containing the expected and actual
      | values.
      |
      */
    pub fn expect_equals<ValueType>(&mut self, 
        actual:          ValueType,
        expected:        ValueType,
        failure_message: Option<String>)  {
    
        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = actual == expected;
            expectResultAndPrint (actual, expected, result, "", failureMessage);
        */
    }

    /**
      | Checks whether a value is not equal to
      | a comparison value.
      | 
      | If this check fails, prints out a message
      | containing the actual and comparison
      | values.
      |
      */
    pub fn expect_not_equals<ValueType>(&mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        failure_message:     Option<String>)  {

        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = value != valueToCompareTo;
            expectResultAndPrint (value, valueToCompareTo, result, "unequal to", failureMessage);
        */
    }

    /**
      | Checks whether a value is greater than
      | a comparison value.
      | 
      | If this check fails, prints out a message
      | containing the actual and comparison
      | values.
      |
      */
    pub fn expect_greater_than<ValueType>(&mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        failure_message:     Option<String>)  {

        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = value > valueToCompareTo;
            expectResultAndPrint (value, valueToCompareTo, result, "greater than", failureMessage);
        */
    }

    /**
      | Checks whether a value is less than a
      | comparison value.
      | 
      | If this check fails, prints out a message
      | containing the actual and comparison
      | values.
      |
      */
    pub fn expect_less_than<ValueType>(&mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        failure_message:     Option<String>)  {
    
        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = value < valueToCompareTo;
            expectResultAndPrint (value, valueToCompareTo, result, "less than", failureMessage);
        */
    }

    /**
      | Checks whether a value is greater or
      | equal to a comparison value.
      | 
      | If this check fails, prints out a message
      | containing the actual and comparison
      | values.
      |
      */
    pub fn expect_greater_or_equal<ValueType>(
        &mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        failure_message:     Option<String>)  {

        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = value >= valueToCompareTo;
            expectResultAndPrint (value, valueToCompareTo, result, "greater or equal to", failureMessage);
        */
    }

    /**
      | Checks whether a value is less or equal
      | to a comparison value.
      | 
      | If this check fails, prints out a message
      | containing the actual and comparison
      | values.
      |
      */
    pub fn expect_less_or_equal<ValueType>(
        &mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        failure_message:     Option<String>)  {
    
        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            bool result = value <= valueToCompareTo;
            expectResultAndPrint (value, valueToCompareTo, result, "less or equal to", failureMessage);
        */
    }

    /**
      | Computes the difference between a value
      | and a comparison value, and if it is larger
      | than a specified maximum value, prints
      | out a message containing the actual
      | and comparison values and the maximum
      | allowed error.
      |
      */
    pub fn expect_within_absolute_error<ValueType>(&mut self, 
        actual:             ValueType,
        expected:           ValueType,
        max_absolute_error: ValueType,
        failure_message:    Option<String>)  {

        let failure_message: String =
            failure_message.unwrap_or(String::new());

        todo!();
        /*
            const ValueType diff = std::abs (actual - expected);
            const bool result = diff <= maxAbsoluteError;

            expectResultAndPrint (actual, expected, result, " within " + String (maxAbsoluteError) + " of" , failureMessage);
        */
    }
    
    pub fn expect_result_and_print<ValueType>(&mut self, 
        value:               ValueType,
        value_to_compare_to: ValueType,
        result:              bool,
        comp_description:    String,
        failure_message:     String)  {
    
        todo!();
        /*
            if (! result)
            {
                if (failureMessage.isNotEmpty())
                    failureMessage << " -- ";

                failureMessage << "Expected value" << (compDescription.isEmpty() ? "" : " ")
                               << compDescription << ": " << valueToCompareTo
                               << ", Actual value: " << value;
            }

            expect (result, failureMessage);
        */
    }
    
    /**
      | Creates a test with the given name and
      | optionally places it in a category.
      |
      */
    pub fn new(
        nm:  &String,
        ctg: Option<&str>) -> Self {

        let ctg = ctg.unwrap_or("");
    
        todo!();
        /*
        : name(nm),
        : category(ctg),

            getAllTests().add (this);
        */
    }
    
    /**
      | Returns the set of all UnitTest objects
      | that currently exist.
      |
      */
    pub fn get_all_tests(&mut self) -> &mut [*mut UnitTest] {
        
        todo!();
        /*
            static Vec<UnitTest*> tests;
        return tests;
        */
    }
    
    /**
      | Returns the set of UnitTests in a specified
      | category.
      |
      */
    pub fn get_tests_in_category(&mut self, category: &String) -> &[*mut UnitTest] {
        
        todo!();
        /*
            if (category.isEmpty())
            return getAllTests();

        Vec<UnitTest*> unitTests;

        for (auto* test : getAllTests())
            if (test->getCategory() == category)
                unitTests.add (test);

        return unitTests;
        */
    }
    
    /**
      | Returns a StringArray containing all
      | of the categories of UnitTests that
      | have been registered.
      |
      */
    pub fn get_all_categories(&mut self) -> Vec<String> {
        
        todo!();
        /*
            StringArray categories;

        for (auto* test : getAllTests())
            if (test->getCategory().isNotEmpty())
                categories.addIfNotAlreadyThere (test->getCategory());

        return categories;
        */
    }
    
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Runs the test, using the specified UnitTestRunner.
      | 
      | You shouldn't need to call this method
      | directly - use UnitTestRunner::runTests()
      | instead.
      |
      */
    pub fn perform_test(&mut self, new_runner: *mut UnitTestRunner)  {
        
        todo!();
        /*
            jassert (newRunner != nullptr);
        runner = newRunner;

        initialise();
        runTest();
        shutdown();
        */
    }
    
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            // This method's only valid while the test is being run!
        jassert (runner != nullptr);

        runner->logMessage (message);
        */
    }
    
    /**
      | Tells the system that a new subsection
      | of tests is beginning.
      | 
      | This should be called from your runTest()
      | method, and may be called as many times
      | as you like, to demarcate different
      | sets of tests.
      |
      */
    pub fn begin_test(&mut self, test_name: &String)  {
        
        todo!();
        /*
            // This method's only valid while the test is being run!
        jassert (runner != nullptr);

        runner->beginNewTest (this, testName);
        */
    }
    
    /** 
      | Checks that the result of a test is true,
      | and logs this result.
      |
      | In your runTest() method, you should call
      | this method for each condition that you
      | want to check, e.g.
      |
      |   @code
      |   void runTest()
      |   {
      |       beginTest ("basic tests");
      |       expect (x + y == 2);
      |       expect (getThing() == someThing);
      |       ...etc...
      |   }
      |   @endcode
      |
      |   If testResult is true, a pass is logged;
      |   if it's false, a failure is logged.
      |
      |   If the failure message is specified, it
      |   will be written to the log if the test fails.
      */
    pub fn expect(&mut self, 
        result:          bool,
        failure_message: Option<&str>)  {

        let failure_message = failure_message.unwrap_or("");
        
        todo!();
        /*
            // This method's only valid while the test is being run!
        jassert (runner != nullptr);

        if (result)
            runner->addPass();
        else
            runner->addFail (failureMessage);
        */
    }
    
    /**
      | Returns a shared RNG that all unit tests
      | should use.
      | 
      | If a test needs random numbers, it's
      | important that when an error is found,
      | the exact circumstances can be re-created
      | in order to re-test the problem, by repeating
      | the test with the same random seed value.
      | 
      | To make this possible, the UnitTestRunner
      | class creates a master seed value for
      | the run, writes this number to the log,
      | and then this method returns a Random
      | object based on that seed. All tests
      | should only use this method to create
      | any Random objects that they need.
      | 
      | -----------
      | @note
      | 
      | this method will return an identical
      | object each time it's called for a given
      | run, so if you need several different
      | Random objects, the best way to do that
      | is to call Random::combineSeed() on
      | the result to permute it with a constant
      | value.
      |
      */
    pub fn get_random(&self) -> ThreadRng {
        
        todo!();
        /*
            // This method's only valid while the test is being run!
        jassert (runner != nullptr);

        return runner->randomForTest;
        */
    }
}
