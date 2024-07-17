crate::ix!();

/**
  | Controls the appearance of this action.
  |
  */
pub enum PushNotificationActionStyle
{
    /**
      | Show this action as a button.
      |
      */
    button,                    

    /**
      | Show this action as a text input field
      | (on Android API 20 or higher is required).
      |
      */
    text,                       
}
