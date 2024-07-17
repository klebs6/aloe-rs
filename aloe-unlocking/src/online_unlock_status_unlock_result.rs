crate::ix!();

/**
  | This provides some details about the
  | reply that the server gave in a call to
  | attemptWebserverUnlock().
  |
  */
pub struct OnlineUnlockStatusUnlockResult
{
    /**
      | If an unlock operation fails, this is
      | the error message that the webserver
      | supplied (or a message saying that the
      | server couldn't be contacted)
      |
      */
    error_message:       String,


    /**
      | This is a message that the webserver
      | returned, and which the user should
      | be shown.
      | 
      | It's not necessarily an error message,
      | e.g. it might say that there's a new version
      | of the app available or some other status
      | update.
      |
      */
    informative_message: String,


    /**
      | If the webserver wants the user to be
      | directed to a web-page for further information,
      | this is the Url that it would like them
      | to go to.
      |
      */
    url_to_launch:       String,


    /**
      | If the unlock operation succeeded,
      | this will be set to true.
      |
      */
    succeeded:           bool,
}
