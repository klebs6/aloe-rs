crate::ix!();

pub trait UnitTestRunnerInterface {

    /**
      | Called when the list of results changes.
      | 
      | You can override this to perform some
      | sort of behaviour when results are added.
      |
      */
    fn results_updated(&mut self);

    /**
      | Logs a message about the current test
      | progress.
      | 
      | By default this just writes the message
      | to the Logger class, but you could override
      | this to do something else with the data.
      |
      */
    fn log_message(&mut self, message: &String);

    /**
      | This can be overridden to let the runner
      | know that it should abort the tests as
      | soon as possible, e.g. because the thread
      | needs to stop.
      |
      */
    fn should_abort_tests(&mut self) -> bool;
}

