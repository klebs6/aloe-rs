crate::ix!();

pub trait GetLastError {

    /**
      | Returns the last error that happened
      | if anything went wrong.
      |
      */
    fn get_last_error(&mut self) -> String;
}
