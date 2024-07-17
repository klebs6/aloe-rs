crate::ix!();

pub trait UnitTestInterface {

    /**
      | You can optionally implement this method
      | to set up your test.
      | 
      | This method will be called before runTest().
      |
      */
    fn initialise(&mut self);

    /**
      | You can optionally implement this method
      | to clear up after your test has been run.
      | 
      | This method will be called after runTest()
      | has returned.
      |
      */

    fn shutdown(&mut self);

    /**
      | Implement this method in your subclass
      | to actually run your tests.
      | 
      | The content of your implementation
      | should call beginTest() and expect()
      | to perform the tests.
      |
      */
    fn run_test(&mut self);
}
