crate::ix!();

/**
  | Describes how to show the notification
  | when the screen is locked. Available
  | from Android API 21 or above.
  |
  */
pub enum PushNotificationLockScreenAppearance
{
    /**
      | The notification is not allowed on the
      | lock screen
      |
      */
    dontShow       = -1,  

    /**
      | Only some information is allowed on
      | the lock screen
      |
      */
    showPartially  =  0,  

    /**
      | The entire notification is allowed
      | on the lock screen
      |
      */
    showCompletely =  1,   
}

impl Default for PushNotificationLockScreenAppearance {

    fn default() -> Self {
        todo!();
    }
}
