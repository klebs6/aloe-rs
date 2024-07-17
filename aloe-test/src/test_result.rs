crate::ix!();

/**
  | Contains the results of a test.
  | 
  | One of these objects is instantiated
  | each time UnitTest::beginTest() is
  | called, and it contains details of the
  | number of subsequent UnitTest::expect()
  | calls that are made.
  |
  */
pub struct UnitTestRunnerTestResult {

    /**
      | The main name of this test (i.e. the name
      | of the UnitTest object being run).
      |
      */
    unit_test_name: String,

    /**
      | The name of the current subcategory
      | (i.e. the name that was set when UnitTest::beginTest()
      | was called).
      |
      */
    subcategory_name: String,

    /**
      | The number of UnitTest::expect() calls
      | that succeeded.
      |
      */
    passes: i32, // default = 0

    /**
      | The number of UnitTest::expect() calls
      | that failed.
      |
      */
    failures: i32, // default = 0

    /**
      | A list of messages describing the failed
      | tests.
      |
      */
    messages: Vec<String>,

    /**
      | The time at which this test was started.
      |
      */
    start_time: Instant, // = Time::getCurrentTime();

    /**
      | The time at which this test ended.
      |
      */
    end_time: Instant,

}

impl Default for UnitTestRunnerTestResult {

    fn default() -> Self {
        Self {
            unit_test_name:   "".to_string(),
            subcategory_name: "".to_string(),
            passes:           0,
            failures:         0,
            messages:         vec![],
            start_time:       Instant::now(),
            end_time:         Instant::now(),
        }
    }
}

impl UnitTestRunnerTestResult {

    pub fn new(
        name:         &String,
        sub_category: &String) -> Self {
    
        todo!();
        /*
        : unit_test_name(name),
        : subcategory_name(subCategory),

        
        */
    }
}
