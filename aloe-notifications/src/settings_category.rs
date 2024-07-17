crate::ix!();

/**
  | Describes a category of a notification.
  | Each category has a unique identifier
  | and a list of associated actions.
  | 
  | -----------
  | @note
  | 
  | the OS may allow only a limited number
  | of actions to be presented, so always
  | present most important actions first.
  |
  */
pub struct PushNotificationSettingsCategory
{

    /**
      | unique identifier
      |
      */
    identifier:          String,


    /**
      | optional list of actions within this
      | category
      |
      */
    actions:             Vec<PushNotificationAction>,


    /**
      | whether dismiss action will be sent
      | to the app (from iOS 10 only)
      |
      */
    send_dismiss_action: bool, // default = false
}
